use std::time::Duration;

use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{
    config::AppState,
    types::{ext_keys::ExtKeysRequest, KeyValueElement},
};

/// The ext_keys_subsystem sends out `ext_keys` requests at fixed intervals.
/// This emulates the behavior of a third-party KMS sending inbound requests of its own.
pub async fn ext_keys_subsystem(app_state: AppState) {
    let ss_config = app_state.config.send_ext_keys_config;

    // If disabled, sleep indefinitely.
    if !ss_config.enabled {
        debug!("Send ext_keys subsystem is disabled by config.");
        tokio::time::sleep(Duration::MAX).await;
        return;
    }

    let mut interval = tokio::time::interval(ss_config.dispatch_interval);
    let ext_keys_url = match ss_config.target_url.join("/kmapi/v1/ext_keys") {
        Ok(url) => url,
        Err(e) => {
            error!("Failed to construct ext_keys URL. Error: {e}");
            return;
        }
    };

    loop {
        interval.tick().await;

        let body = ExtKeysRequest {
            keys: vec![KeyValueElement {
                key_id: Uuid::now_v7(),
                value: "wHHVxRwDJs3/bXd38GHP3oe4svTuRpZS0yCC7x4Ly+s=".to_string(),
            }],
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
