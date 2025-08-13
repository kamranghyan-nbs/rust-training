FROM rust:latest as builder
WORKDIR /app

# Install build deps
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs so Cargo is happy
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch dependencies (no build output needed)
RUN cargo fetch

# Now copy the actual source
COPY . .

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/product-api .

EXPOSE 8080
CMD ["./product-api"]