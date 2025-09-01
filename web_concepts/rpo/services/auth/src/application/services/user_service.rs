use crate_errors::{AppError, Result};
use crate_security::PasswordManager;
use crate::domain::entities::{User, CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::domain::repositories::{UserRepository, RoleRepository};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
    role_repository: Arc<dyn RoleRepository>,
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        role_repository: Arc<dyn RoleRepository>,
    ) -> Self {
        Self {
            user_repository,
            role_repository,
        }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse> {
        request.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        // Check if user with email already exists
        if let Some(_) = self.user_repository.find_by_email(&request.email, request.tenant_id).await? {
            return Err(AppError::Conflict("User with this email already exists".to_string()));
        }

        // Check if user with username already exists  
        if let Some(_) = self.user_repository.find_by_username(&request.username, request.tenant_id).await? {
            return Err(AppError::Conflict("User with this username already exists".to_string()));
        }

        // Validate roles exist
        for role_id in &request.role_ids {
            if self.role_repository.find_by_id(*role_id).await?.is_none() {
                return Err(AppError::NotFound(format!("Role {} not found", role_id)));
            }
        }

        // Hash password
        let password_hash = PasswordManager::hash_password(&request.password)?;

        // Create user
        let user = User::new(request.clone(), password_hash);
        let created_user = self.user_repository.create(user).await?;

        // Assign roles
        if !request.role_ids.is_empty() {
            self.user_repository.assign_roles(created_user.id, request.role_ids).await?;
        }

        // Get user with roles and permissions
        self.get_user_response(created_user.id).await
    }

    pub async fn get_user(&self, id: Uuid) -> Result<UserResponse> {
        self.get_user_response(id).await
    }

    pub async fn update_user(&self, id: Uuid, request: UpdateUserRequest) -> Result<UserResponse> {
        request.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        // Check if user exists
        let user = self.user_repository.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Check for email conflicts
        if let Some(email) = &request.email {
            if let Some(existing_user) = self.user_repository.find_by_email(email, user.tenant_id).await? {
                if existing_user.id != id {
                    return Err(AppError::Conflict("User with this email already exists".to_string()));
                }
            }
        }

        // Check for username conflicts
        if let Some(username) = &request.username {
            if let Some(existing_user) = self.user_repository.find_by_username(username, user.tenant_id).await? {
                if existing_user.id != id {
                    return Err(AppError::Conflict("User with this username already exists".to_string()));
                }
            }
        }

        // Validate roles exist if provided
        if let Some(role_ids) = &request.role_ids {
            for role_id in role_ids {
                if self.role_repository.find_by_id(*role_id).await?.is_none() {
                    return Err(AppError::NotFound(format!("Role {} not found", role_id)));
                }
            }
        }

        // Update user
        self.user_repository.update(id, request).await?;

        // Return updated user with roles and permissions
        self.get_user_response(id).await
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        // Check if user exists
        if self.user_repository.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        self.user_repository.delete(id).await
    }

    pub async fn list_users(&self, tenant_id: Uuid, page: u64, page_size: u64) -> Result<Vec<UserResponse>> {
        let offset = page * page_size;
        let users = self.user_repository.list_by_tenant(tenant_id, page_size, offset).await?;
        
        let mut user_responses = Vec::new();
        for user in users {
            if let Ok(user_response) = self.get_user_response(user.id).await {
                user_responses.push(user_response);
            }
        }
        
        Ok(user_responses)
    }

    async fn get_user_response(&self, user_id: Uuid) -> Result<UserResponse> {
        let user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        
        let roles = self.user_repository.get_user_roles(user_id).await?;
        let permissions = self.user_repository.get_user_permissions(user_id).await?;

        Ok(UserResponse {
            id: user.id,
            tenant_id: user.tenant_id,
            email: user.email,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            is_active: user.is_active,
            is_verified: user.is_verified,
            last_login_at: user.last_login_at,
            roles,
            permissions,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }
}