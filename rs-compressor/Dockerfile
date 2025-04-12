# === Build stage ===
FROM rust:1.78-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Install cargo-chef to optimize dependency builds
RUN cargo install cargo-chef

# Copy only the files needed for dependency resolution
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Cache dependencies
RUN cargo chef prepare --recipe-path recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build the application
RUN cargo build --release

# === Runtime stage ===
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN addgroup --system --gid 1001 appgroup && \
    adduser --system --uid 1001 --gid 1001 appuser

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder --chown=appuser:appgroup /app/target/release/rs-compressor /app/rs-compressor

# Switch to non-root user
USER appuser

# Set the entrypoint
ENTRYPOINT ["/app/rs-compressor"]
