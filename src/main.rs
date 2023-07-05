use flate2::{write::GzEncoder, Compression};
use threadpool::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Instant,
};

const HTTP_VERSION: &'static str = "HTTP/2.0";
const SERVER_ADDR: &'static str = "127.0.0.1:8080";
const RESOURCE_DIRECTORY: &'static str = "res";
const OK_200_STATUS: &'static str = "200 OK";
const ROOT_PATH: &'static str = "/";
const INDEX_FILE: &'static str = "/index.html";
const NUMBER_OF_THREADS: usize = 16;

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

fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).unwrap();
    let pool = ThreadPool::new(NUMBER_OF_THREADS);

    for maybe_stream in listener.incoming() {
        let now = Instant::now();
        match maybe_stream {
            Ok(stream) => { pool.execute(move || handle_connection(stream, now)); ()},
            Err(error) => println!("Error occured with a connection => {:.2?}", error),
        }
    }
}

fn handle_connection(stream: TcpStream, now: Instant) {
    let request_path = respond(stream);
    let elapsed = now.elapsed();
    println!("For {} => Elapsed: {:.2?}", request_path, elapsed);
}

fn respond(mut stream: TcpStream) -> String {
    let buf_reader = BufReader::new(&stream);

    let request_path = get_request_path(buf_reader);
    if request_path.is_none() {
        return String::new();
    }
    let request_path = request_path.unwrap();

    let content = get_contents(&request_path);
    if content.is_none() {
        return request_path;
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

    match stream.write_all(&response) {
        Err(error) => println!("Error occured will writing to stream: {:.2?}", error),
        _ => (),
    }

    request_path
}

fn get_request_path(buf_reader: BufReader<&TcpStream>) -> Option<String> {
    let request_line = buf_reader.lines().next();
    if request_line.is_none() {
        return None;
    }

    let request_line = request_line.unwrap();
    if request_line.is_err() {
        return None;
    }

    let request_line = request_line.unwrap();
    let mut request_line = request_line.split_whitespace().skip(1);

    if let Some(path) = request_line.next() {
        return Some(path.to_owned());
    }
    None
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
