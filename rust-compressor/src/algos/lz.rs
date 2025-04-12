#[derive(Debug, PartialEq)]
struct Lz77Token {
    offset: usize,
    length: usize,
    next_char: u8,
}

fn lzCompress(input: &[u8]) -> Vec<Lz77Token> {
    let mut output = Vec::new();
    let mut pos = 0;
    let window_size = 10;

    while pos < input.len() {
        let (best_offset, best_length) = find_longest_match(input, pos, window_size);
        
        if best_length > 0 {
            let next_char = if pos + best_length < input.len() {
                input[pos + best_length]
            } else {
                0 
            };
            output.push(Lz77Token {
                offset: best_offset,
                length: best_length,
                next_char,
            });
            pos += best_length + 1;
        } else {
            output.push(Lz77Token {
                offset: 0,
                length: 0,
                next_char: input[pos],
            });
            pos += 1;
        }
    }

    output
}

fn find_longest_match(input: &[u8], pos: usize, window_size: usize) -> (usize, usize) {
    let start = if pos > window_size { pos - window_size } else { 0 };
    let max_pos = input.len().min(pos + 255); // Limit match length
    
    let mut best_offset = 0;
    let mut best_length = 0;
    
    for i in start..pos {
        let mut current_length = 0;
        
        while pos + current_length < max_pos 
            && i + current_length < pos 
            && input[i + current_length] == input[pos + current_length] {
            current_length += 1;
        }
        
        if current_length > best_length {
            best_length = current_length;
            best_offset = pos - i;
        }
    }
    
    (best_offset, best_length)
}

fn lzDecompress(tokens: &[Lz77Token]) -> Vec<u8> {
    let mut output = Vec::new();
    
    for token in tokens {
        if token.length > 0 {
            let start = output.len() - token.offset;
            for i in 0..token.length {
                output.push(output[start + i]);
            }
        }
        if token.next_char != 0 {
            output.push(token.next_char);
        }
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = lzCompress(input);
        let decompressed = lzDecompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_compress() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = lzCompress(input);
        assert_eq!(compressed.len(), 9);
    }

    #[test]
    fn test_decompress() {
        let tokens = vec![
            Lz77Token { offset: 0, length: 0, next_char: b'A' },
            Lz77Token { offset: 1, length: 2, next_char: b'B' },
            Lz77Token { offset: 1, length: 2, next_char: b'C' },
            Lz77Token { offset: 1, length: 4, next_char: b'D' },
            Lz77Token { offset: 1, length: 3, next_char: b'E' },
        ];
        let decompressed = lzDecompress(&tokens);
        assert_eq!(decompressed, b"AAABBBCCCCCDDDDE");
    }
}