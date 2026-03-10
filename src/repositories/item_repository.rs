use crate::models::Item;
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
}
