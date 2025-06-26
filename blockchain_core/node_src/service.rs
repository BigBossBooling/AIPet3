//! Node Service Orchestrator for CritterChain.
//!
//! This module defines the core service construction logic for a CritterChain node,
//! including full and light client services, transaction pool setup, and
//! integration of Babe and Grandpa consensus engines.
//!
//! It's meticulously crafted to align with The Architect's vision for
//! modularity, scalability, and robust operation of the CritterCraft digital ecosystem.

#![allow(clippy::redundant_closure_call)] // Allow explicit closures for genesis functions
#![allow(clippy::type_complexity)] // Allow complex types for Substrate definitions

// Standard library imports
use std::{sync::Arc, time::Duration};

// Substrate Crates
use sc_client_api::{ExecutorProvider, RemoteBackend}; // Client API traits
use sc_executor::NativeElseWasmExecutor; // WASM executor
use sc_service::{
    BuildNetworkParams,          // Parameters for building the network service
    ChainType,                   // Type of chain
    Configuration,               // Core service configuration
    PartialComponents,           // Struct for partial service components
    ServiceFactory,              // Trait for service factories
    SpawnTasksParams,            // Parameters for spawning service tasks
    TFullBackend,                // Type alias for full node backend
    TFullClient,                 // Type alias for full node client
    TaskExecutor,                // Trait for task executors
    TaskManager,                 // Task manager
    new_full_parts,              // Helper for building full node parts
    new_light_parts,             // Helper for building light node parts
    error::Error as ServiceError, // Service-specific errors
    RpcHandlers,                 // Type for RPC handlers
    ChainProperties,             // Chain properties
};
use sc_network::{NetworkService, config::{FullNetworkConfiguration, LightNetworkConfiguration}}; // Network service and configuration
use sc_transaction_pool::{BasicPool, FullPool, LightPool}; // Transaction pool implementations
use sc_transaction_pool_api::MaintainedTransactionPool; // Transaction pool API trait
use sp_runtime::traits::Block as BlockT; // Block trait
use sc_consensus::{LongestChain, BlockImport as BlockImportT}; // Consensus traits
use sc_consensus_babe::{
    BabeBlockImport,            // Babe block import
    BabeConfiguration,          // Babe configuration
    BabeConsensusDataProvider,  // Data provider for Babe
    BabeLink,                   // Link struct for Babe consensus
    start_babe,                 // Helper to start Babe consensus
};
use sc_consensus_grandpa::{
    FinalityProofProvider as GrandpaFinalityProofProvider, // Grandpa finality proof provider
    GrandpaBlockImport,         // Grandpa block import
    GrandpaLink,                // Link struct for Grandpa consensus
    GrandpaParams,              // Parameters for Grandpa
    GrandpaJustificationStream, // Stream of Grandpa justifications
    SharedVoterState,           // Shared state for Grandpa voters
    run_grandpa_voter,          // Helper to run Grandpa voter
    AuthorityId as GrandpaId,   // Grandpa authority ID
};
use sc_telemetry::{TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle}; // Telemetry components
use sc_rpc::SubscriptionTaskExecutor; // For RPC subscriptions
use sc_basic_authorship::ProposerFactory; // For block proposal

// CritterChain Specific Imports
// Assume your runtime is in a crate named `critterchain_runtime`
use critterchain_runtime::{
    self,                      // The runtime crate itself
    opaque::Block,             // The opaque Block type from runtime
    RuntimeApi,                // Runtime API trait
    RuntimeExecutor,           // The executor for the WASM runtime
    EXISTENTIAL_DEPOSIT,       // Existential deposit constant (if used)
    BABE_GENESIS_EPOCH_CONFIG, // Babe genesis epoch config (if constant)
    SlotDuration,              // From pallet_babe, if needed for BabeConfiguration
};

// RPC imports - need to be defined in node/src/rpc.rs
// For now, these are conceptually assumed to exist.
use crate::rpc::{create_full_rpc_handlers, create_light_rpc_handlers}; // Corrected RPC builder imports.


/// Type aliases for full node components.
/// These enhance clarity and maintainability, aligning with "Know Your Core, Keep it Clear".
pub type FullClient = TFullClient<Block, RuntimeApi, Executor>;
pub type FullBackend = TFullBackend<Block>;
pub type FullSelectChain = LongestChain<FullBackend, Block>;
pub type FullTransactionPool = BasicPool<FullClient, Block>;
pub type LightTransactionPool = LightPool<Block, FullClient, sc_network::NetworkService<Block, Block::Hash>>;

