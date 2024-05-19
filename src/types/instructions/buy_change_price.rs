use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::MagicedenError;

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "InstructionsBuyChangePriceRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct InstructionsBuyChangePriceRequest {
    /// Buyer wallet.
    pub buyer: String,
    /// Auction house.
    pub auction_house_address: String,
    /// Token mint address.
    pub token_mint: String,
    /// Price in SOL.
    pub price: f64,
    /// New price in SOL
    pub new_price: f64,
    /// Buyer referral wallet. Option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_referral: Option<String>,
    /// timestamp in seconds in the future, 0 will default to 7 days. Option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionsBuyChangePriceResponse {
    //todo: add fields
}
