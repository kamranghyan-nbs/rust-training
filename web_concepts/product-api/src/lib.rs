// src/lib.rs
pub mod config;
pub mod entities;
pub mod error;
pub mod handlers;
pub mod logging;
pub mod middleware;
pub mod models;
pub mod repository;
pub mod services;
pub mod utils;

pub use crate::error::AppError;

// move AppState struct here:
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Arc<crate::config::Config>,
}
