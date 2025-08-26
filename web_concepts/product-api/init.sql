-- Database initialization script
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create role enum type
CREATE TYPE user_role AS ENUM ('admin', 'manager', 'user');

-- Users table for authentication with RBAC
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(100) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role DEFAULT 'user' NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Products table
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10,2) NOT NULL,
    quantity INTEGER DEFAULT 0,
    category VARCHAR(100),
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insert default users with different roles for testing
-- Insert default users with different roles for testing
INSERT INTO users (username, email, password_hash, role) VALUES 
-- Password: admin123
('admin', 'admin@example.com', '$2b$12$dTe69b8Eo0s4zP9QjMqJ9uJBHd1lZ5DLVQeM.rusvslR1CexRtq.i', 'admin'),
-- Password: manager123
('manager', 'manager@example.com', '$2b$12$/Y9hO.1sU4Y2qIuC0lLgQeFvjXgMiqQ1r8mTX0w6jefqlIfU9Iie6', 'manager'),
-- Password: user123
('user', 'user@example.com', '$2b$12$Y1QbpJ7PPY81efmkkSi3ceBQZs6yArwAMF8wUBr5cIhMPaZ95B6u6', 'user')
ON CONFLICT (username) DO UPDATE SET 
    role = EXCLUDED.role,
    password_hash = EXCLUDED.password_hash;


-- Insert some sample products with creator tracking
INSERT INTO products (name, description, price, quantity, category, created_by) 
SELECT 
    'Laptop', 
    'High-performance laptop for developers', 
    1299.99, 
    10, 
    'Electronics',
    u.id
FROM users u WHERE u.username = 'admin'
UNION ALL
SELECT 
    'Coffee Mug', 
    'Ceramic coffee mug with company logo', 
    15.99, 
    50, 
    'Office Supplies',
    u.id
FROM users u WHERE u.username = 'manager'
UNION ALL
SELECT 
    'Wireless Mouse', 
    'Ergonomic wireless mouse', 
    49.99, 
    25, 
    'Electronics',
    u.id
FROM users u WHERE u.username = 'admin'
ON CONFLICT DO NOTHING;

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_active ON users(is_active);
CREATE INDEX IF NOT EXISTS idx_products_created_by ON products(created_by);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);