//! The implementation for generating a batch of simulations and running them

use std::sync::Arc;

use earthmover_achiever::goals::Rewardable;
use futures::{stream::FuturesUnordered, FutureExt, StreamExt};
use tracing::info;

use crate::{
    sim::{backend::Simulation, SimArgs, SimRes},
    simulate, Orchestrator,
};

impl<const N: usize, SIM: Simulation + Send + Sync + Copy + 'static> Orchestrator<SIM, N> {
    /// Submits `sim_amount` simulations to the Orchestrator for execution
    pub fn submit<REWARD: Rewardable + Sync + Send + 'static>(
        &mut self,
        job: SimArgs<REWARD>,
        sim_amount: usize,
    ) {
        info!(
            "Submitting {sim_amount} simulation requests to backend {}",
            self.simulation_backend.name()
        );
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

    /// Creates a new Orchestrator based on a given simulation backend
    pub fn new(sim: SIM) -> Self {
        Self {
            batch_sims: FuturesUnordered::new(),
            simulation_backend: sim,
        }
    }
}
