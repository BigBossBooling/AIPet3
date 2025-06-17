# CritterChain: Project 2 - Core Gameplay Pallet Runtime Integration Notes

This document outlines the conceptual changes required in the CritterChain runtime (`runtime/src/lib.rs`) to integrate and configure the core gameplay pallets developed as part of "Project 2":
*   `pallet-critter-nfts` (as `CritterNftsPallet`)
*   `pallet-marketplace` (as `MarketplacePallet`)
*   `pallet-battles` (as `BattlesPallet`)
*   `pallet-quests` (as `QuestsPallet`)

It assumes that `pallet-items` and `pallet-user-profile` (conceptual outlines from Stage 8) are also being integrated to satisfy dependencies for `pallet-quests` and advanced `pallet-critter-nfts` interactions. It also assumes a Nominated Proof-of-Stake (NPoS) consensus mechanism with `pallet-staking`, `pallet-balances`, etc., is already configured as per "Project 1" / `CONSENSUS_MIGRATION.md`.

## 1. Add Pallets to Runtime Dependencies (`runtime/Cargo.toml`)

Ensure the `Cargo.toml` for the runtime includes these pallets as dependencies, pointing to their correct paths within the `blockchain_core/pallets/` directory. Example:
```toml
# In runtime/Cargo.toml
# [dependencies]
# pallet-critter-nfts = { path = "../pallets/critter_nfts_pallet", default-features = false, version = "0.1.0" } # Adjust path as needed
# pallet-marketplace = { path = "../pallets/pallet-marketplace", default-features = false, version = "0.1.0" } # Adjust path
# pallet-battles = { path = "../pallets/pallet-battles", default-features = false, version = "0.1.0" } # Adjust path
# pallet-quests = { path = "../pallets/pallet-quests", default-features = false, version = "0.1.0" } # Adjust path
# pallet-items = { path = "../pallets/pallet-items", default-features = false, version = "0.1.0" } # Dependency for quests & critter-nfts
# pallet-user-profile = { path = "../pallets/pallet-user-profile", default-features = false, version = "0.1.0" } # Dependency for quests
#
# [features]
# default = ["std"]
# std = [
#    # ... other pallets'/std' features
#    "pallet-critter-nfts/std",
#    "pallet-marketplace/std",
#    "pallet-battles/std",
#    "pallet-quests/std",
#    "pallet-items/std",
#    "pallet-user-profile/std",
# ]
```
*(Ensure correct relative paths from `runtime/Cargo.toml` to `blockchain_core/pallets/*` and appropriate versioning/tagging if used)*.

## 2. Declare Pallets in `construct_runtime!` Macro

Add these pallets to the `construct_runtime!` macro in `runtime/src/lib.rs`:
```rust
// In runtime/src/lib.rs
// construct_runtime!(
//     pub enum Runtime where
//         Block = Block,
//         NodeBlock = opaque::Block,
//         UncheckedExtrinsic = UncheckedExtrinsic
//     {
//         // ... (System, Timestamp, Balances, Sudo)
//         // ... (NPoS Consensus Pallets: Babe, Grandpa, Staking, Session, ImOnline, Offences, Historical) ...

//         // Project 2 Core Gameplay Pallets & Dependencies
//         CritterNftsPallet: pallet_critter_nfts::{Pallet, Call, Storage, Event<T>, Config<T>},
//         ItemsPallet: pallet_items::{Pallet, Call, Storage, Event<T>, Config<T>},
//         UserProfilePallet: pallet_user_profile::{Pallet, Call, Storage, Event<T>, Config<T>},
//         MarketplacePallet: pallet_marketplace::{Pallet, Call, Storage, Event<T>, Config<T>},
//         BattlesPallet: pallet_battles::{Pallet, Call, Storage, Event<T>, Config<T>},
//         QuestsPallet: pallet_quests::{Pallet, Call, Storage, Event<T>, Config<T>},
//     }
// );
```
*(Note: Added `Config<T>` to each pallet entry, which is common practice in `construct_runtime!`).*

## 3. Implement `Config` Traits for Each Pallet

