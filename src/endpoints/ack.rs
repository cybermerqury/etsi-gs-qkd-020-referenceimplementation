use std::{collections::HashMap, error::Error};

use actix_web::{post, web, HttpResponse, Responder};
use reqwest::StatusCode;
use tracing::info;

use crate::types::{ack::AckRequest, ErrorResponse};

#[post("/kmapi/v1/ext_keys/ack")]
pub async fn ack(request_body: web::Json<AckRequest>) -> impl Responder {
    if let Err(err) = validate_request(&request_body) {
        return HttpResponse::BadRequest().json(err);
    }

    if let Err(e) = service_request(request_body.0).await {
        return HttpResponse::InternalServerError().json(ErrorResponse::from_status_code(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error processing ACK request",
            [("server_error", e)],
        ));
    }

    HttpResponse::Ok().finish()
}

async fn service_request(request_body: AckRequest) -> Result<(), Box<dyn Error>> {
    info!(
        "ACK: Request received. {} total containers to process.",
        request_body.len()
    );

    for (i, container) in request_body.iter().enumerate() {
        info!(
            "ACK: container {i} status: {:?}, initiator: '{}', target: '{}', message: '{}'",
            container.ack_status,
            container.initiator_sae_id,
            container.target_sae_id,
            container.message
        );

        for key_id in &container.key_ids {
            info!("ACK: * Acknowledging key_id {}", key_id.key_id);
        }
    }

    Ok(())
}

/// Validate the `ack` request body. If valid, return `None`, else return `Some`.
fn validate_request(request_body: &AckRequest) -> Result<(), ErrorResponse> {
    if request_body.is_empty() {
        return Err(ErrorResponse::from_status_code(
            StatusCode::BAD_REQUEST,
            "Empty ack array.",
            [(
                "ack_container_array",
                "At least one ack_container is required",
            )],
        ));
    }

    for container in request_body.iter() {
        let mut error_details = HashMap::new();

        if container.key_ids.is_empty() {
            error_details.insert("no_key_ids", "No key_ids present in request body.");
        }

        if container.initiator_sae_id.is_empty() {
            error_details.insert(
                "no_initiator_sae_id",
                "No initiator_sae_id present in request body.",
            );
        }

        if container.target_sae_id.is_empty() {
            error_details.insert(
                "no_target_sae_id",
                "No target_sae_id present in response body.",
            );
        }

        if !error_details.is_empty() {
            return Err(ErrorResponse::from_status_code(
                StatusCode::BAD_REQUEST,
                "Invalid request",
                error_details,
            ));
        }
    }

    Ok(())
}
