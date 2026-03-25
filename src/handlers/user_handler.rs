use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};

use crate::{
    error::AppError,
    models::{LoginResponse, UserRequest, UserResponse},
    state::AppStates,
};

pub fn user_routes() -> Router<Arc<AppStates>> {
    Router::new()
        .route("/login", post(log_in))
        .route("/signup", post(create_user))
}

pub fn admin_routes() -> Router<Arc<AppStates>> {
    Router::new().route("/users", get(get_all_users))
}

pub async fn log_in(
    State(state): State<Arc<AppStates>>,
    Json(user_req): Json<UserRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let login_user = state.user_service.log_in(user_req).await?;

    Ok(Json(login_user))
}

pub async fn create_user(
    State(state): State<Arc<AppStates>>,
    Json(user_req): Json<UserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.user_service.create_user(user_req).await?;

    Ok(Json(user))
}
pub async fn get_all_users(
    State(state): State<Arc<AppStates>>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = state.user_service.get_all_users().await?;

    Ok(Json(users))
}
