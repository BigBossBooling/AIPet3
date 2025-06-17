# CritterChain: Node & Runtime Setup Details (Project 1 Focus)

This document provides conceptual details for setting up the Node and Runtime `Cargo.toml` files and specific parameter choices for `pallet-staking`, crucial for "Project 1: Core Protocol & Network Foundation Implementation."

## 1. Node `Cargo.toml` - Conceptual Dependencies Outline

The `node/Cargo.toml` file for a Substrate node running NPoS (Babe + Grandpa) would typically include dependencies like these (versions and exact features may vary):

```toml
[package]
name = "critterchain-node"
version = "0.1.0"
authors = ["CritterCraft Developers"]
edition = "2021"
build = "build.rs" # If using build script for wasm an runtime versions

[dependencies]
# CLI
clap = { version = "4.2", features = ["derive"] } # Or a version compatible with Substrate's sc-cli
log = "0.4"
env_logger = "0.10" # Or a compatible version

# Substrate Core & Service (Illustrative: using polkadot-v1.0.0 branch, specific tags/commits preferred)
sp-core = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-runtime = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-consensus-babe = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-consensus-grandpa = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-keystore = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-blockchain = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-api = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-benchmarking-cli = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false, optional = true }

sc-cli = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # Removed clap feature, handled by top-level clap
sc-service = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false, features = ["wasmtime"] }
sc-executor = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false, features = ["wasmtime"] } # Or wasmer
sc-network = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sc-telemetry = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false, optional = true }
sc-transaction-pool = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sc-transaction-pool-api = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sc-consensus = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sc-consensus-babe = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sc-consensus-grandpa = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sc-rpc = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sc-rpc-api = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sc-basic-authorship = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sc-client-api = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }

# JSON-RPC related - ensure these match your node template if using a specific one
jsonrpsee = { version = "0.20.0", features = ["server"] } # Or a version compatible with Substrate's RPC setup

# CritterChain Runtime (Path relative to node crate)
critterchain-runtime = { path = "../runtime", default-features = false }

[build-dependencies]
substrate-build-script-utils = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0" }

[features]
default = ["std"]
std = [
    "critterchain-runtime/std",
    "sp-core/std",
    "sp-runtime/std",
    "sp-consensus-babe/std",
    "sp-consensus-grandpa/std",
    "sp-keystore/std",
    "sp-blockchain/std",
    "sp-api/std",
    "frame-benchmarking-cli/std",
    "sc-cli/std",
    "sc-service/std",
    "sc-executor/std",
    "sc-network/std",
    "sc-telemetry/std",
    "sc-transaction-pool/std",
    "sc-transaction-pool-api/std",
    "sc-consensus/std",
    "sc-consensus-babe/std",
    "sc-consensus-grandpa/std",
    "sc-rpc/std",
    "sc-rpc-api/std",
    "sc-basic-authorship/std",
    "sc-client-api/std",
    "jsonrpsee/server",
]
# "runtime-benchmarks" feature to enable benchmarking via CLI
runtime-benchmarks = ["critterchain-runtime/runtime-benchmarks", "frame-benchmarking-cli"]
```
*Note: Substrate git dependencies should ideally point to a specific commit hash or version tag for reproducible builds. `branch = "polkadot-v1.0.0"` is illustrative. `sc-cli` usually brings its own `clap` dependency; the top-level `clap` is for the node's own CLI arguments.*

## 2. Runtime `Cargo.toml` - Conceptual Dependencies Outline

The `runtime/Cargo.toml` file declares dependencies on all FRAME pallets used and Substrate primitives.

