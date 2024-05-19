use crate::{error::MagicedenError, types::TokenListingsResponse, Client};

pub struct Tokens<'c> {
    pub client: &'c Client,
}

impl<'c> Tokens<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get listings for a token
    /// @path: /tokens/{token_mint}/listings
    pub async fn listings(
        &self,
        token_mint: &str,
    ) -> Result<TokenListingsResponse, MagicedenError> {
        self.client
            .get(&format!("/tokens/{token_mint}/listings"))
            .await
    }

    // Get received offers for a token
    // @path: /tokens/{token_mint}/offers_received

    // Get activities for a token
    // @path: /tokens/{token_mint}/activities

    // Get token metadata by mint address
    // @path: /tokens/{token_mint}
}
