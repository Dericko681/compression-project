function decode_utf8(s) {
  return decodeURIComponent(escape(s));
}

function compress(string) {
  let encoded = [];
  let count = 1;
  string = decode_utf8(string);
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

function decompress(string) {
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
  return decoded.join("");
}

module.exports = { compress, decompress };
