use actix_web::{middleware, App, HttpServer};

use crate::application::controllers::user_controller::load_user_controller;

mod application;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = ("127.0.0.1", 8080);
    let app_factory = || {
        App::new()
            .wrap(middleware::Compress::default())
            .configure(load_user_controller)
    };
    HttpServer::new(app_factory).bind(address)?.run().await
}

