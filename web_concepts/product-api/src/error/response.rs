use crate::error::{types::AppError, registry::ErrorRegistry};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, warn, info, debug};
use uuid::Uuid;

/// Standardized error response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Machine-readable error code
    pub error: ErrorInfo,
    /// Request correlation ID
    pub request_id: Option<String>,
    /// Timestamp of the error
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional context (only in development)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<DebugInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Error code (e.g., "AUTH_UNAUTHORIZED")
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// User-friendly message (safe to display to end users)
    pub user_message: String,
    /// Error category for grouping
    pub category: String,
    /// Whether this error is retryable
    pub retryable: bool,
    /// Retry delay in seconds (if retryable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
    /// Link to error documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    /// Additional error-specific fields
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfo {
    /// Detailed error information (only in development)
    pub details: String,
    /// Stack trace or additional context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<String>,
    /// Internal error ID for correlation
    pub error_id: String,
}

impl ErrorResponse {
    /// Create an error response from AppError
    pub fn from_app_error(error: &AppError, request_id: Option<String>) -> Self {
        let definition = ErrorRegistry::get_definition(error);
        let error_id = error.error_id();
        
        // Determine if we should include debug info (based on environment)
        let include_debug = std::env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string()) 
            != "production";

        let mut details = HashMap::new();

        // Add error-specific details
        match error {
            AppError::ValidationError { field, message, .. } => {
                if let Some(field) = field {
                    details.insert("field".to_string(), serde_json::Value::String(field.clone()));
                }
                details.insert("validation_message".to_string(), serde_json::Value::String(message.clone()));
            }
            AppError::NotFound { resource_type, resource_id, .. } => {
                details.insert("resource_type".to_string(), serde_json::Value::String(resource_type.clone()));
                if let Some(id) = resource_id {
                    details.insert("resource_id".to_string(), serde_json::Value::String(id.clone()));
                }
            }
            AppError::RateLimitExceeded { limit_type, retry_after, .. } => {
                details.insert("limit_type".to_string(), serde_json::Value::String(limit_type.clone()));
                if let Some(retry) = retry_after {
                    details.insert("retry_after_seconds".to_string(), serde_json::Value::Number((*retry).into()));
                }
            }
            AppError::DatabaseError { operation, table, .. } => {
                details.insert("operation".to_string(), serde_json::Value::String(operation.clone()));
                if let Some(table) = table {
                    details.insert("table".to_string(), serde_json::Value::String(table.clone()));
                }
            }
            AppError::ExternalServiceError { service, operation, status_code, .. } => {
                details.insert("service".to_string(), serde_json::Value::String(service.clone()));
                details.insert("operation".to_string(), serde_json::Value::String(operation.clone()));
                if let Some(status) = status_code {
                    details.insert("external_status_code".to_string(), serde_json::Value::Number((*status).into()));
                }
            }
            AppError::BusinessRuleViolation { rule, context, .. } => {
                details.insert("rule".to_string(), serde_json::Value::String(rule.clone()));
                details.insert("context".to_string(), serde_json::Value::String(context.clone()));
            }
            _ => {} // No additional details for other error types
        }

        Self {
            error: ErrorInfo {
                code: definition.code.to_string(),
                message: definition.message.to_string(),
                user_message: definition.user_message.to_string(),
                category: format!("{:?}", definition.category).to_lowercase(),
                retryable: definition.retryable,
                retry_after: error.retry_delay(),
                documentation_url: definition.documentation_url.map(|s| s.to_string()),
                details,
            },
            request_id,
            timestamp: chrono::Utc::now(),
            debug_info: if include_debug {
                Some(DebugInfo {
                    details: format!("{:?}", error),
                    trace: None, // Could be populated with stack trace if needed
                    error_id: error_id.to_string(),
                })
            } else {
                None
            },
        }
    }

    /// Log the error with appropriate level and context
    pub fn log_error(&self, context: &str) {
        let error_code = &self.error.code;
        let error_id = self.debug_info.as_ref().map(|d| &d.error_id);
        
        // Log with different levels based on error category and severity
        match self.error.category.as_str() {
            "authentication" | "authorization" => {
                warn!(
                    error_code = %error_code,
                    error_id = ?error_id,
                    request_id = ?self.request_id,
                    context = %context,
                    retryable = %self.error.retryable,
                    " Authentication/Authorization error"
                );
            }
            "validation" | "resource" => {
                info!(
                    error_code = %error_code,
                    error_id = ?error_id,
                    request_id = ?self.request_id,
                    context = %context,
                    details = ?self.error.details,
                    " Client error"
                );
            }
            "database" | "system" | "security" => {
                error!(
                    error_code = %error_code,
                    error_id = ?error_id,
                    request_id = ?self.request_id,
                    context = %context,
                    retryable = %self.error.retryable,
                    retry_after = ?self.error.retry_after,
                    " System error"
                );
            }
            "ratelimit" => {
                warn!(
                    error_code = %error_code,
                    error_id = ?error_id,
                    request_id = ?self.request_id,
                    context = %context,
                    retry_after = ?self.error.retry_after,
                    " Rate limit exceeded"
                );
            }
            "business" => {
                warn!(
                    error_code = %error_code,
                    error_id = ?error_id,
                    request_id = ?self.request_id,
                    context = %context,
                    details = ?self.error.details,
                    " Business rule violation"
                );
            }
            _ => {
                error!(
                    error_code = %error_code,
                    error_id = ?error_id,
                    request_id = ?self.request_id,
                    context = %context,
                    " Unknown error category"
                );
            }
        }
    }
}

/// Implement IntoResponse for AppError
/// This is the main integration point with Axum
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let definition = ErrorRegistry::get_definition(&self);
        
        // Extract request ID from current span context if available
        let request_id = tracing::Span::current()
            .field("request_id")
            .and_then(|field| {
                // This is a simplified extraction - in real implementation,
                // you'd extract from span fields properly
                None as Option<String>
            });

        // Create structured error response
        let error_response = ErrorResponse::from_app_error(&self, request_id);
        
        // Log the error with appropriate context
        error_response.log_error("http_response");

        // Add custom headers for client handling
        let mut response = (definition.status_code, Json(error_response)).into_response();
        
        // Add retry-after header if applicable
        if let Some(retry_after) = self.retry_delay() {
            response.headers_mut().insert(
                "retry-after",
                retry_after.to_string().parse().unwrap(),
            );
        }

        // Add error ID header for correlation
        response.headers_mut().insert(
            "x-error-id",
            self.error_id().to_string().parse().unwrap(),
        );

        // Add cache control for error responses
        response.headers_mut().insert(
            "cache-control",
            "no-cache, no-store, must-revalidate".parse().unwrap(),
        );

        response
    }
}

/// Helper function for handlers to create error responses with context
pub fn create_error_response(error: AppError, context: &str) -> Response {
    // Add context to tracing span
    tracing::Span::current().record("error_context", context);
    
    error.into_response()
}

/// Middleware helper for extracting and adding request ID to error responses
pub fn with_request_id(error: AppError, request_id: String) -> Response {
    let error_response = ErrorResponse::from_app_error(&error, Some(request_id.clone()));
    let definition = ErrorRegistry::get_definition(&error);
    
    error_response.log_error("http_request");
    
    let mut response = (definition.status_code, Json(error_response)).into_response();
    
    // Add headers
    response.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    response.headers_mut().insert(
        "x-error-id", 
        error.error_id().to_string().parse().unwrap(),
    );

    if let Some(retry_after) = error.retry_delay() {
        response.headers_mut().insert(
            "retry-after",
            retry_after.to_string().parse().unwrap(),
        );
    }

    response
}