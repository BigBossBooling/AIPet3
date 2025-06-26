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

use sp_core::{Pair, Public, sr25519}; // For key generation utilities
use sp_runtime::{
    traits::{IdentifyAccount, Verify}, // Traits for account identification and signature verification
    Perbill, // For percentage calculations (e.g., slash_reward_fraction)
};
use sc_service::ChainType; // For ChainType enum (Development, LocalTestnet, Live)
use sp_consensus_babe::AuthorityId as BabeId; // Babe consensus authority ID
use sp_consensus_grandpa::AuthorityId as GrandpaId; // Grandpa finality authority ID
use pallet_im_online::sr25519::AuthorityId as ImOnlineId; // ImOnline pallet authority ID (sr25519 specific)
use sc_service::GenericChainSpec; // Generic builder for ChainSpec implementations


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

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> TPublic {
    TPublic::from_string(&format!("//{}", seed))
        .expect("static seed to be valid and generate a valid public key")
}

/// Helper function to generate an AccountId from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<TPublic>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate authority keys (Stash, Controller, Session Keys) from a seed.
/// This is a common pattern for testnets. For mainnet, keys would be generated securely and offline.
/// Returns (StashId, ControllerId, GrandpaKey, BabeKey, ImOnlineKey)
pub fn authority_keys_from_seed(
    seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)), // Stash account
        get_account_id_from_seed::<sr25519::Public>(seed), // Controller account
        get_from_seed::<GrandpaId>(seed),                  // Grandpa key
        get_from_seed::<BabeId>(seed),                     // Babe key
        get_from_seed::<ImOnlineId>(seed),                 // ImOnline key
    )
}

// --- Chain Specification Definitions ---

/// Development config (single validator for local rapid development).
pub fn development_config() -> Result<GenericChainSpec, String> {
    let wasm_binary = WASM_BINARY
        .ok_or_else(|| "Development Wasm binary not available, please build it!".to_string())?;

    Ok(GenericChainSpec::builder(
        wasm_binary,
        None, // No bootnodes for single dev node, or specify local peer id
        ChainType::Development,
        move || {
            testnet_genesis(
                vec![authority_keys_from_seed("Alice")], // Alice is the sole validator
                get_account_id_from_seed::<sr25519::Public>("Alice"), // Alice as sudo
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                true, // Enable NPoS from genesis
            )
        },
        vec![], // Bootnodes (empty for local dev)
        None,   // Telemetry endpoints
        Some("critterchain_dev"), // Protocol ID
        None,   // Fork ID
        Some(critterchain_properties()), // Chain properties (token symbol, decimals)
        None,   // Extensions
    )?)
}

/// Local testnet config (multiple validators for local multi-node testing).
pub fn local_testnet_config() -> Result<GenericChainSpec, String> {
    let wasm_binary = WASM_BINARY
        .ok_or_else(|| "Testnet Wasm binary not available, please build it!".to_string())?;

    Ok(GenericChainSpec::builder(
        wasm_binary,
        None, // No bootnodes initially
        ChainType::Local, // Explicitly local chain type
        move || {
            testnet_genesis(
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                    authority_keys_from_seed("Charlie"), // Adding Charlie for a larger testnet
                ],
                get_account_id_from_seed::<sr25519::Public>("Alice"), // Alice as sudo
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"), // Ensure stashes are endowed
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                ],
                true, // Enable NPoS from genesis
            )
        },
        vec![], // Bootnodes (empty for local testnet)
        None,   // Telemetry endpoints
        Some("critterchain_local"), // Protocol ID
        None,   // Fork ID
        Some(critterchain_properties()), // Chain properties
        None,   // Extensions
    )?)
}

/// Helper function to create common chain properties for CritterChain.
pub fn critterchain_properties() -> sc_service::Properties {
    let mut properties = sc_service::Properties::new();
    properties.insert("tokenSymbol".into(), "PTCN".into());
    properties.insert("tokenDecimals".into(), 10.into()); // 10 decimal places for PTCN
    // properties.insert("ss58Format".into(), 42.into()); // Substrate generic format
    properties
}

