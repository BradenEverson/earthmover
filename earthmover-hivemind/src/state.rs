//! Current state of the server, including a message queue

use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use earthmover_achiever::{
    body::Body,
    goals::{Goal, Rewardable},
};
use earthmover_simulation::{sim::backend::physics::BevyPhysicsInformedBackend, Orchestrator};
use message::{Response, ResponseSender};
use uuid::Uuid;

pub mod message;

/// How many simulations per "batch"
pub const NUM_SIMS: usize = 100_000;
/// Dimension of batch
pub const NUM_DIMS: usize = 3;

/// The current server's state
#[derive(Default)]
pub struct ServerState<REWARD: Rewardable> {
    /// The sessions this state is aware of
    sessions: HashMap<Uuid, Connection<REWARD>>,
}

impl<REWARD: Rewardable> ServerState<REWARD> {
    /// Adds a new session to the internal sessions
    pub fn new_session(&mut self, id: Uuid, channel: ResponseSender) {
        self.sessions.insert(id, Connection::new(channel));
    }
}

impl<REWARD: Rewardable> Index<&Uuid> for ServerState<REWARD> {
    type Output = Connection<REWARD>;
    fn index(&self, index: &Uuid) -> &Self::Output {
        &self.sessions[index]
    }
}

impl<REWARD: Rewardable> IndexMut<&Uuid> for ServerState<REWARD> {
    fn index_mut(&mut self, index: &Uuid) -> &mut Self::Output {
        self.sessions.get_mut(index).unwrap()
    }
}

/// A current connection session
pub struct Connection<REWARD: Rewardable> {
    /// Where to send response messages
    response_channel: ResponseSender,
    /// Simulation dimensions
    dims: usize,
    /// Current goal
    goal: Option<Goal<REWARD>>,
    /// Current body
    body: Option<Body>,
    /// Current data read in
    buf: Vec<f32>,
}

impl<REWARD: Rewardable> Connection<REWARD> {
    /// Creates a new connection with just a response channel
    pub fn new(response_channel: ResponseSender) -> Self {
        Self {
            response_channel,
            dims: 0,
            goal: None,
            body: None,
            buf: vec![],
        }
    }

    /// Sets the dims for this session
    pub fn set_dims(&mut self, dims: usize) {
        self.dims = dims
    }

    /// Sends a response to the underlying client
    pub fn send(
        &mut self,
        response: Response,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<message::Response>> {
        self.response_channel.send(response)
    }

    /// Writes data to the buffer
    pub fn write(&mut self, buf: &[f32]) {
        self.buf.extend(buf)
    }

    /// Begins training the agent
    pub fn train(&mut self) -> Option<()> {
        if self.goal.is_none() || self.body.is_none() {
            return None;
        }

        let _orchestrator: Orchestrator<BevyPhysicsInformedBackend, NUM_DIMS> =
            Orchestrator::new(BevyPhysicsInformedBackend);

        Some(())
    }
}
