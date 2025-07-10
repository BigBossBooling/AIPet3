# CritterCraftUniverse: Blockchain Integration

## 1. Core Blockchain (EmPower1 Blockchain)

* **Layer 1:** Utilizes the **EmPower1 Blockchain** as the primary ledger for all critical on-chain transactions.
* **Consensus:** Leverages EmPower1's Proof-of-Stake (PoS) mechanism for block production and network security.
* **Native Token:** Uses $QRASL as the native currency for all economic interactions.

## 2. Decentralized Identity (DigiSocialBlock DID)

* **User Identity:** Pet owners' identities are managed via **Decentralized Identifiers (DIDs)** from **DigiSocialBlock (Nexus Protocol)**.
* **Verifiable Ownership:** DIDs link users to their owned CritterCraftUniverse NFTs and on-chain activities.

## 3. Decentralized Storage (DigiSocialBlock DDS)

* **Asset Storage:** Game assets (sprites, models, ability animations, map layouts) are stored on **DigiSocialBlock's DDS** (Distributed Data Storage).
* **Metadata Storage:** Dynamic pet metadata (e.g., detailed personality profiles, training logs) can be stored on DDS, with CIDs referenced in pet NFTs.
* **Censorship Resistance:** Ensures game assets and pet data are decentralized and resilient.

## 4. On-Chain Assets (NFTs)

* **Pet NFTs:** Each CritterCraftUniverse pet is a unique Non-Fungible Token (NFT) on the EmPower1 Blockchain.
* **NFT Metadata:** Pet stats, traits, abilities, and progression are reflected in the NFT's on-chain metadata, updated via smart contracts.
* **Ownership & Transfer:** NFTs enable verifiable ownership and secure transfer of pets between users.

## 5. Smart Contract Interactions (Substrate Pallets)

* **Framework:** Game logic, governance rules, and economic mechanics are implemented as **Substrate pallets** (Rust smart contracts).
* **Key Pallets:**
    * `pallet-battles`: Manages Critter Tactics game state, move validation, and outcome.
    * `pallet-contracts`: Manages Caregiver Contracts, task verification, and reward distribution.
    * `pallet-economy`: Handles $QRASL token transfers, fees, staking, and rewards.
    * `pallet-governance`: Implements Zoologist's Guild DAO rules, voting, and proposal management.
    * `pallet-reputation`: Manages the On-Chain Reputation System.
    * `pallet-moderation`: Implements Moderation Council rules and consequences.
    * `pallet-dispute`: Implements Marketplace Dispute Resolution.
* **On-Chain State:** Only critical, minimal game/system state is stored on-chain for verifiability.

## 6. Tokenomics Integration ($QRASL)

* **Native Currency:** $QRASL is the primary medium of exchange, entry fees, and rewards.
* **Staking:** $QRASL is staked for validator roles, governance participation, and Council eligibility.
* **Treasury:** Community-governed Treasury funded by protocol fees and a portion of rewards.

## 7. Verifiable Credentials (VCs)

* **Ethics Training VCs:** On-chain VCs issued for completing Ethics Training Modules (Moderation, Marketplace), linking to DID and boosting Reputation.

## 8. Cross-Chain Interoperability (Future)

* Conceptual plans for bridging $QRASL and NFTs to other blockchain networks.
