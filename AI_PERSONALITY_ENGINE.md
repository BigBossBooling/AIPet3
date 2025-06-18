# CritterCraft: AI Personality Engine (Conceptual Specification)

This document outlines the conceptual specification for an off-chain AI Personality Engine designed to drive the dynamic evolution of Pet NFT personalities within CritterCraft.

## 1. Overview and Purpose

The AI Personality Engine aims to make Pet NFTs feel more alive and responsive to their experiences on CritterChain. Instead of personality traits being static or only changed by direct item use, this engine will observe a pet's history and interactions to suggest or trigger nuanced personality developments, reflecting their unique journey.

This engine operates primarily **off-chain**, processing on-chain data and then interfacing with `pallet-critter-nfts` to apply changes, ideally through owner-approved suggestions for MVP.

## 2. Inputs to the AI Personality Engine

For each Pet NFT, the engine would ideally consume and analyze:

*   **Static Pet Data (from `PetNft` struct in `pallet-critter-nfts`):**
    *   `id: PetId`
    *   `dna_hash: [u8; 16]` (Might influence predispositions to certain traits)
    *   `initial_species: Vec<u8>` (Species might have typical personality leanings)
    *   Existing `personality_traits: BoundedVec<BoundedVec<u8, MaxTraitLen>, MaxTraits>` (MaxTraitLen and MaxTraits from Config)
    *   Charter Attributes (`base_strength`, `base_agility`, etc.)
*   **Dynamic Pet Data (from `PetNft` struct):**
    *   `level: u32`
    *   `mood_indicator: u8` (Frequent low mood might lead to negative traits)
    *   Interaction Timestamps: `last_fed_block`, `last_played_block`, `last_state_update_block`.
*   **Historical On-Chain Event Data (Associated with the Pet NFT):**
    *   **Care Interactions:** Frequency and type of items used in `feed_pet` and `play_with_pet` calls (from `pallet-critter-nfts` events; implies item data from `pallet-items` might be referenced by the engine if item effects on personality are complex).
    *   **Battle History (`pallet-battles` events):**
        *   Number of battles fought, win/loss ratio.
        *   Types of opponents faced (e.g., relative strength/level).
        *   Specific battle events (e.g., "survived a battle with 1 HP," "achieved a critical hit at a key moment").
    *   **Quest History (`pallet-quests` events):**
        *   Types of quests completed (e.g., "exploration," "charity," "difficult combat").
        *   Choices made within quests (if quests had branching narratives recorded on-chain - a future concept).
    *   **Item Usage (Specific Items from `pallet-items` events via `NftManagerForItems`):**
        *   Use of specific mood-altering items or items explicitly designed to influence personality (beyond basic care).
    *   **Day Care / Social Interactions (Future `pallet-daycare` events):**
        *   Time spent in day care, type of care received, interactions with specific caregiver pets.
    *   **Breeding History (Future `pallet-breeding` events):**
        *   Number of offspring, success/traits of offspring (if this data influences parent's "legacy" or "nurturing" traits).

## 3. Core Logic Principles (Conceptual "Black Box")

The internal AI/heuristic algorithms are not specified here, but would adhere to principles like:

*   **Pattern Recognition:** Identifying recurring behaviors or experiences (e.g., pet frequently wins battles -> suggest "Brave"; pet often neglected -> suggest "Solitary" or "Resentful").
*   **Weighted Influences:** Different events have different impacts. A major battle win might weigh more than a single feeding.
*   **Thresholds & Cooldowns:** Traits might only emerge after certain event counts or state durations. Changes shouldn't be too frequent.
*   **Positive & Negative Development:** Based on a spectrum of interactions (e.g., neglect vs. consistent care; aggressive battle style vs. defensive).
*   **Trait Synergy/Conflict/Slots:** The engine would respect `MaxPetPersonalityTraits`. It might try to replace a conflicting trait or a less dominant one if a new, stronger trait emerges.
*   **Gradual Evolution:** Personality changes should feel earned and gradual.
*   **Species/DNA Predispositions:** `initial_species` or patterns in `dna_hash` could make certain traits more or less likely to develop.

## 4. Outputs from the AI Personality Engine

The engine's primary output would be **suggestions for changes** to a Pet NFT's `personality_traits` list:
*   **Add Trait:** Suggest a new trait (e.g., `Vec<u8>` string like "Brave").
*   **Remove Trait:** Suggest removal of an existing trait (e.g., if a "Timid" pet performs many brave actions, "Timid" might be suggested for removal if "Brave" is added).
*   **(Future) Modify Trait Intensity:** If traits were structs with intensity values, suggestions could modify these. (For MVP, traits are binary).

## 5. Integration with CritterChain (`pallet-critter-nfts`) - MVP Approach

*   **Owner-Approved Suggestions (Preferred for MVP):**
    *   The off-chain AI Personality Engine (run as a community service, by individual users via a companion tool, or by a project-maintained service) processes on-chain data and generates personality change suggestions for specific pets.
    *   These suggestions are presented to the pet owner (e.g., in the UI Wallet, via an inbox, or a companion app).
    *   The owner reviews the suggested changes (e.g., "Your pet's recent victories suggest it might develop the 'Brave' trait. Add it?").
    *   If the owner accepts, they initiate an on-chain transaction by calling the existing **`update_pet_metadata`** extrinsic in `pallet-critter-nfts`. They would provide the full new list of `personality_traits` (their existing traits plus or minus the suggested changes).
    *   **Pros:** Maximizes player agency; uses existing, simpler on-chain extrinsic; reduces on-chain complexity for personality evolution.
    *   **Cons:** Relies on owner action to apply changes; off-chain component needed to generate suggestions.

*   **Oracle-Driven Updates (Post-MVP Consideration):**
    *   A trusted oracle could call a specialized extrinsic (e.g., `oracle_update_personality_traits`) to directly apply AI-determined trait changes. This is more automated but less player-centric for personality.

## 6. Data Availability for Off-Chain Engine

*   Efficient access to historical on-chain event data is crucial. This would likely require:
    *   CritterChain archive nodes.
    *   Specialized blockchain indexers (e.g., SubQuery, Subsquid projects) to provide structured, queryable APIs for pet histories.

This AI Personality Engine is key to making pets feel dynamic and truly reflective of their unique experiences within CritterCraft.
```
