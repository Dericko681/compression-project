function decode_utf8(s) {
  return decodeURIComponent(escape(s));
}

function compress(input) {
  // Convert Buffer to string if needed
  const string = Buffer.isBuffer(input) ? input.toString('utf8') : input;
  let encoded = [];
  let count = 1;
  
  for (let i = 0; i < string.length; i++) {
    if (string[i] === string[i + 1] && count < 9) {
      count++;
    } else {
      if (count > 1) {
        encoded.push(string[i] + count);
        count = 1;
      } else {
        encoded.push(string[i]);
      }
    }
  }

  return encoded.join("");
}

function decompress(input) {
  // Convert Buffer to string if needed
  const string = Buffer.isBuffer(input) ? input.toString('utf8') : input;
  let decoded = [];
  
  for (let i = 0; i < string.length; i++) {
    if (isNaN(string[i]) && !isNaN(string[i + 1])) {
      for (let j = 0; j < string[i + 1]; j++) {
        decoded.push(string[i]);
      }
      i = i + 1;
    } else {
      decoded.push(string[i]);
    }
  }
  
  return Buffer.from(decoded.join(""), 'utf8');
}

module.exports = { compress, decompress };
