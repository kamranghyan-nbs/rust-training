use crate::{
    error::{AppError, not_found_error, conflict_error},
    models::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
    repository::auth::AuthRepositoryTrait,
    utils::{create_jwt, hash_password, verify_password},
};
use std::sync::Arc;

pub struct AuthService<T: AuthRepositoryTrait> {
    auth_repository: Arc<T>,
}

impl<T: AuthRepositoryTrait> AuthService<T> {
    pub fn new(auth_repository: Arc<T>) -> Self {
        Self { auth_repository }
    }

    pub async fn login(
        &self,
        config: Arc<crate::config::Config>,
        request: LoginRequest,
    ) -> Result<AuthResponse, AppError> {
        let user = self
            .auth_repository
            .find_by_username(&request.username)
            .await?
            .ok_or_else(|| AppError::unauthorized_with_context(
                format!("Invalid credentials for user: {}", request.username)
            ))?;

        let is_valid = verify_password(&request.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::unauthorized_with_context(
                "Invalid password provided".to_string()
            ));
        }

        let token = create_jwt(
            &user.id.to_string(),
            &user.username,
            &config.jwt_secret,
            config.jwt_expiration,
        )?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }

    pub async fn register(
        &self,
        config: Arc<crate::config::Config>,
        request: RegisterRequest,
    ) -> Result<AuthResponse, AppError> {
        // Check if user already exists
        let existing_user = self
            .auth_repository
            .find_by_username_or_email(&request.username, &request.email)
            .await?;

        if existing_user.is_some() {
            return Err(conflict_error(
                "user",
                &format!("User with username '{}' or email '{}' already exists", 
                    request.username, request.email)
            ));
        }

        let password_hash = hash_password(&request.password)?;

        let user = self
            .auth_repository
            .create_user(request.username.clone(), request.email.clone(), password_hash)
            .await?;

        let token = create_jwt(
            &user.id.to_string(),
            &user.username,
            &config.jwt_secret,
            config.jwt_expiration,
        )?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }
}