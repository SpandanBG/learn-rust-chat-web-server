mod constants;

use self::constants::{CERTIFICATE_CHAIN_FILE_PATH, PRIVATE_KEY_FILE_PATH};

use rustls::{Certificate, PrivateKey};
use rustls_pemfile;
use std::{fs::File, io::BufReader, sync::Arc};
use tokio::net::TcpStream;
use tokio_rustls::{TlsAcceptor, server::TlsStream};

pub struct SSL {
    acceptor: TlsAcceptor,
}

impl SSL {
    pub fn new() -> SSL {
        let private_key = SSL::load_private_key_from_file(PRIVATE_KEY_FILE_PATH);
        let cert_chain = SSL::load_certs_from_prem(CERTIFICATE_CHAIN_FILE_PATH);

        let config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, private_key)
            .expect("unable to setup tls server config");

        SSL {
            acceptor: TlsAcceptor::from(Arc::new(config)),
        }
    }

    pub async fn get_tls_stream(&self, stream: TcpStream) -> TlsStream<TcpStream> {
      match self.acceptor.accept(stream).await{
        Ok(tls_stream) => tls_stream,
        Err(error) => {

            panic!("failed to convert to tls stream {:.2?}", error);
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
}

impl Clone for SSL {
    fn clone(&self) -> Self {
        SSL {
            acceptor: self.acceptor.clone(),
        }
    }
}
