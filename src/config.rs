// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use std::{any::type_name, env, error::Error, str::FromStr};
use tracing::error;

use crate::client::Etsi020Client;

const ENV_ROOT_CERT: &str = "ETSI_020_REF_IMPL_ROOT_CERT";
const ENV_PRIVATE_KEY: &str = "ETSI_020_REF_IMPL_PRIVATE_KEY";
const ENV_PUBLIC_CERT: &str = "ETSI_020_REF_IMPL_PUBLIC_CERT";
const ENV_PORT_NUM: &str = "ETSI_020_REF_IMPL_PORT_NUM";
const ENV_IP_ADDR: &str = "ETSI_020_REF_IMPL_IP_ADDR";
const ENV_INTRA_NETWORK_SAE_IDS: &str = "ETSI_020_REF_IMPL_INTRA_NETWORK_SAE_IDS";
const ENV_THIRD_PARTY_SAE_IDS: &str = "ETSI_020_REF_IMPL_THIRD_PARTY_SAE_IDS";

#[derive(Clone, Debug)]
pub struct Config {
    pub root_cert: String,
    pub private_key: String,
    pub public_cert: String,
    pub port_num: u16,
    pub ip_addr: String,
    pub intra_network_sae_ids: Vec<String>,
    pub third_party_sae_ids: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            root_cert: Self::extract_string_value(ENV_ROOT_CERT),
            private_key: Self::extract_string_value(ENV_PRIVATE_KEY),
            public_cert: Self::extract_string_value(ENV_PUBLIC_CERT),
            port_num: Self::extract_u16_value(ENV_PORT_NUM),
            ip_addr: Self::extract_string_value(ENV_IP_ADDR),
            intra_network_sae_ids: Self::extract_comma_sep_vec(ENV_INTRA_NETWORK_SAE_IDS),
            third_party_sae_ids: Self::extract_comma_sep_vec(ENV_THIRD_PARTY_SAE_IDS),
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

    fn extract_comma_sep_vec<Item>(var_name: &str) -> Vec<Item>
    where
        Item: FromStr,
    {
        let extracted_value = Self::extract_string_value(var_name);

        let result = extracted_value
            .split(',')
            .map(|item| item.parse::<Item>())
            .collect::<Result<Vec<Item>, Item::Err>>();

        match result {
            Ok(vec) => vec,
            Err(_) => {
                panic!(
                    "Error when converting '{extracted_value}' in '{}' to list of '{}'. Incorrect value set.",
                    var_name,
                    type_name::<Item>()
                );
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
    pub config: Config,
    pub callback_client: Etsi020Client,
}

impl AppState {
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let client =
            Etsi020Client::new(&config.root_cert, &config.public_cert, &config.private_key)?;

        Ok(Self {
            config,
            callback_client: client,
        })
    }
}
