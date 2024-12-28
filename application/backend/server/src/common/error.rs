use super::ResponseInfo;
use actix_web::HttpResponse;
use log::{error, warn};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum InternalServerError {
    MongoDbError(String),
    InvalidDbData(String),
    KegClientErrorResponse(String),
    KegServerErrorResponse(String),
    KegInvalidData(String),
    InvalidUsername(String),
    InvalidPassword(String, String),
    InvalidHeaderData(String),
    InactiveSession,
    ServerComponentError(String),
    InvalidUrl(String),
}

impl Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalServerError::MongoDbError(msg) => {
                write!(f, "InternalServerError::MongoDbError: {}", msg)
            }
            InternalServerError::InactiveSession => {
                write!(
                    f,
                    "InternalServerError::InactiveSession: Operation require active session."
                )
            }
            InternalServerError::InvalidDbData(msg) => {
                write!(f, "InternalServerError::InvalidDbData: {}", msg)
            }
            InternalServerError::KegClientErrorResponse(status) => {
                write!(
                    f,
                    "InternalServerError::KegClientErrorResponse: Keg client error with status {}",
                    status
                )
            }
            InternalServerError::KegServerErrorResponse(status) => {
                write!(
                    f,
                    "InternalServerError::KegServerErrorResponse: Keg client error with status {}",
                    status
                )
            }
            InternalServerError::KegInvalidData(msg) => {
                write!(f, "InternalServerError::KegInvalidData: {}", msg)
            }
            InternalServerError::InvalidUsername(username) => {
                write!(
                    f,
                    "InternalServerError::InvalidUsername: no user {} in db",
                    username
                )
            }
            InternalServerError::InvalidPassword(password, expected_password) => {
                write!(
                    f,
                    "InternalServerError::InvalidPassword: send {}, expected {}",
                    password, expected_password
                )
            }
            InternalServerError::InvalidHeaderData(msg) => {
                write!(f, "InternalServerError::InvalidHeaderData: {}", msg)
            }
            InternalServerError::ServerComponentError(msg) => {
                write!(f, "InternalServerError::ServerComponentError: {}", msg)
            }
            InternalServerError::InvalidUrl(key) => {
                write!(f, "InternalServerError::InvalidUrl: Invalid key {}", key)
            }
        }
    }
}

impl InternalServerError {
    pub fn handle_error_for_http_resp(&self) -> HttpResponse {
        match self {
            InternalServerError::InvalidDbData(_)
            | InternalServerError::InvalidUsername(_)
            | InternalServerError::InvalidPassword(_, _) => warn!("{}", &self),
            _ => error!("{}", &self),
        }

        match self {
            InternalServerError::InvalidDbData(msg) => {
                HttpResponse::BadRequest().json(ResponseInfo { info: msg.clone() })
            }
            InternalServerError::MongoDbError(_) => HttpResponse::BadGateway().json(ResponseInfo {
                info: "Cannot connect to database.".to_string(),
            }),
            InternalServerError::InvalidUsername(_)
            | InternalServerError::InvalidPassword(_, _) => {
                HttpResponse::Unauthorized().json(ResponseInfo {
                    info: "Invalid login or password.".to_string(),
                })
            }
            InternalServerError::InactiveSession => {
                HttpResponse::Unauthorized().json(ResponseInfo {
                    info: "Unactive session.".to_string(),
                })
            }
            InternalServerError::InvalidHeaderData(_) => {
                HttpResponse::BadRequest().json(ResponseInfo {
                    info: "Invalid header data".to_string(),
                })
            }
            InternalServerError::InvalidUrl(_) => HttpResponse::NotFound().json(ResponseInfo {
                info: "Invalid URL.".to_string(),
            }),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl Error for InternalServerError {}
