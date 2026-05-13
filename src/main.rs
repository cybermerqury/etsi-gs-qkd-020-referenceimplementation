pub mod client;
pub mod config;
pub mod endpoints;
pub mod ext_keys_caller;
pub mod server;
pub mod types;

use std::process::exit;
use tokio::signal::ctrl_c;

use crate::{
    config::{AppState, Config},
    ext_keys_caller::ext_keys_subsystem,
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

    let server = run_server(app_state.clone(), tls_config)?;

    tokio::select! {
        _ = server => {
            println!("Server exited.")
        },
        _ = ext_keys_subsystem(app_state.clone()) => {
            println!("ext_keys caller subsystem exited.");
        },
        _ = ctrl_c() => {
            println!("Signal caught. Terminating.")
        }
    }

    println!("Shutting down.");

    Ok(())
}
