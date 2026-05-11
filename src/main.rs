use std::process::exit;

use ref_impl_lib::{
    config::{AppState, Config},
    server::{build_tls_configuration, run_server},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    let tls_config =
        build_tls_configuration(&config.root_cert, &config.public_cert, &config.private_key);

    let app_state = match AppState::new(config) {
        Ok(state) => state,
        Err(e) => {
            println!("Failed to initialise app state. Exiting. Error: {e}");
            exit(1);
        }
    };

    println!(
        "Listening on {}:{}",
        app_state.config.ip_addr, app_state.config.port_num
    );

    let server = run_server(app_state, tls_config)?;

    server.await
}
