class Lz77Token {
    constructor(offset, length, nextChar) {
        this.offset = offset;
        this.length = length;
        this.nextChar = nextChar;
    }
}

function compress(input) {
    const output = [];
    let pos = 0;
    const windowSize = 10; // Adjust window size as needed

    while (pos < input.length) {
        const [bestOffset, bestLength] = findLongestMatch(input, pos, windowSize);
        
        if (bestLength > 0) {
            const nextChar = pos + bestLength < input.length ? input[pos + bestLength] : 0;
            output.push(new Lz77Token(bestOffset, bestLength, nextChar));
            pos += bestLength + 1;
        } else {
            output.push(new Lz77Token(0, 0, input[pos]));
            pos += 1;
        }
    }

    return output;
}

function findLongestMatch(input, pos, windowSize) {
    const start = pos > windowSize ? pos - windowSize : 0;
    const maxPos = Math.min(input.length, pos + 255); // Limit match length
    
    let bestOffset = 0;
    let bestLength = 0;
    
    for (let i = start; i < pos; i++) {
        let currentLength = 0;
        
        while (pos + currentLength < maxPos && 
               i + currentLength < pos && 
               input[i + currentLength] === input[pos + currentLength]) {
            currentLength += 1;
        }
        
        if (currentLength > bestLength) {
            bestLength = currentLength;
            bestOffset = pos - i;
        }
    }
    
    return [bestOffset, bestLength];
}

function decompress(tokens) {
    const output = [];
    
    for (const token of tokens) {
        if (token.length > 0) {
            const start = output.length - token.offset;
            for (let i = 0; i < token.length; i++) {
                output.push(output[start + i]);
            }
        }
        if (token.nextChar !== 0) {
            output.push(token.nextChar);
        }
    }
    
    return Buffer.from(output);
}

// Export for testing
module.exports = {
    compress,
    decompress,
    Lz77Token // Exporting for testing purposes
};