use axum::{routing::get, Extension, Router};
mod db;
mod models;
mod routes;
use db::{get_session, AppState};
use migration::{Migrator, MigratorTrait};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db = get_session().await;
    let _ = Migrator::up(&db, None).await;
    let api_routes = Router::new().nest("/account", routes::account::router());
    let state = Arc::new(AppState { db });
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", api_routes)
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
