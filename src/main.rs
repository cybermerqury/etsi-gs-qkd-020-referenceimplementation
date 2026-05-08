mod config;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use base64::Engine;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod, SslVerifyMode};
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

#[post("/kmapi/v1/ext_keys")]
async fn ext_keys(request_body: web::Json<ExtKeysRequest>) -> impl Responder {
    println!("Request received: {request_body:?}.");

    let mut valid_request = request_body.target_sae_ids.len() == 1;
    valid_request = valid_request && (!request_body.initiator_sae_id.is_empty());
    valid_request = valid_request
        && ((request_body.keys.len() == 1)
            && is_key_value_valid(request_body.keys[0].value.as_str()));

    if valid_request {
        println!("Valid response");
        HttpResponse::Ok().finish()
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

    println!("Listening on {}:{}", config.ip_addr, config.port_num);

    HttpServer::new(|| App::new().service(ext_keys))
        .bind_openssl(
            (config.ip_addr.as_str(), config.port_num),
            build_tls_configuration(&config),
        )?
        .run()
        .await
}
