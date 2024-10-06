//! Enum and Struct definitions for the *ArrowHead Transfer Protocol*

use crate::brain::instruction::Instruction;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

/// A message to send to an already initialized AHTP accepting simulation server. This must first
/// be initialized by sending an initiation request with respect to serialized URDF and agent body
/// information.
#[derive(Serialize, Deserialize)]
pub enum AhtpMessage<const DIMS: usize>
where
    [f32; DIMS]: ArrayBoundedSize + Serialize + DeserializeOwned,
{
    /// Send a buffer of collected data points to the server in DIMS dimensions.
    Send(Vec<[f32; DIMS]>),
    /// Connect to a session via websocket
    Connect(Uuid),
    /// Set the current goal of the agent. That is, which index of dimension we want to
    /// maximize (true) or minimize (false).
    ///
    /// For example, `Goal([(0, true)])` would attempt to maximize the first dimension on the
    /// agent's readings.
    Goal(Vec<(usize, bool)>),
}

/// A response from the simulation server.
#[derive(Serialize, Deserialize)]
pub enum AhtpResponse {
    /// The initialization step was a success. Here is the session ID to init WebSocket
    /// communication with.
    Initialized(Uuid),
    /// An instruction set from the simulation server.
    Instruction(Vec<Instruction>),
}

impl AhtpResponse {
    /// Returns the initialized ID if the type of this response is an init
    pub fn get_init(self) -> Option<Uuid> {
        match self {
            Self::Initialized(id) => Some(id),
            _ => None,
        }
    }
}

/// A trait to indicate that array sizes are bounded.
pub trait ArrayBoundedSize {}

/// Macro to implement ArrayBoundedSize for `[f32; N]` arrays up to the specified maximum size.
#[allow(edition_2024_expr_fragment_specifier)]
macro_rules! impl_array_bounded_size {
    ($($size:expr),*) => {
        $(impl ArrayBoundedSize for [f32; $size] {} )*
    };
}

// Implement ArrayBoundedSize for arrays of size 0 to 32.
impl_array_bounded_size!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32
);

// Some helpful stuff for creating messages conveniently

impl<const DIMS: usize> From<Vec<(usize, bool)>> for AhtpMessage<DIMS>
where
    [f32; DIMS]: ArrayBoundedSize + Serialize + DeserializeOwned,
{
    fn from(value: Vec<(usize, bool)>) -> Self {
        Self::Goal(value)
    }
}

impl<const DIMS: usize> From<Vec<[f32; DIMS]>> for AhtpMessage<DIMS>
where
    [f32; DIMS]: ArrayBoundedSize + Serialize + DeserializeOwned,
{
    fn from(value: Vec<[f32; DIMS]>) -> Self {
        Self::Send(value)
    }
}
