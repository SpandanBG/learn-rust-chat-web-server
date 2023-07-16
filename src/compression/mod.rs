mod constants;

use self::constants::{COMPRESSION_TYPE_GZIP, COMPRESSION_TYPE_NONE, MIN_SIZE_TO_COMPRESS};

use flate2::{write::GzEncoder, Compression};
use std::io::Write;

#[derive(Copy, Clone)]
pub enum CompressionType {
    None,
    GZip,
}

impl CompressionType {
    pub fn to_string(&self) -> String {
        String::from(match self {
            CompressionType::None => COMPRESSION_TYPE_NONE,
            CompressionType::GZip => COMPRESSION_TYPE_GZIP,
        })
    }
}

pub struct CompressedData {
    pub data: Vec<u8>,
    pub compressed_type: CompressionType,
}

impl CompressedData {
    pub fn new(data: &Vec<u8>) -> CompressedData {
        if data.len() < MIN_SIZE_TO_COMPRESS {
            return CompressedData {
                data: data.clone(),
                compressed_type: CompressionType::None,
            };
        }

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

        let encoding_body_result = encoder.write_all(data);
        if encoding_body_result.is_err() {
            panic!("Error occured while encoding body")
        }

        let data = match encoder.finish() {
            Ok(compressed_response) => compressed_response,
            Err(error) => panic!(
                "Error occured while finishing encoding response => {:.2?}",
                error
            ),
        };

        CompressedData {
            data: data,
            compressed_type: CompressionType::GZip,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
