use crate::{
    entities::{prelude::*, product},
    error::AppError,
    models::{
        CategoryStats, CreateProductRequest, ProductSearchRequest, ProductStatsResponse,
        UpdateProductRequest,
    },
};
use async_trait::async_trait;
use rust_decimal::Decimal;
use sea_orm::{
    prelude::*, ActiveModelTrait, DatabaseBackend, FromQueryResult,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, Statement,
};
use uuid::Uuid;

//  **Custom Result Mapping** - FromQueryResult derive macro
// These structs map raw SQL query results to Rust types
#[derive(FromQueryResult)]
struct ProductCount {}

#[derive(FromQueryResult)]
struct CategoryStatsRaw {
    category: Option<String>,
    count: i64,
    total_value: Decimal,
}

#[derive(FromQueryResult)]
struct ProductStatsRaw {
    total_products: i64,
    total_value: Decimal,
    avg_price: Option<Decimal>,
}

#[async_trait]
pub trait ProductRepositoryTrait {
    async fn create(&self, request: CreateProductRequest) -> Result<product::Model, AppError>;
    async fn find_all(
        &self,
        page: Option<u64>,
        per_page: Option<u64>,
    ) -> Result<(Vec<product::Model>, u64), AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<product::Model>, AppError>;
    async fn update(
        &self,
        id: Uuid,
        request: UpdateProductRequest,
    ) -> Result<product::Model, AppError>;
    async fn delete(&self, id: Uuid) -> Result<bool, AppError>;
    // Custom search queries demonstrating advanced SeaORM features
    async fn search(
        &self,
        search_request: ProductSearchRequest,
    ) -> Result<(Vec<product::Model>, u64), AppError>;
    async fn find_by_category(&self, category: &str) -> Result<Vec<product::Model>, AppError>;
    async fn find_by_price_range(
        &self,
        min_price: Decimal,
        max_price: Decimal,
    ) -> Result<Vec<product::Model>, AppError>;
    async fn find_low_stock(&self, threshold: i32) -> Result<Vec<product::Model>, AppError>;
    async fn get_product_stats(&self) -> Result<ProductStatsResponse, AppError>;
    async fn find_similar_products(
        &self,
        product_name: &str,
        limit: u64,
    ) -> Result<Vec<product::Model>, AppError>;
    async fn get_trending_categories(&self, limit: u64) -> Result<Vec<CategoryStats>, AppError>;
}

#[derive(Clone)]
pub struct ProductRepository {
    db: DatabaseConnection,
}

