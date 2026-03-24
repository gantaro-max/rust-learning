use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Type)]
#[sqlx(type_name = "varchar")]
pub enum Category {
    #[serde(rename = "果物")]
    Fruit,
    #[serde(rename = "飲み物")]
    Drink,
    #[serde(rename = "日用品")]
    DailyNecessity,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Item {
    pub id: Option<i32>,
    pub name: String,
    pub price: i32,
    pub stock: i32,
    pub category: Category,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateStockRequest {
    pub id: i32,
    pub stock: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteRequest {
    pub id: i32,
}

#[derive(Debug, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub user_id: String,
    pub user_name: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserRequest {
    pub user_id: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub id: Option<i32>,
    pub user_id: String,
    pub user_name: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            user_id: user.user_id,
            user_name: user.user_name,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_res: UserResponse,
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub subject: String,
    pub issued_at: usize,
    pub exp_time: usize,
}
