//! Example for visualizing the Bevy physics simulation

use earthmover_achiever::body::Body;
use earthmover_simulation::sim::{
    backend::{physics::BevyPhysicsInformedBackend, Simulation},
    SimArgs,
};
use rand::{thread_rng, Rng};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Runs the Bevy Simulation with default args
pub fn main() {
    let backend = BevyPhysicsInformedBackend;
    let messages = mpsc::unbounded_channel();

    let mut data = vec![];

    let mut rng = thread_rng();
    for _ in 0..10_000 {
        let x = rng.gen_range(-3f32..3f32);
        let y = rng.gen_range(0f32..3f32);
        let z = f32::cos(x);

        data.push([x, y, z]);
    }

    let args: SimArgs<f32, 3> = SimArgs::new(0f32, data, Body::default());

    backend.simulate(Arc::new(args), messages.0)
}
