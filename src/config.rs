// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use std::{any::type_name, env, error::Error, fmt::Display, str::FromStr, time::Duration};
use tracing::error;
use url::Url;

use crate::client::Etsi020Client;

// ----------------
// Base config keys
// ----------------

const ENV_ROOT_CERT: &str = "ETSI_020_REF_IMPL_ROOT_CERT";
const ENV_PRIVATE_KEY: &str = "ETSI_020_REF_IMPL_PRIVATE_KEY";
const ENV_PUBLIC_CERT: &str = "ETSI_020_REF_IMPL_PUBLIC_CERT";
const ENV_PORT_NUM: &str = "ETSI_020_REF_IMPL_PORT_NUM";
const ENV_IP_ADDR: &str = "ETSI_020_REF_IMPL_IP_ADDR";
const ENV_INTRA_NETWORK_SAE_IDS: &str = "ETSI_020_REF_IMPL_INTRA_NETWORK_SAE_IDS";
const ENV_THIRD_PARTY_SAE_IDS: &str = "ETSI_020_REF_IMPL_THIRD_PARTY_SAE_IDS";

// -----------------------
// ext_keys caller ss keys
// -----------------------

const ENV_SEND_OUTBOUND_EXT_KEYS_ENABLE: &str = "ETSI_020_REF_IMPL_SEND_EXT_KEYS_ENABLE";
const ENV_SEND_OUTBOUND_EXT_KEYS_INTERVAL_SECONDS: &str =
    "ETSI_020_REF_IMPL_SEND_EXT_KEYS_INTERVAL_SECONDS";
const ENV_SEND_OUTBOUND_EXT_KEYS_BASE_URL: &str = "ETSI_020_REF_IMPL_SEND_EXT_KEYS_BASE_URL";
const ENV_SEND_OUTBOUND_EXT_KEYS_ACK_URL: &str = "ETSI_020_REF_IMPL_SEND_EXT_KEYS_ACK_CALLBACK_URL";

#[derive(Clone, Debug)]
pub struct SendExtKeysConfig {
    pub enabled: bool,
    pub dispatch_interval: Duration,
    pub target_url: Url,
    pub ack_url: Url,
}

impl SendExtKeysConfig {
    pub fn new() -> Self {
        Self {
            enabled: extract_value(ENV_SEND_OUTBOUND_EXT_KEYS_ENABLE),
            dispatch_interval: Duration::from_secs(extract_value(
                ENV_SEND_OUTBOUND_EXT_KEYS_INTERVAL_SECONDS,
            )),
            target_url: extract_value(ENV_SEND_OUTBOUND_EXT_KEYS_BASE_URL),
            ack_url: extract_value(ENV_SEND_OUTBOUND_EXT_KEYS_ACK_URL),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub root_cert: String,
    pub private_key: String,
    pub public_cert: String,
    pub port_num: u16,
    pub ip_addr: String,
    pub send_ext_keys_config: SendExtKeysConfig,
    pub intra_network_sae_ids: Vec<String>,
    pub third_party_sae_ids: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            root_cert: extract_string_value(ENV_ROOT_CERT),
            private_key: extract_string_value(ENV_PRIVATE_KEY),
            public_cert: extract_string_value(ENV_PUBLIC_CERT),
            port_num: extract_value(ENV_PORT_NUM),
            ip_addr: extract_string_value(ENV_IP_ADDR),
            send_ext_keys_config: SendExtKeysConfig::new(),
            intra_network_sae_ids: extract_comma_sep_vec(ENV_INTRA_NETWORK_SAE_IDS),
            third_party_sae_ids: extract_comma_sep_vec(ENV_THIRD_PARTY_SAE_IDS),
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

fn extract_comma_sep_vec<Item>(var_name: &str) -> Vec<Item>
where
    Item: FromStr,
{
    let extracted_value = extract_string_value(var_name);

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

fn extract_value<Value>(var_name: &str) -> Value
where
    Value: FromStr,
    Value::Err: Display,
{
    let extracted_value = extract_string_value(var_name);

    match extracted_value.parse() {
        Ok(val) => val,
        Err(e) => panic!(
            "Error when converting '{extracted_value}' in '{}' to '{}'. Error: {e}",
            var_name,
            type_name::<Value>()
        ),
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
