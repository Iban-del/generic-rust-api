pub struct SqlQuery<DB: sqlx::Database> {
    pool: sqlx::Pool<DB>,
}

impl<DB> SqlQuery<DB>
where
    DB: sqlx::Database,
{
    pub fn new(pool: sqlx::Pool<DB>) -> Self {
        Self { pool }
    }

    pub async fn fetch_all<'q, T>(
        &self,
        query: String,
        params: Vec<crate::database::sql::types::SqlType>,
    ) -> Result<crate::database::sql::resultset::ResultSet<T>, crate::error::SqlError>
    where
        for<'r> T: sqlx::FromRow<'r, <DB as sqlx::Database>::Row>,
        <DB as sqlx::Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
        i64: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        String: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        f64: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        bool: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        for<'c> &'c mut <DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = DB>,
    {
        let rows: Vec<<DB as sqlx::Database>::Row> = self
            .build_query::<'q>(query, params)?
            .fetch_all(&self.pool)
            .await?;

        Ok(crate::database::sql::resultset::ResultSet::new::<DB>(rows)?)
    }

    pub async fn execute<'q>(
        &self,
        query: String,
        params: Vec<crate::database::sql::types::SqlType>,
    ) -> Result<<DB as sqlx::Database>::QueryResult, crate::error::SqlError>
    where
        <DB as sqlx::Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
        i64: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        String: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        f64: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        bool: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        for<'c> &'c mut <DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = DB>,
    {
        let result: <DB as sqlx::Database>::QueryResult = self
            .build_query::<'q>(query, params)?
            .execute(&self.pool)
            .await?;

        Ok(result)
    }

    fn build_query<'q>(
        &self,
        query: String,
        params: Vec<crate::database::sql::types::SqlType>,
    ) -> Result<
        sqlx::query::Query<'q, DB, <DB as sqlx::Database>::Arguments<'q>>,
        crate::error::SqlError,
    >
    where
        <DB as sqlx::Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
        i64: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        String: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        f64: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
        bool: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    {
        let static_query: &'static str = Box::leak(query.into_boxed_str());

        let mut blt_query: sqlx::query::Query<'q, DB, <DB as sqlx::Database>::Arguments<'q>> =
            sqlx::query(static_query);

        for param in params.into_iter() {
            blt_query = match param {
                crate::database::sql::types::SqlType::Text(val) => blt_query.bind(val),
                crate::database::sql::types::SqlType::INumber(val) => blt_query.bind(val),
                crate::database::sql::types::SqlType::Float(val) => blt_query.bind(val),
                crate::database::sql::types::SqlType::Bool(val) => blt_query.bind(val),
                _ => {
                    return Err(crate::error::SqlError::UnsupportedType(
                        "Only the primitive types of `sqlType` are supported.".to_string(),
                    ));
                }
            };
        }

        Ok(blt_query)
    }
}
