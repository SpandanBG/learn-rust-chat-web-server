use crate::response::status::Status;
use crate::response::constants::HTTP_VERSION;

pub struct Headers {
    content_type: String,
    content_length: String,
}

impl Headers {
    pub fn new(content_type: String, content_length: usize) -> Headers {
        Headers {
            content_type,
            content_length: content_length.to_string(),
        }
    }

    pub fn as_bytes(&self, status: Status) -> Vec<u8> {
        vec![
            HTTP_VERSION, " ", &status.status_code, "\r\n",
            "Connection: keep-alive\r\n",
            "Content-Encoding: gzip\r\n",
            "Content-Type: ", &self.content_type, "\r\n",
            "Content-Length: ", &self.content_length, "\r\n\r\n",
        ].concat().as_bytes().to_owned()
    }
}
