function decode_utf8(s) {
  return decodeURIComponent(escape(s));
}

function compress(input) {
  // Handle both Buffer and string input
  const data = Buffer.isBuffer(input) ? input : Buffer.from(input);
  const output = [];
  let i = 0;
  
  while (i < data.length) {
    let count = 1;
    const current = data[i];
    
    while (i + count < data.length && data[i + count] === current && count < 255) {
      count++;
    }
    
    output.push(count);
    output.push(current);
    i += count;
  }
  
  return Buffer.from(output);
}

function decompress(input) {
  // Handle both Buffer and string input
  const data = Buffer.isBuffer(input) ? input : Buffer.from(input);
  const output = [];
  let i = 0;
  
  while (i + 1 < data.length) {
    const count = data[i];
    const byte = data[i + 1];
    
    for (let j = 0; j < count; j++) {
      output.push(byte);
    }
    
    i += 2;
  }
  
  return Buffer.from(output);
}

module.exports = { compress, decompress };
