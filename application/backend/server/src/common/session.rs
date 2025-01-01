use super::InternalServerError;
use actix_web::{http::header, HttpRequest};
use chrono;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

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

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(token) => Ok(token),
        Err(e) => Err(InternalServerError::ServerComponentError(e.to_string())),
    }
}

fn validate_token(header_data: &String) -> Result<Option<Uuid>, InternalServerError> {
    if !header_data.starts_with("Bearer ") {
        return Err(InternalServerError::InvalidHeaderData(
            "Invalid header data.".to_string(),
        ));
    }

    let token = header_data.trim_start_matches("Bearer ").to_string();
    let secret = env::var("JWT_SECRET").unwrap();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            info!("User successfully authorized for token: {}.", token);
            Ok(Some(token_data.claims.sub))
        }
        Err(e) => Err(InternalServerError::InvalidHeaderData(e.to_string())),
    }
}

pub fn check_session(req: &HttpRequest) -> Result<Option<Uuid>, InternalServerError> {
    match req.headers().get(header::AUTHORIZATION) {
        Some(header) => validate_token(&header.to_str().unwrap().to_string()),
        None => Ok(None),
    }
}
