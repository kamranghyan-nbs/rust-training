# Product API - Rust Axum Web Service

A production-ready REST API built with Rust, Axum 0.7, SeaORM, and PostgreSQL, featuring JWT authentication, CRUD operations for products, and comprehensive error handling.

## Features

- **REST API** with Axum 0.7 web framework
- **PostgreSQL** database with SeaORM
- **JWT Authentication** with middleware protection
- **Product CRUD operations** (Create, Read, Update, Delete)
- **User authentication** (Register, Login)
- **Advanced Rate Limiting** (IP-based and User-based)
- **Comprehensive error handling**
- **High-Performance Async Logging** with structured data
- **Docker containerization**
- **Request validation**
- **Structured logging**
- **Pagination support**

## Project Structure

```

## Async Logging Examples

### **Development Mode** (Pretty Console Logging)
```bash
LOG_FORMAT=pretty LOG_OUTPUT=console RUST_LOG=debug cargo run
```

### **Production Mode** (JSON File + Console)
```bash
LOG_FORMAT=json LOG_OUTPUT=both LOG_DIR=./logs cargo run
```

### **High-Performance Mode** (File Only, Compact)
```bash
LOG_FORMAT=compact LOG_OUTPUT=file LOG_ROTATION=hourly cargo run
```

### **Debug Mode** (With Tokio Console)
```bash
ENABLE_TOKIO_CONSOLE=true RUST_LOG=debug cargo run
# In another terminal: tokio-console http://localhost:6669
```

### **Distributed Tracing** (With Jaeger)
```bash
# Start Jaeger (Docker)
docker run -d -p 16686:16686 -p 14268:14268 jaegertracing/all-in-one:latest

# Run app with tracing
ENABLE_DISTRIBUTED_TRACING=true \
JAEGER_AGENT_ENDPOINT=http://localhost:14268/api/traces \
cargo run

# View traces: http://localhost:16686
```

### **View Real-time Logs**
```bash
# Follow application logs
tail -f logs/product-api.2025-08-21

# Parse JSON logs with jq
tail -f logs/product-api.2025-08-21 | jq '.fields'

# Filter by log level
tail -f logs/product-api.2025-08-21 | jq 'select(.level == "ERROR")'

# Monitor performance
tail -f logs/product-api.2025-08-21 | jq 'select(.fields.duration_ms > 100)'
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── error.rs             # Error handling
├── entities/            # SeaORM entities
│   ├── mod.rs
│   ├── prelude.rs
│   ├── user.rs
│   └── product.rs
├── models/              # Request/Response DTOs
│   ├── mod.rs
│   ├── auth.rs
│   └── product.rs
├── services/            # Business logic
│   ├── mod.rs
│   ├── auth.rs
│   └── product.rs
├── handlers/            # HTTP handlers
│   ├── mod.rs
│   ├── auth.rs
│   └── product.rs
├── middleware/          # Custom middleware
│   ├── mod.rs
│   └── auth.rs
└── utils/               # Utility functions
    ├── mod.rs
    ├── jwt.rs
    └── password.rs
```

## Getting Started

### Prerequisites

- Docker and Docker Compose
- Rust 1.75+ (if running locally)

### Running with Docker

1. **Clone the repository and navigate to the project directory**

2. **Create environment file:**
   ```bash
   cp .env.example .env
   ```

3. **Start the services:**
   ```bash
   docker-compose up --build
   ```

4. **The API will be available at:**
   - Web service: http://localhost:8080
   - PostgreSQL: localhost:5432

### Running Locally

1. **Set up PostgreSQL database**

2. **Install dependencies:**
   ```bash
   cargo build
   ```

3. **Run the application:**
   ```bash
   cargo run
   ```

## API Endpoints

### Authentication

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/auth/register` | Register new user | No |
| POST | `/auth/login` | Login user | No |