impl ProductRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProductRepositoryTrait for ProductRepository {
    async fn create(&self, request: CreateProductRequest) -> Result<product::Model, AppError> {
        let product_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let new_product = product::ActiveModel {
            id: Set(product_id),
            name: Set(request.name),
            description: Set(request.description),
            price: Set(request.price),
            quantity: Set(request.quantity),
            category: Set(request.category),
            created_by: Set(request.created_by),
            updated_by: Set(request.updated_by),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let product = new_product.insert(&self.db).await?;
        Ok(product)
    }

    //  **Pagination & Sorting** - Basic pagination with sorting
    async fn find_all(
        &self,
        page: Option<u64>,
        per_page: Option<u64>,
    ) -> Result<(Vec<product::Model>, u64), AppError> {
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(10).min(100);

        // SeaORM Pagination: Creates a paginator that handles LIMIT/OFFSET automatically
        let paginator = Product::find()
            .order_by_desc(product::Column::CreatedAt) // Simple sorting by creation date
            .paginate(&self.db, per_page);

        // Get total count for pagination metadata
        let total = paginator.num_items().await?;
        // Fetch specific page (0-indexed internally)
        let products = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((products, total))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<product::Model>, AppError> {
        let product = Product::find_by_id(id).one(&self.db).await?;
        Ok(product)
    }

    async fn update(
        &self,
        id: Uuid,
        request: UpdateProductRequest,
    ) -> Result<product::Model, AppError> {
        let product = Product::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound {
                resource_type: "Product".to_string(),
                resource_id: Some(id.to_string()),
                error_id: uuid::Uuid::new_v4(),
            })?;

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

        let updated_product = active_product.update(&self.db).await?;
        Ok(updated_product)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = Product::delete_by_id(id).exec(&self.db).await?;
        Ok(result.rows_affected > 0)
    }

    //  **Dynamic Query Building** + **Complex WHERE Clauses** + **Range Queries** + **Text Search** + **Pagination & Sorting**
    // This method demonstrates the most advanced SeaORM query building features
    async fn search(
        &self,
        search_request: ProductSearchRequest,
    ) -> Result<(Vec<product::Model>, u64), AppError> {
        let page = search_request.page.unwrap_or(1);
        let per_page = search_request.per_page.unwrap_or(10).min(100);

        //  **Dynamic Query Building** - Start with base query and conditionally add filters
        let mut query = Product::find();

        //  **Text Search** - LIKE/ILIKE pattern matching with OR conditions
        //  **Complex WHERE Clauses** - Multiple conditions with AND/OR logic
        if let Some(search_text) = &search_request.query {
            let search_pattern = format!("%{search_text}%");
            query = query.filter(
                // OR condition: search in both name AND description
                product::Column::Name
                    .contains(&search_pattern)
                    .or(product::Column::Description.contains(&search_pattern)),
            );
        }

        //  **Dynamic Query Building** - Conditional filters based on request parameters
        if let Some(name) = &search_request.name {
            query = query.filter(product::Column::Name.eq(name));
        }

        if let Some(category) = &search_request.category {
            query = query.filter(product::Column::Category.eq(category));
        }

        //  **Range Queries** - Greater than/equal, less than/equal operations
        if let Some(min_price) = search_request.min_price {
            query = query.filter(product::Column::Price.gte(min_price));
        }
        if let Some(max_price) = search_request.max_price {
            query = query.filter(product::Column::Price.lte(max_price));
        }

        //  **Range Queries** - Quantity range filtering
        if let Some(min_quantity) = search_request.min_quantity {
            query = query.filter(product::Column::Quantity.gte(min_quantity));
        }
        if let Some(max_quantity) = search_request.max_quantity {
            query = query.filter(product::Column::Quantity.lte(max_quantity));
        }

        //  **Complex WHERE Clauses** - Conditional logic for stock status
        if let Some(in_stock) = search_request.in_stock {
            if in_stock {
                query = query.filter(product::Column::Quantity.gt(0)); // In stock
            } else {
                query = query.filter(product::Column::Quantity.eq(0)); // Out of stock
            }
        }

        //  **Pagination & Sorting** - Dynamic sorting based on user input
        query = match (
            search_request.sort_by.as_deref(),
            search_request.sort_order.as_deref(),
        ) {
            (Some("name"), Some("desc")) => query.order_by_desc(product::Column::Name),
            (Some("name"), _) => query.order_by_asc(product::Column::Name),
            (Some("price"), Some("desc")) => query.order_by_desc(product::Column::Price),
            (Some("price"), _) => query.order_by_asc(product::Column::Price),
            (Some("quantity"), Some("desc")) => query.order_by_desc(product::Column::Quantity),
            (Some("quantity"), _) => query.order_by_asc(product::Column::Quantity),
            (Some("created_at"), Some("asc")) => query.order_by_asc(product::Column::CreatedAt),
            _ => query.order_by_desc(product::Column::CreatedAt), // Default sorting
        };

        //  **Pagination & Sorting** - Apply pagination to the dynamically built query
        let paginator = query.paginate(&self.db, per_page);
        let total = paginator.num_items().await?;
        let products = paginator.fetch_page(page.saturating_sub(1)).await?;

        Ok((products, total))
    }

    //  **Complex WHERE Clauses** - Simple category filter with sorting
    async fn find_by_category(&self, category: &str) -> Result<Vec<product::Model>, AppError> {
        let products = Product::find()
            .filter(product::Column::Category.eq(category))
            .order_by_desc(product::Column::CreatedAt)
            .all(&self.db)
            .await?;

        Ok(products)
    }

    //  **Range Queries** - BETWEEN operation for price range
    async fn find_by_price_range(
        &self,
        min_price: Decimal,
        max_price: Decimal,
    ) -> Result<Vec<product::Model>, AppError> {
        let products = Product::find()
            .filter(product::Column::Price.between(min_price, max_price)) // BETWEEN clause
            .order_by_asc(product::Column::Price) // Sort by price ascending
            .all(&self.db)
            .await?;

        Ok(products)
    }

    //  **Range Queries** + **Complex WHERE Clauses** - Multiple conditions with AND logic
    async fn find_low_stock(&self, threshold: i32) -> Result<Vec<product::Model>, AppError> {
        let products = Product::find()
            .filter(product::Column::Quantity.lte(threshold)) // Less than or equal to threshold
            .filter(product::Column::Quantity.gt(0)) // AND greater than 0 (exclude out of stock)
            .order_by_asc(product::Column::Quantity) // Sort by quantity ascending
            .all(&self.db)
            .await?;

        Ok(products)
    }

    //  **Raw SQL Integration** + **Aggregations** + **Custom Result Mapping**
    // Complex analytics using raw SQL for operations not easily expressed in SeaORM query builder
    async fn get_product_stats(&self) -> Result<ProductStatsResponse, AppError> {
        //  **Raw SQL Integration** - Custom SQL for complex aggregations
        //  **Aggregations** - COUNT, SUM, AVG operations
        let stats_query = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
            SELECT 
                COUNT(*) as total_products,                    -- COUNT aggregation
                COALESCE(SUM(price * quantity), 0) as total_value,  -- SUM with calculation
                AVG(price) as avg_price                        -- AVG aggregation
            FROM products
            "#,
            [], // No parameters for this query
        );

        //  **Custom Result Mapping** - Map raw SQL results to custom struct
        let stats_result: ProductStatsRaw = ProductStatsRaw::find_by_statement(stats_query)
            .one(&self.db)
            .await?
            .ok_or(AppError::InternalServerError {
                context: Some("Database returned None".to_string()), // or None
                error_id: uuid::Uuid::new_v4(),
            })?;

        //  **Raw SQL Integration** + **Aggregations** - Category-wise statistics
        let category_stats_query = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
            SELECT 
                category,
                COUNT(*) as count,                             -- COUNT by category
                COALESCE(SUM(price * quantity), 0) as total_value    -- SUM by category
            FROM products 
            WHERE category IS NOT NULL
            GROUP BY category                                  -- GROUP BY for aggregation
            ORDER BY count DESC                                -- Sort by count
            "#,
            [],
        );

