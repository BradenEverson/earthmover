//! The agent's application cycle

use std::path::PathBuf;

use clap::Parser;
use earthmover_achiever::brain::agent::Untrained;
use earthmover_achiever::goals::Goal;
use earthmover_achiever::{body::Body, brain::AgentSession};

#[derive(Parser, Debug)]
/// Configuration for the achiever session from the CLI
struct Config {
    #[arg(short = 'b', long = "body")]
    /// A path to the JSON file serializing the agent's body
    body: PathBuf,
    #[arg(short = 'g', long = "with-goals", value_delimiter=' ', num_args = 1..)]
    /// An arbitrary list of usize/bool alternating pairs to create basic goals for the agent
    with_goals: Option<Vec<String>>,
    #[arg(short = 's', long = "server")]
    /// An optional server to bind to
    server: Option<String>,
}

impl Config {
    /// Parses a config into an agent's Body and Goals
    pub fn get_body_and_goals(&self) -> (Option<Body>, Option<Vec<Goal<f32>>>) {
        todo!()
    }
}

/// Initializes an achiever system via a URDF file parsed into a `Body` and any arbitrary goals set
/// through the CLI
#[tokio::main]
pub async fn main() {
    let args = Config::parse();

    //Todo: Parse These Args into a Body and a Goal
    let (body, goals) = args.get_body_and_goals();
    let mut body = body.unwrap();
    let goals = goals.unwrap();

    let _agent = AgentSession::<_, Untrained, 100_000>::builder()
        .with_body(&mut body)
        .with_goal(Goal::Complex(goals))
        .build()
        .unwrap();

    loop {}
}
