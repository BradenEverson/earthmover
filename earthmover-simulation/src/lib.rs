//! The simulation enviornment responsible for constructing and interpretting collected data from
//! the `agent`. Allows for concurrent reinforcement learning and modular agent construction
//! through URDF parsing to interpret agent structure.

use std::{future::Future, pin::Pin, sync::Arc};

use earthmover_achiever::goals::Rewardable;
use futures::stream::FuturesUnordered;
use sim::{backend::Simulation, SimArgs, SimMessage, SimRes};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[cfg(test)]
use earthmover_achiever::brain::instruction::Instruction;
#[cfg(test)]
use rand::{thread_rng, Rng};
use tracing::info;

pub mod orchestrate;
pub mod sim;

/// The future responsible for fully executing a simulation
type SimulationExecution<OUT> = Pin<Box<dyn Future<Output = OUT> + Send>>;

/// Asynchronous function responsible for constructing and then simulating an environment given a
/// collection of N-dimensional points, an agent's configuration(hardware alongside current
/// angles/position) and a `GOAL` function
pub async fn simulate<
    REWARD: Rewardable + Send + Sync + 'static,
    const N: usize,
    SIM: Simulation + Send + Sync + 'static,
>(
    simulation_backend: SIM,
    sim_args: Arc<SimArgs<REWARD>>,
) -> SimRes {
    info!("Beginning simulation...");
    let mut res = SimRes::default();
    let (sender, mut receiver): (UnboundedSender<SimMessage>, UnboundedReceiver<SimMessage>) =
        mpsc::unbounded_channel();

    let args_clone = sim_args.clone();
    tokio::spawn(async move { simulation_backend.simulate(args_clone, sender) });

    while let Some(msg) = receiver.recv().await {
        match msg {
            SimMessage::Instruction(instr) => res.push_instruction(instr),
            SimMessage::Close(score) => {
                info!("Final Score: {score}");
                res.set_score(score);
                break;
            }
        }
    }

    res
}

/// The struct responsible for running a collection of N simulations
pub struct Orchestrator<SIM: Simulation + Send + Sync + Copy + 'static, const N: usize> {
    /// All simulations currently being run
    batch_sims: FuturesUnordered<SimulationExecution<SimRes>>,
    /// The simulation's backend
    simulation_backend: SIM,
}

#[cfg(test)]
/// A test backend that just creates dummy instructions and scores
#[derive(Clone, Copy)]
struct SimpleBackend;

#[cfg(test)]
impl Simulation for SimpleBackend {
    fn simulate<REWARD: Rewardable>(
        &self,
        _args: Arc<SimArgs<REWARD>>,
        message_sender: UnboundedSender<SimMessage>,
    ) {
        let _ = tracing_subscriber::fmt::try_init();
        let mut rng = thread_rng();
        for _ in 0..rng.gen_range(0..10) {
            let instruction = Instruction::default();
            message_sender
                .send(SimMessage::Instruction(instruction))
                .expect("Send failed :(");
        }

        message_sender
            .send(SimMessage::Close(rng.gen_range(0f64..1f64)))
            .expect("Failed to close out simulation");
    }

    fn name(&self) -> String {
        "Simple Backend".into()
    }
}

#[cfg(test)]
mod tests {
    use earthmover_achiever::body::Body;

    use crate::{sim::SimArgs, Orchestrator, SimpleBackend};

    #[tokio::test]
    async fn orchestrator_simple_simulation_backend() {
        let mut orchestrator: Orchestrator<SimpleBackend, 3> = Orchestrator::new(SimpleBackend);
        orchestrator.submit(SimArgs::new(1.0, vec![], Body::default()), 1_000_000);
        let _ = orchestrator.run().await;
    }
}
