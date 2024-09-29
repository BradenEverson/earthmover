//! The simulation enviornment responsible for constructing and interpretting collected data from
//! the `agent`. Allows for concurrent reinforcement learning and modular agent construction
//! through URDF parsing to interpret agent structure.

use std::{future::Future, pin::Pin, sync::Arc};

use earthmover_achiever::goals::Rewardable;
use futures::stream::FuturesUnordered;
use sim::{backend::Simulation, SimArgs, SimMessage, SimRes};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub mod orchestrate;
pub mod sim;

/// The future responsible for fully executing a simulation
type SimulationExecution<OUT> = Pin<Box<dyn Future<Output = OUT> + Send>>;

/// Asynchronous function responsible for constructing and then simulating an environment given a
/// collection of N-dimensional points, an agent's configuration(hardware alongside current
/// angles/position) and a `GOAL` function
pub async fn simulate<'agent, REWARD: Rewardable, const N: usize, const BUFFER_SIZE: usize, SIM: Simulation + Sync>(
    simulation_backend: SIM,
    args: Arc<SimArgs<'agent, REWARD, BUFFER_SIZE>>,
) -> SimRes {
    let mut res = SimRes::default();
    let (sender, mut receiver): (UnboundedSender<SimMessage>, UnboundedReceiver<SimMessage>) =
        mpsc::unbounded_channel();

    tokio::spawn(async move {
        simulation_backend.simulate(args.clone(), sender)
    });

    while let Some(msg) = receiver.recv().await {
        match msg {
            SimMessage::Instruction(instr) => res.push_instruction(instr),
            SimMessage::Close(score) => {
                res.set_score(score);
                break;
            }
        }
    }

    res
}

/// The struct responsible for running a collection of N simulations
pub struct Orchestrator<const N: usize> {
    /// All simulations currently being run
    batch_sims: FuturesUnordered<SimulationExecution<SimRes>>,
}
