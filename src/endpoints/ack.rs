use std::{collections::HashMap, error::Error};

use actix_web::{post, web, HttpResponse, Responder};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::types::{ErrorResponse, KeyIdElement};

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
pub struct AckRequest {
    pub key_ids: Vec<KeyIdElement>,
    pub ack_status: AckStatus,
    pub initiator_sae_id: String,
    pub target_sae_id: String,
    pub message: String,
}

#[post("/kmapi/v1/ack")]
pub async fn ack(request_body: web::Json<AckRequest>) -> impl Responder {
    if let Some(err) = validate_request(&request_body) {
        return HttpResponse::BadRequest().json(err);
    }

    if let Err(e) = service_request(request_body.0).await {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16().into(),
            title: "Error processing ACK request".to_string(),
            type_name: StatusCode::INTERNAL_SERVER_ERROR.as_str().to_string(),
            details: HashMap::from_iter([("server_error".to_string(), e.to_string())]),
        });
    }

    HttpResponse::Ok().finish()
}

async fn service_request(request_body: AckRequest) -> Result<(), Box<dyn Error>> {
    println!(
        "ACK: Received request. status: {:?}, initiator: '{}', target: '{}', message: '{}'",
        request_body.ack_status,
        request_body.initiator_sae_id,
        request_body.target_sae_id,
        request_body.message
    );

    for key_id in &request_body.key_ids {
        println!("ACK: * Acknowledging key_id {}", key_id.key_id);
    }

    Ok(())
}

fn validate_request(request_body: &AckRequest) -> Option<ErrorResponse> {
    let mut error_details = HashMap::new();

    if request_body.key_ids.is_empty() {
        error_details.insert("no_key_ids", "No key_ids present in request body.");
    }

    if request_body.initiator_sae_id.is_empty() {
        error_details.insert(
            "no_initiator_sae_id",
            "No initiator_sae_id present in request body.",
        );
    }

    if request_body.target_sae_id.is_empty() {
        error_details.insert(
            "no_target_sae_id",
            "No target_sae_id present in response body.",
        );
    }

    if error_details.is_empty() {
        None
    } else {
        Some(ErrorResponse {
            status: 400,
            type_name: "bad_request".to_string(),
            title: "Invalid Request".to_string(),
            details: error_details
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        })
    }
}
