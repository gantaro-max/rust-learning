mod error;
mod handlers;
mod models;
mod repositories;
mod services;
use crate::repositories::item_repository::ItemRepositoryTrait;
use crate::services::item_service::ItemService;
use dotenvy::dotenv;
use repositories::item_repository::ItemRepository;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let repository: Arc<dyn ItemRepositoryTrait> = Arc::new(ItemRepository::new(pool));
    let service = Arc::new(ItemService::new(Arc::clone(&repository)));
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/api/items", get(handlers::item_handler::get_items))
        .route("/api/items", post(handlers::item_handler::add_items))
        .route("/api/items", patch(handlers::item_handler::update_stock))
        .route("/api/items", delete(handlers::item_handler::delete_item))
        .route(
            "/api/items/search",
            get(handlers::item_handler::find_by_name),
        )
        .with_state(service)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("🚀 Webサーバーがポート8000で起動しました! http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
