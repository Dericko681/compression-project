# === Build stage ===
FROM rust:1.78-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Install cargo-chef to optimize dependency builds
RUN cargo install cargo-chef

COPY . .

# Cache dependencies
RUN cargo chef prepare --recipe-path recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy actual source code
COPY . .

# Build the project
RUN cargo build --release

# Strip debug symbols to reduce binary size
RUN strip target/release/rs-compressor

# === Runtime stage ===
FROM alpine:3.19

# Install CA certificates
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/rs-compressor .

# Run the binary
ENTRYPOINT ["./rs-compressor"]
