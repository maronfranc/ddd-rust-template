use actix_web::{get, web, HttpResponse, Responder};

use crate::application::dtos::response_message_dto::ResponseMessage;

pub fn load_user_controller(config: &mut web::ServiceConfig) {
    let web_scope = web::scope("/users").service(hello).service(hello_txt);
    config.service(web_scope);
}

#[get("/hello-txt/{name}")]
async fn hello_txt(name: web::Path<String>) -> impl Responder {
    let msg = format!("Hello {name}");
    HttpResponse::Ok().body(msg)
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    let response = ResponseMessage {
        message: format!("Hello {name}"),
    };
    HttpResponse::Ok().json(response)
}
