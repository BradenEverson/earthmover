//! Trait for defining how a simulation is handled, through for example Bevy simulations

use std::sync::Arc;

use earthmover_achiever::goals::Rewardable;
use tokio::sync::mpsc::UnboundedSender;

use super::{SimArgs, SimMessage};

/// A simulation that takes in physical context and creates an instruction-set and final fitness
/// score
pub trait Simulation {
    /// Runs through a simulation based on beginning arguments, reports back to a Receiver with
    /// instructions to reach a certain `Score`
    fn simulate<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize>(&self, args: Arc<SimArgs<'agent, REWARD, BUFFER_SIZE>>, message_sender: UnboundedSender<SimMessage>);
}