### Products

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/products` | Create product | Yes |
| GET | `/products` | Get all products (paginated) | Yes |
| GET | `/products/{id}` | Get product by ID | Yes |
| PUT | `/products/{id}` | Update product | Yes |
| DELETE | `/products/{id}` | Delete product | Yes |

### Product Search & Analytics

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/products/search` | Advanced product search with filters | Yes |
| GET | `/products/category` | Get products by category | Yes |
| GET | `/products/price-range` | Get products by price range | Yes |
| GET | `/products/low-stock` | Get low stock products | Yes |
| GET | `/products/similar` | Get similar products by name | Yes |
| GET | `/products/stats` | Get product statistics | Yes |
| GET | `/products/trending-categories` | Get trending categories | Yes |

### Health Check

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/health` | Health check | No |

## API Usage Examples

### Register User
```bash
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }'
```

### Login
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }'
```

### Create Product (with JWT token)
```bash
curl -X POST http://localhost:8080/products \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "name": "New Product",
    "description": "Product description",
    "price": "99.99",
    "quantity": 10,
    "category": "Electronics"
  }'
```

### Get All Products
```bash
curl -X GET "http://localhost:8080/products?page=1&per_page=10" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Get Product by ID
```bash
curl -X GET http://localhost:8080/products/{product_id} \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Update Product
```bash
curl -X PUT http://localhost:8080/products/{product_id} \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "name": "Updated Product Name",
    "price": "129.99"
  }'
```

### Delete Product
```bash
curl -X DELETE http://localhost:8080/products/{product_id} \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Advanced Search Examples

### Search Products with Multiple Filters
```bash
curl -X GET "http://localhost:8080/products/search?query=laptop&category=Electronics&min_price=500&max_price=2000&in_stock=true&sort_by=price&sort_order=asc&page=1&per_page=10" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Get Products by Category
```bash
curl -X GET "http://localhost:8080/products/category?category=Electronics" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Get Products by Price Range
```bash
curl -X GET "http://localhost:8080/products/price-range?min_price=100&max_price=500" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Get Low Stock Products
```bash
curl -X GET "http://localhost:8080/products/low-stock?threshold=5" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Get Similar Products
```bash
curl -X GET "http://localhost:8080/products/similar?name=laptop&limit=5" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Get Product Statistics
```bash
curl -X GET "http://localhost:8080/products/stats" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Get Trending Categories
```bash
curl -X GET "http://localhost:8080/products/trending-categories?limit=5" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## High-Performance Async Logging

The API implements a sophisticated async logging system with multiple output targets and formats:

### **🚀 Async Logging Features**

#### **Non-Blocking Writers**
- **Async File Appenders**: Uses `tracing-appender` for non-blocking file I/O
- **Buffered Output**: Prevents logging from blocking request processing
- **Worker Threads**: Dedicated threads for log processing

#### **Multiple Output Formats**
```rust
// Pretty format for development
LOG_FORMAT=pretty

// JSON format for production/log aggregation  
LOG_FORMAT=json

// Compact format for resource-constrained environments
LOG_FORMAT=compact
```

#### **Flexible Output Targets**
```bash
LOG_OUTPUT=console    # Console only
LOG_OUTPUT=file      # File only  
LOG_OUTPUT=both      # Console + File
```

#### **File Rotation Options**
```bash
LOG_ROTATION=hourly   # New file every hour
LOG_ROTATION=daily    # New file every day (default)
LOG_ROTATION=never    # Single log file
```

### **📊 Structured Logging**

All logs include structured data for easy parsing and analysis:

```json
{
  "timestamp": "2025-08-21T10:30:00Z",
  "level": "INFO", 
  "fields": {
    "method": "POST",
    "path": "/products",
    "status": 201,
    "duration_ms": 45,
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "user_id": "123e4567-e89b-12d3-a456-426614174000"
  },
  "message": "HTTP request completed successfully"
}
```

### **🔍 Request Tracing**

Every HTTP request gets:
- **Unique Request ID**: For correlation across services
- **Performance Metrics**: Response time, status codes
- **User Context**: Authentication info when available
- **Error Details**: Structured error information

### **⚡ Performance Monitoring**

Built-in performance measurement:
```rust
let timer = PerformanceTimer::new("database_query".to_string());
// ... perform operation
timer.finish(); // Automatically logs duration
```

### **🌐 Distributed Tracing** (Optional)

