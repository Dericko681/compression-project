# Stage 1: Build Rust library
FROM rust:1.70-slim AS builder

WORKDIR /app

# Copy dependency files first for caching
COPY rust-compressor/Cargo.toml rust-compressor/Cargo.lock ./

# Download and cache dependencies without building
RUN cargo fetch

# Copy the real source code
COPY rust-compressor/src ./src

# Build the actual project (dependencies are cached from previous step)
RUN cargo build --release

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