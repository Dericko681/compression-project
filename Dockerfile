# Stage 1: Rust builder
FROM rust:1.70 as rust-builder
WORKDIR /usr/src/rust-compressor
COPY rust-compressor .
RUN cargo build --release

# Stage 2: Node.js builder
FROM node:18 as node-builder
WORKDIR /usr/src/app
COPY js-compressor/package.json js-compressor/package-lock.json ./js-compressor/
WORKDIR /usr/src/app/js-compressor
RUN npm ci --production

# Copy application files after npm install to leverage layer caching
COPY js-compressor .
RUN npm run build

# Final stage
FROM node:18-slim
WORKDIR /usr/src/app

# Copy built Rust binary
COPY --from=rust-builder /usr/src/rust-compressor/target/release/lz_rust_compressor /usr/local/bin/

# Copy built Node.js application
COPY --from=node-builder /usr/src/app/js-compressor ./js-compressor

# Set up entry point
WORKDIR /usr/src/app/js-compressor
ENTRYPOINT ["node", "index.js"]