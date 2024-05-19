use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletInfoResponse {
    pub display_name: String,
    pub avatar: String,
}
