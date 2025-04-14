pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    if input.is_empty() {
        return compressed;
    }

    let mut current = input[0];
    let mut count = 1u8;

    for &byte in &input[1..] {
        if byte == current && count < 255 {
            count += 1;
        } else {
            compressed.push(count);
            compressed.push(current);
            current = byte;
            count = 1;
        }
    }

    // Add the last run
    compressed.push(count);
    compressed.push(current);

    compressed
}

pub fn decompress(compressed: &[u8]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut i = 0;
    let len = compressed.len();

    while i + 1 < len {
        let count = compressed[i] as usize;
        let byte = compressed[i + 1];
        decompressed.extend(std::iter::repeat(byte).take(count));
        i += 2;
    }

    decompressed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_empty_input() {
        let input = b"";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_single_char() {
        let input = b"A";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_max_repetition() {
        let input = vec![65u8; 255]; // 255 A's
        let compressed = compress(&input);
        let decompressed = decompress(&compressed);
        assert_eq!(input, decompressed);
    }

    #[test]
    fn test_binary_data() {
        let input = vec![0u8, 0u8, 0u8, 1u8, 1u8, 2u8, 2u8, 2u8];
        let compressed = compress(&input);
        let decompressed = decompress(&compressed);
        assert_eq!(input, decompressed);
    }
}
