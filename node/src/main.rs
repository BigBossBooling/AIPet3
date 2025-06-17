// --- BEGIN CONCEPTUAL NODE main.rs ---
// File: node/src/main.rs

// This is a conceptual outline based on a standard Substrate node template,
// adapted for CritterChain with Babe/Grandpa consensus.

// std
use std::{path::PathBuf, sync::Arc};

// External Crates
use clap::Parser;
use log::info; // For logging

// Substrate Crates (selection, more might be needed)
use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli, CliConfiguration};
use sc_service::{
    BasePath, ChainType, Configuration, GenericChainSpec, PartialComponents, TaskManager,
    new_full_parts, new_light_parts, // For constructing service parts
    TFullBackend, TFullClient, // Type aliases for full node backend/client
    TFullCallApi, // For full node call API
    DenyUnsafe, // For RPC deny_unsafe option
};
use sc_rpc::SubscriptionTaskExecutor; // For RPC subscriptions
use sc_consensus_babe::{self, SlotProportion}; // For Babe
use sc_consensus_grandpa::{self, FinalityProofProvider, SharedVoterState, GrandpaJustificationStream}; // For Grandpa
use sp_core::crypto::Ss58AddressFormat;
use sp_runtime::traits::Block as BlockT;
use sp_keystore::KeystorePtr;
use sc_telemetry::TelemetryWorker; // For telemetry

// CritterChain Specific Imports
// Assume your runtime is in a crate named `critterchain_runtime`
use critterchain_runtime::{opaque::Block, AccountId, Balance, Index as Nonce, RuntimeApi, RuntimeExecutor};

// --- Node CLI Definition ---
#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[clap(flatten)]
    pub run: sc_cli::RunCmd, // Standard run command
}

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

// --- Main Entry Point ---
fn main() -> sc_cli::Result<()> {
    // Initialize logging
    env_logger::init();
    // Set default SS58 address format
    sp_core::crypto::set_default_ss58_format(Ss58AddressFormat::SubstrateAccount);

    // Parse CLI arguments
    let cli = Cli::parse();

    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let PartialComponents { client, task_manager, import_queue, .. } =
                    service::new_partial(&mut config)?; // Use service::new_partial
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let PartialComponents { client, task_manager, .. } = service::new_partial(&mut config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let PartialComponents { client, task_manager, .. } = service::new_partial(&mut config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let PartialComponents { client, task_manager, import_queue, .. } =
                    service::new_partial(&mut config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let PartialComponents { client, task_manager, backend, .. } = service::new_partial(&mut config)?;
                let finality_proof_provider = GrandpaFinalityProofProvider::new(backend.clone()); // Corrected
                Ok((cmd.run(client, backend, Some(Box::new(finality_proof_provider))), task_manager))
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|mut config| {
                // new_full_parts now requires mutable config
                let (client, _, _, _) = sc_service::new_full_parts::<
                    Block,
                    RuntimeApi,
                    service::Executor,
                >(&mut config, None, Arc::new(service::Executor::new( // Provide executor Arc
                    config.wasm_method,
                    config.default_heap_pages,
                    config.max_runtime_instances,
                    config.runtime_cache_size,
                )))?;
                cmd.run::<Block, critterchain_runtime::ExistentialDeposit>(config, client)
            })
        }
        Some(Subcommand::TryRuntime(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                // We don't need most of the components returned by new_partial, just a
                // task manager and some way to run try-runtime against the client.
                let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
                let task_manager =
                    TaskManager::new(config.tokio_handle.clone(), registry)
                        .map_err(|e| sc_cli::Error::Service(e.into()))?;
                Ok((cmd.run::<Block, RuntimeApi>(config), task_manager))
            })
        }
        Some(Subcommand::Db(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                match config.role {
                    Role::Light => service::new_light(config).map(|(task_manager, _, _, _, rpc_handlers, telemetry_worker_handle)| (task_manager, rpc_handlers, telemetry_worker_handle)),
                    _ => service::new_full(config).map(|(task_manager, _, _, _, rpc_handlers, telemetry_worker_handle)| (task_manager, rpc_handlers, telemetry_worker_handle)),
                }
                .map_err(sc_cli::Error::Service)
            })
        }
    }
}

