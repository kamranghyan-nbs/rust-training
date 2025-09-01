#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Seeding database with test data...${NC}"

# Database connection details
DB_HOST=${DB_HOST:-localhost}
DB_PORT=${DB_PORT:-5432}
DB_NAME=${DB_NAME:-pos_auth}
DB_USER=${DB_USER:-pos_user}
DB_PASSWORD=${DB_PASSWORD:-pos_password}

# Create a test tenant
echo -e "${BLUE}Creating test tenant...${NC}"
TENANT_ID=$(uuidgen)
psql "postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME" << EOF
INSERT INTO tenants (id, name, code, settings, is_active, created_at, updated_at) 
VALUES (
    '$TENANT_ID',
    'Demo Store',
    'demo',
    '{"theme": "default", "currency": "USD", "timezone": "UTC"}',
    true,
    now(),
    now()
);
EOF

# Create admin role
echo -e "${BLUE}Creating admin role...${NC}"
ADMIN_ROLE_ID=$(uuidgen)
psql "postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME" << EOF
INSERT INTO roles (id, tenant_id, name, code, description, is_system, is_active, created_at, updated_at)
VALUES (
    '$ADMIN_ROLE_ID',
    '$TENANT_ID',
    'Administrator',
    'admin',
    'Full system access',
    false,
    true,
    now(),
    now()
);
EOF

# Assign all permissions to admin role
echo -e "${BLUE}Assigning permissions to admin role...${NC}"
psql "postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME" << EOF
INSERT INTO role_permissions (id, role_id, permission_id, created_at)
SELECT gen_random_uuid(), '$ADMIN_ROLE_ID', id, now()
FROM permissions;
EOF

# Create manager role
echo -e "${BLUE}Creating manager role...${NC}"
MANAGER_ROLE_ID=$(uuidgen)
psql "postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME" << EOF
INSERT INTO roles (id, tenant_id, name, code, description, is_system, is_active, created_at, updated_at)
VALUES (
    '$MANAGER_ROLE_ID',
    '$TENANT_ID',
    'Manager',
    'manager',
    'Store management access',
    false,
    true,
    now(),
    now()
);
EOF

# Create test admin user
echo -e "${BLUE}Creating test admin user...${NC}"
ADMIN_USER_ID=$(uuidgen)
# Password hash for "admin123" (you should use your password hashing function)
ADMIN_HASH='$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdFfCl/A9kQH6u2'
psql "postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME" << EOF
INSERT INTO users (id, tenant_id, email, username, password_hash, first_name, last_name, is_active, is_verified, created_at, updated_at)
VALUES (
    '$ADMIN_USER_ID',
    '$TENANT_ID',
    'admin@demo.com',
    'admin',
    '$ADMIN_HASH',
    'System',
    'Administrator',
    true,
    true,
    now(),
    now()
);
EOF

# Assign admin role to admin user
echo -e "${BLUE}Assigning admin role to user...${NC}"
psql "postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME" << EOF
INSERT INTO user_roles (id, user_id, role_id, created_at)
VALUES (gen_random_uuid(), '$ADMIN_USER_ID', '$ADMIN_ROLE_ID', now());
EOF

# Create test manager user
echo -e "${BLUE}Creating test manager user...${NC}"
MANAGER_USER_ID=$(uuidgen)
# Password hash for "manager123"
MANAGER_HASH='$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdFfCl/A9kQH6u2'
psql "postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME" << EOF
INSERT INTO users (id, tenant_id, email, username, password_hash, first_name, last_name, is_active, is_verified, created_at, updated_at)
VALUES (
    '$MANAGER_USER_ID',
    '$TENANT_ID',
    'manager@demo.com',
    'manager',
    '$MANAGER_HASH',
    'Store',
    'Manager',
    true,
    true,
    now(),
    now()
);
EOF

# Assign manager role to manager user
echo -e "${BLUE}Assigning manager role to user...${NC}"
psql "postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME" << EOF
INSERT INTO user_roles (id, user_id, role_id, created_at)
VALUES (gen_random_uuid(), '$MANAGER_USER_ID', '$MANAGER_ROLE_ID', now());
EOF

echo -e "${GREEN}Database seeding completed!${NC}"
echo -e "${GREEN}Test credentials:${NC}"
echo -e "  Admin: admin@demo.com / admin123"
echo -e "  Manager: manager@demo.com / manager123"
echo -e "  Tenant code: demo"