use crate::{
    auth,
    error::AppError,
    models::{User, UserRequest, UserResponse},
    repositories::user_repository::UserRepositoryTrait,
};
use std::sync::Arc;

pub struct UserService {
    user_repository: Arc<dyn UserRepositoryTrait>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryTrait>) -> Self {
        Self { user_repository }
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, AppError> {
        let users = self.user_repository.get_all_users().await?;
        let response = users.into_iter().map(|user_rep| user_rep.into()).collect();

        Ok(response)
    }

    pub async fn create_user(&self, user_req: UserRequest) -> Result<UserResponse, AppError> {
        let hashed_pass = auth::hash_password(&user_req.password)?;
        let new_user = User {
            id: None,
            user_id: user_req.user_id,
            user_name: user_req.user_name,
            password_hash: hashed_pass,
        };
        let created = self.user_repository.create_user(new_user).await?;

        Ok(created.into())
    }
}
