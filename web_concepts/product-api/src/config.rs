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