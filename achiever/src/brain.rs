//! The 'brain' of an Agent, responsible for creating new sessions and sending collected data

pub mod agent;
pub mod buffer;
pub mod instruction;

pub use agent::AgentSession;
