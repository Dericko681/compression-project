import init, { compress, compress_batch, decompress, decompress_batch, Algorithm } from './pkg/rs_compressor.js';

let wasmInitialized = false;
let wasmModule = null;

async function initWasm() {
    if (!wasmInitialized) {
        try {
            wasmModule = await init();
            wasmInitialized = true;
            console.log('WebAssembly module initialized successfully');
        } catch (error) {
            console.error('Failed to initialize WebAssembly module:', error);
            throw error;
        }
    }
    return { compress, compress_batch, decompress, decompress_batch, Algorithm };
}

async function compressFile(file, algorithm = Algorithm.Auto) {
    try {
        const wasm = await initWasm();
        const arrayBuffer = await file.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        
        const result = wasm.compress(uint8Array, algorithm, file.name);
        console.log('Compression successful!');
        console.log('Algorithm used:', result.algorithm);
        return result;
    } catch (error) {
        console.error('Compression failed:', error);
        throw error;
    }
}

async function compressFiles(files, algorithm = Algorithm.Auto) {
    try {
        const wasm = await initWasm();
        const fileArray = [];
        
        for (const file of files) {
            const arrayBuffer = await file.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);
            fileArray.push({
                name: file.name,
                data: uint8Array
            });
        }
        
        const result = wasm.compress_batch(fileArray, algorithm);
        console.log('Batch compression successful!');
        return result.results;
    } catch (error) {
        console.error('Batch compression failed:', error);
        throw error;
    }
}

async function decompressFile(compressedData, algorithm = Algorithm.Auto) {
    try {
        const wasm = await initWasm();
        const result = wasm.decompress(compressedData, algorithm);
        console.log('Decompression successful!');
        return result;
    } catch (error) {
        console.error('Decompression failed:', error);
        throw error;
    }
}

async function decompressFiles(files, algorithm = Algorithm.Auto) {
    try {
        const wasm = await initWasm();
        const fileArray = [];
        
        for (const file of files) {
            const arrayBuffer = await file.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);
            fileArray.push({
                name: file.name,
                data: uint8Array
            });
        }
        
        const results = wasm.decompress_batch(fileArray, algorithm);
        console.log('Batch decompression successful!');
        return results;
    } catch (error) {
        console.error('Batch decompression failed:', error);
        throw error;
    }
}

// Example usage:
async function example() {
    // Compress a file
    const fileInput = document.getElementById('fileInput');
    const file = fileInput.files[0];
    
    // Compress with auto algorithm selection
    const compressed = await compressFile(file);
    
    // Or specify an algorithm
    const compressedWithRLE = await compressFile(file, Algorithm.RLE);
    
    // Decompress
    const decompressed = await decompressFile(compressed);
    
    // Create a download link for the compressed file
    const blob = new Blob([compressed], { type: 'application/octet-stream' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'compressed.bin';
    a.click();
}

// Export functions for use in other modules
export { compressFile, compressFiles, decompressFile, decompressFiles, Algorithm }; 