```toml
[package]
name = "critterchain-runtime"
version = "0.1.0"
authors = ["CritterCraft Developers"]
edition = "2021"

[dependencies]
# Substrate Primitives
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false, features = ["derive"] }
scale-info = { version = "2.5.0", default-features = false, features = ["derive"] } # Or compatible version
sp-api = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-core = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-runtime = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-std = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-io = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-version = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-consensus-babe = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-consensus-grandpa = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # Not directly used in runtime logic but good for type consistency
sp-session = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-staking = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
sp-offchain = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # For off-chain workers if any pallet uses them
sp-block-builder = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false } # Needed for `construct_runtime!`

# FRAME Pallets (System & Utility)
frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-support = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-timestamp = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-balances = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-transaction-payment = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-sudo = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }

# FRAME Pallets (Consensus & Staking)
pallet-authorship = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-babe = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-grandpa = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-session = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false, features = ["historical"] }
pallet-staking = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-im-online = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-offences = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
pallet-historical = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }
frame-election-provider-support = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", default-features = false }

# CritterCraft Custom Pallets (Paths relative to runtime crate, e.g., `../pallets/pallet-name`)
pallet-critter-nfts = { path = "../pallets/critter_nfts_pallet", default-features = false }
pallet-marketplace = { path = "../pallets/pallet-marketplace", default-features = false }
pallet-battles = { path = "../pallets/pallet-battles", default-features = false }
pallet-quests = { path = "../pallets/pallet-quests", default-features = false }
pallet-items = { path = "../pallets/pallet-items", default-features = false }
pallet-user-profile = { path = "../pallets/pallet-user-profile", default-features = false }
pallet-breeding = { path = "../pallets/pallet-breeding", default-features = false } # Conceptual
# pallet-daycare = { path = "../pallets/pallet-daycare", default-features = false } # Conceptual

[build-dependencies]
substrate-wasm-builder = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v1.0.0", optional = true } # Optional: only if building wasm with this runtime

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "sp-api/std",
    "sp-core/std",
    "sp-runtime/std",
    "sp-std/std",
    "sp-io/std",
    "sp-version/std",
    "sp-consensus-babe/std",
    "sp-consensus-grandpa/std",
    "sp-session/std",
    "sp-staking/std",
    "sp-offchain/std",
    "sp-block-builder/std",
    "frame-system/std",
    "frame-support/std",
    "pallet-timestamp/std",
    "pallet-balances/std",
    "pallet-transaction-payment/std",
    "pallet-sudo/std",
    "pallet-authorship/std",
    "pallet-babe/std",
    "pallet-grandpa/std",
    "pallet-session/std",
    "pallet-staking/std",
    "pallet-im-online/std",
    "pallet-offences/std",
    "pallet-historical/std",
    "frame-election-provider-support/std",
    "pallet-critter-nfts/std",
    "pallet-marketplace/std",
    "pallet-battles/std",
    "pallet-quests/std",
    "pallet-items/std",
    "pallet-user-profile/std",
    "pallet-breeding/std",
    # "pallet-daycare/std",
]
runtime-benchmarks = [
    "frame-system/runtime-benchmarks",
    "frame-support/runtime-benchmarks", # Needed for some benchmark macros
    "pallet-balances/runtime-benchmarks",
    "pallet-timestamp/runtime-benchmarks",
    "pallet-staking/runtime-benchmarks", # If you benchmark staking related operations
    # Add other pallets that have benchmarks
    "pallet-critter-nfts/runtime-benchmarks",
    # ... etc. for other custom pallets with benchmarks
]
```

## 3. Specific Staking Parameters (`pallet_staking::Config` in Runtime)

These are example values and rationale for key staking parameters. They would be defined using `parameter_types!` in `runtime/src/lib.rs`. Assume PTCN has 12 decimals. `pub const PTCN: Balance = 1_000_000_000_000;` (adjust if different decimal count). Type `Balance` is typically `u128`.

*   **`SessionsPerEra: sp_staking::SessionIndex = 6`** (e.g., if session is 1 hour, era is 6 hours)
    *   **Rationale:** Balances reward frequency and validator set stability. If a session is targeted for 1 hour (e.g., 600 blocks at 6s/block), an era of 6 hours allows for multiple reward cycles per day.
