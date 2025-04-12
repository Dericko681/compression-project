FROM rust:1.70-slim AS builder

WORKDIR /app

# Copy Cargo files
COPY rust-compressor/Cargo.toml rust-compressor/Cargo.lock ./

# Dummy build to cache dependencies
RUN mkdir src && echo "pub fn dummy() {}" > src/lib.rs && cargo build --release --lib

# Now copy the actual source code
COPY rust-compressor/src ./src

# Build real library
RUN cargo build --release --verbose --lib
