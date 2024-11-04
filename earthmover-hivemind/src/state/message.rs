//! The variants a message may be

use earthmover_achiever::brain::instruction::Instruction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A message receiver for the message enum
pub type MessageReceiver = tokio::sync::mpsc::UnboundedReceiver<Message>;
/// A message sender for the message enum
pub type MessageSender = tokio::sync::mpsc::UnboundedSender<Message>;

/// A response receiver for the response enum
pub type ResponseReceiver = tokio::sync::mpsc::UnboundedReceiver<Response>;
/// A response sender for the response enum
pub type ResponseSender = tokio::sync::mpsc::UnboundedSender<Response>;

/// All variants that a message can be, including connection requests and existing user contexts
#[derive(Deserialize, Serialize)]
pub enum Message {
    /// A client initial connection
    #[serde(skip_deserializing, skip_serializing)]
    Connection(Uuid, ResponseSender),
    /// Set the dimensionality of this simulation
    SetDims(Uuid, usize),
    /// Send a data buffer in sequences of set dimension chunks(every n elements are considered a
    /// point)
    SendData(Uuid, Vec<f32>),
    /// Set the goal for the current agent (what point in the dimension to focus on and whether we
    /// want to maximize(true) or minizmize(false))
    Goal(Uuid, Vec<(usize, bool)>),
    /// Begin training
    Train(Uuid),
    /// Disconnect from the session
    Disconnection,
}

impl Message {
    /// Attempts to get a message from a serde string
    pub fn from_string(from: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(from)
    }

    /// Attempts to serialize a message to a json string
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// All variants a server response may have
#[derive(Serialize)]
pub enum Response {
    /// Client is connected - here's the session ID
    Connected(Uuid),
    /// Generated instruction for achieving a goal
    Instruction(Vec<Instruction>),
    /// Training error
    TrainError(&'static str),
}

impl Response {
    /// Attempts to serialize the given response to a json string
    pub fn serialize_to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
