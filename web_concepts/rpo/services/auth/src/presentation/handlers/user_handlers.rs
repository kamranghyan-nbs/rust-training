use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json as ResponseJson,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::presentation::extractors::AuthUser;
use crate::presentation::routes::AppState;
use crate::domain::entities::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate_errors::Result;

#[derive(Deserialize)]
pub struct ListUsersQuery {
    page: Option<u64>,
    page_size: Option<u64>,
}

/// Create a new user
#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists")
    ),
    security(
        ("bearer_auth" = ["users.create"])
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    _auth: AuthUser, // Ensures user is authenticated
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, ResponseJson<UserResponse>)> {
    let user = state.user_service.create_user(request).await?;
    Ok((StatusCode::CREATED, ResponseJson(user)))
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = ["users.read"])
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<UserResponse>> {
    let user = state.user_service.get_user(id).await?;
    Ok(ResponseJson(user))
}

/// Update user
#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 409, description = "Email or username conflict")
    ),
    security(
        ("bearer_auth" = ["users.update"])
    )
)]
pub async fn update_user(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<ResponseJson<UserResponse>> {
    let user = state.user_service.update_user(id, request).await?;
    Ok(ResponseJson(user))
}

/// Delete user
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = ["users.delete"])
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    state.user_service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// List users
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 0)"),
        ("page_size" = Option<u64>, Query, description = "Page size (default: 20)")
    ),
    responses(
        (status = 200, description = "Users list", body = Vec<UserResponse>)
    ),
    security(
        ("bearer_auth" = ["users.read"])
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<ListUsersQuery>,
) -> Result<ResponseJson<Vec<UserResponse>>> {
    let page = params.page.unwrap_or(0);
    let page_size = params.page_size.unwrap_or(20).min(100); // Max 100 items per page
    
    let users = state.user_service.list_users(auth.tenant_id, page, page_size).await?;
    Ok(ResponseJson(users))
}