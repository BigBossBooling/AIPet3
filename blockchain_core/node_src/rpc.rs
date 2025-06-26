//! CritterChain Node RPC Gateway
//!
//! This module defines and builds the JSON-RPC interface for the CritterChain node.
//! It merges standard Substrate RPCs and custom CritterChain-specific RPCs, exposing
//! them to wallets, explorers, dApps, and CritterCraft Mobile.
//!
//! It's meticulously crafted to align with The Architect's vision for
//! modularity, scalability, and robust operation of the CritterCraft digital ecosystem.

#![warn(missing_docs)] // Ensure all public API are documented

use std::sync::Arc;
use jsonrpsee::RpcModule; // JSON-RPC server module
use sc_rpc_api::DenyUnsafe; // RPC security option
use sc_client_api::{ExecutorProvider, RemoteBackend, BlockchainEvents}; // Client API traits, BlockchainEvents for subscriptions
use sc_transaction_pool_api::{TransactionPool, MaintainedTransactionPool}; // Transaction pool API traits
use sc_network::NetworkService; // Network service
use sp_api::ProvideRuntimeApi; // Trait for providing runtime API
use sp_blockchain::{HeaderBackend, Blockchain}; // Blockchain backend traits
use sp_runtime::traits::Block as BlockT; // Block trait
use sp_keystore::KeystorePtr; // Keystore pointer for local signing RPCs
use sc_rpc::SubscriptionTaskExecutor; // For RPC subscriptions

// Import CritterChain runtime and custom RPCs
use critterchain_runtime::{
    opaque::Block, // Opaque block type
    AccountId,     // Account ID type
    Balance,       // Balance type
    Nonce,         // Nonce type
    RuntimeApi,    // Runtime API trait
};

// Substrate standard RPCs. These are common and well-defined.
use sc_rpc::{
    system::SystemApiServer,            // System information RPC
    tx_pool::TransactionPoolApiServer,  // Transaction pool information RPC
    system_local::SystemLocalApiServer, // Local system information (e.g., keystore related)
    account_sync::AccountSyncApiServer, // Account synchronization RPC
};

// Consensus specific RPCs
use sc_consensus_babe_rpc::BabeApiServer;
use sc_consensus_grandpa_rpc::GrandpaApiServer;

// --- CritterChain Custom Runtime RPCs ---
// This assumes you have a `critterchain_runtime_rpc` crate in your runtime,
// and it defines a service for your custom pallets.
// For example, it would expose RPCs for querying Pet NFTs, Marketplace listings, etc.
use critterchain_runtime_rpc::{CritterchainApiServer, CritterchainRpcExt};


/// Full client type alias.
/// This alias is for the specific client implementation used by the full node.
pub type FullClient = sc_service::TFullClient<Block, RuntimeApi, critterchain_runtime::Executor>;

/// Full transaction pool type alias.
/// This alias is for the specific transaction pool implementation used by the full node.
pub type FullPool = sc_transaction_pool::BasicPool<FullClient, Block>;

/// Light client type alias.
/// Note: Light clients typically expose the same runtime API via their `RemoteBackend`.
pub type LightClient = sc_service::TFullClient<Block, RuntimeApi, critterchain_runtime::Executor>;


