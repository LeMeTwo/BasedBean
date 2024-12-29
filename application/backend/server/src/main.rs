mod app;
mod authorization;
mod common;
mod database;
mod paste;

use actix_web::{web, App, HttpServer};
use app::state::AppState;
use authorization::{log_user, register_user};
use dotenv::dotenv;
use env_logger;
use log::info;
use paste::{add_paste, delete_paste, get_paste, check_expiry};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let app_state = AppState::new().await;

    let ip = env::var("SERVER_IP").unwrap();
    let port = env::var("SERVER_PORT").unwrap();

    info!("Running server on {}:{}.", ip, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(log_user)
            .service(register_user)
            .service(add_paste)
            .service(delete_paste)
            .service(get_paste)
            .service(check_expiry)
    })
    .bind((ip, port.parse().unwrap()))?
    .run()
    .await
}