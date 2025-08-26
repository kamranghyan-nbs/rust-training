use crate::entities::user::UserRole;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(
        min = 3,
        max = 100,
        message = "Username must be between 3 and 100 characters"
    ))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    /// Optional role - defaults to 'user' if not specified
    /// Only admins can create other admins/managers
    pub role: Option<UserRole>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub username: String,
    pub role: UserRole, // Add role to JWT claims
    pub exp: usize,
}

/// Request to update user role (admin only)
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRoleRequest {
    #[validate(length(min = 1, message = "User ID is required"))]
    pub user_id: String,
}

/// Response for user role update
#[derive(Debug, Serialize)]
pub struct UpdateUserRoleResponse {
    pub user_id: Uuid,
    pub username: String,
    pub old_role: UserRole,
    pub new_role: UserRole,
    pub updated_by: String,
}
