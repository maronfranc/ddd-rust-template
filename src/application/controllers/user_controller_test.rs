#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{middleware, test, web, App, HttpMessage};
    use serde_json;
    use sqlx::PgPool;

    use crate::application;
    use crate::application::controllers::user_controller::load_user_controller;
    use crate::application::dtos::response_message_dto::ResponseMessage;

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
        let code = resp.status();
        assert_eq!(code, StatusCode::OK);

        let bytes = test::read_body(resp).await;
        let body: ResponseMessage = serde_json::from_slice(&bytes).unwrap();
        let expected = "Hello many";

        assert_eq!(body.message, expected);
    }

    // #[actix_web::test]
    // async fn test_get_by_id() {
    //     let db_url = "postgresql://my_user:pass123@localhost:5432/my_db";
    //     let pool = PgPool::connect(db_url)
    //         .await
    //         .expect("Failed to connect to postgres");
    //     let app_state = application::State { pool };
    //     let app = test::init_service(
    //         App::new()
    //             .wrap(middleware::Compress::default())
    //             .app_data(web::Data::new(app_state.clone()))
    //             .configure(load_user_controller),
    //     )
    //     .await;
    //     let test_value: i32 = 1;
    //     let req = test::TestRequest::get()
    //         .uri(&format!("/users/{test_value}"))
    //         .to_request();
    //     let resp: ResponseMessage = test::call_and_read_body_json(&app, req).await;
    //     let expected = format!("Hello {test_value}");
    //
    //     assert_eq!(resp.message, expected);
    // }
}
