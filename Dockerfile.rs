# Stage 1: Build Rust library
FROM rust:1.70-slim AS builder

WORKDIR /app

# Copy just the Cargo files first
COPY rust-compressor/Cargo.toml rust-compressor/Cargo.lock ./

# Create minimal source structure needed for initial build
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn lib_func() {}" > src/lib.rs

# Cache dependencies by doing an initial build
RUN cargo build --release

# Now copy the real source files (this will invalidate cache when sources change)
COPY rust-compressor/src ./src

# Touch Cargo.toml to ensure rebuild if needed
RUN touch Cargo.toml && \
    cargo build --release

# Stage 2: Create minimal runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Copy built library
COPY --from=builder /app/target/release /usr/local/lib

# Set library path
ENV LD_LIBRARY_PATH=/usr/local/lib

# Non-root user for security
RUN useradd -m appuser
USER appuser