use crate::app::state::AppState;
use crate::common::ResponseInfo;
use crate::common::{session::check_session, InternalServerError, PasteData};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use chrono;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AddPasteReq {
    text: String,
    title: String,
}

#[derive(Serialize)]
struct GetPasteResp {
    text: String,
    title: String,
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
    const GUEST_ID: &str = "guest";

    let key = state.get_key_client().get_key().await?;
    let paste_data = PasteData {
        key: key.clone(),
        content: add_paste_req.text.clone(),
        title: add_paste_req.title.clone(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    state.get_db().add_paste(&paste_data).await?;

    match check_session(&req)? {
        Some(id) => state.get_db().add_key(&id.to_string(), &key).await,
        None => state.get_db().add_key(&GUEST_ID.to_string(), &key).await,
    }
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
                title: paste_data.title,
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
    match check_session(&req)? {
        Some(id) => remove_user_paste(&state, &key, &id.to_string()).await,
        None => Err(InternalServerError::InactiveSession),
    }
}

async fn remove_user_paste(
    state: &web::Data<AppState>,
    key: &String,
    id: &String,
) -> Result<(), InternalServerError> {
    match state.get_db().get_keys(&id).await? {
        Some(user_keys) => {
            if !user_keys.keys.iter().any(|k| k == key) {
                return Err(InternalServerError::InvalidUrl(key.clone()));
            }

            state.get_key_client().delete_key(&key).await?;
            state.get_db().delete_paste(&key).await?;
            state.get_db().dekete_key(&id, &key).await
        }
        None => Err(InternalServerError::InvalidUrl(key.clone())),
    }
}
