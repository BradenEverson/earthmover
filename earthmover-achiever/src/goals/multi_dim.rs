//! A REWARD implementation for a struct wrt an agent's current position

use super::{Goal, Rewardable};

/// A REWARD trait impl for when context
pub struct PositionContextualReward<const N: usize> {
    goals: [Option<Goal>; N],
    curr_reading: [f64; N],
}

impl<const N: usize> Rewardable for PositionContextualReward<N> {
    fn to_reward(&self) -> f64 {
        let mut final_score = 0.0;
        for (goal, val) in self.goals.iter().zip(self.curr_reading.iter()) {
            if let Some(goal) = goal {
                final_score += goal.match_against(*val, 1.0);
            }
        }

        final_score
    }
}
