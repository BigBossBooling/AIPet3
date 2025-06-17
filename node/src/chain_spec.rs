// --- BEGIN CONCEPTUAL NODE chain_spec.rs ---
// File: node/src/chain_spec.rs

// This is a conceptual outline based on a standard Substrate node template,
// adapted for CritterChain with Babe/Grandpa NPoS consensus.

use sp_core::{Pair, Public, sr25519}; // For key generation
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use sc_service::ChainType; // For ChainType enum (Development, LocalTestnet, Live)
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId; // Assuming sr25519 for ImOnline
use sc_service::GenericChainSpec; // Moved GenericChainSpec import here for clarity

// Import runtime types
// Assume your runtime is in a crate named `critterchain_runtime`
use critterchain_runtime::{
    AccountId, Balance, Signature, BlockNumber, // Core types
    SystemConfig, BalancesConfig, SessionConfig, StakingConfig, SudoConfig, // Pallet Genesis Config types
    BabeConfig, GrandpaConfig, ImOnlineConfig, // Consensus Pallet Genesis Config types
    RuntimeGenesisConfig, // The aggregate GenesisConfig for the runtime
    WASM_BINARY, // Compiled Wasm blob of the runtime
    PTCN, // Constant for 1 PTCN unit (e.g., 1_000_000_000_000_000_000)
    SessionKeys, // Import runtime's SessionKeys struct
};

// Type alias for the signature type used by accounts.
type AccountPublic = <Signature as Verify>::Signer;

// --- Helper Functions for Key Generation ---

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> TPublic {
    TPublic::from_string(&format!("//{}", seed)).expect("static seed to be valid")
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
pub fn authority_keys_from_seed(
    seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)), // Stash account
        get_account_id_from_seed::<sr25519::Public>(seed), // Controller account
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
    )
}

// --- Chain Specification Definitions ---

/// Development config (single validator).
pub fn development_config() -> Result<GenericChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development Wasm binary not available".to_string())?;

    Ok(GenericChainSpec::builder(
        wasm_binary,
        None, // No bootnodes for single dev node
        ChainType::Development,
        move || { // Genesis closure - move wasm_binary
            testnet_genesis(
                // wasm_binary, // wasm_binary is captured by the closure now, not passed directly
                // Initial PoA authorities (if transitioning from PoA, otherwise directly PoS)
                // For a new NPoS chain, provide initial PoS authorities:
                vec![authority_keys_from_seed("Alice")],
                get_account_id_from_seed::<sr25519::Public>("Alice"), // Sudo account
                vec![ // Endowed accounts
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    // Ensure stash and controller accounts for Alice are also endowed if different
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                ],
                true, // Enable NPoS from genesis
            )
        },
        vec![], // Bootnodes
        None,   // Telemetry endpoints
        Some("critterchain_dev"),   // Protocol ID
        None,   // Fork ID
        None,   // Properties
        None,   // Extensions
    )?)
}

/// Local testnet config (multiple validators).
pub fn local_testnet_config() -> Result<GenericChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Testnet Wasm binary not available".to_string())?;

    Ok(GenericChainSpec::builder(
        wasm_binary,
        None,
        ChainType::Local, // Or ChainType::Custom("CritterChainTestnet")
        move || { // Genesis closure - move wasm_binary
            testnet_genesis(
                // wasm_binary,
                // Initial PoS authorities:
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                    // authority_keys_from_seed("Charlie"), // Add more for a larger testnet
                ],
                get_account_id_from_seed::<sr25519::Public>("Alice"), // Sudo account
                vec![ // Endowed accounts
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    // Ensure stash and controller accounts for authorities are endowed
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    // get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                ],
                true, // Enable NPoS from genesis
            )
        },
        vec![], // Bootnodes
        None,   // Telemetry endpoints
        Some("critterchain_local"),   // Protocol ID
        None,   // Fork ID
        None,   // Properties
        None,   // Extensions
    )?)
}


