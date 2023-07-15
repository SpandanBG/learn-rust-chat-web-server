pub mod headers;

mod constants;
mod status;

use crate::Headers;
use crate::response::status::Status;
use std::{io::Error, io::Write, net::TcpStream};

pub struct ResponseHandler<'a> {
    stream: &'a TcpStream,
}

impl<'a> ResponseHandler<'a> {
    pub fn new(stream: &'a TcpStream) -> ResponseHandler {
        ResponseHandler { stream }
    }

    pub fn build_response(&self, data: &Vec<u8>, headers: &Headers) -> Vec<u8> {
        [
            Status::new(String::from(constants::OK_200_STATUS)).as_bytes(),
            headers.as_bytes(),
            data.clone(),
        ]
        .concat()
    }

    pub fn write(&mut self, data: &Vec<u8>) {
        let write_status = self.stream.write_all(&data);
        let _ = ResponseHandler::log(write_status);
    }

    async fn log(write_status: Result<(), Error>) {
        match write_status {
            Err(err) => {
                print!("error occured on sending data over http => {:.2?}", err);
                ()
            }
            _ => (),
        };
    }
}
