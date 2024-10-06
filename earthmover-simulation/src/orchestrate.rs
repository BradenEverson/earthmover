//! The implementation for generating a batch of simulations and running them

use std::{sync::Arc, time::Duration};

use earthmover_achiever::goals::Rewardable;
use futures::{stream::FuturesUnordered, FutureExt, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use tracing::info;

use crate::{
    sim::{backend::Simulation, SimArgs, SimRes},
    simulate, Orchestrator,
};

impl<const N: usize, SIM: Simulation + Send + Sync + Copy + 'static> Orchestrator<SIM, N> {
    /// Submits `sim_amount` simulations to the Orchestrator for execution
    pub fn submit<REWARD: Rewardable + Sync + Send + 'static>(
        &mut self,
        job: SimArgs<REWARD, N>,
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
        let progress = ProgressBar::new(self.batch_sims.len() as u64);
        progress.set_style(
            ProgressStyle::with_template(
                "{spinner:.cyan/blue} [{elapsed_precise}] [{wide_bar:.cyan/blue}] \
                {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        progress.inc(0);
        progress.enable_steady_tick(Duration::from_millis(100));

        while let Some(result) = self.batch_sims.next().await {
            progress.inc(1);
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
