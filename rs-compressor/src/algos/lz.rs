const WINDOW_SIZE: usize = 4096;
const MAX_MATCH: usize = 18;

pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut i = 0;
    
    while i < input.len() {
        let mut best_match = (0, 0);
        
        // Search for the longest match in the sliding window
        let start = if i > WINDOW_SIZE { i - WINDOW_SIZE } else { 0 };
        for j in start..i {
            let mut match_len = 0;
            while i + match_len < input.len() 
                  && input[j + match_len] == input[i + match_len] 
                  && match_len < MAX_MATCH {
                match_len += 1;
            }
            
            if match_len > best_match.1 {
                best_match = (i - j, match_len);
            }
        }
        
        if best_match.1 >= 3 {
            // Encode as a match
            output.push(((best_match.0 >> 4) & 0xF0) as u8 | (best_match.1 - 3) as u8);
            output.push((best_match.0 & 0xFF) as u8);
            i += best_match.1;
        } else {
            // Encode as a literal
            output.push(0);
            output.push(input[i]);
            i += 1;
        }
    }
    
    output
}

pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut i = 0;
    
    while i + 1 < input.len() {
        let flag = input[i];
        
        if flag == 0 {
            // Literal
            output.push(input[i + 1]);
            i += 2;
        } else {
            // Match
            let length = ((flag & 0x0F) + 3) as usize;
            let offset = ((flag as usize & 0xF0) << 4) | (input[i + 1] as usize);
            
            let start = output.len() - offset;
            for j in 0..length {
                output.push(output[start + j]);
            }
            
            i += 2;
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
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_compress() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress(input);
        assert_eq!(compressed.len(), 9);
    }

    #[test]
    fn test_decompress() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}