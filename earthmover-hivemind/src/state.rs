//! Current state of the server, including a message queue

use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use earthmover_achiever::{body::Body, goals::multi_dim::PositionContextualReward};
use earthmover_simulation::{
    sim::{backend::physics::BevyPhysicsInformedBackend, SimArgs, SimRes},
    Orchestrator,
};
use message::{Response, ResponseSender};
use uuid::Uuid;

pub mod message;

/// How many simulations per "batch"
pub const NUM_SIMS: usize = 100_000;
/// Dimension of batch
pub const NUM_DIMS: usize = 3;

/// The current server's state
#[derive(Default)]
pub struct ServerState {
    /// The sessions this state is aware of
    sessions: HashMap<Uuid, Connection>,
}

impl ServerState {
    /// Adds a new session to the internal sessions
    pub fn new_session(&mut self, id: Uuid, channel: ResponseSender) {
        self.sessions.insert(id, Connection::new(channel));
    }
}

impl Index<&Uuid> for ServerState {
    type Output = Connection;
    fn index(&self, index: &Uuid) -> &Self::Output {
        &self.sessions[index]
    }
}

impl IndexMut<&Uuid> for ServerState {
    fn index_mut(&mut self, index: &Uuid) -> &mut Self::Output {
        self.sessions.get_mut(index).unwrap()
    }
}

/// A current connection session
pub struct Connection {
    /// Where to send response messages
    response_channel: ResponseSender,
    /// Simulation dimensions
    dims: usize,
    /// Current goal
    goal: PositionContextualReward<NUM_DIMS>,
    /// Current data read in
    buf: Vec<f32>,
}

impl Connection {
    /// Creates a new connection with just a response channel
    pub fn new(response_channel: ResponseSender) -> Self {
        Self {
            response_channel,
            dims: 0,
            goal: PositionContextualReward::default(),
            buf: vec![],
        }
    }

    /// Sets the dims for this session
    pub fn set_dims(&mut self, dims: usize) {
        self.dims = dims
    }

    /// Sets the goals for the current session
    pub fn set_goals(&mut self, goals: Vec<(usize, bool)>) {
        self.goal.update(goals)
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
    pub async fn train(&mut self) -> Option<SimRes> {
        let mut orchestrator: Orchestrator<BevyPhysicsInformedBackend, NUM_DIMS> =
            Orchestrator::new(BevyPhysicsInformedBackend);

        let body = Body::default();
        let job: SimArgs<_, NUM_DIMS> = SimArgs::new(self.goal, vec![], body);

        orchestrator.submit(job, NUM_SIMS);

        let best_fit = orchestrator.run().await;

        Some(best_fit)
    }
}
