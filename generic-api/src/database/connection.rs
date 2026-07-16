pub struct SqlConnection<DB: sqlx::Database> {
    pool: Option<sqlx::Pool<DB>>,
    url: String,
    max_connections: u32,
    min_connections: u32,
}

impl<DB> SqlConnection<DB>
where
    DB: sqlx::Database,
{
    pub fn new(url: String, max_connections: u32, min_connections: u32) -> Self {
        Self {
            pool: None,
            url,
            max_connections,
            min_connections,
        }
    }

    pub async fn connect(&mut self) -> Result<(), crate::error::DatabaseError> {
        let pool = sqlx::pool::PoolOptions::<DB>::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .connect(&self.url)
            .await
            .map_err(|e| crate::error::DatabaseError::Sql(crate::error::SqlError::Sqlx(e)))?;

        self.pool = Some(pool);
        Ok(())
    }

    pub fn get_query(
        &self,
    ) -> Result<crate::database::query::SqlQuery<DB>, crate::error::DatabaseError> {
        let pool: sqlx::Pool<DB> = match &self.pool {
            Some(pl) => pl.clone(),
            None => {
                return Err(crate::error::DatabaseError::Sql(
                    crate::error::SqlError::PoolNotDefined("The pool is not defined".to_string()),
                ));
            }
        };

        Ok(crate::database::query::SqlQuery::new(pool))
    }
}

// ---- enum qui regroupe les connecteurs concrets pour la HashMap unique ----

pub enum SqlConnector {
    Postgres(SqlConnection<sqlx::Postgres>),
    MySql(SqlConnection<sqlx::MySql>),
}

impl SqlConnector {
    pub async fn connect(&mut self) -> Result<(), crate::error::DatabaseError> {
        match self {
            SqlConnector::Postgres(c) => c.connect().await,
            SqlConnector::MySql(c) => c.connect().await,
        }
    }
}
