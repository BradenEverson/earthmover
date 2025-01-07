//! The arguments to be passed through to a simulation and outputs that can be returned

pub mod backend;

use std::sync::Arc;

use bevy::prelude::Resource;
use earthmover_achiever::{body::Body, brain::instruction::Instruction, goals::Rewardable};

/// Any agruments that a simulation may take in
pub struct SimArgs<REWARD: Rewardable + Send + Sync + 'static, const DIMS: usize> {
    /// The simulation reward
    pub reward: REWARD,
    /// The data passed in
    pub data: Vec<[f32; DIMS]>,
    /// The agent's body
    pub body: Body,
    /// The URDF file path
    pub urdf: String,
}

impl<REWARD: Rewardable + Send + Sync + 'static, const DIMS: usize> SimArgs<REWARD, DIMS> {
    /// Wraps self in an arc
    pub fn arc(self) -> ArcSimArgs<REWARD, DIMS> {
        ArcSimArgs(Arc::new(self))
    }

    /// Creates a new SimArgs from raw parts
    pub fn new(
        reward: REWARD,
        data: Vec<[f32; DIMS]>,
        body: Body,
        urdf: impl Into<String>,
    ) -> Self {
        Self {
            reward,
            data,
            body,
            urdf: urdf.into(),
        }
    }
}

/// An arc-wrapped SimArg
#[derive(Resource)]
pub struct ArcSimArgs<REWARD: Rewardable + Send + Sync + 'static, const DIMS: usize>(
    pub Arc<SimArgs<REWARD, DIMS>>,
);

/// The output from a simulation's runtime
#[derive(Default, Debug)]
pub struct SimRes {
    /// The agent's score
    pub score: f64,
    /// The instructions to achieve this score
    pub instructions: Vec<Instruction>,
}

impl PartialOrd for SimRes {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SimRes {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.total_cmp(&other.score)
    }
}

impl PartialEq for SimRes {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for SimRes {}

impl SimRes {
    /// Adds a new instruction to the back of the instruction set
    pub fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    /// Sets the agents score for the provided instructions
    pub fn set_score(&mut self, score: f64) {
        self.score = score
    }

    /// Returns the fitness score
    pub fn get_score(&self) -> f64 {
        self.score
    }

    /// Returns the instructions
    pub fn get_instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}

/// A message coming back from a simulation
pub enum SimMessage {
    /// A new instruction
    Instruction(Instruction),
    /// Simulation has ended with a given score
    Close(f64),
}
