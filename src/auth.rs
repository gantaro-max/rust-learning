use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use std::env;

use crate::{error::AppError, models::Claims};
// ハッシュ生成
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .to_string();

    Ok(password_hash)
}
// パスワードとハッシュ化パスワードの比較
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn create_jwt(user_id: &str) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        subject: user_id.to_owned(),
        issued_at: Utc::now().timestamp() as usize,
        exp_time: expiration,
    };

    let secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::InternalServerError(e.to_string()))
}
