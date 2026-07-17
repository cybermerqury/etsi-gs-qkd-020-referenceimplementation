// SPDX-FileCopyrightText: © 2026 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use std::error::Error;

use base64::{Engine, engine::general_purpose::STANDARD};
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{
    config::AppState,
    types::{KeyValueElement, ext_keys::ExtKeysRequest},
};

/// The ext_keys_subsystem sends out `ext_keys` requests at fixed intervals.
/// This emulates the behavior of a third-party KMS sending inbound requests of its own.
pub async fn ext_keys_subsystem(app_state: AppState) -> Result<(), Box<dyn Error>> {
    let ss_config = app_state.config.send_ext_keys_config;

    if !ss_config.enabled {
        debug!("Send ext_keys subsystem is disabled by config.");

        return Ok(());
    }

    let mut interval = tokio::time::interval(ss_config.dispatch_interval);
    let ext_keys_url = ss_config
        .target_url
        .join("/kmapi/v1/ext_keys")
        .inspect_err(|e| error!("Failed to construct ext_keys URL. Error: {e}"))?;

    // Remove the first tick since it happens instantaneously.
    interval.tick().await;

    loop {
        interval.tick().await;

        // Construct a vector of key-values of the required quantity and key size.
        let keys = (0..ss_config.key_count)
            .into_iter()
            .map(|_| {
                let mut val = vec![0u8; (ss_config.key_size_bits / 8) as usize];
                rand::fill(&mut val[..]);

                KeyValueElement {
                    key_id: Uuid::now_v7(),
                    value: STANDARD.encode(val),
                }
            })
            .collect();

        let body = ExtKeysRequest {
            keys,
            ack_callback_url: ss_config.ack_url.clone(),
            extension_mandatory: None,
            extension_optional: None,
            initiator_sae_id: app_state.config.intra_network_sae_ids[0].clone(),
            target_sae_ids: vec![app_state.config.third_party_sae_ids[0].clone()],
        };

        info!(
            "Sending ext_keys request to {}. Body:\n\t{:?}",
            ext_keys_url, body
        );

        let result = app_state
            .callback_client
            .ext_keys_async(ext_keys_url.clone(), body)
            .await;

        match result {
            Ok(()) => info!("EXT_KEYS response: ACCEPTED."),
            Err(e) => info!("Failed to call EXT_KEYS. Error: {e}"),
        }
    }
}
