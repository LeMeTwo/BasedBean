use crate::app::state::AppState;
use crate::common::{InternalServerError, ResponseInfo, GUEST_ID};
use actix_web::{delete, web, HttpResponse, Responder};
use chrono;
use log::info;

#[delete("/expiry")]
pub async fn check_expiry(state: web::Data<AppState>) -> impl Responder {
    info!("Check key expiry.");

    let result = expire_outdated_pastes(&state).await;
    match result {
        Ok(_) => {
            info!("Expiry checked successfully.");
            HttpResponse::Ok().json({
                ResponseInfo {
                    info: "Expiry checked successfully.".to_string(),
                }
            })
        }
        Err(e) => e.handle_error_for_http_resp(),
    }
}

async fn expire_outdated_pastes(state: &web::Data<AppState>) -> Result<(), InternalServerError> {
    let guest_keys = state.get_db().get_keys(&GUEST_ID.to_string()).await?;
    if guest_keys.is_none() {
        return Ok(());
    }

    for key in guest_keys.unwrap().keys.iter() {
        check_single_paste(&state, &key).await?;
    }
    Ok(())
}

async fn check_single_paste(
    state: &web::Data<AppState>,
    key: &String,
) -> Result<(), InternalServerError> {
    match state.get_db().get_paste(key).await? {
        Some(paste) => {
            delete_outdated_paste(&state, &key, paste.timestamp).await?;
        }
        None => {
            state
                .get_db()
                .delete_key(&GUEST_ID.to_string(), &key)
                .await?;
        }
    }
    Ok(())
}

async fn delete_outdated_paste(
    state: &web::Data<AppState>,
    key: &String,
    paste_timestamp: i64,
) -> Result<(), InternalServerError> {
    let oldest_valid_timestamp = (chrono::Utc::now() - chrono::Duration::days(7)).timestamp();
    if oldest_valid_timestamp > paste_timestamp {
        state.get_key_client().delete_key(&key).await?;
        state.get_db().delete_paste(&key).await?;
        state
            .get_db()
            .delete_key(&GUEST_ID.to_string(), &key)
            .await?;
    }
    Ok(())
}
