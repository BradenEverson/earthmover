//! All required defintions for handling AHTP incoming state messages

use earthmover_achiever::goals::Rewardable;
use service::ServerService;
use state::{message::MessageReceiver, ServerState};

pub mod service;
pub mod state;

/// Creates a new State and State Service linked together by message and response channels
pub fn new_state<REWARD: Rewardable + Default>(
) -> (MessageReceiver, ServerState<REWARD>, ServerService) {
    let (msg_sender, msg_reader) = tokio::sync::mpsc::unbounded_channel();

    let state = ServerState::default();
    let service = ServerService::new(msg_sender);

    (msg_reader, state, service)
}
