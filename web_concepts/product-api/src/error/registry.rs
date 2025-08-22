use crate::error::types::{AppError, ErrorCategory, ErrorSeverity};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error code registry - centralized error definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDefinition {
    pub code: &'static str,
    pub message: &'static str,
    pub user_message: &'static str, // User-friendly message
    pub status_code: StatusCode,
    pub category: ErrorCategory,
    pub severity: ErrorSeverity,
    pub retryable: bool,
    pub documentation_url: Option<&'static str>,
}

/// Central registry of all application errors
///  ADD NEW ERROR TYPES HERE - One place for all error definitions!
pub struct ErrorRegistry;

impl ErrorRegistry {
    /// Get error definition for any AppError
    pub fn get_definition(error: &AppError) -> ErrorDefinition {
        match error {
            // Authentication & Authorization Errors
            AppError::Unauthorized { .. } => ErrorDefinition {
                code: "AUTH_UNAUTHORIZED",
                message: "Authentication required",
                user_message: "Please log in to access this resource",
                status_code: StatusCode::UNAUTHORIZED,
                category: ErrorCategory::Authentication,
                severity: ErrorSeverity::Medium,
                retryable: false,
                documentation_url: Some("/docs/errors#auth-unauthorized"),
            },

            AppError::Forbidden { resource, .. } => ErrorDefinition {
                code: "AUTH_FORBIDDEN",
                message: "Access to resource is forbidden",
                user_message: "You don't have permission to access this resource",
                status_code: StatusCode::FORBIDDEN,
                category: ErrorCategory::Authorization,
                severity: ErrorSeverity::Medium,
                retryable: false,
                documentation_url: Some("/docs/errors#auth-forbidden"),
            },

            AppError::InvalidToken { .. } => ErrorDefinition {
                code: "AUTH_INVALID_TOKEN",
                message: "Invalid or expired authentication token",
                user_message: "Your session has expired. Please log in again",
                status_code: StatusCode::UNAUTHORIZED,
                category: ErrorCategory::Authentication,
                severity: ErrorSeverity::Medium,
                retryable: false,
                documentation_url: Some("/docs/errors#auth-invalid-token"),
            },

            // Validation Errors
            AppError::ValidationError { .. } => ErrorDefinition {
                code: "VALIDATION_FAILED",
                message: "Request validation failed",
                user_message: "The information provided is invalid. Please check your input",
                status_code: StatusCode::BAD_REQUEST,
                category: ErrorCategory::Validation,
                severity: ErrorSeverity::Low,
                retryable: false,
                documentation_url: Some("/docs/errors#validation-failed"),
            },

            AppError::BadRequest { .. } => ErrorDefinition {
                code: "BAD_REQUEST",
                message: "Invalid request format",
                user_message: "The request format is invalid. Please check the API documentation",
                status_code: StatusCode::BAD_REQUEST,
                category: ErrorCategory::Validation,
                severity: ErrorSeverity::Low,
                retryable: false,
                documentation_url: Some("/docs/errors#bad-request"),
            },

            // Resource Errors
            AppError::NotFound { resource_type, .. } => ErrorDefinition {
                code: "RESOURCE_NOT_FOUND",
                message: "Requested resource not found",
                user_message: "The requested item could not be found",
                status_code: StatusCode::NOT_FOUND,
                category: ErrorCategory::Resource,
                severity: ErrorSeverity::Low,
                retryable: false,
                documentation_url: Some("/docs/errors#resource-not-found"),
            },

            AppError::Conflict { resource_type, .. } => ErrorDefinition {
                code: "RESOURCE_CONFLICT",
                message: "Resource already exists",
                user_message: "This item already exists. Please use a different name or identifier",
                status_code: StatusCode::CONFLICT,
                category: ErrorCategory::Resource,
                severity: ErrorSeverity::Medium,
                retryable: false,
                documentation_url: Some("/docs/errors#resource-conflict"),
            },

            // Database Errors  
            AppError::DatabaseError { operation, .. } => ErrorDefinition {
                code: "DATABASE_ERROR",
                message: "Database operation failed",
                user_message: "A temporary issue occurred. Please try again later",
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                category: ErrorCategory::Database,
                severity: ErrorSeverity::High,
                retryable: true,
                documentation_url: Some("/docs/errors#database-error"),
            },

            AppError::DatabaseConnectionError { .. } => ErrorDefinition {
                code: "DATABASE_CONNECTION_ERROR",
                message: "Unable to connect to database",
                user_message: "Service temporarily unavailable. Please try again later",
                status_code: StatusCode::SERVICE_UNAVAILABLE,
                category: ErrorCategory::Database,
                severity: ErrorSeverity::Critical,
                retryable: true,
                documentation_url: Some("/docs/errors#database-connection-error"),
            },

            // External Service Errors
            AppError::ExternalServiceError { service, .. } => ErrorDefinition {
                code: "EXTERNAL_SERVICE_ERROR",
                message: "External service error",
                user_message: "A dependent service is currently unavailable. Please try again later",
                status_code: StatusCode::BAD_GATEWAY,
                category: ErrorCategory::ExternalService,
                severity: ErrorSeverity::High,
                retryable: true,
                documentation_url: Some("/docs/errors#external-service-error"),
            },

            // Rate Limiting
            AppError::RateLimitExceeded { limit_type, .. } => ErrorDefinition {
                code: "RATE_LIMIT_EXCEEDED",
                message: "Rate limit exceeded",
                user_message: "Too many requests. Please slow down and try again later",
                status_code: StatusCode::TOO_MANY_REQUESTS,
                category: ErrorCategory::RateLimit,
                severity: ErrorSeverity::Medium,
                retryable: true,
                documentation_url: Some("/docs/errors#rate-limit-exceeded"),
            },

            // Business Logic Errors
            AppError::BusinessRuleViolation { rule, .. } => ErrorDefinition {
                code: "BUSINESS_RULE_VIOLATION",
                message: "Business rule violation",
                user_message: "This action violates business rules",
                status_code: StatusCode::UNPROCESSABLE_ENTITY,
                category: ErrorCategory::Business,
                severity: ErrorSeverity::Medium,
                retryable: false,
                documentation_url: Some("/docs/errors#business-rule-violation"),
            },

            AppError::InsufficientPrivileges { required_role, .. } => ErrorDefinition {
                code: "INSUFFICIENT_PRIVILEGES",
                message: "Insufficient privileges",
                user_message: "You don't have the required permissions for this action",
                status_code: StatusCode::FORBIDDEN,
                category: ErrorCategory::Authorization,
                severity: ErrorSeverity::Medium,
                retryable: false,
                documentation_url: Some("/docs/errors#insufficient-privileges"),
            },

            // System Errors
            AppError::InternalServerError { .. } => ErrorDefinition {
                code: "INTERNAL_SERVER_ERROR",
                message: "Internal server error",
                user_message: "An unexpected error occurred. Please try again later",
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                category: ErrorCategory::System,
                severity: ErrorSeverity::Critical,
                retryable: true,
                documentation_url: Some("/docs/errors#internal-server-error"),
            },

            AppError::ServiceUnavailable { service, .. } => ErrorDefinition {
                code: "SERVICE_UNAVAILABLE",
                message: "Service temporarily unavailable",
                user_message: "Service is temporarily unavailable. Please try again later",
                status_code: StatusCode::SERVICE_UNAVAILABLE,
                category: ErrorCategory::System,
                severity: ErrorSeverity::High,
                retryable: true,
                documentation_url: Some("/docs/errors#service-unavailable"),
            },

            AppError::ConfigurationError { parameter, .. } => ErrorDefinition {
                code: "CONFIGURATION_ERROR",
                message: "Configuration error",
                user_message: "Service configuration issue. Please contact support",
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                category: ErrorCategory::System,
                severity: ErrorSeverity::Critical,
                retryable: false,
                documentation_url: Some("/docs/errors#configuration-error"),
            },

            // I/O and Parsing Errors
            AppError::IoError { operation, .. } => ErrorDefinition {
                code: "IO_ERROR",
                message: "Input/Output operation failed",
                user_message: "File operation failed. Please try again",
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                category: ErrorCategory::System,
                severity: ErrorSeverity::High,
                retryable: true,
                documentation_url: Some("/docs/errors#io-error"),
            },

            AppError::ParseError { data_type, .. } => ErrorDefinition {
                code: "PARSE_ERROR",
                message: "Data parsing failed",
                user_message: "Invalid data format. Please check your input",
                status_code: StatusCode::BAD_REQUEST,
                category: ErrorCategory::Validation,
                severity: ErrorSeverity::Low,
                retryable: false,
                documentation_url: Some("/docs/errors#parse-error"),
            },

            // Crypto and Security Errors
            AppError::CryptoError { operation, .. } => ErrorDefinition {
                code: "CRYPTO_ERROR",
                message: "Cryptographic operation failed",
                user_message: "Security operation failed. Please try again",
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                category: ErrorCategory::Security,
                severity: ErrorSeverity::Critical,
                retryable: false,
                documentation_url: Some("/docs/errors#crypto-error"),
            },
        }
    }

