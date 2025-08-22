use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, warn, error, Span};
use uuid::Uuid;

// Async request logging middleware with structured data
pub async fn request_logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();
    
    // Generate unique request ID for correlation
    let request_id = Uuid::new_v4();
    
    // Extract matched path for cleaner logging (removes query params)
    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str())
        .unwrap_or(uri.path());

    // Extract user agent
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    // Extract IP address (considering proxies)
    let client_ip = extract_client_ip(&request);

    // Create a span for this request with structured fields
    let request_span = tracing::info_span!(
        "http_request",
        method = %method,
        path = %matched_path,
        version = ?version,
        request_id = %request_id,
        client_ip = %client_ip,
        user_agent = %user_agent,
    );

    // Enter the span context
    let _enter = request_span.enter();

    // Log the incoming request
    info!(
        "📥 HTTP request received"
    );

    // Process the request
    let response = next.run(request).await;
    
    // Calculate duration
    let duration = start.elapsed();
    let status = response.status();
    
    // Extract response size if available
    let response_size = response
        .headers()
        .get("content-length")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    // Log response with different levels based on status
    match status.as_u16() {
        200..=299 => {
            info!(
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                response_size_bytes = %response_size,
                "📤 HTTP request completed successfully"
            );
        }
        300..=399 => {
            info!(
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                "🔄 HTTP request redirected"
            );
        }
        400..=499 => {
            warn!(
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                "⚠️ HTTP request completed with client error"
            );
        }
        500..=599 => {
            error!(
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                "❌ HTTP request completed with server error"
            );
        }
        _ => {
            warn!(
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                "❓ HTTP request completed with unusual status"
            );
        }
    }

    // Add request ID to response headers for correlation
    let mut response = response;
    response.headers_mut().insert(
        "x-request-id",
        request_id.to_string().parse().unwrap(),
    );

    response
}

// Extract client IP considering various proxy headers
fn extract_client_ip(request: &Request) -> String {
    // Try X-Forwarded-For header first (most common)
    if let Some(forwarded_for) = request.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(ip) = forwarded_str.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }

    // Try X-Real-IP header
    if let Some(real_ip) = request.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.to_string();
        }
    }

    // Try X-Forwarded header
    if let Some(forwarded) = request.headers().get("x-forwarded") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(ip_part) = forwarded_str.split("for=").nth(1) {
                if let Some(ip) = ip_part.split(';').next() {
                    return ip.trim().to_string();
                }
            }
        }
    }

    // Fallback to connection info (would need to be passed from connection)
    "unknown".to_string()
}

// Database operation logging utilities
pub fn log_database_operation<'a, T>(
    operation: &'a str,
    table: &'a str,
    span: &'a Span,
) -> impl Fn(Result<T, sea_orm::DbErr>) -> Result<T, sea_orm::DbErr> + 'a
where
    T: std::fmt::Debug,
{
    move |result| {
        match &result {
            Ok(_) => {
                span.in_scope(|| {
                    tracing::debug!(
                        operation = %operation,
                        table = %table,
                        "✅ Database operation successful"
                    );
                });
            }
            Err(e) => {
                span.in_scope(|| {
                    tracing::error!(
                        operation = %operation,
                        table = %table,
                        error = %e,
                        "❌ Database operation failed"
                    );
                });
            }
        }
        result
    }
}

// Async task logging utilities
pub fn create_task_span(task_name: &str, task_id: Option<&str>) -> tracing::Span {
    match task_id {
        Some(id) => tracing::info_span!("async_task", task_name = %task_name, task_id = %id),
        None => tracing::info_span!("async_task", task_name = %task_name),
    }
}

// Performance measurement utilities
pub struct PerformanceTimer {
    start: Instant,
    operation: String,
}

impl PerformanceTimer {
    pub fn new(operation: String) -> Self {
        tracing::debug!(operation = %operation, "⏱️ Starting performance measurement");
        Self {
            start: Instant::now(),
            operation,
        }
    }

    pub fn finish(self) {
        let duration = self.start.elapsed();
        tracing::info!(
            operation = %self.operation,
            duration_ms = %duration.as_millis(),
            duration_micros = %duration.as_micros(),
            "⏱️ Performance measurement completed"
        );
    }
}

// Structured error logging
pub fn log_application_error(error: &crate::error::AppError, context: &str) {
    match error {
        crate::error::AppError::DatabaseError(db_err) => {
            error!(
                error_type = "database_error",
                error = %db_err,
                context = %context,
                "💾 Database error occurred"
            );
        }
        crate::error::AppError::ValidationError(validation_err) => {
            warn!(
                error_type = "validation_error",
                error = %validation_err,
                context = %context,
                "📋 Validation error occurred"
            );
        }
        crate::error::AppError::Unauthorized => {
            warn!(
                error_type = "unauthorized",
                context = %context,
                "🔒 Unauthorized access attempt"
            );
        }
        crate::error::AppError::NotFound => {
            info!(
                error_type = "not_found",
                context = %context,
                "🔍 Resource not found"
            );
        }
        _ => {
            error!(
                error_type = "application_error",
                error = %error,
                context = %context,
                "⚠️ Application error occurred"
            );
        }
    }
}