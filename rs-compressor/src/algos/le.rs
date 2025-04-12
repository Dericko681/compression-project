pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    if input.is_empty() {
        return compressed;
    }

    let mut current = input[0];
    let mut count = 1u8;

    for &byte in &input[1..] {
        if byte == current && count < 9 {
            count += 1;
        } else {
            compressed.push(current);
            if count > 1 {
                compressed.push(count + b'0');
            }
            current = byte;
            count = 1;
        }
    }

    compressed.push(current);
    if count > 1 {
        compressed.push(count + b'0');
    }

    compressed
}

pub fn decompress(compressed: &[u8]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut i = 0;
    let len = compressed.len();

    while i < len {
        let byte = compressed[i];
        i += 1;

        if i < len && compressed[i].is_ascii_digit() {
            let count = (compressed[i] - b'0') as usize;
            decompressed.extend(std::iter::repeat(byte).take(count));
            i += 1;
        } else {
            decompressed.push(byte);
        }
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
        let input = b"AAAAAAAAA"; // 9 A's
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
