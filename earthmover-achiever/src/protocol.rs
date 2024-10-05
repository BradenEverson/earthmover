//! Enum and Struct definitions for the *ArrowHead Transfer Protocol*

use uuid::Uuid;

use crate::brain::instruction::Instruction;

/// A message to send to an already initialized AHTP accepting simulation server. This must first
/// be initialized by sending an initiation request with respect to serialized urdf and agent body
/// information
pub enum AhtpMessage<const DIMS: usize> {
    /// Send a buffer of collected data points to the server in DIMS dimensions
    Send(Vec<[f32; DIMS]>),
    /// Set the current goal of the agent. That is, what index of dimension do we want to
    /// maximize(true) or minimize(false). 
    ///
    /// For example, `Goal([(0, true)])` would attempt to maximize the first dimension on the
    /// agent's readings.
    GOAL(Vec<(usize, bool)>)
}

/// A response from the simulation server
pub enum AhtpResponse {
    /// The initialization step was a success, here is the session ID to init websocket
    /// communication with.
    Initialized(Uuid),
    /// An instruction set from the simulation server
    Instruction(Vec<Instruction>)
}
