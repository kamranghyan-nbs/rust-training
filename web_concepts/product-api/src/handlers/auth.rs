use crate::{
    error::AppError,
    models::{LoginRequest, RegisterRequest},
    services::AuthService,
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use validator::Validate;

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let response = AuthService::login(&state.db, state.config, request).await?;

    Ok((StatusCode::OK, Json(response)))
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let response = AuthService::register(&state.db, state.config, request).await?;

    Ok((StatusCode::CREATED, Json(response)))
}