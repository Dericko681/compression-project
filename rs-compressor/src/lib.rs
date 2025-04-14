use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, Array};
use web_sys::console;
use wasm_bindgen::JsCast;

mod algos;
mod header;

use algos::le;
use algos::lz;
use algos::file_type::{detect_file_type, select_algorithm, Algorithm as FileAlgorithm};
use header::{Algorithm as HeaderAlgorithm, create_header, read_header};

#[wasm_bindgen]
#[derive(Clone)]
pub enum Algorithm {
    RLE,
    LZ,
    Auto,
}

impl AsRef<JsValue> for Algorithm {
    fn as_ref(&self) -> &JsValue {
        let s = match self {
            Algorithm::RLE => "RLE",
            Algorithm::LZ => "LZ",
            Algorithm::Auto => "Auto",
        };
        unsafe { &*(&JsValue::from_str(s) as *const JsValue) }
    }
}

impl JsCast for Algorithm {
    fn instanceof(_val: &JsValue) -> bool {
        true
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        let s = val.as_string().unwrap();
        match s.as_str() {
            "RLE" => Self::RLE,
            "LZ" => Self::LZ,
            "Auto" => Self::Auto,
            _ => panic!("Invalid algorithm value"),
        }
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const Self) }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CompressionResult {
    data: Vec<u8>,
    algorithm: Algorithm,
    filename: String,
}

impl AsRef<JsValue> for CompressionResult {
    fn as_ref(&self) -> &JsValue {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("data"), &Uint8Array::from(&self.data[..])).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("algorithm"), &JsValue::from(self.algorithm.as_ref())).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("filename"), &JsValue::from_str(&self.filename)).unwrap();
        unsafe { &*(&obj as *const js_sys::Object as *const JsValue) }
    }
}

impl JsCast for CompressionResult {
    fn instanceof(_val: &JsValue) -> bool {
        true
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        let obj = val.dyn_into::<js_sys::Object>().unwrap();
        let data = js_sys::Reflect::get(&obj, &JsValue::from_str("data"))
            .unwrap()
            .dyn_into::<Uint8Array>()
            .unwrap()
            .to_vec();
        let algorithm = js_sys::Reflect::get(&obj, &JsValue::from_str("algorithm"))
            .unwrap()
            .as_string()
            .unwrap();
        let algorithm = match algorithm.as_str() {
            "RLE" => Algorithm::RLE,
            "LZ" => Algorithm::LZ,
            "Auto" => Algorithm::Auto,
            _ => panic!("Invalid algorithm value"),
        };
        let filename = js_sys::Reflect::get(&obj, &JsValue::from_str("filename"))
            .unwrap()
            .as_string()
            .unwrap();
        Self {
            data,
            algorithm,
            filename,
        }
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const Self) }
    }
}

#[wasm_bindgen]
pub struct BatchResult {
    results: Vec<CompressionResult>,
    errors: Vec<String>,
}

#[wasm_bindgen]
impl BatchResult {
    #[wasm_bindgen(getter)]
    pub fn results(&self) -> Array {
        let array = Array::new();
        for result in &self.results {
            array.push(&JsValue::from(result));
        }
        array
    }

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> Array {
        let array = Array::new();
        for error in &self.errors {
            array.push(&JsValue::from_str(error));
        }
        array
    }
}

#[wasm_bindgen]
pub fn compress(input: &[u8], algorithm: Algorithm, filename: &str) -> Result<CompressionResult, JsValue> {
    let selected_algorithm = match algorithm {
        Algorithm::Auto => {
            let file_type = detect_file_type(input);
            select_algorithm(file_type, input)
        },
        Algorithm::RLE => FileAlgorithm::RLE,
        Algorithm::LZ => FileAlgorithm::LZ,
    };

    let compressed_data = match selected_algorithm {
        FileAlgorithm::RLE => le::compress(input),
        FileAlgorithm::LZ => lz::compress(input),
    };

    // Add header to compressed data
    let header = create_header(match selected_algorithm {
        FileAlgorithm::RLE => HeaderAlgorithm::RLE,
        FileAlgorithm::LZ => HeaderAlgorithm::LZ,
    });

    let result = [&header[..], &compressed_data[..]].concat();

    Ok(CompressionResult {
        data: result,
        algorithm: match selected_algorithm {
            FileAlgorithm::RLE => Algorithm::RLE,
            FileAlgorithm::LZ => Algorithm::LZ,
        },
        filename: filename.to_string(),
    })
}

