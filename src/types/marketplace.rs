use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::MagicedenError;

use super::Collection;

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(name = "PopularCollectionsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MagicedenError"))]
pub struct PopularCollectionsRequest {
    /// The time range to return popular collections for. Default 1d.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
}

pub type PopularCollectionsResponse = Vec<Collection>;
