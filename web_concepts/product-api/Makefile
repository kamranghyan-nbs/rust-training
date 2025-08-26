.PHONY: build run test clean docker-build docker-up docker-down docker-logs

# Build the application
build:
	cargo build --release

# Run the application locally
run:
	cargo run

# Run tests
test:
	cargo test

# Run rate limiting tests specifically
test-rate-limit:
	cargo test rate_limit

# Test async logging functionality
test-logging:
	chmod +x scripts/test_logging.sh
	./scripts/test_logging.sh

# Run with pretty console logging (development)
run-dev:
	LOG_FORMAT=pretty LOG_OUTPUT=console RUST_LOG=debug cargo run

# Run with JSON file logging (production)
run-prod:
	LOG_FORMAT=json LOG_OUTPUT=both LOG_DIR=./logs RUST_LOG=info cargo run

# Run with tokio console debugging
run-debug:
	ENABLE_TOKIO_CONSOLE=true RUST_LOG=debug cargo run

# View real-time logs
logs:
	tail -f logs/product-api.*

# Parse JSON logs with jq
logs-json:
	tail -f logs/product-api.* | jq '.'

# Monitor errors only
logs-errors:
	tail -f logs/product-api.* | jq 'select(.level == "ERROR")'

# Monitor performance (requests > 100ms)
logs-slow:
	tail -f logs/product-api.* | jq 'select(.fields.duration_ms > 100)'

# Clean build artifacts
clean:
	cargo clean

# Build Docker image
docker-build:
	docker-compose build

# Start all services with Docker Compose
docker-up:
	docker-compose up -d

# Stop all services
docker-down:
	docker-compose down

# View logs
docker-logs:
	docker-compose logs -f

# Start services in foreground (for development)
docker-dev:
	docker-compose up

# Reset database (recreate containers)
docker-reset:
	docker-compose down -v
	docker-compose up --build

# Check if services are healthy
health-check:
	curl -f http://localhost:8080/health || exit 1

# Format code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy -- -D warnings

# Install development dependencies
dev-setup:
	rustup component add rustfmt clippy

# Full development cycle
dev: fmt clippy test build

# Create .env file from example
env:
	cp .env.example .env

# Lint, Build, Test, Audit, Deny, Geiger
# cargo install cargo-audit
# cargo install cargo-deny
# cargo install cargo-geiger
lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

build:
	cargo build --all-targets --all-features

test:
	cargo test --all

audit:
	cargo audit

deny:
	cargo deny check

geiger:
	cargo geiger

check-all: lint build test audit deny geiger
