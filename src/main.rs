mod models;
mod repositories;
use crate::models::DeleteRequest;
use crate::models::Item;
use crate::models::UpdateStockRequest;
use axum::extract::State;
use dotenvy::dotenv;
use repositories::item_repository::ItemRepository;
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

    let repository = Arc::new(ItemRepository::new(pool));

    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/", get(hello))
        .route("/api/items", get(get_items))
        .route("/api/items", post(add_items))
        .route("/api/items", patch(update_stock))
        .route("/api/items", delete(delete_item))
        .with_state(repository)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("🚀 Webサーバーがポート8000で起動しました! http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn hello() -> &'static str {
    "Hello,Web World! Rust!Rust!Rust!"
}

async fn get_items(
    State(repository): State<Arc<ItemRepository>>,
) -> Result<Json<Vec<Item>>, (StatusCode, String)> {
    let items = repository.fetch_all().await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("見つかりませんでした:{}", e)),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("予期せぬエラーが発生しました:{}", e),
        ),
    })?;

    Ok(Json(items))
}

async fn add_items(
    State(repository): State<Arc<ItemRepository>>,
    Json(new_item): Json<Item>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository.create(new_item).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("予期せぬエラーが発生しました:{}", e),
        )
    })?;

    Ok(StatusCode::CREATED)
}

async fn update_stock(
    State(repository): State<Arc<ItemRepository>>,
    Json(up_req): Json<UpdateStockRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let rows = repository.update_stock(&up_req).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("予期せぬエラーが発生しました:{}", e),
        )
    })?;

    if rows == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            format!("見つかりませんでした id:{}", up_req.id),
        ));
    }

    Ok(StatusCode::OK)
}

async fn delete_item(
    State(repository): State<Arc<ItemRepository>>,
    Json(del_req): Json<DeleteRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let rows = repository.delete(&del_req).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("予期せぬエラーが発生しました:{}", e),
        )
    })?;

    if rows == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            format!("見つかりませんでした id:{}", del_req.id),
        ));
    }

    Ok(StatusCode::OK)
}
