use crate::{
    error::MagicedenError,
    types::{
        InstructionResponse, InstructionsBuyCancelRequest, InstructionsBuyChangePriceRequest,
        InstructionsBuyNowRequest, InstructionsBuyNowTransferNftRequest, InstructionsBuyRequest,
        InstructionsDepositRequest, InstructionsSellCancelRequest,
        InstructionsSellChangePriceRequest, InstructionsSellNowRequest, InstructionsSellRequest,
        InstructionsWithdrawRequest,
    },
    Client,
};

pub struct Instructions<'c> {
    pub client: &'c Client,
}

impl<'c> Instructions<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get instruction to buy (bid)
    /// @path: /instructions/buy
    pub async fn buy(
        &self,
        request: InstructionsBuyRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/buy", &request)
            .await
    }

    /// Get instruction to buy now
    /// @path: /instructions/buy_now
    pub async fn buy_now(
        &self,
        request: InstructionsBuyNowRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/buy_now", &request)
            .await
    }

    /// Get instruction to buy now and transfer nft to another owner
    /// @path: /instructions/buy_now_transfer_nft
    pub async fn buy_now_transfer_nft(
        &self,
        request: InstructionsBuyNowTransferNftRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/buy_now_transfer_nft", &request)
            .await
    }

    /// Get instruction to cancel a buy
    /// @path: /instructions/buy_cancel
    pub async fn buy_cancel(
        &self,
        request: InstructionsBuyCancelRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/buy_cancel", &request)
            .await
    }

    /// Get instruction to change a buy price
    /// @path: /instructions/buy_change_price
    pub async fn buy_change_price(
        &self,
        request: InstructionsBuyChangePriceRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/buy_change_price", &request)
            .await
    }

    /// Get instruction to sell (list)
    /// @path: /instructions/sell
    pub async fn sell(
        &self,
        request: InstructionsSellRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/sell", &request)
            .await
    }

    /// Get instruction to change a sell price
    /// @path: /instructions/sell_change_price
    pub async fn sell_change_price(
        &self,
        request: InstructionsSellChangePriceRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/sell_change_price", &request)
            .await
    }

    /// Get instruction to sell now (accept offer)
    /// @path: /instructions/sell_now
    pub async fn sell_now(
        &self,
        request: InstructionsSellNowRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/sell_now", &request)
            .await
    }

    /// Get instruction to cancel a sell
    /// @path: /instructions/sell_cancel
    pub async fn sell_cancel(
        &self,
        request: InstructionsSellCancelRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/sell_cancel", &request)
            .await
    }

    /// Get instruction to deposit to escrow
    /// @path: /instructions/deposit
    pub async fn deposit(
        &self,
        request: InstructionsDepositRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/deposit", &request)
            .await
    }

    /// Get instruction to withdraw from escrow
    /// @path: /instructions/deposit
    pub async fn withdraw(
        &self,
        request: InstructionsWithdrawRequest,
    ) -> Result<InstructionResponse, MagicedenError> {
        self.client
            .get_with_query("/instructions/withdraw", &request)
            .await
    }
}
