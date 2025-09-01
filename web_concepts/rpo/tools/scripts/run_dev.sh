#!/bin/bash

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}Starting development environment for Auth Service...${NC}"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}Docker is not running. Please start Docker first.${NC}"
    exit 1
fi

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo -e "${BLUE}Creating .env file...${NC}"
    cp .env.example .env
    echo -e "${GREEN}.env file created successfully${NC}"
fi

# Start databases
echo -e "${BLUE}Starting PostgreSQL and Redis...${NC}"
docker-compose up -d postgres redis

# Wait for databases to be ready
echo -e "${BLUE}Waiting for databases to be ready...${NC}"
sleep 5

# Run migrations
echo -e "${BLUE}Running database migrations...${NC}"
cargo run --package auth-service --bin migrator up

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Migrations completed successfully!${NC}"
else
    echo -e "${RED}Migration failed!${NC}"
    exit 1
fi

# Seed database (optional)
echo -e "${BLUE}Seeding database with test data...${NC}"
./tools/scripts/seed_db.sh

# Start the auth service
echo -e "${BLUE}Starting Auth Service...${NC}"
echo -e "${GREEN}Auth Service will be available at: http://localhost:8080${NC}"
echo -e "${GREEN}Swagger UI will be available at: http://localhost:8080/swagger-ui/${NC}"
echo -e "${GREEN}Press Ctrl+C to stop the service${NC}"

cargo run --package auth-service