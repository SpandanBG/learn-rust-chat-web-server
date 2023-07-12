mod constants;

use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};
use constants::{DEFAULT_REQUEST_METHOD, DEFAULT_REQUEST_PATH};

pub struct Request {
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Request {
        let buf_reader = BufReader::new(stream);

        let http_request_raw: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut request_iter = http_request_raw.iter();

        let request_path = request_iter.next();
        if request_path.is_none() {
            println!("could not extract request path from request");
            return Request::default();
        }
        let request_path = request_path.unwrap();

        let mut request_path_iter = request_path.split_whitespace();

        Request {
            method: String::from(
                request_path_iter
                    .next()
                    .unwrap_or(DEFAULT_REQUEST_METHOD),
            ),
            path: String::from(
                request_path_iter
                    .next()
                    .unwrap_or(DEFAULT_REQUEST_PATH),
            ),
        }
    }

    fn default() -> Request {
        Request {
            method: String::from(DEFAULT_REQUEST_METHOD),
            path: String::from(DEFAULT_REQUEST_PATH),
        }
    }
}
