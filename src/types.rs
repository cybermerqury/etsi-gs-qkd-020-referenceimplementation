use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct KeyValueElement {
    pub key_id: Uuid,
    pub value: String, // This is a base64 string
}

#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub type_name: String,
    pub status: u32,
    pub title: String,
    pub details: HashMap<String, String>,
}
