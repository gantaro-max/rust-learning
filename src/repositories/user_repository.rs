use crate::error::AppError;
use crate::models::Users;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn get_all_users(&self) -> Result<Vec<Users>, AppError>;
    async fn create_user(&self, user: Users) -> Result<Users, AppError>;
    async fn find_by_user_id(&self, user_id: String) -> Result<Option<Users>, AppError>;
}

pub struct UserRepository {
    pool: sqlx::PgPool,
}
impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn get_all_users(&self) -> Result<Vec<Users>, AppError> {
        let sql = "SELECT * FROM users";
        let users = sqlx::query_as::<_, Users>(sql)
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn create_user(&self, user: Users) -> Result<Users, AppError> {
        let sql = r#"INSERT INTO users(user_id,user_name,password_hash)
         VALUES($1,$2,$3) RETURNING id,user_id,user_name,password_hash"#;
        let created_user = sqlx::query_as::<_, Users>(sql)
            .bind(&user.user_id)
            .bind(&user.user_name)
            .bind(&user.password_hash)
            .fetch_one(&self.pool)
            .await?;

        Ok(created_user)
    }

    async fn find_by_user_id(&self, user_id: String) -> Result<Option<Users>, AppError> {
        let sql = r#"SELECT * FROM users WHERE user_id = $1"#;
        let user = sqlx::query_as::<_, Users>(sql)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
