# Stage 1: Build Rust library
FROM rust:1.70-slim AS builder

WORKDIR /app

# Copy Cargo files
COPY rust-compressor/Cargo.toml rust-compressor/Cargo.lock ./

# Create dummy src/lib.rs to cache dependencies
RUN mkdir src && echo "" > src/lib.rs

# Cache dependencies
RUN cargo build --release

# Now copy actual source
COPY rust-compressor/src ./src

# Build the real library
RUN cargo build --release

# Output directory: /app/target/release
