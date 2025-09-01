use auth_service::migrations::Migrator;
use crate_config::{load_config, DatabaseConfig};
use sea_orm_migration::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MigratorConfig {
    database: DatabaseConfig,
}

// Add Default implementation for MigratorConfig
impl Default for MigratorConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config_path = env::var("AUTH_CONFIG_PATH").ok();
    
    let config: MigratorConfig = load_config(config_path.as_deref(), "auth")?;

    // Connect to database
    let db = sea_orm::Database::connect(&config.database.url).await?;

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("up");

    match command {
        "up" => {
            println!("Running migrations...");
            Migrator::up(&db, None).await?;
            println!("Migrations completed successfully!");
        }
        "down" => {
            let steps = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
            println!("Rolling back {} migration(s)...", steps);
            Migrator::down(&db, Some(steps)).await?;
            println!("Rollback completed successfully!");
        }
        "status" => {
            println!("Checking migration status...");
            let status = Migrator::status(&db).await?;
            println!("Migration status: {:?}", status);
        }
        _ => {
            eprintln!("Usage: migrator [up|down|status] [steps]");
            std::process::exit(1);
        }
    }

    Ok(())
}