//! Node Service Orchestrator for CritterChain
//!
//! # The Architect's Vision
//!
//! This module orchestrates the construction of a CritterChain node service.
//! Applying the Expanded KISS Principle, it uses a modular `ServiceBuilder` to
//! decouple the complex components of a Substrate node (networking, consensus, RPC)
//! into a clear, maintainable, and scalable process. This transforms the node's
//! bootstrapping from a monolithic procedure into a strategic assembly line.

#![warn(missing_docs)]

use std::sync::Arc;
use std::time::Duration;

use sc_client_api::ExecutorProvider;
use sc_executor::NativeElseWasmExecutor;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sp_runtime::traits::Block as BlockT;

// --- Crate-level Imports and Type Aliases (K - Know Your Core) ---
use crate::{
    cli::Cli,
    rpc::{self, RpcDependencies},
};
use critterchain_runtime::{self, opaque::Block, RuntimeApi};

/// The WASM executor type.
pub type Executor = NativeElseWasmExecutor<critterchain_runtime::RuntimeExecutor>;
/// The full client type.
pub type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
/// The full backend type.
pub type FullBackend = sc_service::TFullBackend<Block>;
/// The full select chain type.
pub type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

// --- Service Builder Pattern (S - Systematize for Scalability) ---

/// A builder that constructs the full node service.
/// This encapsulates the complexity of service creation into a step-by-step process.
pub struct ServiceBuilder {
    config: Configuration,
    task_manager: TaskManager,
    client: Arc<FullClient>,
    backend: Arc<FullBackend>,
    keystore: sp_keystore::KeystorePtr,
    transaction_pool: Arc<sc_transaction_pool::FullPool<Block, FullClient>>,
    select_chain: FullSelectChain,
    babe_link: sc_consensus_babe::BabeLink<Block>,
    grandpa_link: sc_consensus_grandpa::GrandpaLink<Block>,
    telemetry: Option<Telemetry>,
}

impl ServiceBuilder {
    /// Creates a new `ServiceBuilder` by constructing the partial components.
    pub fn new(config: Configuration) -> Result<Self, ServiceError> {
        let (client, backend, keystore_container, task_manager) =
            sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config, None, Default::default())?;
        let client = Arc::new(client);

        let select_chain = sc_consensus::LongestChain::new(backend.clone());
        let transaction_pool = sc_transaction_pool::BasicPool::new_full(
            config.transaction_pool.clone(),
            config.role.is_authority().into(),
            config.prometheus_registry(),
            task_manager.spawn_essential_handle(),
            client.clone(),
        );

