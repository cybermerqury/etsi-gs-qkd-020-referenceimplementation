// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use reqwest::{Certificate, Client, Identity};
use std::{env, error::Error, time::Duration};
use tracing::error;

static ENV_ROOT_CERT: &str = "ETSI_020_REF_IMPL_ROOT_CERT";
static ENV_PRIVATE_KEY: &str = "ETSI_020_REF_IMPL_PRIVATE_KEY";
static ENV_PUBLIC_CERT: &str = "ETSI_020_REF_IMPL_PUBLIC_CERT";
static ENV_PORT_NUM: &str = "ETSI_020_REF_IMPL_PORT_NUM";
static ENV_IP_ADDR: &str = "ETSI_020_REF_IMPL_IP_ADDR";

#[derive(Debug)]
pub struct Config {
    pub root_cert: String,
    pub private_key: String,
    pub public_cert: String,
    pub port_num: u16,
    pub ip_addr: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            root_cert: Self::extract_string_value(ENV_ROOT_CERT),
            private_key: Self::extract_string_value(ENV_PRIVATE_KEY),
            public_cert: Self::extract_string_value(ENV_PUBLIC_CERT),
            port_num: Self::extract_u16_value(ENV_PORT_NUM),
            ip_addr: Self::extract_string_value(ENV_IP_ADDR),
        }
    }

    fn extract_u16_value(var_name: &str) -> u16 {
        let extracted_value = Self::extract_string_value(var_name);

        match extracted_value.parse() {
            Ok(val) => val,
            Err(e) => {
                error!("Error when converting '{}' to a u16: {:?}", var_name, e);
                panic!("'{}' incorrect value set", var_name)
            }
        }
    }

    fn extract_string_value(var_name: &str) -> String {
        match env::var(var_name) {
            Ok(val) => val,
            Err(e) => {
                error!("Error when extracting '{}': {:?}", var_name, e);
                panic!("Environment variable '{}' not set", var_name)
            }
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub callback_client: Client,
}

impl AppState {
    pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
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