/// Builds the full node's JSON-RPC module, merging all standard and custom APIs.
/// This function is typically called from `node/src/service.rs` to construct the RPC handlers.
pub fn create_full_rpc_handlers<C, P, SC, B, BE>(
    client: Arc<C>,
    pool: Arc<P>,
    select_chain: Arc<SC>,
    backend: Arc<BE>,
    keystore: KeystorePtr, // Pass keystore explicitly for RPCs needing it
    grandpa_link: sc_consensus_grandpa::GrandpaLink<Block>, // Grandpa link for consensus RPCs
    babe_link: sc_consensus_babe::BabeLink<Block>, // Babe link for consensus RPCs
    deny_unsafe: DenyUnsafe, // Security setting for RPCs
    subscription_executor: SubscriptionTaskExecutor, // Executor for subscriptions
    telemetry_handle: Option<TelemetryHandle>, // Telemetry for RPCs
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    // Type constraints for the client
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + ExecutorProvider<Block>
        + BlockchainEvents<Block> // Needed for subscriptions
        + 'static,
    C::Api: critterchain_runtime_rpc::RuntimeApiCollection<Block> // Custom RPC API
        + sp_api::Core<Block>
        + sp_api::Metadata<Block>
        + sp_api::BlockBuilder<Block>
        + sp_api::TxValidation<Block>
        + sp_api::ApiExt<Block, StateBackend = sc_client_api::StateBackendFor<BE, Block>>, // StateBackend needed for some RPCs
    // Type constraints for the transaction pool
    P: TransactionPool<Block = Block> + 'static,
    P: MaintainedTransactionPool<Block = Block> + 'static, // Needs MaintainedTransactionPool
    // Type constraints for select_chain (e.g., LongestChain)
    SC: sc_consensus::SelectChain<Block> + 'static,
    // Type constraints for backend
    BE: Blockchain<Block> + RemoteBackend<Block> + Send + Sync + 'static,
{
    let mut module = RpcModule::new(());

    // Standard Substrate RPCs (System, TransactionPool, etc.)
    module.merge(SystemApiServer::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
    module.merge(TransactionPoolApiServer::new(client.clone(), pool.clone()).into_rpc())?;
    module.merge(sc_rpc::account_sync::AccountSyncApiServer::new(client.clone(), keystore.clone()).into_rpc())?;
    module.merge(sc_rpc::system_local::SystemLocalApiServer::new(client.clone(), keystore.clone()).into_rpc())?;

    // CritterChain Custom RPCs (Game-specific logic)
    // This is where custom RPC methods for Pet NFTs, Marketplace, Battles, etc., are exposed.
    module.merge(CritterchainApiServer::new(client.clone()).into_rpc())?;

    // Consensus RPCs (Babe and Grandpa)
    // These require specific consensus links and shared states.
    module.merge(BabeApiServer::new(
        client.clone(),
        babe_link.shared_authority_set().clone(), // Get shared authority set from Babe link
        keystore.clone(),
        babe_link.clone(),
    ).into_rpc())?;
    module.merge(GrandpaApiServer::new(
        grandpa_link.clone(),
        grandpa_link.shared_voter_state().clone(), // Get shared voter state from Grandpa link
        grandpa_link.justification_stream(), // Get justification stream
        subscription_executor, // Pass subscription executor for async RPCs
        telemetry_handle.clone(), // Pass telemetry handle
    ).into_rpc())?;

    Ok(module) // Return the constructed RpcModule
}

/// Builds the light node's JSON-RPC module.
/// Light clients offer a subset of RPCs, primarily focused on querying chain state and
/// submitting transactions, without consensus-specific authoring RPCs.
pub fn create_light_rpc_handlers<C, P, SC, B, BE>(
    client: Arc<C>,
    pool: Arc<P>,
    select_chain: Arc<SC>,
    backend: Arc<BE>,
    keystore: KeystorePtr, // Light client might still need keystore for local signing.
    deny_unsafe: DenyUnsafe, // Security setting for RPCs
    subscription_executor: SubscriptionTaskExecutor, // Executor for subscriptions
    telemetry_handle: Option<TelemetryHandle>, // Telemetry for RPCs
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    // Type constraints for the client (same as full client for runtime API)
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + ExecutorProvider<Block>
        + BlockchainEvents<Block> // Needed for subscriptions
        + 'static,
    C::Api: critterchain_runtime_rpc::RuntimeApiCollection<Block> // Custom RPC API (if supported for light)
        + sp_api::Core<Block>
        + sp_api::Metadata<Block>
        + sp_api::BlockBuilder<Block>
        + sp_api::TxValidation<Block>
        + sp_api::ApiExt<Block, StateBackend = sc_client_api::StateBackendFor<BE, Block>>,
    // Type constraints for the transaction pool (LightPool)
    P: TransactionPool<Block = Block> + 'static,
    P: MaintainedTransactionPool<Block = Block> + 'static,
    // Type constraints for select_chain
    SC: sc_consensus::SelectChain<Block> + 'static,
    // Type constraints for backend
    BE: Blockchain<Block> + RemoteBackend<Block> + Send + Sync + 'static,
{
    let mut module = RpcModule::new(());

    // Standard Substrate RPCs (subset for light client)
    module.merge(SystemApiServer::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
    module.merge(TransactionPoolApiServer::new(client.clone(), pool.clone()).into_rpc())?;
    module.merge(sc_rpc::account_sync::AccountSyncApiServer::new(client.clone(), keystore.clone()).into_rpc())?;
    module.merge(sc_rpc::system_local::SystemLocalApiServer::new(client.clone(), keystore.clone()).into_rpc())?;

    // CritterChain Custom RPCs (if read-only operations are needed for light clients)
    // Example: module.merge(CritterchainApiServer::new(client.clone()).into_rpc())?;

    // Light clients typically do NOT run consensus authoring RPCs (Babe or Grandpa).
    // They might have read-only versions of consensus RPCs if needed to query chain state (e.g., authorities).
    // These are often merged from their respective `rpc` crates (e.g., `sc_consensus_babe_rpc`).

    Ok(module)
}
