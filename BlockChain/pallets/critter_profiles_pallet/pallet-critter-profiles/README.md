# Critter Profiles Pallet

This pallet manages user profiles, achievements, and gameplay progression for the CritterCraft ecosystem.

## Overview

The Critter Profiles pallet provides the on-chain infrastructure for user profiles, achievements, badges, and social interactions in the CritterCraft ecosystem. It tracks user progression, rewards accomplishments, and enables social connections between players.

## Features

- **User Profiles**: Customizable profiles with usernames, bios, and avatars
- **Status System**: Online status indicators (Online, Offline, Busy, Away, Invisible)
- **Achievement System**: Trackable accomplishments with rewards
- **Badge System**: Collectible and displayable badges for profile customization
- **Experience & Leveling**: Progressive user leveling based on gameplay activities
- **Reputation System**: Tracks user standing in the community
- **Friends System**: Social connections between players

## Integration with Other Pallets

This pallet integrates with:

- **pallet-balances**: For BITS currency rewards from achievements
- **pallet-critter-nfts**: For pet ownership verification
- **pallet-critter-minigames**: For experience and achievement tracking
- **pallet-critter-jobs**: For experience and achievement tracking
- **pallet-critter-daycare**: For social interactions and reputation

## Extrinsics

### Profile Management
- `create_profile`: Create a new user profile
- `update_profile`: Update an existing profile
- `set_status`: Change user online status

### Achievement System
- `create_achievement`: Create a new achievement (admin only)
- `award_achievement`: Award an achievement to a user (admin only)

### Badge System
- `create_badge`: Create a new badge (admin only)
- `award_badge`: Award a badge to a user (admin only)
- `equip_badge`: Equip a badge on user profile
- `unequip_badge`: Unequip a badge from user profile

### Social Features
- `send_friend_request`: Send a friend request to another user
- `accept_friend_request`: Accept a friend request
- `reject_friend_request`: Reject a friend request
- `remove_friend`: Remove a user from friends list

### Progression System
- `add_experience_admin`: Add experience to a user (admin only)
- `change_reputation`: Change a user's reputation (admin only)

## User Profile Mechanics

The user profile system creates a comprehensive player identity:

1. **Profile Creation**:
   - Players create profiles with unique usernames
   - Profiles include customizable bios and avatars
   - Each profile tracks experience, level, and reputation

2. **Achievement System**:
   - Players earn achievements through gameplay
   - Achievements reward experience, BITS, and badges
   - Hidden achievements provide surprise rewards

3. **Badge System**:
   - Badges are earned through achievements or special events
   - Players can equip badges to customize their profiles
   - Rare badges showcase player accomplishments

4. **Social System**:
   - Players can send and accept friend requests
   - Friend lists enable social interactions
   - Online status indicators show player availability

5. **Progression System**:
   - Experience points are earned through activities
   - Levels unlock new features and capabilities
   - Reputation affects social standing and opportunities

## Implementation Notes

The pallet is designed with the KISS principle in mind, focusing on:

- Clear separation of concerns
- Modular architecture
- Security through bounded collections and proper type safety
- Efficient on-chain storage (minimal state bloat)

The profile system serves as the social backbone of the CritterCraft ecosystem, creating player identity and enabling meaningful interactions between users.