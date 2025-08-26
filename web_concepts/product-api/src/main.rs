use axum::{
    http::StatusCode,
    response::{IntoResponse},
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info, instrument, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

mod config;
mod entities;
mod error;
mod handlers;
mod logging;
mod middleware;
mod models;
mod repository;
mod services;
mod utils;

use config::Config;
use error::AppError;
use handlers::{auth, product};
use middleware::{
    auth::auth_middleware,
    logging::request_logging_middleware,
    rate_limit::{ip_rate_limit_middleware, user_rate_limit_middleware, RateLimiter},
    rbac::{
        require_create_permission, require_delete_permission, require_read_permission,
        require_update_permission,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Arc<Config>,
    pub rate_limiter: RateLimiter,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing early
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "product_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Product API application...");

    // Load configuration
    tracing::info!("Loading configuration...");
    let config = Arc::new(Config::from_env()?);
    tracing::info!("Configuration loaded successfully");

    // Connect to database with retries
    tracing::info!("Connecting to database: {}", config.database_url);
    let db = connect_with_retry(&config.database_url).await?;

    // Create rate limiter
    tracing::info!("Initializing rate limiter...");
    let rate_limiter = RateLimiter::new(config.rate_limit_per_ip, config.rate_limit_per_user);

    // Create app state
    tracing::info!("Creating application state...");
    let state = AppState {
        db,
        config,
        rate_limiter,
    };

    // Build the application router
    tracing::info!("Building application router...");
    let app = create_app(state).await;

    // Start the server
    tracing::info!("Binding to address 0.0.0.0:8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server starting successfully on http://0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}

#[instrument(name = "database_connection", skip(database_url))]
async fn connect_with_retry(database_url: &str) -> Result<DatabaseConnection, AppError> {
    let mut retries = 5;
    let mut delay = std::time::Duration::from_secs(1);

    info!("Starting database connection with retry logic...");

    loop {
        let connection_attempt = retries;
        match Database::connect(database_url).await {
            Ok(db) => {
                info!(
                    database_url = %database_url,
                    attempts_used = %(6 - retries),
                    "Successfully connected to database"
                );
                return Ok(db);
            }
            Err(e) if retries > 0 => {
                warn!(
                    error = %e,
                    retries_left = %retries,
                    delay_seconds = %delay.as_secs(),
                    attempt = %connection_attempt,
                    "Failed to connect to database, retrying..."
                );

                tokio::time::sleep(delay).await;
                retries -= 1;
                delay *= 2; // Exponential backoff
            }
            Err(e) => {
                error!(
                    error = %e,
                    database_url = %database_url,
                    total_attempts = %6,
                    "Failed to connect to database after all retries"
                );
                return Err(AppError::DatabaseError {
                    operation: "insert".to_string(),
                    table: Some("users".to_string()),
                    details: e.to_string(),
                    error_id: Uuid::new_v4(),
                });
            }
        }
    }
}

// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal, starting graceful shutdown...");
        },
        _ = terminate => {
            info!("Received terminate signal, starting graceful shutdown...");
        },
    }
}

async fn create_app(state: AppState) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .layer(axum::middleware::from_fn_with_state(
            state.rate_limiter.clone(),
            ip_rate_limit_middleware,
        ));

    // Read-only routes (all authenticated users can access)
    let read_only_routes = Router::new()
        .route("/products", get(product::get_all_products))
        .route("/products/:id", get(product::get_product))
        .route("/products/search", get(product::search_products))
        .route("/products/category", get(product::get_products_by_category))
        .route(
            "/products/price-range",
            get(product::get_products_by_price_range),
        )
        .route("/products/low-stock", get(product::get_low_stock_products))
        .route("/products/similar", get(product::get_similar_products))
        .route("/products/stats", get(product::get_product_stats))
        .route(
            "/products/trending-categories",
            get(product::get_trending_categories),
        )
        .layer(axum::middleware::from_fn(require_read_permission));

    // Create routes (Admin and Manager can access)
    let create_routes = Router::new()
        .route("/products", post(product::create_product))
        .layer(axum::middleware::from_fn(require_create_permission));

    // Update routes (Admin and Manager can access)
    let update_routes = Router::new()
        .route("/products/:id", put(product::update_product))
        .layer(axum::middleware::from_fn(require_update_permission));

    // Delete routes (Admin only)
    let delete_routes = Router::new()
        .route("/products/:id", delete(product::delete_product))
        .layer(axum::middleware::from_fn(require_delete_permission));

    // Combine all protected routes
    let protected_routes = Router::new()
        .merge(read_only_routes)
        .merge(create_routes)
        .merge(update_routes)
        .merge(delete_routes)
        // Apply authentication and rate limiting to all protected routes
        .layer(axum::middleware::from_fn_with_state(
            state.rate_limiter.clone(),
            user_rate_limit_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(
            ServiceBuilder::new()
                // Add request logging middleware (outermost layer for full request/response logging)
                .layer(axum::middleware::from_fn(request_logging_middleware))
                // Add HTTP tracing for internal spans
                .layer(TraceLayer::new_for_http())
                // Add CORS support
                .layer(CorsLayer::permissive()),
        )
        .with_state(state)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Service is healthy")
}
