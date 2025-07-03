//! CritterChain Node Chain Specification
//!
//! This module defines the chain specifications for CritterChain, including:
//! - Development configuration (single validator for local development).
//! - Local testnet configuration (multiple validators for local testing).
//! - The core genesis configuration for the CritterChain runtime.
//!
//! This file is meticulously crafted to align with The Architect's vision,
//! ensuring a robust and well-defined starting state for the CritterCraft digital ecosystem.

#![allow(clippy::redundant_closure_call)] // Allow explicit closures for genesis functions
#![allow(clippy::type_complexity)] // Allow complex types for Substrate definitions

//! CritterChain Node Chain Specification
//!
//! # The Architect's Vision
//!
//! This module defines the genesis blueprint for the Critter-Craft Universe.
//! Guided by the Expanded KISS Principle, it establishes a clear, modular, and
//! strategically sound foundation for the ecosystem. The genesis block is not
//! merely a technical starting point; it is the first chapter in the story of
//! Critter-Craft, imbuing the chain with initial lore, purpose, and balance.
//!
//! It specifies configurations for:
//! - **Development:** A single-validator network for rapid local iteration.
//! - **Local Testnet:** A multi-validator network for testing consensus and interactions.

#![allow(clippy::redundant_closure_call)]
#![allow(clippy::type_complexity)]

use critterchain_runtime::{
    AccountId, Balance, BlockNumber, RuntimeGenesisConfig, SessionKeys, Signature, WASM_BINARY, PTCN,
};
use sc_service::{ChainType, GenericChainSpec};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::Perbill;

// --- Pallet Genesis Config Imports ---
use critterchain_runtime::{
    pallet_balances::GenesisConfig as BalancesConfig,
    pallet_grandpa::GenesisConfig as GrandpaConfig,
    pallet_session::GenesisConfig as SessionConfig,
    pallet_staking::{Forcing, GenesisConfig as StakingConfig, StakerStatus},
    pallet_sudo::GenesisConfig as SudoConfig,
    // --- CritterCraft Pallet Genesis ---
    pallet_critter_nfts::GenesisConfig as CritterNftsConfig,
    pallet_quests::GenesisConfig as QuestsConfig,
};

type AccountPublic = <Signature as Verify>::Signer;

/// A set of helper constants for defining the genesis state.
/// (S) - Systematizes configuration, making it easy to tune.
mod constants {
    use super::PTCN;
    use critterchain_runtime::Balance;

    pub const ENDOWMENT: Balance = 1_000_000 * PTCN;
    pub const STASH_BOND: Balance = 10_000 * PTCN;

    pub mod seeds {
        pub const ARCHITECT: &str = "Alice";
        pub const GUARDIAN_1: &str = "Bob";
        pub const GUARDIAN_2: &str = "Charlie";
        pub const PLAYER_1: &str = "Dave";
        pub const PLAYER_2: &str = "Eve";
    }
}


// Import runtime types from your `critterchain_runtime` crate
// These imports are crucial and must correctly reference your runtime's pallet GenesisConfig types.
use critterchain_runtime::{
    AccountId,            // Account ID type (from frame_system)
    Balance,              // Balance type (from pallet_balances)
    Signature,            // Signature type (from sp_runtime)
    BlockNumber,          // Block number type (from frame_system)
    // Pallet Genesis Config types - these must match what's defined in runtime/src/lib.rs `impl_genesis_config!`
    pallet_balances::GenesisConfig as BalancesConfig,
    pallet_session::GenesisConfig as SessionConfig,
    pallet_staking::{Forcing, GenesisConfig as StakingConfig, StakerStatus},
    pallet_sudo::GenesisConfig as SudoConfig,
    pallet_babe::GenesisConfig as BabeConfig,
    pallet_grandpa::GenesisConfig as GrandpaConfig,
    pallet_im_online::GenesisConfig as ImOnlineConfig,
    
    // --- CritterCraft Specific Pallets GenesisConfig Imports ---
    // These are critical for defining the starting state of our game-specific logic.
    pallet_critter_nfts::GenesisConfig as CritterNftsConfig,
    pallet_marketplace::GenesisConfig as MarketplaceConfig,
    pallet_quests::GenesisConfig as QuestsConfig,
    pallet_rewards::GenesisConfig as RewardsConfig, // Assuming a pallet for distributing rewards
    pallet_users::GenesisConfig as UsersConfig, // Assuming a pallet for user profiles/scores
    pallet_items::GenesisConfig as ItemsConfig, // Assuming a pallet for items
    pallet_breeding::GenesisConfig as BreedingConfig, // Assuming a pallet for breeding
    pallet_battles::GenesisConfig as BattlesConfig, // Assuming a pallet for battles
    
    // --- New Governance and Blockchain Functionality Pallets ---
    pallet_critter_governance::GenesisConfig as GovernanceConfig,
    pallet_critter_node_rewards::GenesisConfig as NodeRewardsConfig,
    pallet_critter_treasury::GenesisConfig as TreasuryConfig,
    
    // The aggregate RuntimeGenesisConfig for the runtime
    RuntimeGenesisConfig,
    // The compiled Wasm blob of the runtime (usually from `build.rs` or `#[no_link]` attribute)
    WASM_BINARY,
    // A constant defining the base unit for PTCN tokens (e.g., 10^10 for 10 decimals)
    PTCN,
    SessionKeys, // Import runtime's SessionKeys struct, typically custom for each runtime
};

