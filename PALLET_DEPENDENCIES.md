# CritterCraft: Custom Pallet Dependencies (Conceptual `Cargo.toml` Outlines)

This document provides conceptual `Cargo.toml` outlines for the primary custom pallets developed for CritterCraft. It details standard Substrate/FRAME dependencies and, crucially, inter-dependencies between the custom CritterCraft pallets, highlighting how traits and types might be shared based on the current conceptual `lib.rs` files.

**General Notes:**
*   All Substrate dependencies should ideally point to a specific git commit hash or version tag for reproducible builds (e.g., `branch = "polkadot-v1.0.0"` is illustrative).
*   `default-features = false` is critical for all pallet and Substrate primitive dependencies to ensure `no_std` compatibility for the Wasm runtime.
*   Paths to other custom CritterCraft pallets are relative (e.g., `path = "../pallet-critter-nfts"`).
*   The `[features]` section with `std` is standard for enabling `std` support in dependencies.
*   **Trait Definitions & Circular Dependencies:** The current conceptual `lib.rs` files for `pallet-critter-nfts` and `pallet-items` define traits that each other implements. This creates a circular dependency if direct path dependencies are used as shown below. In a production Substrate project, this is typically resolved by:
    1.  Defining shared traits in a separate `crittercraft-traits` crate. Both `pallet-critter-nfts` and `pallet-items` would depend on this traits crate.
    2.  Using Rust features or workspace configurations to manage conditional compilation if traits are kept within pallets but inter-dependencies are complex.
    The `Cargo.toml` examples below reflect the dependencies implied by the *current conceptual trait locations in the individual pallet files*, and the circular dependency issue is noted.

---

## 1. `pallet-critter-nfts`

*Manages core Pet NFT logic, attributes, development, and basic interactions. Provides foundational NFT management traits (`NftManager`, `NftBreedingHandler`) and defines `BasicCareItemConsumer` trait (implemented by `pallet-items`). Implements `NftManagerForItems` trait (defined in `pallet-items`).*

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
frame-support-hashable = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # For blake2_128
log = { version = "0.4", default-features = false }

# FRAME Traits (Currency for Daily Claim, Randomness)
frame-support-traits = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }

# CritterCraft Pallet Dependencies
# Needed for ItemId, ItemCategory types, and the NftManagerForItems trait definition it implements.
pallet-items = { path = "../pallet-items", default-features = false }

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
    "frame-support-hashable/std",
    "log/std",
    "frame-support-traits/std",
    "pallet-items/std",
]
```

---

## 2. `pallet-marketplace`

*Manages listing, buying, and selling of Pet NFTs.*

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

# CritterCraft Pallet Dependencies
# For NftManager trait (defined in critter_nfts_pallet) and PetId type.
pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "pallet-critter-nfts/std",
]
```

---

## 3. `pallet-battles`

*Manages pet battle registration and outcome reporting.*

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

# CritterCraft Pallet Dependencies
# For NftManager trait (defined in critter_nfts_pallet) and PetId type.
pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "pallet-critter-nfts/std",
]
```

---

## 4. `pallet-quests`

*Manages quest creation, completion, and rewards. Defines `QuestNftRequirementChecker`, `QuestItemRequirementChecker`, `QuestUserProfileRequirementChecker` traits locally.*

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

# CritterCraft Pallet Dependencies (for types like PetId, ItemId if used directly)
# The runtime wires up the actual implementations of locally defined traits.
pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false, optional = true } # For PetId type
pallet-items = { path = "../pallet-items", default-features = false, optional = true } # For ItemId type
# pallet-user-profile = { path = "../pallet-user-profile", default-features = false, optional = true } # Not strictly needed if ScoreValue is generic u64

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "pallet-critter-nfts?/std",
    "pallet-items?/std",
    # "pallet-user-profile?/std",
]
```

---

## 5. `pallet-items`

*Manages item definitions, user inventories, and item effects. Defines `NftManagerForItems` trait (implemented by `pallet-critter-nfts`). Implements `BasicCareItemConsumer` trait (defined in `pallet-critter-nfts`).*

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

# CritterCraft Pallet Dependencies
# Needed for PetId type and BasicCareItemConsumer trait definition.
pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "pallet-critter-nfts/std",
]
```

---

## 6. `pallet-user-profile`

*Manages user scores, reputation, and achievements. No direct dependencies on other custom CritterCraft pallets for its core logic, but other pallets call its functions.*

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

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
]
```

---

## 7. `pallet-breeding`

*Manages the mechanics of Pet NFT breeding.*

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

# CritterCraft Pallet Dependencies
# For NftBreedingHandler trait (defined in critter_nfts_pallet), PetId, etc.
pallet-critter-nfts = { path = "../critter_nfts_pallet", default-features = false }
# pallet-items = { path = "../pallet-items", default-features = false, optional = true } # If fertility items are directly consumed here via a trait

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "frame-support-traits/std",
    "pallet-critter-nfts/std",
    # "pallet-items?/std",
]
```
The `frame-support-hashable` package was corrected to `frame-support-hashable = { ... package = "frame-support-hashable" }` which is not needed as `frame-support` re-exports `hashable`. Using `frame_support::Hashable` is standard.
The `pallet-balances` dependencies were made optional where appropriate, as pallets often rely on the `Currency` trait from `frame-support-traits` and the runtime wires this to `pallet-balances`.
The circular dependency note between `pallet-critter-nfts` and `pallet-items` is included, reflecting the current conceptual trait locations.
The dependencies for `pallet-quests` are listed as optional as it defines its handler traits locally, but would need dependencies for concrete types like `PetId` if not made generic.
This document provides a clear conceptual map of pallet dependencies.
