# CritterChain CLI Wallet - Specification

This document outlines the specifications for a basic Command-Line Interface (CLI) wallet for interacting with the CritterChain. This wallet is intended for initial testing, development, and basic user interactions as part of "Project 1: Core Protocol & Network Foundation Implementation."

## 1. General Principles

*   **Simplicity:** Focus on core functionalities for an MVP.
*   **Interaction with Local Node:** The CLI will connect to a running CritterChain node (e.g., `ws://127.0.0.1:9944`).
*   **Key Management:** Users will typically provide their private key seed/URI (e.g., "//Alice", "//Bob", or a raw seed) for signing transactions. Secure key storage is outside the scope of this basic CLI's first version but should be considered for any user-facing tools.
*   **Output:** Provide clear feedback to the user, including transaction status (submitted, in block, finalized) and any relevant data or error messages.

## 2. Supported Commands

### 2.1. Account & Balance Operations

**a. `get-balance <AccountId>`**
    *   **Purpose:** Queries and displays the free PTCN balance of a specified account.
    *   **Arguments:**
        *   `<AccountId>`: The SS58 address of the account to query.
    *   **Interaction:** Queries `System_Account` storage item for `pallet-balances`.
    *   **Output:**
        ```
        Account: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
        Free Balance: 1234.5678 PTCN
        ```

**b. `transfer <ToAccountId> <Amount> --from <SeedOrUri>`**
    *   **Purpose:** Transfers a specified amount of PTCN from the sender's account to a recipient account.
    *   **Arguments:**
        *   `<ToAccountId>`: The SS58 address of the recipient.
        *   `<Amount>`: The amount of PTCN to transfer (e.g., "100.5"). The CLI should convert this to the chain's smallest unit.
        *   `--from <SeedOrUri>`: The seed phrase or URI for the sender's account to sign the transaction.
    *   **Extrinsic:** `pallet_balances::transfer_allow_death(dest, value)` or `pallet_balances::transfer_keep_alive(dest, value)`. For MVP, `transfer_allow_death` is simpler if existential deposit policies are clear.
    *   **Output:**
        ```
        Transaction submitted with hash: 0x...
        Status: InBlock
        Status: Finalized
        Successfully transferred <Amount> PTCN from <SenderAddress> to <ToAccountId>.
        ```
        Or error messages.

### 2.2. Pet NFT Operations (`pallet-critter-nfts`)

**a. `mint-nft <Species> <Name> --from <SeedOrUri>`**
    *   **Purpose:** Mints a new Pet NFT with the given species and name, owned by the sender.
    *   **Arguments:**
        *   `<Species>`: A string for the pet's species (e.g., "RoboDog", "PixelCat").
        *   `<Name>`: A string for the pet's name (e.g., "Sparky").
        *   `--from <SeedOrUri>`: The seed phrase or URI for the owner's account.
    *   **Extrinsic:** `pallet_critter_nfts::mint_pet_nft(species: Vec<u8>, name: Vec<u8>)`
    *   **Output:**
        ```
        Transaction submitted with hash: 0x...
        Status: InBlock
        Status: Finalized
        Successfully minted Pet NFT!
        Pet ID: <NewPetId>
        Species: <Species>
        Name: <Name>
        Owner: <SenderAddress>
        (Optionally display derived charter attributes)
        ```
        Or error messages (e.g., `ExceedMaxOwnedPets`).

**b. `list-my-nfts <AccountId>`**
    *   **Purpose:** Lists all Pet NFTs owned by a specified account, showing basic details.
    *   **Arguments:**
        *   `<AccountId>`: The SS58 address of the owner.
    *   **Interaction:** Queries `pallet_critter_nfts::OwnerOfPet(AccountId)` to get owned Pet IDs, then queries `pallet_critter_nfts::PetNfts(PetId)` for each ID to get details.
    *   **Output:**
        ```
        Pets owned by <AccountId>:
        ------------------------------------
        Pet ID: <PetId_1> | Name: <Name_1> | Species: <Species_1> | Level: <Lvl_1>
        Pet ID: <PetId_2> | Name: <Name_2> | Species: <Species_2> | Level: <Lvl_2>
        ...
        ------------------------------------
        Total Pets: <Count>
        ```
        Or "No pets found for this account."

