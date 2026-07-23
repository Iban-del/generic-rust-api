pub struct RegistryManager {
    db_register: crate::database::registry::RegistryDatabase,
    service_register: crate::service::ServiceRegistry,
}

impl RegistryManager {
    /// Construit db + service_register (qui ne fait qu'enregistrer
    /// les builders, sans les exécuter) → aucune boucle possible ici.
    pub fn new(db_register: crate::database::registry::RegistryDatabase) -> Self {
        Self {
            db_register,
            service_register: crate::service::ServiceRegistry::new(),
        }
    }

    pub fn db(&self) -> &crate::database::registry::RegistryDatabase {
        &self.db_register
    }

    /// Construit (ou récupère) un service à la demande.
    /// `&self` sert de RegistryManager pour la construction du
    /// service, y compris s'il dépend d'autres services.
    pub fn get<T: crate::service::StartableService>(&self) -> std::sync::Arc<T> {
        self.service_register.get(self)
    }

    pub fn get_database(
        &self,
        alias: String,
    ) -> Result<&sea_orm::DatabaseConnection, crate::error::DatabaseError> {
        self.db_register.get_sql_connection(alias)
    }
}
