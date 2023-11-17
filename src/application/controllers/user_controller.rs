use crate::application::State;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::application::dtos::response_message_dto::ResponseMessage;
use crate::infrastructure::repositories::user_repository;

pub fn load_user_controller(config: &mut web::ServiceConfig) {
    let web_scope = web::scope("/users").service(get_many).service(get_by_id);
    config.service(web_scope);
}

#[get("")]
async fn get_many(req: HttpRequest) -> impl Responder {
    let data_state = req
        .app_data::<web::Data<State>>()
        .expect("Global application data error");
    let db_response = user_repository::find_many(&data_state.pool)
        .await
        .expect("Failed to select values");
    println!("{:?}", db_response);

    let response = ResponseMessage {
        message: "Hello many".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/{id}")]
async fn get_by_id(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    let data_state = req
        .app_data::<web::Data<State>>()
        .expect("Global application data error");
    // let db_response = user_repository::find_many(&data_state.pool)
    let db_response = user_repository::find_one(&data_state.pool, 1)
        .await
        .expect("Failed to select values");
    println!("{:#?}", db_response);

    let response = ResponseMessage {
        message: format!("Hello {id}"),
    };
    HttpResponse::Ok().json(response)
}
