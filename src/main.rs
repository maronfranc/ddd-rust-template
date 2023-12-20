#![allow(dead_code)]
use actix_web::{middleware, web, App, HttpServer};
use load_dotenv::load_dotenv;
use sqlx::{migrate, PgPool};
use std::env;

use crate::application::controllers::user_controller::load_user_controller;

mod application;
mod infrastructure;

#[actix_web::main] // #[tokio::main]
async fn main() -> std::io::Result<()> {
    load_dotenv!();
    let db_url = env!("DATABASE_URL");

    let pool = PgPool::connect(db_url)
        .await
        .expect("Failed to connect to postgres");
    // https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
    migrate!("src/infrastructure/migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate postgres");
    let address = ("127.0.0.1", 8080);
    let app_state = application::State { pool };
    println!("Server listening on {}", address.0);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(middleware::Compress::default())
            .configure(load_user_controller)
    })
    .bind(address)?
    .run()
    .await
}
