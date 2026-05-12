use std::time::Duration;

use actix_web::{post, web, HttpResponse, Responder};
use reqwest::StatusCode;

use crate::{
    config::AppState,
    types::{
        ack::{AckRequest, AckStatus},
        ext_keys::ExtKeysRequest,
        ErrorResponse, KeyIdElement,
    },
};

async fn call_ack(
    sleep_duration: Duration,
    app_state: web::Data<AppState>,
    request: ExtKeysRequest,
) {
    tokio::time::sleep(sleep_duration).await;

    println!("Calling ACK url {}", request.ack_callback_url);

    let key_ids = request
        .keys
        .into_iter()
        .map(|kv| KeyIdElement { key_id: kv.key_id })
        .collect();

    let ack_body = AckRequest {
        ack_status: AckStatus::Relayed,
        initiator_sae_id: request.initiator_sae_id,
        target_sae_id: request.target_sae_ids[0].clone(),
        message: "TEST".to_string(),
        key_ids: key_ids,
    };

    match app_state
        .callback_client
        .ack(request.ack_callback_url, ack_body)
        .await
    {
        Ok(()) => println!("ext_keys servicing concluded."),
        Err(e) => println!("Error calling 'ack' for 'ext_keys'. Error: {e}"),
    }
}

#[post("/kmapi/v1/ext_keys")]
async fn ext_keys(
    request_body: web::Json<ExtKeysRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    println!("Request received: {request_body:?}.");

    let mut valid_request = request_body.target_sae_ids.len() == 1;
    valid_request = valid_request && (!request_body.initiator_sae_id.is_empty());
    valid_request = valid_request
        && ((request_body.keys.len() == 1) && request_body.keys[0].is_key_value_valid());

    if !app_state
        .config
        .third_party_sae_ids
        .contains(&request_body.initiator_sae_id)
    {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "Invalid parameter",
            400,
            "Bad initiator_sae_id. Passed SAE ID is not recognized.",
            [("initiator_sae_id", &request_body.initiator_sae_id)],
        ));
    }

    let missing_target_sae_ids = request_body
        .target_sae_ids
        .iter()
        .filter(|sae_id| !app_state.config.intra_network_sae_ids.contains(&sae_id))
        .map(String::as_str)
        .collect::<Vec<_>>();

    if missing_target_sae_ids.len() > 0 {
        return HttpResponse::BadRequest().json(ErrorResponse::from_status_code(
            StatusCode::BAD_REQUEST,
            "Bad target_sae_ids. This instance is not configured for one or more of the supplied SAE IDs.",
            [("target_sae_ids", missing_target_sae_ids.join(","))]
        ));
    }

    if valid_request {
        println!("Valid response, spawning worker.");

        let sleep_duration = Duration::from_secs(3);

        actix_web::rt::spawn(call_ack(sleep_duration, app_state, request_body.0));

        HttpResponse::Accepted().finish()
    } else {
        println!("Invalid response");
        let response_body = ErrorResponse::new(
            "General Error",
            400,
            "Error message",
            [("Details 1", "Details 1 message")],
        );

        HttpResponse::InternalServerError().json(response_body)
    }
}
