# Product API - Rust Axum Web Service

A production-ready REST API built with Rust, Axum 0.7, SeaORM, and PostgreSQL, featuring JWT authentication, CRUD operations for products, and comprehensive error handling.

## Features

- **REST API** with Axum 0.7 web framework
- **PostgreSQL** database with SeaORM
- **JWT Authentication** with middleware protection
- **Product CRUD operations** (Create, Read, Update, Delete)
- **User authentication** (Register, Login)
- **Comprehensive error handling**
- **Docker containerization**
- **Request validation**
- **Structured logging**
- **Pagination support**

## Project Structure

```
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

## Configuration

Environment variables can be set in `.env` file:

- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT tokens
- `JWT_EXPIRATION`: Token expiration time in seconds
- `RUST_LOG`: Logging level

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
- **CORS**: Cross-origin resource sharing configuration
- **Error Handling**: Comprehensive error handling with proper HTTP status codes

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