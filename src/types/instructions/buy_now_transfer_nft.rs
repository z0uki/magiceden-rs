use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{error::MagicedenError, types::default_true};

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "InstructionsBuyNowTransferNftRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct InstructionsBuyNowTransferNftRequest {
    /// Buyer wallet.
    pub buyer: String,
    /// Seller wallet.
    pub seller: String,
    /// Auction house.
    pub auction_house_address: String,
    /// Token mint address.
    pub token_mint: String,
    /// Associate Token Account
    #[serde(rename = "tokenATA")]
    pub token_ata: String,
    /// Price in SOL.
    pub price: f64,
    /// Associated token account to send bought NFT to
    #[serde(rename = "destinationATA")]
    pub destination_ata: String,
    /// Owner of token account
    #[serde(rename = "destinationOwner")]
    pub destination_owner: String,
    /// whether to include create ATA instructions
    #[serde(rename = "createATA", default = "default_true")]
    pub create_ata: bool,
    /// Buyer referral wallet. Option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_referral: Option<String>,
    /// Seller referral wallet. Option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_referral: Option<String>,
    /// timestamp in seconds in the future, 0 means no expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_expiry: Option<i64>,
    /// timestamp in seconds in the future, 0 means no expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_expiry: Option<i64>,
    /// buyerCreatorRoyaltyPercent, integer 0-100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_creator_royalty_percent: Option<u8>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionsBuyNowTransferNftResponse {
    //todo: add fields
}
