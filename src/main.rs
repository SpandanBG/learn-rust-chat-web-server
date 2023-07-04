use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Instant,
};

const HTTP_VERSION: &'static str = "HTTP/2.0";
const SERVER_ADDR: &'static str = "127.0.0.1:8080";
const RESOURCE_DIRECTORY: &'static str = "res";
const OK_200_STATUS: &'static str = "200 OK";
const ROOT_PATH: &'static str = "/";
const INDEX_FILE: &'static str = "/index.html";

fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).unwrap();

    for maybe_stream in listener.incoming() {
        if let Ok(stream) = maybe_stream {
            thread::spawn(move || {
                let now = Instant::now();
                let request_path = handle_connection(stream);
                let elapsed = now.elapsed();
                println!("For {} => Elapsed: {:.2?}", request_path, elapsed);
            });
        } else if let Err(error) = maybe_stream {
            println!("Error occured with a connection => {:.2?}", error);
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> String {
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

    let status = format!("{} {}", HTTP_VERSION, OK_200_STATUS);
    let (contents, file_type) = content.unwrap();
    let length = contents.len();
    let content_type = get_content_type(&file_type);

    match stream.write_all(
        &[
            format!("{status}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n")
                .as_bytes()
                .to_owned(),
            contents,
        ]
        .concat(),
    ) {
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
