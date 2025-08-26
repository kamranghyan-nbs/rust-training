use crate::{
    entities::user::{Permission, UserRole},
    error::AppError,
    models::Claims,
};
use axum::{
    extract::{Request},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use tracing::{info, warn};

// RBAC configuration for different endpoints
#[derive(Debug, Clone, Default)]
pub struct RbacConfig {
    route_permissions: HashMap<String, Vec<Permission>>,
}

impl RbacConfig {
    pub fn new() -> Self {
        let mut route_permissions = HashMap::new();

        // Product endpoints permissions
        route_permissions.insert("/products".to_string(), vec![Permission::Read]);
        route_permissions.insert("/products/create".to_string(), vec![Permission::Create]);
        route_permissions.insert("/products/update".to_string(), vec![Permission::Update]);
        route_permissions.insert("/products/delete".to_string(), vec![Permission::Delete]);

        // Search endpoints (read-only)
        route_permissions.insert("/products/search".to_string(), vec![Permission::Read]);
        route_permissions.insert("/products/category".to_string(), vec![Permission::Read]);
        route_permissions.insert("/products/price-range".to_string(), vec![Permission::Read]);
        route_permissions.insert("/products/low-stock".to_string(), vec![Permission::Read]);
        route_permissions.insert("/products/similar".to_string(), vec![Permission::Read]);
        route_permissions.insert("/products/stats".to_string(), vec![Permission::Read]);
        route_permissions.insert("/products/trending-categories".to_string(), vec![Permission::Read]);

        Self { route_permissions }
    }

    /// Get required permissions for a route and method
    pub fn get_required_permissions(&self, path: &str, method: &str) -> Vec<Permission> {
        // If a specific mapping exists in route_permissions, use it
        if let Some(perms) = self.route_permissions.get(path) {
            return perms.clone();
        }

        // Otherwise, infer from HTTP method
        match method {
            "POST" => vec![Permission::Create],
            "GET" => vec![Permission::Read],
            "PUT" => vec![Permission::Update],
            "DELETE" => vec![Permission::Delete],
            _ => vec![Permission::Read],
        }
    }
}

/// RBAC middleware that checks user permissions
// pub async fn rbac_middleware(mut request: Request, next: Next) -> Result<Response, AppError> {
//     // Extract user claims from request extensions (set by auth middleware)
//     let claims = request
//         .extensions()
//         .get::<Claims>()
//         .cloned()
//         .ok_or_else(|| {
//             error!("RBAC middleware called without authentication context");
//             AppError::unauthorized_with_context(
//                 "Authentication required for this endpoint".to_string(),
//             )
//         })?;

//     // Get request path and method
//     let path = request.uri().path();
//     let method = request.method().as_str();

//     // Create RBAC config
//     let rbac_config = RbacConfig::new();

//     // Get required permissions for this endpoint
//     let required_permissions = rbac_config.get_required_permissions(path, method);

//     // Parse user role from claims
//     let user_role = parse_user_role(&claims.username).await?;

//     // Check if user has required permissions
//     for permission in &required_permissions {
//         if !user_role.has_permission(permission) {
//             warn!(
//                 user_id = %claims.sub,
//                 username = %claims.username,
//                 role = ?user_role,
//                 required_permission = ?permission,
//                 path = %path,
//                 method = %method,
//                 "Access denied: Insufficient permissions"
//             );

//             return Err(AppError::InsufficientPrivileges {
//                 required_role: format!("{permission:?}"),
//                 current_role: Some(format!("{user_role:?}")),
//                 error_id: uuid::Uuid::new_v4(),
//             });
//         }
//     }

//     // Log successful authorization
//     info!(
//         user_id = %claims.sub,
//         username = %claims.username,
//         role = ?user_role,
//         permissions = ?required_permissions,
//         path = %path,
//         method = %method,
//         "Access granted: User has required permissions"
//     );

//     // Add user role to request extensions for handlers to use
//     request.extensions_mut().insert(user_role.clone());
//     request.extensions_mut().insert(UserContext {
//         user_id: claims.sub.clone(),
//         username: claims.username.clone(),
//         role: user_role,
//     });

//     Ok(next.run(request).await)
// }

/// User context for handlers
#[derive(Debug, Clone)]
pub struct UserContext {
    pub user_id: String,
    pub username: String,
    pub role: UserRole,
}

impl UserContext {
    pub fn can_perform(&self, permission: &Permission) -> bool {
        self.role.has_permission(permission)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }

    pub fn is_manager(&self) -> bool {
        matches!(self.role, UserRole::Manager)
    }

    pub fn is_user(&self) -> bool {
        matches!(self.role, UserRole::User)
    }
}

/// Helper function to get user role from database or cache
/// In a real implementation, you'd cache this or get it from the JWT claims
async fn parse_user_role(username: &str) -> Result<UserRole, AppError> {
    // For now, parse from username (in real app, get from database or JWT)
    // This is a simplified version - in production, you'd:
    // 1. Store role in JWT claims during login
    // 2. Or query database with caching
    // 3. Or use a user service

    match username {
        name if name.contains("admin") => Ok(UserRole::Admin),
        name if name.contains("manager") => Ok(UserRole::Manager),
        _ => Ok(UserRole::User),
    }
}

/// Middleware that requires CREATE permission
pub async fn require_create_permission(request: Request, next: Next) -> Result<Response, AppError> {
    check_permission(request, next, Permission::Create).await
}

/// Middleware that requires UPDATE permission  
pub async fn require_update_permission(request: Request, next: Next) -> Result<Response, AppError> {
    check_permission(request, next, Permission::Update).await
}

/// Middleware that requires DELETE permission
pub async fn require_delete_permission(request: Request, next: Next) -> Result<Response, AppError> {
    check_permission(request, next, Permission::Delete).await
}

/// Middleware that requires READ permission (least restrictive)
pub async fn require_read_permission(request: Request, next: Next) -> Result<Response, AppError> {
    check_permission(request, next, Permission::Read).await
}

/// Helper function to check specific permission
async fn check_permission(
    mut request: Request,
    next: Next,
    required_permission: Permission,
) -> Result<Response, AppError> {
    // Extract user claims
    let claims = request
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(AppError::unauthorized)?;

    // Get user role
    let user_role = parse_user_role(&claims.username).await?;

    // Check permission
    if !user_role.has_permission(&required_permission) {
        warn!(
            user_id = %claims.sub,
            username = %claims.username,
            role = ?user_role,
            required_permission = ?required_permission,
            "Access denied: Missing required permission"
        );

        return Err(AppError::InsufficientPrivileges {
            required_role: format!("{required_permission:?}"),
            current_role: Some(format!("{user_role:?}")),
            error_id: uuid::Uuid::new_v4(),
        });
    }

    // Add context to request
    request.extensions_mut().insert(UserContext {
        user_id: claims.sub.clone(),
        username: claims.username.clone(),
        role: user_role,
    });

    info!(
        user_id = %claims.sub,
        username = %claims.username,
        permission = ?required_permission,
        "Permission check passed"
    );

    Ok(next.run(request).await)
}

// Admin-only middleware (convenience function)
// pub async fn require_admin(request: Request, next: Next) -> Result<Response, AppError> {
//     let claims = request
//         .extensions()
//         .get::<Claims>()
//         .cloned()
//         .ok_or_else(AppError::unauthorized)?;

//     let user_role = parse_user_role(&claims.username).await?;

//     if !matches!(user_role, UserRole::Admin) {
//         return Err(AppError::InsufficientPrivileges {
//             required_role: "Admin".to_string(),
//             current_role: Some(format!("{user_role:?}")),
//             error_id: uuid::Uuid::new_v4(),
//         });
//     }

//     Ok(next.run(request).await)
// }

// Manager or Admin middleware (convenience function)
// pub async fn require_manager_or_admin(request: Request, next: Next) -> Result<Response, AppError> {
//     let claims = request
//         .extensions()
//         .get::<Claims>()
//         .cloned()
//         .ok_or_else(AppError::unauthorized)?;

//     let user_role = parse_user_role(&claims.username).await?;

//     if !matches!(user_role, UserRole::Admin | UserRole::Manager) {
//         return Err(AppError::InsufficientPrivileges {
//             required_role: "Manager or Admin".to_string(),
//             current_role: Some(format!("{user_role:?}")),
//             error_id: uuid::Uuid::new_v4(),
//         });
//     }

//     Ok(next.run(request).await)
// }
