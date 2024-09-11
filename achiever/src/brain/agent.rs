//! An Agent's behavior, session states, and builder

use std::{error::Error, marker::PhantomData, time::Duration};

use crate::{
    body::{Body, Peripheral},
    goals::{Goal, Rewardable},
};

use super::{buffer::DataBuffer, instruction::Instruction};

/// TypeState for a newly untrained session
pub struct Untrained;
/// TypeState for the type of agent session that is received after training
pub struct InReview;

/// Result type for when we're dealing with a bunch of input/output peripherals that have an
/// unknown Error type
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// The agent will hold both the current goal of the system and a reference to the actual hardware
/// we're using. After sufficient data from the environment has been collected, the goal, agent
/// body, and data buf will all be sent to the remote simulation engine to before parralellized RL
/// The response will then be a finished AgentSession that can be attempted to run in the field!
pub struct AgentSession<'agent, REWARD: Rewardable, STATE, const BUFFER_SIZE: usize> {
    /// The goal of the agent
    goal: Goal<REWARD>,
    /// A reference to the agent's hardware
    body: &'agent mut Body,
    /// The data collection buffer
    buffer: DataBuffer<BUFFER_SIZE>,
    /// Instruction sets on completed training
    directions: Option<Vec<Instruction>>,
    /// PhantomData for state :)
    _spooky_ghost: PhantomData<STATE>,
}

impl<'agent, REWARD: Rewardable, STATE, const BUFFER_SIZE: usize>
    AgentSession<'agent, REWARD, STATE, BUFFER_SIZE>
{
    /// Creates a new builder for an agent's session
    pub fn builder() -> Builder<'agent, REWARD, BUFFER_SIZE> {
        Builder::default()
    }

    /// Gets the current reward of the agent session
    pub fn get_reward(&self) -> Option<f64> {
        self.goal.evaluate()
    }

    /// Returns a reference to the agent's hardware
    pub fn get_body(&self) -> &Body {
        self.body
    }
}

impl<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize>
    AgentSession<'agent, REWARD, Untrained, BUFFER_SIZE>
{
    /// Adds a slice of data to the buffer, if that slice is too large `None` is returned
    pub fn add_data(&mut self, buf: &[u8]) -> Option<()> {
        self.buffer.add_data(buf)
    }
}

impl<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize>
    AgentSession<'agent, REWARD, InReview, BUFFER_SIZE>
{
    /// Gets the directions of a newly trained Agent
    pub fn act(&mut self) -> Result<()> {
        if let Some(instructions) = &self.directions {
            for instruction in instructions {
                if let Some(node) = self.body.get_by_id_mut(instruction.node) {
                    if let Peripheral::Output(output) = &mut node.peripheral {
                        output.write(&instruction.instructions)?
                    }
                }
                std::thread::sleep(Duration::from_millis(instruction.lasts_for_ms as u64))
            }
        }
        Ok(())
    }
}

/// Builder will create a new agent session from a Body's reference. The STATE of this
/// AgentSession will always be untrained. Only when receiving a new Agent back from the simulation
/// server will we receive an agent tagged as Trained. Untrained agents do not have access to the
/// directions bytes, preventing them from being runnable
pub struct Builder<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize> {
    /// The goal
    goal: Option<Goal<REWARD>>,
    /// A reference to the agent's hardware
    body: Option<&'agent mut Body>,
}

impl<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize> Default
    for Builder<'agent, REWARD, BUFFER_SIZE>
{
    fn default() -> Self {
        Self {
            goal: None,
            body: None,
        }
    }
}

impl<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize> Builder<'agent, REWARD, BUFFER_SIZE> {
    /// Set an agent's goal
    pub fn with_goal(mut self, goal: Goal<REWARD>) -> Self {
        self.goal = Some(goal);
        self
    }

    /// Set an agent's body
    pub fn with_body(mut self, body: &'agent mut Body) -> Self {
        self.body = Some(body);
        self
    }

    /// Build a fully configured `AgentSession`
    pub fn build(self) -> Option<AgentSession<'agent, REWARD, Untrained, BUFFER_SIZE>> {
        match (self.goal, self.body) {
            (Some(goal), Some(body)) => Some(AgentSession {
                goal,
                body,
                buffer: DataBuffer::default(),
                directions: None,
                _spooky_ghost: PhantomData,
            }),
            _ => None,
        }
    }
}
