//! The agent's application cycle

use std::path::PathBuf;

use clap::Parser;

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

/// Initializes an achiever system via a URDF file parsed into a `Body` and any arbitrary goals set
/// through the CLI
#[tokio::main]
pub async fn main() {
    let args = Config::parse();
    println!("Todo: Parse These Args into a Body and a Goal:\n{:?}", args);
}
