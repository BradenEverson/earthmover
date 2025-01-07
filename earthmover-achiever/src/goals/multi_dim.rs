//! A REWARD implementation for a struct wrt an agent's current position

use super::{Goal, Rewardable};

/// A REWARD trait impl for when context
#[derive(Clone, Copy)]
pub struct PositionContextualReward<const N: usize> {
    goals: [Option<Goal>; N],
    curr_reading: [f64; N],
}

impl<const N: usize> Default for PositionContextualReward<N> {
    fn default() -> Self {
        Self {
            goals: [None; N],
            curr_reading: [0.0; N],
        }
    }
}

impl<const N: usize> PositionContextualReward<N> {
    /// Sets the current reading of the agent
    pub fn set_reading(&mut self, new_pos: [f64; N]) {
        self.curr_reading = new_pos
    }

    /// Updates a set of goals based on id-goal pairings
    pub fn update(&mut self, goals: Vec<(usize, bool)>) {
        for (idx, maximize) in goals {
            if N < idx {
                let goal = match maximize {
                    true => Goal::Maximize,
                    false => Goal::Minimize,
                };
                self.goals[idx] = Some(goal)
            }
        }
    }
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
