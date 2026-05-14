use serde::{Deserialize, Serialize};

use super::KeyIdElement;

#[derive(Deserialize, Serialize, Debug)]
pub enum AckStatus {
    #[serde(rename = "relayed")]
    Relayed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "voided")]
    Voided,
    #[serde(rename = "failed to void")]
    FailedToVoid,
    #[serde(rename = "key not present")]
    KeyNotPresent,
}

/// The request body for an `ack` endpoint call.
#[derive(Deserialize, Serialize, Debug)]
pub struct AckRequest {
    pub key_ids: Vec<KeyIdElement>,
    pub ack_status: AckStatus,
    pub initiator_sae_id: String,
    pub target_sae_id: String,
    pub message: String,
}