// Type alias for the signature type used by accounts (e.g., sr25519::Public).
type AccountPublic = <Signature as Verify>::Signer;

// --- Key Generation Utilities ---

/// A structured representation of the keys for a network authority.
/// (K) - Keeps the core concept of an "authority" clear and self-contained.
#[derive(Debug, Clone)]
pub struct AuthorityKeys {
    pub stash: AccountId,
    pub controller: AccountId,
    pub session_keys: SessionKeys,
}

impl AuthorityKeys {
    /// Generates a full set of authority keys from a seed string.
    pub fn from_seed(seed: &str) -> Self {
        let controller = get_account_id_from_seed::<sr25519::Public>(seed);
        let stash = get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed));
        let session_keys = SessionKeys {
            grandpa: get_from_seed::<sp_consensus_grandpa::AuthorityId>(seed),
            babe: get_from_seed::<sp_consensus_babe::AuthorityId>(seed),
            im_online: get_from_seed::<pallet_im_online::sr25519::AuthorityId>(seed),
        };
        Self { stash, controller, session_keys }
    }
}

/// Generates a public key from a seed string.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> TPublic {
    TPublic::from_string(&format!("//{}", seed))
        .expect("static seed to be valid; `from_string` succeeds")
}

/// Generates an AccountId from a seed string.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<TPublic>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

// --- Chain Specification Definitions ---

// --- Chain Specification Definitions ---

/// Generates a development chain spec (single validator).
pub fn development_config() -> Result<GenericChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development Wasm binary not available")?;
    let architect = get_account_id_from_seed::<sr25519::Public>(constants::seeds::ARCHITECT);

    Ok(GenericChainSpec::builder(wasm_binary, None)
        .with_chain_type(ChainType::Development)
        .with_name("CritterChain Development")
        .with_id("critter_dev")
        .with_protocol_id("critter_dev")
        .with_genesis_config_constructor(Box::new(move || {
            // (I) - Iterate Intelligently: The builder pattern makes genesis construction clear.
            GenesisBuilder::new()
                .with_sudo(architect.clone())
                .with_validators(vec![AuthorityKeys::from_seed(constants::seeds::ARCHITECT)])
                .with_endowed_accounts(vec![
                    architect,
                    get_account_id_from_seed::<sr25519::Public>(constants::seeds::GUARDIAN_1),
                ])
                .with_crittercraft_genesis()
                .build()
        }))
        .with_properties(critterchain_properties())
        .build())
}

/// Generates a local testnet chain spec (multiple validators).
pub fn local_testnet_config() -> Result<GenericChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Testnet Wasm binary not available")?;
    let architect = get_account_id_from_seed::<sr25519::Public>(constants::seeds::ARCHITECT);

    Ok(GenericChainSpec::builder(wasm_binary, None)
        .with_chain_type(ChainType::Local)
        .with_name("CritterChain Local Testnet")
        .with_id("critter_local")
        .with_protocol_id("critter_local")
        .with_genesis_config_constructor(Box::new(move || {
            GenesisBuilder::new()
                .with_sudo(architect.clone())
                .with_validators(vec![
                    AuthorityKeys::from_seed(constants::seeds::ARCHITECT),
                    AuthorityKeys::from_seed(constants::seeds::GUARDIAN_1),
                    AuthorityKeys::from_seed(constants::seeds::GUARDIAN_2),
                ])
                .with_endowed_accounts(vec![
                    architect,
                    get_account_id_from_seed::<sr25519::Public>(constants::seeds::PLAYER_1),
                    get_account_id_from_seed::<sr25519::Public>(constants::seeds::PLAYER_2),
                ])
                .with_crittercraft_genesis()
                .build()
        }))
        .with_properties(critterchain_properties())
        .build())
}

