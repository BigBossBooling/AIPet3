# Critter-Craft Activities and Zoologist's Lodge

This module implements the Activities and Zoologist's Lodge systems for Critter-Craft.

## Overview

The Activities system provides a variety of gameplay loops for players to engage with, including:

- **Mini-Games**: Short, engaging, skill-based activities designed to directly train specific pet stats and provide a steady stream of the soft currency, $BITS.
- **Two-Player Games**: Games designed to foster social interaction and high-level strategic competition.
- **Jobs**: The primary, reliable method for earning $BITS, directly tied to a pet's trained stats.
- **Quests**: The backbone of the player's journey, driving them to explore and interact with the world.
- **Adventurous Quests**: Long, challenging questlines that tell the deeper story of the Critter-Craft world and offer the greatest rewards.

The Zoologist's Lodge (daycare) system allows players to leave their pets when they are offline and hire other players as temporary Caregivers.

## Architecture

The implementation follows the Expanded KISS Principle:

- **K (Keep it Simple)**: Core systems are decoupled (ActivityManager, Lodge).
- **I (Intelligent)**: The system intelligently loads all game content from a config file.
- **S (Systematized)**: The menu is a scalable, systematized dispatcher.
- **S (Secure)**: The UI provides a clear, engaging, and secure user experience.

## File Structure

- `activities.py`: Core definitions for the Activities system.
- `activities_system.py`: Business logic for the Activities system.
- `config_activities.py`: Data-driven configuration for all activities.
- `lodge.py`: Data models for the Zoologist's Lodge system.
- `lodge_system.py`: Business logic for the Zoologist's Lodge system.
- `demo_updated.py`: Interactive demo for the Activities and Zoologist's Lodge systems.
- `run_demo.py`: Script to run the interactive demo.
- `run_test.py`: Script to run a simple test of the systems.

## Running the Demo

To run the interactive demo:

```bash
./run_demo.py
```

This will start an interactive command-line interface where you can explore the Activities and Zoologist's Lodge systems.

## Running the Test

To run a simple test of the systems:

```bash
./run_test.py
```

This will run a basic test of the Activities and Zoologist's Lodge systems, demonstrating their core functionality.

## Activities

### Mini-Games

- **Logic Leaper**: A tile-based puzzle game of pathfinding and strategic planning.
- **Aura Weaving**: A rhythm and pattern-matching game of timing and memory.
- **Habitat Dash**: An "endless runner" style game through procedurally generated habitats.

### Two-Player Games

- **Critter Tactics**: A turn-based tactical board game, the ultimate strategic duel.
- **Cooperative Crafting**: Two players bring unique materials to a Synergy Workbench to craft powerful items.

### Jobs

- **Crystal Mining**: Mine deep caves for rare gems. Requires high Strength.
- **Bioluminescent Guide**: Guide travelers through dark areas. Requires high Charisma.
- **Herbalist's Assistant**: Identify and sort rare herbs for an apothecary. Requires high IQ.

### Quests

- **Gathering: Sunpetal Pollen**: Collect 10 Sunpetal Pollens for the local herbalist.
- **Crafting: Healing Salves**: Craft 3 Healing Salves for the outpost.
- **Pacification: Alpha Glimmer-Moth**: A territorial Alpha Glimmer-Moth is causing trouble. Pacify it in a battle.

### Adventurous Quests

- **The Whispering Blight**: Investigate a strange torpor affecting critters in the Verdant Maw.

## Zoologist's Lodge

The Zoologist's Lodge allows players to:

- Leave their pets when they are offline.
- Hire other players as temporary Caregivers.
- Perform care activities (feed, play, groom) on pets.
- Earn rewards for caring for other players' pets.

## Future Enhancements

- Add more activities of each type.
- Implement a reputation system for Caregivers.
- Add more care activities for the Zoologist's Lodge.
- Implement a scheduling system for Jobs.
- Add more complex rewards for Adventurous Quests.# Critter-Craft Activities and Zoologist's Lodge

