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
