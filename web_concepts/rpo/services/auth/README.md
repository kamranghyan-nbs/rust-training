# POS Auth Service - Complete Guide

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/yourorg/retail-pos)
[![Docker](https://img.shields.io/badge/docker-ready-blue)](https://hub.docker.com)
[![API](https://img.shields.io/badge/API-REST-orange)](http://localhost:8080/swagger-ui/)

A production-ready authentication and authorization microservice built with Rust, Axum, and PostgreSQL. Features JWT-based authentication, flexible RBAC, multi-tenancy, and comprehensive API documentation.

## 🚀 Quick Start

### Prerequisites
- Docker & Docker Compose
- Rust 1.78+ (for local development)
- curl or Postman (for testing)

### Start the Service
```bash
# Clone the repository
git clone <your-repo-url>
cd retail-pos

# Start all services
docker-compose up --build

# Wait for services to be ready (~30 seconds)
# Auth service will be available at: http://localhost:8080
# Swagger UI: http://localhost:8080/swagger-ui/
```

### Verify Installation
```bash
# Health check
curl http://localhost:8080/health

# Expected response: "OK"
```

## 📋 Table of Contents
- [Architecture Overview](#-architecture-overview)
- [API Documentation](#-api-documentation)
- [User Management](#-user-management)
- [Role & Permission System](#-role--permission-system)
- [Testing Examples](#-testing-examples)
- [Troubleshooting](#-troubleshooting)
- [Development](#-development)

## 🏗️ Architecture Overview

### Database Schema
The service creates the following tables:
- `tenants` - Multi-tenant organization data
- `users` - User accounts with tenant isolation
- `roles` - Flexible role definitions per tenant
- `permissions` - System-wide permission definitions
- `user_roles` - Many-to-many user-role assignments
- `role_permissions` - Many-to-many role-permission assignments
- `sessions` - JWT refresh token management

### Key Features
- ✅ **Multi-tenant Architecture** - Complete tenant isolation
- ✅ **JWT Authentication** - Secure token-based auth
- ✅ **Flexible RBAC** - Role-based access control with custom permissions
- ✅ **Session Management** - Refresh token handling
- ✅ **Account Security** - Failed login tracking and account locking
- ✅ **Production Ready** - Docker, health checks, structured logging
- ✅ **API Documentation** - Auto-generated Swagger UI

## 📚 API Documentation

### Base URL
```
http://localhost:8080
```

### Authentication Header
```bash
Authorization: Bearer <access_token>
```

## 🔐 Authentication Endpoints

### 1. Login
```bash
POST /auth/login
Content-Type: application/json

{
  "email": "admin@demo.com",
  "password": "admin123",
  "tenant_code": "demo"
}
```

**Response:**
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "admin@demo.com",
    "username": "admin",
    "first_name": "System",
    "last_name": "Administrator",
    "roles": ["admin"],
    "permissions": ["users.create", "users.read", "users.update", "users.delete", "..."],
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001"
  }
}
```

### 2. Refresh Token
```bash
POST /auth/refresh
Content-Type: application/json

{
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

### 3. Validate Token (Internal Service Use)
```bash
POST /auth/validate
Content-Type: application/json

"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```

### 4. Logout
```bash
POST /auth/logout
Authorization: Bearer <token>
Content-Type: application/json

{
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

### 5. Change Password
```bash
PUT /auth/change-password
Authorization: Bearer <token>
Content-Type: application/json

{
  "current_password": "oldpassword123",
  "new_password": "newpassword456"
}
```

## 👥 User Management

### 1. Create User
```bash
POST /users
Authorization: Bearer <admin_token>
Content-Type: application/json

{
  "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
  "email": "manager@demo.com",
  "username": "manager",
  "password": "manager123",
  "first_name": "Store",
  "last_name": "Manager",
  "phone": "+1234567890",
  "role_ids": ["550e8400-e29b-41d4-a716-446655440010"]
}
```

### 2. Get User
```bash
GET /users/{user_id}
Authorization: Bearer <token>
```

### 3. Update User
```bash
PUT /users/{user_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "first_name": "Updated Name",
  "is_active": true,
  "role_ids": ["550e8400-e29b-41d4-a716-446655440010", "550e8400-e29b-41d4-a716-446655440011"]
}
```

### 4. List Users
```bash
GET /users?page=0&page_size=20
Authorization: Bearer <token>
```

### 5. Delete User
```bash
DELETE /users/{user_id}
Authorization: Bearer <admin_token>
```

## 🏢 Tenant Management

### 1. Create Tenant
```bash
POST /tenants
Authorization: Bearer <super_admin_token>
Content-Type: application/json

{
  "name": "My Restaurant",
  "code": "my_restaurant",
  "domain": "myrestaurant.com",
  "settings": {
    "currency": "USD",
    "timezone": "America/New_York",
    "theme": "dark"
  }
}
```

### 2. Get Tenant
```bash
GET /tenants/{tenant_id}
Authorization: Bearer <token>
```

### 3. List Tenants
```bash
GET /tenants?page=0&page_size=20
Authorization: Bearer <super_admin_token>
```

## 🛡️ Role & Permission System

### Default Permissions Available
```json
[
  "users.create", "users.read", "users.update", "users.delete",
  "roles.create", "roles.read", "roles.update", "roles.delete",
  "products.create", "products.read", "products.update", "products.delete",
  "orders.create", "orders.read", "orders.update", "orders.delete",
  "inventory.read", "inventory.update",
  "reports.sales", "reports.inventory", "reports.users",
  "system.settings", "system.logs"
]
```

### Pre-seeded Test Data

After running `docker-compose up`, you'll have:

**Tenant:**
- Name: "Demo Store"
- Code: "demo"

**Users:**
- **Admin**: admin@demo.com / admin123 (full access)
- **Manager**: manager@demo.com / manager123 (limited access)

## 🧪 Testing Examples

### Complete User Creation Flow

#### 1. First, Login as Admin
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@demo.com",
    "password": "admin123",
    "tenant_code": "demo"
  }'
```

#### 2. Create a Custom Role (Store Cashier)
```bash
# First get available permissions
curl -X GET http://localhost:8080/permissions \
  -H "Authorization: Bearer <admin_token>"

# Create cashier role with limited permissions
curl -X POST http://localhost:8080/roles \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "Cashier",
    "code": "cashier",
    "description": "Point of sale operations only",
    "permission_ids": [
      "products.read",
      "orders.create", 
      "orders.read",
      "inventory.read"
    ]
  }'
```

#### 3. Create Different Types of Users

**Super Admin User:**
```bash
curl -X POST http://localhost:8080/users \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "email": "superadmin@demo.com",
    "username": "superadmin",
    "password": "superadmin123",
    "first_name": "Super",
    "last_name": "Administrator",
    "phone": "+1234567891",
    "role_ids": ["<admin_role_id>"]
  }'
```

**Store Manager User:**
```bash
curl -X POST http://localhost:8080/users \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "email": "storemanager@demo.com",
    "username": "storemanager",
    "password": "manager123",
    "first_name": "Store",
    "last_name": "Manager",
    "phone": "+1234567892",
    "role_ids": ["<manager_role_id>"]
  }'
```

**Cashier User:**
```bash
curl -X POST http://localhost:8080/users \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "email": "cashier1@demo.com",
    "username": "cashier1",
    "password": "cashier123",
    "first_name": "John",
    "last_name": "Cashier",
    "phone": "+1234567893",
    "role_ids": ["<cashier_role_id>"]
  }'
```

**Multi-Role User (Manager + Cashier):**
```bash
curl -X POST http://localhost:8080/users \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "tenant_id": "550e8400-e29b-41d4-a716-446655440001",
    "email": "supervisor@demo.com",
    "username": "supervisor",
    "password": "supervisor123",
    "first_name": "Jane",
    "last_name": "Supervisor",
    "phone": "+1234567894",
    "role_ids": ["<manager_role_id>", "<cashier_role_id>"]
  }'
```

#### 4. Test Different User Access Levels

**Test Cashier Access (Limited):**
```bash
# Login as cashier
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "cashier1@demo.com",
    "password": "cashier123",
    "tenant_code": "demo"
  }'

# Try to create user (should fail - no permission)
curl -X POST http://localhost:8080/users \
  -H "Authorization: Bearer <cashier_token>" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@demo.com", ...}'
  
# Expected: 403 Forbidden
```

**Test Manager Access:**
```bash
# Login as manager
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "storemanager@demo.com",
    "password": "manager123",
    "tenant_code": "demo"
  }'

# List users (should work)
curl -X GET http://localhost:8080/users \
  -H "Authorization: Bearer <manager_token>"
```

### Database Verification

#### Connect to PostgreSQL
```bash
# Connect to database
docker-compose exec postgres psql -U pos_user -d pos_auth

# Check all tables
\dt

# View tenants
SELECT id, name, code, is_active FROM tenants;

# View users
SELECT id, email, username, first_name, last_name, is_active FROM users;

# View roles
SELECT r.name, r.code, r.description 
FROM roles r 
JOIN tenants t ON r.tenant_id = t.id 
WHERE t.code = 'demo';

# View user roles
SELECT u.email, r.name as role_name 
FROM users u 
JOIN user_roles ur ON u.id = ur.user_id 
JOIN roles r ON ur.role_id = r.id;

# View role permissions
SELECT r.name as role_name, p.code as permission 
FROM roles r 
JOIN role_permissions rp ON r.id = rp.role_id 
JOIN permissions p ON rp.permission_id = p.id 
ORDER BY r.name, p.code;
```

## 🛠️ Development

### Local Development Setup
```bash
# Start dependencies only
docker-compose up -d postgres redis

# Run migrations
cargo run --package auth-service --bin migrator up

# Start auth service
cargo run --package auth-service

# Run tests
cargo test --package auth-service
```

### Environment Variables
```bash
# Database
AUTH_DATABASE__URL=postgres://pos_user:pos_password@localhost:5432/pos_auth
AUTH_DATABASE__MAX_CONNECTIONS=10

# JWT
AUTH_JWT__SECRET=your-super-secret-jwt-key-change-in-production
AUTH_JWT__ACCESS_TOKEN_EXPIRE_MINUTES=60
AUTH_JWT__REFRESH_TOKEN_EXPIRE_DAYS=7
AUTH_JWT__ISSUER=pos-system

# Server
AUTH_SERVER__HOST=0.0.0.0
AUTH_SERVER__PORT=8080

# Logging
AUTH_LOGGING__LEVEL=info
AUTH_LOGGING__FORMAT=json
```

### Adding New Permissions
```sql
-- Connect to database and add custom permissions
INSERT INTO permissions (id, name, code, resource, action, description, is_system, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    'Inventory - Adjust Stock',
    'inventory.adjust',
    'inventory',
    'adjust',
    'Adjust inventory stock levels',
    false,
    now(),
    now()
);
```

### Creating Custom Roles
```bash
# Create a custom role via API
curl -X POST http://localhost:8080/roles \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "tenant_id": "your-tenant-id",
    "name": "Inventory Manager",
    "code": "inventory_manager",
    "description": "Manages inventory operations",
    "permission_ids": [
      "inventory.read",
      "inventory.update",
      "inventory.adjust",
      "products.read",
      "reports.inventory"
    ]
  }'
```

## 🚨 Troubleshooting

### Common Issues

#### 1. Migration Fails
```bash
# Check migration status
docker-compose run --rm auth-service ./migrator status

# Re-run migrations
docker-compose run --rm auth-service ./migrator up

# Check database connection
docker-compose exec postgres psql -U pos_user -d pos_auth -c "SELECT version();"
```

#### 2. Authentication Fails
```bash
# Check if user exists
docker-compose exec postgres psql -U pos_user -d pos_auth -c "SELECT email, is_active FROM users WHERE email='admin@demo.com';"

# Check tenant code
docker-compose exec postgres psql -U pos_user -d pos_auth -c "SELECT code, name FROM tenants WHERE code='demo';"
```

#### 3. Permission Denied
```bash
# Check user permissions
docker-compose exec postgres psql -U pos_user -d pos_auth -c "
SELECT u.email, p.code 
FROM users u 
JOIN user_roles ur ON u.id = ur.user_id 
JOIN role_permissions rp ON ur.role_id = rp.role_id 
JOIN permissions p ON rp.permission_id = p.id 
WHERE u.email = 'your-user@demo.com';
"
```

#### 4. Service Won't Start
```bash
# Check logs
docker-compose logs auth-service

# Check health
curl http://localhost:8080/health

# Restart service
docker-compose restart auth-service
```

### Logs and Monitoring
```bash
# View logs
docker-compose logs -f auth-service

# Check health endpoint
curl http://localhost:8080/health

# View database logs
docker-compose logs postgres
```

## 📋 API Reference Summary

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/auth/login` | User login | No |
| POST | `/auth/refresh` | Refresh token | No |
| POST | `/auth/validate` | Validate token | No |
| POST | `/auth/logout` | User logout | Yes |
| PUT | `/auth/change-password` | Change password | Yes |
| POST | `/users` | Create user | Yes (admin) |
| GET | `/users` | List users | Yes |
| GET | `/users/{id}` | Get user | Yes |
| PUT | `/users/{id}` | Update user | Yes |
| DELETE | `/users/{id}` | Delete user | Yes (admin) |
| POST | `/tenants` | Create tenant | Yes (super admin) |
| GET | `/tenants` | List tenants | Yes |
| GET | `/tenants/{id}` | Get tenant | Yes |
| PUT | `/tenants/{id}` | Update tenant | Yes (admin) |
| GET | `/health` | Health check | No |

## 🔗 Related Services

This auth service is designed to integrate with:
- **Product Service** - Product catalog management
- **Order Service** - Order processing
- **Inventory Service** - Stock management
- **Customer Service** - Customer management
- **Payment Service** - Payment processing

All services use this auth service for:
- JWT token validation
- Permission checking
- User information retrieval

## 📄 License

MIT License - see LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

---

**🎉 Your POS Auth Service is ready!**

Visit http://localhost:8080/swagger-ui/ for interactive API documentation.