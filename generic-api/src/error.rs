#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("I/O error : {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error : {0}")]
    Database(#[from] DatabaseError),
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Failed to load the connector : {0}")]
    FailedLoadConnector(String),

    #[error("Sql error : {0}")]
    Sql(SqlError),
}

#[derive(Debug, thiserror::Error)]
pub enum SqlError {
    #[error("Sqlx error : {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Pool not found : {0}")]
    PoolNotDefined(String),

    #[error("The predicate not found : {0}")]
    PredicateNotExist(String),
}
