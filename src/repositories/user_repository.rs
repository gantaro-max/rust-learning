use crate::error::AppError;
use crate::models::User;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn get_all_users(&self) -> Result<Vec<User>, AppError>;
    async fn create_user(&self, user: User) -> Result<User, AppError>;
    async fn find_by_user_id(&self, user_id: String) -> Result<Option<User>, AppError>;
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
    async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        let sql = "SELECT * FROM users";
        let users = sqlx::query_as::<_, User>(sql).fetch_all(&self.pool).await?;

        Ok(users)
    }

    async fn create_user(&self, user: User) -> Result<User, AppError> {
        let sql = r#"INSERT INTO users(user_id,user_name,password_hash)
         VALUES($1,$2,$3) RETURNING id,user_id,user_name,password_hash"#;
        let created_user = sqlx::query_as::<_, User>(sql)
            .bind(&user.user_id)
            .bind(&user.user_name)
            .bind(&user.password_hash)
            .fetch_one(&self.pool)
            .await?;

        Ok(created_user)
    }

    async fn find_by_user_id(&self, user_id: String) -> Result<Option<User>, AppError> {
        let sql = r#"SELECT * FROM users WHERE user_id = $1"#;
        let user = sqlx::query_as::<_, User>(sql)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct MockUserRepository {
    pub users: Vec<User>,
    pub error_type: Option<AppError>,
}

#[cfg(test)]
#[async_trait]
impl UserRepositoryTrait for MockUserRepository {
    async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        if let Some(error) = &self.error_type {
            return Err(error.clone());
        }
        Ok(self.users.clone())
    }

    async fn create_user(&self, user: User) -> Result<User, AppError> {
        if let Some(error) = &self.error_type {
            return Err(error.clone());
        }
        Ok(user)
    }

    async fn find_by_user_id(&self, _user_id: String) -> Result<Option<User>, AppError> {
        if let Some(error) = &self.error_type {
            return Err(error.clone());
        }
        Ok(Some(self.users[0].clone()))
    }
}

#[cfg(test)]
mod user_rep_test {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_get_all_users_ok(pool: PgPool) -> Result<(), AppError> {
        let mock_user = User {
            id: None,
            user_id: "mock@example.com".to_string(),
            user_name: "モックン".to_string(),
            password_hash: "abcdefg".to_string(),
        };
        let mock_repo = UserRepository::new(pool);

        let create_user = mock_repo.create_user(mock_user.clone()).await?;
        assert!(create_user.id.is_some());
        assert_eq!(create_user.user_name, "モックン");

        let users = mock_repo.get_all_users().await?;
        assert!(!users.is_empty());
        assert!(users.iter().any(|user| user.user_name == "モックン"));

        Ok(())
    }
}
