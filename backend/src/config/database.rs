// src/config/database.rs
use sea_orm::{Database, DatabaseConnection};
use std::env;
use dotenvy::dotenv;

/// Connects to the database using DATABASE_URL from .env
pub async fn connect() -> DatabaseConnection {
    // Load .env file
    dotenv().ok();

    // Get the database URL
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    Database::connect(&db_url)
        .await
        .expect("Failed to connect to the database")
}
