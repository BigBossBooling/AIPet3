# Pallet Critter NFTs

## Overview

The `pallet-critter-nfts` is a Substrate pallet designed to manage non-fungible tokens (NFTs) within the critter ecosystem. This pallet provides a robust framework for creating, managing, and interacting with NFTs, enabling various functionalities such as trading, breeding, and questing.

## Features

- **NFT Management**: Implements traits for managing NFTs across different functionalities, including marketplaces and battles.
- **Inter-Pallet Communication**: Defines traits that facilitate interaction with other pallets, ensuring modularity and decoupling.
- **Genetic Information**: Supports genetic traits for critters, allowing for unique and diverse NFT characteristics.

## Structure

- `src/lib.rs`: Contains the core logic and runtime interface for the pallet.
- `src/traits.rs`: Central hub for inter-pallet contracts, defining traits such as:
  - `NftManager`
  - `NftManagerForItems`
  - `NftBreedingHandler`
  - `QuestNftRequirementChecker`
- `src/types.rs`: Defines common types and structures used throughout the pallet.

## Usage

To use the `pallet-critter-nfts`, include it in your Substrate runtime and configure it according to your project's requirements. Ensure that other pallets implementing the defined traits are also included in your runtime.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.