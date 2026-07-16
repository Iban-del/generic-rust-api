#[derive(sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
}

pub struct ResultSet<M> {
    data: Vec<M>,
    index: usize,
}

impl<M> ResultSet<M> {
    pub fn new<DB>(rows: Vec<<DB as sqlx::Database>::Row>) -> Result<Self, sqlx::Error>
    where
        DB: sqlx::Database,
        M: for<'r> sqlx::FromRow<'r, <DB as sqlx::Database>::Row>,
    {
        let data: Vec<M> = rows
            .iter()
            .map(M::from_row)
            .collect::<Result<Vec<M>, sqlx::Error>>()?;

        Ok(Self { data, index: 0 })
    }

    pub fn get_data(&self) -> &Vec<M> {
        &self.data
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn current(&self) -> Result<&M, crate::error::ResultSetError> {
        if self.count() > 0 {
            return Ok(&self.data[0]);
        }
        Err(crate::error::ResultSetError::EmptyError())
    }
}

impl<'a, M> Iterator for ResultSet<M> {
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = self.data.remove(self.index);
            Some(item)
        } else {
            None
        }
    }
}
