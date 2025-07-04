# Critter-Craft Battle System

A sophisticated and strategic turn-based battle system for Critter-Craft that focuses on tactical gameplay using critters' unique adaptations and environmental factors.

## Core Philosophy

Battles in Critter-Craft are not about defeating an opponent in a brutal sense, but about a **contest of will, stamina, and strategy**. A Zoologist aims to study, pacify, or prove dominance over a wild critter or a rival's companion. The "health bar" is termed **Stamina** or **Resolve**.

## Key Features

### Action Point (AP) System

Each turn, your critter gains a set amount of AP (e.g., 4 AP). Every action costs AP, forcing strategic trade-offs:

- **Basic Maneuver (1 AP)**: A simple, low-damage attack or defensive action.
- **Use Adaptation (2-4 AP)**: Unique biological abilities like Camouflage, Bioluminescence, etc.
- **Interact with Environment (2 AP)**: Use the environment to your advantage.
- **Use Equipped Item (1 AP)**: Use a crafted consumable.
- **Defend / Wait (1 AP)**: End the turn and gain a small defensive bonus.

### Environmental Factors

The battle environment is not just a backdrop; it's an active participant that influences strategy:

- **Sun-Dappled Forest**: Provides "Cover." All critters gain a passive +10% Evasion. Camouflage is 50% more effective here.
- **Murky Swamp**: At the end of each turn, there is a 25% chance for a non-aquatic critter to become Slowed.
- **Geothermal Vents**: Fire-type abilities are 20% stronger. Non-fire types take minor Burn damage each turn.
- **Crystal Cavern**: Dark environment. Bioluminescence and Echolocation abilities are highly effective.

### Status Effects

Status effects add another strategic layer:

- **Pacified**: Stamina is at zero. Cannot fight.
- **Poisoned**: Loses a percentage of Stamina each turn.
- **Burned**: Loses Stamina each turn and has reduced attack power.
- **Blinded**: Accuracy is significantly reduced.
- **Camouflaged**: Evasion is significantly increased.
- **Empowered**: Damage output is increased.
- **Slowed**: Gains fewer AP per turn.
- **Inspired**: Increased chance of landing critical hits.

### Craftable Items

Items are crafted using materials gathered from the world:

- **Consumables**:
  - **Healing Salve**: Restores a flat amount of Stamina.
  - **Adrenaline Berry**: Grants +2 AP for the current turn.
  - **Focus Root**: Cures the Blinded status effect.
  - **Thick Mud**: Can be thrown at an opponent to inflict the Slowed status.

- **Gear**:
  - **Toughened Bark Armor**: Provides a passive +10 defense.
  - **Polished River Stone**: Increases resistance to Burn effects.
  - **Amplifying Crystal**: Increases the power of aura-based or elemental abilities.

## Code Structure

The battle system is designed with the KISS principle of Modularity and Decoupling. Each file has a clear, single responsibility:

- **manager.py**: The main battle engine and state machine.
- **state.py**: Contains all data models for battle.
- **abilities.py**: Defines all unique Adaptation abilities.
- **items.py**: Defines all craftable items used in battle.
- **ui.py**: Handles all user-facing output.
- **formulas.py**: Centralizes all game-balancing math.

## Getting Started

To run a demo battle:

```python
from battle import start_battle

# Define your pets and environment
player_pet = {
    "name": "Sparkles",
    "species": "Chameleon",
    "level": 3,
    "adaptations": ["camouflage", "echolocation"]
}

opponent_pet = {
    "name": "Glimmer",
    "species": "Anglerfish",
    "level": 2,
    "adaptations": ["bioluminescence", "venom_strike"]
}

# Start a battle
result = start_battle(player_pet, opponent_pet, "forest")
```

Or run the included demo:

```python
from battle.demo import run_demo
run_demo()
```

## Extending the System

The battle system is designed to be easily extended:

- Add new abilities by creating new classes in `abilities.py`
- Add new items by creating new classes in `items.py`
- Add new environments by extending the environment creation in `manager.py`
- Adjust game balance by modifying the formulas in `formulas.py`