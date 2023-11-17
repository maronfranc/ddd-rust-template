#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{middleware, test, web, App};
    use serde_json;
    use sqlx::PgPool;

    use crate::application;
    use crate::application::controllers::user_controller::load_user_controller;
    use crate::application::dtos::response_message_dto::{DtoResponse, DtoResponseMany, DtoUser};

    #[actix_web::test]
    async fn test_get_many() {
        let db_url = "postgresql://my_user:pass123@localhost:5432/my_db";
        let pool = PgPool::connect(db_url)
            .await
            .expect("Failed to connect to postgres");
        let app_state = application::State { pool };
        let app = test::init_service(
            App::new()
                .wrap(middleware::Compress::default())
                .app_data(web::Data::new(app_state.clone()))
                .configure(load_user_controller),
        )
        .await;
        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let bytes = test::read_body(resp).await;
        let body: DtoResponseMany<DtoUser> = serde_json::from_slice(&bytes).unwrap();
        println!("MANY:::: {:#?}", body);
        assert_ne!(body.items.len(), 0);
    }

    #[actix_web::test]
    async fn test_get_by_id() {
        let db_url = "postgresql://my_user:pass123@localhost:5432/my_db";
        let pool = PgPool::connect(db_url)
            .await
            .expect("Failed to connect to postgres");
        let app_state = application::State { pool };
        let app = test::init_service(
            App::new()
                .wrap(middleware::Compress::default())
                .app_data(web::Data::new(app_state.clone()))
                .configure(load_user_controller),
        )
        .await;
        let id: i32 = 1;
        let req = test::TestRequest::get()
            .uri(&format!("/users/{id}"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let bytes = test::read_body(resp).await;
        let body: DtoResponse<DtoUser> = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(body.item.id, id);
    }
}
