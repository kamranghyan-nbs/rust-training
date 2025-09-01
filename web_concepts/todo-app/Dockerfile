FROM rust:latest as builder
WORKDIR /app

# Install cli
RUN cargo install sea-orm-cli

COPY . .

# Build both backend and migration binaries
RUN cargo build --release -p backend -p migration

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# copy backend binary
COPY --from=builder /app/target/release/backend /app/backend
# copy migration binary (not a folder!)
COPY --from=builder /app/target/release/migration /app/migration
COPY --from=builder /usr/local/cargo/bin/sea-orm-cli /usr/local/bin/sea-orm-cli

CMD ["./backend"]