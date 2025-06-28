# CritterCraft: Custom Pallet Dependencies (Conceptual `Cargo.toml` Outlines)

This document provides conceptual `Cargo.toml` outlines for the primary custom pallets developed for CritterCraft. It details standard Substrate/FRAME dependencies and, crucially, inter-dependencies between the custom CritterCraft pallets, highlighting how traits and types are shared via the `crittercraft-traits` crate.

## General Note on Inter-Pallet Dependencies & Traits

The introduction of the `crittercraft-traits` crate (conceptually located at `blockchain_core/traits/src/lib.rs`) is the primary strategy for managing inter-pallet interactions and avoiding circular dependencies in the build process.

*   **Centralized Trait Definitions:** Shared traits defining interfaces between pallets (e.g., `NftManager`, `BasicCareItemConsumer`, `NftManagerForItems`, `QuestNftRequirementChecker`, etc.) are defined in `crittercraft-traits`.
*   **Pallet Dependencies:**
    *   Pallets that *implement* one of these shared traits will depend on `crittercraft-traits` to access the trait definition they need to implement.
    *   Pallets that *consume* these traits (i.e., require another pallet to fulfill an interface via their `Config` trait) will also depend on `crittercraft-traits` to define the associated type for their `Config` trait.
    *   Direct dependencies between functional pallets (e.g., `pallet-items` on `pallet-critter-nfts`) might still be necessary if a pallet needs to use specific *concrete types* (structs, enums, type aliases like `PetId`) defined directly within another pallet and not exposed generically via the traits crate. For a fully decoupled system, such shared types might also eventually move to the traits crate or a dedicated `crittercraft-types` crate. For now, this document reflects dependencies needed for both traits and potentially some core types.

This structure promotes better modularity and cleaner dependency graphs.

---

## 1. `pallet-critter-nfts`

*Manages core Pet NFT logic. Implements `SharedNftManager`, `NftBreedingHandler`, `NftManagerForItems`, `QuestNftRequirementChecker`. Uses `BasicCareItemConsumer`.*

```toml
[package]
name = "pallet-critter-nfts"
version = "0.1.0"
authors = ["CritterCraft Developers"]
edition = "2021"

[dependencies]
# Substrate/FRAME
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-support = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-std = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-runtime = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-core = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
log = { version = "0.4", default-features = false }
frame-support-traits = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # For Currency, Randomness

# CritterCraft Traits Crate
crittercraft-traits = { path = "../../traits", default-features = false } # Path from blockchain_core/pallets/critter_nfts_pallet to blockchain_core/traits

# Optional: Direct dependency if concrete types from pallet-items are needed beyond what traits provide
# pallet-items = { path = "../pallet-items", default-features = false, optional = true }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "sp-runtime/std",
    "sp-core/std",
    "log/std",
    "frame-support-traits/std",
    "crittercraft-traits/std",
    # "pallet-items?/std",
]
```
    **NOTE on `lib.rs` Refactoring for Shared Traits (Applied in Stage 18, Step 2):**
    *   Local definitions of `NftManager`, `BasicCareItemConsumer` were removed.
    *   Uses `crate::traits::{NftManager as SharedNftManager, NftBreedingHandler, NftManagerForItems, QuestNftRequirementChecker, BasicCareItemConsumer as SharedBasicCareItemConsumer, ...}`.
    *   `Config::ItemHandler` points to `crate::traits::BasicCareItemConsumer`.
    *   Implements `crate::traits::NftManager` (as `SharedNftManager`), `crate::traits::NftBreedingHandler`, `crate::traits::NftManagerForItems`, and `crate::traits::QuestNftRequirementChecker`.
    *   Calls to `ItemHandler.consume_item_of_category` use `ItemCategoryTag` constants.

---

## 2. `pallet-marketplace`

*Manages listing, buying, and selling of Pet NFTs. Uses `NftManager` trait.*

