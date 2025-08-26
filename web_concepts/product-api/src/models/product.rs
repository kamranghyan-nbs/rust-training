use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    #[validate(range(min = 0, message = "Quantity must be non-negative"))]
    pub quantity: i32,
    pub category: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub quantity: Option<i32>,
    pub category: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct ProductSearchRequest {
    pub query: Option<String>,      // General text search
    pub name: Option<String>,       // Search by name
    pub category: Option<String>,   // Filter by category
    pub min_price: Option<Decimal>, // Price range filter
    pub max_price: Option<Decimal>,
    pub min_quantity: Option<i32>, // Quantity range filter
    pub max_quantity: Option<i32>,
    pub in_stock: Option<bool>,     // Filter by stock availability
    pub sort_by: Option<String>,    // Sort field (name, price, created_at)
    pub sort_order: Option<String>, // Sort order (asc, desc)
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub quantity: i32,
    pub category: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProductListResponse {
    pub products: Vec<ProductResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize)]
pub struct ProductSearchResponse {
    pub products: Vec<ProductResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub filters_applied: ProductSearchFilters,
}

#[derive(Debug, Serialize)]
pub struct ProductSearchFilters {
    pub query: Option<String>,
    pub category: Option<String>,
    pub price_range: Option<(Decimal, Decimal)>,
    pub quantity_range: Option<(i32, i32)>,
    pub in_stock: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ProductStatsResponse {
    pub total_products: u64,
    pub total_value: Decimal,
    pub avg_price: Option<Decimal>,
    pub categories: Vec<CategoryStats>,
}

#[derive(Debug, Serialize)]
pub struct CategoryStats {
    pub category: String,
    pub count: u64,
    pub total_value: Decimal,
}
