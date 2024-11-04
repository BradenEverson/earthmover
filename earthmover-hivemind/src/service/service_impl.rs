//! The hyper service implementation for the custom `ServerService`

use std::{future::Future, pin::Pin};

use futures_util::{SinkExt, StreamExt};
use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    Request, Response, StatusCode,
};
use uuid::Uuid;

use crate::state::message::Message;

use super::ServerService;

impl Service<Request<body::Incoming>> for ServerService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, mut req: Request<body::Incoming>) -> Self::Future {
        let sender = self.message_sender.clone();
        if hyper_tungstenite::is_upgrade_request(&req) {
            let (response, websocket) =
                hyper_tungstenite::upgrade(&mut req, None).expect("Error upgrading to WebSocket");
            tokio::spawn(async move {
                match websocket.await {
                    Ok(ws) => {
                        let (mut writer, mut reader) = ws.split();
                        let id = Uuid::new_v4();
                        let (com_writer, mut com_reader) = tokio::sync::mpsc::unbounded_channel();
                        sender
                            .send(Message::Connection(id, com_writer))
                            .expect("Failed to send to sender");

                        tokio::spawn(async move {
                            while let Some(response) = com_reader.recv().await {
                                writer
                                    .send(hyper_tungstenite::tungstenite::Message::Text(
                                        response
                                            .serialize_to_string()
                                            .expect("Failed to serialize response message"),
                                    ))
                                    .await
                                    .expect("Failed to send ws message");
                            }
                        });

                        while let Some(msg) = reader.next().await {
                            match msg {
                                Ok(msg) => {
                                    if let hyper_tungstenite::tungstenite::Message::Text(txt) = msg
                                    {
                                        let message = Message::from_string(&txt);
                                        if let Ok(message) = message {
                                            sender
                                                .send(message)
                                                .expect("Failed to send back to channel");
                                        }
                                    }
                                }
                                Err(err) => eprintln!("{err}"),
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to establish WebSocket Connection: {}", err)
                    }
                }
            });

            Box::pin(async { Ok(response) })
        } else {
            let response = Response::builder().status(StatusCode::OK);

            let message = r#"
    EARTHMOVER HIVEMIND

                .-~~~-.
              /        }
             /      .-~
            |        }
___\.~~-.-~|     .-~_
    { O  |  ` .-~.   ; ~-.__
     ~--~/-|_\|  :   : .-~
        /  |  \~ - - ~
       /   |   \       
    "#;

            let res = response.body(Full::new(Bytes::copy_from_slice(message.as_bytes())));
            Box::pin(async { res })
        }
    }
}
