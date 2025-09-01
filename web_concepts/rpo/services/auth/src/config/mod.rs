use crate_config::{DatabaseConfig, JwtConfig, ServerConfig, LoggingConfig, RedisConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub redis: Option<RedisConfig>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            jwt: JwtConfig::default(),
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
            redis: Some(RedisConfig::default()),
        }
    }
}