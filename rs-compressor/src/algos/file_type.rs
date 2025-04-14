use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Text,
    Binary,
    Image,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum Algorithm {
    RLE,
    LZ,
}

pub fn detect_file_type(data: &[u8]) -> FileType {
    // Check for common text file characteristics
    let is_text = data.iter().all(|&b| b.is_ascii() || b == b'\n' || b == b'\r' || b == b'\t');
    if is_text {
        return FileType::Text;
    }

    // Check for common image file signatures
    if data.len() >= 8 {
        let header = &data[0..8];
        if header.starts_with(b"\x89PNG\r\n\x1a\n") || 
           header.starts_with(b"\xFF\xD8\xFF") || // JPEG
           header.starts_with(b"GIF87a") || 
           header.starts_with(b"GIF89a") {
            return FileType::Image;
        }
    }

    // If not text or image, consider it binary
    FileType::Binary
}

pub fn select_algorithm(file_type: FileType, data: &[u8]) -> Algorithm {
    match file_type {
        FileType::Text => {
            // For text files, use LZ as it's better for patterns
            Algorithm::LZ
        },
        FileType::Binary => {
            // For binary files, check if RLE would be more efficient
            let rle_ratio = calculate_rle_ratio(data);
            if rle_ratio > 0.5 {
                Algorithm::RLE
            } else {
                Algorithm::LZ
            }
        },
        FileType::Image => {
            // For images, use RLE as they often have repeated pixels
            Algorithm::RLE
        },
        FileType::Unknown => {
            // Default to LZ for unknown types
            Algorithm::LZ
        }
    }
}

fn calculate_rle_ratio(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut current_byte = data[0];
    let mut current_count = 1;
    let mut total_runs = 1;
    let mut total_bytes = 1;

    for &byte in &data[1..] {
        if byte == current_byte {
            current_count += 1;
        } else {
            current_byte = byte;
            current_count = 1;
            total_runs += 1;
        }
        total_bytes += 1;
    }

    // Calculate compression ratio (lower is better)
    (total_runs * 2) as f64 / total_bytes as f64
} 