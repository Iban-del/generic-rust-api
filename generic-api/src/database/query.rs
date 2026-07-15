pub trait Query {}

// ---- query sql ----

pub struct SqlQuery<DB: sqlx::Database> {
    pool: sqlx::Pool<DB>,
}

impl<DB> Query for SqlQuery<DB> where DB: sqlx::Database {}

impl<DB> SqlQuery<DB>
where
    DB: sqlx::Database,
{
    pub fn new(pool: sqlx::Pool<DB>) -> Self {
        Self { pool }
    }

    fn build_query<'q>(
        query: String,
        params: Vec<crate::database::sql::types::SqlType>,
    ) -> Result<
        sqlx::query::Query<'q, DB, <DB as sqlx::Database>::Arguments<'q>>,
        crate::error::SqlError,
    > {
        let static_query: &'static str = Box::leak(query.into_boxed_str());

        let mut blt_query: sqlx::query::Query<'q, DB, <DB as sqlx::Database>::Arguments<'q>> =
            sqlx::query::<DB>(static_query);

        for param in params.into_iter() {
            //blt_query = blt_query.bind(param);
        }

        Ok(blt_query)
    }
}
