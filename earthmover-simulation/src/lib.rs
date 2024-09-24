//! The simulation enviornment responsible for constructing and interpretting collected data from
//! the `agent`. Allows for concurrent reinforcement learning and modular agent construction
//! through URDF parsing to interpret agent structure. 

use std::{future::Future, pin::Pin};

use futures::stream::FuturesUnordered;
use sim::{SimArgs, SimRes};

pub mod sim;
pub mod orchestrate;

type SimulationExecution<OUT> = Pin<Box<dyn Future<Output = OUT> + Send>>;

/// Asynchronous function responsible for constructing and then simulating an environment given a
/// collection of N-dimensional points, an agent's configuration(hardware alongside current
/// angles/position) and a `GOAL` function
pub async fn simulate<const N: usize>(args: impl AsRef<SimArgs>) -> SimRes {
    let _sim_args = args.as_ref();

    todo!()
}

/// The struct responsible for running a collection of N simulations
pub struct Orchestrator<const N: usize> {
    batch_sims: FuturesUnordered<SimulationExecution<SimRes>>,
}
