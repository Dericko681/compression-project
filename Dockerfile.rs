# Stage 1: Build Rust library
FROM --platform=$BUILDPLATFORM rust:1.70-slim AS builder

WORKDIR /app

# Cache cargo dependencies
COPY rust-compressor/Cargo.toml rust-compressor/Cargo.lock ./
# Create dummy lib.rs to satisfy initial build
RUN mkdir -p src && \
    echo "// dummy file" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy real source files
COPY rust-compressor/src ./src
# Touch build files to trigger rebuild
RUN touch src/lib.rs && \
    cargo build --release

# Stage 2: Runtime image (if needed for tests/benchmarks)
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies if needed
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy built artifacts
COPY --from=builder /app/target/release /usr/local/lib

# Non-root user for security
RUN useradd -m appuser
USER appuser

# Set up environment variables if needed
ENV LD_LIBRARY_PATH=/usr/local/lib