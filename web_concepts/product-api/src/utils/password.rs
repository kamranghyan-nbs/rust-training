use crate::error::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(AppError::from)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(AppError::from)
}


// ----- Mockable trait for testing -----
pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, AppError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError>;
}

pub struct BcryptPasswordHasher;

impl PasswordHasher for BcryptPasswordHasher {
    fn hash(&self, password: &str) -> Result<String, AppError> {
        hash_password(password)
    }
    fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        verify_password(password, hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    // Mock trait for PasswordHasher
    mock! {
        pub Hasher {}
        impl PasswordHasher for Hasher {
            fn hash(&self, password: &str) -> Result<String, AppError>;
            fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError>;
        }
    }

    #[test]
    fn test_hash_password_real() {
        let password = "my_secret";
        let hashed = hash_password(password).unwrap();
        assert!(hashed.starts_with("$2b$")); // bcrypt hashes start with $2b$
    }

    #[test]
    fn test_verify_password_real() {
        let password = "my_secret";
        let hashed = hash_password(password).unwrap();
        let is_valid = verify_password(password, &hashed).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_verify_password_real_fail() {
        let password = "my_secret";
        let hashed = hash_password(password).unwrap();
        let is_valid = verify_password("wrong_password", &hashed).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_mock_hash_success() {
        let mut mock = MockHasher::new();
        mock.expect_hash()
            .with(eq("mock_pass"))
            .returning(|_| Ok("mock_hash".to_string()));

        let result = mock.hash("mock_pass").unwrap();
        assert_eq!(result, "mock_hash");
    }

    #[test]
    fn test_mock_hash_failure() {
        let mut mock = MockHasher::new();
        mock.expect_hash()
            .with(eq("mock_pass"))
            .returning(|_| Err(AppError::BcryptError(
                bcrypt::BcryptError::InvalidCost("50".to_string()),
            )));

        let result = mock.hash("mock_pass");
        assert!(matches!(result, Err(AppError::BcryptError(_))));
    }

    #[test]
    fn test_mock_verify_success() {
        let mut mock = MockHasher::new();
        mock.expect_verify()
            .with(eq("mock_pass"), eq("mock_hash"))
            .returning(|_, _| Ok(true));

        let result = mock.verify("mock_pass", "mock_hash").unwrap();
        assert!(result);
    }

    #[test]
    fn test_mock_verify_failure() {
        let mut mock = MockHasher::new();
        mock.expect_verify()
            .with(eq("mock_pass"), eq("mock_hash"))
            .returning(|_, _| Err(AppError::BcryptError(
                bcrypt::BcryptError::InvalidCost("50".to_string()),
            )));

        let result = mock.verify("mock_pass", "mock_hash");
        assert!(matches!(result, Err(AppError::BcryptError(_))));
    }
}
