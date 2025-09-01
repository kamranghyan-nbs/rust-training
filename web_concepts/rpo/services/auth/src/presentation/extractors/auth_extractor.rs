use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::request::Parts,
};
use crate_errors::{AppError, Result};
use crate::presentation::middleware::auth::AuthContext;
use uuid::Uuid;

// Extractor for authenticated user context
pub struct AuthUser {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let auth_context = parts
            .extensions
            .get::<AuthContext>()
            .ok_or_else(|| AppError::Authentication("No authentication context".to_string()))?;

        Ok(AuthUser {
            user_id: auth_context.user_id,
            tenant_id: auth_context.tenant_id,
            roles: auth_context.roles.clone(),
            permissions: auth_context.permissions.clone(),
        })
    }
}

// Extractor for optional authentication
pub struct OptionalAuthUser {
    pub user_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let auth_context = parts.extensions.get::<AuthContext>();

        match auth_context {
            Some(context) => Ok(OptionalAuthUser {
                user_id: Some(context.user_id),
                tenant_id: Some(context.tenant_id),
                roles: context.roles.clone(),
                permissions: context.permissions.clone(),
            }),
            None => Ok(OptionalAuthUser {
                user_id: None,
                tenant_id: None,
                roles: vec![],
                permissions: vec![],
            }),
        }
    }
}