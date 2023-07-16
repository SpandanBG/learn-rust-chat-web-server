use crate::compression::CompressionType;
use crate::response::{constants::HTTP_VERSION, status::Status};

pub struct Headers {
    compression_type: String,
    content_type: String,
    content_length: String,
}

impl Headers {
    pub fn new(
        compression_type: CompressionType,
        content_type: String,
        content_length: usize,
    ) -> Headers {
        Headers {
            compression_type: compression_type.to_string(),
            content_type,
            content_length: content_length.to_string(),
        }
    }

    pub fn as_bytes(&self, status: Status) -> Vec<u8> {
        vec![
            HTTP_VERSION, " ", &status.status_code, "\r\n",
            "Content-Encoding: ", &self.compression_type, "\r\n",
            "Content-Type: ", &self.content_type, "\r\n",
            "Content-Length: ", &self.content_length, "\r\n\r\n",
        ].concat().as_bytes().to_owned()
    }
}
