//! The variants a message may be

use earthmover_achiever::brain::instruction::Instruction;
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
pub enum Message {
    /// A client initial connection
    Connection,
    /// Set the dimensionality of this simulation
    SetDims(Uuid, usize),
    /// Send a data buffer in sequences of set dimension chunks(every n elements are considered a
    /// point)
    SendData(Uuid, Vec<f32>),
    /// Set the goal for the current agent (what point in the dimension to focus on and whether we
    /// want to maximize(true) or minizmize(false))
    Goal(Uuid, Vec<(usize, bool)>),
    /// Disconnect from the session
    Disconnection,
}

/// All variants a server response may have
pub enum Response {
    /// Client is connected - here's the session ID
    Connected(Uuid),
    /// Generated instruction for achieving a goal
    Instruction(Vec<Instruction>),
}
