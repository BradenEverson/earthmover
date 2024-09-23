use std::{fs::File, future::Future, io::Read, pin::Pin, sync::Arc};

use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    Method, Request, Response, StatusCode,
};
use tokio::sync::RwLock;

#[derive(Default)]
pub struct State {
    body: Vec<achiever::body::Body>,
}

impl State {}

impl State {}

pub struct JointService {
    state: Arc<RwLock<State>>,
}

impl JointService {
    pub fn new(state: Arc<RwLock<State>>) -> Self {
        Self { state }
    }
}

impl Service<Request<body::Incoming>> for JointService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<body::Incoming>) -> Self::Future {
        let response = Response::builder().status(StatusCode::OK);

        let res = match req.method() {
            &Method::GET => {
                let path = match req.uri().path() {
                    "/" => "frontends/joint-ui/index.html",
                    _ => "frontends/joint-ui/404.html",
                };

                let mut page = File::open(path).expect("Failed to open test file");
                let mut buf = vec![];
                page.read_to_end(&mut buf).expect("Failed to read file");

                response.body(Full::new(Bytes::copy_from_slice(&buf)))
            }
            _ => response.body(Full::new(Bytes::copy_from_slice(&[]))),
        };

        Box::pin(async { res })
    }
}
