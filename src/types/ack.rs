// SPDX-FileCopyrightText: © 2026 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use std::ops::Deref;

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

#[derive(Deserialize, Serialize, Debug)]
pub struct AckRequest(pub Vec<AckContainer>);

/// The request body for an `ack` endpoint call.
#[derive(Deserialize, Serialize, Debug)]
pub struct AckContainer {
    pub key_ids: Vec<KeyIdElement>,
    pub ack_status: AckStatus,
    pub initiator_sae_id: String,
    pub target_sae_id: String,
    pub message: String,
}

impl Deref for AckRequest {
    type Target = Vec<AckContainer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
