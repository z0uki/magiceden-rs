use crate::{error::MagicedenError, types::WalletInfoResponse, Client};

pub struct Wallets<'c> {
    pub client: &'c Client,
}

impl<'c> Wallets<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get info about the wallet owner.
    /// @path: /wallets/{wallet_address}
    pub async fn info(&self, wallet_address: &str) -> Result<WalletInfoResponse, MagicedenError> {
        self.client.get(&format!("/wallets/{wallet_address}")).await
    }
}
