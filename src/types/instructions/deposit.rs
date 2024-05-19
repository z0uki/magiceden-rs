use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::MagicedenError;

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "InstructionsDepositRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct InstructionsDepositRequest {
    /// Buyer wallet.
    pub buyer: String,
    /// Auction house.
    pub auction_house_address: String,
    /// Amount in SOL
    pub amount: f64,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionsDepositResponse {
    //todo: add fields
}
