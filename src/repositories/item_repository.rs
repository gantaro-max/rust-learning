use crate::error::AppError;
use crate::models::DeleteRequest;
use crate::models::Item;
use crate::models::UpdateStockRequest;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait ItemRepositoryTrait: Send + Sync {
    async fn create(&self, item: Item) -> Result<Item, AppError>;
    async fn fetch_all(&self) -> Result<Vec<Item>, AppError>;
    async fn update_stock(&self, up_req: &UpdateStockRequest) -> Result<u64, AppError>;
    async fn delete(&self, del_req: &DeleteRequest) -> Result<u64, AppError>;
}
#[cfg(test)]
pub struct MockRepository {
    pub items: Vec<Item>,
    pub error_type: Option<AppError>,
    pub affected_row: u64,
}
#[cfg(test)]
#[async_trait]
impl ItemRepositoryTrait for MockRepository {
    async fn create(&self, item: Item) -> Result<Item, AppError> {
        if let Some(err) = &self.error_type {
            return Err(err.clone());
        }
        Ok(item)
    }
    async fn fetch_all(&self) -> Result<Vec<Item>, AppError> {
        if let Some(err) = &self.error_type {
            return Err(err.clone());
        }
        Ok(self.items.clone())
    }
    async fn update_stock(&self, _up_req: &UpdateStockRequest) -> Result<u64, AppError> {
        if let Some(err) = &self.error_type {
            return Err(err.clone());
        }
        Ok(self.affected_row)
    }
    async fn delete(&self, _del_req: &DeleteRequest) -> Result<u64, AppError> {
        if let Some(err) = &self.error_type {
            return Err(err.clone());
        }
        Ok(self.affected_row)
    }
}

pub struct ItemRepository {
    pool: PgPool,
}
impl ItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ItemRepositoryTrait for ItemRepository {
    async fn fetch_all(&self) -> Result<Vec<Item>, AppError> {
        let items = sqlx::query_as::<_, Item>("SELECT id,name,price,stock,category FROM items")
            .fetch_all(&self.pool)
            .await?;

        Ok(items)
    }

    async fn create(&self, item: Item) -> Result<Item, AppError> {
        let created_item = sqlx::query_as!(
            Item,
            r#"INSERT INTO items(name,price,stock,category) VALUES($1,$2,$3,$4) RETURNING id,name,price,stock,category as "category: _""#,
            item.name,
            item.price,
            item.stock,
            item.category as _
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(created_item)
    }

    async fn update_stock(&self, up_stock: &UpdateStockRequest) -> Result<u64, AppError> {
        let result = sqlx::query!(
            "UPDATE items SET stock=$1 WHERE id=$2",
            up_stock.stock,
            up_stock.id
        )
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    async fn delete(&self, del_req: &DeleteRequest) -> Result<u64, AppError> {
        let result = sqlx::query!("DELETE FROM items WHERE id=$1", &del_req.id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
