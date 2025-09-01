use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    
    #[error("Tenant not found: {tenant_id}")]
    TenantNotFound { tenant_id: Uuid },
    
    #[error("Permission denied: {permission}")]
    PermissionDenied { permission: String },
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
    pub request_id: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let request_id = Uuid::new_v4().to_string();
        
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                tracing::error!(error = %e, "Database error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::Validation(ref message) => (StatusCode::BAD_REQUEST, message.clone()),
            AppError::Authentication(ref message) => (StatusCode::UNAUTHORIZED, message.clone()),
            AppError::Authorization(ref message) => (StatusCode::FORBIDDEN, message.clone()),
            AppError::NotFound(ref message) => (StatusCode::NOT_FOUND, message.clone()),
            AppError::Conflict(ref message) => (StatusCode::CONFLICT, message.clone()),
            AppError::Internal(ref message) => {
                tracing::error!(error = %message, "Internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, message.clone())
            }
            AppError::BadRequest(ref message) => (StatusCode::BAD_REQUEST, message.clone()),
            AppError::Jwt(ref e) => {
                tracing::error!(error = %e, "JWT error occurred");
                (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
            }
            AppError::TenantNotFound { tenant_id } => {
                (StatusCode::NOT_FOUND, format!("Tenant not found: {}", tenant_id))
            }
            AppError::PermissionDenied { permission } => {
                (StatusCode::FORBIDDEN, format!("Permission denied: {}", permission))
            }
        };

        let body = ErrorResponse {
            error: status.canonical_reason().unwrap_or("Unknown").to_string(),
            message: error_message,
            details: None,
            request_id,
        };

        (status, Json(body)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;