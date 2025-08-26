use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use base64::Engine;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod, SslVerifyMode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct KeyElement {
    key_id: String,
    value: String,
}

#[derive(Deserialize)]
struct RequestBody {
    keys: Vec<KeyElement>,
    initiator_sae_id: String,
    target_sae_ids: Vec<String>,
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

fn build_tls_configuration() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_modern_v5(SslMethod::tls()).unwrap();

    builder.set_ca_file("certificates/root.crt").unwrap();
    builder
        .set_private_key_file("certificates/gateway_1.key", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("certificates/gateway_1.crt")
        .unwrap();
    builder.set_verify(SslVerifyMode::PEER | SslVerifyMode::FAIL_IF_NO_PEER_CERT);

    builder
}

#[post("/kmapi/v1/ext_keys")]
async fn ext_keys(request_body: web::Json<RequestBody>) -> impl Responder {
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
    HttpServer::new(|| App::new().service(ext_keys))
        .bind_openssl(("127.0.0.1", 8080), build_tls_configuration())?
        .run()
        .await
}
