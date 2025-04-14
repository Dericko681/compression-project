const FileType = {
    TEXT: 'text',
    BINARY: 'binary',
    IMAGE: 'image',
    UNKNOWN: 'unknown'
};

const Algorithm = {
    RLE: 'rle',
    LZ: 'lz'
};

function detectFileType(data) {
    // Check if it's text (ASCII/UTF-8)
    const isText = data.every(byte => 
        (byte >= 32 && byte <= 126) || // Printable ASCII
        byte === 10 || // Newline
        byte === 13 || // Carriage return
        byte === 9     // Tab
    );
    if (isText) {
        return FileType.TEXT;
    }

    // Check for image file signatures
    if (data.length >= 8) {
        const header = data.slice(0, 8);
        if (header[0] === 0x89 && header[1] === 0x50 && header[2] === 0x4E && header[3] === 0x47 || // PNG
            header[0] === 0xFF && header[1] === 0xD8 && header[2] === 0xFF || // JPEG
            (header[0] === 0x47 && header[1] === 0x49 && header[2] === 0x46 && header[3] === 0x38 && // GIF
             (header[4] === 0x37 || header[4] === 0x39) && header[5] === 0x61)) {
            return FileType.IMAGE;
        }
    }

    return FileType.BINARY;
}

function calculateRleRatio(data) {
    if (data.length === 0) return 0;

    let currentByte = data[0];
    let currentCount = 1;
    let totalRuns = 1;
    let totalBytes = 1;

    for (let i = 1; i < data.length; i++) {
        if (data[i] === currentByte) {
            currentCount++;
        } else {
            currentByte = data[i];
            currentCount = 1;
            totalRuns++;
        }
        totalBytes++;
    }

    return (totalRuns * 2) / totalBytes;
}

function selectAlgorithm(fileType, data) {
    switch (fileType) {
        case FileType.TEXT:
            return Algorithm.LZ;
        case FileType.BINARY:
            const rleRatio = calculateRleRatio(data);
            return rleRatio > 0.5 ? Algorithm.RLE : Algorithm.LZ;
        case FileType.IMAGE:
            return Algorithm.RLE;
        default:
            return Algorithm.LZ;
    }
}

module.exports = {
    FileType,
    Algorithm,
    detectFileType,
    selectAlgorithm
}; 