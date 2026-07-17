// SPDX-FileCopyrightText: © 2026 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use std::{collections::HashMap, time::Duration};

use actix_web::{HttpResponse, Responder, post, web};
use reqwest::StatusCode;
use tracing::{error, info};

use crate::{
    config::{AppState, Config},
    types::{
        ErrorResponse, KeyIdElement,
        ack::{AckContainer, AckRequest, AckStatus},
        ext_keys::ExtKeysRequest,
    },
};

#[post("/kmapi/v1/ext_keys")]
async fn ext_keys(
    request_body: web::Json<ExtKeysRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    info!("Request received: {request_body:?}.");

    if let Err(validation_err) = validate_request(&app_state.config, &request_body) {
        error!("EXT_KEYS: Validation error in request.");
        return HttpResponse::BadRequest().json(validation_err);
    }

    info!("Valid response, spawning worker.");

    actix_web::rt::spawn(call_ack(
        app_state.config.ack_callback_delay,
        app_state,
        request_body.0,
    ));

    HttpResponse::Accepted().finish()
}

/// Validate the `ext_keys` request body.
fn validate_request(config: &Config, request_body: &ExtKeysRequest) -> Result<(), ErrorResponse> {
    let mut err_details: HashMap<String, String> = HashMap::new();

    if request_body.target_sae_ids.is_empty() {
        err_details.insert(
            "target_sae_ids".to_string(),
            "Empty target_sae_ids array.".to_string(),
        );
    }

    if request_body.initiator_sae_id.trim().is_empty() {
        err_details.insert(
            "target_sae_ids".to_string(),
            "Empty initiator_sae_ids.".to_string(),
        );
    }

    if request_body.keys.is_empty() {
        err_details.insert("keys".to_string(), "Empty keys array".to_string());
    }

    request_body
        .keys
        .iter()
        .filter_map(|key| Some(key.is_key_value_valid().err()?.details))
        .flatten()
        .for_each(|(k, v)| {
            err_details.insert(k, v);
        });

    if !err_details.is_empty() {
        return Err(ErrorResponse::from_status_code(
            StatusCode::BAD_REQUEST,
            "Validation failed",
            err_details,
        ));
    }

    if !config
        .intra_network_sae_ids
        .contains(&request_body.initiator_sae_id)
    {
        return Err(ErrorResponse::new(
            "Invalid parameter",
            400,
            "Invalid initiator_sae_id. Passed SAE ID is not recognized.",
            [("initiator_sae_id", &request_body.initiator_sae_id)],
        ));
    }

    let missing_target_sae_ids = request_body
        .target_sae_ids
        .iter()
        .filter(|sae_id| !config.third_party_sae_ids.contains(&sae_id))
        .map(String::as_str)
        .collect::<Vec<_>>();

    if missing_target_sae_ids.len() > 0 {
        return Err(ErrorResponse::from_status_code(
            StatusCode::BAD_REQUEST,
            "Invalid target_sae_ids. This instance is not configured for one or more of the supplied SAE IDs.",
            [("target_sae_ids", missing_target_sae_ids.join(","))],
        ));
    }

    Ok(())
}

/// Sleep for a given duration, then issue an `ack` callback request.
async fn call_ack(
    sleep_duration: Duration,
    app_state: web::Data<AppState>,
    request: ExtKeysRequest,
) {
    tokio::time::sleep(sleep_duration).await;

    info!("Calling ACK url {}", request.ack_callback_url);

    let key_ids: Vec<_> = request
        .keys
        .into_iter()
        .map(|kv| KeyIdElement { key_id: kv.key_id })
        .collect();
    let initiator = request.initiator_sae_id.clone();

    let ack_body = AckRequest(
        request
            .target_sae_ids
            .iter()
            .map(move |target| AckContainer {
                ack_status: AckStatus::Relayed,
                initiator_sae_id: initiator.clone(),
                target_sae_id: target.clone(),
                message: "TEST".to_string(),
                key_ids: key_ids.clone(),
            })
            .collect(),
    );

    match app_state
        .callback_client
        .ack(request.ack_callback_url, ack_body)
        .await
    {
        Ok(()) => info!("ext_keys servicing concluded."),
        Err(e) => error!("Error calling 'ack' for 'ext_keys'. Error: {e}"),
    }
}
