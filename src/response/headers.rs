pub struct Headers {
    content_type: String,
    content_length: usize,
}

impl Headers {
    pub fn new(content_type: String, content_length: usize) -> Headers {
        Headers {
            content_type,
            content_length,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        format!(
            "Content-Encoding: gzip\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            self.content_type, self.content_length,
        )
        .as_bytes()
        .to_owned()
    }
}
