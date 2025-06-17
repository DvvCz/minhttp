mod config;
mod response;

use futures_rustls::TlsAcceptor;
use rustls::ServerConfig;
use smol::prelude::*;
use std::sync::Arc;

async fn load_tls_config(cert_file: &std::path::Path, key_file: &std::path::Path) -> anyhow::Result<Arc<ServerConfig>> {
    let cert_file_content = smol::fs::read(cert_file).await?;
    let cert_chain = rustls_pemfile::certs(&mut cert_file_content.as_slice()).collect::<Result<Vec<_>, _>>()?;

    let key_file_content = smol::fs::read(key_file).await?;
    let private_key = rustls_pemfile::private_key(&mut key_file_content.as_slice())?
        .ok_or_else(|| anyhow::anyhow!("No private key found in key file"))?;

    let config = ServerConfig::builder().with_no_client_auth().with_single_cert(cert_chain, private_key)?;

    Ok(Arc::new(config))
}

async fn handle_request<S>(mut stream: S, request: httparse::Request<'_, '_>) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    match request.path {
        Some(path) if path.starts_with("/") => {
            stream.write_all(&response::Response::text("Hello, world!", 200)).await?;
        }

        _ => {
            stream.write_all(&response::Response::status(404)).await?;
        }
    }

    Ok(())
}

async fn handle_client<S>(mut stream: S) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut raw_bytes = [0; 1024];
    let byte_count = stream.read(&mut raw_bytes).await?;

    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut request = httparse::Request::new(&mut headers);

    match request.parse(&raw_bytes[..byte_count]) {
        Ok(httparse::Status::Complete(_)) => handle_request(stream, request).await?,
        Ok(httparse::Status::Partial) => (),
        Err(_) => {
            stream.write_all(&response::Response::status(400)).await?;
        }
    }

    Ok(())
}

#[dotenvy::load(required = false)]
fn main() -> anyhow::Result<()> {
    match &*config::CONFIG {
        config::Config::Development { url } => {
            println!("Running in development mode at http://{}", url);

            smol::block_on(async {
                let stream = smol::net::TcpListener::bind(url).await?;

                loop {
                    let (stream, _) = stream.accept().await?;
                    smol::spawn(handle_client(stream)).detach();
                }
            })
        }

        config::Config::Production {
            url,
            cert_file,
            private_key_file,
        } => {
            println!(
                "Running in production mode at https://{} with cert: {} and key: {}",
                url,
                cert_file.display(),
                private_key_file.display()
            );

            smol::block_on(async {
                let tls_config = load_tls_config(cert_file, private_key_file).await?;
                let acceptor = TlsAcceptor::from(tls_config);

                let listener = smol::net::TcpListener::bind(url).await?;

                loop {
                    let (stream, _) = listener.accept().await?;
                    let acceptor = acceptor.clone();

                    smol::spawn(async move {
                        match acceptor.accept(stream).await {
                            Ok(tls_stream) => {
                                if let Err(e) = handle_client(tls_stream).await {
                                    eprintln!("Error handling TLS client: {}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("TLS handshake failed: {}", e);
                            }
                        }
                    })
                    .detach();
                }
            })
        }
    }
}
