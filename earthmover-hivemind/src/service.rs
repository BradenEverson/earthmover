//! The service for handling the current state

pub mod service_impl;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::state::ServerState;

pub struct ServerService {
    /// The current state this service works with respect to
    pub state: Arc<RwLock<ServerState>>,
}

impl ServerService {
    pub fn new(state: Arc<RwLock<ServerState>>) -> Self {
        Self { state }
    }
}
