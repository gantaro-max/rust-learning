mod auth;
mod error;
mod handlers;
mod models;
mod repositories;
mod services;
mod state;
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};
use crate::services::item_service::ItemService;
use crate::state::AppStates;
use crate::{
    repositories::item_repository::ItemRepositoryTrait, services::user_service::UserService,
};
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

    let item_repo: Arc<dyn ItemRepositoryTrait> = Arc::new(ItemRepository::new(pool.clone()));
    let user_repo: Arc<dyn UserRepositoryTrait> = Arc::new(UserRepository::new(pool));
    let app_states = Arc::new(AppStates {
        item_service: Arc::new(ItemService::new(item_repo)),
        user_service: Arc::new(UserService::new(user_repo)),
    });

    let cors = CorsLayer::permissive();
    let app = Router::new()
        .nest("auth", handlers::user_handler::auth_routes())
        .with_state(app_states)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("🚀 Webサーバーがポート8000で起動しました! http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
