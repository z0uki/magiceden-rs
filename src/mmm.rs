use crate::{
    error::MagicedenError,
    types::{
        InstructionResponse, MmmCreatePoolRequest, MmmPoolsRequest, MmmPoolsResponse,
        MmmSolFulfillBuyRequest, MmmSolFulfillSellRequest, MmmSolWithdrawBuyRequest,
        MmmTokenPoolsRequest, MmmTokenPoolsResponse,
    },
    Client,
};

pub struct Mmm<'c> {
    pub client: &'c Client,
}

impl<'c> Mmm<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get AMM pools corresponding to an owner or collection symbol. At least one of required collectionSymbol or owner is required!
    /// @path: /mmm/pools
    pub async fn pools(
        &self,
        request: MmmPoolsRequest,
    ) -> Result<MmmPoolsResponse, MagicedenError> {
        self.client.get_with_query("/mmm/pools", &request).await
    }

    /// Get best offers for an NFT
    /// @path: /token/{mint_address}/pools
    pub async fn token_pools(
        &self,
        mint_address: &str,
        request: MmmTokenPoolsRequest,
    ) -> Result<MmmTokenPoolsResponse, MagicedenError> {
        self.client
            .get_with_query(&format!("/token/{mint_address}/pools"), &request)
            .await
    }

    /// Get instruction to create a pool
    /// @path: /instructions/mmm/create-pool
    pub async fn create_pool(
        &self,
        request: MmmCreatePoolRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/mmm/create-pool", &request)
            .await
    }

    /// Get instruction to withdraw sol payment from a pool
    /// @path: /instructions/mmm/sol-withdraw-buy
    pub async fn sol_withdraw_buy(
        &self,
        request: MmmSolWithdrawBuyRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/mmm/sol-withdraw-buy", &request)
            .await
    }

    /// Get instruction to have a pool fulfill a buy
    /// @path: /instructions/mmm/sol-fulfill-buy
    pub async fn sol_fulfill_buy(
        &self,
        request: MmmSolFulfillBuyRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/mmm/sol-fulfill-buy", &request)
            .await
    }

    /// Get instruction to have a pool fulfill a sell
    /// @path: /instructions/mmm/sol-fulfill-sell
    pub async fn sol_fulfill_sell(
        &self,
        request: MmmSolFulfillSellRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/mmm/sol-fulfill-sell", &request)
            .await
    }
}
