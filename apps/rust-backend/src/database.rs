use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&db_url)
        .await
        .expect("Failed to connect to database")
}