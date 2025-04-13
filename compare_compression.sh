#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if input and output filenames are provided
if [ $# -ne 2 ]; then
    echo -e "${RED}Usage: $0 <input_file> <compressed_file>${NC}"
    echo "Example: $0 test.txt test.txt.compressed"
    exit 1
fi

INPUT_FILE=$1
COMPRESSED_FILE=$2
REPORT_FILE="compression_report.md"

# Check if input file exists
if [ ! -f "$INPUT_FILE" ]; then
    echo -e "${RED}Error: Input file '$INPUT_FILE' does not exist${NC}"
    exit 1
fi

# Function to measure compression time
measure_compression() {
    local impl=$1
    local algo=$2
    local input=$3
    local output=$4
    local start_time=$(date +%s.%N)
    
    if [ "$impl" == "js" ]; then
        cd js-compressor
        if [ "$algo" == "rle" ]; then
            node cli.js compress "../$input" "../$output" --rle > /dev/null 2>&1
        else
            node cli.js compress "../$input" "../$output" --lz > /dev/null 2>&1
        fi
        cd ..
    elif [ "$impl" == "rs" ]; then
        if [ "$algo" == "rle" ]; then
            docker run -v $(pwd):/data rust-compressor compress "/data/$input" "/data/$output" --rle > /dev/null 2>&1
        else
            docker run -v $(pwd):/data rust-compressor compress "/data/$input" "/data/$output" --lz > /dev/null 2>&1
        fi
    fi
    
    local end_time=$(date +%s.%N)
    echo "$end_time - $start_time" | bc
}

# Function to measure decompression time
measure_decompression() {
    local impl=$1
    local algo=$2
    local compressed=$3
    local decompressed=$4
    local start_time=$(date +%s.%N)
    
    if [ "$impl" == "js" ]; then
        cd js-compressor
        if [ "$algo" == "rle" ]; then
            node cli.js decompress "../$compressed" "../$decompressed" --rle > /dev/null 2>&1
        else
            node cli.js decompress "../$compressed" "../$decompressed" --lz > /dev/null 2>&1
        fi
        cd ..
    elif [ "$impl" == "rs" ]; then
        if [ "$algo" == "rle" ]; then
            docker run -v $(pwd):/data rust-compressor decompress "/data/$compressed" "/data/$decompressed" --rle > /dev/null 2>&1
        else
            docker run -v $(pwd):/data rust-compressor decompress "/data/$compressed" "/data/$decompressed" --lz > /dev/null 2>&1
        fi
    fi
    
    local end_time=$(date +%s.%N)
    echo "$end_time - $start_time" | bc
}

# Function to verify decompressed files match original
verify_decompression() {
    local input=$1
    local decompressed=$2
    
    if [ ! -f "$decompressed" ]; then
        echo "✗"
        return
    fi
    
    cmp -s "$input" "$decompressed"
    
    if [ $? -eq 0 ]; then
        echo "✓"
    else
        echo "✗"
    fi
}

# Function to get file size
get_file_size() {
    local file=$1
    if [ -f "$file" ]; then
        du -h "$file" | cut -f1
    else
        echo "N/A"
    fi
}

# Function to compare times and return the faster implementation
compare_times() {
    local time1=$1
    local time2=$2
    local impl1=$3
    local impl2=$4
    
    if [ "$time1" == "N/A" ] || [ "$time2" == "N/A" ]; then
        echo "N/A"
        return
    fi
    
    if (( $(echo "$time1 < $time2" | bc -l) )); then
        echo "$impl1"
    else
        echo "$impl2"
    fi
}

# Create markdown report
echo "# Compression Performance Comparison Report" > $REPORT_FILE
echo -e "\n## Test Results\n" >> $REPORT_FILE
echo "| Algorithm | Implementation | Comp Time | Decomp Time | Comp Size | Verify |" >> $REPORT_FILE
echo "|-----------|----------------|-----------|-------------|-----------|--------|" >> $REPORT_FILE

echo -e "${YELLOW}Testing $INPUT_FILE...${NC}"

# Get original file size
size=$(du -h "$INPUT_FILE" | cut -f1)

# Test all combinations
for algo in "rle" "lz"; do
    for impl in "js" "rs"; do
        echo -e "${GREEN}Testing $impl $algo...${NC}"
        
        # Create filenames
        COMPRESSED="${COMPRESSED_FILE}.${impl}.${algo}"
        DECOMPRESSED="${COMPRESSED_FILE}.${impl}.${algo}.decompressed"
        
        # Measure compression
        comp_time=$(measure_compression "$impl" "$algo" "$INPUT_FILE" "$COMPRESSED")
        
        # Measure decompression
        decomp_time=$(measure_decompression "$impl" "$algo" "$COMPRESSED" "$DECOMPRESSED")
        
        # Get compressed size
        comp_size=$(get_file_size "$COMPRESSED")
        
        # Verify decompression
        verify=$(verify_decompression "$INPUT_FILE" "$DECOMPRESSED")
        
        # Add to report
        echo "| $algo | $impl | $comp_time | $decomp_time | $comp_size | $verify |" >> $REPORT_FILE
        
        # Cleanup
        rm -f "$COMPRESSED" "$DECOMPRESSED"
    done
done

# Add summary section
echo -e "\n## Summary\n" >> $REPORT_FILE
echo "This report compares the performance of different compression algorithms and implementations." >> $REPORT_FILE
echo "- Algorithms: RLE (Run-Length Encoding) and LZ (Lempel-Ziv)" >> $REPORT_FILE
echo "- Implementations: JavaScript and Rust" >> $REPORT_FILE
echo "- Compression Time: Time taken to compress the input file" >> $REPORT_FILE
echo "- Decompression Time: Time taken to restore the original file" >> $REPORT_FILE
echo "- Compressed Size: Size of the compressed output file" >> $REPORT_FILE
echo "- Verification: Checks if the decompressed file matches the original (✓ = success, ✗ = failure)" >> $REPORT_FILE

echo -e "\n${GREEN}Report generated: $REPORT_FILE${NC}"
echo -e "${YELLOW}To view the report, run: cat $REPORT_FILE${NC}" 