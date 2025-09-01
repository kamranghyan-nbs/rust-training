use crate::config::AuthConfig;
use crate::infrastructure::database::Database;
use crate::infrastructure::repositories::*;
use crate::application::services::{AuthService, UserService, TenantService};
use crate_security::JwtManager;
use crate::presentation::handlers;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth_handlers::login,
        handlers::auth_handlers::refresh_token,
        handlers::auth_handlers::validate_token,
        handlers::auth_handlers::logout,
        handlers::user_handlers::create_user,
        handlers::user_handlers::get_user,
        handlers::user_handlers::update_user,
        handlers::user_handlers::list_users,
        handlers::tenant_handlers::create_tenant,
        handlers::tenant_handlers::get_tenant,
        handlers::tenant_handlers::list_tenants,
    ),
    components(
        schemas(
            crate::domain::value_objects::LoginRequest,
            crate::domain::value_objects::LoginResponse,
            crate::domain::value_objects::RefreshTokenRequest,
            crate::domain::value_objects::TokenValidationResponse,
            crate::domain::entities::CreateUserRequest,
            crate::domain::entities::UserResponse,
            crate::domain::entities::CreateTenantRequest,
            crate::domain::entities::TenantResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "tenants", description = "Tenant management endpoints"),
    )
)]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
    pub tenant_service: Arc<TenantService>,
    pub jwt_manager: Arc<JwtManager>,
}

pub fn create_router(config: AuthConfig, database: Database) -> Router {
    // Initialize repositories
    let user_repo = Arc::new(UserRepositoryImpl::new(database.clone()));
    let tenant_repo = Arc::new(TenantRepositoryImpl::new(database.clone()));
    let session_repo = Arc::new(SessionRepositoryImpl::new(database.clone()));
    let role_repo = Arc::new(RoleRepositoryImpl::new(database.clone()));
    let permission_repo = Arc::new(PermissionRepositoryImpl::new(database.clone()));

    // Initialize JWT manager
    let jwt_manager = JwtManager::new(
        &config.jwt.secret,
        config.jwt.issuer.clone(),
        config.jwt.access_token_expire_minutes,
        config.jwt.refresh_token_expire_days,
    );

    // Initialize services
    let auth_service = Arc::new(AuthService::new(
        user_repo.clone(),
        tenant_repo.clone(),
        session_repo.clone(),
        jwt_manager.clone(),
    ));

    let user_service = Arc::new(UserService::new(
        user_repo.clone(),
        role_repo.clone(),
    ));

    let tenant_service = Arc::new(TenantService::new(tenant_repo.clone()));

    let app_state = AppState {
        auth_service,
        user_service,
        tenant_service,
        jwt_manager: Arc::new(jwt_manager),
    };

    Router::new()
        // Auth routes
        .route("/auth/login", post(handlers::auth_handlers::login))
        .route("/auth/refresh", post(handlers::auth_handlers::refresh_token))
        .route("/auth/validate", post(handlers::auth_handlers::validate_token))
        .route("/auth/logout", post(handlers::auth_handlers::logout))
        .route("/auth/change-password", put(handlers::auth_handlers::change_password))
        
        // User routes
        .route("/users", post(handlers::user_handlers::create_user))
        .route("/users", get(handlers::user_handlers::list_users))
        .route("/users/:id", get(handlers::user_handlers::get_user))
        .route("/users/:id", put(handlers::user_handlers::update_user))
        .route("/users/:id", delete(handlers::user_handlers::delete_user))
        
        // Tenant routes
        .route("/tenants", post(handlers::tenant_handlers::create_tenant))
        .route("/tenants", get(handlers::tenant_handlers::list_tenants))
        .route("/tenants/:id", get(handlers::tenant_handlers::get_tenant))
        .route("/tenants/:id", put(handlers::tenant_handlers::update_tenant))
        
        // Internal service communication routes (no auth required)
        .route("/internal/validate-token", post(handlers::internal_handlers::validate_token_internal))
        .route("/internal/get-user", post(handlers::internal_handlers::get_user_internal))
        
        // Health check
        .route("/health", get(|| async { "OK" }))
        
        // Documentation
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        
        .with_state(app_state)
}