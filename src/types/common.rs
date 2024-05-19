use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tx {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionResponse {
    pub tx: Tx,
    pub tx_signed: Tx,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub value: String,
    pub trait_type: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flagged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed_count: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_price24hr: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_all: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_badged: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub pda_address: String,
    pub auction_house: String,
    pub token_address: String,
    pub token_mint: String,
    pub seller: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_referral: Option<String>,
    pub token_size: i64,
    pub price: f64,
    pub rarity: Rarity,
    pub extra: Extra,
    pub expiry: i64,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Extra {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rarity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub howrare: Option<Howrare>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moonrank: Option<Moonrank>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merarity: Option<Merarity>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Howrare {
    pub rank: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Moonrank {
    pub crawl: Crawl,
    #[serde(rename = "absolute_rarity")]
    pub absolute_rarity: i32,
    pub rank: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Merarity {
    pub token_key: String,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_supply: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_with_counts: Option<Vec<AttributeWithCount>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Crawl {
    /// Flag for if Moonrank has fully crawled the collection. If this is false,
    /// then it means the rarity values are incomplete and should not be used.
    pub complete: bool,
    pub id: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeWithCount {
    pub trait_type: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

pub fn default_true() -> bool {
    true
}
