use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: env::var("AUTH_DATABASE__URL").unwrap_or_else(|_| "postgres://localhost:5432/pos_auth".to_string()),
            max_connections: env::var("AUTH_DATABASE__MAX_CONNECTIONS").unwrap_or_else(|_| "10".to_string()).parse().unwrap_or(10),
            min_connections: env::var("AUTH_DATABASE__MIN_CONNECTIONS").unwrap_or_else(|_| "2".to_string()).parse().unwrap_or(2),
            acquire_timeout: env::var("AUTH_DATABASE__ACQUIRE_TIMEOUT").unwrap_or_else(|_| "30".to_string()).parse().unwrap_or(30),
            idle_timeout: env::var("AUTH_DATABASE__IDLE_TIMEOUT").unwrap_or_else(|_| "300".to_string()).parse().unwrap_or(300),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expire_minutes: u64,
    pub refresh_token_expire_days: u64,
    pub issuer: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: env::var("AUTH_JWT__SECRET").unwrap_or_else(|_| "default-secret-change-in-production".to_string()),
            access_token_expire_minutes: env::var("AUTH_JWT__ACCESS_TOKEN_EXPIRE_MINUTES").unwrap_or_else(|_| "60".to_string()).parse().unwrap_or(60),
            refresh_token_expire_days: env::var("AUTH_JWT__REFRESH_TOKEN_EXPIRE_DAYS").unwrap_or_else(|_| "7".to_string()).parse().unwrap_or(7),
            issuer: env::var("AUTH_JWT__ISSUER").unwrap_or_else(|_| "pos-system".to_string()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: env::var("AUTH_SERVER__HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("AUTH_SERVER__PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080),
            workers: env::var("AUTH_SERVER__WORKERS").unwrap_or_else(|_| "4".to_string()).parse().unwrap_or(4),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub jaeger_endpoint: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: env::var("AUTH_LOGGING__LEVEL").unwrap_or_else(|_| "info".to_string()),
            format: env::var("AUTH_LOGGING__FORMAT").unwrap_or_else(|_| "pretty".to_string()),
            jaeger_endpoint: env::var("AUTH_LOGGING__JAEGER_ENDPOINT").ok(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: env::var("AUTH_REDIS__URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            pool_size: env::var("AUTH_REDIS__POOL_SIZE").unwrap_or_else(|_| "10".to_string()).parse().unwrap_or(10),
        }
    }
}

pub trait ConfigLoader<T> {
    fn load(config_path: Option<&str>) -> anyhow::Result<T>;
}

pub fn load_config<T>(config_path: Option<&str>, _service_name: &str) -> anyhow::Result<T>
where
    T: Default,
{
    // For now, just use the Default implementation which reads from env vars
    // You can add TOML file loading later if needed
    if let Some(path) = config_path {
        if !std::path::Path::new(path).exists() {
            eprintln!("Config file {} not found, using environment variables and defaults", path);
        }
    }
    
    Ok(T::default())
}