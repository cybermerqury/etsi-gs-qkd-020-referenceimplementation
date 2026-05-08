use std::collections::HashMap;

use base64::Engine;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct KeyValueElement {
    pub key_id: Uuid,
    pub value: String, // This is a base64 string
}

impl KeyValueElement {
    pub fn is_key_value_valid(&self) -> bool {
        match base64::engine::general_purpose::STANDARD.decode(&self.value) {
            Ok(decoded_key) => {
                decoded_key.len() == 32 // Keys must be 256bits long
            }
            Err(_) => false,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KeyIdElement {
    pub key_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub type_name: String,
    pub status: u32,
    pub title: String,
    pub details: HashMap<String, String>,
}
