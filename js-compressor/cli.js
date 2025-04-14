#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { compress: rleCompress, decompress: rleDecompress } = require('./le');
const { compress: lzCompress, decompress: lzDecompress, Lz77Token } = require('./lz');
const { detectFileType, selectAlgorithm } = require('./file_type');

// Magic number and version for file format identification
const MAGIC_NUMBER = Buffer.from('JSCMP');
const VERSION = 1;

function printUsage() {
    console.log('Usage: node cli.js compress|decompress [input_file1 input_file2 ...] [output_dir] [--rle|--lz|--auto]');
    console.log('Example: node cli.js compress file1.txt file2.png output_dir --auto');
    console.log('Example: node cli.js decompress file1.compressed file2.compressed output_dir --auto');
    process.exit(1);
}

async function readInput(input) {
    if (input === '-') {
        return new Promise((resolve, reject) => {
            const chunks = [];
            process.stdin.on('data', chunk => chunks.push(chunk));
            process.stdin.on('end', () => resolve(Buffer.concat(chunks)));
            process.stdin.on('error', reject);
        });
    }
    return fs.promises.readFile(input);
}

async function writeOutput(output, data) {
    if (output === '-') {
        process.stdout.write(data);
        return;
    }
    return fs.promises.writeFile(output, data);
}

function createCompressedHeader(algorithm) {
    const header = Buffer.alloc(8); // 5 bytes magic + 1 byte version + 1 byte algorithm + 1 byte reserved
    MAGIC_NUMBER.copy(header);
    header[5] = VERSION;
    header[6] = algorithm === '--rle' ? 1 : 2; // 1 for RLE, 2 for LZ
    return header;
}

function readCompressedHeader(data) {
    if (data.length < 8) {
        throw new Error('Invalid compressed file format');
    }
    
    const magic = data.slice(0, 5);
    if (!magic.equals(MAGIC_NUMBER)) {
        throw new Error('Invalid compressed file format');
    }
    
    const version = data[5];
    if (version !== VERSION) {
        throw new Error('Unsupported version');
    }
    
    const algorithm = data[6];
    return algorithm === 1 ? '--rle' : '--lz';
}

async function compressBatch(files, outputDir, algorithm) {
    const results = [];
    for (const file of files) {
        try {
            const inputData = await readInput(file);
            const filename = path.basename(file);
            const outputPath = path.join(outputDir, filename + '.compressed');
            
            let result;
            if (algorithm === '--auto') {
                const fileType = detectFileType(inputData);
                const selectedAlgorithm = selectAlgorithm(fileType, inputData);
                if (selectedAlgorithm === 'RLE') {
                    result = Buffer.from(rleCompress(inputData));
                } else {
                    const tokens = lzCompress(inputData);
                    const bytes = [];
                    for (const token of tokens) {
                        const offsetBytes = Buffer.alloc(8);
                        offsetBytes.writeBigUInt64LE(BigInt(token.offset));
                        bytes.push(...offsetBytes);
                        
                        const lengthBytes = Buffer.alloc(8);
                        lengthBytes.writeBigUInt64LE(BigInt(token.length));
                        bytes.push(...lengthBytes);
                        
                        bytes.push(token.next_char);
                    }
                    result = Buffer.from(bytes);
                }
            } else if (algorithm === '--rle') {
                result = Buffer.from(rleCompress(inputData));
            } else if (algorithm === '--lz') {
                result = Buffer.from(lzCompress(inputData));
            }

            const header = createCompressedHeader(algorithm);
            const finalResult = Buffer.concat([header, result]);
            await writeOutput(outputPath, finalResult);
            results.push({
                success: true,
                input: file,
                output: outputPath,
                algorithm: algorithm === '--auto' ? 'auto' : algorithm.slice(2)
            });
        } catch (error) {
            results.push({
                success: false,
                input: file,
                error: error.message
            });
        }
    }
    return results;
}

async function decompressBatch(files, outputDir, algorithm) {
    const results = [];
    for (const file of files) {
        try {
            const inputData = await readInput(file);
            const filename = path.basename(file, '.compressed');
            const outputPath = path.join(outputDir, filename);
            
            let result;
            if (algorithm === '--auto') {
                const header = readHeader(inputData);
                if (header.algorithm === 'RLE') {
                    result = Buffer.from(rleDecompress(inputData.slice(header.size)));
                } else {
                    const tokens = [];
                    for (let i = 0; i + 17 <= inputData.length; i += 17) {
                        const offset = Number(inputData.readBigUInt64LE(i));
                        const length = Number(inputData.readBigUInt64LE(i + 8));
                        const next_char = inputData[i + 16];
                        tokens.push(new Lz77Token(offset, length, next_char));
                    }
                    result = Buffer.from(lzDecompress(tokens));
                }
            } else if (algorithm === '--rle') {
                result = Buffer.from(rleDecompress(inputData));
            } else if (algorithm === '--lz') {
                result = Buffer.from(lzDecompress(inputData));
            }

            await writeOutput(outputPath, result);
            results.push({
                success: true,
                input: file,
                output: outputPath,
                algorithm: algorithm === '--auto' ? 'auto' : algorithm.slice(2)
            });
        } catch (error) {
            results.push({
                success: false,
                input: file,
                error: error.message
            });
        }
    }
    return results;
}

async function main() {
    const args = process.argv.slice(2);

    if (args.length < 4) {
        printUsage();
    }

    const operation = args[0];
    const algorithm = args[args.length - 1];
    
    // Validate algorithm
    if (!['--rle', '--lz', '--auto'].includes(algorithm)) {
        console.error('Invalid algorithm. Use --rle, --lz, or --auto');
        printUsage();
    }

    // Get input files and output directory
    const inputFiles = args.slice(1, args.length - 2);
    const outputDir = args[args.length - 2];

    // Create output directory if it doesn't exist
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }

    // Process all input files
    if (operation === 'compress') {
        const results = await compressBatch(inputFiles, outputDir, algorithm);
        console.log('Batch compression completed:');
        results.forEach(r => console.log(`${r.input} -> ${r.output} (${r.algorithm})`));
    } else if (operation === 'decompress') {
        const results = await decompressBatch(inputFiles, outputDir, algorithm);
        console.log('Batch decompression completed:');
        results.forEach(r => console.log(`${r.input} -> ${r.output} (${r.algorithm})`));
    } else {
        console.error('Invalid operation. Use compress or decompress');
        printUsage();
    }
}

main().catch(e => {
    console.error(e);
    process.exit(1);
}); 