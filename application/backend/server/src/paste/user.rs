use crate::app::state::AppState;
use crate::common::{session::check_session, InternalServerError};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use log::info;
use serde::Serialize;

#[derive(Serialize, Clone)]
struct UserPaste {
    key: String,
    title: String,
}

#[derive(Serialize, Clone)]
struct GetUserPastesResp {
    pastes: Vec<UserPaste>,
}

#[get("/user/pastes")]
pub async fn get_user_pastes(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    info!("Get user pastes.");

    let result = get_pastes_from_db(&state, &req).await;
    match result {
        Ok(user_pastes) => {
            info!("Sending user pastes.");
            HttpResponse::Ok().json(user_pastes)
        }
        Err(e) => e.handle_error_for_http_resp(),
    }
}

async fn get_pastes_from_db(
    state: &web::Data<AppState>,
    req: &HttpRequest,
) -> Result<GetUserPastesResp, InternalServerError> {
    match check_session(&req)? {
        Some(id) => get_pastes_from_db_for_user(&state, &id.to_string()).await,
        None => Err(InternalServerError::InactiveSession),
    }
}

async fn get_pastes_from_db_for_user(
    state: &web::Data<AppState>,
    id: &String,
) -> Result<GetUserPastesResp, InternalServerError> {
    let mut resp = GetUserPastesResp { pastes: vec![] };

    let user_keys = state.get_db().get_keys(&id).await?;
    if user_keys.is_some() {
        for key in user_keys.unwrap().keys.iter() {
            add_key_to_resp(&state, &id, &key, &mut resp).await?;
        }
    }
    Ok(resp)
}

async fn add_key_to_resp(
    state: &web::Data<AppState>,
    id: &String,
    key: &String,
    resp: &mut GetUserPastesResp,
) -> Result<(), InternalServerError> {
    let paste = state.get_db().get_paste(&key).await?;

    if paste.is_none() {
        state.get_db().delete_key(&id, &key).await?;
        return Err(InternalServerError::ServerComponentError(
            "User owns invalid paste.".to_string(),
        ));
    }
    resp.pastes.push(UserPaste {
        key: key.clone(),
        title: paste.unwrap().title,
    });
    Ok(())
}
