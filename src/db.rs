use sea_orm::{Database, DatabaseConnection};
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn get_session() -> DatabaseConnection {
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    Database::connect(db_url)
        .await
        .expect("Database connection failed")
}
