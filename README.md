# Compression Algorithms Comparison

This repo implements two compression algorithms (RLE and LZ) in both JavaScript and Rust, with a comparison script to benchmark their performance.

## Algorithms

### RLE (Run-Length Encoding)
Simple compression that replaces repeated characters with a count. Good for files with lots of repeated data.

### LZ (Lempel-Ziv)
More complex algorithm that looks for repeated patterns. Better for general-purpose compression.

## Usage

You have two options:

### Option 1: Clone and Build
```bash
# Clone the repo
git clone https://github.com/yourusername/compression-project.git
cd compression-project

# Build JS implementation
cd js-compressor
npm install
cd ..

# Build Rust implementation
cd rs-compressor
cargo build --release
cd ..

# Run comparison
./compare_compression.sh input.txt output
```

### Option 2: Use Docker Images
```bash
# Pull images
docker pull ghcr.io/yourusername/js-compressor:latest
docker pull ghcr.io/yourusername/rust-compressor:latest

# Run comparison
./compare_compression.sh input.txt output
```

The comparison script will:
- Compress your file using all four combinations (JS/Rust × RLE/LZ)
- Measure compression and decompression times
- Show compressed file sizes
- Generate a report in `compression_report.md`

## Direct Usage

### JavaScript
```bash
# Compress
node cli.js compress input.txt output.txt --rle  # or --lz

# Decompress
node cli.js decompress output.txt decompressed.txt --rle  # or --lz
```

### Rust
```bash
# Compress
cargo run -- compress input.txt output.txt --rle  # or --lz

# Decompress
cargo run -- decompress output.txt decompressed.txt --rle  # or --lz
```

## License
MIT