#[wasm_bindgen]
pub fn compress_batch(files: Array, algorithm: Algorithm) -> Result<BatchResult, JsValue> {
    let mut results = Vec::new();
    let mut errors = Vec::new();
    
    for i in 0..files.length() {
        let file = files.get(i);
        let js_file = match file.dyn_into::<js_sys::Object>() {
            Ok(obj) => obj,
            Err(_) => {
                errors.push(format!("Invalid file object at index {}", i));
                continue;
            }
        };
        
        let name = match js_sys::Reflect::get(&js_file, &JsValue::from_str("name")) {
            Ok(name) => name.as_string().unwrap_or_else(|| format!("file_{}", i)),
            Err(_) => {
                errors.push(format!("Missing name for file at index {}", i));
                continue;
            }
        };
            
        let data = match js_sys::Reflect::get(&js_file, &JsValue::from_str("data")) {
            Ok(data) => match data.dyn_into::<Uint8Array>() {
                Ok(arr) => arr.to_vec(),
                Err(_) => {
                    errors.push(format!("Invalid data for file {} at index {}", name, i));
                    continue;
                }
            },
            Err(_) => {
                errors.push(format!("Missing data for file {} at index {}", name, i));
                continue;
            }
        };
            
        match compress(&data, algorithm.clone(), &name) {
            Ok(result) => results.push(result),
            Err(e) => {
                errors.push(format!("Failed to compress file {}: {}", name, e.as_string().unwrap_or_default()));
            }
        }
    }
    
    Ok(BatchResult { results, errors })
}

#[wasm_bindgen]
pub fn decompress(input: &[u8], algorithm: Algorithm) -> Result<Uint8Array, JsValue> {
    let (decompress_algorithm, data) = match algorithm {
        Algorithm::Auto => {
            match read_header(input) {
                Ok(alg) => (alg, &input[8..]),
                Err(e) => return Err(JsValue::from_str(e)),
            }
        },
        Algorithm::RLE => (HeaderAlgorithm::RLE, input),
        Algorithm::LZ => (HeaderAlgorithm::LZ, input),
    };

    let decompressed = match decompress_algorithm {
        HeaderAlgorithm::RLE => le::decompress(data),
        HeaderAlgorithm::LZ => lz::decompress(data),
    };

    Ok(Uint8Array::from(&decompressed[..]))
}

#[wasm_bindgen]
pub fn decompress_batch(files: Array, algorithm: Algorithm) -> Result<Array, JsValue> {
    let results = Array::new();
    let mut errors = Vec::new();
    
    for i in 0..files.length() {
        let file = files.get(i);
        let js_file = match file.dyn_into::<js_sys::Object>() {
            Ok(obj) => obj,
            Err(_) => {
                errors.push(format!("Invalid file object at index {}", i));
                continue;
            }
        };
        
        let name = match js_sys::Reflect::get(&js_file, &JsValue::from_str("name")) {
            Ok(name) => name.as_string().unwrap_or_else(|| format!("file_{}", i)),
            Err(_) => {
                errors.push(format!("Missing name for file at index {}", i));
                continue;
            }
        };
            
        let data = match js_sys::Reflect::get(&js_file, &JsValue::from_str("data")) {
            Ok(data) => match data.dyn_into::<Uint8Array>() {
                Ok(arr) => arr.to_vec(),
                Err(_) => {
                    errors.push(format!("Invalid data for file {} at index {}", name, i));
                    continue;
                }
            },
            Err(_) => {
                errors.push(format!("Missing data for file {} at index {}", name, i));
                continue;
            }
        };
            
        match decompress(&data, algorithm.clone()) {
            Ok(decompressed) => {
                let result = js_sys::Object::new();
                js_sys::Reflect::set(&result, &JsValue::from_str("name"), &JsValue::from_str(&name))?;
                js_sys::Reflect::set(&result, &JsValue::from_str("data"), &decompressed)?;
                results.push(&result);
            },
            Err(e) => {
                errors.push(format!("Failed to decompress file {}: {}", name, e.as_string().unwrap_or_default()));
            }
        }
    }
    
    if !errors.is_empty() {
        console::error_1(&JsValue::from_str(&format!("Batch operation completed with {} errors", errors.len())));
    }
    
    Ok(results)
} 