use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::MagicedenError;

use super::{Attribute, Collection, Listing};

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "CollectionActivitiesRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct CollectionActivitiesRequest {
    /// The number of items to skip, default 0, min 0
    pub offset: Option<i64>,
    /// The numbers of items to return, default 100, min 1, max 1000
    pub limit: Option<i64>,
}

pub type CollectionActivitiesResponse = Vec<CollectionActivitiy>;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionActivitiy {
    pub signature: String,
    pub r#type: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_symbol: Option<String>,
    pub slot: i64,
    pub block_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer: Option<String>,
    pub buyer_referral: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_referral: Option<String>,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionStatsResponse {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_price24hr: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_all: Option<f64>,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "CollectionsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct CollectionsRequest {
    /// The number of items to skip, default 0, min 0
    pub offset: Option<i64>,
    /// The numbers of items to return, default 100, min 1, max 1000
    pub limit: Option<i64>,
}

pub type CollectionsResponse = Vec<Collection>;

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "CollectionListingsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct CollectionListingsRequest {
    /// The number of items to skip, default 0, min 0
    pub offset: Option<i64>,
    /// The numbers of items to return, default 100, min 1, max 1000
    pub limit: Option<i64>,
    /// Filter listings that are less than this price
    pub min_price: Option<f64>,
    /// Filter listings that are more than this price
    pub max_price: Option<f64>,
    /// Represents a filtering mechanism where the elements within each inner array are logically ANDed, and the resulting arrays are ORed together at the top level. Each inner array consists of objects with two properties: traitType (a string) and value (a string).
    pub attributes: Option<Vec<Attribute>>,
    /// The field to sort the listings, default 'listPrice'. [listPrice, updatedAt]
    pub sort: Option<String>,
    /// The direction returned elements should be sorted in, default 'asc'. [asc, desc]
    pub sort_direction: Option<String>,
}

pub type CollectionListingsResponse = Vec<Listing>;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderStatsResponse {
    pub symbol: String,
    pub total_supply: Option<i32>,
    pub unique_holders: Option<i32>,
    // todo: add holder stats
}