*   **`BondingDurationInEras: sp_staking::EraIndex = 28`** (e.g., 7 days if 1 era = 6 hours)
    *   **Rationale:** A common unbonding period. 28 eras * 6 hours/era = 168 hours = 7 days. Provides a balance between capital liquidity for stakers and network security (making it costly for malicious actors to quickly unbond).
*   **`SlashDeferDuration: sp_staking::EraIndex = BondingDurationInEras::get() / 4`** (i.e., 7 Eras in this example)
    *   **Rationale:** Allows a period for governance to review and potentially cancel slashes if they are deemed erroneous or due to exceptional circumstances, before they are fully applied. A fraction of the bonding duration is typical.
*   **`MaxNominations: u32 = 16`**
    *   **Rationale:** Standard Substrate value. Allows nominators to diversify their nominations across multiple validators, improving decentralization and risk management, without excessively complex election computations.
*   **`MaxNominatorRewardedPerValidator: u32 = 256`** (or 512)
    *   **Rationale:** Limits the number of nominators per validator that share in the rewards. This encourages nominators to seek out and support less saturated (but still performant) validators, further aiding stake decentralization.
*   **`MinimumValidatorCount: u32 = 5`** (Can be increased by governance)
    *   **Rationale:** Establishes a baseline for network decentralization and security, especially for a new network. Should be increased as the network matures.
*   **`MaxValidatorsCount: u32 = 75`** (A target for a maturing network)
    *   **Rationale:** Balances decentralization with network performance. A very large number of validators can increase inter-validator communication latency. This can be adjusted by governance.
*   **`MinNominatorBond: Balance = 100 * PTCN`** (e.g., 100 PTCN)
    *   **Rationale:** Sets a minimum stake for nominators to participate, preventing "dust" nominations and ensuring a basic level of economic commitment. Value depends on tokenomics.
*   **`MinValidatorBond: Balance = 1000 * PTCN`** (e.g., 1000 PTCN)
    *   **Rationale:** Ensures validators have a significant self-stake ("skin in the game"), aligning their incentives with the network's long-term health and security. Higher than the nominator bond.
*   **`OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17)`**
    *   **Rationale:** If more than this percentage of the active validator set commits an equivocation or other slashable offense in a single session, no rewards are paid out for that session to any validator. This discourages large-scale coordinated misbehavior. Standard Substrate value.
*   **`ElectionProviderMultiPhase`: `onchain::OnChainExecution<OnChainSeqPhragmen>`**
    *   **Rationale:** `OnChainExecution` with `OnChainSeqPhragmen` is the standard, well-tested on-chain election mechanism for Substrate, suitable for most networks. More advanced signed solutions exist but add complexity.
*   **`CurrencyToVote`: `()`** (Usually () to use active stake for voting weight)
    *   **Rationale:** Typically, staking uses the staked amount (active bond) as voting power in NPoS elections. This parameter allows for alternative "currency" representations for voting weight if needed, but `()` is standard.
*   **Reward Mechanism (Conceptual - related to how `pallet-staking` interacts with inflation):**
    *   **MVP Approach:** The simplest approach is for `pallet-staking` to distribute rewards from a pre-funded account or through a simple inflation model managed by `pallet-staking` itself (e.g., its `InflationConfig`). If `pallet-balances` handles inflation (e.g., by minting new tokens to an account that `pallet-staking` draws from), that's also viable.
    *   **Integration with Treasury:** For a more complete system, slashed funds and potentially unallocated reward portions (`RewardRemainder`) can be directed to `pallet-treasury` by implementing `Handle অফUnbalanced` for the `NegativeImbalance` and `PositiveImbalance` types. The Treasury can then fund staking rewards or other ecosystem initiatives via governance.
    *   A clear tokenomic model defining the source and rate of staking rewards (e.g., fixed annual inflation like 5-10% of total issuance, distributed per era based on staked points) is essential but designed outside the direct scope of these parameter settings.

These parameters provide a robust starting point for CritterChain's NPoS consensus and staking mechanism. They would be subject to review, simulation, and community governance as the network evolves.
```
