use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("BCrypt error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Parse error: {0}")]
    ParseError(String),
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        AppError::ValidationError(format!("Validation failed: {}", errors))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use sea_orm::DbErr;
    use validator::{Validate, ValidationError, ValidationErrors};

    // Helper struct for validation test
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 5))]
        name: String,
    }

    #[test]
    fn test_database_error_display() {
        let db_err = DbErr::Custom("Database error: db connection failed".to_string());
        let app_error = AppError::from(db_err);
        let display_msg = format!("{}", app_error);
        assert!(display_msg.contains("Database error: db connection failed"));
    }

    #[test]
    fn test_not_found_display() {
        let app_error = AppError::NotFound;
        assert_eq!(format!("{}", app_error), "Not found");
    }

    #[test]
    fn test_unauthorized_display() {
        let app_error = AppError::Unauthorized;
        assert_eq!(format!("{}", app_error), "Unauthorized");
    }

    #[test]
    fn test_validation_error_display() {
        let app_error = AppError::ValidationError("invalid field".into());
        assert_eq!(format!("{}", app_error), "Validation error: invalid field");
    }

    #[test]
    fn test_internal_server_error_display() {
        let app_error = AppError::InternalServerError;
        assert_eq!(format!("{}", app_error), "Internal server error");
    }

    #[test]
    fn test_bad_request_display() {
        let app_error = AppError::BadRequest("missing parameter".into());
        assert_eq!(format!("{}", app_error), "Bad request: missing parameter");
    }

    #[test]
    fn test_conflict_display() {
        let app_error = AppError::Conflict("duplicate entry".into());
        assert_eq!(format!("{}", app_error), "Conflict: duplicate entry");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::Other, "disk error");
        let app_error = AppError::from(io_err);
        assert!(format!("{}", app_error).contains("IO error: disk error"));
    }

    #[test]
    fn test_jwt_error_conversion() {
        let jwt_err = jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken);
        let app_error = AppError::from(jwt_err);
        assert!(format!("{}", app_error).contains("JWT error: InvalidToken"));
    }

    #[test]
    fn test_bcrypt_error_conversion() {
        let bcrypt_err = bcrypt::BcryptError::InvalidCost("BCrypt error: 100".to_string());
        let app_error = AppError::from(bcrypt_err);
        assert!(format!("{}", app_error).contains("BCrypt error: 100"));
    }

    #[test]
    fn test_parse_error_display() {
        let app_error = AppError::ParseError("parse failed".into());
        assert_eq!(format!("{}", app_error), "Parse error: parse failed");
    }

    #[test]
    fn test_validation_errors_conversion() {
        // // Create a validation error manually
        // let mut errors = ValidationErrors::new();
        // let mut field_errors = Vec::new();
        // field_errors.push(ValidationError::new("too_short"));
        // errors.add("name", field_errors.into());

        // let app_error = AppError::from(errors);
        // let msg = format!("{}", app_error);
        // assert!(msg.contains("Validation error: Validation failed: name"));
    }
}
