use crate::{
    entities::user::UserRole,
    error::{conflict_error, AppError},
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
            .ok_or_else(|| {
                AppError::unauthorized_with_context(format!(
                    "Invalid credentials for user: {}",
                    request.username
                ))
            })?;

        // Check if user is active
        if !user.is_active {
            return Err(AppError::forbidden(format!(
                "User account '{}' is deactivated",
                user.username
            )));
        }

        let is_valid = verify_password(&request.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::unauthorized_with_context(
                "Invalid password provided".to_string(),
            ));
        }

        // Create JWT with role information
        let token = create_jwt(
            &user.id.to_string(),
            &user.username,
            &user.role,
            &config.jwt_secret,
            config.jwt_expiration,
        )?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                role: user.role,
                is_active: user.is_active,
            },
        })
    }

    pub async fn register(
        &self,
        config: Arc<crate::config::Config>,
        request: RegisterRequest,
        requesting_user_role: Option<UserRole>,
    ) -> Result<AuthResponse, AppError> {
        // Check if user already exists
        let existing_user = self
            .auth_repository
            .find_by_username_or_email(&request.username, &request.email)
            .await?;

        if existing_user.is_some() {
            return Err(conflict_error(
                "user",
                &format!(
                    "User with username '{}' or email '{}' already exists",
                    request.username, request.email
                ),
            ));
        }

        // Determine the role for the new user
        let user_role = self.determine_user_role(request.role, requesting_user_role)?;

        let password_hash = hash_password(&request.password)?;

        let user = self
            .auth_repository
            .create_user(
                request.username.clone(),
                request.email.clone(),
                password_hash,
                user_role.clone(),
            )
            .await?;

        // Create JWT with role information
        let token = create_jwt(
            &user.id.to_string(),
            &user.username,
            &user.role,
            &config.jwt_secret,
            config.jwt_expiration,
        )?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                role: user.role,
                is_active: user.is_active,
            },
        })
    }

    /// Determine what role a new user should have based on request and permissions
    fn determine_user_role(
        &self,
        requested_role: Option<UserRole>,
        requesting_user_role: Option<UserRole>,
    ) -> Result<UserRole, AppError> {
        match (requested_role, requesting_user_role) {
            // No role requested - default to User
            (None, _) => Ok(UserRole::User),
            // Role requested but no requesting user (public registration) - only allow User
            (Some(UserRole::User), None) => Ok(UserRole::User),
            (Some(UserRole::Manager | UserRole::Admin), None) => {
                Err(AppError::InsufficientPrivileges {
                    required_role: "Admin".to_string(),
                    current_role: None,
                    error_id: uuid::Uuid::new_v4(),
                })
            }
            // Role requested with requesting user context
            (Some(requested), Some(requester_role)) => {
                match (requested, requester_role) {
                    // Anyone can create User accounts
                    (UserRole::User, _) => Ok(UserRole::User),
                    // Only admins can create Manager or Admin accounts
                    (UserRole::Manager, UserRole::Admin) => Ok(UserRole::Manager),
                    (UserRole::Admin, UserRole::Admin) => Ok(UserRole::Admin),
                    // Managers cannot create other Managers or Admins
                    (UserRole::Manager | UserRole::Admin, UserRole::Manager) => {
                        Err(AppError::InsufficientPrivileges {
                            required_role: "Admin".to_string(),
                            current_role: Some("Manager".to_string()),
                            error_id: uuid::Uuid::new_v4(),
                        })
                    }
                    // Regular users cannot create privileged accounts
                    (UserRole::Manager | UserRole::Admin, UserRole::User) => {
                        Err(AppError::InsufficientPrivileges {
                            required_role: "Admin".to_string(),
                            current_role: Some("User".to_string()),
                            error_id: uuid::Uuid::new_v4(),
                        })
                    }
                }
            }
        }
    }
}
