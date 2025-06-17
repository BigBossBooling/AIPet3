# CritterCraft: Off-Chain Battle Simulation Engine Specification

This document provides the specification for the off-chain battle simulation engine used by CritterCraft. This engine is responsible for determining the outcome of Pet NFT battles based on their on-chain attributes. The results are then reported back to `pallet-battles` on CritterChain.

## 1. Overview and Purpose

The off-chain battle simulation engine handles the computationally intensive aspects of pet battles, allowing for complex and engaging combat without overburdening the blockchain. It ensures that battles are resolved based on defined rules and pet characteristics, with the outcome being verifiably recorded on-chain. For MVP, the logic is simplified, focusing on core mechanics.

## 2. Inputs to the Battle Engine

The engine expects the following data for each battle simulation:

*   **`battle_id: BattleId`**: The unique identifier of the battle instance from `pallet-battles`.
*   **`pet1_stats: MvpBattlePetStats`**: Comprehensive stats for the first participating pet.
*   **`pet2_stats: MvpBattlePetStats`**: Comprehensive stats for the second participating pet.
    *   The `MvpBattlePetStats` struct (conceptually defined for the simulation) should include:
        *   `pet_id: PetId` (on-chain ID)
        *   `owner: AccountId`
        *   `level: u32`
        *   `base_strength: u8`
        *   `base_agility: u8`
        *   `base_vitality: u8`
        *   `primary_elemental_affinity: Option<ElementType>` (where `ElementType` is defined in `pallet-critter-nfts`)
        *   *(Optional MVP addition) current_mood: u8` (0-100, could give minor +/- 5% combat effectiveness, fetched from `PetNft.mood_indicator`)*
*   **`random_seed: Option<[u8; 32]>`**:
    *   An optional shared random seed. If provided, the battle simulation MUST be deterministic.
    *   If `None`, the engine may use its own internal randomness, but the simulation might not be easily verifiable by third parties running the same inputs. For MVP and simplicity of reporting, the on-chain `report_battle_outcome` might not require this seed, but the engine should ideally support it for future verifiability.

This data would be fetched from `pallet-critter-nfts` (via `NftHandler` or direct queries) by the entity triggering the simulation (e.g., a player's client initiating a P2P battle, or a centralized service for automated battles, or an oracle).

## 3. Core Logic / Algorithms (Conceptual - MVP Focus)

The simulation proceeds in turns until a win condition is met.

### a. Initialization
1.  **Calculate Effective Combat Stats for each pet:**
    *   **Effective HP (EHP):** `(pet_stats.base_vitality * VITALITY_TO_HP_FACTOR_CONST) + (pet_stats.level * LEVEL_TO_HP_FACTOR_CONST)`
        *   *Example: `VITALITY_TO_HP_FACTOR_CONST = 10`, `LEVEL_TO_HP_FACTOR_CONST = 5`*
    *   **Effective Attack (EATK):** `(pet_stats.base_strength * STRENGTH_TO_ATK_FACTOR_CONST) + (pet_stats.level * LEVEL_TO_ATK_FACTOR_CONST)`
    *   **Effective Defense (EDEF):** `(pet_stats.base_vitality * VITALITY_TO_DEF_FACTOR_CONST) + (pet_stats.level * LEVEL_TO_DEF_FACTOR_CONST)` (Vitality contributes to both HP and Defense)
    *   **Effective Speed (ESPD):** `(pet_stats.base_agility * AGILITY_TO_SPD_FACTOR_CONST) + (pet_stats.level * LEVEL_TO_SPD_FACTOR_CONST)`
    *   *(All `_CONST` values are part of game balancing and defined within the engine).*
    *   *(Optional MVP Mood Influence): If `current_mood` is provided and used: EATK and EDEF might be adjusted by +/- 5% if mood is very high (e.g., >80) or very low (e.g., <20).*
2.  Set `pet1_current_hp = pet1_EHP`, `pet2_current_hp = pet2_EHP`.

### b. Turn Management
1.  **Determine Attack Order:** The pet with higher `ESPD` attacks first in each round. If `ESPD` is equal, use `random_seed` (if provided, e.g., by hashing seed + current turn number + pet IDs to pick one) to break ties deterministically, or default to Pet1 if no seed.
2.  **Maximum Turns:** A battle will have a maximum number of turns (e.g., `MAX_BATTLE_TURNS_CONST = 50`) to prevent indefinite loops.

### c. Action Phase (per turn)
For MVP, each pet performs a basic "Attack" action.

1.  **Attacker:** The pet whose turn it is.
2.  **Defender:** The opposing pet.
3.  **Hit Chance Calculation (Simplified MVP):**
    *   `BASE_HIT_CHANCE_PERCENT = 85` (Constant within the engine)
    *   Generate a random number `roll_hit` (0-99) using `random_seed` (or internal RNG if seed is `None`). If using seed, derive turn-specific randomness (e.g., hash seed + turn number).
    *   If `roll_hit < BASE_HIT_CHANCE_PERCENT`, the attack hits. Otherwise, it's a "Miss."
4.  **Damage Calculation (If Hit - Simplified MVP):**
    *   `base_damage = Attacker_EATK`
    *   `mitigation = Defender_EDEF / DEFENSE_MITIGATION_FACTOR_CONST` (e.g., `DEFENSE_MITIGATION_FACTOR_CONST = 2`)
    *   `calculated_damage = base_damage.saturating_sub(mitigation)`
    *   `actual_damage = calculated_damage.max(MINIMUM_DAMAGE_CONST)` (e.g., `MINIMUM_DAMAGE_CONST = 1`)
5.  **Elemental Modifier Application:**
    *   Fetch `attacker_pet_stats.primary_elemental_affinity` and `defender_pet_stats.primary_elemental_affinity`.
    *   Apply a multiplier to `actual_damage` based on a predefined elemental matchup table (e.g., Fire vs. Nature: 1.5x; Fire vs. Water: 0.75x; Neutral vs. Any: 1.0x; Same Element vs. Same: 0.9x or 1.0x). The engine defines this table.
    *   `final_damage = (actual_damage * elemental_multiplier) as u32` (ensure rounding or consistent conversion).
6.  **Apply Damage:**
    *   `defender_current_hp = defender_current_hp.saturating_sub(final_damage)`.
7.  **Check for Win Condition:**
    *   If `defender_current_hp == 0`, the Attacker is the winner. The battle ends.

### d. Winning/Losing Conditions
1.  A pet wins if the opponent's `current_hp` reaches 0.
2.  If `MAX_BATTLE_TURNS_CONST` is reached:
    *   The pet with the higher remaining HP percentage (`current_hp / EHP`) wins.
    *   If HP percentages are equal, the pet with higher initial total effective stats (EHP+EATK+EDEF+ESPD) might win.
    *   As a final tie-breaker if still equal (or if preferred for simplicity): use `random_seed` (if provided, e.g., hash seed + "TIE_BREAKER" to pick Pet1 or Pet2) or default to Pet1 as the winner. For MVP, `report_battle_outcome` expects a single winner.

## 4. Outputs from the Battle Engine

For MVP, the engine MUST output:
*   **`winner_pet_id: PetId`**: The `PetId` of the winning pet.

Future (Post-MVP) outputs could include:
*   `loser_pet_id: PetId`
*   `battle_log_hash: [u8; 32]` (A hash of the detailed battle log).
*   `turns_taken: u32`
*   `damage_dealt_by_winner: u32`
*   `damage_dealt_by_loser: u32`

## 5. Battle Log Structure (Conceptual)

The engine should generate a detailed, structured log of the battle for transparency and potential off-chain verification or display. This log is not stored on-chain for MVP.

Example Log Entry (per turn/action):
```json
{
  "turn": 1,
  "attacker_pet_id": "PET_ID_1", // Actual PetId
  "defender_pet_id": "PET_ID_2", // Actual PetId
  "action_type": "BasicAttack", // Could be "Ability: 'Fireball'" in future
  "random_roll_hit": 75, // (0-99)
  "hit_chance_threshold": 85,
  "did_hit": true,
  "calculated_damage_pre_modifier": 15, // EATK - (EDEF / Factor)
  "elemental_modifier_applied": 1.5, // Multiplier value
  "final_damage_dealt": 22,
  "defender_hp_before_damage": 150,
  "defender_hp_after_damage": 128,
  "status_effects_applied": [] // For future, e.g., ["Burned"]
}
```
The full log would be an array of such entries, plus initial pet stats (EHP, EATK etc.) and the final outcome summary.

## 6. Security & Verifiability Considerations

*   **Determinism (with Seed):** If a `random_seed` is provided as input, the entire battle simulation MUST be deterministic. This means given the same set of pet stats and the same seed, the outcome will always be the same. This is crucial for any future system allowing players to verify battle outcomes by re-running the simulation with public data.
*   **Open Source Logic (Recommended):** Making the battle simulation engine's core logic open source would allow the community to inspect and verify its fairness.
*   **`battle_log_hash` (Future):** Submitting a hash of the detailed battle log to `pallet-battles` would allow for challenges or dispute resolution if the reported outcome is questioned, assuming players have access to the inputs and the simulation logic.
*   **Oracle Trust (MVP):** For MVP, if an oracle or a single player (e.g., player1 as per `pallet-battles` MVP) reports the outcome, there's an element of trust. The system design acknowledges this and should aim for more robust verification (e.g., dual reporting, log hash verification) in later stages.

## 7. Interaction with `pallet-battles`

The off-chain engine (or the client/server operating it) is responsible for:
1.  Gathering input data for the participating pets (likely via RPC calls to the CritterChain node to query `pallet-critter-nfts`).
2.  Running the simulation as described above.
3.  Submitting the `winner_pet_id` (and other relevant data post-MVP) by calling the `report_battle_outcome` extrinsic on `pallet-battles`.

This specification provides a clear guideline for an MVP off-chain battle engine, focusing on core mechanics while allowing for future expansion.
```
