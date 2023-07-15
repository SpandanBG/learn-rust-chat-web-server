mod cache;
mod compression;
mod request;
mod response;

use crate::{
    cache::Cache,
    compression::CompressedData,
    request::Request,
    response::{headers::Headers, ResponseHandler},
};

use std::{fs, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    task,
};

const SERVER_ADDR: &'static str = "127.0.0.1:8080";
const RESOURCE_DIRECTORY: &'static str = "res";
const ROOT_PATH: &'static str = "/";
const INDEX_FILE: &'static str = "/index.html";

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    let shared_cache = Cache::new();

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to build TCP socket connection with client");
        let shared = Arc::clone(&shared_cache);
        task::spawn(handle_connection(stream, shared));
    }
}

async fn handle_connection(mut stream: TcpStream, shared: Arc<Cache>) {
    let request = Request::new(&mut stream).await;
    let mut response = ResponseHandler::new(stream);
    respond(&request, &mut response, shared).await;
}

async fn respond(request: &Request, response_handler: &mut ResponseHandler, shared: Arc<Cache>) {
    if let Some(response) = shared.get_data(&request.path) {
        response_handler.write(&response).await;
        return ();
    }

    let content = get_contents(&request.path);
    if content.is_none() {
        return;
    }

    let (response_body, response_type) = content.unwrap();
    let response_body = CompressedData::new(&response_body);
    let response_headers = Headers::new(
        response_body.compressed_type,
        get_content_type(&response_type),
        response_body.len(),
    );

    let response = response_handler.build_response(&response_body.data, &response_headers);
    response_handler.write(&response).await;
    let _ = shared.async_set_data(request.path.clone(), response);
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
