use crate::application::State;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::application::dtos::response_message_dto::{
    convert_user_into_dto, DtoResponse, DtoResponseMany, DtoUser,
};
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
    let db_responses = user_repository::find_many(&data_state.pool)
        .await
        .expect("Failed to select values");
    println!("{:?}", db_responses);
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
    let db_response = user_repository::find_one(&data_state.pool, id)
        .await
        .expect("Failed to select values");
    let response = DtoResponse::<DtoUser> {
        item: convert_user_into_dto(db_response),
    };

    HttpResponse::Ok().json(response)
}
