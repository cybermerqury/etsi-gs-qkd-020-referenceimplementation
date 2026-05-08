mod config;

use std::{error::Error, process::exit, time::Duration};

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use base64::Engine;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod, SslVerifyMode};
use reqwest::{Certificate, Client, Identity};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::config::Config;

#[derive(Deserialize, Serialize, Debug)]
pub struct KeyValueElement {
    pub key_id: Uuid,
    pub value: String, // This is a base64 string
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtKeysRequest {
    pub keys: Vec<KeyValueElement>,
    pub initiator_sae_id: String,
    pub target_sae_ids: Vec<String>,
    pub ack_callback_url: Url,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    details: Vec<String>,
}

#[derive(Clone)]
struct AppState {
    callback_client: Client,
}

impl AppState {
    fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let root_cert = Certificate::from_pem(&std::fs::read(&config.root_cert)?)?;
        let client_cert = std::fs::read(&config.public_cert)?;
        let client_key = std::fs::read(&config.private_key)?;
        let identity = Identity::from_pem(&[client_cert, client_key].concat())?;

        let client = reqwest::Client::builder()
            .tls_certs_only([root_cert])
            .identity(identity)
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {
            callback_client: client,
        })
    }
}

fn is_key_value_valid(key_value: &str) -> bool {
    match base64::engine::general_purpose::STANDARD.decode(key_value) {
        Ok(decoded_key) => {
            decoded_key.len() == 32 // Keys must be 256bits long
        }
        Err(_) => false,
    }
}

fn build_tls_configuration(config: &Config) -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_modern_v5(SslMethod::tls()).unwrap();

    builder.set_ca_file(&config.root_cert).unwrap();
    builder
        .set_private_key_file(&config.private_key, SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(&config.public_cert)
        .unwrap();
    builder.set_verify(SslVerifyMode::PEER | SslVerifyMode::FAIL_IF_NO_PEER_CERT);

    builder
}

async fn call_ack(
    sleep_duration: Duration,
    app_state: web::Data<AppState>,
    request: ExtKeysRequest,
) {
    tokio::time::sleep(sleep_duration).await;

    println!("Calling ACK url {}", request.ack_callback_url);

    let ack_url_response = app_state
        .callback_client
        .post(request.ack_callback_url)
        .send()
        .await
        .map(|r| r.error_for_status());

    match ack_url_response {
        Ok(Ok(response)) => {
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Couldn't extract text from response.".to_string());

            println!("ACK url call OK. Response: {text}")
        }
        Ok(Err(e)) => {
            println!("ACK url call ERR. Status: {:?}", e.status());
        }
        Err(e) => {
            println!(
                "Failed to call ACK url. Error: '{:?}', status: {:?}",
                e.source(),
                e.status()
            )
        }
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
        && ((request_body.keys.len() == 1)
            && is_key_value_valid(request_body.keys[0].value.as_str()));

    if valid_request {
        println!("Valid response, spawning worker.");

        let sleep_duration = Duration::from_secs(3);

        actix_web::rt::spawn(call_ack(sleep_duration, app_state, request_body.0));

        HttpResponse::Accepted().finish()
    } else {
        println!("Invalid response");
        let response_body = ErrorResponse {
            message: String::from("Error message"),
            details: vec![String::from("Details 1"), String::from("Details 2")],
        };

        HttpResponse::InternalServerError().json(response_body)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    let app_state = match AppState::new(&config) {
        Ok(state) => state,
        Err(e) => {
            println!("Failed to initialise app state. Exiting. Error: {e}");
            exit(1);
        }
    };

    println!("Listening on {}:{}", config.ip_addr, config.port_num);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(ext_keys)
    })
    .bind_openssl(
        (config.ip_addr.as_str(), config.port_num),
        build_tls_configuration(&config),
    )?
    .run()
    .await
}
