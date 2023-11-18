use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::application::dtos::response_message_dto::{
    DtoResponse, DtoResponseMany, ResponseMessage,
};
use crate::application::dtos::user_dto::{convert_user_into_dto, DtoUser};
use crate::application::State;
use crate::infrastructure::repositories::user_repository;

pub fn load_user_controller(config: &mut web::ServiceConfig) {
    let web_scope = web::scope("/users")
        .service(create_one)
        .service(get_many)
        .service(get_by_id);
    config.service(web_scope);
}

#[post("")]
async fn create_one(req: HttpRequest) -> impl Responder {
    let data_state = req
        .app_data::<web::Data<State>>()
        .expect("Global application data error");
    let user_todo = DtoUser {
        id: 1,
        username: "".to_string(),
        email: "".to_string(),
        person: None,
    };
    let db_response = user_repository::create_one(&data_state.pool, user_todo)
        .await
        .expect("Failed to select values");
    let response = ResponseMessage {
        message: db_response.id.to_string(),
    };

    HttpResponse::Ok().json(response)
}

#[get("")]
async fn get_many(req: HttpRequest) -> impl Responder {
    let data_state = req
        .app_data::<web::Data<State>>()
        .expect("Global application data error");
    let db_responses = user_repository::find_many(&data_state.pool)
        .await
        .expect("Failed to select values");
    let user_dtos = db_responses
        .into_iter()
        .map(convert_user_into_dto)
        .collect();
    let response = DtoResponseMany::<DtoUser> { items: user_dtos };

    HttpResponse::Ok().json(response)
}

#[get("/{id}")]
async fn get_by_id(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();
    let data_state = req
        .app_data::<web::Data<State>>()
        .expect("Global application data error");
    let db_response = user_repository::find_by_id(&data_state.pool, id)
        .await
        .expect("Failed to select values");
    let response = DtoResponse::<DtoUser> {
        item: convert_user_into_dto(db_response),
    };

    HttpResponse::Ok().json(response)
}
