use auth_service::{
    config::AuthConfig,
    infrastructure::database::Database,
    presentation::routes::create_router,
    migrations::Migrator,
};
use crate_config::load_config;
use crate_logging::{init_tracing, LoggingConfig, LogFormat};
use sea_orm_migration::MigratorTrait;
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use std::time::Duration;
use anyhow::Result; // Add this import

#[tokio::main]
async fn main() -> Result<()> { // Change return type
    // Load configuration
    let config_path = env::var("AUTH_CONFIG_PATH")
        .ok()
        .or_else(|| Some("config/dev/auth-service.toml".to_string()));
    
    let config: AuthConfig = load_config(config_path.as_deref(), "auth")?;

     // Debug: print config (without secrets)
    println!("Database URL: {}", config.database.url.chars().take(20).collect::<String>() + "...");
    println!("Server: {}:{}", config.server.host, config.server.port);
    println!("JWT Issuer: {}", config.jwt.issuer);

    // Initialize logging
    let logging_config = LoggingConfig {
        level: config.logging.level.clone(),
        format: LogFormat::from(config.logging.format.as_str()),
        service_name: "auth-service".to_string(),
        jaeger_endpoint: config.logging.jaeger_endpoint.clone(),
    };
    init_tracing(logging_config)?; // This should work now

    tracing::info!("Starting auth service...");

    // Connect to database
    let database = Database::new(&config.database).await?;
    
    // Run migrations
    Migrator::up(database.get_connection(), None).await?;
    tracing::info!("Database migrations completed");

    // Create application router
    let app = create_router(config.clone(), database)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(crate_logging::middleware::RequestTracingLayer)
        );

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port)).await?;
    
    tracing::info!("Auth service listening on http://{}:{}", config.server.host, config.server.port);
    tracing::info!("Swagger UI available at http://{}:{}/swagger-ui/", config.server.host, config.server.port);

    axum::serve(listener, app).await?;

    Ok(())
}