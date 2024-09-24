//! The implementation for generating a batch of simulations and running them

use futures::{FutureExt, StreamExt};

use crate::{sim::SimRes, simulate, Orchestrator};

impl<const N: usize> Orchestrator<N> {
    /// Submits `sim_amount` simulations to the Orchestrator for execution
    pub fn submit(&mut self, sim_amount: usize) {
        let fut = (0..sim_amount).map(|_| simulate::<N>("todo").boxed());
        self.batch_sims.extend(fut)
    }

    /// Runs all batch simulations and returns the simulation with the best fitness
    pub async fn run(&mut self) -> SimRes {
        let mut results = vec![];
        while let Some(result) = self.batch_sims.next().await {
            results.push(result);
        }

        todo!("Process the returned SimReses and find the one with the best fitness");
    }
}
