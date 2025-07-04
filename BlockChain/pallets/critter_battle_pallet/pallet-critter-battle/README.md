# CritterCraft Battle Pallet

A comprehensive battle system for the CritterCraft ecosystem that enables pet vs. pet battles with strategic gameplay mechanics.

## Overview

The Battle Pallet provides a turn-based battle system for CritterCraft pets, allowing players to challenge each other, execute strategic moves, and earn rewards based on battle outcomes. The pallet also includes a tournament system for organized competitive play.

## Features

### Core Battle Mechanics

- **Challenge System**: Players can challenge other players' pets to battles
- **Turn-Based Combat**: Alternating turns with strategic move selection
- **Battle Moves**: Six different move types with unique effects:
  - Attack: Basic damage move
  - Defend: Recover health
  - Special Attack: High damage with chance to miss
  - Heal: Significant health recovery
  - Dodge: Small heal with increased chance to avoid next attack
  - Elemental Attack: Damage based on elemental advantages

### Elemental Advantage System

The battle system includes an elemental advantage mechanic where certain elements are strong against others:
- Fire > Earth > Water > Fire
- Air > Tech > Nature > Air
- Mystic and Neutral have no advantages or disadvantages

### Battle Outcomes and Rewards

- **Experience Rewards**: Pets earn experience based on battle performance
- **Currency Rewards**: Winners receive PTCN tokens as rewards
- **Forfeit Penalties**: Players who forfeit battles lose a portion of their staked tokens

### Tournament System

- **Tournament Creation**: Admins can create tournaments with custom parameters
- **Registration Phase**: Players can enter their pets into tournaments
- **Tournament Brackets**: Automatic bracket generation and progression
- **Prize Pools**: Entry fees contribute to tournament prize pools
- **Level-Based Divisions**: Tournaments can be restricted to specific pet level ranges

## Usage

### Battle Challenges

```rust
// Challenge another pet to a battle
battle.create_challenge(pet_id, target_pet_id)

// Accept a battle challenge
battle.accept_challenge(battle_id)

// Decline a battle challenge
battle.decline_challenge(battle_id)
```

### Battle Execution

```rust
// Execute a battle move
battle.execute_move(battle_id, BattleMove::Attack)
battle.execute_move(battle_id, BattleMove::Defend)
battle.execute_move(battle_id, BattleMove::SpecialAttack)
battle.execute_move(battle_id, BattleMove::Heal)
battle.execute_move(battle_id, BattleMove::Dodge)
battle.execute_move(battle_id, BattleMove::ElementalAttack)

// Forfeit a battle
battle.forfeit_battle(battle_id)

// Claim battle rewards
battle.claim_rewards(battle_id)
```

### Tournament Participation

```rust
// Enter a tournament
battle.enter_tournament(tournament_id, pet_id)

// Create a tournament (admin only)
battle.create_tournament(name, description, max_participants, min_pet_level, max_pet_level, entry_fee, start_block)

// Set battle parameters (admin only)
battle.set_battle_params(params)
```

## Integration with Other Pallets

The Battle Pallet integrates with several other pallets in the CritterCraft ecosystem:

- **NFT Pallet**: For pet ownership verification and stats
- **Currency Pallet**: For battle bonds, rewards, and tournament entry fees
- **Randomness Source**: For battle move outcomes and critical hits
- **Pet Management**: For experience rewards and level progression

## Technical Implementation

The pallet uses several key Substrate features:

- **StorageMap**: For storing battle and tournament data
- **StorageDoubleMap**: For tournament participants
- **Randomness Trait**: For battle move outcomes
- **Currency Trait**: For handling tokens
- **Events**: For notifying clients of battle actions and outcomes
- **Hooks**: For checking expired challenges and starting tournaments

## Future Enhancements

- **Team Battles**: Allow multiple pets per side in battles
- **Battle Items**: Consumable items that can be used during battles
- **Spectator Mode**: Allow other players to watch ongoing battles
- **Battle Replays**: Store and replay past battles
- **Ranked Ladder System**: Competitive ranking system with seasons
- **Battle Abilities**: Pet-specific special abilities based on species and level