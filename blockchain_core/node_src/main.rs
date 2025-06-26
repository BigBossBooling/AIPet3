//! Substrate Node Template and CLI
//!
//! This node template is a fork of the `substrate-node-template` from Parity Technologies,
//! adapted for the specific needs of the CritterCraft blockchain.
//! It serves as the entry point for running a full CritterChain node, supporting
//! both full and light client roles with Babe/Grandpa consensus.
//!
//! This file has been meticulously crafted to align with The Architect's vision,
//! ensuring a robust foundation for the CritterCraft digital ecosystem.

#![warn(missing_docs)] // Ensure all public API are documented
#![warn(unused_extern_crates)] // Warn about unused crates

// Standard library imports
use std::{path::PathBuf, sync::Arc};

// External Crates - Commonly used by Substrate nodes
use clap::Parser; // CLI argument parsing
use log::info;    // For logging (used via `env_logger`)

// Substrate CLI and Service Components
use sc_cli::{
    build_network, // Helper for building the network service
    ChainSpec,     // Trait for chain specifications
    CliConfiguration, // Trait for CLI configuration
    Executor,      // The WASM executor trait
    LocalCallExecutor, // For local RPC calls
    RpcHandlers,   // Type for RPC handlers
    RunCmd,        // Standard `run` command for `sc_cli`
    Subcommand,    // Trait for CLI subcommands
    SubstrateCli,  // Main Substrate CLI struct
    TaskManager,   // For managing asynchronous tasks
    build_rpc_interface, // Helper for building RPCs
    PartialComponents, // Struct for partial service components
};
use sc_service::{
    config::Configuration, // Core service configuration
    error::Error as ServiceError, // Service-specific errors
    ChainType,     // Type of chain (Development, Local, Live)
    BasePath,      // Base path for database/config
    GenericChainSpec, // Generic ChainSpec type
    new_full_parts, // Helper for building full node parts
    new_light_parts, // Helper for building light node parts
    SpawnTasksParams, // Parameters for spawning service tasks
    TFullBackend,  // Type alias for full node backend
    TFullClient,   // Type alias for full node client
    TFullCallApi,  // Type alias for full node call API
    DenyUnsafe,    // RPC security option
    RpcExtension, // RPC extension trait
};
use sc_rpc::SubscriptionTaskExecutor; // For RPC subscriptions
use sc_consensus_babe::{
    BabeBlockImport,        // Babe-specific block import
    BabeConfiguration,      // Babe configuration
    BabeConsensusDataProvider, // Data provider for Babe
    BabeLink,               // Link struct for Babe consensus
};
use sc_consensus_grandpa::{
    FinalityProofProvider as GrandpaFinalityProofProvider, // Grandpa-specific finality proof provider
    GrandpaBlockImport,     // Grandpa-specific block import
    GrandpaJustificationStream, // Stream of Grandpa justifications
    GrandpaLink,            // Link struct for Grandpa consensus
    SharedVoterState,       // Shared state for Grandpa voters
    GrandpaApi,             // Trait for Grandpa RPCs
};
use sc_client_api::RemoteBackend; // For accessing remote backend
use sp_consensus_babe::AuthorityId as BabeId; // Babe authority ID type
use sp_consensus_grandpa::AuthorityId as GrandpaId; // Grandpa authority ID type
use sp_core::crypto::Ss58AddressFormat; // SS58 address format
use sp_runtime::traits::Block as BlockT; // Block trait
use sp_keystore::KeystorePtr; // Keystore pointer type
use sc_telemetry::{TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle}; // Telemetry components
use sc_transaction_pool::{BasicPool, FullPool, LightPool}; // Transaction pool implementations
use sc_transaction_pool_api::MaintainedTransactionPool; // Transaction pool API

// RPCs
use jsonrpsee::RpcModule; // JSON-RPC server module
use sc_rpc::system::SystemApiServer; // System RPC
use sc_rpc::tx_pool::TransactionPoolApiServer; // Transaction pool RPC
use sc_rpc::system_local::SystemLocalApiServer; // System local RPC
use sc_rpc_api::DenyUnsafe as RpcDenyUnsafe; // RPC deny unsafe trait

// Consensus RPCs
use sc_consensus_babe_rpc::BabeApiServer;
use sc_consensus_grandpa_rpc::GrandpaApiServer;