    /// Get all error codes and their definitions (useful for documentation generation)
    pub fn get_all_definitions() -> HashMap<&'static str, ErrorDefinition> {
        // This would be populated with all possible errors
        // For now, returning empty map - in real implementation, you'd populate this
        HashMap::new()
    }

    /// Check if an error code is valid
    pub fn is_valid_error_code(code: &str) -> bool {
        // Implementation would check against all registered error codes
        // For now, basic validation
        code.starts_with("AUTH_") 
            || code.starts_with("VALIDATION_")
            || code.starts_with("RESOURCE_")
            || code.starts_with("DATABASE_")
            || code.starts_with("EXTERNAL_")
            || code.starts_with("RATE_LIMIT_")
            || code.starts_with("BUSINESS_")
            || code.starts_with("SYSTEM_")
            || code.starts_with("IO_")
            || code.starts_with("CRYPTO_")
    }

    /// Get user-friendly error message by error code
    pub fn get_user_message(code: &str) -> Option<&'static str> {
        match code {
            "AUTH_UNAUTHORIZED" => Some("Please log in to access this resource"),
            "AUTH_FORBIDDEN" => Some("You don't have permission to access this resource"),
            "VALIDATION_FAILED" => Some("The information provided is invalid. Please check your input"),
            "RESOURCE_NOT_FOUND" => Some("The requested item could not be found"),
            "RATE_LIMIT_EXCEEDED" => Some("Too many requests. Please slow down and try again later"),
            _ => None,
        }
    }
}

/// Helper function to create custom business rule violations
///  USE THIS TO ADD NEW BUSINESS RULE ERRORS EASILY
pub fn business_rule_error(rule_name: &str, context: &str) -> AppError {
    AppError::business_rule_violation(rule_name.to_string(), context.to_string())
}

/// Helper function to create custom validation errors
///  USE THIS TO ADD NEW VALIDATION ERRORS EASILY  
pub fn validation_error(field_name: &str, message: &str) -> AppError {
    AppError::validation_error(Some(field_name.to_string()), message.to_string())
}

/// Helper function to create not found errors
///  USE THIS TO ADD NEW NOT FOUND ERRORS EASILY
pub fn not_found_error(resource_type: &str, resource_id: Option<&str>) -> AppError {
    AppError::not_found(resource_type.to_string(), resource_id.map(|s| s.to_string()))
}

/// Helper function to create conflict errors
///  USE THIS TO ADD NEW CONFLICT ERRORS EASILY
pub fn conflict_error(resource_type: &str, message: &str) -> AppError {
    AppError::conflict(resource_type.to_string(), message.to_string())
}