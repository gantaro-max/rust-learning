use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Type)]
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
impl Item {
    pub fn new(name: String, price: i32, stock: i32, category: Category) -> Self {
        Self {
            id: None,
            name,
            price,
            stock,
            category,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateStockRequest {
    pub name: String,
    pub stock: i32,
}
