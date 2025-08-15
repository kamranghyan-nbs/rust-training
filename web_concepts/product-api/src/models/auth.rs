use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 100, message = "Username must be between 3 and 100 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub username: String,
    pub exp: usize,
}


#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{automock, predicate::*};
    use validator::Validate;
    use uuid::Uuid;

    // ===== Example Trait to Mock for Testing =====
    #[automock]
    trait AuthServiceTrait {
        fn login(&self, request: &LoginRequest) -> bool;
        fn register(&self, request: &RegisterRequest) -> bool;
    }

    #[test]
    fn test_login_request_valid() {
        let req = LoginRequest {
            username: "testuser".into(),
            password: "secret".into(),
        };

        let result = req.validate();
        assert!(result.is_ok(), "Expected valid login request");
    }

    #[test]
    fn test_login_request_invalid_empty_fields() {
        let req = LoginRequest {
            username: "".into(),
            password: "".into(),
        };

        let result = req.validate();
        assert!(result.is_err(), "Expected validation errors for empty fields");
        let errors = result.unwrap_err();
        let msg = format!("{:?}", errors);
        assert!(msg.contains("Username is required"));
        assert!(msg.contains("Password is required"));
    }

    #[test]
    fn test_register_request_valid() {
        let req = RegisterRequest {
            username: "validusername".into(),
            email: "test@example.com".into(),
            password: "strongpass".into(),
        };

        let result = req.validate();
        assert!(result.is_ok(), "Expected valid register request");
    }

    #[test]
    fn test_register_request_invalid_username_too_short() {
        let req = RegisterRequest {
            username: "ab".into(), // too short
            email: "test@example.com".into(),
            password: "strongpass".into(),
        };

        let result = req.validate();
        assert!(result.is_err(), "Expected username length validation error");
        let msg = format!("{:?}", result.unwrap_err());
        assert!(msg.contains("Username must be between 3 and 100 characters"));
    }

    #[test]
    fn test_register_request_invalid_email_format() {
        let req = RegisterRequest {
            username: "validusername".into(),
            email: "not-an-email".into(),
            password: "strongpass".into(),
        };

        let result = req.validate();
        assert!(result.is_err(), "Expected invalid email format error");
        let msg = format!("{:?}", result.unwrap_err());
        assert!(msg.contains("Invalid email format"));
    }

    #[test]
    fn test_register_request_invalid_password_too_short() {
        let req = RegisterRequest {
            username: "validusername".into(),
            email: "test@example.com".into(),
            password: "123".into(), // too short
        };

        let result = req.validate();
        assert!(result.is_err(), "Expected password length validation error");
        let msg = format!("{:?}", result.unwrap_err());
        assert!(msg.contains("Password must be at least 6 characters"));
    }

    #[test]
    fn test_auth_service_mock_login() {
        let mut mock_service = MockAuthServiceTrait::new();

        let valid_req = LoginRequest {
            username: "mockuser".into(),
            password: "mockpass".into(),
        };

        // Expect login to be called with our valid request and return true
        mock_service
            .expect_login()
            .withf(|req| req.username == "mockuser" && req.password == "mockpass")
            .return_const(true);

        let result = mock_service.login(&valid_req);
        assert!(result);
    }

    #[test]
    fn test_auth_service_mock_register() {
        let mut mock_service = MockAuthServiceTrait::new();

        let valid_req = RegisterRequest {
            username: "mockuser".into(),
            email: "mock@example.com".into(),
            password: "mockpass".into(),
        };

        // Expect register to be called with our valid request and return true
        mock_service
            .expect_register()
            .withf(|req| req.username == "mockuser" && req.email == "mock@example.com")
            .return_const(true);

        let result = mock_service.register(&valid_req);
        assert!(result);
    }

    #[test]
    fn test_auth_response_and_claims_serialization() {
        let user = UserResponse {
            id: Uuid::new_v4(),
            username: "user1".into(),
            email: "user1@example.com".into(),
        };

        let response = AuthResponse {
            token: "sometoken".into(),
            user,
        };

        let json_str = serde_json::to_string(&response).unwrap();
        assert!(json_str.contains("sometoken"));
        assert!(json_str.contains("user1@example.com"));

        let claims = Claims {
            sub: "userid".into(),
            username: "user1".into(),
            exp: 123456,
        };

        let claims_json = serde_json::to_string(&claims).unwrap();
        assert!(claims_json.contains("userid"));
    }
}
