use crate::application::State;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::application::dtos::response_message_dto::ResponseMessage;

pub fn load_user_controller(config: &mut web::ServiceConfig) {
    let web_scope = web::scope("/users").service(hello).service(hello_txt);
    config.service(web_scope);
}

#[get("/hello-txt/{name}")]
async fn hello_txt(req: HttpRequest, name: web::Path<String>) -> impl Responder {
    let msg = format!("Hello {name}");
    let data_state = req
        .app_data::<web::Data<State>>()
        .expect("Global application data error");
    println!("{:#?}", data_state);

    HttpResponse::Ok().body(msg)
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    let response = ResponseMessage {
        message: format!("Hello {name}"),
    };
    HttpResponse::Ok().json(response)
}
