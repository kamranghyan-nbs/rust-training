# Variables
DOCKER_COMPOSE = docker-compose
SERVICE_NAME = auth-service
DB_NAME = pos_auth
DB_USER = pos_user
DB_PASSWORD = pos_password

# Colors for output
RED = \033[0;31m
GREEN = \033[0;32m
YELLOW = \033[0;33m
BLUE = \033[0;34m
NC = \033[0m # No Color

.PHONY: help setup build test clean dev stop logs db-migrate db-reset lint format check

help: ## Show this help message
	@echo "$(BLUE)Available commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $1, $2}'

setup: ## Setup development environment
	@echo "$(BLUE)Setting up development environment...$(NC)"
	cp .env.example .env
	$(DOCKER_COMPOSE) up -d postgres redis
	@echo "$(GREEN)Waiting for database to be ready...$(NC)"
	@sleep 5
	make db-migrate
	@echo "$(GREEN)Setup complete!$(NC)"

build: ## Build the auth service
	@echo "$(BLUE)Building auth service...$(NC)"
	cargo build --package auth-service
	@echo "$(GREEN)Build complete!$(NC)"

build-release: ## Build the auth service in release mode
	@echo "$(BLUE)Building auth service (release)...$(NC)"
	cargo build --release --package auth-service
	@echo "$(GREEN)Release build complete!$(NC)"

test: ## Run tests
	@echo "$(BLUE)Running tests...$(NC)"
	cargo test --package auth-service
	@echo "$(GREEN)Tests complete!$(NC)"

test-integration: ## Run integration tests
	@echo "$(BLUE)Running integration tests...$(NC)"
	$(DOCKER_COMPOSE) up -d postgres-test
	@sleep 3
	RUST_LOG=debug cargo test --package auth-service --test integration_tests -- --test-threads=1
	$(DOCKER_COMPOSE) down postgres-test
	@echo "$(GREEN)Integration tests complete!$(NC)"

clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	cargo clean
	@echo "$(GREEN)Clean complete!$(NC)"

dev: ## Start development environment
	@echo "$(BLUE)Starting development environment...$(NC)"
	$(DOCKER_COMPOSE) up -d postgres redis
	@sleep 2
	make db-migrate
	@echo "$(GREEN)Development environment ready!$(NC)"
	@echo "$(YELLOW)Run 'cargo run --package auth-service' to start the service$(NC)"

dev-full: ## Start full development stack
	@echo "$(BLUE)Starting full development stack...$(NC)"
	$(DOCKER_COMPOSE) up -d
	@echo "$(GREEN)Full stack started!$(NC)"

stop: ## Stop all services
	@echo "$(BLUE)Stopping all services...$(NC)"
	$(DOCKER_COMPOSE) down
	@echo "$(GREEN)All services stopped!$(NC)"

logs: ## Show logs for all services
	$(DOCKER_COMPOSE) logs -f

logs-auth: ## Show logs for auth service
	$(DOCKER_COMPOSE) logs -f $(SERVICE_NAME)

db-migrate: ## Run database migrations
	@echo "$(BLUE)Running database migrations...$(NC)"
	cargo run --package auth-service --bin migrator up
	@echo "$(GREEN)Migrations complete!$(NC)"

db-reset: ## Reset database (drop and recreate)
	@echo "$(YELLOW)Resetting database...$(NC)"
	$(DOCKER_COMPOSE) exec postgres psql -U $(DB_USER) -c "DROP DATABASE IF EXISTS $(DB_NAME);"
	$(DOCKER_COMPOSE) exec postgres psql -U $(DB_USER) -c "CREATE DATABASE $(DB_NAME);"
	make db-migrate
	@echo "$(GREEN)Database reset complete!$(NC)"

db-seed: ## Seed database with test data
	@echo "$(BLUE)Seeding database...$(NC)"
	./tools/scripts/seed_db.sh
	@echo "$(GREEN)Database seeded!$(NC)"

lint: ## Run clippy lints
	@echo "$(BLUE)Running lints...$(NC)"
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)Lints complete!$(NC)"

format: ## Format code
	@echo "$(BLUE)Formatting code...$(NC)"
	cargo fmt --all
	@echo "$(GREEN)Format complete!$(NC)"

format-check: ## Check code formatting
	@echo "$(BLUE)Checking code formatting...$(NC)"
	cargo fmt --all -- --check
	@echo "$(GREEN)Format check complete!$(NC)"

check: ## Run all checks (format, lint, test)
	@echo "$(BLUE)Running all checks...$(NC)"
	make format-check
	make lint
	make test
	@echo "$(GREEN)All checks passed!$(NC)"

docker-build: ## Build Docker image
	@echo "$(BLUE)Building Docker image...$(NC)"
	docker build -t $(SERVICE_NAME):latest -f services/auth/Dockerfile .
	@echo "$(GREEN)Docker image built!$(NC)"

docker-run: ## Run Docker container
	@echo "$(BLUE)Running Docker container...$(NC)"
	docker run -p 8080:8080 --env-file .env $(SERVICE_NAME):latest
	@echo "$(GREEN)Docker container started!$(NC)"

# CI/CD targets
ci-test: ## Run tests in CI environment
	@echo "$(BLUE)Running CI tests...$(NC)"
	cargo test --all-features --workspace
	@echo "$(GREEN)CI tests complete!$(NC)"

ci-build: ## Build for CI environment
	@echo "$(BLUE)Building for CI...$(NC)"
	cargo build --release --all-features --workspace
	@echo "$(GREEN)CI build complete!$(NC)"

# Documentation
docs: ## Generate documentation
	@echo "$(BLUE)Generating documentation...$(NC)"
	cargo doc --no-deps --workspace
	@echo "$(GREEN)Documentation generated!$(NC)"

docs-open: ## Open documentation
	cargo doc --no-deps --workspace --open