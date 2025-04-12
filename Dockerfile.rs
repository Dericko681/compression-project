FROM rust:1.70-slim AS builder

WORKDIR /app

# Copy only Cargo files
COPY rust-compressor/Cargo.toml rust-compressor/Cargo.lock ./

# Dummy build to cache dependencies
RUN mkdir src && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    echo "[lib]\npath = \"src/lib.rs\"" >> Cargo.toml && \
    cargo build --release --lib

# Now copy actual sources
COPY rust-compressor/src ./src

# Build real library
RUN cargo build --release --lib
