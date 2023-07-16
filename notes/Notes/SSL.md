## Creating SSL certs in Windows

- `choco install openssl`
- `openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout privatekey.key -out certificate.crt`

## SSL in Rust with Tokio

#### Packages

Add the following packages

```toml
[dependencies]
tokio = { version = "1.", features=["full"] }
tokio-rustls = "0.24.1"
rustls = "0.21.5"
```

The `rustls` package will provide us with the SSL/TLS features, while the `tokio-rustls` will provide async capabilities to the `rustls` package.

#### Code

```rust
use std::fs;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::{server::TlsStream, TlsAcceptor};
use rustls::{
	NoClientAuth, ServerConfig, ServerSession, Session, TLSError, HandshakeError, 
}

#[tokio::main]
async fn main() {
    // Load the server's private key and certificate chain
    let private_key = fs::read("path/to/private_key.pem").expect("Failed to read private key");
    let cert_chain = fs::read("path/to/cert_chain.pem").expect("Failed to read certificate chain");

    // Create a Rustls server configuration
    let mut config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    let cert_chain = rustls::internal::pemfile::certs(&mut cert_chain.as_slice())
        .expect("Failed to parse certificate chain");
    let private_key = rustls::internal::pemfile::pkcs8_private_keys(&mut private_key.as_slice())
        .expect("Failed to parse private key");

    config
        .set_single_cert(cert_chain, private_key[0].clone())
        .expect("Failed to set certificate and private key");

    // Create a TlsAcceptor from the server configuration
    let acceptor = TlsAcceptor::from(Arc::new(config));

    // Bind the TCP listener to a specific address and port
    let listener = TcpListener::bind("127.0.0.1:8443").await.expect("Failed to bind TCP listener");

    println!("Server started and listening on 127.0.0.1:8443");

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                // Spawn a new task to handle the client connection
                tokio::spawn(handle_client(socket, acceptor.clone()));
            }
            Err(e) => {
                eprintln!("Failed to accept client connection: {}", e);
            }
        }
    }
}

async fn handle_client(socket: tokio::net::TcpStream, acceptor: TlsAcceptor) {
    match acceptor.accept(socket).await {
        Ok(tls_stream) => {
            if let Err(e) = process_tls_stream(tls_stream).await {
                eprintln!("Error processing TLS stream: {}", e);
            }
        }
        Err(e) => {
            eprintln!("TLS handshake failed: {}", e);
        }
    }
}

async fn process_tls_stream(mut tls_stream: TlsStream<tokio::net::TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    let (mut reader, mut writer) = tls_stream.split();

    // Process the TLS-encrypted stream
    let mut buf = [0u8; 1024];
    loop {
        let bytes_read = reader.read(&mut buf).await?;
        if bytes_read == 0 {
            break; // Connection closed by the client
        }

        // Process the received data
        // ...

        // Echo the data back to the client
        writer.write_all(&buf[..bytes_read]).await?;
    }

    Ok(())
}
```

The server loads the private key and certificate chain from the specified file paths and configures a `rustls::ServerConfig`. The `rustls::ServerConfig` is wrapped inside a `TlsAcceptor` provided by `tokio-rustls`.

Inside the `handle_client` function, the `TcpStream` is accepted by the `TlsAcceptor`, and if successful, the resulting `TlsStream` is passed to the `process_tls_stream` function. The `process_tls_stream` function reads and processes data from the TLS-encrypted stream, echoing it back to the client.