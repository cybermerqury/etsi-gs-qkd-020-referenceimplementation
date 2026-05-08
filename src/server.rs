use std::{net::ToSocketAddrs, path::Path};

use actix_web::{dev::Server, web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod, SslVerifyMode};

use crate::{
    config::AppState,
    endpoints::{ack::ack, ext_keys::ext_keys},
};

pub fn build_tls_configuration(
    ca_cert: impl AsRef<Path>,
    cert: impl AsRef<Path>,
    key: impl AsRef<Path>,
) -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_modern_v5(SslMethod::tls()).unwrap();

    builder.set_ca_file(ca_cert).unwrap();
    builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert).unwrap();
    builder.set_verify(SslVerifyMode::PEER | SslVerifyMode::FAIL_IF_NO_PEER_CERT);

    builder
}

pub fn run_server(
    app_state: AppState,
    addr: impl ToSocketAddrs,
    tls_config: SslAcceptorBuilder,
) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(ext_keys)
            .service(ack)
    })
    .bind_openssl(addr, tls_config)?
    .run();

    Ok(server)
}