```toml
[package]
name = "pallet-marketplace"
version = "0.1.0"
authors = ["CritterCraft Developers"]
edition = "2021"

[dependencies]
# Substrate/FRAME
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-support = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-std = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-support-traits = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # For Currency

# CritterCraft Traits Crate
crittercraft-traits = { path = "../../traits", default-features = false }

# Optional: Direct dependency if concrete types (e.g. PetId) from pallet-critter-nfts are needed.
# pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false, optional = true }


[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "crittercraft-traits/std",
    # "pallet-critter-nfts?/std",
]
```
    **NOTE on `lib.rs` Refactoring for Shared Traits:**
    *   The `lib.rs` for this pallet needs to be updated to utilize the `crittercraft-traits` crate.
    *   Add `use crittercraft_traits::{NftManager, PetId as SharedPetId};` (adjust path based on actual crate structure, e.g. `crate::traits::*` if traits is a module).
    *   Update the `Config` trait: `type NftHandler: crittercraft_traits::NftManager<Self::AccountId, Self::PetId, DispatchResult>;` (ensure `Self::PetId` aligns with `SharedPetId`).

---

## 3. `pallet-battles`

*Manages pet battle registration and outcome reporting. Uses `NftManager` trait.*

```toml
[package]
name = "pallet-battles"
version = "0.1.0"
authors = ["CritterCraft Developers"]
edition = "2021"

[dependencies]
# Substrate/FRAME
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-support = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-std = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-support-traits = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # For Currency

# CritterCraft Traits Crate
crittercraft-traits = { path = "../../traits", default-features = false }

# Optional: Direct dependency if concrete types (e.g. PetId) from pallet-critter-nfts are needed.
# pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false, optional = true }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "crittercraft-traits/std",
    # "pallet-critter-nfts?/std",
]
```
    **NOTE on `lib.rs` Refactoring for Shared Traits:**
    *   The `lib.rs` for this pallet needs to be updated to utilize the `crittercraft-traits` crate.
    *   Add `use crittercraft_traits::{NftManager, PetId as SharedPetId};`.
    *   Update the `Config` trait: `type NftHandler: crittercraft_traits::NftManager<Self::AccountId, Self::PetId, DispatchResult>;`.

---

## 4. `pallet-quests`

*Manages quest creation, completion, and rewards. Uses `QuestNftRequirementChecker`, `QuestItemRequirementChecker`, `QuestUserProfileRequirementChecker` traits.*

```toml
[package]
name = "pallet-quests"
version = "0.1.0"
authors = ["CritterCraft Developers"]
edition = "2021"

[dependencies]
# Substrate/FRAME
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-support = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-std = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-support-traits = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # For Currency

# CritterCraft Traits Crate
crittercraft-traits = { path = "../../traits", default-features = false }

# Optional: Direct dependencies if concrete types (PetId, ItemId, SpeciesType) are used from other pallets.
# pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false, optional = true }
# pallet-items = { path = "../pallet-items", default-features = false, optional = true }
# pallet-user-profile = { path = "../pallet-user-profile", default-features = false, optional = true }


[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "crittercraft-traits/std",
    # "pallet-critter-nfts?/std",
    # "pallet-items?/std",
    # "pallet-user-profile?/std",
]
```
    **NOTE on `lib.rs` Refactoring for Shared Traits:**
    *   The `lib.rs` for this pallet needs to be updated.
    *   Remove local definitions of `QuestNftRequirementChecker`, `QuestItemRequirementChecker`, `QuestUserProfileRequirementChecker`.
    *   Add `use crittercraft_traits::{QuestNftRequirementChecker, QuestItemRequirementChecker, QuestUserProfileRequirementChecker, PetId as SharedPetId, ItemId as SharedItemId, SpeciesType};`.
    *   Update the `Config` trait:
        *   `type NftChecker: crittercraft_traits::QuestNftRequirementChecker<Self::AccountId, Self::PetId, SpeciesType>;`
        *   `type ItemChecker: crittercraft_traits::QuestItemRequirementChecker<Self::AccountId, SharedItemId, DispatchResult>;`
        *   `type UserProfileChecker: crittercraft_traits::QuestUserProfileRequirementChecker<Self::AccountId>;`

---

## 5. `pallet-items`

*Manages item definitions and inventories. Implements `BasicCareItemConsumer`, `QuestItemRequirementChecker`. Uses `NftManagerForItems`.*

