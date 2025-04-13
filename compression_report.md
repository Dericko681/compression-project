# Compression Performance Comparison Report

## Test Results

| Algorithm | Implementation | Comp Time | Decomp Time | Comp Size | Verify |
|-----------|----------------|-----------|-------------|-----------|--------|
| rle | js | .045215213 | .038302056 | 4.0K | ✗ |
| rle | rs | .022162878 | .021739981 | N/A | ✗ |
| lz | js | .081754806 | .047950491 | 12K | ✓ |
| lz | rs | .023403279 | .023048029 | N/A | ✗ |

## Summary

This report compares the performance of different compression algorithms and implementations.
- Algorithms: RLE (Run-Length Encoding) and LZ (Lempel-Ziv)
- Implementations: JavaScript and Rust
- Compression Time: Time taken to compress the input file
- Decompression Time: Time taken to restore the original file
- Compressed Size: Size of the compressed output file
- Verification: Checks if the decompressed file matches the original (✓ = success, ✗ = failure)
