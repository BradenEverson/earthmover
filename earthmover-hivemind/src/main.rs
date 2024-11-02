//! The server crate responsible for handling incoming data and simulating physics of the
//! environment

use earthmover_hivemind::{service::ServerService, state::ServerState};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(ServerState::default()));

    let listener = TcpListener::bind("0.0.0.0:1940").await.unwrap();
    println!(
        "Listening on http://localhost:{}",
        listener.local_addr().unwrap().port()
    );

    let state_clone = state.clone();
    let connection_handler = async move {
        let state = state_clone.clone();
        loop {
            // Handle connections
            let (socket, _) = listener
                .accept()
                .await
                .expect("Error accepting incoming connection");

            let io = TokioIo::new(socket);

            let server_service = ServerService::new(state.clone());

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .serve_connection(io, server_service)
                    .await
                {
                    eprintln!("Error serving connection: {}", e);
                }
            });
        }
    };

    connection_handler.await
}