/// Define our Executor type for the runtime WASM.
pub type Executor = NativeElseWasmExecutor<critterchain_runtime::RuntimeExecutor>;


/// Builds a new service for a partial client (used by CLI subcommands).
/// This function constructs core components required to run a node, but without starting
/// network or consensus tasks.
///
/// Returns: PartialComponents struct containing `client`, `backend`, `task_manager`,
/// `import_queue`, `keystore_container`, `select_chain`, `transaction_pool`, and
/// `other` components (Grandpa block import/link, Telemetry handle).
pub fn new_partial(
    config: &mut Configuration,
) -> Result<
    PartialComponents<
        FullClient,
        FullBackend,
        FullSelectChain,
        sc_consensus_babe::BabeImportQueue<Block, FullClient>, // Use BabeImportQueue
        BabeLink<Block>,
        ( // Other components tuple
            GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>, // Grandpa block import
            GrandpaLink<Block>, // Grandpa link
            Option<TelemetryHandle>, // Telemetry handle
        )
    >,
    ServiceError,
> {
    // Executor for WASM runtime calls. This is the heart of our runtime execution.
    let executor = Arc::new(NativeElseWasmExecutor::<RuntimeExecutor>::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
        config.runtime_cache_size,
    ));

    // Telemetry setup: essential for observability in production.
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?; // Create telemetry worker
            let telemetry = worker.handle().new_telemetry_handle(); // Get handle
            for endpoint in endpoints {
                worker.add_fallback_log_target(endpoint.clone())?; // Add endpoints
            }
            Ok((worker, telemetry))
        })
        .transpose()?;

    // Basic full node parts: client, backend, keystore, task_manager.
    let (client, backend, keystore_container, task_manager) =
        new_full_parts::<Block, RuntimeApi, Executor>(
            config,
            telemetry.as_ref().map(|(_, handle)| handle.clone()), // Pass telemetry handle
            executor.clone(), // Pass executor Arc
        )?;

    let client = Arc::new(client); // Wrap client in Arc for shared ownership
    let telemetry_handle = telemetry.as_ref().map(|(_, handle)| handle.clone()); // Get telemetry handle

    // Select chain strategy: Longest chain rule for fork choice.
    let select_chain = LongestChain::new(backend.clone());

    // Transaction pool setup: Manages pending transactions for inclusion in blocks.
    let transaction_pool = BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(), // Authority nodes get a full pool
        config.prometheus_registry(), // Prometheus registry for metrics
        task_manager.spawn_essential_handle(), // Task spawn handle
        client.clone(), // Client reference
    );
    let transaction_pool = Arc::new(transaction_pool); // Wrap pool in Arc

    // Grandpa block import and link: For finality.
    let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
        client.clone(),
        &(client.clone() as Arc<_>), // Client reference for API
        select_chain.clone(), // Select chain for Grandpa's view
        telemetry.as_ref().map(|(_, T)| T.handle()), // Telemetry for Grandpa
    )?;

    // Babe block import and link: For block production.
    let (block_import, babe_link) = sc_consensus_babe::block_import_authority_discovery(
        BabeConfiguration::get_or_compute(&*client)?, // Babe configuration from client
        grandpa_block_import.clone(), // Grandpa block import for Babe
        client.clone(), // Client reference
        None, // No authority discovery service for this example
        backend.clone(), // Backend reference
        task_manager.spawn_handle(), // Task spawn handle
        telemetry.as_ref().map(|(_, T)| T.handle()), // Telemetry for Babe
    )?;

    // Import queue setup: Orchestrates block import from network/sync.
    let import_queue = sc_consensus_babe::import_queue(
        babe_link.clone(), // Babe link for import queue
        block_import, // Babe block import
        Some(Box::new(grandpa_block_import.clone())), // Grandpa block import for finality
        client.clone(), // Client reference
        select_chain.clone(), // Select chain for import queue
        task_manager.spawn_handle(), // Task spawn handle
        config.prometheus_registry(), // Prometheus registry
        telemetry.as_ref().map(|(_, T)| T.handle()), // Telemetry handle
        None, // No authority discovery receiver
    )?;

    // Return partial components
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
/// This function sets up a full node, which is essential for running validators and providing complete
/// blockchain data to light clients.
/// Returns: A tuple containing `TaskManager`, `FullClient` (for runtime interaction),
/// `NetworkService`, `FullTransactionPool`, `RpcHandlers`, and `TelemetryWorkerHandle`.
pub fn new_full(
    mut config: Configuration,
) -> Result<(
    TaskManager,
    Arc<FullClient>,
    Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
    Arc<FullTransactionPool>,
    RpcHandlers,
    Option<TelemetryWorkerHandle>,
), ServiceError> {
    // Get partial components
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

    // Build network service
    let (network, system_rpc_tx, network_starter) = {
        let net_config = FullNetworkConfiguration::new(&config.network);
        sc_service::build_network(BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_attach_handle: Some(import_queue), // Attach import queue to network
            custom_peers_set: Default::default(),
            block_announce_validator_builder: None,
            net_config,
            // Warp sync service for faster initial chain synchronization.
            warp_sync_service: Some(Arc::new(GrandpaFinalityProofProvider::new(
                backend.clone(),
                grandpa_link.shared_authority_set().clone(), // Shared authority set from Grandpa link
                Vec::default(), // No custom finality proof stream
            ))),
        })?
    };
    let network = Arc::new(network);
    let telemetry_worker_handle = telemetry_handle.as_ref().map(|w| w.handle().clone());


    // Build RPC extensions
    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();
        let keystore = keystore_container.sync_keystore(); // Keystore for RPCs
        let justification_stream = grandpa_link.justification_stream();
        let shared_authority_set = grandpa_link.shared_authority_set().clone();
        let shared_voter_state = grandpa_link.shared_voter_state();
        let deny_unsafe = DenyUnsafe::default(); // Use DenyUnsafe::default()

        Box::new(move |subscription_executor: Arc<SubscriptionTaskExecutor>| {
            let mut io = jsonrpsee::RpcModule::new(());
            // Merge standard system RPCs
            io.merge(sc_rpc::system::SystemApiServer::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
            io.merge(sc_rpc::tx_pool::TransactionPoolApiServer::new(client.clone(), pool.clone()).into_rpc())?;
            io.merge(sc_rpc::system_local::SystemLocalApiServer::new(client.clone(), keystore.clone()).into_rpc())?;
            io.merge(sc_rpc::system::AccountSyncApiServer::new(client.clone(), keystore.clone()).into_rpc())?;
            
            // Add custom RPC for critterchain_runtime (if defined in runtime/src/rpc.rs)
            // Example: io.merge(critterchain_runtime_rpc::CritterchainApiServer::new(client.clone()).into_rpc())?; 

            // Merge consensus RPCs (Babe and Grandpa)
            io.merge(sc_consensus_babe_rpc::BabeApiServer::new(
                client.clone(),
                shared_authority_set.clone(),
                keystore.clone(),
                babe_link.clone(),
            ).into_rpc())?;
            io.merge(sc_consensus_grandpa_rpc::GrandpaApiServer::new(
                grandpa_link.clone(),
                shared_voter_state.clone(),
                justification_stream,
                subscription_executor,
                telemetry_handle.as_ref().map(|h| h.handle().clone()), // Pass telemetry handle
            ).into_rpc())?;

            Ok(io)
        })
    };

    // Set RPC ID provider
    config.rpc_id_provider = Some(Box::new(SystemRpcIdProvider));

    // Spawn tasks and get RPC handlers
    let rpc_handlers = sc_service::spawn_tasks(SpawnTasksParams {
        config,
        backend: backend.clone(),
        client: client.clone(),
        keystore: keystore_container.sync_keystore(), // Pass keystore
        network: network.clone(),
        rpc_builder: rpc_extensions_builder, // Our custom RPC builder
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        remote_blockchain: None, // No remote blockchain for full client
        system_rpc_tx, // System RPC sender
        chain_props: Some(critterchain_runtime::chain_properties()), // Chain properties from runtime
        deny_unsafe: DenyUnsafe::Yes, // Defaulting to Yes
        // Other parameters like consensus_client can be added here
    })?.rpc_handlers;

    // Consensus orchestration
    if config.role.is_authority() {
        let proposer_factory = ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            config.prometheus_registry(),
            telemetry_handle.clone(),
        );

        let babe_config = BabeConfiguration::get_or_compute(&*client)?;
        let (babe_consensus_service, _) = start_babe(
            babe_link.clone(),
            client.clone(),
            select_chain.clone(),
            grandpa_block_import.clone().into(), // Use GrandpaBlockImport here for block import for Babe
            proposer_factory,
            keystore_container.sync_keystore(),
            task_manager.spawn_handle(),
            None, // No authority discovery for this example
            telemetry_handle.clone(),
            None, // No epoch_change_notifier
            None, // No finality proof request builder
        )?;
        task_manager.spawn_essential_handle().spawn_blocking("babe-consensus", None, babe_consensus_service);

        let (grandpa_consensus_service, _) = sc_consensus_grandpa::start_grandpa(
            grandpa_link.clone(),
            select_chain.clone(),
            grandpa_block_import.clone().into(), // Use GrandpaBlockImport here for block import for Grandpa
            task_manager.spawn_handle(),
            telemetry_handle.clone(),
            grandpa_link.shared_voter_state().clone(),
            keystore_container.sync_keystore(),
        )?;
        task_manager.spawn_essential_handle().spawn_blocking(
            "grandpa-voter",
            None,
            grandpa_consensus_service,
        );
    }

    // Start the network service
    network_starter.start_network();
    Ok((task_manager, client, network, transaction_pool, rpc_handlers, telemetry_handle))
}

