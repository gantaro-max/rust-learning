mod models;
mod repositories;
use crate::models::Item;
use crate::models::UpdateStockRequest;
use repositories::item_repository::ItemRepository;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use std::fs;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/", get(hello))
        .route("/api/items", get(get_items))
        .route("/api/items", post(add_items))
        .route("/api/items", patch(update_stock))
        .route("/api/items", delete(delete_item))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("🚀 Webサーバーがポート8000で起動しました! http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello,Web World! Rust!Rust!Rust!"
}

async fn get_list() -> Result<Vec<Item>, (StatusCode, String)> {
    let json_string = fs::read_to_string("./inventory.json").map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            format!("ファイルが見つかりません:{}", e),
        )
    })?;

    let items = serde_json::from_str(&json_string).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("予期せぬエラーが発生しました:{}", e),
        )
    })?;
    Ok(items)
}

async fn save_list(items: &Vec<Item>) -> Result<(), (StatusCode, String)> {
    let new_json = serde_json::to_string_pretty(&items).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("予期せぬエラーが発生しました:{}", e),
        )
    })?;
    fs::write("./inventory.json", new_json).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("予期せぬエラーが発生しました:{}", e),
        )
    })?;

    Ok(())
}

async fn get_items() -> Result<Json<Vec<Item>>, (StatusCode, String)> {
    let items: Vec<Item> = get_list().await?;

    Ok(Json(items))
}

async fn add_items(Json(new_item): Json<Item>) -> Result<StatusCode, (StatusCode, String)> {
    let mut items: Vec<Item> = get_list().await?;
    items.push(new_item);

    save_list(&items).await?;

    Ok(StatusCode::CREATED)
}

async fn update_stock(
    Json(up_req): Json<UpdateStockRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut items: Vec<Item> = get_list().await?;
    if let Some(result) = items.iter_mut().find(|item| item.name == up_req.name) {
        result.stock = up_req.stock
    } else {
        return Err((
            StatusCode::NOT_FOUND,
            format!("{}は見つかりません", up_req.name),
        ));
    }

    save_list(&items).await?;

    Ok(StatusCode::OK)
}

async fn delete_item(Json(item_name): Json<String>) -> Result<StatusCode, (StatusCode, String)> {
    let items: Vec<Item> = get_list().await?;
    let items_length = items.len();
    let new_items: Vec<Item> = items
        .into_iter()
        .filter(|item| item.name != item_name)
        .collect();

    if items_length == new_items.len() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("{}は見つかりません", item_name),
        ));
    }

    save_list(&new_items).await?;

    Ok(StatusCode::OK)
}
