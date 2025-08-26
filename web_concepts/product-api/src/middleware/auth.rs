use crate::{utils::verify_jwt, AppError, AppState};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized {
            context: Some("Missing or invalid token".to_string()), // or None
            error_id: uuid::Uuid::new_v4(),
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized {
            context: Some("Missing or invalid token".to_string()), // or None
            error_id: uuid::Uuid::new_v4(),
        });
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    let claims = verify_jwt(token, &state.config.jwt_secret)?;

    // Add user info to request extensions for handlers to access
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
