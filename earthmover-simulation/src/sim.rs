//! The arguments to be passed through to a simulation and outputs that can be returned

use std::sync::Arc;

use earthmover_achiever::{
    brain::{agent::Untrained, instruction::Instruction, AgentSession},
    goals::Rewardable,
};

/// Any agruments that a simulation may take in
pub struct SimArgs<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize>(
    pub Arc<AgentSession<'agent, REWARD, Untrained, BUFFER_SIZE>>,
);

impl<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize>
    Into<SimArgs<'agent, REWARD, BUFFER_SIZE>>
    for AgentSession<'agent, REWARD, Untrained, BUFFER_SIZE>
{
    fn into(self) -> SimArgs<'agent, REWARD, BUFFER_SIZE> {
        let arc_self = Arc::new(self);
        SimArgs(arc_self)
    }
}

/// The output from a simulation's runtime
#[derive(Default, Debug)]
pub struct SimRes {
    /// The agent's score
    score: f64,
    /// The instructions to achieve this score
    instructions: Vec<Instruction>,
}

impl SimRes {
    /// Adds a new instruction to the back of the instruction set
    pub fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    /// Sets the agents score for the provided instructions
    pub fn set_score(&mut self, score: f64) {
        self.score = score
    }
}

/// A message coming back from a simulation
pub enum SimMessage {
    /// A new instruction
    Instruction(Instruction),
    /// Simulation has ended with a given score
    Close(f64),
}
