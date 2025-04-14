const MAGIC_NUMBER: &[u8] = b"RSCMP";
const VERSION: u8 = 1;

#[derive(Debug)]
pub enum Algorithm {
    RLE,
    LZ,
}

pub fn create_header(algorithm: Algorithm) -> Vec<u8> {
    let mut header = Vec::with_capacity(8);
    header.extend_from_slice(MAGIC_NUMBER);
    header.push(VERSION);
    header.push(match algorithm {
        Algorithm::RLE => 1,
        Algorithm::LZ => 2,
    });
    header.push(0); // Reserved byte
    header
}

pub fn read_header(data: &[u8]) -> Result<Algorithm, &'static str> {
    if data.len() < 8 {
        return Err("Invalid compressed file format");
    }

    if &data[0..5] != MAGIC_NUMBER {
        return Err("Invalid magic number");
    }

    if data[5] != VERSION {
        return Err("Unsupported version");
    }

    match data[6] {
        1 => Ok(Algorithm::RLE),
        2 => Ok(Algorithm::LZ),
        _ => Err("Invalid algorithm identifier"),
    }
} 