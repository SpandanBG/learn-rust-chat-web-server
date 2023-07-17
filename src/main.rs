mod cache;
mod compression;
mod request;
mod resource;
mod response;
mod ssl;

use crate::{
    cache::Cache,
    compression::CompressedData,
    request::Request,
    resource::Resource,
    response::{headers::Headers, ResponseHandler},
    ssl::SSL,
};

use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    task,
};

const SERVER_ADDR: &'static str = "127.0.0.1:443";

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let ssl_handler = SSL::new();
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    let shared_cache = Cache::new();

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to build TCP socket connection with client");
        let shared = Arc::clone(&shared_cache);
        task::spawn(handle_connection(stream, shared, ssl_handler.clone()));
    }
}

async fn handle_connection(stream: TcpStream, shared: Arc<Cache>, ssl_handler: SSL) {
    let mut tls_stream = ssl_handler.get_tls_stream(stream).await;
    let request = Request::new(&mut tls_stream).await;
    let mut response: ResponseHandler = ResponseHandler::new(tls_stream);
    respond(&request, &mut response, shared).await;
}

async fn respond(request: &Request, response_handler: &mut ResponseHandler, shared: Arc<Cache>) {
    if let Some(response) = shared.get_data(&request.path) {
        response_handler.write(&response).await;
        return ();
    }

    let resource = Resource::new(&request.path);
    if resource.is_none() {
        return;
    }
    let resource = resource.unwrap();

    let response_body = CompressedData::new(&resource.data);
    let response_headers = Headers::new(
        response_body.compressed_type,
        resource.resource_type.get_mime_type(),
        response_body.len(),
    );

    let response = response_handler.build_response(&response_body.data, &response_headers);
    response_handler.write(&response).await;
    let _ = shared.async_set_data(request.path.clone(), response);
}
