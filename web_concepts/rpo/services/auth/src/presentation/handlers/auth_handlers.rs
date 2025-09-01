use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use crate::presentation::routes::AppState;
use crate::domain::value_objects::*;
use crate_errors::Result;

/// Login user
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 400, description = "Validation error")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<ResponseJson<LoginResponse>> {
    let response = state.auth_service.login(request, None, None).await?;
    Ok(ResponseJson(response))
}

/// Refresh access token
#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = LoginResponse),
        (status = 401, description = "Invalid refresh token")
    )
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<ResponseJson<LoginResponse>> {
    let response = state.auth_service.refresh_token(request).await?;
    Ok(ResponseJson(response))
}

/// Validate token
#[utoipa::path(
    post,
    path = "/auth/validate",
    tag = "auth",
    responses(
        (status = 200, description = "Token validation result", body = TokenValidationResponse)
    )
)]
pub async fn validate_token(
    State(state): State<AppState>,
    Json(token): Json<String>,
) -> Result<ResponseJson<TokenValidationResponse>> {
    let response = state.auth_service.validate_token(&token).await?;
    Ok(ResponseJson(response))
}

/// Logout user
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Invalid refresh token")
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<StatusCode> {
    state.auth_service.logout(&request.refresh_token).await?;
    Ok(StatusCode::OK)
}

/// Change password
#[utoipa::path(
    put,
    path = "/auth/change-password",
    tag = "auth",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 401, description = "Invalid current password")
    )
)]
pub async fn change_password(
    State(state): State<AppState>,
    Json(request): Json<ChangePasswordRequest>,
) -> Result<StatusCode> {
    // Note: In a real implementation, you'd extract the user ID from the JWT token
    // using a custom extractor middleware
    let user_id = uuid::Uuid::new_v4(); // Placeholder
    state.auth_service.change_password(user_id, request).await?;
    Ok(StatusCode::OK)
}