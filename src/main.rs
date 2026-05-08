use std::process::exit;

use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod, SslVerifyMode};
use ref_impl_lib::{
    config::{AppState, Config},
    endpoints::ext_keys::ext_keys,
};

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
