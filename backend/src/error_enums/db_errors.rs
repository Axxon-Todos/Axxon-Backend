#[derive(thiserror::Error, Debug)]
pub enum DbErrors {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("Record not found")]
    NotFound,

    #[error("Unique constraint violated")]
    UniqueViolation,
}
