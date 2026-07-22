#[generic_macros::route_space("")]
mod routes {

    #[generic_macros::route(HttpMethod::GET)]
    async fn home() -> Result<generic_api::http::AppResponse<String>, generic_api::http::AppError> {
        Ok(generic_api::response!("Hello world".to_string()))
    }

    #[generic_macros::route(HttpMethod::GET)]
    async fn test_service(
        axum::extract::State(app_state): axum::extract::State<
            std::sync::Arc<generic_api::application::state::AppState>,
        >,
    ) -> Result<generic_api::http::AppResponse<String>, generic_api::http::AppError> {
        let services = &app_state.service_registry;
        let test_service = services
            .get::<crate::application::service::TestService>()
            .unwrap();

        Ok(generic_api::response!(test_service.say_hello()))
    }
}
