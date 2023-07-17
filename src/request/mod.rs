mod constants;

use constants::{DEFAULT_REQUEST_METHOD, DEFAULT_REQUEST_PATH};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
};
use tokio_rustls::server::TlsStream;

pub struct Request {
    pub method: String,
    pub path: String,
}

impl Request {
    pub async fn new(stream: &mut TlsStream<TcpStream>) -> Request {
        let buf_reader = BufReader::new(stream);

        let request_path = buf_reader.lines().next_line().await.unwrap_or(None);
        if request_path.is_none() {
            println!("could not extract request path from request");
            return Request::default();
        }
        let request_path = request_path.unwrap();

        let mut request_path_iter = request_path.split_whitespace();

        Request {
            method: String::from(request_path_iter.next().unwrap_or(DEFAULT_REQUEST_METHOD)),
            path: String::from(request_path_iter.next().unwrap_or(DEFAULT_REQUEST_PATH)),
        }
    }

    fn default() -> Request {
        Request {
            method: String::from(DEFAULT_REQUEST_METHOD),
            path: String::from(DEFAULT_REQUEST_PATH),
        }
    }
}
