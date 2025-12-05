pub enum DbErrors {
    ConnectionError(sea_orm::DbErr),
    NotFound,
    uniqueViolation,

}