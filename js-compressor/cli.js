#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { compress: rleCompress, decompress: rleDecompress } = require('./le');
const { compress: lzCompress, decompress: lzDecompress, Lz77Token } = require('./lz');

function printUsage() {
    console.log('Usage: node cli.js compress|decompress <input_file> <output_file> --rle|--lz');
    console.log('Example: node cli.js compress input.txt output.txt --rle');
    process.exit(1);
}

function main() {
    const args = process.argv.slice(2);

    if (args.length !== 4) {
        printUsage();
    }

    const [operation, inputFile, outputFile, algorithm] = args;

    if (!fs.existsSync(inputFile)) {
        console.error(`Error: Input file '${inputFile}' does not exist`);
        process.exit(1);
    }

    let inputData;
    try {
        inputData = fs.readFileSync(inputFile);
    } catch (e) {
        console.error(`Error reading input file: ${e.message}`);
        process.exit(1);
    }

    let result;
    try {
        switch (operation) {
            case 'compress':
                if (algorithm === '--rle') {
                    result = Buffer.from(rleCompress(inputData.toString()));
                } else if (algorithm === '--lz') {
                    const tokens = lzCompress(inputData);
                    const bytes = [];
                    for (const token of tokens) {
                        const offsetBytes = Buffer.alloc(8);
                        offsetBytes.writeBigUInt64LE(BigInt(token.offset));
                        bytes.push(...offsetBytes);
                        
                        const lengthBytes = Buffer.alloc(8);
                        lengthBytes.writeBigUInt64LE(BigInt(token.length));
                        bytes.push(...lengthBytes);
                        
                        bytes.push(token.nextChar);
                    }
                    result = Buffer.from(bytes);
                }
                break;
            case 'decompress':
                if (algorithm === '--rle') {
                    result = Buffer.from(rleDecompress(inputData.toString()));
                } else if (algorithm === '--lz') {
                    const tokens = [];
                    let i = 0;
                    while (i + 17 <= inputData.length) {
                        const offset = Number(inputData.readBigUInt64LE(i));
                        const length = Number(inputData.readBigUInt64LE(i + 8));
                        const nextChar = inputData[i + 16];
                        tokens.push(new Lz77Token(offset, length, nextChar));
                        i += 17;
                    }
                    result = lzDecompress(tokens);
                }
                break;
            default:
                console.error('Invalid operation');
                printUsage();
        }
    } catch (e) {
        console.error(`Error during ${operation}: ${e.message}`);
        process.exit(1);
    }

    try {
        fs.writeFileSync(outputFile, result);
    } catch (e) {
        console.error(`Error writing output file: ${e.message}`);
        process.exit(1);
    }

    console.log('Operation completed successfully!');
    console.log(`Input size: ${inputData.length} bytes`);
    console.log(`Output size: ${result.length} bytes`);
    console.log(`Compression ratio: ${((result.length / inputData.length) * 100).toFixed(2)}%`);
}

main(); 