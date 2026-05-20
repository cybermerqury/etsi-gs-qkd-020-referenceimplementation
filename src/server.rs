use std::{error::Error, sync::Arc};

use actix_tls::accept::rustls_0_23::reexports::ServerConfig;
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    RootCertStore,
};

use crate::{
    config::{AppState, Config},
    endpoints::{ack::ack, ext_keys::ext_keys},
};

/// Initialize an mTLS configuration for the actix-web server.
pub fn build_tls_configuration(config: &Config) -> Result<ServerConfig, Box<dyn Error>> {
    let mut root_store = RootCertStore::empty();
    root_store.add(CertificateDer::from_pem_file(&config.root_cert)?)?;

    let verifier = WebPkiClientVerifier::builder(Arc::new(root_store)).build()?;

    let server_cert = CertificateDer::from_pem_file(&config.public_cert)?;
    let server_key = PrivateKeyDer::from_pem_file(&config.private_key)?;

    let server_config = ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(vec![server_cert], server_key)?;

    Ok(server_config)
}

/// Initialize the server and begin serving requests.
/// Returns a handle which can be used to remotely abort the server.
pub fn run_server(app_state: AppState, tls_config: ServerConfig) -> std::io::Result<Server> {
    let bind_addr = (app_state.config.ip_addr.clone(), app_state.config.port_num);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(ext_keys)
            .service(ack)
            .wrap(Logger::default())
    })
    .bind_rustls_0_23(bind_addr, tls_config)?
    .run();

    Ok(server)
}
