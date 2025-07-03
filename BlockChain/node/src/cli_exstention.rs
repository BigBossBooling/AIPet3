//! CritterChain Node CLI Extensions: The Architect's Toolkit
//!
//! # Strategic Vision
//!
//! This module defines the command-line interface for CritterChain, applying the
//! Expanded KISS Principle to create a tool that is both powerful and clear. It
//! separates standard node operations from a dedicated suite of custom,
//! game-specific utilities designed for administration, debugging, and ecosystem management.
//!
//! This is the primary interface for node operators and the core development team
//! to interact with and maintain the Critter-Craft Universe at its deepest level.

#![warn(missing_docs)]

use clap::{Parser, Subcommand};
use sc_cli::{CliConfiguration, RunCmd, SubstrateCli};
use std::path::PathBuf;
use crate::chain_spec; // Assuming chain_spec is in the same crate root

/// The main CLI entry point for the CritterChain node.
#[derive(Debug, Parser)]
#[clap(
    name = "CritterChain Node",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "The core node for the Critter-Craft digital ecosystem."
)]
pub struct Cli {
    #[command(subcommand)]
    /// The subcommand to execute.
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    /// Standard node running options.
    pub run: RunCmd,
}

/// A unified enum for all available subcommands.
/// It cleanly separates standard Substrate tools from our custom `critter-admin` suite.
#[derive(Debug, Subcommand)]
pub enum Subcommand {
    /// Standard key management utilities.
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

    /// Database management commands.
    #[command(flatten)]
    Db(sc_cli::DbCmd),

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

    /// Try-runtime utilities.
    #[command(flatten)]
    TryRuntime(sc_cli::TryRuntimeCmd),

    /// (K) A dedicated suite of custom commands for managing the Critter-Craft ecosystem.
    #[command(subcommand, name = "critter-admin", about = "Critter-Craft specific administration and debugging tools.")]
    CritterAdmin(CritterAdminCmds),
}

/// (S) Defines the set of custom commands for Critter-Craft administration.
/// Each variant is a self-contained command struct with its own logic.
#[derive(Debug, Subcommand)]
pub enum CritterAdminCmds {
    /// (I) Mint a new Genesis Pet NFT to a specified account. Requires Sudo privileges.
    AdminMint(AdminMintCmd),

    /// (I) A powerful debugging tool to query raw storage values from a live node.
    QueryStorage(QueryStorageCmd),

    /// (S) Validate a new runtime WASM blob before submitting a `set_code` upgrade.
    ValidateRuntime(ValidateRuntimeCmd),
}

/// Command to mint a new Genesis Pet.
#[derive(Debug, Parser)]
pub struct AdminMintCmd {
    /// The AccountId (in SS58 format) of the new owner.
    #[arg(long)]
    pub owner: String,

    /// The species archetype key (e.g., 'sprite_glow').
    #[arg(long)]
    pub species: String,

    /// The aura color key (e.g., 'aura-blue').
    #[arg(long)]
    pub aura: String,

    #[command(flatten)]
    pub run_cmd: RunCmd, // Reuse run command for node connection details
}

/// Command to query on-chain storage.
#[derive(Debug, Parser)]
pub struct QueryStorageCmd {
    /// The hexadecimal key of the storage item to query.
    pub key: String,

    #[command(flatten)]
    pub run_cmd: RunCmd,
}

/// Command to validate a runtime WASM file.
#[derive(Debug, Parser)]
pub struct ValidateRuntimeCmd {
    /// The file path to the runtime WASM blob.
    pub wasm_path: PathBuf,
}

// --- Command Execution Logic (S - Systematize for Scalability) ---

impl CritterAdminCmds {
    /// Runs the selected admin command. This systematizes execution.
    pub fn run(&self) -> sc_cli::Result<()> {
        match self {
            CritterAdminCmds::AdminMint(cmd) => cmd.run(),
            CritterAdminCmds::QueryStorage(cmd) => cmd.run(),
            CritterAdminCmds::ValidateRuntime(cmd) => cmd.run(),
        }
    }
}

impl AdminMintCmd {
    /// Executes the logic for minting a pet.
    fn run(&self) -> sc_cli::Result<()> {
        println!("Connecting to node to execute admin-mint...");
        println!("  Owner: {}", self.owner);
        println!("  Species: {}", self.species);
        println!("  Aura: {}", self.aura);
        // In a real implementation, this would connect to the node via RPC,
        // construct an `unsubmittable_extrinsic`, and submit it with the
        // appropriate sudo key.
        println!("✅ (Simulation) Extrinsic to mint pet has been submitted.");
        Ok(())
    }
}

impl QueryStorageCmd {
    /// Executes the logic for querying storage.
    fn run(&self) -> sc_cli::Result<()> {
        println!("Connecting to node to query storage...");
        println!("  Storage Key: {}", self.key);
        // In a real implementation, this would use an RPC client to call
        // `state_getStorage` with the provided key.
        println!("✅ (Simulation) Query successful. Value: 0x... (data placeholder)");
        Ok(())
    }
}

impl ValidateRuntimeCmd {
    /// Executes the logic for validating a WASM blob.
    fn run(&self) -> sc_cli::Result<()> {
        println!("Validating runtime at: {:?}", self.wasm_path);
        // In a real implementation, this would load the WASM file and perform
        // checks, such as verifying its metadata and API versions.
        if !self.wasm_path.exists() {
            return Err("WASM file not found at the specified path.".into());
        }
        println!("✅ (Simulation) Runtime WASM appears valid and well-formed.");
        Ok(())
    }
}

// --- Substrate CLI Trait Implementation ---

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "CritterChain Node".into()
    }

    fn impl_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn description() -> String {
        "CritterChain: A next-generation blockchain for digital pets and games.".into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/BigBossBooling/AIPet3".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()?),
            "" | "local" => Box::new(chain_spec::local_testnet_config()?),
            path => Box::new(chain_spec::GenericChainSpec::from_json_file(path.into())?),
        })
    }
}