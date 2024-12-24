use chrono;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use super::InternalServerError;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: i64,
    iat: i64,
}

pub fn generate_token(sub: &Uuid) -> Result<String, InternalServerError> {
    let claims = Claims {
        sub: sub.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(2)).timestamp(),
        iat: chrono::Utc::now().timestamp(),
    };
    let secret = env::var("JWT_SECRET").unwrap();

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())) {
        Ok(token) => Ok(token),
        Err(e) => Err(InternalServerError::ServerComponentError(e.to_string())),
    }
}

pub fn validate_token(header_data: &String) -> Result<(), InternalServerError> {
    if !header_data.starts_with("Bearer ") {
        return Err(InternalServerError::UnauthorizedSession("Invalid header data.".to_string()));
    }

    let token = header_data.trim_start_matches("Bearer ").to_string();
    let secret = env::var("JWT_SECRET").unwrap();

    match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default()) {
        Ok(_) => {
            info!("User successfully authorized for token: {}.", token);
            Ok(())
        }
        Err(e) => Err(InternalServerError::UnauthorizedSession(e.to_string()))
    }
}
