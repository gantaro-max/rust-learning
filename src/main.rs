mod error;
mod models;
mod repositories;
mod services;
use crate::error::AppError;
use crate::models::DeleteRequest;
use crate::models::Item;
use crate::models::UpdateStockRequest;
use crate::repositories::item_repository::ItemRepositoryTrait;
use axum::extract::State;
use dotenvy::dotenv;
use repositories::item_repository::ItemRepository;
use services::ItemService;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

use axum::{
    Json, Router,
    http::StatusCode,
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
        .route("/", get(hello))
        .route("/api/items", get(get_items))
        .route("/api/items", post(add_items))
        .route("/api/items", patch(update_stock))
        .route("/api/items", delete(delete_item))
        .with_state(service)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("🚀 Webサーバーがポート8000で起動しました! http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn hello() -> &'static str {
    "Hello,Web World! Rust!Rust!Rust!"
}

async fn get_items(State(service): State<Arc<ItemService>>) -> Result<Json<Vec<Item>>, AppError> {
    let items = service.get_items().await?;

    Ok(Json(items))
}

async fn add_items(
    State(service): State<Arc<ItemService>>,
    Json(new_item): Json<Item>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let created_item = service.add_items(new_item).await?;

    Ok((StatusCode::CREATED, Json(created_item)))
}

async fn update_stock(
    State(service): State<Arc<ItemService>>,
    Json(up_req): Json<UpdateStockRequest>,
) -> Result<StatusCode, AppError> {
    service.update_stock(&up_req).await?;

    Ok(StatusCode::OK)
}

async fn delete_item(
    State(service): State<Arc<ItemService>>,
    Json(del_req): Json<DeleteRequest>,
) -> Result<StatusCode, AppError> {
    service.delete_item(&del_req).await?;

    Ok(StatusCode::OK)
}