Define necessary `parameter_types!` and implement the `Config` trait for each new pallet.
*(Assume `Balance` is `u128`, `BlockNumber` is `u32`, `AccountId` is standard. `PTCN` is a constant representing the smallest unit of the currency, e.g., `1_000_000_000_000`).*

### a. `impl pallet_critter_nfts::Config for Runtime`
```rust
// parameter_types! {
//    pub const MaxTraitStringLenValue: u32 = 32;
//    pub const MaxPetPersonalityTraitsValue: u32 = 5;
//    pub const MaxCharterAttributesValue: u32 = 10; // Example for charter attributes
//    pub const MaxMoodValueConst: u8 = 100;
//    pub const FeedMoodBoostConst: u8 = 20;
//    pub const PlayMoodBoostConst: u8 = 15;
//    pub const FeedXpGainConst: u32 = 10;
//    pub const PlayXpGainConst: u32 = 15;
//    pub const NeglectMoodPenaltyConst: u8 = 10;
//    pub const NeglectXpPenaltyConst: u32 = 5; // Example
//    pub const NeglectThresholdBlocksConst: BlockNumber = 1000; // Example: ~1.6 hours if 6s blocks
//    pub const MaxOwnedPetsValue: u32 = 20;
//    pub const DefaultDailyClaimAmount: Balance = 100 * PTCN; // 100 PTCN
//    pub const DefaultClaimCooldownPeriod: BlockNumber = 14400; // 1 day in blocks (14400 blocks * 6s/block)
// }
// impl pallet_critter_nfts::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances; // Assuming Balances pallet for PTCN
//     type PetRandomness = RandomnessCollectiveFlip; // Or another randomness source
//     type MaxOwnedPets = MaxOwnedPetsValue;
//     type DailyClaimAmount = DefaultDailyClaimAmount;
//     type ClaimCooldownPeriod = DefaultClaimCooldownPeriod;
//     type MaxTraitStringLen = MaxTraitStringLenValue;
//     type MaxPetPersonalityTraits = MaxPetPersonalityTraitsValue;
//     type MaxCharterAttributes = MaxCharterAttributesValue;
//     type MaxMoodValue = MaxMoodValueConst;
//     type FeedMoodBoost = FeedMoodBoostConst;
//     type PlayMoodBoost = PlayMoodBoostConst;
//     type FeedXpGain = FeedXpGainConst;
//     type PlayXpGain = PlayXpGainConst;
//     type NeglectMoodPenalty = NeglectMoodPenaltyConst;
//     type NeglectXpPenalty = NeglectXpPenaltyConst;
//     type NeglectThresholdBlocks = NeglectThresholdBlocksConst;
//     type ItemHandler = ItemsPallet; // ItemsPallet implements BasicCareItemConsumer
//     type ItemId = pallet_items::ItemId; // Assuming ItemId is defined in pallet_items
// }
```

### b. `impl pallet_items::Config for Runtime`
```rust
// parameter_types! {
//    pub const MaxItemNameLengthValue: u32 = 50;
//    pub const MaxItemDescriptionLengthValue: u32 = 200;
//    pub const MaxEffectsPerItemValue: u32 = 3;
// }
// impl pallet_items::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances;
//     type NftHandler = CritterNftsPallet; // CritterNftsPallet implements NftManagerForItems
//     type MaxItemNameLength = MaxItemNameLengthValue;
//     type MaxItemDescriptionLength = MaxItemDescriptionLengthValue;
//     type MaxEffectsPerItem = MaxEffectsPerItemValue;
//     type MaxTraitLength = MaxTraitStringLenValue; // Reuse from critter_nfts
//     type PetId = pallet_critter_nfts::PetId; // Assuming PetId is defined in pallet_critter_nfts
//     // ItemId is typically defined within pallet_items itself (e.g., pub type ItemId = u32;)
//     // PetAttributeType enum is defined in pallet_items.
// }
```

