pub struct AppState {
    pub db_state: crate::database::state::StateDataBase,
}

impl AppState {
    pub async fn new(config: crate::config::Config) -> Result<Self, crate::error::AppError> {
        let db_state = crate::database::state::StateDataBase::new(config.database).await?;
        Ok(Self { db_state })
    }
}
