use crate::{
    error::MagicedenError,
    types::{PopularCollectionsRequest, PopularCollectionsResponse},
    Client,
};

pub struct Marketplace<'c> {
    pub client: &'c Client,
}

impl<'c> Marketplace<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get the top 50 popular collections for a time range
    /// @path: /marketplace/popular_collections
    pub async fn popular_collections(
        &self,
        request: PopularCollectionsRequest,
    ) -> Result<PopularCollectionsResponse, MagicedenError> {
        self.client
            .get_with_query("/marketplace/popular_collections", &request)
            .await
    }
}
