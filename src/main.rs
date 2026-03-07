use axum::{Json, Router, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    name: String,
    price: u32,
    stock: u32,
    category: Category,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "果物")]
    Fruit,
    #[serde(rename = "飲み物")]
    Drink,
    #[serde(rename = "日用品")]
    DailyNecessity,
}
impl Item {
    pub fn new(name: String, price: u32, stock: u32, category: Category) -> Self {
        Self {
            name,
            price,
            stock,
            category,
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/api/items", get(get_items));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!(
        "🚀 Webサーバーがポート8080で起動しました! http://localhost:8080にアクセスしてください"
    );

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello,Web World! Rust!Rust!Rust!"
}

async fn get_items() -> Result<Json<Vec<Item>>, (StatusCode, String)> {
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

    Ok(Json(items))
}
