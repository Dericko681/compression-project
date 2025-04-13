#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test files
TEST_FILES=("test1.txt" "test2.txt" "test3.txt")
REPORT_FILE="compression_report.md"

# Create test files with different sizes and patterns
echo "Creating test files..."
# Test file 1: Random data
dd if=/dev/urandom of=test1.txt bs=1M count=1 2>/dev/null

# Test file 2: Repeated patterns (good for RLE)
echo "Creating test file with repeated patterns..."
for i in {1..1000}; do
    echo "AAAAA BBBBB CCCCC DDDDD" >> test2.txt
done

# Test file 3: Mixed content
echo "Creating mixed content test file..."
dd if=/dev/urandom of=test3.txt bs=512K count=1 2>/dev/null
for i in {1..500}; do
    echo "REPEATED_PATTERN_$i" >> test3.txt
done

# Function to measure compression time
measure_compression() {
    local impl=$1
    local file=$2
    local start_time=$(date +%s.%N)
    
    if [ "$impl" == "js" ]; then
        cd js-compressor
        node cli.js compress "../$file" "../${file}.js.compressed" > /dev/null 2>&1
        cd ..
    elif [ "$impl" == "rs" ]; then
        docker run -v $(pwd):/data rust-compressor compress "/data/$file" "/data/${file}.rs.compressed" --rle > /dev/null 2>&1
    fi
    
    local end_time=$(date +%s.%N)
    echo "$end_time - $start_time" | bc
}

# Function to measure decompression time
measure_decompression() {
    local impl=$1
    local file=$2
    local start_time=$(date +%s.%N)
    
    if [ "$impl" == "js" ]; then
        cd js-compressor
        node cli.js decompress "../${file}.js.compressed" "../${file}.js.decompressed" > /dev/null 2>&1
        cd ..
    elif [ "$impl" == "rs" ]; then
        docker run -v $(pwd):/data rust-compressor decompress "/data/${file}.rs.compressed" "/data/${file}.rs.decompressed" > /dev/null 2>&1
    fi
    
    local end_time=$(date +%s.%N)
    echo "$end_time - $start_time" | bc
}

# Function to verify decompressed files match original
verify_decompression() {
    local file=$1
    local impl=$2
    
    if [ "$impl" == "js" ]; then
        cmp -s "$file" "${file}.js.decompressed"
    else
        cmp -s "$file" "${file}.rs.decompressed"
    fi
    
    if [ $? -eq 0 ]; then
        echo "✓"
    else
        echo "✗"
    fi
}

# Create markdown report
echo "# Compression Performance Comparison Report" > $REPORT_FILE
echo -e "\n## Test Results\n" >> $REPORT_FILE
echo "| File | Size | JS Comp Time | RS Comp Time | JS Decomp Time | RS Decomp Time | JS Comp Size | RS Comp Size | JS Verify | RS Verify |" >> $REPORT_FILE
echo "|------|------|--------------|--------------|----------------|----------------|--------------|--------------|-----------|-----------|" >> $REPORT_FILE

# Run tests and generate report
for file in "${TEST_FILES[@]}"; do
    echo -e "${YELLOW}Testing $file...${NC}"
    
    # Get original file size
    size=$(du -h "$file" | cut -f1)
    
    # Measure JS compression
    echo -e "${GREEN}Measuring JS compression...${NC}"
    js_comp_time=$(measure_compression "js" "$file")
    
    # Measure Rust compression
    echo -e "${GREEN}Measuring Rust compression...${NC}"
    rs_comp_time=$(measure_compression "rs" "$file")
    
    # Measure JS decompression
    echo -e "${GREEN}Measuring JS decompression...${NC}"
    js_decomp_time=$(measure_decompression "js" "$file")
    
    # Measure Rust decompression
    echo -e "${GREEN}Measuring Rust decompression...${NC}"
    rs_decomp_time=$(measure_decompression "rs" "$file")
    
    # Get compressed file sizes
    js_comp_size=$(du -h "${file}.js.compressed" | cut -f1)
    rs_comp_size=$(du -h "${file}.rs.compressed" | cut -f1)
    
    # Verify decompression
    js_verify=$(verify_decompression "$file" "js")
    rs_verify=$(verify_decompression "$file" "rs")
    
    # Add to report
    echo "| $file | $size | $js_comp_time | $rs_comp_time | $js_decomp_time | $rs_decomp_time | $js_comp_size | $rs_comp_size | $js_verify | $rs_verify |" >> $REPORT_FILE
    
    # Cleanup
    rm -f "${file}.js.compressed" "${file}.rs.compressed" "${file}.js.decompressed" "${file}.rs.decompressed"
done

# Cleanup test files
echo -e "${YELLOW}Cleaning up test files...${NC}"
rm -f "${TEST_FILES[@]}"

# Add summary section
echo -e "\n## Summary\n" >> $REPORT_FILE
echo "This report compares the performance of JavaScript and Rust implementations of compression algorithms." >> $REPORT_FILE
echo "- Compression Time: Time taken to compress the input file" >> $REPORT_FILE
echo "- Decompression Time: Time taken to restore the original file" >> $REPORT_FILE
echo "- Compressed Size: Size of the compressed output file" >> $REPORT_FILE
echo "- Verification: Checks if the decompressed file matches the original (✓ = success, ✗ = failure)" >> $REPORT_FILE

echo -e "\n${GREEN}Report generated: $REPORT_FILE${NC}"
echo -e "${YELLOW}To view the report, run: cat $REPORT_FILE${NC}" 