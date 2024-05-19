use crate::{
    error::MagicedenError,
    types::{
        CollectionActivitiesRequest, CollectionActivitiesResponse, CollectionListingsRequest,
        CollectionListingsResponse, CollectionStatsResponse, CollectionsRequest,
        CollectionsResponse, HolderStatsResponse,
    },
    Client,
};

pub struct Collections<'c> {
    pub client: &'c Client,
}

impl<'c> Collections<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get activities of a collection
    /// @path: /collections/{symbol}/activities
    pub async fn activities(
        &self,
        symbol: &str,
        request: CollectionActivitiesRequest,
    ) -> Result<CollectionActivitiesResponse, MagicedenError> {
        self.client
            .get_with_query(&format!("/collections/{symbol}/activities"), &request)
            .await
    }

    /// Get stats of a collection
    /// @path: /collections/{symbol}/stats
    pub async fn stats(&self, symbol: &str) -> Result<CollectionStatsResponse, MagicedenError> {
        self.client
            .get(&format!("/collections/{symbol}/stats"))
            .await
    }

    /// Get collections
    /// @path: /collections
    pub async fn collections(
        &self,
        request: CollectionsRequest,
    ) -> Result<CollectionsResponse, MagicedenError> {
        self.client.get_with_query("/collections", &request).await
    }

    /// Get listings of a collection
    /// @path: /collections/{symbol}/listings
    pub async fn listings(
        &self,
        symbol: &str,
        request: CollectionListingsRequest,
    ) -> Result<CollectionListingsResponse, MagicedenError> {
        self.client
            .get_with_query(&format!("/collections/{symbol}/listings"), &request)
            .await
    }

    /// Get holder stats of a collection
    /// @path: /collections/{symbol}/holder_stats
    pub async fn holder_stats(&self, symbol: &str) -> Result<HolderStatsResponse, MagicedenError> {
        self.client
            .get(&format!("/collections/{symbol}/holder_stats"))
            .await
    }
}
