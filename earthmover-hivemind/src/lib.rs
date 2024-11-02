//! All required defintions for handling AHTP incoming state messages

use std::sync::Arc;

use service::ServerService;
use state::ServerState;
use tokio::sync::RwLock;

pub mod service;
pub mod state;

/// Creates a new State and State Service linked together by message and response channels
pub fn new_state() -> (Arc<RwLock<ServerState>>, ServerService) {
    let (msg_sender, msg_reader) = tokio::sync::mpsc::unbounded_channel();
    let (res_sender, res_reader) = tokio::sync::mpsc::unbounded_channel();

    let state = ServerState::new(msg_reader, res_sender);
    let state_lock = Arc::new(RwLock::new(state));
    let service = ServerService::new(state_lock.clone(), res_reader, msg_sender);

    (state_lock, service)
}
