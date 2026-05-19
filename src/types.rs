pub mod ack;
pub mod ext_keys;

use std::{collections::HashMap, error::Error, fmt::Display};

use base64::Engine;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct KeyValueElement {
    pub key_id: Uuid,
    pub value: String, // This is a base64 string
}

impl KeyValueElement {
    pub fn is_key_value_valid(&self) -> bool {
        base64::engine::general_purpose::STANDARD
            .decode(&self.value)
            .is_ok_and(|v| v.len() == 32)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct KeyIdElement {
    pub key_id: Uuid,
}

/// Represents a standard `ETSI020` response body for HTTP errors.
#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub type_name: String,
    pub status: u32,
    pub title: String,
    pub details: HashMap<String, String>,
}

impl ErrorResponse {
    pub fn from_status_code<DetailsIter, K, V>(
        status: StatusCode,
        title: impl ToString,
        details: DetailsIter,
    ) -> Self
    where
        DetailsIter: IntoIterator<Item = (K, V)>,
        K: ToString,
        V: ToString,
    {
        Self::new(status, status.as_u16().into(), title, details)
    }

    pub fn new<DetailsIter, K, V>(
        type_name: impl ToString,
        status: u32,
        title: impl ToString,
        details: DetailsIter,
    ) -> Self
    where
        DetailsIter: IntoIterator<Item = (K, V)>,
        K: ToString,
        V: ToString,
    {
        Self {
            type_name: type_name.to_string(),
            status,
            title: title.to_string(),
            details: details
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ETSI020 Error response: '{}' (code {}): {}",
            self.type_name, self.status, self.title
        )
    }
}

impl Error for ErrorResponse {}