        //  **Custom Result Mapping** - Map aggregated results to struct
        let category_results: Vec<CategoryStatsRaw> =
            CategoryStatsRaw::find_by_statement(category_stats_query)
                .all(&self.db)
                .await?;

        // Transform raw results into response model
        let categories = category_results
            .into_iter()
            .filter_map(|stats| {
                stats.category.map(|cat| CategoryStats {
                    category: cat,
                    count: stats.count as u64,
                    total_value: stats.total_value,
                })
            })
            .collect();

        Ok(ProductStatsResponse {
            total_products: stats_result.total_products as u64,
            total_value: stats_result.total_value,
            avg_price: stats_result.avg_price,
            categories,
        })
    }

    //  **Text Search** - Fuzzy text search using LIKE pattern matching
    async fn find_similar_products(
        &self,
        product_name: &str,
        limit: u64,
    ) -> Result<Vec<product::Model>, AppError> {
        let search_pattern = format!("%{product_name}%"); // Create LIKE pattern
        let products = Product::find()
            .filter(product::Column::Name.contains(&search_pattern)) // LIKE/ILIKE operation
            .limit(limit) // Limit results
            .order_by_desc(product::Column::CreatedAt)
            .all(&self.db)
            .await?;

        Ok(products)
    }

    //  **Raw SQL Integration** + **Subqueries** + **Aggregations** + **Custom Result Mapping**
    // Advanced query with date filtering, grouping, and complex conditions
    async fn get_trending_categories(&self, limit: u64) -> Result<Vec<CategoryStats>, AppError> {
        //  **Raw SQL Integration** + **Subqueries** - Complex query with date filtering
        let trending_query = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
            SELECT 
                category,
                COUNT(*) as count,                             -- COUNT aggregation
                SUM(price * quantity) as total_value           -- SUM aggregation with calculation
            FROM products 
            WHERE category IS NOT NULL                         -- Filter null categories
                AND created_at >= NOW() - INTERVAL '30 days'  --  **Subqueries** - Date filtering (last 30 days)
            GROUP BY category                                  -- GROUP BY for category-wise stats
            HAVING COUNT(*) > 0                                -- HAVING clause for post-aggregation filtering
            ORDER BY count DESC, total_value DESC             -- Multiple column sorting
            LIMIT $1                                           -- Dynamic limit parameter
            "#,
            [limit.into()], // Parameterized query
        );

        //  **Custom Result Mapping** - Map complex query results
        let results: Vec<CategoryStatsRaw> = CategoryStatsRaw::find_by_statement(trending_query)
            .all(&self.db)
            .await?;

        let categories = results
            .into_iter()
            .filter_map(|stats| {
                stats.category.map(|cat| CategoryStats {
                    category: cat,
                    count: stats.count as u64,
                    total_value: stats.total_value,
                })
            })
            .collect();
        Ok(categories)
    }
}
