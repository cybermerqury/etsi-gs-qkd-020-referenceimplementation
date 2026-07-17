// SPDX-FileCopyrightText: © 2026 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
pub mod client;
pub mod config;
pub mod endpoints;
pub mod ext_keys_caller;
pub mod server;
pub mod types;

use std::process::exit;
use tokio::signal::ctrl_c;
use tracing::{error, info};

use crate::{
    config::{AppState, Config},
    ext_keys_caller::ext_keys_subsystem,
    server::{build_tls_configuration, run_server},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    tracing_subscriber::fmt()
        .with_max_level(config.log_level)
        .init();

    let tls_config = build_tls_configuration(&config).unwrap_or_else(|e| {
        error!("Failed to load TLS server config. Error: {e}");
        exit(1);
    });

    let app_state = AppState::new(config).unwrap_or_else(|e| {
        error!("Failed to initialise app state. Exiting. Error: {e}");
        exit(1);
    });

    info!(
        "Listening on {}:{}",
        app_state.config.ip_addr, app_state.config.port_num
    );

    let server = run_server(app_state.clone(), tls_config)?;

    // Wait until either a task exits prematurely, or a SIGTERM is caught.
    // In the meantime, let the server service requests.
    tokio::select! {
        Err(e) = server => {
            error!("Server exited. Error: {e}");
        }
        Err(e) = ext_keys_subsystem(app_state.clone()) => {
            error!("ext_keys caller subsystem exited. Error: {e}");
        }
        _ = ctrl_c() => {
            info!("Signal caught. Terminating.");
        }
        else => {
            info!("One of the subsystems shut down. Exiting.");
        }
    }

    info!("Shutting down.");

    Ok(())
}
