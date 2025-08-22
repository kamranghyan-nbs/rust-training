use crate::{
    error::AppError,
    models::{
        CreateProductRequest, ProductListResponse, ProductResponse, UpdateProductRequest,
        ProductSearchRequest, ProductSearchResponse, ProductSearchFilters, ProductStatsResponse,
    },
    repository::product::ProductRepositoryTrait,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct ProductService<T: ProductRepositoryTrait> {
    product_repository: Arc<T>,
}

impl<T: ProductRepositoryTrait> ProductService<T> {
    pub fn new(product_repository: Arc<T>) -> Self {
        Self { product_repository }
    }

    pub async fn create_product(
        &self,
        request: CreateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let product = self.product_repository.create(request).await?;
        Ok(ProductResponse::from(product))
    }

    pub async fn get_all_products(
        &self,
        page: Option<u64>,
        per_page: Option<u64>,
    ) -> Result<ProductListResponse, AppError> {
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(10).min(100);

        let (products, total) = self.product_repository.find_all(Some(page), Some(per_page)).await?;

        let products = products
            .into_iter()
            .map(ProductResponse::from)
            .collect();

        Ok(ProductListResponse {
            products,
            total,
            page,
            per_page,
        })
    }

    pub async fn get_product(
        &self,
        product_id: Uuid,
    ) -> Result<ProductResponse, AppError> {
        let product = self
            .product_repository
            .find_by_id(product_id)
            .await?
            .ok_or(AppError::NotFound {
                resource_type: "Product".to_string(),
                resource_id: Some(product_id.to_string()),
                error_id: uuid::Uuid::new_v4(),
            })?;

        Ok(ProductResponse::from(product))
    }

    pub async fn update_product(
        &self,
        product_id: Uuid,
        request: UpdateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let updated_product = self.product_repository.update(product_id, request).await?;
        Ok(ProductResponse::from(updated_product))
    }

    pub async fn delete_product(
        &self,
        product_id: Uuid,
    ) -> Result<(), AppError> {
        let deleted = self.product_repository.delete(product_id).await?;
        
        if !deleted {
            return Err(AppError::NotFound {
                resource_type: "Product".to_string(),
                resource_id: Some(product_id.to_string()),
                error_id: uuid::Uuid::new_v4(),
            });
        }

        Ok(())
    }

    // NEW SEARCH METHODS
    pub async fn search_products(
        &self,
        search_request: ProductSearchRequest,
    ) -> Result<ProductSearchResponse, AppError> {
        let page = search_request.page.unwrap_or(1);
        let per_page = search_request.per_page.unwrap_or(10).min(100);

        let (products, total) = self.product_repository.search(search_request.clone()).await?;

        let products = products
            .into_iter()
            .map(ProductResponse::from)
            .collect();

        // Build filters applied summary
        let filters_applied = ProductSearchFilters {
            query: search_request.query,
            category: search_request.category,
            price_range: match (search_request.min_price, search_request.max_price) {
                (Some(min), Some(max)) => Some((min, max)),
                _ => None,
            },
            quantity_range: match (search_request.min_quantity, search_request.max_quantity) {
                (Some(min), Some(max)) => Some((min, max)),
                _ => None,
            },
            in_stock: search_request.in_stock,
        };

        Ok(ProductSearchResponse {
            products,
            total,
            page,
            per_page,
            filters_applied,
        })
    }

    pub async fn get_products_by_category(
        &self,
        category: &str,
    ) -> Result<Vec<ProductResponse>, AppError> {
        let products = self.product_repository.find_by_category(category).await?;
        
        Ok(products
            .into_iter()
            .map(ProductResponse::from)
            .collect())
    }

    pub async fn get_products_by_price_range(
        &self,
        min_price: rust_decimal::Decimal,
        max_price: rust_decimal::Decimal,
    ) -> Result<Vec<ProductResponse>, AppError> {
        let products = self.product_repository.find_by_price_range(min_price, max_price).await?;
        
        Ok(products
            .into_iter()
            .map(ProductResponse::from)
            .collect())
    }

    pub async fn get_low_stock_products(
        &self,
        threshold: i32,
    ) -> Result<Vec<ProductResponse>, AppError> {
        let products = self.product_repository.find_low_stock(threshold).await?;
        
        Ok(products
            .into_iter()
            .map(ProductResponse::from)
            .collect())
    }

    pub async fn get_product_stats(&self) -> Result<ProductStatsResponse, AppError> {
        self.product_repository.get_product_stats().await
    }

    pub async fn get_similar_products(
        &self,
        product_name: &str,
        limit: u64,
    ) -> Result<Vec<ProductResponse>, AppError> {
        let products = self.product_repository.find_similar_products(product_name, limit).await?;
        
        Ok(products
            .into_iter()
            .map(ProductResponse::from)
            .collect())
    }

    pub async fn get_trending_categories(
        &self,
        limit: u64,
    ) -> Result<Vec<crate::models::CategoryStats>, AppError> {
        self.product_repository.get_trending_categories(limit).await
    }
}

impl From<crate::entities::product::Model> for ProductResponse {
    fn from(product: crate::entities::product::Model) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            quantity: product.quantity,
            category: product.category,
            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}