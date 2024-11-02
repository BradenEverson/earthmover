//! Current state of the server, including a message queue

use message::{MessageReceiver, ResponseSender};

pub mod message;

/// The current server's state
pub struct ServerState {
    /// The messages waiitng to be handled
    pub message_queue: MessageReceiver,
    /// The pathway for sending out new responses
    pub response_sender: ResponseSender,
}

impl ServerState {
    /// Creates a new state
    pub fn new(message_queue: MessageReceiver, response_sender: ResponseSender) -> Self {
        Self { message_queue, response_sender }
    }
}
