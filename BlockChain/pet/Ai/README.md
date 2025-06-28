# CritterCraft: Virtual Pet & Creature Creation

## 🚀 Project Overview

CritterCraft is a comprehensive digital companion experience that combines virtual pet care with educational creature creation. The application features two integrated systems:

1. **Virtual Pet Care**: Nurture and interact with your digital companion, watching it grow and develop based on your care.
2. **Creature Creation**: Design and customize unique critters inspired by real-world animals, learning about biology and adaptations along the way.

This dual approach creates a rich, engaging experience that's both entertaining and educational, with a path toward blockchain integration for true digital ownership.

## 🧠 KISS Principles Applied

The implementation follows the Expanded KISS Principles:

- **K (Know Your Core, Keep it Clear)**: Clear separation of concerns with well-defined interfaces. Each class has a single responsibility, and the code is organized into logical modules.

- **I (Iterate Intelligently)**: The code is structured for easy updates and maintenance. New features can be added without disrupting existing functionality.

- **S (Systematize for Scalability)**: The modular design with clear interfaces allows for easy extension. The system can be scaled to include new features and functionality.

- **S (Sense the Landscape & Stimulate Engagement)**: The system is designed for user engagement, with features that encourage exploration and learning.

## 🧠 KISS Principles Applied

The implementation follows the Expanded KISS Principles:

- **K (Know Your Core, Keep it Clear)**: Clear separation of concerns with well-defined interfaces. Each class has a single responsibility, and the code is organized into logical modules.

- **I (Iterate Intelligently)**: The code is structured for easy updates and maintenance. New features can be added without disrupting existing functionality.

- **S (Systematize for Scalability)**: The modular design with clear interfaces allows for easy extension. The system can be scaled to include new features and functionality.

- **S (Sense the Landscape & Stimulate Engagement)**: The system is designed for user engagement, with features that encourage exploration and learning.

## ✨ Features

### Core Pet Management & Companionship
- **Custom Pet Creation**: Choose from various species archetypes and aura colors, each with unique traits and benefits.
- **Comprehensive Pet Statistics**: Monitor and manage your pet's happiness, energy, hunger, IQ, charisma, cleanliness, and social needs.
- **Time-Based Pet Needs**: Pet stats naturally change over time, requiring consistent care and attention.
- **Owner-Pet Interactions**: Feed, play, chat, and groom your pet to maintain its well-being.
- **Dynamic Status & Mood System**: Pet's mood changes based on its stats, with contextual alerts for critical needs.

### Critter Creation & Educational Exploration
- **Interactive Crafting Interface**: Create unique critter designs by adding materials and adaptations to your pet.
- **Diverse Material Palette**: Choose from virtual fur, scales, feathers, shells, and customizable colors.
- **Educational Integration**: Each crafting challenge is linked to a real animal, providing facts about habitat, diet, adaptations, and conservation status.
- **Adaptation Station**: Apply specific adaptations (e.g., camouflage, bioluminescence) to your critter and see how they function in simulated environments.
- **Zoologist's Journal**: Unlock new animal facts and crafting materials as you progress through zoologist levels.

### Progression & Blockchain Integration
- **Blockchain Migration Readiness**: Work toward meeting specific thresholds to prepare your pet for future blockchain migration.
- **Interaction History Tracking**: The system maintains a detailed log of all successful owner interactions.
- **Zoologist Progression System**: Advance through zoologist levels by creating critters, unlocking new materials and adaptations.

## 🕹️ How to Play

1. **Create Your Pet**: Choose a name, species, and aura color for your digital companion.
2. **Care for Your Pet**: Regularly feed, play with, chat with, and groom your pet to maintain its well-being.
3. **Create a Critter Form**: Select a base animal and customize it with materials and adaptations.
4. **Learn & Explore**: Discover facts about real animals and test your critter in different environments.
5. **Progress & Grow**: Work toward blockchain migration readiness and advance through zoologist levels.

## 📋 File Structure

### Core Components
- `config.py`: Centralized configuration for the entire system, using Enums and namespaced classes.
- `pet_core.py`: Core implementation of the virtual pet system with PetPersistence class.
- `critter_core.py`: Core implementation of the critter creation system with CritterPersistence class.
- `integrated_core.py`: Integration of the pet and critter systems with IntegratedPetPersistence class.
- `integrated_demo.py`: Interactive demonstration of the integrated system.

Each component follows the KISS principles with clear separation of concerns:
- Data models (Pet, Critter, IntegratedPet)
- Logic managers (PetLogicManager, IntegratedPetManager)
- Persistence (PetPersistence, CritterPersistence, IntegratedPetPersistence)

## 🚀 Getting Started

### Prerequisites

- Python 3.6 or higher

### Installation

1. Clone the repository:
```
git clone https://github.com/yourusername/crittercraft.git
cd crittercraft
```

2. Run the integrated application:
```
python integrated_main.py
```

### Demo

To see a demonstration of the integrated functionality:
```
python integrated_demo.py
```

## 🌟 Progression Systems

### Pet Migration Readiness
Work toward meeting these thresholds to prepare your pet for blockchain migration:
- Minimum happiness, energy, IQ, and charisma levels
- Maximum hunger level
- Minimum number of interactions
- Minimum days of ownership

### Zoologist Levels
As you create more critters, you'll progress through different zoologist levels:
1. **Novice Zoologist**: Starting level with basic materials and adaptations.
2. **Apprentice Zoologist**: Unlocks additional materials and adaptations.
3. **Journeyman Zoologist**: Unlocks even more materials and adaptations.
4. **Expert Zoologist**: Unlocks advanced materials and adaptations.
5. **Master Zoologist**: Unlocks all materials and adaptations.

## 🧪 Adaptation Station

The Adaptation Station allows you to simulate how your critters would perform in different environments:
- **Forest**: Test how well your critter adapts to forest environments.
- **Ocean**: Test how well your critter adapts to ocean environments.
- **Desert**: Test how well your critter adapts to desert environments.
- **Arctic**: Test how well your critter adapts to arctic environments.
- **Grassland**: Test how well your critter adapts to grassland environments.

## 🔮 Future Development

- **Enhanced AI Integration**: Dynamic personality system and deeper conversational capabilities.
- **Graphical Interface**: Moving beyond CLI to a rich graphical environment.
- **Full Blockchain Implementation**: Complete migration to blockchain-based digital assets.
- **Community & Social Features**: Gallery showcasing, challenges, and competitions.

## 🤝 Contributing

We welcome contributions from the community! If you'd like to contribute, please fork the repository and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 📧 Contact

For any inquiries, feel free to reach out to support@crittercraftapp.com.