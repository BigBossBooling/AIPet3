# Critter Mini-Games Pallet

This pallet manages mini-games and activities for CritterCraft pets. It defines the game types, rewards, and interactions that drive pet development through engaging gameplay loops.

## Overview

The Critter Mini-Games pallet provides the on-chain infrastructure for various mini-games and activities in the CritterCraft ecosystem. These activities allow players to train their pets' stats, earn rewards, and deepen their bond with their digital companions.

## Features

- **Multiple Game Types**: Supports various mini-games focused on different pet stats:
  - **Logic Leaper**: Intelligence training
  - **Aura Weaving**: Charisma training
  - **Habitat Dash**: Energy/Agility training
  - **Critter Tactics**: Strategic duels (2-player)
  - **Cooperative Crafting**: Synergy-based creation (2-player)

- **Difficulty Levels**: Each game can be played at different difficulty levels, affecting rewards:
  - Easy
  - Medium
  - Hard
  - Expert

- **Reward System**: Players earn both experience points for their pets and BITS currency based on:
  - Game type
  - Difficulty level
  - Score achieved

- **Game History**: Tracks a pet's participation in games, creating a record of achievements

## Integration with Other Pallets

This pallet integrates with:

- **pallet-critter-nfts**: For pet ownership verification and experience updates
- **pallet-balances**: For BITS currency rewards

## Extrinsics

- `create_game`: Start a new mini-game instance
- `complete_game`: Finish a game and claim rewards
- `abandon_game`: Quit a game without claiming rewards
- `complete_logic_leaper`: Convenience function for the Logic Leaper mini-game
- `complete_aura_weaving`: Convenience function for the Aura Weaving mini-game
- `complete_habitat_dash`: Convenience function for the Habitat Dash mini-game

## Game Mechanics

Each mini-game is designed to train specific pet attributes:

1. **Logic Leaper (IQ & Focus Training)**:
   - A puzzle game where players guide their critter across a grid to a goal
   - Higher pet intelligence allows for more complex puzzles and greater rewards
   - Rewards include IQ experience, BITS, and rare reagents

2. **Aura Weaving (Charisma & Rhythm Training)**:
   - A rhythm and pattern-matching game
   - High-charisma pets can perform longer "combos" for score multipliers
   - Rewards include Charisma experience, BITS, and Aura Dust

3. **Habitat Dash (Energy & Agility Training)**:
   - An "endless runner" style game through procedurally generated habitats
   - Pet's Energy stat acts as the "life bar" for the run
   - Rewards include Energy/Agility experience, BITS, and environmental reagents

## Implementation Notes

The pallet is designed with the KISS principle in mind, focusing on:

- Clear separation of concerns
- Modular architecture
- Security through bounded collections and proper type safety
- Efficient on-chain storage (minimal state bloat)

The actual gameplay logic happens off-chain, with only essential results stored on-chain to minimize blockchain bloat.