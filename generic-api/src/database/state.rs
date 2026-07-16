pub struct DatabaseState {
    connectors: std::collections::HashMap<String, Box<dyn crate::database::connection::Connection>>,
}

impl DatabaseState {
    pub fn new(config: crate::config::Config) -> Self {
        let connectors: std::collections::HashMap<
            String,
            Box<dyn crate::database::connection::Connection>,
        > = Self::load_connectors(config);

        Self {
            connectors: connectors,
        }
    }

    fn load_connectors(
        config: crate::config::Config,
    ) -> std::collections::HashMap<String, Box<dyn crate::database::connection::Connection>> {
        let mut hs_conn: std::collections::HashMap<
            String,
            Box<dyn crate::database::connection::Connection>,
        > = std::collections::HashMap::new();
        let sec_db: crate::config::Database = config.database;

        // gestion des connecteur sql
        for conn in sec_db.sql_connectors.into_iter() {
            let conn_box: Box<dyn crate::database::connection::Connection> = match conn.db_type {
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
            hs_conn.insert(conn.alias, conn_box);
        }

        hs_conn
    }
}
