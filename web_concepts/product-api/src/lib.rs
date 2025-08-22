// src/lib.rs
pub mod config;
pub mod entities;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;
pub mod utils;
pub mod repository;
pub mod logging;

pub use crate::error::AppError;

// move AppState struct here:
use std::sync::Arc;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Arc<crate::config::Config>,
}
