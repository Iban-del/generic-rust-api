pub struct AppState {
    pub db_state: crate::database::state::StateDataBase,
    pub service_registry: crate::service::ServiceRegistry,
}

impl AppState {
    pub async fn new(config: crate::config::Config) -> Result<Self, crate::error::AppError> {
        let db_state = crate::database::state::StateDataBase::new(config.database).await?;
        let service_registry = crate::service::ServiceRegistry::new(&db_state)?;
        Ok(Self {
            db_state,
            service_registry,
        })
    }
}
