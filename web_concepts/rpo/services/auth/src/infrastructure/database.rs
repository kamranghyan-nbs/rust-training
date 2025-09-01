use crate_config::DatabaseConfig;
use crate_errors::{AppError, Result};
use sea_orm::{Database as SeaDatabase, DatabaseConnection, ConnectOptions};
use std::time::Duration;

#[derive(Clone)]
pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let mut options = ConnectOptions::new(&config.url);
        options
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout))
            .idle_timeout(Duration::from_secs(config.idle_timeout))
            .sqlx_logging(true);

        let connection = SeaDatabase::connect(options).await
            .map_err(|e| AppError::Database(e))?;

        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}