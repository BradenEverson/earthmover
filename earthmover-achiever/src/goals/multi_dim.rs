//! A REWARD implementation for a struct wrt an agent's current position

use super::{Goal, Rewardable};

/// A REWARD trait impl for when context
pub struct PositionContextualReward<const N: usize> {
    _goals: [Option<Goal>; N],
}

impl<const N: usize> Rewardable for PositionContextualReward<N> {
    fn to_reward(&self) -> f64 {
        todo!()
    }
}
