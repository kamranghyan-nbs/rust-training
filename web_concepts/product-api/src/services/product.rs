use crate::{
    entities::{prelude::*, product},
    error::AppError,
    models::{CreateProductRequest, ProductListResponse, ProductResponse, UpdateProductRequest},
};
use sea_orm::{prelude::*, ActiveModelTrait, Set, QueryOrder, PaginatorTrait};
use uuid::Uuid;

pub struct ProductService;

impl ProductService {
    pub async fn create_product(
        db: &DatabaseConnection,
        request: CreateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let product_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let new_product = product::ActiveModel {
            id: Set(product_id),
            name: Set(request.name),
            description: Set(request.description),
            price: Set(request.price),
            quantity: Set(request.quantity),
            category: Set(request.category),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let product = new_product.insert(db).await?;
        Ok(ProductResponse::from(product))
    }

    pub async fn get_all_products(
        db: &DatabaseConnection,
        page: Option<u64>,
        per_page: Option<u64>,
    ) -> Result<ProductListResponse, AppError> {
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(10).min(100); // Limit to 100 items per page

        let paginator = Product::find()
            .order_by_desc(product::Column::CreatedAt)
            .paginate(db, per_page);

        let total = paginator.num_items().await?;
        let products = paginator
            .fetch_page(page.saturating_sub(1))
            .await?
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
        db: &DatabaseConnection,
        product_id: Uuid,
    ) -> Result<ProductResponse, AppError> {
        let product = Product::find_by_id(product_id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(ProductResponse::from(product))
    }

    pub async fn update_product(
        db: &DatabaseConnection,
        product_id: Uuid,
        request: UpdateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let product = Product::find_by_id(product_id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active_product: product::ActiveModel = product.into();

        if let Some(name) = request.name {
            active_product.name = Set(name);
        }
        if let Some(description) = request.description {
            active_product.description = Set(Some(description));
        }
        if let Some(price) = request.price {
            active_product.price = Set(price);
        }
        if let Some(quantity) = request.quantity {
            active_product.quantity = Set(quantity);
        }
        if let Some(category) = request.category {
            active_product.category = Set(Some(category));
        }
        active_product.updated_at = Set(chrono::Utc::now());

        let updated_product = active_product.update(db).await?;
        Ok(ProductResponse::from(updated_product))
    }

    pub async fn delete_product(
        db: &DatabaseConnection,
        product_id: Uuid,
    ) -> Result<(), AppError> {
        let result = Product::delete_by_id(product_id).exec(db).await?;
        
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}

impl From<product::Model> for ProductResponse {
    fn from(product: product::Model) -> Self {
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