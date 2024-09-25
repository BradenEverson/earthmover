//! The agent's application cycle

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Config {
    #[arg(short = 'b', long = "body")]
    body: PathBuf,
    #[arg(short = 'g', long = "with-goals", value_delimiter=' ', num_args = 1..)]
    with_goals: Option<Vec<String>>,
    #[arg(short = 's', long = "server")]
    server: String
}

/// Initializes an achiever system via a URDF file parsed into a `Body` and any arbitrary goals set
/// through the CLI
#[tokio::main]
pub async fn main() {
    let args = Config::parse();
    println!("Todo: Parse These Args into a Body and a Goal:\n{:?}", args);
}

