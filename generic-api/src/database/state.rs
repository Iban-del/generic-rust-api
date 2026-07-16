pub struct DatabaseState {
    sql_connectors: std::collections::HashMap<String, crate::database::connection::SqlConnector>,
    // plus tard :
    // neo4j_connectors: std::collections::HashMap<String, crate::database::connection::Neo4jConnector>,
}

impl DatabaseState {
    pub async fn new(config: crate::config::Config) -> Result<Self, crate::error::DatabaseError> {
        let sql_connectors = Self::load_sql_connectors(config).await?;
        Ok(Self { sql_connectors })
    }

    pub fn get_sql_connector(
        &self,
        alias: &str,
    ) -> Result<&crate::database::connection::SqlConnector, crate::error::DatabaseError> {
        self.sql_connectors.get(alias).ok_or_else(|| {
            crate::error::DatabaseError::ConnectorNotFound(format!(
                "The sql connector with alias '{}' not found",
                alias
            ))
        })
    }

    // plus tard :
    // pub fn get_neo4j_connector(&self, alias: &str) -> Result<&Neo4jConnector, ...> { ... }

    async fn load_sql_connectors(
        config: crate::config::Config,
    ) -> Result<
        std::collections::HashMap<String, crate::database::connection::SqlConnector>,
        crate::error::DatabaseError,
    > {
        let mut sql_connectors = std::collections::HashMap::new();
        let sec_db: crate::config::Database = config.database;

        for conn in sec_db.sql_connectors.into_iter() {
            let mut connector = match conn.db_type {
                crate::config::SqlDatabaseType::Postgres => {
                    crate::database::connection::SqlConnector::Postgres(
                        crate::database::connection::SqlConnection::<sqlx::Postgres>::new(
                            conn.url,
                            conn.max_connections,
                            conn.min_connections,
                        ),
                    )
                }
                crate::config::SqlDatabaseType::MySql => {
                    crate::database::connection::SqlConnector::MySql(
                        crate::database::connection::SqlConnection::<sqlx::MySql>::new(
                            conn.url,
                            conn.max_connections,
                            conn.min_connections,
                        ),
                    )
                }
            };
            connector.connect().await?;
            sql_connectors.insert(conn.alias, connector);
        }

        Ok(sql_connectors)
    }
}
