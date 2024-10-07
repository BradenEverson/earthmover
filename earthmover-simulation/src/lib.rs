//! The simulation enviornment responsible for constructing and interpretting collected data from
//! the `agent`. Allows for concurrent reinforcement learning and modular agent construction
//! through URDF parsing to interpret agent structure.

use std::{future::Future, pin::Pin, sync::Arc};

use earthmover_achiever::goals::Rewardable;
use futures::stream::FuturesUnordered;
use sim::{
    backend::{Simulation, ValidDimension},
    SimArgs, SimMessage, SimRes,
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[cfg(test)]
use earthmover_achiever::brain::instruction::Instruction;

#[cfg(test)]
use rand::{thread_rng, Rng};

#[cfg(test)]
use rapier3d::prelude::*;

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
    sim_args: Arc<SimArgs<REWARD, N>>,
) -> SimRes
where
    [f32; N]: ValidDimension,
{
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
    fn simulate<REWARD: Rewardable, const DIMS: usize>(
        &self,
        _args: Arc<SimArgs<REWARD, DIMS>>,
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
/// A test backend that just creates dummy instructions and scores
#[derive(Clone, Copy)]
struct SimplePhysicsBackend;

#[cfg(test)]
impl Simulation for SimplePhysicsBackend {
    fn simulate<REWARD: Rewardable, const DIMS: usize>(
        &self,
        _args: Arc<SimArgs<REWARD, DIMS>>,
        message_sender: UnboundedSender<SimMessage>,
    ) {
        let _ = tracing_subscriber::fmt::try_init();
        let mut rng = thread_rng();

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
        collider_set.insert(collider);

        /* Create the bounding ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![
                rng.gen_range(0f32..3f32),
                rng.gen_range(0f32..10f32),
                0.0
            ])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, rng.gen_range(-9.81..-7.5), 0.0];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = DefaultBroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let mut query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();

        for _ in 0..20 {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut rigid_body_set,
                &mut collider_set,
                &mut impulse_joint_set,
                &mut multibody_joint_set,
                &mut ccd_solver,
                Some(&mut query_pipeline),
                &physics_hooks,
                &event_handler,
            );
        }

        let ball_body = &rigid_body_set[ball_body_handle];
        message_sender
            .send(SimMessage::Close(ball_body.translation().y as f64))
            .expect("Failed to send final ball y");
    }

    fn name(&self) -> String {
        "Simple Physics Informed Backend - Rapier 3d".into()
    }
}

#[cfg(test)]
mod tests {
    use earthmover_achiever::body::Body;

    use crate::{sim::SimArgs, Orchestrator, SimpleBackend, SimplePhysicsBackend};

    #[tokio::test]
    async fn orchestrator_simple_simulation_backend() {
        let mut orchestrator: Orchestrator<_, 3> = Orchestrator::new(SimpleBackend);
        orchestrator.submit(SimArgs::new(1.0, vec![], Body::default()), 1000);
        let _ = orchestrator.run().await;
    }

    #[tokio::test]
    async fn orchestrator_physics_informed_backend() {
        let mut orchestrator: Orchestrator<_, 3> = Orchestrator::new(SimplePhysicsBackend);
        orchestrator.submit(SimArgs::new(1.0, vec![], Body::default()), 1000);
        let _ = orchestrator.run().await;
    }
}