### c. `impl pallet_user_profile::Config for Runtime`
```rust
// parameter_types! {
//    pub const PetLevelScoreWeightValue: u64 = 10;
//    pub const QuestScoreWeightValue: u64 = 50;
//    pub const BattleWinScoreWeightValue: u64 = 20;
//    pub const MaxUsernameLengthValue: u32 = 32;
// }
// impl pallet_user_profile::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type PetLevelScoreWeight = PetLevelScoreWeightValue;
//     type QuestScoreWeight = QuestScoreWeightValue;
//     type BattleWinScoreWeight = BattleWinScoreWeightValue;
//     type MaxUsernameLength = MaxUsernameLengthValue;
// }
```

### d. `impl pallet_marketplace::Config for Runtime`
```rust
// parameter_types! {
//    pub const MarketplaceFixedFeeValue: Balance = 1 * (PTCN / 100); // Example: 0.01 PTCN (1% of 1 PTCN)
//    pub MarketplaceFeeDestinationAccountId: AccountId = AccountId::new([0u8; 32]); // Placeholder, set to actual treasury account
// }
// impl pallet_marketplace::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances;
//     type PetId = pallet_critter_nfts::PetId;
//     type NftHandler = CritterNftsPallet; // CritterNftsPallet implements NftManager
//     type MarketplaceFixedFee = MarketplaceFixedFeeValue;
//     type FeeDestinationAccountId = MarketplaceFeeDestinationAccountId;
// }
```

### e. `impl pallet_battles::Config for Runtime`
```rust
// parameter_types! {
//    pub const DefaultBattleRewardAmount: Balance = 10 * PTCN; // Example: 10 PTCN
// }
// impl pallet_battles::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances;
//     type PetId = pallet_critter_nfts::PetId;
//     type NftHandler = CritterNftsPallet; // CritterNftsPallet implements NftManager
//     type BattleRewardAmount = DefaultBattleRewardAmount;
// }
```

### f. `impl pallet_quests::Config for Runtime`
```rust
// parameter_types! {
//    pub const DefaultMaxQuestDescriptionLength: u32 = 512;
// }
// impl pallet_quests::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances;
//     type QuestId = u32; // Or a more specific type, e.g., pallet_quests::QuestId
//     type MaxDescriptionLength = DefaultMaxQuestDescriptionLength;
//     // Associated types for PetId, ItemId, ScoreValue if they are not globally defined
//     // and are expected by the pallet_quests::Config trait.
//     // However, the handler traits below are often how these types are implicitly known.

//     type NftChecker = CritterNftsPallet;    // CritterNftsPallet needs to impl QuestNftRequirementChecker
//     type ItemChecker = ItemsPallet;         // ItemsPallet needs to impl QuestItemRequirementChecker
//     type UserProfileChecker = UserProfilePallet; // UserProfilePallet needs to impl QuestUserProfileRequirementChecker
// }
```

**4. Implement Handler Traits in Provider Pallets:**

This is a crucial step. The provider pallets must implement the traits defined and expected by the consumer pallets.

*   **`pallet-critter-nfts` must implement:**
    *   `pallet_marketplace::NftManager` (handles locking, unlocking, transferring NFTs for marketplace)
    *   `pallet_battles::NftManager` (same trait, ensures NFT is transferable/eligible for battle)
    *   `pallet_items::NftManagerForItems` (handles applying item effects to NFT attributes)
    *   `pallet_quests::QuestNftRequirementChecker` (provides pet owner, level for quest checks).
*   **`pallet-items` must implement:**
    *   `pallet_critter_nfts::BasicCareItemConsumer` (handles consuming items for pet care actions like feed/play)
    *   `pallet_quests::QuestItemRequirementChecker` (checks if user has items and consumes them for quests).
*   **`pallet-user-profile` must implement:**
    *   `pallet_quests::QuestUserProfileRequirementChecker` (provides user profile data like battles won for quest checks).
    *   *(Conceptual)* It might also need to implement a trait that `pallet-critter-nfts` calls to update pet levels sums, or `pallet-battles` calls to update battles won. Alternatively, these pallets can directly call extrinsics or functions on `pallet-user-profile` if mutually agreed upon (less clean than traits).

