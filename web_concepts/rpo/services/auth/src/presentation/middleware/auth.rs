use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use crate_errors::{AppError, Result};
use crate_security::{Claims, JwtManager};
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub claims: Claims,
    pub user_id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

pub async fn auth_middleware(
    State(jwt_manager): State<Arc<JwtManager>>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    // Extract the Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header_str| {
            if header_str.starts_with("Bearer ") {
                Some(&header_str[7..])
            } else {
                None
            }
        });

    let token = auth_header.ok_or_else(|| {
        AppError::Authentication("Missing or invalid Authorization header".to_string())
    })?;

    // Verify the token
    let claims = jwt_manager.verify_token(token)?;

    // Create auth context
    let auth_context = AuthContext {
        user_id: claims.sub,
        tenant_id: claims.tenant_id,
        roles: claims.roles.clone(),
        permissions: claims.permissions.clone(),
        claims,
    };

    // Add auth context to request extensions
    request.extensions_mut().insert(auth_context);

    Ok(next.run(request).await)
}

// Permission checking middleware
pub fn require_permission(
    permission: &'static str,
) -> impl Fn(Request, Next) -> Pin<Box<dyn Future<Output = std::result::Result<Response, AppError>> + Send>> + Clone {
    let permission = permission.to_string();

    move |mut request: Request, next: Next| {
        let permission = permission.clone();
        Box::pin(async move {
            let auth_context = request
                .extensions()
                .get::<AuthContext>()
                .ok_or_else(|| AppError::Authentication("No authentication context".to_string()))?;

            if !crate_security::PermissionChecker::has_permission(&auth_context.permissions, &permission) {
                return Err(AppError::PermissionDenied { permission });
            }

            Ok(next.run(request).await)
        })
    }
}

// Role checking middleware
pub fn require_role(
    role: &'static str,
) -> impl Fn(Request, Next) -> Pin<Box<dyn Future<Output = std::result::Result<Response, AppError>> + Send>> + Clone {
    let role = role.to_string();

    move |request: Request, next: Next| {
        let role = role.clone();
        Box::pin(async move {
            let auth_context = request
                .extensions()
                .get::<AuthContext>()
                .ok_or_else(|| AppError::Authentication("No authentication context".to_string()))?;

            if !auth_context.roles.contains(&role) {
                return Err(AppError::Authorization(format!("Role '{}' required", role)));
            }

            Ok(next.run(request).await)
        })
    }
}