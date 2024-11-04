//! Current state of the server, including a message queue

use message::ResponseSender;

pub mod message;

/// The current server's state
pub struct ServerState {
    /// The pathway for sending out new responses
    pub response_sender: ResponseSender,
}

impl ServerState {
    /// Creates a new state
    pub fn new(response_sender: ResponseSender) -> Self {
        Self {
            response_sender,
        }
    }
}
