//! The server crate responsible for handling incoming data and simulating physics of the
//! environment

use earthmover_hivemind::{
    new_state,
    state::message::{Message, Response},
};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let (mut msg_queue, mut state, service) = new_state::<f64>();

    let listener = TcpListener::bind("0.0.0.0:1940").await.unwrap();
    println!(
        "Listening on http://localhost:{}",
        listener.local_addr().unwrap().port()
    );

    tokio::spawn(async move {
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
    });

    while let Some(msg) = msg_queue.recv().await {
        match msg {
            Message::Connection(id, res_channel) => {
                state.new_session(id, res_channel);
            }
            Message::SetDims(id, dims) => state[&id].set_dims(dims),
            Message::Goal(_id, _goal) => {
                todo!();
            }
            Message::SendData(id, buf) => state[&id].write(&buf),
            Message::Train(id) => {
                if state[&id].train().is_none() {
                    state[&id]
                        .send(Response::TrainError(
                            "Not all agent attributes have been set yet",
                        ))
                        .expect("Failed to send message to response channel");
                }
            }
            Message::Disconnection => {}
        }
    }
}
