# Stage 1: Build Rust library
FROM rust:1.70-slim AS builder

WORKDIR /app

# Copy only dependency files first for caching
COPY rust-compressor/Cargo.toml rust-compressor/Cargo.lock ./

# Create a temporary empty project to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn lib_func() {}" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the real source code
COPY rust-compressor/src ./src

# Touch Cargo.toml to ensure rebuild
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