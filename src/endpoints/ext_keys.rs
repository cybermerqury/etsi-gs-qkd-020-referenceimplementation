use std::{collections::HashMap, time::Duration};

use actix_web::{post, web, HttpResponse, Responder};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    config::AppState,
    types::{ErrorResponse, KeyValueElement},
};

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

async fn call_ack(
    sleep_duration: Duration,
    app_state: web::Data<AppState>,
    request: ExtKeysRequest,
) {
    tokio::time::sleep(sleep_duration).await;

    println!("Calling ACK url {}", request.ack_callback_url);

    let ack_url_response = match app_state
        .callback_client
        .post(request.ack_callback_url)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            println!("Failed to send 'ack' request. Error: {:?}", e);

            return;
        }
    };

    let status = ack_url_response.status();
    if status != StatusCode::OK {
        println!("ACK url call ERR. Status: {:?}", ack_url_response.status());

        match ack_url_response.json::<ErrorResponse>().await {
            Ok(err) => {
                println!("ACK response body:\n\t{:#?}", err);
            }
            Err(e) => {
                println!("Could not parse ACK response. Error: {}, {:?}", e, e)
            }
        }

        return;
    }

    println!("ACK url call OK.")
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

    if valid_request {
        println!("Valid response, spawning worker.");

        let sleep_duration = Duration::from_secs(3);

        actix_web::rt::spawn(call_ack(sleep_duration, app_state, request_body.0));

        HttpResponse::Accepted().finish()
    } else {
        println!("Invalid response");
        let response_body = ErrorResponse {
            type_name: String::from("General Error."),
            status: 400,
            title: String::from("Error message"),
            details: HashMap::from([("Details 1".to_string(), "Details 1 message".to_string())]),
        };

        HttpResponse::InternalServerError().json(response_body)
    }
}
