use crate::app::state::AppState;
use crate::common::ResponseInfo;
use crate::common::{session::validate_token, InternalServerError, PasteData};
use actix_web::http::header;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AddPasteReq {
    text: String,
}

#[derive(Serialize)]
struct GetPasteResp {
    text: String,
}

#[post("/paste")]
pub async fn add_paste(
    state: web::Data<AppState>,
    add_paste_req: web::Json<AddPasteReq>,
    req: HttpRequest,
) -> impl Responder {
    info!("Inserting paste.");

    let result = save_paste(&state, &add_paste_req, &req).await;
    match result {
        Ok(_) => {
            info!("Paste added successfully.");
            HttpResponse::Created().json({
                ResponseInfo {
                    info: "Paste added successfully.".to_string(),
                }
            })
        }
        Err(e) => e.handle_error_for_http_resp(),
    }
}

async fn save_paste(
    state: &web::Data<AppState>,
    add_paste_req: &web::Json<AddPasteReq>,
    req: &HttpRequest,
) -> Result<(), InternalServerError> {
    check_session(&req)?;

    let client_resp = state.get_key_client().get_key().await?;
    let paste_data = PasteData {
        key: client_resp,
        content: add_paste_req.text.clone(),
    };

    state.get_db().add_paste(&paste_data).await
}

fn check_session(req: &HttpRequest) -> Result<(), InternalServerError> {
    let header_data = match req.headers().get(header::AUTHORIZATION) {
        Some(header) => header
            .to_str()
            .map_err(|e| InternalServerError::ServerComponentError(e.to_string()))?,
        None => {
            return Err(InternalServerError::UnauthorizedSession(
                "Invalid header.".to_string(),
            ));
        }
    };

    validate_token(&header_data.to_string())
}

#[get("/paste/{key}")]
pub async fn get_paste(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();
    info!("Paste requested for paste: {}.", key);

    let result = get_paste_from_db(&state, &key).await;
    match result {
        Ok(paste_data) => {
            info!("Paste received successfully.");
            HttpResponse::Ok().json(GetPasteResp {
                text: paste_data.content,
            })
        }
        Err(e) => e.handle_error_for_http_resp(),
    }
}

async fn get_paste_from_db(
    state: &web::Data<AppState>,
    key: &String,
) -> Result<PasteData, InternalServerError> {
    let paste_data = state.get_db().get_paste(&key).await?;

    match paste_data {
        Some(paste_data) => Ok(paste_data),
        None => Err(InternalServerError::InvalidUrl(key.clone())),
    }
}

#[delete("/paste/{key}")]
pub async fn delete_paste(
    state: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let key = path.into_inner();
    info!("Delete paste with key: {}.", &key);

    let result = remove_paste(&state, &key, &req).await;
    match result {
        Ok(_) => {
            info!("Paste deleted successfully.");
            HttpResponse::Ok().json({
                ResponseInfo {
                    info: "Paste deleted successfully.".to_string(),
                }
            })
        }
        Err(e) => e.handle_error_for_http_resp(),
    }
}

async fn remove_paste(
    state: &web::Data<AppState>,
    key: &String,
    req: &HttpRequest,
) -> Result<(), InternalServerError> {
    check_session(&req)?;

    state.get_key_client().delete_key(&key).await?;
    state.get_db().delete_paste(&key).await
}
