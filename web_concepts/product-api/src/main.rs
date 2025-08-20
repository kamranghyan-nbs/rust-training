use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower::limit::RateLimitLayer;

mod config;
mod entities;
mod error;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;

use config::Config;
use error::AppError;
use handlers::{auth, product};
use middleware::auth::auth_middleware;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Arc<Config>,
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

    // Create app state
    tracing::info!("Creating application state...");
    let state = AppState { db, config };

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

async fn connect_with_retry(database_url: &str) -> Result<DatabaseConnection, AppError> {
    let mut retries = 5;
    let mut delay = std::time::Duration::from_secs(1);

    loop {
        match Database::connect(database_url).await {
            Ok(db) => {
                tracing::info!("Successfully connected to database");
                return Ok(db);
            }
            Err(e) if retries > 0 => {
                tracing::warn!("Failed to connect to database, retrying in {:?}. Retries left: {}", delay, retries);
                tokio::time::sleep(delay).await;
                retries -= 1;
                delay *= 2; // Exponential backoff
            }
            Err(e) => {
                tracing::error!("Failed to connect to database after all retries: {}", e);
                return Err(AppError::DatabaseError(e));
            }
        }
    }
}

async fn create_app(state: AppState) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/products", get(product::get_all_products))
        .route("/products", post(product::create_product))
        .route("/products/:id", get(product::get_product))
        .route("/products/:id", put(product::update_product))
        .route("/products/:id", delete(product::delete_product))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(RateLimitLayer::new(1, std::time::Duration::from_secs(10)))
        )
        .with_state(state)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Service is healthy")
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::Conflict(_) => (StatusCode::CONFLICT, "Conflict"),
            AppError::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO error"),
            AppError::JwtError(_) => (StatusCode::UNAUTHORIZED, "Authentication error"),
            AppError::BcryptError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Password processing error"),
            AppError::ParseError(_) => (StatusCode::BAD_REQUEST, "Parse error"),
        };

        let body = serde_json::json!({
            "error": error_message,
            "details": self.to_string()
        });

        (status, axum::Json(body)).into_response()
    }
}