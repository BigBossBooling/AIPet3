# CritterChain Consensus Migration: PoA to PoS/DPoS

This document outlines the conceptual plan and considerations for transitioning CritterChain from its initial Proof-of-Authority (PoA) consensus mechanism to a more decentralized and community-driven Proof-of-Stake (PoS) or Delegated Proof-of-Stake (DPoS) model. This transition is a key part of Stage 5: "Full Ecosystem & Global Expansion."

## 1. Rationale for Migration

*   **Decentralization:** PoS/DPoS significantly increases the decentralization of network validation and governance compared to PoA.
*   **Security:** A well-configured PoS system with appropriate economic incentives can provide robust network security.
*   **Community Participation:** Allows PTCN holders to actively participate in securing the network by becoming validators or by delegating their stake to chosen validators.
*   **Economic Incentives:** Rewards PTCN holders who stake their tokens, creating an additional utility and earning mechanism for PTCN.
*   **Alignment with Vision:** Supports the "jobs supporting the blockchain" aspect of the CritterCraft vision, where running a validator node is a significant role.

## 2. Overview of Necessary FRAME Pallets

The transition will involve integrating and configuring several standard FRAME pallets:

*   **`pallet-staking`**: The core pallet for managing staking, nominations, validator elections, rewards, and slashing.
*   **`pallet-session`**: Manages session keys for validators, which are different from their account keys. Validators need to set session keys to participate in consensus.
*   **`pallet-authorship`**: Tracks block authors and handles block authoring rewards (though staking rewards are typically managed by `pallet-staking`).
*   **Consensus Engine Pallets**:
    *   **`pallet-babe`**: For block production in a PoS context (BABE - Blind Assignment for Blockchain Extension). Requires session keys.
    *   **`pallet-grandpa`**: For block finalization. Requires session keys.
    *   *(Alternatively, `pallet-aura` could be used for a simpler PoS setup if BABE's complexity is not immediately needed, but BABE/GRANDPA is common for robust PoS).*
*   **`pallet-im-online`**: Allows validators to signal their online presence. Validators who are offline can be penalized (slashed).
*   **`pallet-offences`**: Handles the reporting of validator misbehavior (offences) and coordinates slashing with `pallet-staking`.
*   **`pallet-historical`**: Stores historical session information, which can be useful for `pallet-staking` and `pallet-offences`.
*   **`pallet-staking-rewards` (or similar custom logic/pallet-balances hooks)**: While `pallet-staking` handles reward distribution logic, the actual source of funds (inflationary minting vs. treasury) needs to be defined. `pallet-staking-rewards` is one option, or direct interaction with `pallet-balances` if `pallet-staking` is configured to mint new tokens.

## 3. Key Configuration Parameters for `pallet-staking`

The `pallet_staking::Config` trait in the runtime will require careful configuration. Key parameters include:

*   **`Currency`**: Set to `Balances` (assuming `pallet-balances` manages PTCN).
*   **`SessionsPerEra`**: Number of sessions in an era. An era is a period during which a specific set of validators is active.
*   **`BondingDuration`**: Number of eras PTCN must remain bonded after unbonding before it can be withdrawn.
*   **`SlashDeferDuration`**: Number of eras to defer slashing.
*   **`MaxNominations`**: Maximum number of validators a nominator can nominate.
*   **`MaxNominatorRewardedPerValidator`**: Maximum number of nominators per validator that receive rewards.
*   **`MinimumValidatorCount` / `MaxValidatorsCount`**: Defines the target range for the number of active validators.
*   **`ElectionProvider`**: How validators are chosen (e.g., `onchain::OnChainSequentialPhragmen`).
*   **`GenesisElectionProvider`**: For the initial set of validators at genesis if transitioning with existing state.
*   **Reward Mechanism**: How rewards are calculated and sourced (e.g., fixed inflation, percentage of total stake). This often involves configuring `pallet-staking` to work with the `Currency` pallet or a dedicated rewards pallet.
*   **Slashing Parameters**: Configuration for different offence types (e.g., equivocation, offline) via `pallet-offences` and how they translate to slashes in `pallet-staking`.

## 4. Conceptual Steps for Validator Setup

Validators are key to network security and operation. Setting up as a validator typically involves:

1.  **Hardware Requirements**: Running a dedicated, reliable node with sufficient CPU, RAM, disk space, and network bandwidth.
2.  **PTCN Stake**: Acquiring and bonding a significant amount of PTCN as self-stake. There's often a minimum bond required.
3.  **Node Setup**: Synchronizing a CritterChain node.
4.  **Key Generation**:
    *   Generating stash and controller account keys (for managing staking operations separately from funds).
    *   Generating session keys (BABE, GRANDPA, ImOnline, etc.) and submitting them via the `set_keys` extrinsic from `pallet-session`.
5.  **Validation Intent**: Declaring intent to validate using the `validate` extrinsic from `pallet-staking`, specifying commission rate.
6.  **Nomination (Optional but Recommended)**: Attracting nominations from other PTCN holders to increase total stake and chances of being selected for the active validator set.

## 5. Staking and Nomination Process for PTCN Holders (Delegators)

PTCN holders can participate by nominating validators:

1.  **Bond PTCN**: Lock up a chosen amount of PTCN for staking using `pallet-staking::bond`.
2.  **Nominate Validators**: Select one or more validators to nominate using `pallet-staking::nominate`. The nominator's stake is distributed among their chosen validators.
3.  **Claim Rewards**: Regularly claim staking rewards (e.g., via `pallet-staking::payout_stakers` or an automated mechanism).
4.  **Manage Stake**: Can change nominations, bond more PTCN, or unbond PTCN (subject to `BondingDuration`).

## 6. Slashing

Validators (and their nominators) can be slashed (lose a portion of their staked PTCN) for misbehavior:

*   **Equivocation**: Signing multiple conflicting blocks at the same height.
*   **Being Offline**: Failing to produce blocks or participate in finalization for an extended period.
*   Other offences as defined by `pallet-offences`.

Slashing provides an economic disincentive against actions that harm the network.

## 7. Transition Strategy from PoA

*   **Snapshot (if applicable):** If there's existing state that needs to be preserved, a snapshot and migration plan would be needed. For CritterChain, if transitioning early, a fresh genesis with PoS might be simpler.
*   **Genesis Validators:** The initial set of validators for the PoS chain would need to be defined in the genesis block.
*   **Community Communication:** Clear communication with the community about the transition, validator requirements, and staking process.

This document provides a high-level overview. Detailed technical specifications for each parameter and pallet interaction will be required during the actual implementation phase.

## 8. Runtime Implementation Recipe for PoS/DPoS on CritterChain

This section provides a more detailed "recipe" for configuring the necessary FRAME pallets in the CritterChain runtime (`runtime/src/lib.rs`) to transition from PoA to a Nominated Proof-of-Stake (NPoS) consensus model similar to Polkadot/Kusama.

### 8.1. Pallets to be Included in `construct_runtime!`

The following pallets are essential for NPoS and should be declared in the `construct_runtime!` macro:

*   **Core System & Utilities (Assumed Present):**
    *   `System: frame_system`
    *   `Timestamp: pallet_timestamp`
    *   `Balances: pallet_balances` (for PTCN as the staking and reward currency)
    *   `TransactionPayment: pallet_transaction_payment` (for fee management)
    *   `Sudo: pallet_sudo` (for initial privileged calls, potentially phased out later)

*   **Consensus Engine Pallets:**
    *   `Babe: pallet_babe` (for block production)
    *   `Grandpa: pallet_grandpa` (for block finalization)
    *   `Authorship: pallet_authorship` (to determine block authors)

*   **Staking & Validator Management Pallets:**
    *   `Staking: pallet_staking` (core NPoS logic)
    *   `Session: pallet_session` (manages validator session keys)
    *   `Historical: pallet_historical` (provides historical session data for staking)
    *   `ImOnline: pallet_im_online` (handles validator online status and heartbeats)
    *   `Offences: pallet_offences` (manages reporting of validator misbehavior)

### 8.2. Session Keys Configuration

Validators will need to generate and register specific session keys. These are typically defined in `runtime/src/lib.rs`:

```rust
// runtime/src/lib.rs (conceptual snippet)
// pub mod opaque {
//     // ... (Block, UncheckedExtrinsic, Header)
//     pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
// }
// parameter_types! {
//     pub const BlockHashCount: BlockNumber = 2400; // Example
// }
// impl frame_system::Config for Runtime { /* ... */ }

// pub typeopaque::SessionKeys = sp_runtime::impl_opaque_keys!(
//    pub struct SessionKeys {
//        pub grandpa: GrandpaId,
//        pub babe: BabeId,
//        pub im_online: ImOnlineId,
//        pub authority_discovery: AuthorityDiscoveryId, // If using pallet-authority-discovery
//    }
// );
// // For pallet_session::Config
// impl pallet_session::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type ValidatorId = <Self as frame_system::Config>::AccountId;
//     type ValidatorIdOf = pallet_staking::StashOf<Self>;
//     type ShouldEndSession = Babe; // Babe dictates session changes
//     type NextSessionRotation = Babe; // Babe dictates session changes
//     type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
//     type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
//     type Keys = opaque::SessionKeys;
//     type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
// }
// impl pallet_session::historical::Config for Runtime {
//    type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
//    type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
//}
```
*(Note: `AuthorityDiscoveryId` and `pallet-authority-discovery` are for permissionless validator discovery, might be added later for further decentralization).*

### 8.3. `impl pallet_babe::Config for Runtime`

```rust
// runtime/src/lib.rs (conceptual snippet)
// parameter_types! {
//     pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS; // e.g., 600 slots (1 hour if 6s slots), assuming one Babe epoch per session. SessionsPerEra in pallet-staking then defines how many such sessions form an era.
//     pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK; // e.g., 6000 ms
//     pub const ReportLongevity: u64 = BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get(); // BondingDuration and SessionsPerEra from pallet-staking config
// }
// impl pallet_babe::Config for Runtime {
//     type EpochDuration = EpochDuration;
//     type ExpectedBlockTime = ExpectedBlockTime;
//     type EpochChangeTrigger = pallet_babe::ExternalTrigger; // Can also be `PrimaryContext` or `SameParentContext`
//     type DisabledValidators = Session; // Use Session pallet to get disabled validators
//     type KeyOwnerProofSystem = Historical; // Use Historical pallet for key ownership proofs
//     type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, BabeId)>>::Proof;
//     type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, BabeId)>>::IdentificationTuple;
//     type HandleEquivocation = pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
//     type WeightInfo = pallet_babe::weights::SubstrateWeight<Runtime>;
//     type MaxAuthorities = MaxAuthorities; // From staking or a new constant
// }
```

### 8.4. `impl pallet_grandpa::Config for Runtime`

```rust
// runtime/src/lib.rs (conceptual snippet)
// parameter_types! {
//     pub const MaxAuthorities: u32 = 100; // Example
// }
// impl pallet_grandpa::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type KeyOwnerProofSystem = Historical;
//     type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
//     type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::IdentificationTuple;
//     type HandleEquivocation = pallet_grandpa::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
//     type WeightInfo = pallet_grandpa::weights::SubstrateWeight<Runtime>;
//     type MaxAuthorities = MaxAuthorities;
// }
```

### 8.5. `impl pallet_im_online::Config for Runtime`

```rust
// runtime/src/lib.rs (conceptual snippet)
// parameter_types! {
//    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
//    // Example: Max 32 unsigned TXs per block, half for ImOnline
//    pub const MaxHeartbeatSenders: u32 = Staking::MaxValidatorsCount::get() / 2; // Corrected placeholder name
//}
// impl pallet_im_online::Config for Runtime {
//    type AuthorityId = ImOnlineId;
//    type RuntimeEvent = RuntimeEvent;
//    type NextSessionRotation = Babe; // Or Session directly
//    type ValidatorSet = Historical;
//    type ReportUnresponsiveness = Offences;
//    type UnsignedPriority = ImOnlineUnsignedPriority;
//    type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
//    type MaxKeys = MaxAuthorities; // Number of keys that can be items in the pallet's storage
//    type MaxPeerInHeartbeats = MaxHeartbeatSenders; // Corrected placeholder name
//}
```

### 8.6. `impl pallet_offences::Config for Runtime`

```rust
// runtime/src/lib.rs (conceptual snippet)
// parameter_types! {
//    // Fraction of the validators that slashed for a given kind of offence.
//    pub OffencesFraction: Perbill = Perbill::from_percent(10);
//}
// impl pallet_offences::Config for Runtime {
//    type RuntimeEvent = RuntimeEvent;
//    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
//    type OnOffenceHandler = Staking; // Staking pallet handles the slashing
//}
```

### 8.7. `impl pallet_staking::Config for Runtime` (Detailed Recipe)

This is the core configuration for NPoS.

```rust
// runtime/src/lib.rs (conceptual snippet)
// use frame_election_provider_support::{onchain, SequentialPhragmen}; // For ElectionProvider
// use sp_runtime::traits::ExtendedBalance; // For SolutionAccuracyOf if Balance is not u128
// parameter_types! {
//     // Staking Period and Timeslots
//     pub const SessionsPerEra: sp_staking::SessionIndex = 6; // e.g., 6 sessions (6 hours if 1 hour sessions) per era
//     pub const BondingDuration: sp_staking::EraIndex = 24; // e.g., 24 eras (24 days if 1 day eras) for unbonding
//     pub const SlashDeferDuration: sp_staking::EraIndex = BondingDuration::get() / 4; // e.g., 6 eras

//     // Validators and Nominators
//     pub const MaxNominations: u32 = 16;
//     pub const MaxNominatorRewardedPerValidator: u32 = 256;
//     pub const MinimumValidatorCount: u32 = 4; // Start with a small number for testnet
//     pub const MaxValidatorsCount: u32 = 100;  // Target for a mature network

//     // Election
//     // pub election_provider_solution_type: SolutionAccuracyOf<ExtendedBalance> = SolutionAccuracyOf::<ExtendedBalance>::Maximum(16); // For Phragmen - depends on Balance type
//     pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17); // Example threshold

//     // PTCN specific (assuming 18 decimals for PTCN)
//     pub const MinNominatorBond: Balance = 100 * 1_000_000_000_000_000_000; // 100 PTCN
//     pub const MinValidatorBond: Balance = 1000 * 1_000_000_000_000_000_000; // 1000 PTCN
// }

// pub struct OnChainElectionProvider;
// impl onchain::OnChainExecution<Runtime> for OnChainElectionProvider { /* ... standard implementation ... */ }

// impl pallet_staking::Config for Runtime {
//     type Currency = Balances; // PTCN
//     type UnixTime = Timestamp;
//     type RuntimeEvent = RuntimeEvent;

//     // Staking Period and Timeslots
//     type SessionsPerEra = SessionsPerEra;
//     type BondingDuration = BondingDuration;
//     type SlashDeferDuration = SlashDeferDuration;

//     // Nominators and Validators
//     type MaxNominations = MaxNominations;
//     type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
//     type VoterList = pallet_staking::UseNominatorsAndValidatorsMap<Self>; // Standard
//     type TargetList = pallet_staking::UseValidatorsMap<Self>; // Standard

//     // Election
//     type ElectionProvider = onchain::OnChainSequentialPhragmen<Self>; // Standard election provider
//     type GenesisElectionProvider = Self::ElectionProvider; // Use the same for genesis

//     // Rewards & Slashing
//     // type RewardRemainder = Treasury; // Send remainder to treasury (requires Treasury pallet)
//     // type OnUnbalanced = Treasury; // Send slashed funds to treasury
//     // For MVP, can use `()` if no treasury pallet yet for remainders/slashed funds.
//     type RewardRemainder = ();
//     type OnUnbalanced = ();
//     type Slash = (); // Slash handler, often () or Treasury
//     type Reward = (); // Reward handler, typically () as staking pays out from an inflation model or its own pot.
//                      // If pallet_staking is to mint rewards, `pallet_staking::InflationMint<Runtime>` is used.

//     // Session Management
//     type SessionInterface = Self; // Implements pallet_session::SessionManager

//     // Other Parameters
//     type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
//     // type MinSolutionScoreBump = SolutionAccuracyOf<Balance>; // From election_provider_solution_type
//     // type SolutionImprovementThreshold = SolutionAccuracyOf<Balance>;
//     type HistoryDepth = ConstU32<84>; // Number of eras to keep in history
//     type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
//     type MinNominatorBond = MinNominatorBond;
//     type MinValidatorBond = MinValidatorBond;
//     type MaxUnlockingChunks = ConstU32<32>; // Max items in Unbonding delegators
// }
```

### 8.8. Final Steps

*   **Genesis Configuration for Staking:** The initial set of validators and their stakes would need to be configured in the chain specification's genesis block.
*   **Feature Flags:** Ensure all pallets are compiled with the correct feature flags (e.g., `std` for native, not for Wasm).
*   **Thorough Testing:** Transitioning consensus is a major upgrade and requires extensive testing on a testnet.

This recipe provides a template. Actual values for constants (`SessionsPerEra`, `BondingDuration`, etc.) would need to be chosen based on CritterChain's specific economic model and desired network dynamics. The reward mechanism (inflation vs. treasury) is a key decision point. For an MVP, a simple inflation model configured within `pallet-staking` or even manual reward distribution might be initial steps before a full treasury system.
