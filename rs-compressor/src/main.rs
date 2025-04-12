use std::env::args;
use std::fs;
use std::path::Path;
use std::process;

use algos::le::{compress, decompress};
use algos::lz::{lzCompress, lzDecompress};

mod algos;

fn print_usage() {
    println!("Usage: cargo run -- compress|decompress <input_file> <output_file> --rle|--lz");
    println!("Example: cargo run -- compress input.txt output.txt --rle");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = args().collect();
    let args = if args.len() >= 2 && args[1] == "run" {
        &args[3..]
    } else {
        &args[1..]
    };

    if args.len() != 4 {
        print_usage();
    }

    let operation = &args[0];
    let input_file = &args[1];
    let output_file = &args[2];
    let algorithm = &args[3];

    if !Path::new(input_file).exists() {
        eprintln!("Error: Input file '{}' does not exist", input_file);
        process::exit(1);
    }

    let input_bytes = match fs::read(input_file) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            process::exit(1);
        }
    };

    let result = match (operation.as_str(), algorithm.as_str()) {
        ("compress", "--rle") => compress(&input_bytes),
        ("decompress", "--rle") => decompress(&input_bytes),
        ("compress", "--lz") => {
            let tokens = lzCompress(&input_bytes);
            let mut bytes = Vec::new();
            for token in tokens {
                bytes.extend_from_slice(&token.offset.to_le_bytes());
                bytes.extend_from_slice(&token.length.to_le_bytes());
                bytes.push(token.next_char);
            }
            bytes
        }
        ("decompress", "--lz") => {
            let mut tokens = Vec::new();
            let mut i = 0;
            while i + 9 <= input_bytes.len() {
                let offset = usize::from_le_bytes(input_bytes[i..i + 8].try_into().unwrap());
                let length = usize::from_le_bytes(input_bytes[i + 8..i + 16].try_into().unwrap());
                let next_char = input_bytes[i + 16];
                tokens.push(algos::lz::Lz77Token {
                    offset,
                    length,
                    next_char,
                });
                i += 17;
            }
            lzDecompress(&tokens[..])
        }
        _ => {
            eprintln!("Invalid operation or algorithm");
            print_usage();
            return;
        }
    };

    if let Err(e) = fs::write(output_file, &result) {
        eprintln!("Error writing output file: {}", e);
        process::exit(1);
    }

    println!("Operation completed successfully!");
    println!("Input size: {} bytes", input_bytes.len());
    println!("Output size: {} bytes", result.len());
    println!(
        "Compression ratio: {:.2}%",
        (result.len() as f64 / input_bytes.len() as f64) * 100.0
    );
}