```toml
[package]
name = "pallet-items"
version = "0.1.0"
authors = ["CritterCraft Developers"]
edition = "2021"

[dependencies]
# Substrate/FRAME
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-support = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-std = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-support-traits = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # For Currency

# CritterCraft Traits Crate
crittercraft-traits = { path = "../../traits", default-features = false }

# Optional: Direct dependency if concrete types (e.g. PetId) from pallet-critter-nfts are needed.
# pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false, optional = true }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "crittercraft-traits/std",
    # "pallet-critter-nfts?/std",
]
```
    **NOTE on `lib.rs` Refactoring for Shared Traits:**
    *   The `lib.rs` for this pallet needs to be updated to utilize the `crittercraft-traits` crate.
    *   Remove any local definitions of traits like `NftManagerForItems` or `BasicCareItemConsumer`.
    *   Add `use crittercraft_traits::{NftManagerForItems as SharedNftManagerForItems, BasicCareItemConsumer as SharedBasicCareItemConsumer, QuestItemRequirementChecker, PetId as SharedPetId, ItemId as SharedItemId, TraitTypeString, ItemCategoryTag, ...};`.
    *   Update the `Config` trait:
        *   `type NftHandler: crittercraft_traits::NftManagerForItems<Self::AccountId, SharedPetId, TraitTypeString, BlockNumberFor<Self>, DispatchResult>;`
    *   Implement shared traits:
        *   `impl crittercraft_traits::BasicCareItemConsumer<T::AccountId, SharedItemId, ItemCategoryTag, DispatchResult> for Pallet<T> { ... }` (updating method signature and logic for `ItemCategoryTag`).
        *   `impl crittercraft_traits::QuestItemRequirementChecker<T::AccountId, SharedItemId, DispatchResult> for Pallet<T> { ... }`
    *   Ensure calls to `T::NftHandler` use the updated method names from the shared `NftManagerForItems` trait.

---

## 6. `pallet-user-profile`

*Manages user scores and reputation. Implements `QuestUserProfileRequirementChecker`.*

```toml
[package]
name = "pallet-user-profile"
version = "0.1.0"
authors = ["CritterCraft Developers"]
edition = "2021"

[dependencies]
# Substrate/FRAME
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-support = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-std = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }

# CritterCraft Traits Crate
crittercraft-traits = { path = "../../traits", default-features = false }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "crittercraft-traits/std",
]
```
    **NOTE on `lib.rs` Refactoring for Shared Traits:**
    *   The `lib.rs` for this pallet needs to be updated.
    *   Add `use crittercraft_traits::QuestUserProfileRequirementChecker;`.
    *   Implement the shared trait: `impl crittercraft_traits::QuestUserProfileRequirementChecker<T::AccountId> for Pallet<T> { ... }`.

---

## 7. `pallet-breeding`

*Manages Pet NFT breeding. Uses `NftBreedingHandler` and potentially an item handler trait.*

```toml
[package]
name = "pallet-breeding"
version = "0.1.0"
authors = ["CritterCraft Developers"]
edition = "2021"

[dependencies]
# Substrate/FRAME
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
frame-support = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-std = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-support-traits = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # For Currency, Randomness, Time

# CritterCraft Traits Crate
crittercraft-traits = { path = "../../traits", default-features = false }

# Optional: Direct dependency if concrete types (PetId, DnaHashType, SpeciesType) from pallet-critter-nfts are needed,
# or if ItemId from pallet-items is needed and not passed via a trait.
# pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false, optional = true }
# pallet-items = { path = "../pallet-items", default-features = false, optional = true }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "crittercraft-traits/std",
    # "pallet-critter-nfts?/std",
    # "pallet-items?/std",
]
```
    **NOTE on `lib.rs` Refactoring for Shared Traits:**
    *   The `lib.rs` for this pallet needs to be updated.
    *   Remove local `NftBreedingHandler` or `SimpleGeneticInfo` if defined.
    *   Add `use crittercraft_traits::{NftBreedingHandler, PetId as SharedPetId, DnaHashType, SpeciesType, SimpleGeneticInfo};`.
    *   Update the `Config` trait: `type NftHandler: crittercraft_traits::NftBreedingHandler<Self::AccountId, Self::PetId, DnaHashType, SpeciesType, DispatchResult>;`.
    *   (If fertility items are used via a trait): `type ItemHandler: crittercraft_traits::SomeFertilityItemTrait<...>;`.

This structure significantly improves modularity by centralizing trait definitions.
```
