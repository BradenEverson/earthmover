//! The service for handling the current state

pub mod service_impl;

use crate::state::message::MessageSender;

/// A server service handler
#[derive(Clone)]
pub struct ServerService {
    /// The message send channel
    pub message_sender: MessageSender,
}

impl ServerService {
    /// Creates a new service instance
    pub fn new(
        message_sender: MessageSender,
    ) -> Self {
        Self {
            message_sender,
        }
    }
}
