use crate::error::AppError;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv().ok();

        println!("Loading configuration from environment...");

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/product_api".to_string());
        println!("Database URL: {}", database_url);

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-here-make-it-long-and-secure".to_string());
        println!("JWT secret loaded (length: {})", jwt_secret.len());

        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string()) // 24 hours
            .parse()
            .map_err(|e| {
                eprintln!("Failed to parse JWT_EXPIRATION: {}", e);
                AppError::BadRequest("Invalid JWT_EXPIRATION".to_string())
            })?;
        println!("JWT expiration: {} seconds", jwt_expiration);

        println!("Configuration loaded successfully!");
        
        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expiration,
        })
    }
}

impl Config {
    pub fn test() -> Self {
        Self {
            database_url: "postgres://localhost/test_db".into(), // mock URL
            jwt_secret: "test-secret".into(),
            jwt_expiration: 3600,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use std::env;

    // Step 1: Create a trait for environment variable access
    #[automock]
    trait EnvProvider {
        fn get_var(&self, key: &str) -> Result<String, env::VarError>;
    }

    struct RealEnv;

    impl EnvProvider for RealEnv {
        fn get_var(&self, key: &str) -> Result<String, env::VarError> {
            env::var(key)
        }
    }

    // Step 2: Helper function to build Config from mocked environment
    fn config_from_mock_env(mock_env: &dyn EnvProvider) -> Result<Config, AppError> {
        let database_url = mock_env
            .get_var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/product_api".to_string());

        let jwt_secret = mock_env
            .get_var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-here-make-it-long-and-secure".to_string());

        let jwt_expiration = mock_env
            .get_var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string())
            .parse()
            .map_err(|_| AppError::BadRequest("Invalid JWT_EXPIRATION".to_string()))?;

        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expiration,
        })
    }

    #[test]
    fn test_from_env_all_values_present() {
        let mut mock = MockEnvProvider::new();
        mock.expect_get_var()
            .with(eq("DATABASE_URL"))
            .returning(|_| Ok("postgres://mocked-url".to_string()));
        mock.expect_get_var()
            .with(eq("JWT_SECRET"))
            .returning(|_| Ok("mocked-secret".to_string()));
        mock.expect_get_var()
            .with(eq("JWT_EXPIRATION"))
            .returning(|_| Ok("7200".to_string())); // 2 hours

        let config = config_from_mock_env(&mock).unwrap();
        assert_eq!(config.database_url, "postgres://mocked-url");
        assert_eq!(config.jwt_secret, "mocked-secret");
        assert_eq!(config.jwt_expiration, 7200);
    }

    #[test]
    fn test_from_env_defaults_used() {
        let mut mock = MockEnvProvider::new();
        mock.expect_get_var()
            .with(eq("DATABASE_URL"))
            .returning(|_| Err(env::VarError::NotPresent));
        mock.expect_get_var()
            .with(eq("JWT_SECRET"))
            .returning(|_| Err(env::VarError::NotPresent));
        mock.expect_get_var()
            .with(eq("JWT_EXPIRATION"))
            .returning(|_| Err(env::VarError::NotPresent));

        let config = config_from_mock_env(&mock).unwrap();
        assert_eq!(config.database_url, "postgres://postgres:password@localhost:5432/product_api");
        assert_eq!(config.jwt_secret, "your-secret-key-here-make-it-long-and-secure");
        assert_eq!(config.jwt_expiration, 86400);
    }

    #[test]
    fn test_from_env_invalid_jwt_expiration() {
        let mut mock = MockEnvProvider::new();
        mock.expect_get_var()
            .with(eq("DATABASE_URL"))
            .returning(|_| Ok("postgres://mocked-url".to_string()));
        mock.expect_get_var()
            .with(eq("JWT_SECRET"))
            .returning(|_| Ok("mocked-secret".to_string()));
        mock.expect_get_var()
            .with(eq("JWT_EXPIRATION"))
            .returning(|_| Ok("invalid-number".to_string()));

        let result = config_from_mock_env(&mock);
        assert!(result.is_err());
        if let Err(AppError::BadRequest(msg)) = result {
            assert_eq!(msg, "Invalid JWT_EXPIRATION");
        } else {
            panic!("Expected BadRequest error");
        }
    }
}