// --- Service Construction (Conceptual - often in service.rs) ---
pub mod service {
    use super::*;
    use critterchain_runtime::RuntimeApi;
    use sc_service::{config::Configuration, error::Error as ServiceError, RpcHandlers, TaskManager, BuildNetworkParams, SpawnTasksParams};
    use sc_executor::NativeElseWasmExecutor;
    use sc_consensus_babe::{BabeBlockImport, BabeLink, BabeConfiguration, BabeConsensusDataProvider};
    use sc_consensus_grandpa::{GrandpaBlockImport, GrandpaLink, GrandpaJustificationStream, SharedVoterState, GrandpaApi, FinalityProofProvider as GrandpaFinalityProofProvider};
    use sc_client_api::{ExecutorProvider, RemoteBackend};
    use sp_blockchain::HeaderBackend;
    use sp_consensus_babe::AuthorityId as BabeId;
    use sp_consensus_grandpa::AuthorityId as GrandpaId;
    use std::sync::Arc;
    use sc_network::NetworkService; // For NetworkService type
    use sc_transaction_pool_api::MaintainedTransactionPool; // For MaintainedTransactionPool type
    use sc_transaction_pool::{BasicPool, FullPool, LightPool}; // Pool implementations
	use sc_rpc::{system::SystemApiServer, payment::TransactionPaymentApiServer}; // Standard RPCs
    use sc_consensus_babe_rpc::BabeApiServer; // Babe RPC
    use sc_consensus_grandpa_rpc::GrandpaApiServer; // Grandpa RPC
    use sc_rpc_multiaddr::MultiaddrNamedWithPeerId; // For multiaddr
    use sc_consensus_manual_seal::EngineCommand; // For manual seal if used in dev
    use sc_telemetry::{TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle}; // For telemetry


    // Define our Executor type
    pub type Executor = NativeElseWasmExecutor<RuntimeExecutor>;

    // Full client type alias
    pub type FullClient = TFullClient<Block, RuntimeApi, Executor>;
    // Full backend type alias
    pub type FullBackend = TFullBackend<Block>;
    // Full Call API type alias
    type FullCallApi = TFullCallApi<Block, RuntimeApi, Executor>;
    // Full transaction pool
    pub type FullTransactionPool = BasicPool<FullClient, Block>;


    pub fn new_partial(config: &mut Configuration) -> Result<PartialComponents<
        FullClient,
        FullBackend,
        (), // No select_chain needed for this partial
        sc_consensus_babe::BabeImportQueue<Block, FullClient>,
        BabeLink<Block>,
        ( // Other components tuple
            GrandpaBlockImport<FullBackend, Block, FullClient, ()>, // No select chain for grandpa import
            GrandpaLink<Block>,
            Option<TelemetryHandle>,
        )
    >, ServiceError> {
        let executor = Arc::new(Executor::new(
            config.wasm_method,
            config.default_heap_pages,
            config.max_runtime_instances,
            config.runtime_cache_size,
        ));

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
            &(client.clone() as Arc<_>),
            select_chain.clone(),
            telemetry.as_ref().map(|(_, T)| T.handle()),
        )?;

        let (block_import, babe_link) = sc_consensus_babe::block_import_authority_discovery(
            BabeConfiguration::get_or_compute(&*client)?,
            grandpa_block_import.clone(),
            client.clone(),
            None, // No authority discovery for this example
            backend.clone(), // Pass backend Arc
            task_manager.spawn_handle(),
            telemetry.as_ref().map(|(_, T)| T.handle()),
        )?;

