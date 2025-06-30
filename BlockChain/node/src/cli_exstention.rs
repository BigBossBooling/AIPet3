//! CritterChain Node CLI Extensions
//!
//! This module defines custom command-line utilities and subcommands for CritterChain.
//! These tools enable node operators and developers to perform specific operational,
//! debugging, and administrative tasks beyond standard Substrate CLI commands.
//!
//! Meticulously crafted to align with The Architect's vision for
//! modularity, scalability, and robust operation of the CritterCraft digital ecosystem.

#![warn(missing_docs)] // Ensure all public API are documented

use clap::{Parser, Subcommand}; // CLI argument parsing traits
use sc_cli::{CliConfiguration, RunCmd, SubstrateCli}; // Standard Substrate CLI components
use std::fmt::Debug; // For Debug trait derivation
use std::path::PathBuf; // For file path arguments

/// The main CLI struct for CritterChain, extending standard Substrate CLI.
/// This struct acts as the top-level command for all node operations.
#[derive(Debug, Parser)]
#[clap(name = "CritterChain Node")] // The main executable name
#[clap(version = env!("CARGO_PKG_VERSION"))] // Use Cargo.toml version
#[clap(about = "CritterChain: A next-generation blockchain for digital pets and games.", long_about = None)] // Description from Cargo.toml or custom
#[clap(author = env!("CARGO_PKG_AUTHORS"))] // Authors from Cargo.toml
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>, // Use the unified `Subcommand` enum

    #[clap(flatten)]
    pub run: RunCmd, // Standard Substrate `run` command
}

/// A unified enum for all Substrate's default subcommands and our custom ones.
/// This centralizes all CLI functionality into a single entry point.
#[derive(Debug, Subcommand)]
pub enum Subcommand {
    /// Standard key management CLI utilities.
    #[command(flatten)]
    Key(sc_cli::KeySubcommand),
    /// Build a chain specification.
    #[command(flatten)]
    BuildSpec(sc_cli::BuildSpecCmd),
    /// Validate blocks.
    #[command(flatten)]
    CheckBlock(sc_cli::CheckBlockCmd),
    /// Export blocks.
    #[command(flatten)]
    ExportBlocks(sc_cli::ExportBlocksCmd),
    /// Export the state of a genesis block.
    #[command(flatten)]
    ExportState(sc_cli::ExportStateCmd),
    /// Import blocks.
    #[command(flatten)]
    ImportBlocks(sc_cli::ImportBlocksCmd),
    /// Remove the whole chain.
    #[command(flatten)]
    PurgeChain(sc_cli::PurgeChainCmd),
    /// Revert the chain to a previous state.
    #[command(flatten)]
    Revert(sc_cli::RevertCmd),
    /// Sub-commands concerned with benchmarking.
    #[command(flatten)]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
    /// Try some command against runtime state.
    #[command(flatten)]
    TryRuntime(sc_cli::TryRuntimeCmd),
    /// Db subcommands.
    #[command(flatten)]
    Db(sc_cli::DbCmd),

    // --- CritterChain Custom Subcommands ---
    /// Example: Manipulate genesis data for CritterChain.
    /// This could be used for specific testnet setups or debugging.
    #[command(name = "genesis-tool", about = "Custom utility for genesis manipulation.")]
    GenesisTool {
        /// Path to the genesis file to operate on.
        #[clap(long, value_name = "FILE")]
        file: PathBuf, // Use PathBuf for file paths
        /// Optional: New output file for modified genesis.
        #[clap(long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        // Add more specific options for the genesis tool
    },

    /// Example: Perform a custom runtime upgrade.
    /// This would typically involve submitting an unsigned extrinsic for a `set_code` call.
    #[command(name = "custom-upgrade", about = "Perform a custom runtime upgrade via WASM blob.")]
    CustomUpgrade {
        /// Path to the new runtime Wasm blob.
        #[clap(long, value_name = "WASM_FILE")]
        wasm: PathBuf, // Use PathBuf for file paths
        /// Sudo key URI to sign the upgrade extrinsic (e.g., //Alice).
        #[clap(long, value_name = "SUDO_URI")]
        sudo_key: String,
        /// RPC endpoint to connect to (e.g., ws://127.0.0.1:9944).
        #[clap(long, value_name = "RPC_URL")]
        rpc_url: String,
    },

    // Add more custom subcommands as needed for CritterChain specific operations.
    // E.g., `set-pet-dna`, `admin-mint-critter`, `query-marketplace` (if not exposed via RPC)
    // #[command(name = "admin-mint-critter", about = "Admin utility to mint a new critter to an account.")]
    // AdminMintCritter {
    //     #[clap(long, value_name = "ACCOUNT_ID")]
    //     to: String,
    //     #[clap(long, value_name = "SPECIES")]
    //     species: String,
    //     #[clap(long, value_name = "NAME")]
    //     name: String,
    // },
}

/// Implement the `SubstrateCli` trait for our custom `Cli` struct.
/// This provides the meta-information about the CLI application.
impl SubstrateCli for Cli {
    /// The name of the implementation (e.g., "CritterChain Node").
    fn impl_name() -> &'static str {
        "CritterChain Node"
    }
    /// The version of the implementation (read from Cargo.toml).
    fn impl_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    /// A short description of the CLI application.
    fn description() -> &'static str {
        "CritterChain: A next-generation blockchain for digital pets and games."
    }
    /// The author(s) of the implementation (read from Cargo.toml).
    fn author() -> &'static str {
        env!("CARGO_PKG_AUTHORS")
    }
    /// The support URL for the project.
    fn support_url() -> &'static str {
        "https://github.com/BigBossBooling/AIPet3" // Updated to actual GitHub repo
    }
    /// The year copyright began.
    fn copyright_start_year() -> i32 {
        2024 // Updated to 2024 for project year
    }
    // No need to implement `chain_spec` here if it's handled in `main.rs` directly
    // or by custom `build_chain_spec` in the `service` module.
    // Also `chain_spec` requires `ChainSpec` trait, so direct impl is for basic CLI.

    /// Get a run command instance from the top-level `Cli` struct.
    fn run_cmd(&self) -> &RunCmd {
        &self.run
    }
}
    /// Get the configuration for the CLI application.
    fn cli_configuration(&self) -> &dyn CliConfiguration {
        &self.run
    }
}