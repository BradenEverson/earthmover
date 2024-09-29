//! The implementation for generating a batch of simulations and running them

use std::sync::Arc;

use earthmover_achiever::goals::Rewardable;
use futures::{FutureExt, StreamExt};

use crate::{
    sim::{backend::Simulation, SimArgs, SimRes},
    simulate, Orchestrator,
};

impl<const N: usize, SIM: Simulation + Send + Sync + Copy + 'static> Orchestrator<SIM, N> {
    /// Submits `sim_amount` simulations to the Orchestrator for execution
    pub fn submit<REWARD: Rewardable + Sync + Send + 'static, const BUFFER_SIZE: usize>(
        &mut self,
        job: SimArgs<REWARD>,
        sim_amount: usize,
    ) {
        let arc_job = Arc::new(job);
        let fut = (0..sim_amount)
            .map(|_| simulate::<REWARD, N, SIM>(self.simulation_backend, arc_job.clone()).boxed());
        self.batch_sims.extend(fut)
    }

    /// Runs all batch simulations and returns the simulation with the best fitness
    pub async fn run(&mut self) -> SimRes {
        let mut results = vec![];
        while let Some(result) = self.batch_sims.next().await {
            results.push(result);
        }

        results.into_iter().max().unwrap()
    }
}