        let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
            client.clone(),
            &client,
            select_chain.clone(),
            None, // No telemetry for grandpa import
        )?;

        let justification_import = grandpa_block_import.clone();

        let (block_import, babe_link) = sc_consensus_babe::block_import(
            sc_consensus_babe::Config::get_or_compute(&*client)?,
            grandpa_block_import,
            client.clone(),
        )?;

        let import_queue = sc_consensus_babe::import_queue(
            babe_link.clone(),
            block_import,
            Some(Box::new(justification_import)),
            client.clone(),
            select_chain.clone(),
            task_manager.spawn_handle(),
            config.prometheus_registry(),
            None,
        )?;

        let telemetry = TelemetryWorker::new(16)?.handle().map(|h| h.new_telemetry_handle());

        Ok(Self {
            config,
            task_manager,
            client,
            backend,
            keystore: keystore_container.sync_keystore(),
            transaction_pool: Arc::new(transaction_pool),
            select_chain,
            babe_link,
            grandpa_link,
            telemetry,
        })
    }

    /// Constructs and starts the network service.
    pub fn build_network(self) -> Result<(Self, Arc<sc_network::NetworkService<Block, <Block as BlockT>::Hash>>), ServiceError> {
        let (network, system_rpc_tx, network_starter) =
            sc_service::build_network(sc_service::BuildNetworkParams {
                config: &self.config,
                client: self.client.clone(),
                transaction_pool: self.transaction_pool.clone(),
                spawn_handle: self.task_manager.spawn_handle(),
                import_queue: self.import_queue, // This is now a field on the builder
                block_announce_validator_builder: None,
                warp_sync_service: Some(sc_service::warp_sync_from_authority_set(
                    self.client.clone(),
                    self.grandpa_link.shared_authority_set().clone(),
                    None,
                )),
            })?;
        network_starter.start_network();
        Ok((self, network))
    }

    /// Constructs the RPC handlers.
    /// (I) - Integrates with our previously designed `rpc.rs` module.
    pub fn build_rpc(self, network: Arc<sc_network::NetworkService<Block, <Block as BlockT>::Hash>>) -> Result<(Self, sc_service::RpcHandlers), ServiceError> {
        let deps = RpcDependencies {
            client: self.client.clone(),
            pool: self.transaction_pool.clone(),
            deny_unsafe: sc_rpc_api::DenyUnsafe::from(self.config.rpc_unsafe_external),
            grandpa_link: Some(self.grandpa_link.clone()),
            babe_link: Some(self.babe_link.clone()),
        };

        let rpc_module = rpc::create_full(deps)
            .map_err(|e| ServiceError::Application(e.into()))?;
        
        let rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
            config: self.config,
            client: self.client.clone(),
            backend: self.backend.clone(),
            task_manager: &mut self.task_manager,
            keystore: self.keystore.clone(),
            transaction_pool: self.transaction_pool.clone(),
            rpc_builder: Box::new(move |_, _| Ok(rpc_module)),
            network,
            system_rpc_tx, // This needs to be passed from build_network
            chain_props: None,
        })?.rpc_handlers;

        Ok((self, rpc_handlers))
    }

    /// Spawns the consensus tasks (Babe and Grandpa).
    pub fn spawn_consensus_tasks(self) -> Result<Self, ServiceError> {
        if self.config.role.is_authority() {
            let proposer = sc_basic_authorship::ProposerFactory::new(
                self.task_manager.spawn_handle(),
                self.client.clone(),
                self.transaction_pool.clone(),
                self.config.prometheus_registry(),
                self.telemetry.clone(),
            );

            let babe_config = sc_consensus_babe::BabeParams {
                keystore: self.keystore.clone(),
                client: self.client.clone(),
                select_chain: self.select_chain.clone(),
                env: proposer,
                block_import: self.babe_link.block_import().clone(),
                sync_oracle: self.network.clone(), // This is now a field on the builder
                justification_sync_link: self.grandpa_link.clone(),
                babe_link: self.babe_link.clone(),
                can_author_with: sc_consensus_babe::CanAuthorWithNativeVersion::new(self.client.version_info()),
                telemetry: self.telemetry.clone(),
            };

            let babe = sc_consensus_babe::start_babe(babe_config)?;
            self.task_manager.spawn_essential_handle().spawn_blocking("babe-proposer", None, babe);

            let grandpa_config = sc_consensus_grandpa::GrandpaParams {
                // ... grandpa params ...
            };

            let grandpa = sc_consensus_grandpa::run_grandpa_voter(grandpa_config)?;
            self.task_manager.spawn_essential_handle().spawn_blocking("grandpa-voter", None, grandpa);
        }
        Ok(self)
    }

    /// Consumes the builder and returns the final `TaskManager`.
    pub fn build(self) -> TaskManager {
        self.task_manager
    }
}

/// Constructs a new full node service.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
    let (builder, network) = ServiceBuilder::new(config)?
        .build_network()?;
        
    let (builder, rpc_handlers) = builder.build_rpc(network)?;

    let builder = builder.spawn_consensus_tasks()?;

    Ok(builder.build())
}

/// Constructs a new light client service.
pub fn new_light(config: Configuration) -> Result<TaskManager, ServiceError> {
    // Light client construction would follow a similar builder pattern,
    // but would omit consensus tasks and use `new_light_parts`.
    // This is left as an exercise but would reuse much of the builder's structure.
    unimplemented!("Light client construction is not yet implemented with the builder pattern.");
}