/// Configure initial storage state for FRAME modules.
/// `enable_npos`: If true, configures staking and related pallets for NPoS.
/// If false, might configure for PoA (though this function is geared towards NPoS).
fn testnet_genesis(
    // _wasm_binary: &[u8], // Wasm binary is part of GenericChainSpec now, not passed here
    initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId)>, // (Stash, Controller, GrandpaKey, BabeKey, ImOnlineKey)
    root_key: AccountId, // Sudo key
    endowed_accounts: Vec<AccountId>,
    _enable_npos: bool, // Kept for structure, but this genesis is NPoS focused
) -> RuntimeGenesisConfig { // Use the aggregate RuntimeGenesisConfig from your runtime
    const ENDOWMENT: Balance = 1_000_000 * PTCN; // Example: 1 Million PTCN for endowed accounts
    const STASH_BOND: Balance = 10_000 * PTCN;   // Example: 10,000 PTCN self-stake for validators

    RuntimeGenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: WASM_BINARY.expect("Wasm binary was not build, please build it!").to_vec(),
            ..Default::default() // Use default for other SystemConfig fields
        },
        balances: BalancesConfig {
            balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
        },
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(), // Stash AccountId (ValidatorId)
                        x.0.clone(), // Stash AccountId (again, for pallet_session expectations if ValidatorIdOf is StashOf)
                        SessionKeys { // Use runtime's SessionKeys struct definition
                            grandpa: x.2.clone(),
                            babe: x.3.clone(),
                            im_online: x.4.clone(),
                            // Add other keys if defined in runtime::SessionKeys (e.g., authority_discovery)
                        },
                    )
                })
                .collect::<Vec<_>>(),
        },
        staking: StakingConfig {
            validator_count: initial_authorities.len() as u32,
            minimum_validator_count: initial_authorities.len().min(1).max(1) as u32, // Ensure at least 1 for small testnets
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(), // Stash accounts
            slash_reward_fraction: Perbill::from_percent(10), // Example
            stakers: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(), // Stash AccountId
                        x.1.clone(), // Controller AccountId
                        STASH_BOND,  // Amount to bond (self-stake)
                        pallet_staking::StakerStatus::Validator, // Initial status
                    )
                })
                .collect::<Vec<_>>(),
            // min_nominator_bond and min_validator_bond are typically set in the runtime's staking pallet config
            // and not directly in genesis here, unless overriding default runtime config.
            ..Default::default() // Use if other fields can take default from pallet
        },
        sudo: SudoConfig {
            key: Some(root_key),
        },
        babe: BabeConfig {
            authorities: vec![], // Babe authorities are derived from Session pallet via Staking
            epoch_index: 0,
            // allowed_slots: sp_consensus_babe::AllowedSlots::PrimarySlots, // Or PrimaryAndSecondaryVRFSlots
            ..Default::default() // For `allowed_slots` if it has a sensible default in runtime
        },
        grandpa: GrandpaConfig {
            authorities: vec![], // Grandpa authorities are derived from Session pallet via Staking
            ..Default::default()
        },
        im_online: ImOnlineConfig {
            keys: vec![], // ImOnline keys are also derived from Session pallet
        },
        // --- Configure other pallets' genesis state here ---
        // E.g., pallet_critter_nfts, pallet_marketplace, pallet_quests, etc.
        // critter_nfts_pallet: Some(CritterNftsConfig { ... }), // Example
        // marketplace_pallet: Some(MarketplaceConfig { ... }), // Example
        // quests_pallet: Some(QuestsConfig { initial_quests: vec![...] }), // Example if using GenesisConfig for quests

        transaction_payment: Default::default(), // If it has a default genesis
        // Ensure all pallets included in construct_runtime! have their genesis configured here if needed.
        // For pallets that use `#[pallet::genesis_config]` and `#[pallet::genesis_build]`,
        // their config types will be fields in `RuntimeGenesisConfig`.
        // If `RuntimeGenesisConfig` uses `#[derive(Default)]`, ensure all fields have a Default.
        // If `RuntimeGenesisConfig` is built by `impl_genesis_config!`, this structure might differ slightly.
        // We assume RuntimeGenesisConfig has a Default for this conceptual outline.
        ..Default::default()
    }
}

// --- END CONCEPTUAL NODE chain_spec.rs ---
```
