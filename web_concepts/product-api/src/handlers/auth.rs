use crate::{
    error::{AppError},
    middleware::logging::{log_application_error, PerformanceTimer},
    models::{LoginRequest, RegisterRequest},
    repository::auth::AuthRepository,
    services::AuthService,
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::{error, info, instrument, warn};
use validator::Validate;

#[instrument(
    name = "auth_login",
    skip(state, request),
    fields(
        username = %request.username,
        request_size = %std::mem::size_of_val(&request)
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Start performance timing
    let timer = PerformanceTimer::new("auth_login_handler".to_string());

    // Log the login attempt
    info!(
        username = %request.username,
        "User login attempt initiated"
    );

    // Validate request
    if let Err(validation_error) = request.validate() {
        warn!(
            username = %request.username,
            validation_error = %validation_error,
            "Login request validation failed"
        );
        return Err(AppError::from(validation_error));
    }

    // Create repository and service with structured context
    let auth_repository = {
        let _span = tracing::debug_span!("create_auth_repository").entered();
        Arc::new(AuthRepository::new(state.db.clone()))
    };

    let auth_service = {
        let _span = tracing::debug_span!("create_auth_service").entered();
        AuthService::new(auth_repository)
    };

    // Perform login with error handling and logging
    match auth_service.login(state.config, request).await {
        Ok(response) => {
            info!(
                username = %response.user.username,
                user_id = %response.user.id,
                "User login successful"
            );

            timer.finish();
            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => {
            // Log the error with context
            log_application_error(&error, "auth_login_handler");

            // Add additional context for auth failures
            match &error {
                AppError::Unauthorized { context, .. } => {
                    warn!(
                        unauthorized_context = ?context,
                        "Login failed: Invalid credentials provided"
                    );
                }
                _ => {
                    error!(
                        error = %error,
                        "Login failed: Unexpected error"
                    );
                }
            }

            timer.finish();
            Err(error)
        }
    }
}

#[instrument(
    name = "auth_register",
    skip(state, request),
    fields(
        username = %request.username,
        email = %request.email,
        request_size = %std::mem::size_of_val(&request)
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Start performance timing
    let timer = PerformanceTimer::new("auth_register_handler".to_string());

    // Log the registration attempt
    info!(
        username = %request.username,
        email = %request.email,
        "User registration attempt initiated"
    );

    // Validate request with detailed logging
    if let Err(validation_error) = request.validate() {
        warn!(
            username = %request.username,
            email = %request.email,
            validation_error = %validation_error,
            "Registration request validation failed"
        );
        return Err(AppError::from(validation_error));
    }

    // Create repository and service
    let auth_repository = {
        let _span = tracing::debug_span!("create_auth_repository").entered();
        Arc::new(AuthRepository::new(state.db.clone()))
    };

    let auth_service = {
        let _span = tracing::debug_span!("create_auth_service").entered();
        AuthService::new(auth_repository)
    };

    // Perform registration with comprehensive error handling
    match auth_service.register(state.config, request, None).await {
        Ok(response) => {
            info!(
                username = %response.user.username,
                email = %response.user.email,
                user_id = %response.user.id,
                "User registration successful"
            );

            timer.finish();
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(error) => {
            // Log the error with context
            log_application_error(&error, "auth_register_handler");

            // Add specific context for registration failures
            match &error {
                AppError::Conflict {
                    resource_type,
                    message,
                    error_id,
                } => {
                    warn!(
                        conflict_reason = %message,
                        resource = %resource_type,
                        error_id = %error_id,
                        "Registration failed: User already exists"
                    );
                }
                AppError::ValidationError { field, message, .. } => {
                    warn!(
                        validation_field = ?field,
                        validation_reason = ?message,
                        "Registration failed: Validation error"
                    );
                }
                _ => {
                    error!(
                        error = %error,
                        "Registration failed: Unexpected error"
                    );
                }
            }

            timer.finish();
            Err(error)
        }
    }
}
