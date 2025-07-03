//! CritterChain Node RPC Gateway
//!
//! # The Architect's Vision
//!
//! This module defines and constructs the JSON-RPC interface for the CritterChain node.
//! Applying the Expanded KISS Principle, it uses a modular `RpcModuleBuilder` to create a
//! clean, scalable, and declarative API gateway. It separates core Substrate RPCs from
//! the rich, custom RPCs that expose the unique functionality of the Critter-Craft
//! ecosystem to the outside world.

#![warn(missing_docs)]

use std::sync::Arc;
use jsonrpsee::RpcModule;
use sc_client_api::BlockchainEvents;
use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;

// --- Crate-level Type Aliases for Clarity (K - Keep it Clear) ---
use critterchain_runtime::{opaque::Block, AccountId, Balance, Nonce, RuntimeApi};
pub use sc_rpc_api::DenyUnsafe;

/// A type representing all client dependencies required by the RPC builder.
/// (I) - This simplifies function signatures dramatically.
pub struct RpcDependencies<C, P> {
    pub client: Arc<C>,
    pub pool: Arc<P>,
    pub deny_unsafe: DenyUnsafe,
    // Dependencies for specific RPCs, can be extended as needed
    pub grandpa_link: Option<sc_consensus_grandpa::GrandpaLink<Block>>,
    pub babe_link: Option<sc_consensus_babe::BabeLink<Block>>,
}

/// A builder for creating the final `RpcModule`.
/// (S) - This systematizes RPC construction, making it scalable and easy to read.
pub struct RpcModuleBuilder<C, P> {
    module: RpcModule<()>,
    deps: RpcDependencies<C, P>,
}

impl<C, P> RpcModuleBuilder<C, P>
where
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    C::Api: RuntimeApi,
    P: TransactionPool<Block = Block> + 'static,
{
    /// Creates a new `RpcModuleBuilder`.
    pub fn new(deps: RpcDependencies<C, P>) -> Self {
        Self { module: RpcModule::new(()), deps }
    }

    /// Adds the standard system and transaction-related RPCs.
    pub fn add_system_rpcs(mut self) -> Result<Self, String> {
        use sc_rpc::{SystemApiServer, TransactionPoolApiServer};

        self.module
            .merge(SystemApiServer::new(
                self.deps.client.clone(),
                self.deps.pool.clone(),
                self.deps.deny_unsafe,
            ).into_rpc())
            .map_err(|e| format!("Failed to merge System RPC: {}", e))?;

        self.module
            .merge(TransactionPoolApiServer::new(self.deps.client.clone(), self.deps.pool.clone()).into_rpc())
            .map_err(|e| format!("Failed to merge Transaction Pool RPC: {}", e))?;

        Ok(self)
    }

    /// Adds the consensus-specific RPCs for a full node.
    pub fn add_consensus_rpcs(mut self) -> Result<Self, String> {
        use sc_consensus_babe_rpc::BabeApiServer;
        use sc_consensus_grandpa_rpc::GrandpaApiServer;

        if let (Some(grandpa_link), Some(babe_link)) =
            (self.deps.grandpa_link.as_ref(), self.deps.babe_link.as_ref())
        {
            // Note: In a real implementation, subscription executor and other details would be passed here.
            // This is simplified for clarity.
            self.module
                .merge(GrandpaApiServer::new(
                    grandpa_link.clone(),
                    grandpa_link.shared_voter_state(),
                    grandpa_link.justification_stream(),
                    // Placeholder for subscription_executor and telemetry
                    Default::default(),
                    Default::default(),
                ).into_rpc())
                .map_err(|e| format!("Failed to merge Grandpa RPC: {}", e))?;

            self.module
                .merge(BabeApiServer::new(
                    self.deps.client.clone(),
                    babe_link.shared_authority_set().clone(),
                    // Placeholder for keystore
                    Default::default(),
                    babe_link.clone(),
                ).into_rpc())
                .map_err(|e| format!("Failed to merge Babe RPC: {}", e))?;
        }

        Ok(self)
    }

    /// Adds the custom CritterCraft game-specific RPCs.
    /// (I) - Integrates our specific game logic into the RPC layer.
    pub fn add_crittercraft_rpcs(mut self) -> Result<Self, String> {
        // This assumes a `critterchain_runtime_rpc` crate with these API servers defined.
        use critterchain_runtime_rpc::{
            CritterchainApiServer, // A generic API for top-level info
            PetNftApiServer,     // Specific API for querying Pet NFTs
            MarketplaceApiServer, // Specific API for marketplace listings
        };

        // These would be implemented in your runtime-rpc crate.
        // self.module.merge(PetNftApiServer::new(self.deps.client.clone()).into_rpc())?;
        // self.module.merge(MarketplaceApiServer::new(self.deps.client.clone()).into_rpc())?;
        
        // For now, we merge the placeholder top-level API.
        self.module
            .merge(CritterchainApiServer::new(self.deps.client.clone()).into_rpc())
            .map_err(|e| format!("Failed to merge CritterChain Custom RPC: {}", e))?;

        Ok(self)
    }

    /// Consumes the builder and returns the final `RpcModule`.
    pub fn build(self) -> RpcModule<()> {
        self.module
    }
}

/// Instantiates all RPC extensions for a full node.
pub fn create_full<C, P>(deps: RpcDependencies<C, P>) -> Result<RpcModule<()>, String>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + BlockchainEvents<Block>
        + Send
        + Sync
        + 'static,
    C::Api: critterchain_runtime_rpc::CritterchainRuntimeApi<Block>, // Ensure runtime implements the custom API trait
    P: TransactionPool<Block = Block> + 'static,
{
    RpcModuleBuilder::new(deps)
        .add_system_rpcs()?
        .add_consensus_rpcs()?
        .add_crittercraft_rpcs()?
        .build()
        .into() // Convert Result<RpcModule, String> to Result<RpcModule, Box<dyn Error...>> if needed
}

/// Instantiates RPC extensions for a light client.
/// (S) - Systematically builds a subset of RPCs, reusing the builder logic.
pub fn create_light<C, P>(deps: RpcDependencies<C, P>) -> Result<RpcModule<()>, String>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + BlockchainEvents<Block>
        + Send
        + Sync
        + 'static,
    C::Api: critterchain_runtime_rpc::CritterchainRuntimeApi<Block>,
    P: TransactionPool<Block = Block> + 'static,
{
    // Light clients get system and game-specific RPCs, but not consensus authoring RPCs.
    RpcModuleBuilder::new(deps)
        .add_system_rpcs()?
        .add_crittercraft_rpcs()? // Exposing read-only game state is useful for light clients
        .build()
        .into()
}