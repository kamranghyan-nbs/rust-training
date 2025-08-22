use thiserror::Error;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Main application error type with rich context
#[derive(Error, Debug, Clone)]
pub enum AppError {
    // Authentication & Authorization Errors
    #[error("Authentication failed")]
    Unauthorized { 
        context: Option<String>,
        error_id: Uuid,
    },
    
    #[error("Access forbidden")]
    Forbidden { 
        resource: String,
        context: Option<String>,
        error_id: Uuid,
    },
    
    #[error("Invalid JWT token")]
    InvalidToken { 
        reason: String,
        error_id: Uuid,
    },

    // Validation Errors
    #[error("Request validation failed")]
    ValidationError { 
        field: Option<String>,
        message: String,
        error_id: Uuid,
    },
    
    #[error("Invalid request format")]
    BadRequest { 
        message: String,
        error_id: Uuid,
    },

    // Resource Errors
    #[error("Resource not found")]
    NotFound { 
        resource_type: String,
        resource_id: Option<String>,
        error_id: Uuid,
    },
    
    #[error("Resource conflict")]
    Conflict { 
        resource_type: String,
        message: String,
        error_id: Uuid,
    },

    // Database Errors
    #[error("Database operation failed")]
    DatabaseError { 
        operation: String,
        table: Option<String>,
        // source: String,
        error_id: Uuid,
    },
    
    #[error("Database connection failed")]
    DatabaseConnectionError { 
        database_url: String,
        error_id: Uuid,
    },

    // External Service Errors
    #[error("External service error")]
    ExternalServiceError { 
        service: String,
        operation: String,
        status_code: Option<u16>,
        error_id: Uuid,
    },

    // Rate Limiting
    #[error("Rate limit exceeded")]
    RateLimitExceeded { 
        limit_type: String,
        retry_after: Option<u64>,
        error_id: Uuid,
    },

    // Business Logic Errors
    #[error("Business rule violation")]
    BusinessRuleViolation { 
        rule: String,
        context: String,
        error_id: Uuid,
    },
    
    #[error("Insufficient privileges")]
    InsufficientPrivileges { 
        required_role: String,
        current_role: Option<String>,
        error_id: Uuid,
    },

    // System Errors
    #[error("Internal server error")]
    InternalServerError { 
        context: Option<String>,
        error_id: Uuid,
    },
    
    #[error("Service unavailable")]
    ServiceUnavailable { 
        service: String,
        retry_after: Option<u64>,
        error_id: Uuid,
    },
    
    #[error("Configuration error")]
    ConfigurationError { 
        parameter: String,
        message: String,
        error_id: Uuid,
    },

    // I/O and Parsing Errors
    #[error("File operation failed")]
    IoError { 
        operation: String,
        path: Option<String>,
        source: String,
        error_id: Uuid,
    },
    
    #[error("Data parsing failed")]
    ParseError { 
        data_type: String,
        message: String,
        error_id: Uuid,
    },

    // Crypto and Security Errors
    #[error("Cryptographic operation failed")]
    CryptoError { 
        operation: String,
        error_id: Uuid,
    },
}

/// Error severity levels for logging and alerting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Low,      // Expected errors (validation, not found)
    Medium,   // Recoverable errors (rate limits, conflicts)
    High,     // System errors (database, external services)
    Critical, // Security issues, data corruption
}

/// Error category for grouping and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCategory {
    Authentication,
    Authorization, 
    Validation,
    Resource,
    Database,
    ExternalService,
    RateLimit,
    Business,
    System,
    Security,
}

impl AppError {
    /// Create a new error with auto-generated ID
    pub fn unauthorized() -> Self {
        Self::Unauthorized {
            context: None,
            error_id: Uuid::new_v4(),
        }
    }

    pub fn unauthorized_with_context(context: String) -> Self {
        Self::Unauthorized {
            context: Some(context),
            error_id: Uuid::new_v4(),
        }
    }

    pub fn forbidden(resource: String) -> Self {
        Self::Forbidden {
            resource,
            context: None,
            error_id: Uuid::new_v4(),
        }
    }

    pub fn validation_error(field: Option<String>, message: String) -> Self {
        Self::ValidationError {
            field,
            message,
            error_id: Uuid::new_v4(),
        }
    }

    pub fn not_found(resource_type: String, resource_id: Option<String>) -> Self {
        Self::NotFound {
            resource_type,
            resource_id,
            error_id: Uuid::new_v4(),
        }
    }

    pub fn conflict(resource_type: String, message: String) -> Self {
        Self::Conflict {
            resource_type,
            message,
            error_id: Uuid::new_v4(),
        }
    }

    pub fn database_error(operation: String, table: Option<String>, source: String) -> Self {
        Self::DatabaseError {
            operation,
            table,
            // source,
            error_id: Uuid::new_v4(),
        }
    }

    pub fn business_rule_violation(rule: String, context: String) -> Self {
        Self::BusinessRuleViolation {
            rule,
            context,
            error_id: Uuid::new_v4(),
        }
    }

