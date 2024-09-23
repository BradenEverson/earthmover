use std::sync::Arc;

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use joint_creation::state::{JointService, State};
use tokio::{net::TcpListener, sync::RwLock};

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(State::default()));

    let listener = TcpListener::bind("0.0.0.0:7879").await.unwrap();
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

            let server_service = JointService::new(state.clone());

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

    tokio::spawn(async move {
        let _state = state.clone();
    });

    connection_handler.await
}
