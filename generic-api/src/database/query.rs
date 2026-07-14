pub trait Query {}

// ---- query sql ----

pub struct SqlQuery {
    pool: sqlx::AnyPool,
}

impl Query for SqlQuery {}

impl SqlQuery {
    pub fn new(pool: sqlx::AnyPool) -> Self {
        Self { pool }
    }

    pub fn execute_from_sql_request(&self, sql_request: impl crate::database::sql::SqlRequest) {}
}
