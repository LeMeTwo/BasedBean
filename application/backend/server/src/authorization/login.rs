use crate::app::state::AppState;
use crate::common::{session::generate_token, InternalServerError, UserData};
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginReq {
    user: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResp {
    token: String,
}

#[post("/login")]
pub async fn log_user(
    state: web::Data<AppState>,
    login_req: web::Json<LoginReq>,
) -> impl Responder {
    info!(
        "Log user: {}, password {}",
        &login_req.user, &login_req.password
    );

    let result = is_user_valid(&state, &login_req).await;

    match result {
        Ok(user_data) => handle_valid_user(&user_data),
        Err(e) => e.handle_error_for_http_resp(),
    }
}

async fn is_user_valid(
    state: &web::Data<AppState>,
    login_req: &web::Json<LoginReq>,
) -> Result<UserData, InternalServerError> {
    let user_data = state.get_db().get_user_data(&login_req.user).await?;
    match user_data {
        Some(data) => is_password_valid(&data, &login_req.password),
        None => Err(InternalServerError::InvalidUsername(
            login_req.user.to_string(),
        )),
    }
}

fn is_password_valid(
    user_data: &UserData,
    password: &String,
) -> Result<UserData, InternalServerError> {
    let password_hash = PasswordHash::new(&user_data.password).unwrap();
    match Argon2::default().verify_password(password.as_bytes(), &password_hash) {
        Ok(_) => Ok(user_data.clone()),
        Err(_) => Err(InternalServerError::InvalidPassword(
            password_hash.to_string(),
            password.clone(),
        )),
    }
}

fn handle_valid_user(user_data: &UserData) -> HttpResponse {
    debug!("Setup session for user id {}", user_data.id);

    match generate_token(&user_data.id) {
        Ok(token) => {
            info!("User logged in successfully.");
            HttpResponse::Ok().json(LoginResp { token: token })
        }
        Err(e) => e.handle_error_for_http_resp(),
    }
}
