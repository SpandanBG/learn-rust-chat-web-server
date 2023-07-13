mod cache;
mod request;

use crate::{cache::Cache, request::Request};
use flate2::{write::GzEncoder, Compression};
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
    time::Instant,
};
use tokio;

const HTTP_VERSION: &'static str = "HTTP/2.0";
const SERVER_ADDR: &'static str = "127.0.0.1:8080";
const RESOURCE_DIRECTORY: &'static str = "res";
const OK_200_STATUS: &'static str = "200 OK";
const ROOT_PATH: &'static str = "/";
const INDEX_FILE: &'static str = "/index.html";

struct Headers {
    content_type: String,
    content_length: usize,
}

impl Headers {
    fn to_string(&self) -> String {
        format!(
            "Content-Encoding: gzip\r\nContent-Type: {}\r\nContent-Length: {}\r\n",
            self.content_type, self.content_length
        )
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).unwrap();
    let shared_cache = Cache::new();

    for maybe_stream in listener.incoming() {
        let now = Instant::now();
        let shared = Arc::clone(&shared_cache);
        match maybe_stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream, now, shared);
                });
                ()
            }
            Err(error) => println!("Error occured with a connection => {:.2?}", error),
        }
    }
}

fn handle_connection(mut stream: TcpStream, now: Instant, shared: Arc<Cache>) {
    let request = Request::new(&mut stream);
    let request_path = respond(stream, &request, shared);
    let elapsed = now.elapsed();
    println!("For {} => Elapsed: {:.2?}", request_path, elapsed);
}

fn respond<'a>(mut stream: TcpStream, request: &'a Request, shared: Arc<Cache>) -> &'a String {
    if let Some(response) = shared.get_data(&request.path) {
        match stream.write_all(&response) {
            Err(error) => println!("Error occured will writing to stream: {:.2?}", error),
            _ => (),
        }
        return &request.path;
    }

    let content = get_contents(&request.path);
    if content.is_none() {
        return &request.path;
    }
    let (response_body, response_type) = content.unwrap();
    let response_body = gzip_response_body(&response_body);

    let status = format!("{} {}", HTTP_VERSION, OK_200_STATUS);
    let headers = Headers {
        content_length: response_body.len(),
        content_type: get_content_type(&response_type),
    }
    .to_string();

    let response = [
        format!("{status}\r\n{headers}\r\n").as_bytes().to_owned(),
        response_body,
    ]
    .concat();

    let _ = shared.async_set_data(request.path.clone(), response.clone()); 

    match stream.write_all(&response) {
        Err(error) => println!("Error occured will writing to stream: {:.2?}", error),
        _ => (),
    }

    &request.path
}

fn get_content_type(file_type: &str) -> String {
    match file_type {
        "html" => "text/html".to_owned(),
        "js" => "text/javascript".to_owned(),
        "css" => "text/css".to_owned(),
        "ico" => "image/x-icon".to_owned(),
        "xml" => "application/xml".to_owned(),
        _ => "text/plain".to_owned(),
    }
}

fn get_contents(filename: &str) -> Option<(Vec<u8>, String)> {
    let mut path = RESOURCE_DIRECTORY.clone().to_owned();
    path.push_str(match filename {
        ROOT_PATH => INDEX_FILE,
        _ => filename,
    });

    let file_type = path.split('.');
    let file_type = file_type.last().unwrap();

    match fs::read(&path) {
        Ok(file_content) => Some((file_content, file_type.to_owned())),
        Err(error_message) => {
            println!("For {} => {:?}", path, error_message);
            None
        }
    }
}

fn gzip_response_body(response_body: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    let encoding_body_result = encoder.write_all(response_body);
    if encoding_body_result.is_err() {
        panic!("Error occured while encoding body")
    }

    match encoder.finish() {
        Ok(compressed_response) => compressed_response,
        Err(error) => panic!(
            "Error occured while finishing encoding response => {:.2?}",
            error
        ),
    }
}
