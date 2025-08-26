use crate::{entities::user::UserRole, error::AppError, models::Claims};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create_jwt(
    user_id: &str,
    username: &str,
    role: &UserRole, // changed here
    secret: &str,
    expiration: i64,
) -> Result<String, AppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::InternalServerError {
            context: Some("JWT signing failed".to_string()),
            error_id: uuid::Uuid::new_v4(),
        })?
        .as_secs();

    let claims = Claims {
        sub: user_id.to_owned(),
        username: username.to_owned(),
        role: role.clone(), // `UserRole` is `Clone` (derive it if needed)
        exp: (now + expiration as u64) as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(AppError::from)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(AppError::from)
}
