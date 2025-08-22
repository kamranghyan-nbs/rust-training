use crate::{
    error::AppError,
    models::{CreateProductRequest, UpdateProductRequest, ProductSearchRequest},
    repository::product::ProductRepository,
    services::ProductService,
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryQuery {
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct PriceRangeQuery {
    pub min_price: Decimal,
    pub max_price: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct LowStockQuery {
    pub threshold: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SimilarProductsQuery {
    pub name: String,
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct TrendingQuery {
    pub limit: Option<u64>,
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(request): Json<CreateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.create_product(request).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_all_products(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.get_all_products(params.page, params.per_page).await?;

    Ok((StatusCode::OK, Json(response)))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.get_product(id).await?;

    Ok((StatusCode::OK, Json(response)))
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.update_product(id, request).await?;

    Ok((StatusCode::OK, Json(response)))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    product_service.delete_product(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

// NEW SEARCH ENDPOINTS

// Advanced search with multiple filters
pub async fn search_products(
    State(state): State<AppState>,
    Query(search_request): Query<ProductSearchRequest>,
) -> Result<impl IntoResponse, AppError> {
    search_request.validate()?;

    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.search_products(search_request).await?;

    Ok((StatusCode::OK, Json(response)))
}

// Get products by category
pub async fn get_products_by_category(
    State(state): State<AppState>,
    Query(params): Query<CategoryQuery>,
) -> Result<impl IntoResponse, AppError> {
    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.get_products_by_category(&params.category).await?;

    Ok((StatusCode::OK, Json(response)))
}

// Get products by price range
pub async fn get_products_by_price_range(
    State(state): State<AppState>,
    Query(params): Query<PriceRangeQuery>,
) -> Result<impl IntoResponse, AppError> {
    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.get_products_by_price_range(params.min_price, params.max_price).await?;

    Ok((StatusCode::OK, Json(response)))
}

// Get low stock products
pub async fn get_low_stock_products(
    State(state): State<AppState>,
    Query(params): Query<LowStockQuery>,
) -> Result<impl IntoResponse, AppError> {
    let threshold = params.threshold.unwrap_or(10); // Default threshold of 10

    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.get_low_stock_products(threshold).await?;

    Ok((StatusCode::OK, Json(response)))
}

// Get product statistics
pub async fn get_product_stats(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.get_product_stats().await?;

    Ok((StatusCode::OK, Json(response)))
}

// Get similar products
pub async fn get_similar_products(
    State(state): State<AppState>,
    Query(params): Query<SimilarProductsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = params.limit.unwrap_or(5);

    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.get_similar_products(&params.name, limit).await?;

    Ok((StatusCode::OK, Json(response)))
}

// Get trending categories
pub async fn get_trending_categories(
    State(state): State<AppState>,
    Query(params): Query<TrendingQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = params.limit.unwrap_or(10);

    let product_repository = Arc::new(ProductRepository::new(state.db.clone()));
    let product_service = ProductService::new(product_repository);
    
    let response = product_service.get_trending_categories(limit).await?;

    Ok((StatusCode::OK, Json(response)))
}