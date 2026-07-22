use generic_macros::service;

#[service]
pub struct TestService {}

impl TestService {
    pub fn say_hello(&self) -> String {
        String::from("Hello")
    }
}

impl generic_api::service::StartableService for TestService {
    fn build(db_state: &generic_api::database::state::StateDataBase) -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}

impl Clone for TestService {
    fn clone(&self) -> Self {
        Self {}
    }
}
impl axum::extract::FromRef<::generic_api::application::state::AppState> for TestService {
    fn from_ref(state: &::generic_api::application::state::AppState) -> Self {
        state
            .service_registry
            .get::<TestService>()
            .clone()
            .expect(format!("The service {} not found!", stringify!(TestService)).as_str())
            .clone()
    }
}