/// Configure initial storage state for FRAME modules for a testnet/development genesis.
/// This function constructs the `RuntimeGenesisConfig` for your runtime.
fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId)>, // (Stash, Controller, GrandpaKey, BabeKey, ImOnlineKey)
    root_key: AccountId, // Sudo key
    mut endowed_accounts: Vec<AccountId>, // Use mut to add authority stash accounts if not already present
    enable_npos: bool, // Explicitly enable/disable NPoS config
) -> RuntimeGenesisConfig {
    const ENDOWMENT: Balance = 1_000_000 * PTCN; // Example: 1 Million PTCN for endowed accounts
    const STASH_BOND: Balance = 10_000 * PTCN;   // Example: 10,000 PTCN self-stake for validators

    // Ensure all stash accounts are also in endowed_accounts to receive funds
    // This is crucial for their bonding transaction to succeed in a real chain.
    for (stash, _, _, _, _) in initial_authorities.iter() {
        if !endowed_accounts.contains(stash) {
            endowed_accounts.push(stash.clone());
        }
    }

    // Prepare session keys for authorities
    let session_keys: Vec<(AccountId, AccountId, SessionKeys)> = initial_authorities
        .iter()
        .map(|x| {
            (
                x.0.clone(), // Stash AccountId (ValidatorId)
                x.1.clone(), // Controller AccountId
                SessionKeys { // Use runtime's SessionKeys struct definition (from critterchain_runtime)
                    grandpa: x.2.clone(),
                    babe: x.3.clone(),
                    im_online: x.4.clone(),
                    // Add other keys if defined in runtime::SessionKeys (e.g., authority_discovery)
                },
            )
        })
        .collect();

    RuntimeGenesisConfig {
        // System Pallet: Basic system configuration.
        system: critterchain_runtime::SystemConfig {
            // WASM_BINARY is passed to GenericChainSpec::builder, no need to put in SystemConfig.
            // This ensures the runtime code is loaded correctly.
            // Other fields like block_weights, block_length limits typically use default.
            ..Default::default()
        },
        // Balances Pallet: Manages PTCN token balances.
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        },
        // Session Pallet: Manages validator session keys.
        session: SessionConfig {
            keys: session_keys,
        },
        // Staking Pallet: Manages validator bonding, nominations, and staking logic.
        staking: StakingConfig {
            // Staking configuration only if NPoS is explicitly enabled for this genesis.
            stakers: if enable_npos {
                initial_authorities
                    .iter()
                    .map(|x| {
                        (
                            x.0.clone(), // Stash AccountId (the bonded funds)
                            x.1.clone(), // Controller AccountId (manages bonding/unbonding)
                            STASH_BOND,  // Amount bonded (self-stake)
                            StakerStatus::Validator, // Initial status: validator
                        )
                    })
                    .collect()
            } else {
                vec![] // Empty stakers if NPoS is not enabled
            },
            validator_count: initial_authorities.len() as u32, // Number of initial validators
            minimum_validator_count: initial_authorities.len().min(1).max(1) as u32, // At least 1 validator for a viable chain
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(), // Initial validators are invulnerable
            slash_reward_fraction: Perbill::from_percent(10), // Example: 10% of slashed funds are rewarded to reporter
            force_era: Forcing::ForceNone, // Default to no forced era changes for testnets
            // MaxNominatorsCount, MaxValidatorsCount, ChillThreshold, MinValidatorBond etc.
            // are typically set in the runtime's staking pallet config (via #[pallet::config])
            // and often take their default unless overridden.
            ..Default::default() // Use default for other StakingConfig fields
        },
        // Sudo Pallet: Provides a single account with superuser privileges.
        sudo: SudoConfig {
            key: Some(root_key), // The initial Sudo account
        },
        // Babe Pallet: The block production consensus engine.
        babe: BabeConfig {
            authorities: vec![], // Babe authorities are derived from Session pallet via Staking.
            epoch_config: Some(critterchain_runtime::BABE_GENESIS_EPOCH_CONFIG), // Use the genesis epoch config from runtime constants
            // Initial randomness for Babe
            // randomness: Default::default(), // Usually default or explicitly set for production
            // Other babe config fields typically use Default::default()
            ..Default::default()
        },
        // Grandpa Pallet: The finality gadget.
        grandpa: GrandpaConfig {
            authorities: vec![], // Grandpa authorities are derived from Session pallet via Staking.
            // Other grandpa config fields
            ..Default::default()
        },
        // ImOnline Pallet: Tracks validator liveness.
        im_online: ImOnlineConfig {
            keys: vec![], // ImOnline keys are also derived from Session pallet
            ..Default::default()
        },
        // Transaction Payment Pallet: Manages transaction fees.
        transaction_payment: Default::default(), // Uses default genesis config

        // --- CritterCraft Specific Pallets Genesis State ---
        // These need to be configured based on your runtime's construct_runtime! macro
        // and the GenesisConfig of each pallet. They represent the initial state of
        // our game's digital ecosystem when the chain launches.

        // `pallet-critter-nfts`: Defines initial NFT details (e.g., next ID, any pre-minted NFTs).
        critter_nfts: CritterNftsConfig {
            // For MVP, start with next_critter_id at 0 to signify no pets minted yet.
            // Potentially pre-mint a few "genesis" critters for specific accounts.
            next_critter_id: 0,
            // Example: initial_critters: vec![ (AccountId, PetId, PetNft) ],
            // _phantom: Default::default(), // If your Config requires a PhantomData
            ..Default::default()
        },
        // `pallet-marketplace`: Initial marketplace state (e.g., no listings yet).
        marketplace: MarketplaceConfig {
            // _phantom: Default::default(),
            ..Default::default()
        },
        // `pallet-quests`: Define initial quests available from genesis.
        quests: QuestsConfig {
            // Example: initial_quests: vec![ (QuestId, QuestDefinition) ],
            // _phantom: Default::default(),
            ..Default::default()
        },
        // `pallet-rewards`: Configure initial reward pools or parameters.
        rewards: RewardsConfig {
            // _phantom: Default::default(),
            ..Default::default()
        },
        // `pallet-users`: Initial user profiles or global user-related settings.
        users: UsersConfig {
            // _phantom: Default::default(),
            ..Default::default()
        },
        // `pallet-items`: Define initial item definitions, next ID, or pre-minted items to users.
        items: ItemsConfig {
            // Example: initial_item_definitions: vec![ (ItemId, ItemDetails) ],
            // Example: initial_user_inventory: vec![ (AccountId, ItemId, u32) ],
            // next_item_id: 0,
            // _phantom: Default::default(),
            ..Default::default()
        },
        // `pallet-breeding`: Initial breeding parameters or conceptual gene pool data.
        breeding: BreedingConfig {
            // _phantom: Default::default(),
            ..Default::default()
        },
        // `pallet-battles`: Initial battle parameters, or pending challenges if any.
        battles: BattlesConfig {
            // _phantom: Default::default(),
            ..Default::default()
        },
        // Ensure all pallets included in construct_runtime! have their genesis configured here if needed.
        // If your RuntimeGenesisConfig uses `#[derive(Default)]`, ensure all fields have a Default.
        // If `RuntimeGenesisConfig` is built by `impl_genesis_config!`, this structure might differ slightly.
        // We assume RuntimeGenesisConfig has a Default for this conceptual outline.
        ..Default::default()
    }
}
// Note: Ensure that the `RuntimeGenesisConfig` struct matches the expected structure
//       of your runtime's genesis configuration. This is typically defined in `runtime/src/lib.rs`
//       using `impl_genesis_config!` macro or manually defined struct with `#[derive(Default)]`.