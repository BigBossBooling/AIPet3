# CritterCraft Activities Integration Guide

This guide explains how the new activities pallets integrate with the existing CritterCraft ecosystem to create engaging gameplay loops and economic opportunities.

## Overview

The CritterCraft activities framework consists of three main components:

1. **Mini-Games System** (`pallet-critter-minigames`): Provides solo skill-based activities for training specific pet stats
2. **Jobs System** (`pallet-critter-jobs`): Creates an economic engine where pets can earn BITS based on their stats
3. **Daycare System** (`pallet-critter-daycare`): Establishes a social hub where players can care for each other's pets

These components work together with the existing `pallet-critter-nfts` to create a comprehensive pet development ecosystem.

## Integration Architecture

### Core Integration Points

1. **Pet Stats & Experience**:
   - All activities affect pet stats and experience through the `NftManager` trait
   - Experience gained leads to level-ups, which improve pet capabilities

2. **Economic Loop**:
   - Mini-games and jobs provide BITS rewards
   - Daycare creates a player-to-player economy
   - All economic transactions use the `Currency` trait

3. **Social Interactions**:
   - Daycare system enables player-to-player interactions
   - Future two-player games will build on this foundation

### Trait Interfaces

The activities pallets interact with `pallet-critter-nfts` through these trait interfaces:

1. **`NftManagerForItems`**: Used by mini-games and jobs to:
   - Verify pet ownership
   - Add experience to pets
   - Update pet stats

2. **`NftManagerForDaycare`**: Used by the daycare system to:
   - Verify pet ownership
   - Update pet state based on caregiver interactions

## Implementation Details

### Mini-Games System

The mini-games pallet implements three core games:

1. **Logic Leaper (Intelligence Training)**:
   - Puzzle-based game that trains pet intelligence
   - Higher intelligence allows for more complex puzzles and greater rewards
   - Rewards include XP, BITS, and potential rare reagents

2. **Aura Weaving (Charisma Training)**:
   - Rhythm and pattern-matching game
   - Trains pet charisma for social interactions
   - Rewards include XP, BITS, and Aura Dust

3. **Habitat Dash (Energy/Agility Training)**:
   - "Endless runner" style game
   - Trains pet energy and agility
   - Rewards include XP, BITS, and environmental reagents

The actual gameplay happens off-chain, with only the results stored on-chain to minimize blockchain bloat.

### Jobs System

The jobs pallet implements three core jobs:

1. **Crystal Mining (Strength-based)**:
   - Requires high strength
   - Higher rewards but longer duration
   - Timing-based mini-game where stronger pets can break harder crystals

2. **Bioluminescent Guide (Charisma-based)**:
   - Passive job for high-charisma pets
   - Medium rewards with shorter duration
   - Higher charisma increases chance of bonus tips

3. **Herbalist Assistant (Intelligence-based)**:
   - Requires high intelligence
   - Highest rewards but requires specific pet traits
   - Puzzle/memory mini-game for identifying herbs

Jobs run for a specified duration (in blocks) before rewards can be claimed, creating a time-based economic loop.

### Daycare System

The daycare system implements "The Zoologist's Lodge":

1. **Daycare Creation**:
   - Players create daycares with custom names and fee structures
   - Daycares can host multiple pets up to a configurable limit

2. **Pet Listing**:
   - Owners list their pets when offline
   - Listed pets can be cared for by other players

3. **Caregiver Mechanics**:
   - Players become caregivers by paying a fee
   - Caregivers can interact with pets to improve their stats
   - Interactions include feeding, playing, and grooming

4. **Economic Model**:
   - Daycare owners earn fees from caregivers
   - Caregivers earn experience and reputation
   - Platform takes a small percentage of fees

## Usage Examples

### Example 1: Pet Development Loop

1. Player mints a new pet using `pallet-critter-nfts::mint_pet_nft`
2. Player trains pet's intelligence through Logic Leaper mini-game:
   ```rust
   pallet_critter_minigames::complete_logic_leaper(
       origin,
       pet_id,
       GameDifficulty::Medium,
       score
   )
   ```
3. Pet gains experience and levels up
4. With higher intelligence, pet qualifies for Herbalist Assistant job:
   ```rust
   pallet_critter_jobs::start_herbalist_assistant(
       origin,
       pet_id,
       duration_blocks
   )
   ```
5. After job completion, player claims rewards:
   ```rust
   pallet_critter_jobs::complete_job(
       origin,
       job_id
   )
   ```

### Example 2: Social Interaction Loop

1. Player creates a daycare:
   ```rust
   pallet_critter_daycare::create_daycare(
       origin,
       name,
       description,
       fee_per_block
   )
   ```
2. Another player lists their pet in the daycare:
   ```rust
   pallet_critter_daycare::list_pet(
       origin,
       pet_id,
       daycare_id
   )
   ```
3. A third player becomes a caregiver:
   ```rust
   pallet_critter_daycare::become_caregiver(
       origin,
       pet_id
   )
   ```
4. Caregiver interacts with the pet:
   ```rust
   pallet_critter_daycare::interact_with_pet(
       origin,
       pet_id,
       interaction_type
   )
   ```
5. Pet gains experience and temporary trait boosts
6. Caregiver stops caring for pet and pays fee to daycare owner:
   ```rust
   pallet_critter_daycare::stop_being_caregiver(
       origin,
       pet_id
   )
   ```

## Future Extensions

The activities framework is designed to be extensible:

1. **Two-Player Games**:
   - Critter Tactics (strategic duel)
   - Cooperative Crafting (synergy-based creation)

2. **Quest System**:
   - Standard quests (gathering, crafting, pacification)
   - Epic quests (multi-stage adventures)

3. **Advanced Jobs**:
   - Specialized professions based on pet traits
   - Guild-based collaborative jobs

## Conclusion

The CritterCraft activities framework creates a comprehensive ecosystem for pet development, economic engagement, and social interaction. By leveraging the existing pet NFT system and extending it with mini-games, jobs, and daycare mechanics, the framework provides multiple engaging gameplay loops that work together to create a rich and rewarding player experience.