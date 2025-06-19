# CritterCraft: Advanced Features & Gameplay Loops (Conceptual Outline)

This document provides a high-level conceptual outline for advanced economic loops and gameplay features planned for future stages of CritterCraft development, building upon the foundations of Stage 5.

## 1. User Shops

*   **Concept:** Allow players to set up their own persistent, customizable shops within the CritterCraft ecosystem to sell Pet NFTs and, in the future, other items.
*   **Pallet Name:** `pallet-user-shops`
*   **Pallet Interactions:**
    *   **`crittercraft-traits::NftManager`:** This trait is implemented by `pallet-critter-nfts`. `pallet-user-shops` consumes this trait to verify ownership of Pet NFTs being listed, to **lock NFTs** when they are listed (preventing simultaneous actions like battling or transferring them elsewhere, thus ensuring synchronized state across the ecosystem), to unlock them if unlisted, and to facilitate the transfer upon successful purchase. This clear interface allows `pallet-user-shops` to manage NFT states harmoniously with other system components and promotes system-wide data integrity. The security of these operations relies on `pallet-critter-nfts`'s correct implementation of the `NftManager` trait, including robust ownership checks before executing locks or transfers.
    *   **`frame_support::traits::Currency`:** For handling PTCN payments from buyers to sellers, typically implemented by `pallet-balances`.
    *   **(Future) `crittercraft-traits::ItemManager` (or similar):** If/when shops support fungible items or non-NFT items, an interaction trait for `pallet-items` would be needed.
