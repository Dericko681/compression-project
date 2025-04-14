use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

mod algos;
mod header;

use algos::le;
use algos::lz;
use algos::file_type::{detect_file_type, select_algorithm, Algorithm as FileAlgorithm};
use header::{Algorithm as HeaderAlgorithm, create_header, read_header};

fn print_usage() {
    println!("Usage: rs-compressor compress|decompress [input_file] [output_file] [--rle|--lz|--auto]");
    println!("Example: rs-compressor compress input.txt output.txt --rle");
    println!("Example with auto detection: rs-compressor compress input.txt output.txt --auto");
    println!("Example with stdin/stdout: cat input.txt | rs-compressor compress - - --auto > output.txt");
    std::process::exit(1);
}

fn read_input(input: &str) -> io::Result<Vec<u8>> {
    if input == "-" {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        Ok(buffer)
    } else {
        fs::read(input)
    }
}

fn write_output(output: &str, data: &[u8]) -> io::Result<()> {
    if output == "-" {
        io::stdout().write_all(data)?;
        Ok(())
    } else {
        fs::write(output, data)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        print_usage();
    }

    let operation = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];
    let algorithm = &args[4];

    let input_data = read_input(input_file)?;

    let result = match operation.as_str() {
        "compress" => {
            let selected_algorithm = if algorithm == "--auto" {
                let file_type = detect_file_type(&input_data);
                select_algorithm(file_type, &input_data)
            } else if algorithm == "--rle" {
                FileAlgorithm::RLE
            } else if algorithm == "--lz" {
                FileAlgorithm::LZ
            } else {
                eprintln!("Invalid algorithm. Use --rle, --lz, or --auto");
                std::process::exit(1);
            };

            let compressed_data = match selected_algorithm {
                FileAlgorithm::RLE => {
                    println!("Using RLE compression");
                    le::compress(&input_data)
                },
                FileAlgorithm::LZ => {
                    println!("Using LZ compression");
                    lz::compress(&input_data)
                }
            };

            // Add header to compressed data
            let header = create_header(match selected_algorithm {
                FileAlgorithm::RLE => HeaderAlgorithm::RLE,
                FileAlgorithm::LZ => HeaderAlgorithm::LZ,
            });
            
            [&header[..], &compressed_data[..]].concat()
        }
        "decompress" => {
            let (decompress_algorithm, data) = if algorithm == "--auto" {
                match read_header(&input_data) {
                    Ok(alg) => {
                        println!("Detected algorithm: {:?}", alg);
                        (alg, &input_data[8..])
                    },
                    Err(e) => {
                        eprintln!("Error reading header: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if algorithm == "--rle" {
                (HeaderAlgorithm::RLE, &input_data[..])
            } else if algorithm == "--lz" {
                (HeaderAlgorithm::LZ, &input_data[..])
            } else {
                eprintln!("Invalid algorithm. Use --rle, --lz, or --auto");
                std::process::exit(1);
            };

            match decompress_algorithm {
                HeaderAlgorithm::RLE => le::decompress(data),
                HeaderAlgorithm::LZ => lz::decompress(data),
            }
        }
        _ => {
            eprintln!("Invalid operation. Use compress or decompress");
            std::process::exit(1);
        }
    };

    write_output(output_file, &result)?;
    Ok(())
}