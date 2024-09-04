use std::marker::PhantomData;

use crate::goals::{Goal, Rewardable};

use super::buffer::BufferMarker;

pub struct Untrained;
pub struct InReview;

/// The agent will hold both the current goal of the system and a reference to the actual hardware
/// we're using. After sufficient data from the environment has been collected, the goal, agent
/// body, and data buf will all be sent to the remote simulation engine to before parralellized RL
/// The response will then be a finished AgentSession that can be attempted to run in the field!
pub struct AgentSession<'agent, REWARD: Rewardable, STATE, const BUFFER_SIZE: usize> {
    goal: Goal<REWARD>,
    body: &'agent Body,
    data_buf: [u8; BUFFER_SIZE],
    buffer_pos: BufferMarker<BUFFER_SIZE>,

    directions: Option<Vec<u8>>,
    spooky_ghost: PhantomData<STATE>,
}

impl<'agent, REWARD: Rewardable, STATE, const BUFFER_SIZE: usize>
    AgentSession<'agent, REWARD, STATE, BUFFER_SIZE>
{
    pub(crate) fn with_body(body: &'agent Body) -> Self {
        Self {
            goal: Goal::None,
            body,
            data_buf: [0u8; BUFFER_SIZE],
            buffer_pos: BufferMarker::default(),
            spooky_ghost: PhantomData,
            directions: None
        }
    }

    pub fn set_goal(&mut self, goal: Goal<REWARD>) {
        self.goal = goal
    }
}

impl<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize> AgentSession<'agent, REWARD, InReview, BUFFER_SIZE> {
    pub fn get_directions(&self) -> Option<&[u8]> {
        self.directions.as_deref()
    }
}

/// AgentSessionFactory will create a new agent session from a Body's reference. The STATE of this
/// AgentSession will always be untrained. Only when receiving a new Agent back from the simulation
/// server will we receive an agent tagged as Trained. Untrained agents do not have access to the
/// directions bytes, preventing them from being runnable
pub struct AgentSessionFactory;
impl AgentSessionFactory {
    pub fn create<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize>(
        body: &'agent Body,
    ) -> AgentSession<'agent, REWARD, Untrained, BUFFER_SIZE> {
        AgentSession::with_body(body)
    }
}

/// Abstraction over the hardware of the device. This is still TODO because I don't exactly know
/// how we want to do this. My main idea is we can have an enum for Input/Output, and from there
/// some sort of method for reading or writing. The a body would only be a Vec<Peripheral> but we
/// could maybe also do a graph like structure
pub struct Body;