        let import_queue = sc_consensus_babe::import_queue(
            babe_link.clone(),
            block_import, // Use the BabeBlockImport
            Some(Box::new(grandpa_block_import.clone())),
            client.clone(),
            select_chain,
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
            select_chain: (), // Not needed by CLI subcommands using new_partial
            transaction_pool,
            other: (grandpa_block_import, grandpa_link, telemetry_handle),
        })
    }

    /// Builds a new service for a full client.
    pub fn new_full(mut config: Configuration) -> Result<(
        TaskManager,
        Arc<FullClient>,
        Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
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
            other: (block_import, grandpa_link, mut telemetry_handle),
            ..
        } = new_partial(&mut config)?;


        let mut net_config = sc_network::config::FullNetworkConfiguration::new(&config.network);

        let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
            &client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
            &config.chain_spec,
        );
        net_config.add_notification_protocol(sc_consensus_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));

        let warp_sync = Arc::new(sc_consensus_grandpa::warp_proof::NetworkProvider::new(
            backend.clone(),
            grandpa_link.shared_authority_set().clone(),
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
                block_announce_validator_builder: None, // For Babe, this is handled internally
                net_config, // Pass the modified net_config
                warp_sync_service: Some(warp_sync), // Enable warp sync
        })?;

        let network = Arc::new(network);
        let telemetry_worker_handle = telemetry_handle.as_ref().map(|w| w.handle().clone());


        let rpc_extensions_builder = {
            let client = client.clone();
            let pool = transaction_pool.clone();
            let keystore = keystore_container.sync_keystore();
            let chain_spec = config.chain_spec.cloned_box();
            let justification_stream = grandpa_link.justification_stream();
            let shared_authority_set = grandpa_link.shared_authority_set().clone();
            let shared_voter_state = grandpa_link.shared_voter_state();


            Box::new(move |deny_unsafe, subscription_executor: Arc<SubscriptionTaskExecutor>| {
                let mut io = jsonrpsee::RpcModule::new(());
                io.merge(SystemApiServer::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
                io.merge(TransactionPaymentApiServer::new(client.clone()).into_rpc())?;

                // Babe RPC
                io.merge(BabeApiServer::new(
                    client.clone(),
                    shared_authority_set.clone(), // Use shared_authority_set
                    keystore.clone(), // Pass KeystorePtr
                    babe_link.clone(), // Use babe_link from new_partial
                ).into_rpc())?;

                // Grandpa RPC
                io.merge(GrandpaApiServer::new(
                    grandpa_link.clone(), // Use grandpa_link from new_partial
                    shared_voter_state.clone(),
                    justification_stream.clone(),
                    subscription_executor,
                    None, // No telemetry for RPC
                ).into_rpc())?;

                // Custom pallet RPCs can be added here

                Ok(io)
            })
        };

        config.rpc_id_provider = Some(Box::new(sc_rpc::system::SystemRpcIdProvider));


        let rpc_handlers = sc_service::spawn_tasks(SpawnTasksParams {
            config,
            backend: backend.clone(),
            client: client.clone(),
            keystore: keystore_container.sync_keystore(),
            network: network.clone(),
            rpc_builder: rpc_extensions_builder, // Corrected field name
            transaction_pool: transaction_pool.clone(),
            task_manager: &mut task_manager,
            remote_blockchain: None,
            system_rpc_tx,
            chain_props: None,
            deny_unsafe, // Pass deny_unsafe from config
        })?.rpc_handlers;


        if config.role.is_authority() {
            let proposer = sc_basic_authorship::ProposerFactory::new(
                task_manager.spawn_handle(),
                client.clone(),
                transaction_pool.clone(),
                config.prometheus_registry(),
                telemetry_handle.clone(),
            );

            let babe_config = BabeConfiguration::get_or_compute(&*client)?;
            let (babe_consensus_service, babe_link_from_service) = sc_consensus_babe::start_babe( // Renamed babe_link
                babe_config,
                babe_link.clone(), // Use babe_link from new_partial
                client.clone(),
                select_chain.clone(), // Use select_chain from new_partial
                block_import.clone(), // Use block_import (BabeBlockImport) from new_partial
                proposer,
                keystore_container.sync_keystore(),
                task_manager.spawn_handle(),
                None, // No authority discovery for this example
                telemetry_handle.clone(),
                None, // No epoch_change_notifier
                None, // No finality proof request builder
            )?;
            task_manager.spawn_essential_handle().spawn_blocking("babe-consensus", None, babe_consensus_service);

            let grandpa_config = sc_consensus_grandpa::GrandpaParams {
                grandpa_key: None,
                keystore: Some(keystore_container.sync_keystore()),
                local_role: config.role.clone(),
                link: grandpa_link.clone(),
                telemetry: telemetry_handle.clone(),
            };
            task_manager.spawn_essential_handle().spawn_blocking(
                "grandpa-voter",
                None,
                sc_consensus_grandpa::run_grandpa_voter(grandpa_config)?
            );
        }

        network_starter.start_network();
        Ok((task_manager, client, network, transaction_pool, rpc_handlers, telemetry_handle))
    }

    /// Builds a new service for a light client.
    pub fn new_light(mut config: Configuration) -> Result<(
        TaskManager,
        Arc<FullClient>, // Light client still uses FullClient type for runtime interaction
        Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
        Arc<LightPool<Block, FullClient, NetworkService<Block, <Block as BlockT>::Hash>>>,
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

        let mut net_config = sc_network::config::LightNetworkConfiguration::new(&config.network);
        let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
            &client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
            &config.chain_spec,
        );
        net_config.add_notification_protocol(sc_consensus_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));

        let warp_sync_service = Arc::new(sc_consensus_grandpa::warp_proof::NetworkProvider::new(
            backend.clone(),
            grandpa_link_stub(), // Light clients don't have full shared authority set
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
            // Light clients typically don't have a keystore in the same way, or it's more limited.
            // let keystore = keystore_container.sync_keystore();

            Box::new(move |deny_unsafe, subscription_executor: Arc<SubscriptionTaskExecutor>| {
                let mut io = jsonrpsee::RpcModule::new(());
                io.merge(SystemApiServer::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
                io.merge(TransactionPaymentApiServer::new(client.clone()).into_rpc())?;
                // Light clients usually don't run consensus RPCs like Babe or Grandpa authoring.
                // They might have read-only versions if needed.
                Ok(io)
            })
        };

        config.rpc_id_provider = Some(Box::new(sc_rpc::system::SystemRpcIdProvider));

        let rpc_handlers = sc_service::spawn_tasks(SpawnTasksParams {
            config,
            backend,
            client: client.clone(),
            keystore: keystore_container.sync_keystore(), // Pass for consistency, though light client usage is minimal
            network: network.clone(),
            rpc_builder: rpc_extensions_builder,
            transaction_pool: transaction_pool.clone(),
            task_manager: &mut task_manager,
            remote_blockchain: Some(network.clone()),
            system_rpc_tx,
            chain_props: None,
            deny_unsafe: DenyUnsafe::Yes, // Or from CLI
        })?.rpc_handlers;

        network_starter.start_network();
        Ok((task_manager, client, network, transaction_pool, rpc_handlers, telemetry_worker_handle))
    }

    // Helper for light client grandpa link stub
    fn grandpa_link_stub() -> Arc<sc_consensus_grandpa::Link<Block>> {
        struct Stub;
        impl sc_consensus_grandpa::Link<Block> for Stub {
            type BlockImport = Arc<dyn sp_consensus::BlockImport<Block, Error = sp_consensus::Error> + Send + Sync>;
            type Backend = Arc<dyn sp_consensus::block_validation::Chain<Block, Error = sp_blockchain::Error> + Send + Sync>;
            fn shared_authority_set(&self) -> Arc<sc_consensus_grandpa::SharedAuthoritySet<sp_consensus_grandpa::AuthorityId, sp_runtime::traits::BlockNumberFor<Block>>> { Arc::new(sc_consensus_grandpa::SharedAuthoritySet::empty()) }
            fn shared_voter_state(&self) -> Arc<sc_consensus_grandpa::SharedVoterState> { Arc::new(sc_consensus_grandpa::SharedVoterState::empty()) }
            fn justification_stream(&self) -> GrandpaJustificationStream<Block> { Box::pin(futures::stream::empty()) }
            fn finality_proof_provider(&self, _hash: <Block as BlockT>::Hash, _number: sp_runtime::traits::BlockNumberFor<Block>, _set_id: u64) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error + Send + Sync + 'static>> { Ok(None) }
        }
        Arc::new(Stub)
    }
}

// --- END CONCEPTUAL NODE main.rs ---
