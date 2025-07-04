# Critter Pet Status Pallet

This pallet manages pet status, conditions, and state changes for the CritterCraft ecosystem.

## Overview

The Critter Pet Status pallet provides the on-chain infrastructure for tracking pet states, needs, and conditions in the CritterCraft ecosystem. It enables the Tamagotchi-like care mechanics that drive player engagement and pet development.

## Features

- **Pet Status Tracking**: Monitors pet mood and interaction timestamps
- **Pet Stats System**: Tracks core pet attributes (Strength, Agility, Intelligence, Vitality, Charisma)
- **Pet Needs System**: Manages pet needs (Hunger, Energy, Happiness, Hygiene, Social)
- **Condition System**: Implements buffs and debuffs that affect pet stats and needs
- **Mood System**: Dynamic pet mood based on need satisfaction
- **Interaction Mechanics**: Feed, rest, play, groom, and socialize with pets
- **Automatic Need Decay**: Needs gradually decrease over time, requiring player attention

## Integration with Other Pallets

This pallet integrates with:

- **pallet-critter-nfts**: For pet ownership verification
- **pallet-critter-minigames**: For stat-based gameplay
- **pallet-critter-jobs**: For stat-based job requirements
- **pallet-critter-daycare**: For caregiver interactions

## Extrinsics

### Pet Status Management
- `initialize_pet_status`: Initialize a new pet's status
- `update_pet_stats`: Update a pet's stats (admin only)

### Pet Care Interactions
- `feed_pet`: Feed a pet to restore hunger
- `rest_pet`: Rest a pet to restore energy
- `play_with_pet`: Play with a pet to increase happiness
- `groom_pet`: Groom a pet to increase hygiene
- `socialize_pet`: Socialize a pet with another pet to increase social need

### Condition System
- `create_condition`: Create a new condition (admin only)
- `apply_condition`: Apply a condition to a pet (admin only)
- `remove_condition`: Remove a condition from a pet (admin only)

## Pet Status Mechanics

The pet status system creates a comprehensive care simulation:

1. **Pet Needs**:
   - Five core needs: Hunger, Energy, Happiness, Hygiene, Social
   - Needs decay over time, requiring regular player attention
   - Different interactions restore different needs

2. **Pet Mood**:
   - Dynamic mood based on overall need satisfaction
   - Five mood states: Happy, Content, Neutral, Sad, Distressed
   - Mood affects pet performance in activities

3. **Pet Stats**:
   - Five core stats: Strength, Agility, Intelligence, Vitality, Charisma
   - Stats determine pet capabilities in games and jobs
   - Stats can be improved through training and activities

4. **Condition System**:
   - Conditions can be positive (buffs) or negative (debuffs)
   - Conditions have duration and severity
   - Conditions modify pet stats and needs
   - Some conditions require specific care to resolve

5. **Care Mechanics**:
   - Feeding restores hunger but requires cooldown
   - Resting restores energy but requires longer cooldown
   - Playing increases happiness but consumes energy
   - Grooming improves hygiene
   - Socializing with other pets improves social need

## Implementation Notes

The pallet is designed with the KISS principle in mind, focusing on:

- Clear separation of concerns
- Modular architecture
- Security through bounded collections and proper type safety
- Efficient on-chain storage (minimal state bloat)

The pet status system is the core gameplay loop of the CritterCraft ecosystem, creating the Tamagotchi-like care mechanics that drive player engagement and pet development.