*   **Core On-Chain Logic/Data (for `pallet-user-shops` - MVP Focus):**
    *   **`ShopId`**: For MVP, `ShopId` is simply the `AccountId` of the owner (one shop per user).
    *   **`ShopStatus` Enum:** `Open`, `ClosedTemporarily`.
    *   **`Shop<BoundedName, BoundedDescription>` Struct:**
        *   `owner: AccountId`
        *   `name: BoundedVec<u8, T::MaxShopNameLen>` (Security: `BoundedVec` prevents oversized data)
        *   `description: BoundedVec<u8, T::MaxShopDescriptionLen>` (Security: `BoundedVec` prevents oversized data)
        *   `status: ShopStatus`
        *   *(Future fields: `metadata_uri: Option<BoundedVec<u8, T::MaxUriLen>>`, `reputation_score: u32`)*
    *   **`ListingId` Type:** A unique identifier for each listing, e.g., `u128`.
    *   **`Listing<Balance, NftIdType>` Struct (MVP focuses on unique NFTs):**
        *   `listing_id: ListingId`
        *   `shop_id: AccountId` (Owner of the shop)
        *   `item_id: NftIdType` (For MVP, this is `PetId` from `pallet-critter-nfts`)
        *   `price: Balance`
        *   `quantity: u32` (For MVP, this will always be 1 for unique Pet NFTs)
        *   *(Future fields: `item_type: ListingItemType` enum { NFT, FungibleItem })*
    *   **Storage Items:**
        *   `Shops<AccountId, Shop<T::MaxShopNameLen, T::MaxShopDescriptionLen>>`: Maps an `AccountId` to their `Shop` details.
        *   `ShopOwnedListings<AccountId, BoundedVec<ListingId, T::MaxListingsPerShop>>`: Maps a `ShopId` (owner's `AccountId`) to a list of `ListingId`s they own. (Security: `BoundedVec` prevents unbounded listings per shop).
        *   `AllListings<ListingId, Listing<BalanceOf<T>, T::NftId>>`: Maps a `ListingId` to its `Listing` details.
        *   `NextListingId<ListingId>`: A counter to generate unique `ListingId`s.
*   **Extrinsics (Conceptual Signatures for `pallet-user-shops`):**
    *   `create_shop(origin, name: BoundedVec<u8, T::MaxShopNameLen>, description: BoundedVec<u8, T::MaxShopDescriptionLen>)`: (Security: Validates `origin` is signed, `name` and `description` adhere to bounds via `BoundedVec` type). Creates a shop for the `origin`. Fails if shop already exists for the account.
    *   `update_shop_details(origin, name: Option<BoundedVec<u8, T::MaxShopNameLen>>, description: Option<BoundedVec<u8, T::MaxShopDescriptionLen>>)`: (Security: Validates `origin` is owner, inputs adhere to bounds). Allows owner to update shop details.
    *   `set_shop_status(origin, status: ShopStatus)`: (Security: Validates `origin` is owner). Allows owner to open or temporarily close their shop.
    *   `close_shop_permanently(origin)`: (Security: Validates `origin` is owner). Allows owner to permanently close their shop.
    *   `list_item(origin, item_id: T::NftId, price: BalanceOf<T>)`: (Security: Validates `origin` is shop owner, `item_id` is valid and owned by shop owner, price is reasonable. Relies on `NftManager::lock_nft` for secure locking). Lists a Pet NFT for sale. (MVP: quantity is 1).
    *   `unlist_item(origin, listing_id: ListingId)`: (Security: Validates `origin` is shop owner, `listing_id` is valid and belongs to shop. Relies on `NftManager::unlock_nft` for secure unlocking). Removes a listing.
    *   `buy_item(origin, listing_id: ListingId)`: (Security: Validates `listing_id` is valid and active. Relies on `Currency` trait for secure fund transfer and `NftManager::transfer_nft` for secure NFT transfer). Purchases an item. (MVP: quantity is 1).
*   **Economic Model & Impact:**
    *   Fosters a player-driven economy.
    *   **Fee Mechanism:** `pallet-user-shops` itself **does not implement an intrinsic fee mechanism** for MVP. Any platform fees on sales would need to be implemented externally.
    *   Shop creation and listing are free for MVP to encourage participation.
*   **NFT Interaction:** Uses the `crittercraft-traits::NftManager` for all NFT-related operations. The security of these interactions depends on the robust implementation of this trait by `pallet-critter-nfts`, including thorough ownership and state checks.

    #### Conceptual User Interface for User Shops
    ... (content as before) ...

## 2. Advanced Blockchain Support Jobs
... (content as before) ...
## 3. Quest System (`pallet-quests`)
...
    *   **`pallet-critter-nfts` (via `T::NftChecker` trait implementing `crittercraft_traits::QuestNftRequirementChecker`):** To verify pet-related quest criteria. This clear interface allows `pallet-quests` to request NFT data validation (like ownership, level) without direct dependency on `pallet-critter-nfts` internals, supporting modularity and ensuring that quest conditions are checked against the authoritative and secure NFT state provided by `pallet-critter-nfts`.
    *   **`pallet-items` (via `T::ItemChecker` trait implementing `crittercraft_traits::QuestItemRequirementChecker`):** To verify and consume items. This decouples quest logic from item management specifics, ensuring item state (ownership, existence) is handled by its authoritative pallet (`pallet-items`) in a synchronized and secure manner.
    *   **`pallet-user-profile` (via `T::UserProfileChecker` trait implementing `crittercraft_traits::QuestUserProfileRequirementChecker`):** To verify user-specific stats. This decouples quest logic from user profile details.
...
## 4. Treasure Hunts & Exploration
... (content as before) ...
## 5. Mini-Games & Arcade
... (content as before) ...
## 6. IoT Device Integration
... (content as before) ...

## 7. Pet NFT Charter Attributes (Foundational Traits)

Charter Attributes are the foundational traits that define the core essence, potential, and uniqueness of a Pet NFT from the moment of its creation (minting). Some are directly stored and immutable, while others can be conceptually derived from the pet's "genetic code." Their clear and unambiguous purpose is key to maintaining data integrity and a stable foundation for all pet-related logic.

### Core On-Chain Charter Attributes in `pallet-critter-nfts`:

The `PetNft` struct includes the following explicit charter attributes, which are set at minting and are immutable thereafter, defining the pet's core, unchangeable identity:

1.  **`initial_species: Vec<u8>`**: Represents the pet's immutable biological or constructed classification (e.g., "RoboDog", "PixelCat"). This attribute is fundamental to its visual representation, thematic abilities, and potential unique interactions within the game world. Its immutability ensures a pet's core nature doesn't arbitrarily change. (Security: Input validated for length, e.g. via `BoundedVec` in `PetNft` struct or at extrinsic boundary).
2.  **`dna_hash: [u8; 16]`**: This is the pet's unique, unchangeable genetic fingerprint. It is generated once at minting using secure on-chain randomness (`T::PetRandomness`) and serves as the ultimate deterministic "seed" for deriving several other foundational charter attributes (like base stats and elemental affinity). This ensures each pet's inherent uniqueness is cryptographically secured and provides a consistent, reproducible basis for its core characteristics.
3.  **`base_strength: u8`**: The pet's innate, fixed baseline for physical power, determined once from its `dna_hash` at minting. This value represents the pet's raw, untrained physical potential and forms the non-variable component of future attack power calculations, ensuring a clear distinction between inherent ability and trained skill.
4.  **`base_agility: u8`**: The pet's innate, fixed baseline for nimbleness and reaction time, determined once from its `dna_hash` at minting. This signifies its natural quickness and forms the non-variable part of speed, evasion, and turn-order calculations.
5.  **`base_intelligence: u8`**: The pet's innate, fixed baseline for problem-solving, learning capacity, and potential for special abilities, determined once from its `dna_hash` at minting. It underpins a pet's capability in more complex interactions and the non-variable aspect of special attack/defense calculations.
6.  **`base_vitality: u8`**: The pet's innate, fixed baseline for health, endurance, and resilience, determined once from its `dna_hash` at minting. This is crucial for its survivability and forms the non-variable component of HP and physical defense calculations.
7.  **`primary_elemental_affinity: Option<ElementType>`**: The pet's unchanging core elemental nature (e.g., Fire, Water, Tech), derived from its `dna_hash`. This defines its fundamental strengths and weaknesses in elemental interactions, ensuring consistent combat and gameplay mechanics. The `ElementType` enum provides a clearly defined, limited set of possible affinities.

These attributes are algorithmically and deterministically derived from the `dna_hash` (and potentially `initial_species` for some aspects) during the `mint_pet_nft` extrinsic within `pallet-critter-nfts`. The illustrative derivation algorithm is detailed in that pallet's `lib.rs`. **Security Note on DNA Derivation:** The integrity of the CritterCraft ecosystem heavily relies on this derivation algorithm being strictly deterministic, robust against manipulation or predictable biases (to prevent users from easily generating "perfect" pets through off-chain prediction or exploiting the randomness source), and clearly defined. This ensures fair and consistent pet generation. These attributes, particularly the `dna_hash`, establish the **absolute canonical on-chain source of truth** for a pet's innate characteristics. Any off-chain systems, such as the AI Personality Engine or Battle Engine, MUST treat this on-chain data as immutable and the definitive basis for their logic, ensuring system-wide consistency and preventing desynchronization. The consistent use of `BoundedVec` for string-like inputs such as species and names (enforced by `T::MaxSpeciesNameLen`, `T::MaxPetNameLen` in `Config`, or by type constraints on extrinsic parameters) is a key security measure against data overflow and resource exhaustion attacks across the pallet.

### Scalability of Pet NFT Data
... (content as before) ...

### Extensibility for Future Iterations
... (content as before) ...

### Dynamic Attributes & Simplifications (Recap from Pet Development Lifecycle)
... (content as before, `personality_traits` uses `BoundedVec` which is good for security) ...

## 8. Pet Development Lifecycle (Conceptual)

Beyond the immutable charter attributes set at minting, Pet NFTs in CritterCraft are dynamic entities that grow and change based on time, owner interactions, and experiences. This lifecycle is primarily managed within `pallet-critter-nfts`, potentially influenced by `pallet-items`. This section outlines how these mechanics are designed to foster owner impact, pet growth, individual pet identity, and sustained engagement, while ensuring security and data integrity.

### a. Time-Based State Changes & Dynamic Attribute Simplification
*   **Concept & Simplification:** Pets' core needs (hunger, energy) are now primarily calculated **off-chain** for UI display, based on on-chain timestamps. Explicit on-chain ticking for these attributes is removed to simplify on-chain state.
    *   **Why this design (KISS - Keep it Clear & Lean)?** This approach keeps the core on-chain state lean and interactions clear, significantly reducing transaction costs and blockchain bloat that would arise from frequent, minor updates for every pet. It focuses on-chain data on what is essential for core logic, security, and infrequent significant state changes, while allowing UIs to present a *perception* of continuous aliveness and needs, crucial for the "living pet" feel and sustained daily engagement (checking in on your pet).
*   **On-Chain Timestamps:** The `PetNft` struct in `pallet-critter-nfts` stores:
    *   `last_fed_block: BlockNumberFor<T>`: Updated when `feed_pet` is successfully called.
    *   `last_played_block: BlockNumberFor<T>`: Updated when `play_with_pet` is successfully called. This also serves as a general "last care" timestamp.
    *   `last_state_update_block: BlockNumberFor<T>`: Tracks any significant on-chain interaction.
*   **Off-Chain Calculation:** ... (content as before) ...
*   **Simplified On-Chain Mood:** ... (content as before) ...
    *   **Neglect Check (`apply_neglect_check` extrinsic):** A simplified on-chain function `apply_neglect_check(origin, pet_id)` can be triggered. **Security & Validation:** `origin` must be signed (though any account can trigger it for any pet as a utility function); `pet_id` must be a valid, existing `PetNft`. The core logic relies on secure and reliable access to `frame_system::Pallet::<T>::block_number()`. If `current_block - pet.last_played_block > T::NeglectThresholdBlocks::get()`, `mood_indicator` decreases by `T::NeglectMoodPenalty::get()`.

### b. Interaction-Driven Development (`feed_pet`, `play_with_pet`, `update_pet_metadata`)
*   **Concept:** Owners actively develop their pets by interacting with them. These interactions are designed to be intuitive and directly reflect common pet care paradigms, reinforcing the owner's impact on their pet's well-being and progression, thus stimulating ongoing engagement. Core extrinsics in `pallet-critter-nfts` manage these state changes with clear responsibilities and necessary validations.
*   **Extrinsic Details & Security:**
    *   `mint_pet_nft(origin, species: Vec<u8>, name: Vec<u8>)`: **Security & Validation:** Critical input validations include: `origin` must be a signed account. The system must check that the account does not exceed `T::MaxOwnedPets`. Inputs like `species` and `name` must be validated against length limits defined in `Config` (e.g., `T::MaxSpeciesNameLen`, `T::MaxPetNameLen` - ideally these parameters are `BoundedVec` types at the extrinsic boundary) before being used to create the `PetNft` struct, preventing oversized data storage. The DNA generation process must use secure on-chain randomness (`T::PetRandomness`) to prevent user influence over core stat derivation.
    *   `feed_pet(origin, pet_id: PetId, food_item_id: ItemId)`: Its precise responsibility is to process a `ConsumableCare` item verified as "Food". **Security & Validation:** Critical validations include: `origin` is the signed owner of `pet_id`; `pet_id` exists and is not critically locked (e.g. for permanent deletion, though feeding might be benign); `food_item_id` is a valid item of the correct `ItemCategoryTag::Food` (this check is delegated to and robustly handled by the `ItemHandler` trait implementation in `pallet-items`). Proper error handling for these checks is vital.
        *   Calls `T::ItemHandler::consume_item_of_category(&owner, &food_item_id, FOOD_CATEGORY_TAG)`. This trait interaction ensures a **synchronized state change**: `pallet-items` first validates and consumes the item, and only upon its success does `pallet-critter-nfts` proceed to apply the feeding effects.
        *   Updates `pet.last_fed_block`, mood, XP, and calls `attempt_level_up`.
    *   `play_with_pet(origin, pet_id: PetId, toy_item_id: ItemId)`: Its precise responsibility is to process a `ConsumableCare` item verified as "Toy". **Security & Validation:** Similar to `feed_pet`, critical validations include: `origin` is signed owner; `pet_id` exists; `toy_item_id` is valid and of `ItemCategoryTag::Toy` (checked by `ItemHandler`).
        *   Similar flow, ensuring synchronized item consumption with `pallet-items`.
        *   Updates `pet.last_played_block`, mood, XP, and calls `attempt_level_up`.
    *   `update_pet_metadata(origin, pet_id: PetId, name: Option<Vec<u8>>, personality_traits: Option<BoundedVec<BoundedVec<u8, T::MaxTraitStringLen>, T::MaxPetPersonalityTraits>>)`: **Security & Validation:** `origin` must be the signed owner of `pet_id`, and `pet_id` must be valid. If `name` is provided, its length must be validated against `T::MaxPetNameLen` (ideally the `name` parameter itself is a `BoundedVec<u8, T::MaxPetNameLen>`). The `personality_traits` input is already a `BoundedVec`, inherently checking count against `T::MaxPetPersonalityTraits` and individual trait string lengths against `T::MaxTraitStringLen`. Further content sanitization for trait strings (e.g., disallowing control characters) could be considered if they are displayed raw in UIs, though not strictly an on-chain security issue if stored as opaque bytes. These bounded inputs are crucial for preventing data overflow and ensuring system stability.
    *   **GIGO Antidote Note:** By restricting `feed_pet` and `play_with_pet` to consume items only of specific `ItemCategoryTag`s via the `ItemHandler` trait, the system prevents the misuse of unrelated items for these actions.
    *   **Extensibility Note:** New, distinct types of pet interactions should preferably be introduced as new, focused extrinsics.
*   **Complex Item Effects:** Uses `NftManagerForItems` trait, maintaining separation of concerns.

### c. Leveling and Experience (XP)
*   **Concept:** Pets gain Experience Points (XP) through interactions like simplified feeding and playing (as above), completing quests (via `pallet-quests`), and winning battles (via `pallet-battles`). Leveling is a key mechanic for demonstrating progression, unlocking new capabilities (implicitly, by being a prerequisite for other systems like advanced battles, breeding, or certain quests), and visually representing a pet's growth and the owner's dedication, thereby sustaining engagement over the long term.
... (rest of content as before) ...
    ### d. Personality Trait Evolution (Driven by AI Personality Engine)
    *   **Concept:** ...
    *   **Engine Function:** ...
    *   **Integration with `pallet-critter-nfts` (Owner-Approved for MVP):**
        *   ...
        *   **Why this design (KISS - Iterate Intelligently, Integrate Intuitively & Secure the Solution)?** This owner-approved model via `update_pet_metadata` is chosen for MVP because it clearly upholds owner agency. It keeps complex AI logic off-chain. The `update_pet_metadata` extrinsic, with its strict input validations (e.g., for `personality_traits` using `BoundedVec` to check trait count and individual string lengths against `Config` limits, and potentially content sanitization if traits were more than opaque strings), acts as a **key security checkpoint**. It prevents the AI Personality Engine (even if compromised or producing unexpected suggestions) from directly injecting arbitrarily long, malformed, or excessive trait data onto the chain, safeguarding against data corruption and potential exploits of the personality system. This model also enhances player engagement by making them an active participant in their pet's personality development.
    *   **Data Sources for Engine:** ...
    *   **Impact:** This system aims to make pet personalities feel genuinely emergent, deeply personalized, and reflective of their unique journey and the owner's play style within CritterCraft, significantly boosting the sense of individual pet identity and fostering long-term emotional investment and engagement.

### e. Iterative Design and Integration (`pallet-critter-nfts`)
... (content as before, ending with the new sentence below) ...
This strategic use of a simple core, defined extension points, clear, trait-based interfaces, and robust validation enables `pallet-critter-nfts` to serve as a secure and flexible cornerstone of the CritterCraft ecosystem, supporting both an initial MVP and future iterative development that scales gracefully. **Crucially, specific and informative error handling (through the pallet's `Error<T>` enum) for all extrinsics and internal operations is fundamental to secure operation, making failure states clear and preventing ambiguous conditions that might otherwise be exploited.**

This simplified lifecycle focuses on essential on-chain updates (timestamps, mood boosts, XP) for core interactions, while enabling richer off-chain calculations for dynamic states like hunger and energy, reducing on-chain storage and transaction load for the MVP.

## 9. Item System (`pallet-items`)
...
### 6. Trait Interactions (Crucial for Decoupling)
*   **`crittercraft-traits::NftManagerForItems`:**
    *   ... This interaction promotes a synchronized update to the pet's state based on item use, with `pallet-critter-nfts` being the authority on applying changes to its managed NFTs, ensuring security and data integrity.
*   **`crittercraft-traits::BasicCareItemConsumer`:**
    *   ... This intuitive integration decouples pet care outcome logic in `pallet-critter-nfts` from item management details in `pallet-items`, ensuring a synchronized and secure two-step process (consume then affect) and allowing both to iterate intelligently.
*   **`crittercraft-traits::QuestItemRequirementChecker`:**
    *   ... This promotes modularity for both pallets and ensures quest item states are handled synchronously and securely by the authoritative `pallet-items`.
...
## 11. Pet Breeding & Genetics
...
### 7. Pallet & System Interactions (MVP Focus)
*   **`pallet-critter-nfts`:**
    *   Implements the `crittercraft-traits::NftBreedingHandler` trait. This interface allows `pallet-breeding` to securely access essential genetic information (`get_pet_simple_genetics`) and trigger the creation of new Pet NFTs (`mint_pet_from_breeding`) in a controlled manner. `pallet-critter-nfts` remains the sole authority on NFT minting and the derivation of charter attributes from a given DNA blueprint (which must be deterministic and robust against manipulation), ensuring consistency and **synchronized, secure creation of pets from breeding, based on the canonical DNA data**.
...
## 13. Competitive Pet Battles
...
    ### 1. Core Concepts (MVP Focus)
    *   ...
    *   **Pet Eligibility & Locking:** Pets must be eligible. `pallet-battles` uses the `crittercraft-traits::NftManager` (implemented by `pallet-critter-nfts`) to check pet eligibility (e.g., via `is_transferable`) and to explicitly `lock_nft` for battle. This ensures a pet cannot be simultaneously sold or bred. Upon battle conclusion, `unlock_nft` is called. This **synchronized locking mechanism, with `pallet-critter-nfts` as the authority on NFT state, is vital for maintaining consistent state** across the ecosystem and preventing race conditions or invalid operations, ensuring harmonious and secure interplay between different game features.
...
```
