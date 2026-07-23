pub struct AppState {
    pub registry_manager: crate::registry::RegistryManager,
}

impl AppState {
    pub async fn new(config: crate::config::Config) -> Result<Self, crate::error::AppError> {
        let db_register = crate::database::registry::RegistryDatabase::new(config.database).await?;

        let registry_manager = crate::registry::RegistryManager::new(db_register);

        Ok(Self { registry_manager })
    }
}
