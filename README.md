# Compression Project

A project implementing various compression algorithms in both Rust and JavaScript, with WebAssembly support.

## Features

- Run-Length Encoding (RLE) compression
- LZ77 compression
- Automatic algorithm selection based on file type
- WebAssembly implementation for browser usage
- Command-line interface for both Rust and JavaScript versions
- Batch compression and decompression support

## Prerequisites

- Rust (for Rust implementation)
- Node.js (for JavaScript implementation)
- Python 3 (for local development server)
- Modern web browser with WebAssembly support

## Installation

### Rust Implementation

```bash
cd rs-compressor
cargo build --release
```

### JavaScript Implementation

```bash
cd js-compressor
npm install
```

### WebAssembly Build

To build the WebAssembly module:

```bash
# Navigate to the Rust project directory
cd rs-compressor

# Build for wasm32 target
cargo build --target wasm32-unknown-unknown --release

# Generate JavaScript bindings
wasm-bindgen --target web --out-dir ../js-compressor/pkg target/wasm32-unknown-unknown/release/rs_compressor.wasm
```

## Usage

### Command Line Interface

#### Rust Implementation

```bash
# Compress a file
cargo run -- compress input.txt output.txt --auto

# Decompress a file
cargo run -- decompress output.txt decompressed.txt --auto

# Batch compress files
cargo run -- compress-batch input_dir output_dir --auto

# Batch decompress files
cargo run -- decompress-batch input_dir output_dir --auto
```

#### JavaScript Implementation

```bash
# Compress a file
node cli.js compress input.txt output.txt --auto

# Decompress a file
node cli.js decompress output.txt decompressed.txt --auto

# Batch compress files
node cli.js compress-batch input_dir output_dir --auto

# Batch decompress files
node cli.js decompress-batch input_dir output_dir --auto
```

### WebAssembly Demo

To use the WebAssembly demo in your browser:

1. Start a local web server:
```bash
cd js-compressor
python3 -m http.server 8000
```

2. Open your browser and navigate to:
```
http://localhost:8000/wasm-demo.html
```

3. Use the web interface to:
   - Select files for compression/decompression
   - Choose compression algorithm (Auto, RLE, or LZ)
   - View compression results and download processed files

#### WebAssembly API

The WebAssembly module can also be used programmatically in your JavaScript code:

```javascript
import init, { compress, compress_batch, decompress, decompress_batch, Algorithm } from './pkg/rs_compressor.js';

// Initialize the WASM module
const wasm = await init();

// Compress a single file
const result = await compress(fileData, Algorithm.Auto, filename);

// Compress multiple files
const batchResult = await compress_batch(files, Algorithm.Auto);

// Decompress a file
const decompressed = await decompress(compressedData, Algorithm.Auto);

// Decompress multiple files
const decompressedBatch = await decompress_batch(files, Algorithm.Auto);
```

Available algorithms:
- `Algorithm.Auto`: Automatically selects the best algorithm
- `Algorithm.RLE`: Uses Run-Length Encoding
- `Algorithm.LZ`: Uses LZ77 compression

## Development

### Running Tests

#### Rust Tests
```bash
cd rs-compressor
cargo test
```

#### JavaScript Tests
```bash
cd js-compressor
npm test
```

### Building Documentation

#### Rust Documentation
```bash
cd rs-compressor
cargo doc --open
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