This involves adding `impl TraitName for Pallet<T> { ... }` blocks in the respective provider pallet `lib.rs` files.

## 5. Genesis Configuration for New Pallets

In `chain_spec.rs` (e.g., within the `testnet_genesis` function or a similar setup for your chain spec):
*   **`CritterNftsPallet`**: No specific genesis usually, unless starting with some system-owned NFTs.
*   **`ItemsPallet`**: Potentially pre-define some basic items (e.g., "Basic Food", "Simple Toy") via its `GenesisConfig`.
    ```rust
    // Example for itemsPallet:
    // items: vec![
    //     (b"Basic Food".to_vec(), b"Restores some hunger.".to_vec(), vec![/* ItemEffect::RestoreHunger{amount: 20} */], 10 /* price */, 100 /* initial_stock */),
    // ],
    ```
*   **`UserProfilePallet`**: No specific genesis needed if profiles default to zero/empty.
*   **`MarketplacePallet`**: Usually no specific genesis config unless starting with active listings (unlikely).
*   **`BattlesPallet`**: Usually no specific genesis config.
*   **`QuestsPallet`**: Pre-populate with initial quests using its `GenesisConfig`.
    ```rust
    // Example for questsPallet:
    // initial_quests: vec![
    //     (b"Win your first battle!".to_vec(), 100 * PTCN, None, None, None, false, Some(1)), // Description, reward, req_pet_level, req_item_id, req_item_quantity, consume_item, min_battles_won
    // ],
    ```

This document provides a high-level guide for runtime integration. Specific type paths (e.g., `pallet_critter_nfts::PetId`), trait method signatures, and constant values must be exact for successful compilation. The use of associated types (e.g., `type ItemId = pallet_items::ItemId;` within `pallet_critter_nfts::Config`) is common for type sharing if not globally defined.
The handler traits (e.g., `QuestNftRequirementChecker`) are typically defined in the *consumer* pallet (e.g., `pallet-quests`) and implemented in the *provider* pallet (e.g., `pallet-critter-nfts`).
This integration is a complex but critical step in building the cohesive CritterChain application.
The `PTCN` constant should be defined in the runtime (e.g., `pub const PTCN: Balance = 1_000_000_000_000; // Assuming 12 decimals`).
The `AccountId` for `MarketplaceFeeDestinationAccountId` needs to be a valid account, potentially derived or a well-known address.
Ensure all necessary types like `PetId`, `ItemId`, `QuestId` are consistently defined and accessible. Often, they are defined as `pub type PetId = u32;` within the respective pallet's `lib.rs` and then referenced in the runtime config.
The `Config<T>` in `construct_runtime!` is crucial.
Updated paths in Cargo.toml to be more standard (e.g., `../pallets/`).
Added `MaxCharterAttributesValue` and `NeglectXpPenaltyConst` to `pallet-critter-nfts` config for completeness.
Added `MaxUsernameLengthValue` to `pallet-user-profile` config.
Clarified `ItemId` and `PetId` type definitions.
Refined genesis config examples.
Clarified trait definition and implementation locations.
Added note about `PTCN` definition and `MarketplaceFeeDestinationAccountId`.
Noted that `Config<T>` is needed in `construct_runtime!` entries.The `RUNTIME_PROJECT2_INTEGRATION.md` file has been successfully created at the repository root. This document provides a detailed conceptual guide for integrating the core gameplay pallets (`pallet-critter-nfts`, `pallet-marketplace`, `pallet-battles`, `pallet-quests`, along with dependencies `pallet-items` and `pallet-user-profile`) into the CritterChain runtime.

The document covers:
1.  Adding pallet dependencies to `runtime/Cargo.toml`.
2.  Declaring pallets in the `construct_runtime!` macro.
3.  Detailed `Config` trait implementations for each pallet, including necessary `parameter_types!`.
4.  The requirement for provider pallets to implement handler traits defined by consumer pallets.
5.  Genesis configuration for new pallets in `chain_spec.rs`, with examples for `ItemsPallet` and `QuestsPallet`.

The content is comprehensive and addresses the key aspects of runtime integration for these new pallets.

The subtask is complete.
