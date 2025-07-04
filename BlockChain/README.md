# Critter-Craft Blockchain Core

This repository contains the core blockchain components for the Critter-Craft game, including the battle system, the Zoologist's Ledger blockchain integration, the Echo-Synthesis breeding system, and the Dual-Layer Economy System.

## Overview

Critter-Craft is a game that combines pet simulation, strategic battles, breeding, crafting, and a player-driven economy in a unique ecosystem. The blockchain integration, named the "Zoologist's Ledger," is not a trendy addition but a foundational pillar that reinforces the game's core values. It manages provenance, permanence, and player agency without complicating the core gameplay loop.

## Repository Structure

```
blockchain_core/
├── pallets/                  # Core game components
│   ├── pallet-battles/       # Strategic battle system
│   │   ├── src/              # Source code for the battle system
│   │   │   └── battle/       # Battle system modules
│   │   └── run_demo.py       # Script to run the battle system demo
│   │
│   ├── pallet-ledger/        # Zoologist's Ledger blockchain
│   │   ├── src/              # Source code for the blockchain
│   │   └── run_demo.py       # Script to run the blockchain demo
│   │
│   ├── pallet-breeding/      # Echo-Synthesis breeding system
│   │   ├── src/              # Source code for the breeding system
│   │   └── run_demo.py       # Script to run the breeding system demo
│   │
│   ├── pallet-economy/       # Dual-Layer Economy System
│   │   ├── src/              # Source code for the economy system
│   │   └── run_demo.py       # Script to run the economy system demo
│   │
│   ├── integration.py        # Integration between battle system and blockchain
│   │
│   ├── breeding_integration.py # Integration between breeding system, battle system, and blockchain
│   │
│   └── economy_integration.py # Integration between economy system, breeding system, battle system, and blockchain
│
└── README.md                 # This file
```

## Components

### Battle System

The battle system is a sophisticated and strategic turn-based system that focuses on tactical gameplay using critters' unique adaptations and environmental factors. Key features include:

- **Action Point (AP) System**: Each turn, your critter gains a set amount of AP. Every action costs AP, forcing strategic trade-offs.
- **Environmental Factors**: The battle environment is not just a backdrop; it's an active participant that influences strategy.
- **Status Effects**: Status effects add another strategic layer to battles.
- **Craftable Items**: Items are crafted using materials gathered from the world.

For more details, see the [Battle System README](pallets/pallet-battles/README.md).

### Zoologist's Ledger

The Zoologist's Ledger is the blockchain integration for Critter-Craft. It manages provenance, permanence, and player agency. Key features include:

- **On-Chain Assets (NFTs)**: Only items of true scarcity, identity, and high achievement are tokenized.
- **Decentralized Identity (DID)**: A player's identity is more than a username; it's a verifiable, on-chain reputation.
- **Governance Model**: The Zoologist's Guild gives players a strategic voice in the game's future.
- **Consensus Mechanism**: Proof-of-Reputation & Stake (PoRS) combines investment with merit.
- **Ecosystem Economy**: A single, primary currency ($AURA) underpins the advanced economy.

For more details, see the [Zoologist's Ledger README](pallets/pallet-ledger/README.md).

### Echo-Synthesis Breeding System

The Echo-Synthesis breeding system is a strategic, end-game system that allows players to combine the genetic and spiritual essence of their companions to discover new potential, create unique hybrids, and cement a permanent legacy on the blockchain. Key features include:

- **Genetic Code**: Every critter possesses a unique Genetic Code with Core Genes, Potential Genes, and Cosmetic Genes.
- **Standard Breeding (Intra-Species Synthesis)**: Refine a specific species line with strategic inheritance mechanics.
- **Cross-Species Breeding (Hybrid Synthesis)**: Create unique hybrid species with expanded adaptation pools.
- **Family Tree / Genealogy Charts**: Trace a pet's ancestry through generations with on-chain provenance.
- **Strategic Items**: Use catalysts and gene splicers to influence breeding outcomes.
- **Inbreeding Mechanics**: Manage breeding lines strategically to avoid negative mutations.

For more details, see the [Echo-Synthesis Breeding System README](pallets/pallet-breeding/README.md).

### Dual-Layer Economy System

The Dual-Layer Economy System is a strategic ecosystem that separates high-frequency, everyday activities from high-value, permanent asset transactions. This creates an accessible economy for all players while providing deep strategic layers for dedicated masters. Key features include:

- **Dual-Layer Model**: Separates the Local Economy (off-chain) from the Global Economy (on-chain).
- **Item Taxonomy**: Categorizes items based on their role and permanence (Materials, Consumables, Gear, Bridging Items, NFTs).
- **Player Specialization**: Supports different playstyles (Adventurer, Crafter, Breeder, Achiever).
- **Local Marketplace**: A high-volume hub for everyday transactions using $BITS.
- **Global Marketplace**: A prestigious exchange for high-value assets using $AURA.
- **Strategic Bridging**: Rare items that enable on-chain actions, creating a link between the two economies.

For more details, see the [Dual-Layer Economy System README](pallets/pallet-economy/README.md).

### Integration

The integration modules provide functions for integrating the various systems with the blockchain:

#### Battle Integration

- **record_battle_victory**: Record a significant battle victory on the Zoologist's Ledger.
- **mint_legendary_item**: Mint a legendary item as an NFT on the Zoologist's Ledger.
- **battle_with_blockchain**: Run a battle and record significant events on the blockchain.

#### Breeding Integration

- **perform_breeding**: Perform breeding between two pets and record the result on the blockchain.
- **convert_pet_to_battle_format**: Convert a pet's genetic code to the format expected by the battle system.
- **battle_with_bred_pet**: Battle with a bred pet and record significant events on the blockchain.

#### Economy Integration

- **battle_with_items**: Battle with items from the player's inventory.
- **craft_breeding_catalyst**: Craft a breeding catalyst for use in the breeding system.
- **breed_with_catalyst**: Breed two pets using a catalyst from the player's inventory.
- **mint_legendary_gear**: Mint a piece of gear as a legendary NFT.
- **sell_on_marketplace**: Sell an item on the appropriate marketplace.
- **buy_from_marketplace**: Buy an item from a marketplace.

## Getting Started

To run the battle system demo:

```bash
cd pallets/pallet-battles
python run_demo.py
```

To run the Zoologist's Ledger demo:

```bash
cd pallets/pallet-ledger
python run_demo.py
```

To run the Echo-Synthesis breeding system demo:

```bash
cd pallets/pallet-breeding
python run_demo.py
```

To run the Dual-Layer Economy System demo:

```bash
cd pallets/pallet-economy
python run_demo.py
```

## Development

To extend the system, you can:

1. **Add new abilities** to the battle system by creating new classes in `pallet-battles/src/battle/abilities.py`.
2. **Add new items** to the battle system by creating new classes in `pallet-battles/src/battle/items.py`.
3. **Add new transaction types** to the blockchain by extending the `TransactionType` enum in `pallet-ledger/src/models.py`.
4. **Add new proposal types** to the governance system by extending the `ProposalType` enum in `pallet-ledger/src/models.py`.
5. **Add new species** to the breeding system by updating the hybrid_results dictionary in `pallet-breeding/src/synthesis.py`.
6. **Add new gene splicers** to the breeding system by creating new classes in `pallet-breeding/src/catalysts.py`.
7. **Add new item types** to the economy system by creating new classes in `pallet-economy/src/items.py`.
8. **Add new recipes** to the crafting system by creating new Recipe instances in `pallet-economy/src/crafting.py`.
9. **Enhance the integration** by adding new functions to the integration modules.