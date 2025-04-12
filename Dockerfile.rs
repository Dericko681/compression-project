FROM rust:1.70-slim AS builder

WORKDIR /app

# Copy full source
COPY rust-compressor .

# Build the library
RUN cargo build --release --lib