Enable OpenTelemetry + Jaeger for microservices:
```bash
ENABLE_DISTRIBUTED_TRACING=true
JAEGER_AGENT_ENDPOINT=http://localhost:14268/api/traces
```

### **🔔 Error Reporting** (Optional)

Integrate with Sentry for error monitoring:
```bash
SENTRY_DSN=https://your-sentry-dsn@sentry.io/project-id
ENVIRONMENT=production
```

### **🛠️ Development Tools**

#### **Tokio Console** (Development)
Monitor async tasks and performance:
```bash
ENABLE_TOKIO_CONSOLE=true
# Then run: tokio-console http://localhost:6669
```

### **📈 Log Levels & Filtering**

Granular control over log verbosity:
```bash
# Application logs at debug, dependencies at info
RUST_LOG=debug,product_api=debug,tower_http=info,sqlx=warn

# Production: errors and warnings only
RUST_LOG=warn,product_api=info
```

### **💾 Log Storage**

#### **Local Development**
```bash
LOG_DIR=./logs                    # Local directory
LOG_FILE_PREFIX=product-api       # File naming
```

#### **Production/Docker**
```bash
LOG_DIR=/app/logs                 # Container path
# Files: /app/logs/product-api.2025-08-21
```

### **🚀 Performance Benefits**

**Sync vs Async Logging Comparison:**

| Feature | Sync Logging | Async Logging |
|---------|-------------|---------------|
| **Request Latency** | +5-50ms per request | +0.1-1ms per request |
| **Throughput** | Reduced by 10-30% | Minimal impact (<2%) |
| **I/O Blocking** | Blocks request thread | Non-blocking |
| **Structured Data** | Limited | Full support |
| **Multiple Outputs** | Complex | Native support |

## Rate Limiting

The API implements sophisticated rate limiting with different limits for different types of requests:

### IP-Based Rate Limiting (Public Endpoints)
- Applied to public endpoints like `/auth/login`, `/auth/register`, `/health`
- Default: 100 requests per minute per IP address
- Configurable via `RATE_LIMIT_PER_IP` environment variable

### User-Based Rate Limiting (Protected Endpoints)  
- Applied to authenticated endpoints like `/products/*`
- Default: 200 requests per minute per authenticated user
- Configurable via `RATE_LIMIT_PER_USER` environment variable
- Falls back to IP-based limiting if user info is unavailable

### Rate Limit Headers
All responses include rate limiting information:
- `X-RateLimit-Limit`: Maximum requests allowed per window
- `X-RateLimit-Remaining`: Requests remaining in current window
- `X-RateLimit-Reset`: Window reset time in seconds

### Rate Limit Responses
When rate limited, the API returns:
- **Status Code**: `429 Too Many Requests`
- **Headers**: Rate limit information
- **Body**: Error message indicating rate limit exceeded

## Configuration

Environment variables can be set in `.env` file:

- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT tokens
- `JWT_EXPIRATION`: Token expiration time in seconds
- `RATE_LIMIT_PER_IP`: Max requests per minute per IP (default: 100)
- `RATE_LIMIT_PER_USER`: Max requests per minute per user (default: 200)

### **Async Logging Configuration**
- `RUST_LOG`: Log level and filtering (default: `info,product_api=debug`)
- `LOG_FORMAT`: Output format - `pretty`, `json`, `compact` (default: `pretty`)
- `LOG_OUTPUT`: Output target - `console`, `file`, `both` (default: `console`)
- `LOG_DIR`: Log file directory (default: `./logs`)
- `LOG_FILE_PREFIX`: Log file name prefix (default: `product-api`)
- `LOG_ROTATION`: File rotation - `hourly`, `daily`, `never` (default: `daily`)

### **Advanced Features**
- `ENABLE_TOKIO_CONSOLE`: Enable async task debugging (default: `false`)
- `ENABLE_DISTRIBUTED_TRACING`: Enable OpenTelemetry tracing (default: `false`)
- `JAEGER_AGENT_ENDPOINT`: Jaeger collector endpoint
- `SENTRY_DSN`: Sentry error reporting DSN
- `ENVIRONMENT`: Deployment environment (`development`, `staging`, `production`)