// CritterChain Specific Imports
// This assumes your runtime is in a crate named `critterchain_runtime`
use critterchain_runtime::{
    self,               // The runtime crate itself
    opaque::Block,      // The opaque Block type from runtime
    AccountId,          // Account ID type
    Balance,            // Balance type
    Nonce,              // Nonce type
    RuntimeApi,         // Runtime API trait
    RuntimeExecutor,    // The executor for the WASM runtime
    EXISTENTIAL_DEPOSIT, // Existential deposit constant
};

/// The default CritterChain chain specification.
fn chain_spec() -> Result<Box<dyn ChainSpec>, String> {
    Ok(Box::new(GenericChainSpec::from_json_bytes(
        // This is a placeholder; in a real project, you'd load your actual chain spec.
        // E.g., include_bytes!("../chain_spec.json").to_vec()
        // For development, use a hardcoded dev chain spec or load from file.
        // For a minimal working example, we'll return a simple development chain.
        r#"
        {
          "name": "CritterChain Development",
          "id": "critterchain_dev",
          "chainType": "Development",
          "genesis": {
            "runtime": {
              "system": {
                "code": "0x00..."
              },
              "pallet_balances": {
                "balances": [
                  ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000]
                ]
              },
              "pallet_babe": {
                "authorities": []
              },
              "pallet_grandpa": {
                "authorities": []
              }
            }
          },
          "protocolId": "cc",
          "properties": {
            "tokenSymbol": "PTCN",
            "tokenDecimals": 10
          }
        }
        "#.as_bytes().to_vec(),
    )?))
}

/// Node CLI Definition.
#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[clap(flatten)]
    pub run: RunCmd, // Standard run command for `sc_cli`
}

/// CLI Subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Key management CLI utilities
    Key(sc_cli::KeySubcommand),
    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),
    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),
    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),
    /// Export the state of a genesis block.
    ExportState(sc_cli::ExportStateCmd),
    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),
    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),
    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),
    /// Sub-commands concerned with benchmarking.
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
    /// Try some command against runtime state.
    #[command(name = "try-runtime")]
    TryRuntime(sc_cli::TryRuntimeCmd),
    /// Db subcommands
    Db(sc_cli::DbCmd),
}

// Custom service module, typically located in `node/src/service.rs`
// This module contains the logic for building full and light nodes.
pub mod service {
    use super::*;
    use sc_service::{
        BuildNetworkParams, SpawnTasksParams, LightNetworkConfiguration, FullNetworkConfiguration,
        config::TelemetryEndpoints,
    };
    use sc_executor::NativeElseWasmExecutor;
    use sc_consensus_babe::{
        BabeProposer, BabeConfiguration, BabeBlockImport, BabeLink, BabeConsensusDataProvider
    };
    use sc_consensus_grandpa::{
        GrandpaBlockImport, GrandpaLink, SharedVoterState, GrandpaFinalityProofProvider,
        GrandpaParams, run_grandpa_voter
    };
    use sp_consensus_babe::AuthorityId as BabeId;
    use sp_consensus_grandpa::AuthorityId as GrandpaId;
    use sp_core::{BlockId, H256, Public};
    use sc_rpc::system::SystemRpcIdProvider;
    use std::{collections::BTreeMap, time::Duration};

    /// Full client type alias.
    pub type FullClient = TFullClient<Block, RuntimeApi, Executor>;
    /// Full backend type alias.
    pub type FullBackend = TFullBackend<Block>;
    /// Full transaction pool.
    pub type FullTransactionPool = BasicPool<FullClient, Block>;
    /// Light transaction pool.
    pub type LightTransactionPool = LightPool<Block, FullClient, sc_network::NetworkService<Block, H256>>;

    // Define our Executor type for the runtime WASM.
    pub type Executor = NativeElseWasmExecutor<critterchain_runtime::RuntimeExecutor>;

