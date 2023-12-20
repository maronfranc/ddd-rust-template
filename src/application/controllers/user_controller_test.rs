#![allow(dead_code)]
#[cfg(test)]
mod tests {
    use actix_http::{
        body::{BoxBody, EitherBody},
        encoding::Encoder,
        Request,
    };
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::http::StatusCode;
    use actix_web::{middleware, test, web, App, Error};
    use rand::Rng;
    use serde_json;
    use sqlx::PgPool;

    use crate::application;
    use crate::application::controllers::user_controller::load_user_controller;
    use crate::application::dtos::response_message_dto::{DtoId, DtoResponse, DtoResponseMany};
    use crate::application::dtos::user_dto::{DtoRegisterUser, DtoUser};

    async fn init_app_test(
    ) -> impl Service<Request, Response = ServiceResponse<EitherBody<Encoder<BoxBody>>>, Error = Error>
    {
        let db_url = "postgresql://my_user:pass123@localhost:5432/my_db";
        let pool = PgPool::connect(db_url)
            .await
            .expect("Failed to connect to postgres");
        let app_state = application::State { pool };
        let app = App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(app_state.clone()))
            .configure(load_user_controller);
        test::init_service(app).await
    }

    #[actix_web::test]
    async fn test_create_one() {
        let app = init_app_test().await;
        let random_number = rand::thread_rng().gen_range(0..100000);
        let dto = DtoRegisterUser {
            username: format!("{} dev_test_username", random_number),
            email: format!("email{}@example.com", random_number),
            password: "testpass123".to_string(),
            id_person: None,
            first_name: Some("Test First".to_string()),
            last_name: Some("Test Last".to_string()),
        };

        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(dto)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let bytes = test::read_body(resp).await;
        let body: DtoResponse<DtoId> =
            serde_json::from_slice(&bytes).expect("Test failed json serialization");
        assert_ne!(body.item.id, 1);
    }

    #[actix_web::test]
    async fn test_get_many() {
        let app = init_app_test().await;
        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let bytes = test::read_body(resp).await;
        let body: DtoResponseMany<DtoUser> =
            serde_json::from_slice(&bytes).expect("Test failed json serialization");
        assert_ne!(body.items.len(), 0);
    }

    #[actix_web::test]
    async fn test_get_by_id() {
        let app = init_app_test().await;
        let id: i32 = 2;
        let req = test::TestRequest::get()
            .uri(&format!("/users/{id}"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let bytes = test::read_body(resp).await;
        let body: DtoResponse<DtoUser> =
            serde_json::from_slice(&bytes).expect("Test failed json serialization");

        assert_eq!(body.item.id, id);
    }
}
