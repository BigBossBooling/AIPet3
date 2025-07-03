//! CritterChain Node Entry Point
//!
//! # The Architect's Vision
//!
//! This file serves as the main entry point for the CritterChain node. It embodies
//! the KISS principle by acting as a clean, high-level dispatcher. Its sole
//! responsibility is to parse command-line arguments and delegate control to the
//! appropriate, specialized modules for execution.
//!
//! It orchestrates the bootstrapping of the entire Critter-Craft digital ecosystem.

#![warn(missing_docs)]

use critterchain_node::{cli::Cli, command}; // Use our custom CLI and command runner
use sc_cli::SubstrateCli;

/// The main entry point for the CritterChain node.
#[tokio::main]
async fn main() -> sc_cli::Result<()> {
    // --- (K) Know Your Core, Keep it Clear ---
    // The main function has a single, clear responsibility:
    // to initialize the system and execute the requested command.

    // 1. Initialize Logging & Telemetry
    // Sets up the logging framework, which is essential for debugging and monitoring.
    // The verbosity is controlled by the `RUST_LOG` environment variable.
    sc_cli::init_logger("");

    // 2. Parse Command-Line Arguments
    // `Cli::from_args()` uses the `clap` derive macro on our custom `cli::Cli`
    // struct to parse all command-line arguments into a structured format.
    let cli = Cli::from_args();

    // 3. Dispatch to the Appropriate Command Runner
    // This is the core dispatch logic. It separates the standard node-running
    // command from our custom, game-specific administrative commands.
    match &cli.subcommand {
        Some(subcommand) => {
            // --- (S) Systematize for Scalability ---
            // The `run` method of the `SubstrateCli` trait handles all standard
            // Substrate commands (e.g., `build-spec`, `purge-chain`).
            // Our custom commands are handled by a dedicated runner.
            let runner = cli.create_runner(subcommand)?;
            runner.run_subcommand(subcommand)
        }
        None => {
            // --- The Default Action: Run the Node ---
            // If no subcommand is provided, the default action is to run the node.
            // The `run_cmd` function encapsulates all the complexity of setting up
            // the service, network, and consensus layers.
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                // Here, we delegate the complex task of building and running the
                // service to our dedicated `service` module.
                command::run(config).await
            })
        }
    }
}