    /// Builds a new service for a partial client (used by CLI subcommands).
    pub fn new_partial(config: &mut Configuration) -> Result<
        PartialComponents<
            FullClient,
            FullBackend,
            sc_consensus::LongestChain<FullBackend, Block>,
            sc_consensus_babe::BabeImportQueue<Block, FullClient>,
            BabeLink<Block>,
            (
                GrandpaBlockImport<FullBackend, Block, FullClient, sc_consensus::LongestChain<FullBackend, Block>>,
                GrandpaLink<Block>,
                Option<TelemetryHandle>,
            )
        >,
        ServiceError
    > {
        let executor = Arc::new(Executor::new(
            config.wasm_method,
            config.default_heap_pages,
            config.max_runtime_instances,
            config.runtime_cache_size,
        ));

        // Telemetry setup
        let telemetry = config
            .telemetry_endpoints
            .clone()
            .filter(|x| !x.is_empty())
            .map(|endpoints| -> Result<_, sc_telemetry::Error> {
                let worker = TelemetryWorker::new(16)?;
                let telemetry = worker.handle().new_telemetry_handle();
                for endpoint in endpoints {
                    worker.add_fallback_log_target(endpoint.clone())?;
                }
                Ok((worker, telemetry))
            })
            .transpose()?;

        let (client, backend, keystore_container, task_manager) =
            new_full_parts::<Block, RuntimeApi, _>(
                config,
                telemetry.as_ref().map(|(_, handle)| handle.clone()),
                executor.clone(),
            )?;

        let client = Arc::new(client);
        let telemetry_handle = telemetry.as_ref().map(|(_, handle)| handle.clone());

        let select_chain = sc_consensus::LongestChain::new(backend.clone());

        let transaction_pool = BasicPool::new_full(
            config.transaction_pool.clone(),
            config.role.is_authority().into(),
            config.prometheus_registry(),
            task_manager.spawn_essential_handle(),
            client.clone(),
        );
        let transaction_pool = Arc::new(transaction_pool);

        let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
            client.clone(),
            &(client.clone() as Arc<_>), // Cast to reference
            select_chain.clone(),
            telemetry.as_ref().map(|(_, T)| T.handle()),
        )?;

        let (block_import, babe_link) = sc_consensus_babe::block_import_authority_discovery(
            BabeConfiguration::get_or_compute(&*client)?,
            grandpa_block_import.clone(),
            client.clone(),
            None, // No authority discovery for this example, or provide a service.
            backend.clone(),
            task_manager.spawn_handle(),
            telemetry.as_ref().map(|(_, T)| T.handle()),
        )?;

        let import_queue = sc_consensus_babe::import_queue(
            babe_link.clone(),
            block_import,
            Some(Box::new(grandpa_block_import.clone())),
            client.clone(),
            select_chain.clone(),
            task_manager.spawn_handle(),
            config.prometheus_registry(),
            telemetry.as_ref().map(|(_, T)| T.handle()),
            None, // No authority discovery receiver
        )?;

