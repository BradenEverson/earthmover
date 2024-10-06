//! The agent's application cycle

use std::path::PathBuf;

use clap::Parser;
use earthmover_achiever::brain::agent::Untrained;
use earthmover_achiever::goals::Goal;
use earthmover_achiever::protocol::{AhtpMessage, AhtpResponse};
use earthmover_achiever::{body::Body, brain::AgentSession};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Parser, Debug)]
/// Configuration for the achiever session from the CLI
struct Config {
    #[arg(short = 'b', long = "body")]
    /// A path to the JSON file serializing the agent's body
    body: PathBuf,
    #[arg(short = 'g', long = "with-goals", value_delimiter=' ', num_args = 1..)]
    /// An arbitrary list of usize/bool alternating pairs to create basic goals for the agent
    with_goals: Option<Vec<String>>,
    #[arg(short = 't', long = "threshold")]
    /// The threshold for when the fitness is acceptable
    threshold: f32,
    #[arg(short = 's', long = "server")]
    /// An optional server to bind to
    server: Option<String>,
}

impl Config {
    /// Parses a config into an agent's Body and Goals
    pub fn get_body_and_goals(&self) -> (Option<Body>, Option<Goal<f32>>) {
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

    let _threshold = args.threshold;
    let server_to = args.server.unwrap_or("0.0.0.0:1940".into());

    let _agent = AgentSession::<_, Untrained, 100_000>::builder()
        .with_body(&mut body)
        .with_goal(goals)
        .build()
        .unwrap();

    // Connect to server
    let body_serialized = "todo";
    let body_encoded = urlencoding::encode(body_serialized);
    let connection = format!("http://{}/initiate?urdf={}", server_to, body_encoded);

    let response = reqwest::get(connection)
        .await
        .expect("Failed to send request to server for initiation");

    if !response.status().is_success() {
        panic!("Failed to initiate server connection")
    }

    let response = response
        .text()
        .await
        .expect("Failed to fetch response body");
    let response_ahtp: AhtpResponse =
        serde_json::from_str(&response).expect("Failed to deserialize response");

    let id = response_ahtp
        .get_init()
        .expect("Response wasn't an initialization");
    // Now that we have ID, we can initialize a websocket connection

    let ws_url = format!("ws://{}", server_to);
    let (ws, _) = connect_async(ws_url)
        .await
        .expect("Failed to connect to websocket on hivemind server");

    let (mut write, mut _read) = ws.split();

    let connect_to_session = AhtpMessage::<3>::Connect(id)
        .to_json_string()
        .expect("Failed to serialize connect request");
    write
        .send(Message::Text(connect_to_session))
        .await
        .expect("Failed to send session id to hivemind socket");

    let _conditions_reached = false;
    loop {
        // Collect all data until buffer is full

        // Check assess fitness, if past threshold set conditions_reached to true and break

        // Send buffer out to hivemind server and await update to instructions.

        // Perform instructions
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
