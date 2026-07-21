#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("I/O error : {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error : {0}")]
    Database(#[from] DatabaseError),

    #[error("Service error : {0}")]
    Service(#[from] ServiceError),
}
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("The service already exist : {0}")]
    ServiceAlreadyExist(String),

    #[error("Error loding service : {0}")]
    ErrorLoadingService(String),
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Pool not found: {0}")]
    PoolNotFound(String),

    #[error("Database connection error: {0}")]
    Connection(#[source] sea_orm::DbErr),

    #[error("Query execution error: {0}")]
    Query(#[source] sea_orm::DbErr),

    #[error("Record not found")]
    RecordNotFound,

    #[error("Record was not inserted")]
    RecordNotInserted,

    #[error("Record was not updated")]
    RecordNotUpdated,

    #[error("Failed to convert value from u64: {0}")]
    ConvertFromU64(String),

    #[error("Failed to unpack insert id")]
    UnpackInsertId,

    #[error("Failed to get primary key on update")]
    UpdateGetPrimaryKey,

    #[error("Type error: {0}")]
    Type(String),

    #[error("JSON error: {0}")]
    Json(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Attribute not set: {0}")]
    AttrNotSet(String),

    #[error("Connection pool acquire error: {0}")]
    ConnectionAcquire(String),

    #[error("Custom database error: {0}")]
    Custom(String),

    #[error("Connector with alias '{0}' not found")]
    ConnectorNotFound(String),

    #[error("Connector '{0}' does not match the requested database type")]
    ConnectorTypeMismatch(String),
}

impl From<sea_orm::DbErr> for DatabaseError {
    fn from(err: sea_orm::DbErr) -> Self {
        match err {
            sea_orm::DbErr::RecordNotFound(_) => DatabaseError::RecordNotFound,
            sea_orm::DbErr::RecordNotInserted => DatabaseError::RecordNotInserted,
            sea_orm::DbErr::RecordNotUpdated => DatabaseError::RecordNotUpdated,
            sea_orm::DbErr::ConvertFromU64(s) => DatabaseError::ConvertFromU64(s.to_string()),
            sea_orm::DbErr::UnpackInsertId => DatabaseError::UnpackInsertId,
            sea_orm::DbErr::UpdateGetPrimaryKey => DatabaseError::UpdateGetPrimaryKey,
            sea_orm::DbErr::Type(s) => DatabaseError::Type(s),
            sea_orm::DbErr::Json(s) => DatabaseError::Json(s),
            sea_orm::DbErr::Migration(s) => DatabaseError::Migration(s),
            sea_orm::DbErr::AttrNotSet(s) => DatabaseError::AttrNotSet(s),
            sea_orm::DbErr::Custom(s) => DatabaseError::Custom(s),
            sea_orm::DbErr::Conn(_) => DatabaseError::Connection(err),
            sea_orm::DbErr::Exec(_) => DatabaseError::Query(err),
            sea_orm::DbErr::Query(_) => DatabaseError::Query(err),
            sea_orm::DbErr::ConnectionAcquire(e) => DatabaseError::ConnectionAcquire(e.to_string()),
            _ => DatabaseError::Custom(err.to_string()),
        }
    }
}
