pub mod headers;

mod constants;
mod status;

use crate::response::constants::OK_200_STATUS;
use crate::response::status::Status;
use crate::Headers;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub struct ResponseHandler {
    stream: TcpStream,
}

impl ResponseHandler {
    pub fn new(stream: TcpStream) -> ResponseHandler {
        ResponseHandler { stream }
    }

    pub fn build_response(&self, data: &Vec<u8>, headers: &Headers) -> Vec<u8> {
        [
            headers.as_bytes(Status::new(String::from(OK_200_STATUS))),
            data.clone(),
        ]
        .concat()
    }

    pub async fn write(&mut self, data: &Vec<u8>) {
        self.stream.write_all(&data).await.expect("could not write response to stream");
        self.stream.shutdown().await.expect("could not close connection");
    }
}