    pub fn internal_server_error() -> Self {
        Self::InternalServerError {
            context: None,
            error_id: Uuid::new_v4(),
        }
    }

    pub fn rate_limit_exceeded(limit_type: String, retry_after: Option<u64>) -> Self {
        Self::RateLimitExceeded {
            limit_type,
            retry_after,
            error_id: Uuid::new_v4(),
        }
    }

    /// Get the error ID for correlation
    pub fn error_id(&self) -> Uuid {
        match self {
            Self::Unauthorized { error_id, .. } => *error_id,
            Self::Forbidden { error_id, .. } => *error_id,
            Self::InvalidToken { error_id, .. } => *error_id,
            Self::ValidationError { error_id, .. } => *error_id,
            Self::BadRequest { error_id, .. } => *error_id,
            Self::NotFound { error_id, .. } => *error_id,
            Self::Conflict { error_id, .. } => *error_id,
            Self::DatabaseError { error_id, .. } => *error_id,
            Self::DatabaseConnectionError { error_id, .. } => *error_id,
            Self::ExternalServiceError { error_id, .. } => *error_id,
            Self::RateLimitExceeded { error_id, .. } => *error_id,
            Self::BusinessRuleViolation { error_id, .. } => *error_id,
            Self::InsufficientPrivileges { error_id, .. } => *error_id,
            Self::InternalServerError { error_id, .. } => *error_id,
            Self::ServiceUnavailable { error_id, .. } => *error_id,
            Self::ConfigurationError { error_id, .. } => *error_id,
            Self::IoError { error_id, .. } => *error_id,
            Self::ParseError { error_id, .. } => *error_id,
            Self::CryptoError { error_id, .. } => *error_id,
        }
    }

    /// Get error severity for logging and alerting
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::ValidationError { .. } | Self::BadRequest { .. } | Self::NotFound { .. } => {
                ErrorSeverity::Low
            }
            Self::RateLimitExceeded { .. } | Self::Conflict { .. } | Self::BusinessRuleViolation { .. } => {
                ErrorSeverity::Medium
            }
            Self::DatabaseError { .. } | Self::ExternalServiceError { .. } | Self::ServiceUnavailable { .. } => {
                ErrorSeverity::High
            }
            Self::Unauthorized { .. } | Self::Forbidden { .. } | Self::InvalidToken { .. } 
            | Self::InsufficientPrivileges { .. } | Self::CryptoError { .. } => {
                ErrorSeverity::Critical
            }
            _ => ErrorSeverity::High,
        }
    }

    /// Get error category for metrics and grouping
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::Unauthorized { .. } | Self::InvalidToken { .. } => ErrorCategory::Authentication,
            Self::Forbidden { .. } | Self::InsufficientPrivileges { .. } => ErrorCategory::Authorization,
            Self::ValidationError { .. } | Self::BadRequest { .. } => ErrorCategory::Validation,
            Self::NotFound { .. } | Self::Conflict { .. } => ErrorCategory::Resource,
            Self::DatabaseError { .. } | Self::DatabaseConnectionError { .. } => ErrorCategory::Database,
            Self::ExternalServiceError { .. } => ErrorCategory::ExternalService,
            Self::RateLimitExceeded { .. } => ErrorCategory::RateLimit,
            Self::BusinessRuleViolation { .. } => ErrorCategory::Business,
            Self::CryptoError { .. } => ErrorCategory::Security,
            _ => ErrorCategory::System,
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::ServiceUnavailable { .. }
            | Self::DatabaseConnectionError { .. }
            | Self::ExternalServiceError { .. }
            | Self::InternalServerError { .. }
        )
    }

    /// Get retry delay in seconds
    pub fn retry_delay(&self) -> Option<u64> {
        match self {
            Self::RateLimitExceeded { retry_after, .. } => *retry_after,
            Self::ServiceUnavailable { retry_after, .. } => *retry_after,
            _ if self.is_retryable() => Some(5), // Default 5 second delay
            _ => None,
        }
    }
}

// Conversions from external error types
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::database_error("database_operation".to_string(), None, err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError {
            operation: "io_operation".to_string(),
            path: None,
            source: err.to_string(),
            error_id: Uuid::new_v4(),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::InvalidToken {
            reason: err.to_string(),
            error_id: Uuid::new_v4(),
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        Self::CryptoError {
            operation: "password_hashing".to_string(),
            error_id: Uuid::new_v4(),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let error_messages: Vec<String> = errors
            .field_errors()
            .into_iter()
            .map(|(field, errors)| {
                let messages: Vec<String> = errors
                    .iter()
                    .map(|e| e.message.as_ref().map(|m| m.to_string()).unwrap_or_else(|| "Invalid value".to_string()))
                    .collect();
                format!("{}: {}", field, messages.join(", "))
            })
            .collect();

        Self::validation_error(None, error_messages.join("; "))
    }
}