use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::KeyValueElement;

/// The request body for an `ext_keys` call.
#[derive(Deserialize, Serialize, Debug)]
pub struct ExtKeysRequest {
    pub keys: Vec<KeyValueElement>,
    pub initiator_sae_id: String,
    pub target_sae_ids: Vec<String>,
    pub ack_callback_url: Url,
    pub extension_mandatory: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_optional: Option<serde_json::Value>,
}
