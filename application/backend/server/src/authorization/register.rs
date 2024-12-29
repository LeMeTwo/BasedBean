use crate::{
    app::state::AppState,
    common::{ResponseInfo, UserData},
};
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{password_hash, Argon2, PasswordHasher};
use log::{debug, info};
use rand::rngs::OsRng;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
struct RegisterReq {
    user: String,
    password: String,
}

#[post("/register")]
pub async fn register_user(
    state: web::Data<AppState>,
    register_req: web::Json<RegisterReq>,
) -> impl Responder {
    info!("Reqister user: {}.", &register_req.user);

    let user_data = UserData {
        id: Uuid::new_v4(),
        username: register_req.user.clone(),
        password: hash_password(&register_req.password),
    };

    add_user_to_db(&state, &user_data).await
}

fn hash_password(password: &String) -> String {
    let salt = password_hash::SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), salt.as_salt())
        .unwrap();

    debug!("Password hash: {:}", hash);

    hash.to_string()
}

async fn add_user_to_db(state: &web::Data<AppState>, user_data: &UserData) -> HttpResponse {
    match state.get_db().add_user_data(&user_data).await {
        Ok(_) => {
            info!("User added successfully");
            HttpResponse::Created().json(ResponseInfo {
                info: "User added successfully".to_string(),
            })
        }
        Err(e) => e.handle_error_for_http_resp(),
    }
}