## Custom SeaORM Queries

The repository layer demonstrates various SeaORM query capabilities:

### 1. **Advanced Filtering**
```rust
// Multiple condition filtering with dynamic sorting
let mut query = Product::find();
query = query.filter(
    product::Column::Name.contains(&search_pattern)
        .or(product::Column::Description.contains(&search_pattern))
);
query = query.filter(product::Column::Price.between(min_price, max_price));
```

### 2. **Dynamic Query Building**
```rust
// Conditional filters based on request parameters
if let Some(category) = &search_request.category {
    query = query.filter(product::Column::Category.eq(category));
}
if let Some(min_price) = search_request.min_price {
    query = query.filter(product::Column::Price.gte(min_price));
}
```

### 3. **Complex Aggregations with Raw SQL**
```rust
// Custom SQL for statistics
let stats_query = Statement::from_sql_and_values(
    DatabaseBackend::Postgres,
    r#"
    SELECT 
        COUNT(*) as total_products,
        COALESCE(SUM(price * quantity), 0) as total_value,
        AVG(price) as avg_price
    FROM products
    "#,
    []
);
```

### 4. **Text Search**
```rust
// Case-insensitive text search using LIKE/ILIKE
let search_pattern = format!("%{}%", search_text);
query = query.filter(product::Column::Name.contains(&search_pattern));
```

### 5. **Range Queries**
```rust
// Price and quantity range filtering
query = query.filter(product::Column::Price.between(min_price, max_price));
query = query.filter(product::Column::Quantity.gte(min_quantity));
```

### 6. **Custom Structs for Results**
```rust
#[derive(FromQueryResult)]
struct CategoryStatsRaw {
    category: Option<String>,
    count: i64,
    total_value: Decimal,
}
```

### 7. **Pagination & Sorting**
```rust
// Dynamic sorting with pagination
query = match (sort_by, sort_order) {
    ("price", "desc") => query.order_by_desc(product::Column::Price),
    ("name", "asc") => query.order_by_asc(product::Column::Name),
    _ => query.order_by_desc(product::Column::CreatedAt),
};
let paginator = query.paginate(&self.db, per_page);
```

### 8. **Subqueries & Complex Joins**
```rust
// Trending categories with date filtering
SELECT category, COUNT(*) as count
FROM products 
WHERE category IS NOT NULL 
    AND created_at >= NOW() - INTERVAL '30 days'
GROUP BY category
ORDER BY count DESC
```

## Database Schema

### Users Table
- `id` (UUID, Primary Key)
- `username` (String, Unique)
- `email` (String, Unique)
- `password_hash` (String)
- `created_at` (Timestamp)
- `updated_at` (Timestamp)

### Products Table
- `id` (UUID, Primary Key)
- `name` (String)
- `description` (Text, Optional)
- `price` (Decimal)
- `quantity` (Integer)
- `category` (String, Optional)
- `created_at` (Timestamp)
- `updated_at` (Timestamp)

## Security Features

- **JWT Authentication**: Stateless authentication using JWT tokens
- **Password Hashing**: Secure password storage using bcrypt
- **Request Validation**: Input validation using the validator crate
- **Rate Limiting**: Advanced IP-based and user-based rate limiting
- **CORS**: Cross-origin resource sharing configuration
- **Error Handling**: Comprehensive error handling with proper HTTP status codes

### Running Tests

Run the test suite:
```bash
cargo test
```

Run specific rate limiting tests:
```bash
cargo test rate_limit
```

## Development

### Adding New Features

1. Add new entities in `src/entities/`
2. Create corresponding models in `src/models/`
3. Implement business logic in `src/services/`
4. Add HTTP handlers in `src/handlers/`
5. Update routes in `src/main.rs`

### Next Steps

The following features will be added in subsequent iterations:

1. **Logging**: Structured logging with tracing
2. **Unit Testing**: Comprehensive unit tests
3. **Integration Testing**: End-to-end API tests
4. **Metrics**: Prometheus metrics collection
5. **Rate Limiting**: API rate limiting middleware
6. **OpenAPI**: API documentation with OpenAPI/Swagger

## License

This project is licensed under the MIT License.