This module implements the Activities and Zoologist's Lodge systems for Critter-Craft.

## Overview

The Activities system provides a variety of gameplay loops for players to engage with, including:

- **Mini-Games**: Short, engaging, skill-based activities designed to directly train specific pet stats and provide a steady stream of the soft currency, $BITS.
- **Two-Player Games**: Games designed to foster social interaction and high-level strategic competition.
- **Jobs**: The primary, reliable method for earning $BITS, directly tied to a pet's trained stats.
- **Quests**: The backbone of the player's journey, driving them to explore and interact with the world.
- **Adventurous Quests**: Long, challenging questlines that tell the deeper story of the Critter-Craft world and offer the greatest rewards.

The Zoologist's Lodge (daycare) system allows players to leave their pets when they are offline and hire other players as temporary Caregivers.

## Architecture

The implementation follows the Expanded KISS Principle:

- **K (Keep it Simple)**: Core systems are decoupled (ActivityManager, Lodge).
- **I (Intelligent)**: The system intelligently loads all game content from a config file.
- **S (Systematized)**: The menu is a scalable, systematized dispatcher.
- **S (Secure)**: The UI provides a clear, engaging, and secure user experience.

## File Structure

- `activities.py`: Core definitions for the Activities system.
- `activities_system.py`: Business logic for the Activities system.
- `config_activities.py`: Data-driven configuration for all activities.
- `lodge.py`: Data models for the Zoologist's Lodge system.
- `lodge_system.py`: Business logic for the Zoologist's Lodge system.
- `demo_updated.py`: Interactive demo for the Activities and Zoologist's Lodge systems.
- `run_demo.py`: Script to run the interactive demo.
- `run_test.py`: Script to run a simple test of the systems.

## Running the Demo

To run the interactive demo:

```bash
./run_demo.py
```

This will start an interactive command-line interface where you can explore the Activities and Zoologist's Lodge systems.

## Running the Test

To run a simple test of the systems:

```bash
./run_test.py
```

This will run a basic test of the Activities and Zoologist's Lodge systems, demonstrating their core functionality.

## Activities

### Mini-Games

- **Logic Leaper**: A tile-based puzzle game of pathfinding and strategic planning.
- **Aura Weaving**: A rhythm and pattern-matching game of timing and memory.
- **Habitat Dash**: An "endless runner" style game through procedurally generated habitats.

### Two-Player Games

- **Critter Tactics**: A turn-based tactical board game, the ultimate strategic duel.
- **Cooperative Crafting**: Two players bring unique materials to a Synergy Workbench to craft powerful items.

### Jobs

- **Crystal Mining**: Mine deep caves for rare gems. Requires high Strength.
- **Bioluminescent Guide**: Guide travelers through dark areas. Requires high Charisma.
- **Herbalist's Assistant**: Identify and sort rare herbs for an apothecary. Requires high IQ.

### Quests

- **Gathering: Sunpetal Pollen**: Collect 10 Sunpetal Pollens for the local herbalist.
- **Crafting: Healing Salves**: Craft 3 Healing Salves for the outpost.
- **Pacification: Alpha Glimmer-Moth**: A territorial Alpha Glimmer-Moth is causing trouble. Pacify it in a battle.

### Adventurous Quests

- **The Whispering Blight**: Investigate a strange torpor affecting critters in the Verdant Maw.

## Zoologist's Lodge

The Zoologist's Lodge allows players to:

- Leave their pets when they are offline.
- Hire other players as temporary Caregivers.
- Perform care activities (feed, play, groom) on pets.
- Earn rewards for caring for other players' pets.

## Future Enhancements

- Add more activities of each type.
- Implement a reputation system for Caregivers.
- Add more care activities for the Zoologist's Lodge.
- Implement a scheduling system for Jobs.
- Add more complex rewards for Adventurous Quests.