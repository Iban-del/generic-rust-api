pub trait SqlRequest {
    fn build(&self) -> Result<String, crate::error::SqlError>;
}

pub mod clauses;
pub mod delete;
pub mod insert;
pub mod resultset;
pub mod select;
pub mod table;
pub mod types;
pub mod update;
