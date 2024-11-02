//! The service for handling the current state

pub mod service_impl;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::state::{
    message::{MessageSender, ResponseReceiver},
    ServerState,
};

/// A server service handler
#[derive(Clone)]
pub struct ServerService {
    /// The current state this service works with respect to
    pub state: Arc<RwLock<ServerState>>,
    /// The responses sent back
    pub response_queue: Arc<RwLock<ResponseReceiver>>,
    /// The message send channel
    pub message_sender: MessageSender,
}

impl ServerService {
    /// Creates a new service instance
    pub fn new(
        state: Arc<RwLock<ServerState>>,
        response_queue: ResponseReceiver,
        message_sender: MessageSender,
    ) -> Self {
        Self {
            state,
            response_queue: Arc::new(RwLock::new(response_queue)),
            message_sender,
        }
    }
}
