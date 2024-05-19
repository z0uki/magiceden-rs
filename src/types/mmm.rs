use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::MagicedenError;

use super::{default_true, Attribute};

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "MmmPoolsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct MmmPoolsRequest {
    /// The collection symbol to query the pools from. At least this or "owner" must be passed in
    pub collection_symbol: Option<String>,
    /// The owner public key to query the pools created by. At least this or "collectionSymbol" must be passed in
    pub owner: Option<String>,
    /// The number of items to skip, default 0, min 0
    pub offset: Option<i64>,
    /// The numbers of items to return, default 100, min 1, max 500
    pub limit: Option<i64>,
    /// Sort pools by field. 0: NONE (no sorting field, default), 1: ADDRESS (sort pools by pool address), 2: SPOT_PRICE (sort pools by pool spot price), 5: BUYSIDE_ADJUSTED_PRICE (sort pools by buyside_adjusted_price = spot_price - royalty_fee - lp_fee)
    pub field: Option<u8>,
    /// Sort pools by field in specified directions. 0: NONE (no sorting direction, default to increasing order if field is specified), 1: DESC (descending), 2: INC (increasing)
    pub direction: Option<u8>,
}

pub type MmmPoolsResponse = Vec<MmmPool>;

#[derive(Default, Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum CurveType {
    // default: Linear
    #[default]
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "exp")]
    Exp,
    #[serde(rename = "xyk")]
    Xyk,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum PoolType {
    #[serde(rename = "buy_sided")]
    BuySided,
    #[serde(rename = "sell_sided")]
    SellSided,
    #[serde(rename = "two_sided")]
    TwoSided,
    #[serde(rename = "invalid")]
    Invalid,
}

/// Construct a type with the properties of T except for those in type K.
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MmmPool {
    pub spot_price: f64,
    pub curve_type: CurveType,
    pub curve_delta: f64,
    pub reinvest_fulfill_buy: bool,
    pub reinvest_fulfill_sell: bool,
    pub expiry: i64,
    pub lp_fee_bp: i64,
    pub buyside_creator_royalty_bp: i64,
    pub pool_owner: String,
    pub sellside_asset_amount: i64,
    pub buyside_payment_amount: i64,
    pub buy_orders_amount: i64,
    pub collection_symbol: String,
    pub collection_name: String,
    pub pool_type: PoolType,
    pub uuid: String,
    pub pool_key: String,
    pub cosigner: String,
    pub attributes: Option<Vec<Attribute>>,
    pub blocked_at: Option<String>,
    pub mints: Option<Vec<String>>,
    pub collection_seller_fee_basis_points: i64,
    pub lp_fee_earned: i64,
    pub buy_price_taker: Option<f64>,
    #[serde(rename = "isMIP1")]
    pub is_mip1: Option<bool>,
    #[serde(rename = "isOCP")]
    pub is_ocp: Option<bool>,
    pub updated_at: String,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "MmmTokenPoolsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct MmmTokenPoolsRequest {
    /// Total best offers to return, defaults 1, max 5. Best offer will come first
    pub limit: Option<i64>,
}

pub type MmmTokenPoolsResponse = Vec<MmmPool>;

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "MmmCreatePoolRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct MmmCreatePoolRequest {
    /// Pool initial spot price in SOL
    pub spot_price: f64,
    /// Type curve, either 'linear' or 'exp'
    pub curve_type: CurveType,
    /// Curve delta used to change price after a fill
    pub curve_delta: i64,
    /// Whether to reinvest bought asset or transfer directly to owner
    #[serde(default = "default_true")]
    pub reinvest_buy: bool,
    /// Whether to reinvest payment from sale or transfer directly to owner
    #[serde(default = "default_true")]
    pub reinvest_sell: bool,
    /// Timestamp in seconds in the future, 0 means no expiry
    pub expiry: Option<i64>,
    /// Requested liquidity provider fee in basis points
    pub lp_fee_bp: i64,
    /// Amount of creator royalty the pool should pay in basis points
    pub buyside_creator_royalty_bp: i64,
    /// Mint address of payment (default for SOL)
    pub payment_mint: String,
    /// Collection symbol for which the pool will be valid
    pub collection_symbol: String,
    /// Owner of the pool
    pub owner: String,
    /// Optional sol amount to deposit with pool creation
    pub sol_deposit: Option<f64>,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "MmmSolWithdrawBuyRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct MmmSolWithdrawBuyRequest {
    /// Public key of pool to withdraw from
    pub pool: String,
    /// The of SOL to withdraw
    pub payment_amount: f64,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "MmmSolFulfillBuyRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct MmmSolFulfillBuyRequest {
    /// Public key of pool to interact with
    pub pool: String,
    /// Amount of asset to transact
    pub asset_amount: f64,
    /// Minimum payment amount acceptible by the seller, in SOL
    pub min_payment_amount: f64,
    /// Public key of seller of asset
    pub seller: String,
    /// Public key of mint account of asset
    pub asset_mint: String,
    /// Public key of token account of asset
    pub asset_token_account: String,
    /// The allowlist aux account used for token authentication
    pub allowlist_aux_account: Option<String>,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "MmmSolFulfillSellRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct MmmSolFulfillSellRequest {
    /// Public key of pool to interact with
    pub pool: String,
    /// Amount of asset to transact
    pub asset_amount: f64,
    /// Maximum payment amount to be paid by the buyer, in SOL
    pub max_payment_amount: f64,
    /// Amount of royalty to be paid, in basis points of total royalty
    pub buyside_creator_royalty_bp: i64,
    /// Public key of buyer of asset
    pub buyer: String,
    /// Public key of mint account of asset
    pub asset_mint: String,
    /// The allowlist aux account used for token authentication
    pub allowlist_aux_account: Option<String>,
}
