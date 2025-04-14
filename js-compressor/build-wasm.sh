#!/bin/bash

# Build the Rust WebAssembly module
cd ../rs-compressor
cargo build --target wasm32-unknown-unknown --release

# Generate JavaScript bindings
wasm-bindgen --target web --out-dir ../js-compressor/pkg target/wasm32-unknown-unknown/release/rs_compressor.wasm

# Optimize the WebAssembly module
wasm-opt -O3 -o ../js-compressor/pkg/rs_compressor_bg.wasm ../js-compressor/pkg/rs_compressor_bg.wasm

echo "WebAssembly module built successfully!" 