mod app;
mod authorization;
mod common;
mod database;
mod paste;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let app_state = app::state::AppState::new().await;

    let ip = env::var("SERVER_IP").unwrap();
    let port = env::var("SERVER_PORT").unwrap();

    info!("Running server on {}:{}.", ip, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(authorization::log_user)
            .service(authorization::register_user)
            .service(paste::add_paste)
            .service(paste::delete_paste)
            .service(paste::get_paste)
            .service(paste::check_expiry)
            .service(paste::get_user_pastes)
    })
    .bind((ip, port.parse().unwrap()))?
    .run()
    .await
}
