// SPDX-FileCopyrightText: © 2026 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::types::KeyValueElement;

/// The request body for an `ext_keys` call.
#[derive(Deserialize, Serialize, Debug)]
pub struct ExtKeysRequest {
    pub keys: Vec<KeyValueElement>,
    pub initiator_sae_id: String,
    pub target_sae_ids: Vec<String>,
    pub ack_callback_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_mandatory: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_optional: Option<Value>,
}
