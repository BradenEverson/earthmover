//! The server crate responsible for handling incoming data and simulating physics of the
//! environment

use earthmover_hivemind::new_state;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let (_state, service) = new_state();

    let listener = TcpListener::bind("0.0.0.0:1940").await.unwrap();
    println!(
        "Listening on http://localhost:{}",
        listener.local_addr().unwrap().port()
    );

    let connection_handler = async move {
        loop {
            // Handle connections
            let (socket, _) = listener
                .accept()
                .await
                .expect("Error accepting incoming connection");

            let io = TokioIo::new(socket);

            let service = service.clone();
            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {}", e);
                }
            });
        }
    };

    connection_handler.await
}