/// Helper to create common chain properties (token symbol, decimals).
fn critterchain_properties() -> sc_service::Properties {
    let mut properties = sc_service::Properties::new();
    properties.insert("tokenSymbol".into(), "AURA".into()); // Using AURA from our economic model
    properties.insert("tokenDecimals".into(), 12.into());
    properties
}, // The initial Sudo account
        },
        // Babe Pallet: The block production consensus engine.
        babe: BabeConfig {
            authorities: vec![], // Babe authorities are derived from Session pallet via Staking.
            epoch_config: Some(critterchain_runtime::BABE_GENESIS_EPOCH_CONFIG), // Use the genesis epoch config from runtime constants
    }

    /// Sets the initial set of validators.
    pub fn with_validators(mut self, validators: Vec<AuthorityKeys>) -> Self {
        self.validators = validators;
        self
    }

    /// Sets the accounts that will be endowed with initial funds.
    pub fn with_endowed_accounts(mut self, accounts: Vec<AccountId>) -> Self {
        self.endowed_accounts = accounts;
        self
    }

    /// Configures the initial state of the CritterCraft game world.
    /// (I) - Integrates game lore and content directly into the chain's genesis.
    pub fn with_crittercraft_genesis(mut self) -> Self {
        // Mint a single, non-transferable "Primordial Egg" to the Sudo account ("The Architect").
        // This creates a lore-rich origin for all future critters.
        self.config.critter_nfts = CritterNftsConfig {
            initial_critters: vec![
                (
                    self.config.sudo.key.clone().unwrap(),
                    // Pet data: species, aura, etc.
                    // In a real implementation, you'd define this struct.
                    b"Primordial Egg".to_vec(),
                )],
            ..Default::default()
        };

        // Define a few starting quests.
        self.config.quests = QuestsConfig {
            initial_quests: vec![
                // Quest ID, Quest Details (e.g., description, rewards)
                (0, b"Gather 10 Sunpetal Pollens".to_vec()),
                (1, b"Craft your first Healing Salve".to_vec()),
            ],
            ..Default::default()
        };
        
        // Initialize governance with default parameters
        self.config.governance = GovernanceConfig {
            voting_period: 50400, // ~1 week at 12-second blocks
            proposal_bond: 1000,
            voting_bond: 100,
            min_proposal_deposit: 100,
            ..Default::default()
        };
        
        // Initialize node rewards with default parameters
        self.config.node_rewards = NodeRewardsConfig {
            reward_parameters: Default::default(),
        };
        
        // Initialize treasury with default parameters and initial funds
        self.config.treasury = TreasuryConfig {
            treasury_params: Default::default(),
            initial_balance: 1_000_000,
        };

        self
    }

    /// Builds the final `RuntimeGenesisConfig` after all configurations are set.
    pub fn build(mut self) -> RuntimeGenesisConfig {
        self.build_balances();
        self.build_staking();
        self.build_session();
        self.config
    }

    /// Private helper to configure the Balances pallet.
    fn build_balances(&mut self) {
        // Ensure all validators (stash and controller) and specified users are endowed.
        let mut unique_endowed = self.endowed_accounts.clone();
        for v in &self.validators {
            unique_endowed.push(v.stash.clone());
            unique_endowed.push(v.controller.clone());
        }
        unique_endowed.sort();
        unique_endowed.dedup();

        self.config.balances = BalancesConfig {
            balances: unique_endowed.into_iter().map(|k| (k, constants::ENDOWMENT)).collect(),
        };
    }

    /// Private helper to configure the Staking and ImOnline pallets.
    fn build_staking(&mut self) {
        self.config.staking = StakingConfig {
            stakers: self
                .validators
                .iter()
                .map(|v| {
                    (v.stash.clone(), v.controller.clone(), constants::STASH_BOND, StakerStatus::Validator)
                })
                .collect(),
            validator_count: self.validators.len() as u32,
            minimum_validator_count: 1,
            invulnerables: self.validators.iter().map(|v| v.stash.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        };
    }
    
    /// Private helper to configure the Session, Grandpa, and Babe pallets.
    fn build_session(&mut self) {
         let session_keys: Vec<_> = self.validators.iter().map(|v| {
            (v.stash.clone(), v.stash.clone(), v.session_keys.clone())
        }).collect();

        self.config.session = SessionConfig { keys: session_keys };
        // Babe and Grandpa authorities are derived from the Session pallet, so we
        // just need to provide the epoch config for Babe.
        self.config.babe = Default::default(); // Uses default epoch config from runtime
        self.config.grandpa = Default::default();
        self.config.im_online = Default::default();
    }
}

//       using `impl_genesis_config!` macro or manually defined struct with `#[derive(Default)]`.