/// Builds a new service for a light client.
/// This function sets up a light node, optimized for minimal resource usage,
/// crucial for mobile integration of the CritterCraft app.
/// Returns: A tuple containing `TaskManager`, `FullClient` (for runtime interaction),
/// `NetworkService`, `LightTransactionPool`, `RpcHandlers`, and `TelemetryWorkerHandle`.
pub fn new_light(
    mut config: Configuration,
) -> Result<(
    TaskManager,
    Arc<FullClient>,
    Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
    Arc<LightTransactionPool>,
    RpcHandlers,
    Option<TelemetryWorkerHandle>,
), ServiceError> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?; // Create telemetry worker
            let telemetry = worker.handle().new_telemetry_handle(); // Get handle
            for endpoint in endpoints {
                worker.add_fallback_log_target(endpoint.clone())?; // Add endpoints
            }
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = Arc::new(NativeElseWasmExecutor::<RuntimeExecutor>::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
        config.runtime_cache_size,
    ));

    let (client, backend, keystore_container, mut task_manager, on_demand) =
        sc_service::new_light_parts::<Block, RuntimeApi, Executor>(
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

    let mut net_config = LightNetworkConfiguration::new(&config.network);
    let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
        &client.block_hash(BlockId::Number(0))
            .ok()
            .flatten()
            .expect("Genesis block exists; qed"),
        &config.chain_spec,
    );
    net_config.add_notification_protocol(sc_consensus_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));

    let warp_sync_service = Arc::new(GrandpaFinalityProofProvider::new(
        backend.clone(),
        Arc::new(sc_consensus_grandpa::SharedAuthoritySet::empty()),
        Vec::default(),
    ));


    let (network, system_rpc_tx, network_starter) =
        build_network(BuildNetworkParams {
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
        let keystore = keystore_container.sync_keystore();
        let deny_unsafe = DenyUnsafe::default(); // Use DenyUnsafe::default()


        Box::new(move |subscription_executor: Arc<SubscriptionTaskExecutor>| {
            let mut io = jsonrpsee::RpcModule::new(());
            io.merge(sc_rpc::system::SystemApiServer::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
            io.merge(sc_rpc::tx_pool::TransactionPoolApiServer::new(client.clone(), pool.clone()).into_rpc())?;
            io.merge(sc_rpc::system_local::SystemLocalApiServer::new(client.clone(), keystore.clone()).into_rpc())?;
            io.merge(sc_rpc::system::AccountSyncApiServer::new(client.clone(), keystore.clone()).into_rpc())?;

            // Light clients typically do NOT run consensus RPCs like Babe or Grandpa authoring.
            // They might have read-only versions if needed, but not for initial implementation to keep them lean.
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
        remote_blockchain: Some(network.clone()),
        system_rpc_tx,
        chain_props: None,
        deny_unsafe: DenyUnsafe::Yes,
    })?.rpc_handlers;

    network_starter.start_network();
    Ok((task_manager, client, network, transaction_pool, rpc_handlers, telemetry_worker_handle))
}
