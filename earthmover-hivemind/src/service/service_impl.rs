//! The hyper service implementation for the custom `ServerService`

use std::{future::Future, pin::Pin};

use futures_util::StreamExt;
use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    Request, Response, StatusCode,
};

use super::ServerService;

impl Service<Request<body::Incoming>> for ServerService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, mut req: Request<body::Incoming>) -> Self::Future {
        if hyper_tungstenite::is_upgrade_request(&req) {
            let (response, websocket) =
                hyper_tungstenite::upgrade(&mut req, None).expect("Error upgrading to WebSocket");
            tokio::spawn(async move {
                match websocket.await {
                    Ok(ws) => {
                        let (_writer, mut reader) = ws.split();
                        while let Some(msg) = reader.next().await {
                            match msg {
                                Ok(_msg) => {
                                    todo!("Handle incoming messages")
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

            let res = response.body(Full::new(Bytes::copy_from_slice(b"WIP :)")));
            Box::pin(async { res })
        }
    }
}
