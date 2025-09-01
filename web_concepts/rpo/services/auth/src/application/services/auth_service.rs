use crate_errors::{AppError, Result};
use crate_security::{Claims, JwtManager, PasswordManager};
use crate::domain::entities::*;
use crate::domain::repositories::*;
use crate::domain::value_objects::*;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    tenant_repository: Arc<dyn TenantRepository>,
    session_repository: Arc<dyn SessionRepository>,
    jwt_manager: JwtManager,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        tenant_repository: Arc<dyn TenantRepository>,
        session_repository: Arc<dyn SessionRepository>,
        jwt_manager: JwtManager,
    ) -> Self {
        Self {
            user_repository,
            tenant_repository,
            session_repository,
            jwt_manager,
        }
    }

    pub async fn login(&self, request: LoginRequest, ip_address: Option<String>, user_agent: Option<String>) -> Result<LoginResponse> {
        request.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        // Find tenant if tenant_code is provided
        let tenant_id = if let Some(tenant_code) = &request.tenant_code {
            let tenant = self.tenant_repository.find_by_code(tenant_code).await?
                .ok_or_else(|| AppError::Authentication("Invalid tenant code".to_string()))?;
            tenant.id
        } else {
            // For single-tenant setups, you might want to use a default tenant
            // or infer from domain, etc.
            return Err(AppError::Authentication("Tenant code required".to_string()));
        };

        // Find user by email
        let mut user = self.user_repository.find_by_email(&request.email, tenant_id).await?
            .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

        // Check if user is locked
        if user.is_locked() {
            return Err(AppError::Authentication("Account is temporarily locked".to_string()));
        }

        // Check if user is active
        if !user.is_active {
            return Err(AppError::Authentication("Account is disabled".to_string()));
        }

        // Verify password
        if !PasswordManager::verify_password(&request.password, &user.password_hash)? {
            user.increment_failed_login();
            self.user_repository.update(user.id, UpdateUserRequest {
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                phone: None,
                is_active: None,
                role_ids: None,
            }).await?;
            return Err(AppError::Authentication("Invalid credentials".to_string()));
        }

        // Reset failed login attempts on successful login
        user.reset_failed_login();
        self.user_repository.update(user.id, UpdateUserRequest {
            email: None,
            username: None,
            first_name: None,
            last_name: None,
            phone: None,
            is_active: None,
            role_ids: None,
        }).await?;

        // Get user roles and permissions
        let roles = self.user_repository.get_user_roles(user.id).await?;
        let permissions = self.user_repository.get_user_permissions(user.id).await?;

        // Create JWT claims
        let claims = Claims {
            sub: user.id,
            tenant_id,
            email: user.email.clone(),
            roles: roles.clone(),
            permissions: permissions.clone(),
            exp: 0, // Will be set by create_access_token
            iat: 0, // Will be set by create_access_token
            iss: String::new(), // Will be set by create_access_token
        };

        // Generate tokens
        let access_token = self.jwt_manager.create_access_token(&claims)?;
        let refresh_token = self.jwt_manager.create_refresh_token(user.id, tenant_id)?;

        // Create session
        let session = Session::new(
            user.id,
            tenant_id,
            refresh_token.clone(),
            ip_address,
            user_agent,
            Utc::now() + Duration::days(7), // Refresh token expires in 7 days
        );
        self.session_repository.create(session).await?;

        // Prepare response
        let user_response = UserResponse {
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
        };

        Ok(LoginResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 3600, // 1 hour in seconds
            user: user_response,
        })
    }

    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> Result<LoginResponse> {
        request.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        // Verify refresh token
        let claims = self.jwt_manager.verify_token(&request.refresh_token)?;

        // Find and validate session
        let session = self.session_repository.find_by_token(&request.refresh_token).await?
            .ok_or_else(|| AppError::Authentication("Invalid refresh token".to_string()))?;

        if !session.is_active || session.is_expired() {
            return Err(AppError::Authentication("Refresh token expired or inactive".to_string()));
        }

        // Get user
        let user = self.user_repository.find_by_id(claims.sub).await?
            .ok_or_else(|| AppError::Authentication("User not found".to_string()))?;

        if !user.is_active {
            return Err(AppError::Authentication("Account is disabled".to_string()));
        }

        // Get user roles and permissions
        let roles = self.user_repository.get_user_roles(user.id).await?;
        let permissions = self.user_repository.get_user_permissions(user.id).await?;

        // Create new JWT claims
        let new_claims = Claims {
            sub: user.id,
            tenant_id: user.tenant_id,
            email: user.email.clone(),
            roles: roles.clone(),
            permissions: permissions.clone(),
            exp: 0,
            iat: 0,
            iss: String::new(),
        };

        // Generate new access token
        let access_token = self.jwt_manager.create_access_token(&new_claims)?;

        let user_response = UserResponse {
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
        };

        Ok(LoginResponse {
            access_token,
            refresh_token: request.refresh_token, // Return the same refresh token
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: user_response,
        })
    }

    pub async fn validate_token(&self, token: &str) -> Result<TokenValidationResponse> {
        match self.jwt_manager.verify_token(token) {
            Ok(claims) => {
                // Check if user still exists and is active
                if let Ok(Some(user)) = self.user_repository.find_by_id(claims.sub).await {
                    if user.is_active && !user.is_locked() {
                        return Ok(TokenValidationResponse {
                            valid: true,
                            user_id: Some(claims.sub),
                            tenant_id: Some(claims.tenant_id),
                            permissions: claims.permissions,
                            roles: claims.roles,
                        });
                    }
                }
                
                Ok(TokenValidationResponse {
                    valid: false,
                    user_id: None,
                    tenant_id: None,
                    permissions: vec![],
                    roles: vec![],
                })
            }
            Err(_) => Ok(TokenValidationResponse {
                valid: false,
                user_id: None,
                tenant_id: None,
                permissions: vec![],
                roles: vec![],
            }),
        }
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<()> {
        if let Ok(Some(mut session)) = self.session_repository.find_by_token(refresh_token).await {
            session.deactivate();
            self.session_repository.update(session).await?;
        }
        Ok(())
    }

    pub async fn change_password(&self, user_id: Uuid, request: ChangePasswordRequest) -> Result<()> {
        request.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        let user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Verify current password
        if !PasswordManager::verify_password(&request.current_password, &user.password_hash)? {
            return Err(AppError::Authentication("Invalid current password".to_string()));
        }

        // Hash new password
        let new_password_hash = PasswordManager::hash_password(&request.new_password)?;

        // Update password (this would need to be implemented in the update method)
        // For now, we'll use a simplified approach
        Ok(())
    }
}