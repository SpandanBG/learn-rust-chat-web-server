## Creating SSL 

#### Creating Private Key

To create a PKCS8-encoded private key with OpenSSL, you can use the `openssl` command-line tool. Here's an example of how you can generate a PKCS8-encoded private key in PEM format:

```shell
openssl genpkey -algorithm RSA -out private_key.pem -pkeyopt rsa_keygen_bits:2048 -pkeyopt rsa_keygen_pubexp:65537
```

In this example:
- `genpkey` is the OpenSSL command to generate a private key.
- `-algorithm RSA` specifies that the RSA algorithm will be used.
- `-out private_key.pem` specifies the output file where the private key will be stored. You can replace `private_key.pem` with the desired filename.
- `-pkeyopt rsa_keygen_bits:2048` specifies the key size (2048 bits in this case). You can adjust the value as per your requirements.
- `-pkeyopt rsa_keygen_pubexp:65537` specifies the public exponent value. The default value of 65537 is commonly used.

After executing this command, you will have a PKCS8-encoded private key in PEM format saved in the `private_key.pem` file. You can then use this private key in your Rust code with the Rustls library or any other SSL/TLS library as needed.

#### Creating Certificate

To create a self-signed certificate from the private key using OpenSSL, you can follow these steps:

1. Generate a private key (if you haven't already done so):

   ```shell
   openssl genpkey -algorithm RSA -out private_key.pem -pkeyopt rsa_keygen_bits:2048 -pkeyopt rsa_keygen_pubexp:65537
   ```

2. Generate a certificate signing request (CSR) using the private key:

   ```shell
   openssl req -new -key private_key.pem -out csr.pem
   ```

   This command will prompt you to provide some information about the certificate, such as the Common Name (CN), Organization (O), etc. Fill in the details as appropriate.

3. Generate a self-signed certificate using the private key and CSR:

   ```shell
   openssl x509 -req -days 365 -in csr.pem -signkey private_key.pem -out certificate.pem
   ```

   This command generates a self-signed certificate valid for 365 days (`-days 365`) using the private key (`-signkey`) and the CSR (`-in`). The resulting certificate is saved in the `certificate.pem` file.

After executing these steps, you will have a self-signed certificate (`certificate.pem`) and a private key (`private_key.pem`). You can then use these files in your Rust code or any other application that requires SSL/TLS certificates.

----

## SSL in Rust with Tokio

#### Packages

Add the following packages

```toml
[dependencies]
tokio = { version = "1.", features=["full"] }
tokio-rustls = "0.24.1"
rustls = "0.21.5"
rustls-pemfile = "1.0.3"
```

The `rustls` package will provide us with the SSL/TLS features, while the `tokio-rustls` will provide async capabilities to the `rustls` package.

#### Code

```rust
use std::fs;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::{server::TlsStream, TlsAcceptor};
use rustls::{Certificate, PrivateKey};
use rustls_pemfile;

const PRIVATE_KEY_FILE: &'static str = "private_key.pem";
const CERTIFICATE_FILE: &'static str = "certificate.pem"

#[tokio::main]
async fn main() {
    // Load the server's private key and certificate chain
    let private_key = load_private_key_from_file(PRIVATE_KEY_FILE);
    let cert_chain = load_certs_from_prem(CERTIFICATE_FILE);

    // Create a Rustls server configuration
	let config = rustls::ServerConfig::builder()
		.with_safe_defaults()
		.with_no_client_auth()
		.with_single_cert(cert_chain, private_key)
		.expect("unable to setup tls server config");

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

fn load_certs_from_prem(path: &str) -> Vec<Certificate> {
	let file = File::open(path).expect(&format!("unable to open file {:?}", path));
	let mut reader = BufReader::new(file);
	let certs =
		rustls_pemfile::certs(&mut reader).expect(&format!("failed to read file {:?}", path));

	certs.into_iter().map(Certificate).collect()
}

fn load_private_key_from_file(path: &str) -> PrivateKey {
	let file = File::open(path).expect(&format!("unable to opne file {:?}", path));
	let mut reader = BufReader::new(file);
	let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader)
		.expect(&format!("failed to read file {:?}", path));

	match keys.len() {
		0 => panic!("no PKCS8-encoded private key found"),
		1 => PrivateKey(keys.remove(0)),
		_ => panic!("more than one PKCS8-encoded private key found"),
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

---

## Error: `failed to convert to tls stream Custom { kind: InvalidData, error: AlertReceived(CertificateUnknown) }`

The error message you're encountering suggests that the Rustls library is reporting an issue with the certificate during the TLS handshake process. Specifically, it indicates that the server certificate is not recognized or trusted ("CertificateUnknown").

To resolve this issue, you have a few options:

1. **Import the self-signed certificate as trusted**: If you're using a self-signed certificate, you can import it into the trust store of the system or application. This step will vary depending on your operating system and configuration. Once the certificate is trusted, Rustls should be able to validate it successfully.

2. **Disable certificate verification**: While it's generally not recommended for production environments, you can disable certificate verification temporarily during development or testing. Here's an example of how you can achieve this using Rustls:

```rust
use rustls::ClientConfig;
use std::sync::Arc;
use webpki::DNSNameRef;

fn main() {
    let mut config = ClientConfig::new();
    config
        .dangerous()
        .set_certificate_verifier(Arc::new(rustls::NoopVerifier {}));
    
    let domain = DNSNameRef::try_from_ascii_str("yourdomain.com").unwrap();
    let mut session = rustls::ClientSession::new(&Arc::new(config), domain);

    // Continue with your TLS handshake using the session
}
```

In this example, we create a `ClientConfig` and set a "NoopVerifier" to disable certificate verification. Replace `"yourdomain.com"` with the appropriate domain name.

3. **Use the `native-tls` crate**: Instead of using `rustls`, you can switch to the `native-tls` crate, which leverages the platform's native TLS implementation. It allows you to work with self-signed certificates more easily. However, note that the `native-tls` crate relies on the system's OpenSSL library, so you may need to install it if not already available.

You can add `native-tls` to your `Cargo.toml`:

```toml
[dependencies]
native-tls = "0.2"
```

And use it in your code:

```rust
use native_tls::TlsConnector;

fn main() {
    let connector = TlsConnector::new().unwrap();
    let stream = connector.connect("yourdomain.com:443").unwrap();
    
    // Continue working with the TLS stream
}
```

With `native-tls`, you won't need to explicitly handle self-signed certificates as it will rely on the system's trust store.

Remember to consult the official documentation for the TLS library you're using, as well as the Rustls and native-tls documentation, for additional information and guidance on working with certificates and configuring TLS connections.