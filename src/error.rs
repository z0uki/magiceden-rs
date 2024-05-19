use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum MagicedenError {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{:?}: {}", .0.r#type, .0.message)]
    ApiError(ApiError),

    #[error("failed to deserialize api response: {0}")]
    JSONDeserialize(serde_json::Error),

    #[error("invalid args: {0}")]
    InvalidArgument(String),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub r#type: Option<String>,
    pub param: Option<serde_json::Value>,
    pub code: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WrappedError {
    pub(crate) error: ApiError,
}

pub(crate) fn map_deserialization_error(e: serde_json::Error, bytes: &[u8]) -> MagicedenError {
    tracing::error!(
        "failed deserialization of: {}",
        String::from_utf8_lossy(bytes.as_ref())
    );
    MagicedenError::JSONDeserialize(e)
}
