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
    ) where
        [f32; DIMS]: ValidDimension;
    /// The backend's name
    fn name(&self) -> String {
        "Empty".into()
    }
}

/// Allows the ValidDimension trait for n dimensions
#[allow(edition_2024_expr_fragment_specifier)]
macro_rules! valid_for {
    ($($dim:expr),+) => {
        $(impl ValidDimension for [f32; $dim] {})+
    };
}

/// Denotes if an n-dimensional point is valid in our engine. Currently, any dimension in the
/// range 3 <= n <= 32 are considered valid dimensions
pub trait ValidDimension {}
valid_for!(
    3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32
);
