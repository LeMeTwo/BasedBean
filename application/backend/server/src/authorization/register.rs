use crate::{
    app::state::AppState,
    common::{ResponseInfo, UserData},
};
use actix_web::{post, web, HttpResponse, Responder};
use log::info;
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
    info!(
        "Reqister user: {}, password {}",
        &register_req.user, &register_req.password
    );

    let user_data = UserData {
        id: Uuid::new_v4(),
        username: register_req.user.clone(),
        password: register_req.password.clone(),
    };

    add_user_to_db(&state, &user_data).await
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
