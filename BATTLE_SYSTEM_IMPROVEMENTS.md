# Battle System Improvements

This document outlines the significant improvements made to the CritterCraft Battle System.

## Overview

The battle system has been dramatically enhanced with the following key features:

1. **Advanced Battle Mechanics**
   - Status effects system
   - Energy management
   - Combo system
   - Ultimate moves
   - Battle history tracking

2. **Matchmaking System**
   - Rating-based matchmaking
   - Queue management
   - Automatic battle creation

3. **Battle Statistics**
   - Win/loss tracking
   - Rating adjustments
   - Performance metrics

## 1. Advanced Battle Mechanics

### Status Effects

The battle system now includes a comprehensive status effect system that adds strategic depth to battles:

- **Burn**: Deals damage over time
- **Freeze**: Chance to skip turns
- **Poison**: Deals damage over time
- **Stun**: Chance to skip turns
- **Strengthen**: Increases attack power
- **Shield**: Reduces incoming damage

Status effects have a duration and are processed each turn. They can be applied through special moves and automatically expire when their duration ends.

### Energy Management

A new energy system has been implemented:

- Each pet has an energy meter (0-100)
- Energy regenerates each turn
- Different moves cost different amounts of energy
- Ultimate moves require significant energy
- Strategic energy management is key to victory

### Combo System

The combo system rewards consecutive successful attacks:

- Consecutive hits increase the combo counter
- When the combo threshold is reached, a bonus is applied
- Combo bonuses increase damage output
- Combos are reset when a move misses or the pet takes damage

### Ultimate Moves

Ultimate moves are powerful abilities that can turn the tide of battle:

- Require significant energy to use
- Deal high damage based on both strength and intelligence
- Cannot be used when affected by certain status effects
- Create dramatic moments in battles

### Battle History

A comprehensive battle history system has been implemented:

- Each move is recorded with its result
- Battle history can be queried by players
- Provides transparency and allows for analysis
- Useful for improving battle strategies

## 2. Matchmaking System

### Rating-Based Matchmaking

The matchmaking system pairs pets based on their battle rating:

- Each pet has a rating (starting at 1000)
- Ratings change based on battle outcomes
- Pets are matched with others of similar rating
- Prevents unfair matchups between beginners and veterans

### Queue Management

The queue system efficiently manages pets waiting for battles:

- Pets can enter and leave the queue
- Queue is processed periodically
- Oldest entries are prioritized
- Rating differences are considered when matching

### Automatic Battle Creation

When suitable matches are found, battles are automatically created:

- No need for manual challenge acceptance
- Battles start immediately
- Both participants are notified
- Reduces waiting time for players

## 3. Battle Statistics

### Win/Loss Tracking

Comprehensive statistics are maintained for each pet:

- Wins, losses, and draws are tracked
- Statistics are stored on-chain
- Players can view their pets' performance
- Encourages competitive play

### Rating Adjustments

Ratings are adjusted after each battle:

- Winners gain rating points
- Losers lose rating points
- Amount of change depends on the rating difference
- Minimum rating floor prevents excessive penalties

### Performance Metrics

Additional metrics provide insights into battle performance:

- Battle rating shows overall skill level
- Win percentage indicates success rate
- Battle history shows improvement over time
- Helps players identify strengths and weaknesses

## Technical Implementation

### New Data Structures

Several new data structures have been implemented:

```rust
// Battle move result enum
pub enum BattleMoveResult {
    Hit(u8),           // Damage dealt
    Miss,              // Attack missed
    Critical(u8),      // Critical hit with damage
    Heal(u8),          // Health restored
    StatusApplied(u8), // Status effect applied with ID
    Combo(u8, u8),     // Combo hits and total damage
}

// Status effect enum
pub enum StatusEffect {
    Burn(u8),      // Damage per turn, remaining turns
    Freeze(u8),    // Skip turns chance, remaining turns
    Poison(u8),    // Damage per turn, remaining turns
    Stun(u8),      // Skip turn chance, remaining turns
    Strengthen(u8), // Attack boost, remaining turns
    Shield(u8),    // Damage reduction, remaining turns
}

// Battle move history entry
pub struct BattleMoveHistoryEntry {
    pub turn: u8,
    pub pet_id: PetId,
    pub move_type: BattleMove,
    pub result: BattleMoveResult,
}
```

### Enhanced Battle Parameters

Battle parameters have been expanded to include:

```rust
pub struct BattleParameters<Balance> {
    pub challenge_bond: Balance,
    pub forfeit_penalty: Balance,
    pub base_reward: Balance,
    pub challenge_expiry_blocks: u32,
    pub max_turns: u8,
    pub base_experience_reward: u32,
    pub elemental_advantage_multiplier: Perbill,
    pub critical_hit_chance: Perbill,
    pub critical_hit_multiplier: Perbill,
    pub combo_threshold: u8,
    pub combo_bonus_multiplier: Perbill,
    pub status_effect_duration: u8,
    pub initial_energy: u8,
    pub energy_per_turn: u8,
    pub ultimate_move_energy_cost: u8,
    pub matchmaking_rating_change: u16,
}
```

### New Storage Items

Additional storage items track the enhanced battle system:

```rust
// Battle history storage
pub type BattleHistory<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    BattleId,
    BoundedVec<BattleMoveHistoryEntry, ConstU32<100>>,
    ValueQuery,
>;

// Pet battle stats storage
pub type PetBattleStats<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    PetId,
    (u32, u32, u32, u16), // (wins, losses, draws, rating)
    ValueQuery,
>;

// Matchmaking queue storage
pub type BattleMatchmakingQueue<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    PetId,
    (T::AccountId, u16, T::BlockNumber), // (owner, rating, enqueue_time)
    OptionQuery,
>;
```

## Future Enhancements

The improved battle system provides a foundation for further enhancements:

1. **Team Battles**: Allow multiple pets per side in battles
2. **Battle Items**: Consumable items that can be used during battles
3. **Spectator Mode**: Allow other players to watch ongoing battles
4. **Battle Replays**: Store and replay past battles
5. **Ranked Ladder System**: Competitive ranking system with seasons
6. **Battle Abilities**: Pet-specific special abilities based on species and level

## Conclusion

These improvements transform the CritterCraft Battle System into a sophisticated, strategic gameplay element that adds significant depth to the pet ecosystem. The enhanced mechanics, matchmaking system, and statistics tracking create a more engaging and competitive experience for players.