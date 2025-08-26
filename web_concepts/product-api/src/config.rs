use crate::error::AppError;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub rate_limit_per_ip: u32,
    pub rate_limit_per_user: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv().ok();

        println!("Loading configuration from environment...");

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:password@localhost:5432/product_api".to_string()
        });
        println!("Database URL: {database_url}");

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-here-make-it-long-and-secure".to_string());
        println!("JWT secret loaded (length: {})", jwt_secret.len());

        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string()) // 24 hours
            .parse()
            .map_err(|e| {
                eprintln!("Failed to parse JWT_EXPIRATION: {e}");
                AppError::BadRequest {
                    message: "Invalid JWT_EXPIRATION".to_string(),
                    error_id: uuid::Uuid::new_v4(),
                }
            })?;
        println!("JWT expiration: {jwt_expiration} seconds");

        let rate_limit_per_ip = env::var("RATE_LIMIT_PER_IP")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .map_err(|e| {
                eprintln!("Failed to parse RATE_LIMIT_PER_IP: {e}");
                AppError::BadRequest {
                    message: "Invalid RATE_LIMIT_PER_IP".to_string(),
                    error_id: uuid::Uuid::new_v4(),
                }
            })?;
        println!("Rate limit per IP: {rate_limit_per_ip} requests/minute");

        let rate_limit_per_user = env::var("RATE_LIMIT_PER_USER")
            .unwrap_or_else(|_| "200".to_string())
            .parse()
            .map_err(|e| {
                eprintln!("Failed to parse RATE_LIMIT_PER_USER: {e}");
                AppError::BadRequest {
                    message: "Invalid RATE_LIMIT_PER_USER".to_string(),
                    error_id: uuid::Uuid::new_v4(),
                }
            })?;
        println!("Rate limit per user: {rate_limit_per_user} requests/minute");

        println!("Configuration loaded successfully!");

        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expiration,
            rate_limit_per_ip,
            rate_limit_per_user,
        })
    }
}
