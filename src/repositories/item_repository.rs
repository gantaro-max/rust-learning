use crate::models::DeleteRequest;
use crate::models::Item;
use crate::models::UpdateStockRequest;
use sqlx::PgPool;

pub struct ItemRepository {
    pool: PgPool,
}
impl ItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn fetch_all(&self) -> Result<Vec<Item>, sqlx::Error> {
        sqlx::query_as::<_, Item>("SELECT id,name,price,stock,category FROM items")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create(&self, item: Item) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO items(name,price,stock,category) VALUES($1,$2,$3,$4)",
            item.name,
            item.price,
            item.stock,
            item.category as _
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_stock(&self, up_stock: &UpdateStockRequest) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE items SET stock=$1 WHERE id=$2",
            up_stock.stock,
            up_stock.id
        )
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(&self, del_req: &DeleteRequest) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM items WHERE id=$1", &del_req.id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