        Ok(PartialComponents {
            client,
            backend,
            task_manager,
            import_queue,
            keystore_container,
            select_chain,
            transaction_pool,
            other: (grandpa_block_import, grandpa_link, telemetry_handle),
        })
    }

    /// Builds a new service for a full client.
    pub fn new_full(mut config: Configuration) -> Result<(
        TaskManager,
        Arc<FullClient>,
        Arc<sc_network::NetworkService<Block, <Block as BlockT>::Hash>>,
        Arc<FullTransactionPool>,
        RpcHandlers,
        Option<TelemetryWorkerHandle>
    ), ServiceError> {
        let PartialComponents {
            client,
            backend,
            mut task_manager,
            import_queue,
            keystore_container,
            transaction_pool,
            select_chain,
            other: (grandpa_block_import, grandpa_link, mut telemetry_handle),
        } = new_partial(&mut config)?;

        let (network, system_rpc_tx, network_starter) = {
            let net_config = FullNetworkConfiguration::new(&config.network);
            sc_service::build_network(BuildNetworkParams {
                config: &config,
                client: client.clone(),
                transaction_pool: transaction_pool.clone(),
                spawn_handle: task_manager.spawn_handle(),
                import_attach_handle: Some(import_queue),
                custom_peers_set: Default::default(),
                block_announce_validator_builder: None,
                net_config,
                warp_sync_service: Some(Arc::new(GrandpaFinalityProofProvider::new(
                    backend.clone(),
                    grandpa_link.shared_authority_set().clone(),
                    Vec::default(), // No custom finality proof stream
                ))),
            })?
        };
        let network = Arc::new(network);
        let telemetry_worker_handle = telemetry_handle.as_ref().map(|w| w.handle().clone());

        let rpc_extensions_builder = {
            let client = client.clone();
            let pool = transaction_pool.clone();
            let keystore = keystore_container.sync_keystore();
            let justification_stream = grandpa_link.justification_stream();
            let shared_authority_set = grandpa_link.shared_authority_set().clone();
            let shared_voter_state = grandpa_link.shared_voter_state();
            let deny_unsafe = config.rpc_port.is_some() && config.role.is_authority() && config.rpc_cors.is_empty(); // Example deny_unsafe logic

            Box::new(move |subscription_executor: Arc<SubscriptionTaskExecutor>| {
                let mut io = RpcModule::new(());
                io.merge(SystemApiServer::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
                io.merge(TransactionPoolApiServer::new(client.clone(), pool.clone()).into_rpc())?;
                io.merge(SystemLocalApiServer::new(client.clone(), keystore.clone()).into_rpc())?;
                io.merge(sc_rpc::system::AccountSyncApiServer::new(client.clone(), keystore.clone()).into_rpc())?; // Added for account sync
                
                // Add custom RPC for critterchain_runtime
                io.merge(critterchain_runtime_rpc::CritterchainApiServer::new(client.clone()).into_rpc())?; // Example custom pallet RPC

                // Babe RPC
                io.merge(BabeApiServer::new(
                    client.clone(),
                    shared_authority_set.clone(),
                    keystore.clone(),
                    babe_link.clone(),
                ).into_rpc())?;

                // Grandpa RPC
                io.merge(GrandpaApiServer::new(
                    grandpa_link.clone(),
                    shared_voter_state.clone(),
                    justification_stream,
                    subscription_executor,
                    telemetry_handle.as_ref().map(|h| h.handle().clone()), // Pass telemetry handle
                ).into_rpc())?;

                Ok(io)
            })
        };

        config.rpc_id_provider = Some(Box::new(SystemRpcIdProvider));

        let rpc_handlers = sc_service::spawn_tasks(SpawnTasksParams {
            config,
            backend: backend.clone(),
            client: client.clone(),
            keystore: keystore_container.sync_keystore(),
            network: network.clone(),
            rpc_builder: rpc_extensions_builder,
            transaction_pool: transaction_pool.clone(),
            task_manager: &mut task_manager,
            remote_blockchain: None, // No remote blockchain for full client
            system_rpc_tx,
            chain_props: None, // Or load chain properties
            deny_unsafe: DenyUnsafe::Yes, // Defaulting to Yes
            // Other parameters like consensus_client can be added here
        })?.rpc_handlers;

        // Consensus orchestration
        if config.role.is_authority() {
            let proposer_factory = sc_basic_authorship::ProposerFactory::new(
                task_manager.spawn_handle(),
                client.clone(),
                transaction_pool.clone(),
                config.prometheus_registry(),
                telemetry_handle.clone(),
            );

            let (babe_block_import, babe_link) = sc_consensus_babe::block_import_authority_discovery(
                BabeConfiguration::get_or_compute(&*client)?,
                grandpa_block_import.clone(),
                client.clone(),
                None,
                backend.clone(),
                task_manager.spawn_handle(),
                telemetry_handle.clone(),
            );

            let (babe_consensus_service, _) = sc_consensus_babe::start_babe(
                babe_link.clone(),
                client.clone(),
                select_chain.clone(),
                babe_block_import,
                proposer_factory,
                keystore_container.sync_keystore(),
                task_manager.spawn_handle(),
                None,
                telemetry_handle.clone(),
                None, // No epoch_change_notifier
                None, // No finality proof request builder
            )?;
            task_manager.spawn_essential_handle().spawn_blocking("babe-consensus", None, babe_consensus_service);

            let (grandpa_consensus_service, _) = sc_consensus_grandpa::start_grandpa(
                grandpa_link.clone(),
                select_chain.clone(),
                grandpa_block_import.clone(),
                task_manager.spawn_handle(),
                telemetry_handle.clone(),
                grandpa_link.shared_voter_state().clone(), // Pass the shared_voter_state
                keystore_container.sync_keystore(),
            )?;
            task_manager.spawn_essential_handle().spawn_blocking("grandpa-voter", None, grandpa_consensus_service);
        }

        network_starter.start_network();
        Ok((task_manager, client, network, transaction_pool, rpc_handlers, telemetry_handle))
    }

    /// Builds a new service for a light client.
    pub fn new_light(mut config: Configuration) -> Result<(
        TaskManager,
        Arc<FullClient>,
        Arc<sc_network::NetworkService<Block, <Block as BlockT>::Hash>>,
        Arc<LightTransactionPool>,
        RpcHandlers,
        Option<TelemetryWorkerHandle>
    ), ServiceError> {
        let telemetry = config
            .telemetry_endpoints
            .clone()
            .filter(|x| !x.is_empty())
            .map(|endpoints| -> Result<_, sc_telemetry::Error> {
                let worker = TelemetryWorker::new(16)?;
                let telemetry = worker.handle().new_telemetry_handle();
                for endpoint in endpoints {
                    worker.add_fallback_log_target(endpoint.clone())?;
                }
                Ok((worker, telemetry))
            })
            .transpose()?;

        let executor = Arc::new(Executor::new(
            config.wasm_method,
            config.default_heap_pages,
            config.max_runtime_instances,
            config.runtime_cache_size,
        ));

        let (client, backend, keystore_container, mut task_manager, on_demand) =
            sc_service::new_light_parts::<Block, RuntimeApi, _>(
                &config,
                telemetry.as_ref().map(|(_, handle)| handle.clone()),
                executor.clone(),
            )?;

        let client = Arc::new(client);
        let telemetry_worker_handle = telemetry.as_ref().map(|(w, _)| w.handle().clone());

        let transaction_pool = Arc::new(LightPool::new(
            config.transaction_pool.clone(),
            config.prometheus_registry(),
            task_manager.spawn_essential_handle(),
            client.clone(),
            on_demand.clone(),
        ));

        let net_config = LightNetworkConfiguration::new(&config.network);
        let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
            &client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
            &config.chain_spec,
        );
        let mut net_config = LightNetworkConfiguration::new(&config.network); // Re-declare as mutable
        net_config.add_notification_protocol(sc_consensus_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));

        let warp_sync_service = Arc::new(GrandpaFinalityProofProvider::new(
            backend.clone(),
            // Light clients don't have full shared authority set, so we use a stub or similar.
            Arc::new(sc_consensus_grandpa::SharedAuthoritySet::empty()), // Placeholder for now
            Vec::default(),
        ));

        let (network, system_rpc_tx, network_starter) =
            sc_service::build_network(BuildNetworkParams {
                config: &config,
                client: client.clone(),
                transaction_pool: transaction_pool.clone(),
                spawn_handle: task_manager.spawn_handle(),
                import_attach_handle: None,
                custom_peers_set: Default::default(),
                block_announce_validator_builder: None,
                net_config,
                warp_sync_service: Some(warp_sync_service),
        })?;
        let network = Arc::new(network);

        let rpc_extensions_builder = {
            let client = client.clone();
            let pool = transaction_pool.clone();
            let keystore = keystore_container.sync_keystore(); // Still needs a keystore for some RPCs

            Box::new(move |deny_unsafe, subscription_executor: Arc<SubscriptionTaskExecutor>| {
                let mut io = RpcModule::new(());
                io.merge(SystemApiServer::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
                io.merge(TransactionPoolApiServer::new(client.clone(), pool.clone()).into_rpc())?;
                io.merge(SystemLocalApiServer::new(client.clone(), keystore.clone()).into_rpc())?;
                io.merge(sc_rpc::system::AccountSyncApiServer::new(client.clone(), keystore.clone()).into_rpc())?; // Added for account sync

                // Light clients usually don't run consensus RPCs like Babe or Grandpa authoring.
                // They might have read-only versions if needed, but not for initial implementation.
                Ok(io)
            })
        };

        config.rpc_id_provider = Some(Box::new(SystemRpcIdProvider));

        let rpc_handlers = sc_service::spawn_tasks(SpawnTasksParams {
            config,
            backend,
            client: client.clone(),
            keystore: keystore_container.sync_keystore(),
            network: network.clone(),
            rpc_builder: rpc_extensions_builder,
            transaction_pool: transaction_pool.clone(),
            task_manager: &mut task_manager,
            remote_blockchain: Some(network.clone()), // Light clients need remote blockchain for sync
            system_rpc_tx,
            chain_props: None,
            deny_unsafe: DenyUnsafe::Yes, // Or from CLI
        })?.rpc_handlers;

        network_starter.start_network();
        Ok((task_manager, client, network, transaction_pool, rpc_handlers, telemetry_worker_handle))
    }
}
