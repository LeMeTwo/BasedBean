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
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponseInfo {
    pub info: String,
}

