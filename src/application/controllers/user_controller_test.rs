#[cfg(test)]
mod tests {
    use actix_web::{middleware, test, App};

    use crate::application::controllers::user_controller::load_user_controller;
    use crate::application::dtos::response_message_dto::ResponseMessage;

    #[actix_web::test]
    async fn test_get_hello() {
        let app = test::init_service(
            App::new()
                .wrap(middleware::Compress::default())
                .configure(load_user_controller),
        )
        .await;
        let test_value = "my-test-value";
        let req = test::TestRequest::get()
            .uri(&format!("/users/hello/{test_value}"))
            .to_request();
        let resp: ResponseMessage = test::call_and_read_body_json(&app, req).await;
        let expected = format!("Hello {test_value}");

        assert_eq!(resp.message, expected);
    }

    #[actix_web::test]
    async fn test_get_hello_json() {
        let app = test::init_service(
            App::new()
                .wrap(middleware::Compress::default())
                .configure(load_user_controller),
        )
        .await;
        let test_value = "my-test-value";
        let url = format!("/users/hello-txt/{test_value}");
        let req = test::TestRequest::get().uri(&url).to_request();
        let resp = test::call_and_read_body(&app, req).await;

        let body = String::from_utf8(resp.to_vec()).expect("Error converting to string");
        let expected = format!("Hello {test_value}");

        assert_eq!(body, expected);
    }
}
