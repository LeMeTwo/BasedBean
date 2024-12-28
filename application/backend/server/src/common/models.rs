use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserData {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PasteData {
    pub key: String,
    pub content: String,
    pub title: String,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyData {
    pub id: String,
    pub keys: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponseInfo {
    pub info: String,
}
