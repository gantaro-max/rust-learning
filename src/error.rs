use axum::{
    Json,  http::StatusCode, response::{IntoResponse,Response}
};

use serde_json::json;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum AppError {
    #[error("入力内容に不備があります：{0}")]
    BadRequest(String),
    #[error("データが見つかりませんでした")]
    NotFound,
    #[error("サーバー内部で予期せぬエラーが発生しました：{0}")]
    InternalServerError(String),    
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        let (status,message) = match self {
            Self::BadRequest(msg)=>(StatusCode::BAD_REQUEST,msg),
            Self::NotFound=>(StatusCode::NOT_FOUND,"Resource not found".to_string()),
            Self::InternalServerError(error)=>{
                eprintln!("ログ：{}",error);
                (StatusCode::INTERNAL_SERVER_ERROR,"InternalServerError".to_string())
            }
            
        };

        let body = Json(json!({"error":message}));
        (status,body).into_response()
    }
}

impl From<sqlx::Error> for AppError{
    fn from(value: sqlx::Error) -> Self {
        Self::InternalServerError(value.to_string())
    }
}