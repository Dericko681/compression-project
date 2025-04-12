FROM rust:1.70-slim AS builder

WORKDIR /app

# Copy Cargo files
COPY rust-compressor/Cargo.toml rust-compressor/Cargo.lock ./

# Dummy lib.rs to cache dependencies
RUN mkdir src && echo "" > src/lib.rs

RUN cargo build --release

# Copy full source
COPY rust-compressor/src ./src

RUN cargo build --release --verbose
