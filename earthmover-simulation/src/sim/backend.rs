//! Trait for defining how a simulation is handled, through for example Bevy simulations

pub mod physics;

use std::sync::Arc;

use earthmover_achiever::goals::Rewardable;
use tokio::sync::mpsc::UnboundedSender;

use super::{SimArgs, SimMessage};

/// A simulation that takes in physical context and creates an instruction-set and final fitness
/// score
pub trait Simulation {
    /// Runs through a simulation based on beginning arguments, reports back to a Receiver with
    /// instructions to reach a certain `Score`
    fn simulate<REWARD: Rewardable, const DIMS: usize>(
        &self,
        args: Arc<SimArgs<REWARD, DIMS>>,
        message_sender: UnboundedSender<SimMessage>,
    );
    /// The backend's name
    fn name(&self) -> String {
        "Empty".into()
    }
}
