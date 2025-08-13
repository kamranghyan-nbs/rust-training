use crate::{
    error::AppError,
    models::{CreateProductRequest, UpdateProductRequest},
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
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(request): Json<CreateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let response = ProductService::create_product(&state.db, request).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_all_products(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let response = ProductService::get_all_products(&state.db, params.page, params.per_page).await?;

    Ok((StatusCode::OK, Json(response)))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let response = ProductService::get_product(&state.db, id).await?;

    Ok((StatusCode::OK, Json(response)))
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    request.validate()?;

    let response = ProductService::update_product(&state.db, id, request).await?;

    Ok((StatusCode::OK, Json(response)))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    ProductService::delete_product(&state.db, id).await?;

    Ok(StatusCode::NO_CONTENT)
}