use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::{map_deserialization_error, MagicedenError, WrappedError},
    Collections, Instructions, Marketplace, Mmm, Tokens, Wallets,
};

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    api_base: String,
    http_client: reqwest::Client,
    backoff: backoff::ExponentialBackoff,
}

/// Default v2 API base url
pub const API_BASE: &str = "https://api-mainnet.magiceden.dev/v2";

impl Default for Client {
    fn default() -> Self {
        Self {
            api_key: "".to_string(),
            api_base: API_BASE.to_string(),
            http_client: reqwest::Client::new(),
            backoff: backoff::ExponentialBackoff::default(),
        }
    }
}

#[allow(dead_code)]
impl Client {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }

    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = api_key.into();
        self
    }

    pub fn with_api_base<S: Into<String>>(mut self, api_base: S) -> Self {
        self.api_base = api_base.into();
        self
    }

    pub fn with_backoff(mut self, backoff: backoff::ExponentialBackoff) -> Self {
        self.backoff = backoff;
        self
    }

    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.api_base)
    }

    fn query(&self) -> Vec<(&str, &str)> {
        vec![]
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );

        headers
    }

    // API groups

    pub fn instructions(&self) -> Instructions {
        Instructions::new(self)
    }

    pub fn collections(&self) -> Collections {
        Collections::new(self)
    }

    pub fn marketplace(&self) -> Marketplace {
        Marketplace::new(self)
    }

    pub fn mmm(&self) -> Mmm {
        Mmm::new(self)
    }

    pub fn tokens(&self) -> Tokens {
        Tokens::new(self)
    }

    pub fn wallets(&self) -> Wallets {
        Wallets::new(self)
    }

    pub(crate) async fn get<O>(&self, path: &str) -> Result<O, MagicedenError>
    where
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .get(self.url(path))
                .query(&self.query())
                .headers(self.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    pub(crate) async fn get_with_query<Q, O>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<O, MagicedenError>
    where
        Q: Serialize + ?Sized,
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .get(self.url(path))
                .query(&self.query())
                .query(&query)
                .headers(self.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Make a POST request to {path} and deserialize the response body
    pub(crate) async fn post<I, O>(&self, path: &str, request: I) -> Result<O, MagicedenError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .post(self.url(path))
                .query(&self.query())
                .headers(self.headers())
                .json(&request)
                .build()?)
        };

        self.execute(request_maker).await
    }

    async fn execute_raw<M, Fut>(&self, request_maker: M) -> Result<Bytes, MagicedenError>
    where
        M: Fn() -> Fut,
        Fut: core::future::Future<Output = Result<reqwest::Request, MagicedenError>>,
    {
        let client = self.http_client.clone();

        backoff::future::retry(self.backoff.clone(), || async {
            let request = request_maker().await.map_err(backoff::Error::Permanent)?;
            let response = client
                .execute(request)
                .await
                .map_err(MagicedenError::Reqwest)
                .map_err(backoff::Error::Permanent)?;
            let status = response.status();
            let by = response.bytes().await;
            // println!("response: {:?}", by);
            let bytes = by
                .map_err(MagicedenError::Reqwest)
                .map_err(backoff::Error::Permanent)?;

            // Deserialize response body from either error object or actual response object
            if !status.is_success() {
                let wrapped_error: WrappedError = serde_json::from_slice(bytes.as_ref())
                    .map_err(|e| map_deserialization_error(e, bytes.as_ref()))
                    .map_err(backoff::Error::Permanent)?;

                if status.as_u16() == 429
                        // API returns 429 also when:
                        // "You exceeded your current quota, please check your plan and billing details."
                        && wrapped_error.error.r#type != Some("insufficient_quota".to_string())
                {
                    // Rate limited retry...
                    tracing::warn!("Rate limited: {}", wrapped_error.error.message);
                    return Err(backoff::Error::Transient {
                        err: MagicedenError::ApiError(wrapped_error.error),
                        retry_after: None,
                    });
                } else {
                    return Err(backoff::Error::Permanent(MagicedenError::ApiError(
                        wrapped_error.error,
                    )));
                }
            }

            Ok(bytes)
        })
        .await
    }

    async fn execute<O, M, Fut>(&self, request_maker: M) -> Result<O, MagicedenError>
    where
        M: Fn() -> Fut,
        O: DeserializeOwned,
        Fut: core::future::Future<Output = Result<reqwest::Request, MagicedenError>>,
    {
        let bytes = self.execute_raw(request_maker).await?;

        let response: O = serde_json::from_slice(bytes.as_ref())
            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

        Ok(response)
    }
}
