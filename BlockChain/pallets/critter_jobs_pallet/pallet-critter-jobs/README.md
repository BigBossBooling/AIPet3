# Critter Jobs Pallet

This pallet manages jobs and economic activities for CritterCraft pets. It defines job types, requirements, rewards, and durations that drive the economic engine of the CritterCraft ecosystem.

## Overview

The Critter Jobs pallet provides the on-chain infrastructure for various jobs and economic activities in the CritterCraft ecosystem. These jobs allow players to earn $BITS currency and experience points for their pets based on their stats and abilities.

## Features

- **Multiple Job Types**: Supports various jobs focused on different pet stats:
  - **Crystal Mining**: Strength-based job
  - **Bioluminescent Guide**: Charisma-based job
  - **Herbalist Assistant**: Intelligence-based job

- **Job Requirements**: Each job has specific stat requirements:
  - Minimum strength
  - Minimum agility
  - Minimum intelligence
  - Minimum vitality
  - Minimum level

- **Reward System**: Players earn both experience points for their pets and BITS currency based on:
  - Job type
  - Job duration
  - Pet's stats

- **Time-Based Jobs**: Jobs run for a specified duration (in blocks) before rewards can be claimed

## Integration with Other Pallets

This pallet integrates with:

- **pallet-critter-nfts**: For pet ownership verification and experience updates
- **pallet-balances**: For BITS currency rewards

## Extrinsics

- `start_job`: Start a new job for a pet
- `complete_job`: Finish a job and claim rewards
- `abandon_job`: Quit a job without claiming rewards
- `start_crystal_mining`: Convenience function for the Crystal Mining job
- `start_bioluminescent_guide`: Convenience function for the Bioluminescent Guide job
- `start_herbalist_assistant`: Convenience function for the Herbalist Assistant job

## Job Mechanics

Each job is designed to leverage specific pet attributes:

1. **Crystal Mining (Strength-based)**:
   - Requires a pet with high Strength
   - A timing-based mini-game where stronger pets can break harder crystals for rarer gems
   - Higher rewards but longer duration

2. **Bioluminescent Guide (Charisma-based)**:
   - A passive job where high-Charisma pets guide travelers
   - Higher charisma reduces the time and increases the chance of a bonus "tip"
   - Medium rewards with shorter duration

3. **Herbalist Assistant (Intelligence-based)**:
   - Requires a high-IQ pet to correctly identify and sort rare herbs
   - Higher intelligence increases the quality of herbs identified
   - Highest rewards but requires specific pet traits

## Implementation Notes

The pallet is designed with the KISS principle in mind, focusing on:

- Clear separation of concerns
- Modular architecture
- Security through bounded collections and proper type safety
- Efficient on-chain storage (minimal state bloat)

The job system is designed to be the primary, reliable method for earning $BITS in the CritterCraft economy, creating a sustainable economic loop that rewards active participation.