# Stage 1: Build Rust application
FROM --platform=$BUILDPLATFORM rust:1.70-slim AS builder

WORKDIR /app

# Cache cargo dependencies
COPY rust-app/Cargo.toml rust-app/Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy source and build
COPY rust-app/src ./src
RUN touch src/main.rs && \
    cargo build --release && \
    strip target/release/rust-app

# Stage 2: Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies if needed
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy built binary
COPY --from=builder /app/target/release/rust-app /app/rust-app

# Non-root user for security
RUN useradd -m appuser && \
    chown appuser:appuser /app
USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=3s \
  CMD ["/app/rust-app", "--health"]

ENTRYPOINT ["/app/rust-app"]