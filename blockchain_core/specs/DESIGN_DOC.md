# CritterCraft Design Document

## 1. Overview

This document outlines the technical design for CritterCraft, focusing on its blockchain components. It details the CritterChain, PetCoin (PTCN), and Pet NFTs.

## 2. CritterChain

*   **Blockchain Platform**: Substrate
*   **Consensus Mechanism**: Proof-of-Authority (PoA) for initial development phases. This allows for controlled network management and rapid block production. Future iterations may explore PoS/DPoS.
*   **Network Name**: CritterChain
*   **Modularity**: Designed to support custom pallets for core game logic (e.g., NFTs, battles, quests).

## 3. PetCoin (PTCN)

*   **Token Symbol**: PTCN
*   **Total Initial Supply**: 1,000,000,000 (1 Billion)
*   **Decimal Places**: 18
*   **Type**: Native Utility and Governance Token
*   **Utility**:
    *   Primary currency for transactions within the CritterCraft ecosystem (e.g., NFT marketplace, item purchases).
    *   Rewards for quests, battles, and other in-game activities.
    *   Staking (in future PoS/DPoS phases).
*   **Governance**:
    *   Used for voting on proposals related to game development, economic parameters, and network upgrades.

## 4. Pet NFTs

*   **Standard**: To be evaluated (e.g., ERC721-like if bridging, or a custom Substrate pallet mimicking its unique ownership and metadata features). The custom pallet (`critter_nfts_pallet`) will handle this.
*   **Core Concept**: Each AI pet in CritterCraft will be represented as a unique NFT on CritterChain, ensuring true digital ownership.

*   **Immutable Attributes (Set at Minting)**:
    *   `nft_id`: Unique identifier for the NFT.
    *   `dna_hash`: A hash representing the pet's foundational genetic code, influencing potential traits and appearance (future feature).
    *   `initial_species`: The base species of the pet (e.g., "AquaSprite", "TerraPup").
    *   `mint_date`: Timestamp of when the pet NFT was created.

*   **Mutable Attributes (Dynamic & Updatable On-Chain or via Off-Chain Logic with On-Chain Hashing/Anchoring)**:
    *   `current_pet_name`: The user-given name for the pet (can be different from any initial name or NFT title).
    *   `level`: Current level of the pet, increased through training and interaction.
    *   `experience_points`: Points accumulated towards the next level.
    *   `mood_indicator`: A numerical or state representation of the pet's current mood (e.g., 0=Sad, 1=Neutral, 2=Happy, 3=Playful). Influenced by hunger, energy, interaction.
    *   `hunger_status`: Numerical value representing hunger.
    *   `energy_status`: Numerical value representing energy.
    *   `personality_traits`: Array or struct of developed personality aspects (e.g., "Brave," "Shy," "Curious"). These can evolve based on interactions and experiences.
    *   `skill_points`: Points available to allocate to specific pet skills.
    *   `cosmetic_traits`: Applied visual customizations (e.g., color patterns, accessories).

*   **Minting Process**:
    *   Triggered when a new pet is "born" or "activated" in the game via the Python MVP's logic.
    *   The core Python application will communicate with a service that interacts with the CritterChain to request the minting of a new Pet NFT.
    *   Initial attributes (immutable and some mutable defaults) are set during minting.

*   **Metadata Storage Strategy**:
    *   **Immutable Attributes**: Stored directly on-chain within the NFT pallet.
    *   **Mutable Attributes**:
        *   Option 1 (Full On-Chain): Store all mutable attributes directly on-chain. Can be gas-intensive for frequent updates.
        *   Option 2 (Hybrid - Preferred for MVP): Store critical mutable attributes (e.g., level, core stats) on-chain. Less frequently changing or large metadata (e.g., detailed appearance data, long description) can be stored off-chain (e.g., IPFS or a dedicated server), with a hash of this metadata stored on-chain with the NFT to ensure integrity. Updates to off-chain data would mean updating the on-chain hash.
        *   For Stage 2 (this stage), we will aim for critical mutable attributes on-chain, with the *concept* of off-chain storage for future expansion. The `tick()` method from the Python MVP provides the off-chain logic that would necessitate these updates.

## 5. Future Considerations (Post Stage 2)
*   Wallet UI for NFT and PTCN management.
*   Marketplace for Pet NFTs.
*   Battle and Quest logic integration with NFT attributes.
