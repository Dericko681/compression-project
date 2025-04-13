# Compression Performance Comparison Report

## Test Results

| File | Size | JS Comp Time | RS Comp Time | JS Decomp Time | RS Decomp Time | JS Comp Size | RS Comp Size | JS Verify | RS Verify |
|------|------|--------------|--------------|----------------|----------------|--------------|--------------|-----------|-----------|
| test1.txt | 1.0M | .038279097 | 2.353836257 | .057914516 | .023704874 |  |  | ✗ | ✗ |
| test2.txt | 24K | .036593208 | .021290618 | .038328256 | .021937292 |  |  | ✗ | ✗ |
| test3.txt | 524K | .037591399 | .021831838 | .036490075 | .019965058 |  |  | ✗ | ✗ |

## Summary

This report compares the performance of JavaScript and Rust implementations of compression algorithms.
- Compression Time: Time taken to compress the input file
- Decompression Time: Time taken to restore the original file
- Compressed Size: Size of the compressed output file
- Verification: Checks if the decompressed file matches the original (✓ = success, ✗ = failure)