**c. `get-nft-details <PetId>`**
    *   **Purpose:** Displays detailed information about a specific Pet NFT.
    *   **Arguments:**
        *   `<PetId>`: The ID of the Pet NFT.
    *   **Interaction:** Queries `pallet_critter_nfts::PetNfts(PetId)`.
    *   **Output:**
        ```
        Pet NFT Details (ID: <PetId>):
        ------------------------------------
        Owner: <OwnerAddress>
        Name: <Name>
        Species: <Species>
        DNA Hash: <DnaHash>
        Level: <Level>
        XP: <ExperiencePoints>
        Mood: <MoodIndicator> (e.g., Happy, Neutral, Sad - based on u8 value)
        Last Fed Block: <BlockNumber>
        Last Played Block: <BlockNumber>
        Last State Update Block: <BlockNumber>
        Charter Attributes:
          Base Strength: <Value>
          Base Agility: <Value>
          Base Intelligence: <Value>
          Base Vitality: <Value>
          Elemental Affinity: <Affinity or None>
        Personality Traits:
          - <Trait1>
          - <Trait2>
        ------------------------------------
        ```
        Or "Pet NFT with ID <PetId> not found."

**d. `set-pet-name <PetId> <NewName> --from <SeedOrUri>`**
    *   **Purpose:** Allows the owner of a Pet NFT to change its name.
    *   **Arguments:**
        *   `<PetId>`: The ID of the Pet NFT.
        *   `<NewName>`: The new name string for the pet.
        *   `--from <SeedOrUri>`: Seed/URI of the pet's owner.
    *   **Extrinsic:** `pallet_critter_nfts::update_pet_metadata(pet_id: PetId, name: Some(Vec<u8>), personality_traits: None)`
        *   Note: `update_pet_metadata` was simplified to only take `name` and `personality_traits`.
    *   **Output:**
        ```
        Transaction submitted with hash: 0x...
        Status: Finalized
        Pet ID <PetId>'s name successfully updated to "<NewName>".
        ```

### 2.3. Basic Care Interactions (Simplified for CLI - Conceptual)

**a. `feed-pet-cli <PetId> <FoodItemId> --from <SeedOrUri>`**
    *   **Purpose:** Simulates feeding a pet. For CLI MVP, this might just update `last_fed_block` and `mood_indicator`. Assumes `FoodItemId` is known/valid conceptually.
    *   **Arguments:**
        *   `<PetId>`
        *   `<FoodItemId>` (conceptual, `pallet-items` not fully part of Project 1 implementation)
        *   `--from <SeedOrUri>`
    *   **Extrinsic:** `pallet_critter_nfts::feed_pet(pet_id: PetId, food_item_id: ItemId)`
    *   **Output:** "Pet <PetId> has been fed. Last fed block updated."

**b. `play-with-pet-cli <PetId> <ToyItemId> --from <SeedOrUri>`**
    *   **Purpose:** Simulates playing with a pet. Updates `last_played_block` and `mood_indicator`. Assumes `ToyItemId` is known/valid.
    *   **Arguments:**
        *   `<PetId>`
        *   `<ToyItemId>` (conceptual)
        *   `--from <SeedOrUri>`
    *   **Extrinsic:** `pallet_critter_nfts::play_with_pet(pet_id: PetId, toy_item_id: ItemId)`
    *   **Output:** "You played with Pet <PetId>. Last played block updated."

*(Note: The `*-cli` suffix is to distinguish these simplified CLI interactions from potentially more complex UI interactions that might involve richer feedback from `pallet-items`)*.

## 3. Future CLI Enhancements (Post-Project 1)

*   Key generation and management within the CLI.
*   More detailed error reporting from the node.
*   Interaction with other pallets (Marketplace, Battles, Quests, Staking, Governance) as they are implemented.
*   Querying specific storage items for debugging.
*   Batch transaction submission.

This CLI specification provides the essential commands for developers and early users to interact with the core functionalities of CritterChain during its initial development phase.
```
