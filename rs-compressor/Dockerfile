# === Build stage ===
FROM rust:1.76-alpine as builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Build actual project
COPY src ./src
RUN cargo build --release && strip target/release/rs-compressor

# === Runtime stage ===
FROM alpine:latest

WORKDIR /app
COPY --from=builder /app/target/release/rs-compressor .

ENTRYPOINT ["./rs-compressor"]