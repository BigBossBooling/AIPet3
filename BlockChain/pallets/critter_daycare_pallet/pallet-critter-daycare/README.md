# Critter Daycare Pallet

This pallet manages the "Zoologist's Lodge" daycare system for CritterCraft pets. It allows players to leave their pets at the daycare when they are offline, and hire other players as temporary caregivers.

## Overview

The Critter Daycare pallet provides the on-chain infrastructure for the daycare system in the CritterCraft ecosystem. This system creates a social hub where players can leave their pets when offline and other players can earn BITS by caring for them.

## Features

- **Daycare Creation**: Players can create their own daycares with custom names, descriptions, and fee structures
- **Pet Listing**: Pet owners can list their pets in daycares when they're offline
- **Caregiver System**: Other players can become temporary caregivers for listed pets
- **Interaction Mechanics**: Caregivers can interact with pets to improve their stats and mood
- **Economic Loop**: Creates a viable "Pet Sitter" profession, allowing dedicated players to earn BITS

## Integration with Other Pallets

This pallet integrates with:

- **pallet-critter-nfts**: For pet ownership verification and state updates
- **pallet-balances**: For BITS currency transfers and reservations

## Extrinsics

- `create_daycare`: Create a new daycare facility
- `update_daycare`: Update daycare details
- `set_daycare_status`: Change daycare status (open/closed)
- `list_pet`: List a pet in a daycare
- `remove_pet`: Remove a pet from a daycare
- `become_caregiver`: Become a caregiver for a pet
- `stop_being_caregiver`: Stop being a caregiver for a pet
- `interact_with_pet`: Interact with a pet as a caregiver

## Daycare Mechanics

The daycare system creates a unique social and economic loop:

1. **Daycare Creation**:
   - Players can create their own daycares with custom names and descriptions
   - They set a fee per block that caregivers must pay to care for pets

2. **Pet Listing**:
   - Pet owners can list their pets in daycares when they're offline
   - This allows their pets to continue developing even when the owner isn't active

3. **Caregiver System**:
   - Other players can become temporary caregivers for listed pets
   - They pay a fee to the daycare owner for the privilege
   - A percentage of the fee goes to the platform

4. **Attribute Gain from Caregivers**:
   - A pet's development is influenced by its caregiver
   - When another player interacts with a pet, it gains the normal stat benefits
   - Additionally, it receives a temporary boost to its Personality Traits based on the caregiver's dominant traits

5. **Economic Loop**:
   - Creates a viable "Pet Sitter" profession
   - Daycare owners earn passive income from hosting pets
   - Caregivers earn experience and reputation for their services

## Implementation Notes

The pallet is designed with the KISS principle in mind, focusing on:

- Clear separation of concerns
- Modular architecture
- Security through bounded collections and proper type safety
- Efficient on-chain storage (minimal state bloat)

The daycare system is a key social feature of the CritterCraft ecosystem, creating connections between players and their pets while providing economic opportunities.