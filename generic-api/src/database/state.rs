pub struct DatabaseState {
    connectors: std::collections::HashMap<
        String,
        Box<dyn crate::database::connection::Connection + Send + Sync>,
    >,
}

impl DatabaseState {
    pub async fn new(config: crate::config::Config) -> Result<Self, crate::error::DatabaseError> {
        let connectors: std::collections::HashMap<
            String,
            Box<dyn crate::database::connection::Connection + Send + Sync>,
        > = Self::load_connectors(config).await?;

        Ok(Self {
            connectors: connectors,
        })
    }

    pub fn get_connectors(
        &self,
    ) -> &std::collections::HashMap<
        String,
        Box<dyn crate::database::connection::Connection + Send + Sync>,
    > {
        &self.connectors
    }

    pub fn get_query(
        &self,
        alias: &str,
    ) -> Result<Box<dyn crate::database::query::Query>, crate::error::DatabaseError> {
        let conn: &Box<dyn crate::database::connection::Connection + Send + Sync> =
            self.get_connector(alias)?;
        let query = conn.get_query()?;
        Ok(query)
    }

    pub fn get_connector(
        &self,
        alias: &str,
    ) -> Result<
        &Box<dyn crate::database::connection::Connection + Send + Sync>,
        crate::error::DatabaseError,
    > {
        for (key, conn) in &self.connectors {
            if key.eq(&alias.to_string()) {
                return Ok(conn);
            }
        }
        Err(crate::error::DatabaseError::ConnectorNotFound(format!(
            "The connector with alias '{}' not found",
            alias
        )))
    }

    async fn load_connectors(
        config: crate::config::Config,
    ) -> Result<
        std::collections::HashMap<
            String,
            Box<dyn crate::database::connection::Connection + Send + Sync>,
        >,
        crate::error::DatabaseError,
    > {
        let mut hs_conn: std::collections::HashMap<
            String,
            Box<dyn crate::database::connection::Connection + Send + Sync>,
        > = std::collections::HashMap::new();
        let sec_db: crate::config::Database = config.database;

        // gestion des connecteur sql
        for conn in sec_db.sql_connectors.into_iter() {
            let mut conn_box: Box<dyn crate::database::connection::Connection + Send + Sync> =
                match conn.db_type {
                    crate::config::SqlDatabaseType::Postgres => Box::new(
                        crate::database::connection::SqlConnection::<sqlx::Postgres>::new(
                            conn.url,
                            conn.max_connections,
                            conn.min_connections,
                        ),
                    ),
                    crate::config::SqlDatabaseType::MySql => Box::new(
                        crate::database::connection::SqlConnection::<sqlx::MySql>::new(
                            conn.url,
                            conn.max_connections,
                            conn.min_connections,
                        ),
                    ),
                };
            conn_box.connect().await?;
            hs_conn.insert(conn.alias, conn_box);
        }

        Ok(hs_conn)
    }
}
