# CritterCraft: Advanced Features & Gameplay Loops (Conceptual Outline)

This document provides a high-level conceptual outline for advanced economic loops and gameplay features planned for future stages of CritterCraft development, building upon the foundations of Stage 5.

## 1. Sophisticated User Shops

*   **Concept:** Allow players to set up their own persistent, customizable shops within the CritterCraft ecosystem to sell Pet NFTs, items (from a future Item Pallet), or even services.
*   **Pallet Interactions:**
    *   **`pallet-critter-nfts` (via `NftManager` trait):** To verify ownership of Pet NFTs being listed by a shop owner and to facilitate the transfer of NFTs upon a successful purchase.
    *   **`pallet-balances` (via `Currency` trait):** For handling PTCN payments from buyers to sellers.
    *   **New `pallet-user-shops` (or enhanced `pallet-marketplace`):**
        *   Manages shop creation, potentially minting "Shopfront NFTs" to represent shop ownership and allow customization.
        *   Handles shop inventory: tracking which items/NFTs are listed in which shop, their prices, and quantities.
        *   Stores shop metadata: name, description, category, custom display information (if any).
*   **Core On-Chain Logic/Data (for `pallet-user-shops`):**
    *   `ShopId`: A unique identifier for each shop, possibly linked to a `ShopfrontNftId`.
    *   `ShopDetails` struct: `owner (AccountId), name (Vec<u8>), description (Vec<u8>), metadata_uri (Option<Vec<u8>> for off-chain extended details/cosmetics)`.
    *   `ShopListings`: `StorageDoubleMap<ShopId, ListingType (enum: NFT/Item), ItemOrNftId, ListingDetails (price, quantity)>`. This allows a shop to list various types of assets.
    *   `ShopReputation`: `StorageMap<ShopId, (u32_positive_ratings, u32_negative_ratings)>`.
    *   **Extrinsics:**
        *   `create_shop(name, description, metadata_uri)`: Creates a new shop, potentially mints a Shopfront NFT to the caller.
        *   `set_shop_metadata(shop_id, name, description, metadata_uri)`: Allows owner to update shop details.
        *   `add_to_shop_inventory(shop_id, item_or_nft_id, listing_type, price, quantity)`: Lists an asset for sale in the shop. Requires ownership verification (e.g., NFT owned by shop owner, or items held by shop owner).
        *   `remove_from_shop_inventory(shop_id, listing_type, item_or_nft_id)`: De-lists an asset.
        *   `purchase_from_shop(shop_id, listing_type, item_or_nft_id, quantity)`: Orchestrates the purchase:
            1.  Verifies item/NFT is listed and available.
            2.  Transfers PTCN from buyer to seller (shop owner) via `T::Currency`.
            3.  Transfers NFT from seller to buyer via `T::NftHandler` (or item from seller to buyer via a future Item Pallet).
            4.  Updates shop inventory (e.g., removes/decrements quantity).
            5.  Emits `ItemSoldFromShop` event.
*   **Economic Model & Impact:**
    *   Fosters a player-driven economy, encourages entrepreneurship, and allows for specialized trading hubs.
    *   **PTCN Sink:** Provides more sinks/uses for PTCN through various fees.
    *   **Shop Creation/Maintenance Fees (Conceptual):** Creating a User Shop (perhaps by minting a 'Shopfront NFT') might incur a one-time PTCN fee. Alternatively, a small periodic rental fee could apply to keep a shop active, contributing to PTCN sinks.
    *   **Seller-Set Fees (Conceptual):** Shop owners might have the option to add a small additional percentage fee on their sales, on top of any platform-wide marketplace fee, allowing them to earn directly from their shop's transaction volume.
    *   **Platform Fees:**
        *   Sales through User Shops might also be subject to a general platform fee if they leverage common settlement infrastructure, or User Shops could have their own distinct fee structure.
        *   If a general `pallet-marketplace` is used, a configurable percentage-based transaction fee (`MarketplaceFeeRate`) may be applied to each sale. This fee will be directed to a designated `FeeDestination` (e.g., the on-chain Treasury or a burn address), as defined conceptually in `pallet-marketplace::Config`.

    #### 4. Conceptual User Interface for User Shops

    The "User Shops District" in the UI Wallet will be the central hub for player-to-player commerce beyond the general marketplace.

    *   **Browsing and Searching Shops (`#browse-user-shops`):**
        *   A main view will allow users to see a list of active user shops (`#user-shop-list`), potentially with featured shops or categories.
        *   A search bar (`#search-shops-input`) will enable finding shops by name or owner.
        *   Each shop in the list will display its name, owner, a brief description, and a "Visit Shop" button.

    *   **Viewing an Individual Shop (`#view-individual-shop`):**
        *   Clicking "Visit Shop" will navigate the user to a dedicated view for that shop.
        *   This view will display the shop's name (`#shop-name-display`), owner, and full description (`#shop-description-display`).
        *   A list (`#shop-item-list`) will show all items (Pet NFTs, future game items like food/clothes/equipment) currently for sale in that shop, along with their prices.
        *   Each item will have a "Buy Item" button, which would (conceptually) trigger the `purchase_from_shop` extrinsic. A status area (`#buy-from-shop-status`) provides feedback.
        *   A "Back to Shop Browser" button allows users to return to the main shop list.

    *   **Managing Own Shop (`#manage-my-shop`):**
        *   This section is for users who want to become shop owners.
        *   **Initial State:** If the user doesn't have a shop, a "Create My Shop!" button is shown.
        *   **Shop Dashboard:** Once a shop is created, this area displays the shop's status and provides access to management functions:
            *   "Customize Shop" button: Reveals a form (`#create-customize-shop-form`) to set/update shop name, description, and potentially other customization options (like a banner image URL). This would call an extrinsic like `create_shop` or `update_shop_metadata`.
            *   "Manage Inventory" button: Reveals a form (`#manage-inventory-form`) where shop owners can:
                *   Select Pet NFTs or other items from their personal inventory to list in their shop.
                *   Set prices for these items.
                *   Call an extrinsic like `add_item_to_shop`.
                *   View and manage items currently listed in their shop (`#my-shop-inventory-list`), with options to delist or change prices.
        *   Status paragraphs (`#save-shop-details-status`, `#add-item-status`, `#manage-my-shop-status`) provide feedback on shop management actions.

    This UI structure aims to support discovery, browsing, purchasing, and shop management within a player-driven economy.

## 2. Advanced Blockchain Support Jobs

*   **Concept:** Introduce a system for users to perform tasks beneficial to the ecosystem's health, data integrity, or community, earning PTCN or other rewards.
*   **Pallet Interactions:**
    *   **`pallet-balances` (via `Currency` trait):** For disbursing PTCN rewards upon job completion.
    *   **`pallet-collective` / `pallet-democracy`:** Potentially for approving new job types, validating complex/subjective job completions, or managing a budget for job rewards (e.g., from a Treasury).
    *   **New `pallet-jobs-board`:**
        *   Lists available jobs and their requirements.
        *   Manages job applications, submissions of work/proof.
        *   Triggers reward payouts upon verified completion.
*   **Core On-Chain Logic/Data (for `pallet-jobs-board`):**
    *   `JobId`: Unique identifier for each job.
    *   `JobDetails` struct: `creator (AccountId), title (Vec<u8>), description_hash (H256 for off-chain details), reward_ptcn (Balance), max_participants (Option<u32>), expiry_block (BlockNumber), required_proof_format (enum: None, Hash, URL), completion_oracle (Option<AccountId>)`.
    *   `AvailableJobs`: `StorageMap<JobId, JobDetails>`.
    *   `JobApplications`: `StorageMap<(JobId, AccountId), ApplicationStatus (enum: Applied, Accepted, Rejected)>`.
    *   `JobSubmissions`: `StorageMap<(JobId, AccountId), SubmissionData (e.g., Vec<u8> for proof_url or hash)>`.
    *   `ApprovedCompletions`: `StorageMap<(JobId, AccountId), ()>` to prevent duplicate rewards.
    *   **Extrinsics:**
        *   `post_job(title, description_hash, reward_ptcn, max_participants, expiry_block, proof_format, oracle)`: Callable by authorized accounts (e.g., council, admin) to create new jobs.
        *   `apply_for_job(job_id)`: User expresses interest.
        *   `submit_job_completion(job_id, submission_data)`: User submits proof of work.
        *   `review_job_submission_and_payout(job_id, worker_account, is_approved)`: Callable by the designated `completion_oracle` or a council. If approved, it triggers PTCN reward transfer and marks job as completed for that user.
*   **Oracle Integration:** For jobs requiring off-chain verification (e.g., "Did user X tweet about CritterCraft?"), a trusted oracle system (could be a centralized service initially, or a decentralized oracle network later) would be needed to validate completion and trigger the `review_job_submission_and_payout` extrinsic.
*   **Economic Impact:** Provides diverse earning opportunities beyond core gameplay, incentivizes ecosystem-beneficial activities, and allows for community-driven task fulfillment.

    #### 4. Conceptual User Interface for Jobs Board

    The "Jobs Board" in the UI Wallet will be the interface for users to find, apply for, and manage participation in advanced ecosystem support jobs.

    *   **Browsing Available Jobs (`#available-jobs`):**
        *   A list (`#job-list`) will display currently available jobs. Optional filters (by type, reward, duration) could be added.
        *   Each job listing will show key details: Title, brief description, core requirements, reward (PTCN and/or item NFTs), expiry/duration, and a "View Details & Apply" button.

    *   **Viewing Job Details (`#job-details-view`):**
        *   Clicking "View Details & Apply" would show a dedicated view for a specific job with its full description, detailed requirements, reward structure, and expiry information.
        *   An "Apply for this Job" button would allow users to (conceptually) signal their intent via an `apply_for_job` extrinsic.
        *   If a user has already applied or been accepted for a job, this section might change to show their current status (e.g., "Application Pending," "Job In Progress").
        *   For jobs "In Progress," a placeholder for "Submit Proof of Completion" (e.g., a text area for links/notes and a submit button) would be present, which would (conceptually) call a `submit_job_completion_proof` extrinsic.
        *   A "Back to Jobs List" button and a status area (`#job-action-status`) for feedback.

    *   **Managing Accepted Jobs (`#my-accepted-jobs`):**
        *   A list (`#my-jobs-list`) will display jobs the user has accepted and are currently "In Progress."
        *   Each entry would show the job title, its current status (e.g., "Proof Submitted, Awaiting Review," "Deadline Approaching"), and potentially a button to "Submit/Update Proof" if applicable.

    This UI aims to create a clear and actionable interface for users to engage with various roles and tasks that support the CritterCraft ecosystem, providing them with meaningful ways to contribute and earn.

## 3. Quest System (`pallet-quests`)

*   **Concept:** `pallet-quests` manages the creation, availability, and completion of in-game quests, providing players with objectives and rewards. This system will be enhanced with advanced on-chain verification criteria.
*   **Pallet Interactions (Enhanced):**
    *   **`pallet-critter-nfts` (via `T::NftChecker` trait):** To verify pet-related quest criteria (level, species, ownership).
    *   **`pallet-items` (via `T::ItemChecker` trait):** To verify and consume items required for quests.
    *   **`pallet-user-profile` (via `T::UserProfileChecker` trait):** To verify user-specific stats (e.g., battles won, reputation) as prerequisites.
    *   **`pallet-balances` (via `Currency` trait):** For disbursing PTCN quest rewards.
*   **Core On-Chain Logic/Data (Enhancements):**
    *   **`Quest` Struct & Advanced Criteria:**
        *   The `Quest` struct will be enhanced to include optional fields for defining diverse on-chain verifiable completion criteria:
            *   `description: Vec<u8>`, `reward_ptcn: BalanceOf<T>` (existing)
            *   **Pet-Specific Criteria:**
                *   `required_pet_level: Option<u32>` (e.g., "Target Pet must be level 10").
                *   `required_pet_id_for_level_check: Option<PetId>` (Specifies which pet if quest is about a particular one. If `None`, the user might need to specify an eligible pet they own when attempting completion via `maybe_target_pet_id` in `complete_quest`).
                *   `required_pet_species: Option<PetSpeciesType>` (e.g., "Must use a RoboDog for this task").
                *   `required_pet_id_for_species_check: Option<PetId>`.
            *   **Item-Specific Criteria (interacts with `pallet-items`):**
                *   `required_item_id: Option<ItemId>`.
                *   `required_item_quantity: Option<u32>`.
                *   `consume_item_on_completion: bool` (determines if the item is taken from inventory).
            *   **User Profile Criteria (interacts with `pallet-user-profile`):**
                *   `min_battles_won_for_user: Option<u32>`.
                *   `min_trade_reputation_for_user: Option<i32>`.
        *   Not all quests will use all criteria; many can remain simple "task completion" quests.
    *   **Extrinsic Updates & Verification Logic:**
        *   **`add_quest` (Admin):** Will be updated to allow setting these new optional criteria fields when defining a quest.
        *   **`complete_quest` (User):**
            *   The user might pass an optional `maybe_target_pet_id` if the quest requires action with one of their pets but doesn't specify *which* one in its own definition.
            *   Before distributing rewards, the extrinsic will fetch the `Quest` details and perform **On-Chain Verification** by calling handler traits:
                *   `T::NftChecker` (implemented by `pallet-critter-nfts`): To verify ownership, level, and species of the relevant pet.
                *   `T::ItemChecker` (implemented by `pallet-items`): To verify the user possesses required items and to consume them if necessary (based on `consume_item_on_completion` flag).
                *   `T::UserProfileChecker` (implemented by `pallet-user-profile`): To verify user statistics like battle wins or trade reputation.
            *   Only if all defined criteria for that specific quest are met will the completion proceed.
            *   Appropriate error messages (e.g., `PetLevelTooLow`, `RequiredItemMissing`, `NotEnoughBattlesWon`) will guide the user if criteria are not met.
            *   Upon successful completion, it will also call the `record_quest_completion` function in `pallet-user-profile` to update the user's profile score.
*   **Economic Impact:** Quests serve as a primary mechanism for PTCN distribution (rewards) and can also act as item sinks if `consume_item_on_completion` is true. Complex quests requiring rare items or high pet/user stats can drive demand for those prerequisites.

    This makes quests more integrated with other systems and allows for more varied and challenging objectives.

## 3. Treasure Hunts & Exploration
    // Re-numbering this to keep its original content but adjust its position if necessary.
    // For this diff, we assume the Quest System replaces the old Treasure Hunts placeholder,
    // and Treasure Hunts might become a specialized type of quest or a separate feature later.
    // If Treasure Hunts is meant to be kept as a distinct H2, this diff needs adjustment.
    // For now, this effectively replaces the old H2 "3. Treasure Hunts & Exploration"
    // with the detailed "3. Quest System (`pallet-quests`)".
    // The original "Treasure Hunts & Exploration" content will be removed by this search/replace.
    // If that's not desired, the search block needs to be more specific.
    // Based on the prompt, we are updating the "Quest System", and the old section 3 was "Treasure Hunts".
    // This implies Section 3 is being repurposed for the detailed Quest System.

*   **Concept:** Complex, multi-stage quests or puzzles with significant rewards, potentially involving both on-chain actions and off-chain elements.
*   **Pallet Interactions:**
    *   **`pallet-critter-nfts` (via `NftManager` trait):** To verify ownership of specific NFTs required to start or progress in a hunt, or to mint/reward unique NFT prizes.
    *   **`pallet-items` (Future):** If consumable items or equipment are needed for puzzles or are part of the reward.
    *   **`pallet-balances` (via `Currency` trait):** For PTCN rewards.
    *   **New `pallet-treasure-hunts` (or significantly enhanced `pallet-quests`):**
        *   Manages hunt definitions, player progress, and reward distribution.
*   **Core On-Chain Logic/Data (for `pallet-treasure-hunts`):**
    *   `HuntId`: Unique identifier for each treasure hunt.
    *   `HuntDetails` struct: `creator (AccountId), name (Vec<u8>), description_hash (H256), total_stages (u8), reward_nft (Option<CollectionId, ItemId>), reward_ptcn (Balance), start_block (Option<BlockNumber>), end_block (Option<BlockNumber>)`.
    *   `HuntStages`: `StorageDoubleMap<HuntId, StageIndex (u8), HuntStageDetails>`.
        *   `HuntStageDetails` struct: `description_hash (H256), puzzle_type (enum: OnChainAction, OffChainOracle, CodeRedemption), puzzle_data (Vec<u8> for on-chain checks or oracle instructions), solution_verifier (Option<AccountId> for oracle stages)`.
    *   `PlayerHuntProgress`: `StorageDoubleMap<AccountId, HuntId, CurrentStageIndex (u8)>`.
    *   `PlayerStageAttempts`: `StorageNMap<(AccountId, HuntId, StageIndex), AttemptCount (u32)>` (to limit brute-forcing if needed).
    *   **Extrinsics:**
        *   `create_hunt(name, description_hash, stages_data, reward_nft, reward_ptcn, start_block, end_block)`: Admin/creator function.
        *   `start_hunt(hunt_id)`: Player enrolls in a hunt.
        *   `submit_stage_solution(hunt_id, stage_index, solution_data)`: Player submits solution for a stage. Pallet logic verifies `solution_data` against `HuntStageDetails.puzzle_data` or defers to oracle. If correct, updates `PlayerHuntProgress`.
        *   `claim_final_hunt_reward(hunt_id)`: Callable after completing the final stage. Verifies progress and distributes rewards.
*   **Oracle/Off-Chain Components:** For stages requiring external verification (e.g., "Visit website X and find a code"), an oracle or a secure off-chain mechanism would be needed to confirm completion and potentially call back to the pallet.
*   **Economic Impact:** Creates engaging, high-value player experiences, drives community interaction around solving complex puzzles, and can be used to highlight specific platform features or partners. Rewards can be significant PTCN sinks or distributors of rare NFTs.

    #### 4. Conceptual User Interface for Treasure Hunts

    The "Treasure Hunts" section in the UI Wallet will be the portal for players to discover, participate in, and track their progress on complex adventures.

    *   **Browsing Available Hunts (`#available-hunts`):**
        *   A list (`#hunt-list`) will display currently available treasure hunts.
        *   Each listing will show: Hunt Title/ID, a brief description, estimated difficulty, key potential rewards (e.g., unique NFTs, large PTCN sums), and a "View Details / Start Hunt" button.

    *   **Active Hunt Details & Progress (`#my-active-hunt-details`):**
        *   Once a user starts a hunt (or clicks "View Details" on an active one), this section becomes visible.
        *   It will display the active hunt's title (`#active-hunt-title`).
        *   The current stage number and the clue or task for that stage (`#hunt-current-stage-clue`) will be shown.
        *   A progress indicator (e.g., "Stage X of Y").
        *   An input area (`#hunt-solution-submission`) for the user to submit their solution or proof of completion for the current stage. This might be a text field for answers, a place to paste a transaction hash (if an on-chain action was required), or conceptually, a file upload for off-chain proof.
        *   A "Submit Solution" button would (conceptually) call an extrinsic like `submit_hunt_stage_solution`.
        *   A status area (`#hunt-action-status`) for feedback on submissions.
        *   A "Back to Hunts List" button.

    *   **Viewing Completed Hunts (`#completed-hunts-section`):**
        *   A list (`#completed-hunts-list`) will display treasure hunts the user has successfully completed.
        *   Each entry would show the hunt title/ID, completion date, and a summary of significant rewards claimed.

    This UI aims to guide players through multi-stage treasure hunts, from discovery to completion, making the process engaging and clear.

## 4. Mini-Games & Arcade

*   **Concept:** A suite of on-chain or on-chain verifiable mini-games, allowing players to compete for fun and PTCN/item rewards.
*   **Pallet Interactions:**
    *   **`pallet-balances` (via `Currency` trait):** For handling entry fees (if any) and distributing prize payouts.
    *   **`pallet-randomness` (e.g., `RandomnessCollectiveFlip`):** Essential for games involving elements of chance (dice rolls, card draws).
    *   **New `pallet-arcade` (as a central hub) or individual mini-game pallets (e.g., `pallet-rps-game`):**
        *   `pallet-arcade` could manage game listings, player sessions, and global leaderboards.
        *   Individual game pallets would implement the specific logic for each game.
*   **Core On-Chain Logic/Data (Example: Rock-Paper-Scissors Game in `pallet-rps-game`):**
    *   `GameId`: Unique identifier for each RPS game instance.
    *   `RPSChoice` enum: `Rock, Paper, Scissors`.
    *   `RPSGameDetails` struct: `player1 (AccountId), player1_commit (H256 for choice + salt), player2 (Option<AccountId>), player2_choice (Option<RPSChoice>), entry_fee (Balance), prize_pool (Balance), status (enum: AwaitingPlayer2, AwaitingRevealP1, AwaitingRevealP2, Concluded), winner (Option<AccountId>)`.
    *   `ActiveRPSGames`: `StorageMap<GameId, RPSGameDetails>`.
    *   `NextGameId`: `StorageValue<_, GameId, ValueQuery>`.
    *   **Extrinsics:**
        *   `create_rps_game(commitment: H256, entry_fee: Balance)`: Player 1 creates a game, committing to a choice (hash of choice + salt). Entry fee moved to prize pool or held by pallet.
        *   `join_rps_game(game_id: GameId, player2_choice: RPSChoice)`: Player 2 joins an existing game, submitting their choice directly. Entry fee moved.
        *   `reveal_rps_choice(game_id: GameId, player1_choice: RPSChoice, salt: Vec<u8>)`: Player 1 reveals their choice and salt. Pallet verifies commitment, determines winner, distributes `prize_pool` via `T::Currency`, and updates game status.
    *   **Leaderboards (in `pallet-arcade` or game pallet):**
        *   `GameLeaderboard`: `StorageMap<GameTypeEnum, BoundedVec<(AccountId, u32_score), MaxLeaderboardSize>>`.
*   **Economic Impact:** Provides entertainment and social interaction, additional PTCN sinks (entry fees) and sources (prize pools), and can encourage strategic play.

    #### 5. Conceptual User Interface for Mini-Games & Arcade

    The "Arcade & Mini-Games" section in the UI Wallet will serve as a hub for various casual and competitive games integrated with CritterChain.

    *   **Game Gallery/List (`#game-gallery`):**
        *   A list or grid (`#game-list`) will display available mini-games.
        *   Each entry will show: Game Title, Type (e.g., "On-Chain Turn-Based," "Off-Chain Skill-Based"), Entry Fee (if any), Potential Rewards, and a "Play Game" button.
        *   "Play Game" buttons might either navigate to an active game area within the UI or link to an external web page/application for off-chain games.

    *   **Active Game Area (`#active-game-area`):**
        *   This section becomes visible when a user selects a game.
        *   **For On-Chain Games:** It would contain the necessary UI elements for game interaction (e.g., buttons for choices in Rock-Paper-Scissors, input fields for guessing games). A status area (`#game-action-status` or game-specific status) provides feedback.
        *   **For Off-Chain Games:** It might display an embedded `iframe` for web-based games, or provide clear instructions and a link to launch the game. It would also conceptually include how scores are reported back (e.g., user submits score with proof, or game server reports to an oracle).
        *   A "Back to Games List" button allows users to exit the active game view.

    *   **Leaderboards (`#leaderboards-section`):**
        *   Users can select a game from a dropdown (`#leaderboard-game-select`) to view its leaderboard (`#leaderboard-list`).
        *   The list would display top players and their scores or rankings.

    *   **Arcade Rewards (`#arcade-rewards-section`):**
        *   This area will display any claimable rewards the user has earned from mini-game achievements or leaderboard payouts (e.g., PTCN, Arcade Tickets - a conceptual secondary currency, or specific item NFTs).
        *   A "Claim Rewards" button (`#claimMiniGameRewardsButton`) would (conceptually) trigger an extrinsic to claim all pending mini-game rewards.

    This UI aims to provide easy access to a variety of games, manage participation, track performance, and claim earned rewards, fostering a fun and engaging aspect of the CritterCraft ecosystem.

## 5. (Future Consideration) IoT Device Integration

*   **Concept:** Allow real-world data from Internet of Things (IoT) devices (e.g., smart pet feeders, activity trackers for real pets if users have them and want to link) to influence in-game pet stats or trigger on-chain events/rewards.
*   **Technical Challenges:** Requires secure oracle solutions to bring IoT data on-chain reliably.
*   **Impact:** Blurs the lines between the virtual and real world, offering unique engagement. This is a highly speculative and long-term idea.

These advanced features aim to create a deeply engaging, economically vibrant, and evolving world for CritterCraft players. Each will require careful design and phased implementation.

## 6. Pet NFT Charter Attributes (Foundational Traits)

Charter Attributes are the foundational traits that define the core essence, potential, and uniqueness of a Pet NFT from the moment of its creation (minting). Some are directly stored and immutable, while others can be conceptually derived from the pet's "genetic code."

### Core On-Chain Charter Attributes in `pallet-critter-nfts`:

The `PetNft` struct now includes the following explicit charter attributes, set at minting and immutable thereafter:

1.  **`initial_species: Vec<u8>`**: Defines the pet's species, influencing appearance and potential base abilities.
2.  **`dna_hash: [u8; 16]`**: A unique cryptographic hash, serving as the pet's core genetic code. It's used to deterministically derive other charter attributes.
3.  **`base_strength: u8`**: The pet's innate base strength.
4.  **`base_agility: u8`**: The pet's innate base agility.
5.  **`base_intelligence: u8`**: The pet's innate base intelligence.
6.  **`base_vitality: u8`**: The pet's innate base vitality.
7.  **`primary_elemental_affinity: Option<ElementType>`**: The pet's primary elemental type (e.g., Fire, Water, Tech, Neutral/None). The `ElementType` enum includes variants like `Neutral, Fire, Water, Earth, Air, Tech, Nature, Mystic`.

These attributes are algorithmically determined from the `dna_hash` (and potentially `initial_species`) during the `mint_pet_nft` extrinsic.
A more detailed illustrative derivation algorithm (using byte pairs from `dna_hash` and scaling for base stats, and modulo operations for elemental affinity) is now conceptually outlined within the `mint_pet_nft` function in `blockchain_core/pallets/critter_nfts_pallet/src/lib.rs`.

### Further Derived or Gameplay-Relevant Attributes (from `dna_hash` & On-Chain Base Stats):

While the above are stored on-chain, the `dna_hash` and explicit base stats can still be used to imply or derive further nuanced attributes for off-chain game logic (e.g., in the Python MVP or a future game server) or more complex on-chain systems:

*   **Stat Growth Potential:** How base stats influence the potential maximums or growth curves for dynamic stats like current strength (derived from base + level + items).
*   **Secondary Elemental Affinities/Resistances:** More detailed elemental interactions.
*   **Hidden Talents/Abilities:** Specific rare abilities that might only become apparent or unlockable if certain base stats or affinities are present.
*   **Cosmetic Trait Predispositions:** The `dna_hash` could still influence rare base patterns or color variations not covered by dynamic cosmetics.
*   **Breeding Values:** These on-chain charter attributes will be critical inputs for the genetic algorithm in the future Pet Breeding system, determining the potential traits of offspring.

This combination of explicit on-chain charter attributes and the richer information derivable from the `dna_hash` provides a robust foundation for unique, developable, and breedable Pet NFTs.

    ### 7. Staking UI V2 - Enhanced Interactions (Conceptual)

    Building upon the initial staking UI, future enhancements will provide a more comprehensive and interactive experience, reflecting the detailed NPoS mechanics outlined in `CONSENSUS_MIGRATION.md`.

    #### a. Network Overview Display (`#staking-overview-info`)
    *   **Current Era & Session:** Display current era index (`#current-era-info`), a visual progress bar or percentage for era completion (`#era-progress-info`), estimated time/blocks to next era (`#next-era-eta`), current session index within the era (e.g., "3 of 6" in `#current-session-info`), and estimated time/blocks to next session (`#next-session-eta`).
    *   **Network Stats:** Show total PTCN staked on the network (`#total-network-stake`) and the count of active validators versus the maximum allowed (e.g., "Active Validators: 65 / 75" using `#active-validator-count` and `#max-validator-count-const`).

    #### b. My Staking Dashboard Enhancements (`#my-staking-info`)

    *   **Total Staked:** Clear display of the user's total active bonded PTCN (`#my-staked-amount`).
    *   **Bond More PTCN:** An input field (`#bond-extra-amount`) and button (`#bondExtraButton`) to (conceptually) call `staking::bond_extra`.
    *   **Nomination Management:**
        *   Display current nominations (e.g., list of validator names/addresses in `#my-nominations-display`) and current nomination count vs max allowed (e.g., "My Nominations (3/16)" using `#nomination-count-display` and `#max-nominations-const-display`).
        *   "Change/Set Nominations" button (`#changeNominationsButton`): Toggles visibility of a dedicated area (`#nomination-management-area`). This area would feature:
            *   A list of all available validators (`#nomination-validator-list-area`), potentially with checkboxes or a multi-select interface, allowing users to pick up to the maximum number of nominations (text like `#max-nominations-const-form-display` indicating max).
            *   A "Submit New Nominations" button (`#submitNewNominationsButton`) to (conceptually) call `staking::nominate`.
            *   A "Cancel" button (`#cancelNominationChangeButton`).
        *   "Stop Nominating (Chill)" button (`#chillNominationsButton`): (Conceptually) calls `staking::chill`. UI should provide feedback on what this means (e.g., "You will stop nominating and receiving rewards from the next active era. Your funds will remain bonded until you unbond them.").
    *   **Rewards & Payouts:**
        *   Display estimated time until next potential payout cycle (`#next-payout-eta-display`).
        *   "Claimable Payouts" span (`#claimable-rewards-display`): Text might indicate "Querying...", "None pending", or "X PTCN available".
        *   "Claim All My Payouts" button (`#claimPayoutsButton`): (Conceptual) Would trigger `payout_stakers` for all eligible past eras/validators for the user. The UI will explain standard payout mechanisms.
        *   A scrollable list (`#historical-rewards-list`) to display recent (e.g., last 5-10) reward payouts credited to the user, showing era, validator (if applicable), and amount.
    *   **Unbonding & Stake Management (`#unbonding-info`):**
        *   Display `BondingDuration` (`#bonding-duration-info-display`) dynamically from chain constants if possible, or as configured.
        *   Input (`#unbond-amount`) and button (`#unbondButton`) for `staking::unbond`.
        *   List of `unlocking` chunks (`#unbonding-chunks-list`) with amounts and withdrawable era.
        *   Input (`#rebond-amount`) and button (`#rebondButton`) for `staking::rebond`.
        *   "Withdraw All Unlockable PTCN" button (`#withdrawUnbondedButton`) for `staking::withdraw_unbonded`, which becomes enabled only when withdrawable funds exist.
        *   Status feedback in `#unbond-status`.
        *   Clear slashing warnings with a clickable element (`#view-slashing-incidents-link`) to toggle visibility of `#slashing-info-display`.
    *   **Slashing Information Display (`#slashing-info-display`):**
        *   A (normally hidden) section to display details if any of the user's nominated validators were slashed, or if the user (as a validator) was slashed. Information would include era of slash, validator involved, and amount slashed.

    #### c. Validator List Enhancements (`#validator-list`)
    *   (As previously documented) Detailed stats, filtering, sorting. The nomination UI within `#nomination-management-area` would likely leverage an enhanced version of this validator list.

    #### d. Becoming a Validator (Informational)
    *   (As previously documented) More detailed guides and links.

    These V2 UI enhancements aim to provide a comprehensive and user-friendly interface for all common staking operations and information needs.

## 7. Competitive Pet Battles

CritterCraft will feature a robust system for competitive pet battles, where Pet NFTs engage in strategic combat. `pallet-battles` is the core on-chain component for managing battle registration, state, and outcomes.

    ### 1. Core Concepts (Recap from pallet-battles design)
    *   **Registration:** Players register one of their eligible Pet NFTs for battle.
    *   **Matchmaking:** (Conceptual) Initially, this might be a simple queue or challenge system. Future enhancements could involve ELO ratings or level-based brackets.
    *   **Outcome Reporting:** A trusted source (e.g., player 1 in MVP, oracle, or agreed-upon off-chain simulation result) reports the winner.
    *   **Rewards:** PTCN distributed to the winner.

    ### 2. Core On-Chain Logic/Data (`pallet-battles`)
    *   **`BattleDetails` Struct:** Stores `player1`, `pet1_id`, `player2` (Option), `pet2_id` (Option), `status` (`PendingMatch`, `Concluded`), `winner` (Option).
    *   **`PetInBattle` Storage:** Tracks if a pet is currently in an active battle to prevent multiple registrations.
    *   **`report_battle_outcome` Extrinsic:**
        *   Receives the outcome (e.g., `winner_pet_id`).
        *   Verifies participants and battle status.
        *   Updates `BattleDetails` (status to `Concluded`, records winner).
        *   Distributes PTCN rewards to the winner's owner via `T::Currency`.
        *   (Future Synergy) Could mint Battle Trophy NFTs or update win/loss counters on Pet NFTs (via `T::NftHandler`) or User Profiles (via `pallet_user_profile`).
        *   Emits `BattleConcluded` event.

    ### 3. Conceptual Battle Mechanics & Formulas (Inputs to Off-Chain Simulation / Advanced On-Chain Logic)

    The actual battle simulation (determining the winner based on pet stats and abilities) is conceptually complex and likely performed off-chain initially, with the result submitted to `report_battle_outcome`. However, the design of on-chain pet attributes directly informs this simulation.

    *   **Input Pet Data:** For each participating pet, the simulation would require:
        *   All on-chain **Charter Attributes** from `PetNft` (`base_strength`, `base_agility`, `base_intelligence`, `base_vitality`, `primary_elemental_affinity`).
        *   Current dynamic stats (`level`, `mood_indicator`, `energy_status` - which might influence starting HP or morale).
        *   Current `personality_traits`.
        *   Effects of any **equipped or consumed items** (from `pallet-items`), providing temporary or persistent boosts to attributes. This implies `pallet-critter-nfts` (via `NftHandler`) needs to expose "effective" stats or a way to query active item effects.
    *   **Effective Combat Stats:** The simulation would calculate `effective_attack`, `effective_defense`, `effective_speed`, and `current_hp` (hit points for the battle instance, derived from vitality, level, items) for each pet based on the above inputs.
    *   **Turn-Based Logic (Conceptual Example):**
        *   **Attack Order:** Determined by `effective_speed`, with randomness (from a shared seed provided to the simulation) for ties.
        *   **Actions per Turn:** (Future) Pets could have multiple abilities/moves. Initially, a basic "Attack" action.
        *   **Hit Chance:** Based on attacker's accuracy (e.g., derived from `effective_agility` or `effective_intelligence`) vs. defender's evasion (e.g., derived from `effective_agility`). Example: `hit_chance = (attacker_accuracy / defender_evasion) * base_hit_percentage`.
        *   **Damage Calculation:** If a hit occurs, damage could be: `(Attacker_Effective_Attack * Skill_Power_Factor) - (Defender_Effective_Defense * Mitigation_Factor)`. This is then modified by:
            *   **Elemental Modifiers:** Based on `primary_elemental_affinity` matchups (e.g., Fire vs. Nature: Fire deals 1.5x, Nature deals 0.75x to Fire). A matrix of elemental advantages/disadvantages would be defined.
            *   **Critical Hits:** A chance (e.g., based on `effective_agility` or a dedicated "luck" stat) to deal increased damage (e.g., 1.5x).
            *   **Randomness:** Slight variations (e.g., +/- 10%) in final damage output.
        *   **Personality Trait Influence:** Specific traits could provide situational bonuses/penalties (e.g., "Brave" pets getting an attack boost at low HP; "Timid" pets having a chance to "flinch" and miss a turn if hit critically).
    *   **Winning Condition:** Battle ends when one pet's HP reaches zero. If a maximum number of turns is reached, conditions like higher remaining HP percentage could determine the winner.
    *   **Battle Log:** The simulation should produce a detailed log of actions, damage, and random rolls. A hash of this log can be submitted with the battle outcome for potential dispute resolution or verification.

    This detailed interaction of on-chain attributes and a robust (even if initially off-chain) simulation allows for complex and strategic battles while keeping heavy computation off the main chain. The on-chain data remains the source of truth for pet capabilities.

## 9. Future Stage: Pet Breeding & Genetics
## 9. Future Stage: Pet Breeding & Genetics

A comprehensive Pet Breeding and Genetics system is envisioned as a major future stage for CritterCraft, adding significant depth to pet collection, strategy, and the in-game economy. This system will allow players to breed their Pet NFTs to create new, potentially unique offspring.

### 1. Core Breeding Mechanic

*   **Concept:** Owners of two compatible Pet NFTs (see "Compatibility" below) can choose to breed them together to produce a new Pet NFT (an "egg" or "newborn" pet).
*   **Pallet Interaction/New Pallet (`pallet-breeding` or extend `pallet-critter-nfts`):**
    *   An extrinsic like `initiate_breeding(origin, parent1_pet_id: PetId, parent2_pet_id: PetId, fertility_item_id: Option<ItemId>)` would be called.
    *   The pallet would verify ownership of parent pets, check compatibility, consume any fertility items, and potentially place parent pets into a temporary "breeding cooldown" status.
    *   A new Pet NFT would be minted, its charter attributes and initial state determined by the genetic inheritance logic.
*   **Breeding Cooldowns:** Pets may have cooldown periods after breeding before they can breed again to prevent overpopulation and add strategic depth.

### 2. Genetic Inheritance & Charter Attributes

    *   **Foundation:** The explicit on-chain Charter Attributes (`base_strength`, `base_agility`, `base_intelligence`, `base_vitality`, `primary_elemental_affinity`) and the `dna_hash` of parent pets (from `pallet-critter-nfts`) will be fundamental inputs to the genetic algorithm within `pallet-breeding`.
    *   **Algorithm (Conceptual within `initiate_breeding`):**
        *   **Offspring Species:** Determined by a combination of parent species. For same-species parents, offspring is typically the same. For cross-species (if allowed), rules would define outcome (e.g., 50/50 chance, specific hybrid type, influenced by fertility items).
        *   **Offspring DNA Hash:** A new, unique `dna_hash` will be generated for the offspring, algorithmically derived from the parents' `dna_hash` values and incorporating randomness via `T::RandomnessSource` to ensure uniqueness and variation.
        *   **Offspring Charter Attributes (Base Stats & Affinity):** Each base stat (strength, agility, intelligence, vitality) for the offspring will be calculated based on an average or weighted average of the parents' corresponding base stats, with a small random variation (+/-) applied using `T::RandomnessSource`. The elemental affinity will also be inherited with chances based on parental affinities and potential for random mutation to `None` (Neutral) or another element.
        *   **Fertility Item Influence:** Consumable fertility items (from `pallet-items`) can be used during `initiate_breeding` to provide boosts or influence probabilities within the genetic algorithm (e.g., increasing the chance of inheriting higher stats, a specific elemental affinity, or a rarer species in cross-breeds).
    *   **Storage of Determined Traits:** The `pallet-breeding` will store these determined traits (species, DNA hash, base stats, affinity) for the pending offspring (e.g., in an `OffspringDetails` struct) until it is "claimed" or "hatched."
    *   **Minting Offspring:** The `claim_offspring` extrinsic in `pallet-breeding` will then use these stored, pre-determined charter attributes to mint the new Pet NFT by calling a specialized minting function on `pallet-critter-nfts` (via its `NftManager` trait). This ensures the offspring's foundational traits are set according to the breeding outcome.

### 3. Breeding Scores / Genetic Fitness

*   **Concept:** Each Pet NFT might have one or more "Breeding Scores" or a "Genetic Fitness" rating.
*   **Influence:** This score could influence:
    *   The probability of successful breeding.
    *   The quality or rarity of offspring (e.g., higher chance of inheriting desirable traits or mutations).
    *   The length of breeding cooldowns.
*   **Derivation:** Breeding scores could be derived from a combination of factors: pet's level, rarity of its own charter attributes, lineage (number of successful offspring), or specific "champion" bloodlines.

### 4. Breeding Tree / Lineage Tracking

*   **Concept:** The `PetNft` struct in `pallet-critter-nfts` would be extended to store lineage information.
*   **Implementation:**
    *   `parent1_id: Option<PetId>`
    *   `parent2_id: Option<PetId>`
    *   `generation_number: u32`
*   **UI Impact:** The UI Wallet could display a visual breeding tree for each pet, allowing players to trace ancestry and identify valuable bloodlines. This adds significant collectible value.

### 5. Cross-Species Breeding

*   **Concept (Advanced/Exploratory):** Allow breeding between different (but perhaps related or compatible) pet species.
*   **Outcome:**
    *   Could result in hybrid species with unique appearances and trait combinations.
    *   Might have lower success rates or require specific conditions/items.
*   **Complexity:** Adds significant complexity to the genetic algorithm and species definition but offers immense variety.

### 6. Fertility Items & Consumables

*   **Concept:** Introduce consumable items that can influence the breeding process.
*   **Examples (managed by a future `pallet-items`):**
    *   **Fertility Boosters:** Increase the chance of successful breeding or reduce cooldown times.
    *   **Trait Enhancers:** Slightly increase the chance of passing on a specific desirable trait from a parent.
    *   **Species Compatibility Charms:** Items that might enable or improve success rates for cross-species breeding attempts.
*   **Economic Model & Impact:**
    *   **Breeding Fees:** A flat PTCN fee (`BreedingFee`), configurable conceptually in `pallet-breeding::Config`, may be required to initiate a breeding attempt. This fee could contribute to the Treasury or a specific reward pool, acting as a PTCN sink and value recirculation mechanism.
    *   **Fertility Items:** The use of consumable fertility items (purchased from shops or earned) creates an economic sink for those items and a revenue stream for their sellers/creators, further stimulating the economy.

### 7. Pallet & System Interactions

*   **`pallet-critter-nfts`:** Core for storing pet data (including lineage, cooldowns), minting new pets, and potentially updating pet status during breeding.
*   **New `pallet-breeding` (Recommended):** Given the potential complexity of genetic algorithms, compatibility rules, and managing the breeding process itself, a dedicated pallet is likely the cleanest approach. This pallet would interact heavily with `pallet-critter-nfts` (via `NftManager` or direct calls if tightly coupled) and `pallet-items`.
*   **`pallet-items` (Future):** To manage fertility items.
*   **UI Wallet:** Extensive new UI sections for selecting pets for breeding, viewing lineage, managing breeding cooldowns, and using fertility items.
    *   **Viewing Lineage in Pet Details:**
        *   When a user views the details of their individual Pet NFTs (e.g., in the "My Pet NFTs" list), basic lineage information will be displayed directly as part of the pet's attributes. This would include:
            *   `Parent 1 ID: [ID of Parent A or N/A]`
            *   `Parent 2 ID: [ID of Parent B or N/A]`
            *   `(Generation: Gen X)`
        *   A placeholder button or link, such as "[View Full Lineage Tree (Future)]", will also be present for each pet.
        *   Clicking this would conceptually navigate to a more detailed visual pedigree view in a later implementation, allowing users to trace ancestry across multiple generations if the data is available on-chain via the `parent1_id` and `parent2_id` fields in the `PetNft` struct.

The Pet Breeding & Genetics system aims to be a deeply engaging end-game activity, encouraging long-term player investment and creating a dynamic market for selectively bred Pet NFTs.

## 10. Future Feature: Pet Day Cares & Caregiver Roles

Pet Day Cares introduce a social and passive development mechanic to CritterCraft, allowing pets to gain attributes or experience over time while being looked after by other players or their designated "Caregiver" pets. This feature aims to provide utility for pets not actively questing or battling and creates new service-oriented roles within the ecosystem.

### 1. Core Concept: Passive Pet Development

*   **Boarding Pets:** Owners can choose to place their Pet NFTs into a "Day Care" facility or service for a defined period.
*   **Attribute/XP Gain:** While in day care, pets can passively gain:
    *   Experience points.
    *   Increases in specific attributes (e.g., mood, energy, or even minor skill points).
    *   Potentially, development towards specific personality traits based on the caregiver's specialty.
*   **Cost/Fees:** Day care services might involve a PTCN fee paid by the pet owner to the day care operator/caregiver.

### 2. The Caregiver Role

*   **Human Players as Operators:** Players could operate a Day Care service, setting fees, capacity, and perhaps specializing in certain types of care.
*   **Pets as Caregivers (A Specialized "Pet Job"):**
    *   A unique aspect where a player's own Pet NFT can take on the "Caregiver" job.
    *   The Caregiver Pet's attributes (e.g., level, specific personality traits like "Nurturing," "Wise," or "Trainer," or a dedicated "Caregiving Skill") would directly influence the type and rate of benefits received by the boarded pets.
    *   This makes certain pets valuable not just for their individual prowess but for their ability to nurture others.
    *   The owner of the Caregiver Pet would earn the fees from the day care service.

### 3. Pallet Interactions & On-Chain Logic

*   **New `pallet-daycare` (Recommended) or extend `pallet-critter-nfts` / `pallet-jobs-board`:**
    *   **Storage:**
        *   `DayCareServices`: `StorageMap<OperatorAccountId, DayCareDetails { capacity: u32, fee_per_block_or_session: Balance, caregiver_pet_id: Option<PetId>, specialty: Option<CareType> }>`
        *   `BoardedPets`: `StorageMap<PetId, BoardingRecord { owner: AccountId, caregiver_operator: AccountId, start_block: BlockNumber, accumulated_bonus_points: u32 }>`
        *   `CaregiverPetJobs`: `StorageMap<PetId, CaregiverJobStatus { active: bool, specialty: CareType }>` (if pets are caregivers).
    *   **Extrinsics:**
        *   `register_daycare_service(origin, capacity, fee, caregiver_pet_id_option, specialty_option)`: For operators to list their service.
        *   `enroll_pet_in_daycare(origin, pet_id: PetId, service_operator_id: AccountId)`: For pet owners to board their pets. This would likely involve locking the pet (via `NftManager` in `pallet-critter-nfts`) to prevent transfer/battle while boarded, and potentially an upfront fee payment or escrow.
        *   `retrieve_pet_from_daycare(origin, pet_id: PetId)`: Owner retrieves pet. At this point, accrued benefits (XP, attribute points) are calculated and applied to the Pet NFT (via `update_pet_metadata` in `pallet-critter-nfts`). Fees are settled. Pet is unlocked.
        *   `assign_pet_as_caregiver(origin, pet_id: PetId, specialty: CareType)`: If pets can be caregivers.
        *   `remove_pet_from_caregiver_role(origin, pet_id: PetId)`.
*   **`pallet-critter-nfts`:**
    *   `update_pet_metadata` would be called by `pallet-daycare` to apply accrued attribute/XP gains when a pet is retrieved.
    *   The `NftManager`'s lock mechanism could be used to signify a pet is "in day care."
*   **`pallet-balances` / `Currency Trait`:** For fee payments.
*   **`pallet-jobs-board` (Future):** The "Caregiver" role could be formally defined and managed here, with `pallet-daycare` checking a pet's job status.

### 4. Economic & Social Impact

*   Provides a way for pets to be productive even when their owners are offline or focused on other activities.
*   Creates new service roles and income streams for players (operating day cares or having skilled caregiver pets).
*   Encourages specialization of pets (e.g., breeding pets specifically for high caregiver stats).
*   Adds social depth as players entrust their pets to others or rely on the skills of specific caregiver pets.
    *   **Economic Model:**
        *   **Service Fees:** Day Care operators (players or owners of Caregiver Pets) will define fees for their services. Fees could be structured per block, per session (e.g., a fixed number of blocks), or as a flat rate for a defined care duration. These fees are paid in PTCN by the pet owner to the day care operator.
        *   **Fee Management:** `pallet-daycare` (conceptual) would manage the escrow and transfer of these fees upon successful completion of a care period (when the pet is retrieved). A small platform fee could also be deducted here.

    #### 5. Conceptual User Interface for Pet Day Cares

    The "Pet Day Care Center" in the UI Wallet will provide interfaces for both pet owners seeking day care services and players operating them.

    *   **Finding & Enrolling in Day Care Services (`#find-daycare-services`):**
        *   A list (`#daycare-service-list`) will display available day care services. Each entry would show:
            *   Operator/Caregiver Pet details (e.g., "Operator: Alice (Caregiver Pet: NurturerBot ID: 789)").
            *   Service specialty (e.g., "XP Gain," "Mood Boost").
            *   Fee per session/day.
            *   Current capacity (e.g., "3/5 Pets").
        *   Users can select one of their eligible pets from a dropdown associated with each service.
        *   An "Enroll Selected Pet" button would (conceptually) trigger the `enroll_pet_in_daycare` extrinsic.
        *   A status area (`#enroll-daycare-status`) provides feedback.

    *   **Managing Boarded Pets (`#my-boarded-pets`):**
        *   A list (`#boarded-pets-list`) shows pets the user currently has enrolled in day cares.
        *   Each entry displays:
            *   Pet name/ID and the day care it's at.
            *   Current status (e.g., "Gaining XP," "Session ending in X hours").
            *   Expected benefits upon retrieval.
            *   A "Retrieve Pet" button to (conceptually) call `retrieve_pet_from_daycare`.
        *   A status area (`#retrieve-daycare-status`) provides feedback.

    *   **Managing Own Day Care Service (`#manage-my-daycare` - For Operators/Caregivers):**
        *   This section would display the status of the user's own day care service if they are an operator.
        *   Information shown could include: Caregiver Pet, specialty, fee, capacity, currently boarded pets, and accumulated earnings.
        *   Placeholders for actions like:
            *   "Setup/Update My Day Care Service" button (to call `register_daycare_service` or a similar update extrinsic).
            *   "Withdraw Earnings" button.
        *   A status area (`#manage-daycare-status`) for feedback on these actions.

    This UI aims to make the process of finding, using, and providing day care services intuitive and informative.

## 11. Item System (`pallet-items`)

A dedicated Item System, likely managed by a `pallet-items`, will introduce a variety of usable and equippable objects that can affect Pet NFTs, gameplay, and the economy. These items can be earned, crafted (future), or traded.

### 1. Core Item Concepts
*   **Item Definitions:** Each item type will have a definition including its name, description, category, effects, and stackability.
*   **Item Categories (`ItemCategory` enum):**
    *   `Consumable`: Single-use items (e.g., health potions, food providing temporary buffs, stat-increase snacks).
    *   `Equipment`: Wearable items providing persistent bonuses while equipped (e.g., collars for defense, charms for luck). (Note: Equipping logic would be complex and might involve a separate system or `pallet-critter-nfts` extension).
    *   `TraitModifier`: Rare items that can grant a Pet NFT a new personality trait or modify an existing one.
    *   `FertilityBooster`: Items specifically designed to influence the Pet Breeding system (e.g., increase success rates, affect offspring traits).
    *   `Cosmetic`: Items that change a pet's appearance (conceptual, requires visual representation).
*   **Item Effects (`ItemEffect` enum):**
    *   `AttributeBoost { attribute: PetAttributeType, value: i16, is_percentage: bool, is_permanent: bool, duration_blocks: Option<BlockNumberType> }`: Modifies a specified pet attribute (e.g., `ExperiencePoints`, `MoodIndicator`, `HungerStatus`, `EnergyStatus`). The `value` can be positive or negative. `is_percentage` determines if `value` is an absolute number or a percentage of the current attribute (more complex). `is_permanent` determines if the change is lasting (e.g., a permanent increase to max energy, or direct XP gain) or temporary (requiring `duration_blocks`). **Note:** Permanent changes to base charter attributes like `BaseStrength` by items would be exceptionally rare or disallowed to maintain their foundational nature.
    *   `GrantPersonalityTrait { trait_to_grant: Vec<u8> }`: Adds a new personality trait string to the pet, if the pet doesn't already have it and hasn't reached a maximum trait limit.
    *   `ModifyFertility { fertility_points_change: i16, cooldown_reduction_blocks: Option<BlockNumberType> }`: Specifically for breeding, this can alter a conceptual "fertility score" on a Pet NFT or reduce its breeding cooldown time. This interacts with `pallet-breeding` via effects applied to the Pet NFT.
*   **User Inventories:** Each user will have an on-chain inventory tracking the quantity of each `ItemId` they own.

### 2. Pallet Structure (`pallet-items` Conceptual Outline)
*   **`Config` Trait:**
    *   Dependencies: `Currency` (for item costs if bought from system). Crucially, it defines an associated type `NftHandler` which must implement the `NftManagerForItems` trait. This trait (conceptually defined within `pallet-items`'s `lib.rs` scope for clarity of requirements) details functions that `pallet-critter-nfts` needs to expose.
    *   Constants: `MaxItemNameLength`, `MaxItemDescriptionLength`, `MaxEffectsPerItem`, `MaxTraitLength`.
*   **Storage:**
    *   `NextItemId`: Counter for unique item type IDs.
    *   `ItemDefinitions<ItemId, ItemDetails>`: Stores the properties of each defined item type.
    *   `UserItemInventory<(AccountId, ItemId), Quantity>`: Tracks how many of each item a user owns.
*   **Events:**
    *   `ItemDefined`: When a new item type is added by an admin.
    *   `ItemUsedOnPet`: When a user successfully applies an item to their pet.
    *   `ItemsTransferred`: When items are moved between users.
*   **Errors:** For issues like item not found, insufficient quantity, item not applicable to target, etc.
*   **Extrinsics:**
    *   `admin_add_item_definition(...)`: Admin-only extrinsic to define new item types, their categories, effects (including `is_permanent`, `is_percentage`, `duration_blocks` for attribute boosts), and stackability.
    *   `user_apply_item_to_pet(origin, item_id, target_pet_id)`: The core extrinsic for item usage. Its logic involves:
        1.  Verifying the `item_id` exists and the `user` (origin) possesses it in `UserItemInventory`.
        2.  Calling `T::NftHandler::get_pet_owner(&target_pet_id)` to confirm the `user` owns the target `PetNft`.
        3.  Determining item consumption:
            *   If `ItemCategory::Consumable`, decrement or remove from inventory.
            *   If stackable and not `Equipment`, decrement quantity.
            *   `Equipment` items are not "used" via this extrinsic if they require a separate equipping mechanism. If "using" equipment implies a one-time permanent effect application, it's treated like a consumable.
        4.  Iterating through the `item_details.effects` vector:
            *   For `ItemEffect::AttributeBoost`, call `T::NftHandler::apply_attribute_boost_to_pet(&user, &target_pet_id, attribute, value, is_percentage, is_permanent, duration_blocks)`.
            *   For `ItemEffect::GrantPersonalityTrait`, call `T::NftHandler::grant_personality_trait_to_pet(&user, &target_pet_id, trait_to_grant)`.
            *   For `ItemEffect::ModifyFertility`, call relevant methods on `T::NftHandler` like `modify_pet_fertility_value` or `reduce_pet_breeding_cooldown`.
        5.  Emitting an `ItemUsedOnPet` event with details of the effects applied.
    *   Future extrinsics: `transfer_item`, `buy_item_from_system_shop` (if a central shop exists).

### 3. Interaction with Other Systems
*   **`pallet-critter-nfts` (implements `NftManagerForItems`):**
    *   `get_pet_owner`: Returns the owner of the specified `PetId`.
    *   `apply_attribute_boost_to_pet`: This is a key function. It would:
        *   Verify the `caller` is the owner of the `pet_id`.
        *   Match the `PetAttributeType` to the corresponding field in the `PetNft` struct.
        *   Apply the `value` (handling `is_percentage` logic if applicable).
        *   If `is_permanent` is true, directly modify the pet's state (e.g., `experience_points`, `mood_indicator`, `hunger_status`, `energy_status`). **Modification of base charter attributes (like `BaseStrength`) by items should be disallowed or extremely rare and controlled.**
        *   If `is_permanent` is false and `duration_blocks` is `Some`, this implies a temporary buff/debuff system. `pallet-critter-nfts` would need additional storage to track active temporary effects on pets, their magnitudes, and expiry block numbers. The `apply_time_tick` function in `pallet-critter-nfts` would then also need to process these temporary effects.
    *   `grant_personality_trait_to_pet`: Appends the given trait to the `PetNft.personality_traits` vector, potentially checking for duplicates or a maximum number of traits.
    *   `modify_pet_fertility_value`: Modifies a conceptual fertility-related value on the `PetNft` (this might require adding a new field to `PetNft` or dedicated storage).
    *   `reduce_pet_breeding_cooldown`: Interacts with `pallet-breeding` (if cooldowns are managed there) or updates a cooldown field on the `PetNft` itself if `pallet-critter-nfts` manages that aspect of breeding readiness.
*   **Shops & Marketplace (`pallet-marketplace`, `pallet-user-shops`):** Items will be listable and tradable, creating economic activity.
*   **Quests (`pallet-quests`):** Items can be quest objectives or rewards.
*   **Breeding:** Fertility items will directly interact with the breeding system.

The Item System will significantly enhance gameplay depth, pet customization, and economic activity within CritterCraft.

    #### 4. Conceptual User Interface for Item System

    The UI Wallet will provide an "My Item Inventory" section (`#item-inventory-section`) for users to manage and use their items.

    *   **Displaying Item Inventory (`#item-inventory-list`):**
        *   A list will display all items owned by the user, grouped by `ItemId`.
        *   Each entry will show: Item Name, ID, Quantity (if stackable), Category, Description, and a summary of its Effects.
    *   **Using Items:**
        *   For usable items (e.g., consumables), each item entry will have a dropdown (`.item-target-pet-select`) allowing the user to select one of their own Pet NFTs as the target.
        *   A "Use Item" button (`.use-item-button`) next to the selector would (conceptually) trigger the `user_apply_item_to_pet` extrinsic.
        *   For items like "Equipment," the button might be "Equip (Conceptual)," indicating a more complex equipping system for future development (potentially involving dedicated equipment slots on pets).
    *   **Status Feedback (`#item-action-status`):** Provides feedback on item usage attempts.

## 12. User Score & Reputation System (`pallet-user-profile`)

To quantify user engagement, achievements, and trustworthiness within the CritterCraft ecosystem, a `pallet-user-profile` will be introduced. This pallet will maintain various scores for each user, derived from their on-chain activities.

### 1. Core Concepts
*   **Derived Scores:** User scores are not directly set by users but are calculated and updated by the system when users perform specific actions in other integrated pallets (e.g., completing quests, winning battles, successful trades, pet development).
*   **Reputation & Trust:** Certain scores, like a trade reputation score, can help build trust between players in P2P interactions.
*   **Progression & Recognition:** Scores can serve as a measure of a user's overall progress and contribution to the ecosystem, potentially unlocking perks, titles, or influencing governance weight in the future.
*   **Activity Tracking:** The profile can also store the `last_active_block` for a user, updated whenever they interact with a system that calls this pallet.

### 2. `UserProfile` Struct Components
The core data structure, `UserProfile<BlockNumber>`, will store metrics such as:
*   `total_pet_levels_sum: ScoreValue`: Aggregate sum of levels of all pets owned by the user.
*   `quests_completed_count: u32`: Total number of unique quests completed.
*   `battles_won_count: u32`: Total number of battles won.
*   `successful_trades_count: u32`: Number of successful trades in the marketplace or user shops.
*   `community_contributions_score: ScoreValue`: Score derived from participation in governance, validated ecosystem support jobs, or other community-benefiting activities.
    *   `overall_progress_score: ScoreValue`: A weighted composite score. **Its calculation will involve summing weighted contributions from other scores (e.g., pet levels, quests, battles, trades, community contributions). To maintain balance, conceptual caps or diminishing returns might be applied to individual components before they are weighted and summed (e.g., the score contribution from 'quests_completed_count' might cap after 500 quests, as detailed in the `recalculate_overall_score` logic).**
*   `trade_reputation_score: i32`: A score that can increase or decrease based on feedback from trading partners (requires a future feedback mechanism).
    *   `last_active_block: BlockNumber`: The block number of the user's last recorded significant on-chain activity, **updated by a central helper function (`update_profile_and_recalculate`) whenever a score-affecting action occurs, or by `record_user_activity` for general actions.**

### 3. Pallet Structure (`pallet-user-profile` Conceptual Outline)
*   **`Config` Trait:**
    *   Includes `RuntimeEvent`.
    *   Defines constants for score weights (e.g., `PetLevelScoreWeight`, `QuestScoreWeight`, `BattleWinScoreWeight`, `TradeScoreWeight`) used in calculating the `overall_progress_score`.
*   **Storage:**
    *   `UserProfiles<AccountId, UserProfile<BlockNumberFor<T>>>`: Stores the profile data for each user.
*   **Events:**
    *   `UserProfileUpdated { user, new_overall_score }`: When a user's profile (and particularly overall score) is updated.
    *   `TradeReputationChanged { user, new_reputation, change_delta }`.
*   **Hooks:**
    *   Conceptual `on_initialize` hook placeholder for potential general activity tracking.
*   **Internal Functions (Callable by Other Pallets):**
    *   A central private helper `update_profile_and_recalculate(user, mutator_fn)` streamlines updates. It applies the specific metric change via `mutator_fn`, updates `last_active_block`, calls `recalculate_overall_score`, and emits `UserProfileUpdated`.
    *   Public functions like `update_pet_level_sum`, `record_quest_completion`, `record_battle_win`, `record_successful_trade`, and `record_community_contribution` use this helper.
    *   `record_user_activity` only updates `last_active_block`.
    *   `update_trade_reputation` directly mutates the profile for reputation and `last_active_block`, emitting its own `TradeReputationChanged` event, as it may not always directly influence the `overall_progress_score`.
*   **Score Calculation (`recalculate_overall_score` internal function):**
    *   This function implements the weighted formula for `overall_progress_score`.
    *   **Example Weighting & Caps (Conceptual):**
        *   `pet_score = total_pet_levels_sum * PetLevelScoreWeight`
        *   `quest_score = min(quests_completed_count, QUEST_COUNT_SCORE_CAP) * QuestScoreWeight` (e.g., `QUEST_COUNT_SCORE_CAP = 500`)
        *   `battle_score = min(battles_won_count, BATTLE_WINS_SCORE_CAP) * BattleWinScoreWeight` (e.g., `BATTLE_WINS_SCORE_CAP = 1000`)
        *   `trade_activity_score = min(successful_trades_count, TRADES_COUNT_SCORE_CAP) * TradeScoreWeight` (e.g., `TRADES_COUNT_SCORE_CAP = 200`)
        *   `overall_score = pet_score + quest_score + battle_score + trade_activity_score + community_contributions_score` (community score might also have caps/weights).
    *   Weights are defined in `Config`. Caps are illustrative internal constants for balancing.
*   **Extrinsics:** Likely minimal initially; future additions could allow users to set cosmetic profile aspects.

### 4. Impact and Future Uses
*   **Gamification:** Provides clear progression metrics for users.
*   **Access Control/Perks:** High scores or specific achievements could grant access to exclusive content, events, quests, or titles.
*   **Governance Influence:** Could be a factor in future advanced governance models (e.g., reputation-weighted voting, eligibility for council).
*   **Matchmaking:** In battles or other systems, scores could be used for fairer or more targeted matchmaking.
*   **Personalized Experiences:** User profile data can help tailor in-game events or offers.

The User Score & Reputation System aims to create a richer, more rewarding experience by recognizing and quantifying player contributions and achievements across the entire CritterCraft platform.

    #### 5. Conceptual User Interface for User Profile & Scores

    A "My Profile & Achievements" section (`#user-profile-section`) in the UI Wallet will allow users to view their calculated scores and ecosystem standing.

    *   **Score Display (`#profile-scores-display`):**
        *   This area will clearly list all the scores and metrics stored in the user's `UserProfile` struct:
            *   Overall Progress Score (`#profile-overall-score`)
            *   Total Pet Levels Sum (`#profile-pet-levels-sum`)
            *   Quests Completed (`#profile-quests-completed`)
            *   Battles Won (`#profile-battles-won`)
            *   Successful Trades (`#profile-successful-trades`)
            *   Community Contributions Score (`#profile-community-contributions`)
            *   Trade Reputation Score (`#profile-trade-reputation`)
            *   Last Active Block (`#profile-last-active`)
    *   **Achievements/Badges (Conceptual):**
        *   Future enhancements could include displaying badges, titles, or achievements unlocked based on reaching certain score thresholds or completing specific milestones.

    This UI aims to give users a clear overview of their engagement, reputation, and progress within CritterCraft.

## 13. Pet Development Lifecycle (Conceptual)

Beyond the immutable charter attributes set at minting, Pet NFTs in CritterCraft are dynamic entities that grow and change based on time, owner interactions, and experiences. This lifecycle is primarily managed within `pallet-critter-nfts`, potentially influenced by `pallet-items`.

### a. Time-Based State Changes (The "Tick" Mechanic)
*   **Concept:** Pets' core needs and states (hunger, energy, mood) change passively over time, simulating a living creature.
*   **Implementation Idea (`apply_time_tick` internal function in `pallet-critter-nfts`):**
    *   A conceptual internal function, `apply_time_tick(&pet_id, blocks_since_last_tick)`, would be responsible for these updates.
    *   This function would need a `last_tick_applied_block` field (or similar) in the `PetNft` struct to calculate `blocks_since_last_tick`.
    *   **Hunger:** Increases gradually over time (e.g., +1 point per N blocks). Reaches a maximum (e.g., 100).
    *   **Energy:** Decreases gradually over time (e.g., -1 point per M blocks). Reaches a minimum (e.g., 0).
    *   **Mood:** Dynamically adjusts based on hunger and energy levels. High hunger or low energy negatively impacts mood, while low hunger and high energy positively impact it. Mood could be a scale (e.g., 0-100).
*   **Triggering Ticks:** This logic could be triggered:
    *   When any interaction with the pet occurs (e.g., before feeding, playing).
    *   Periodically by an off-chain worker that queries pets and submits a "tick update" extrinsic (less decentralized, more complex).
    *   Conceptually, a future on-chain scheduler could also trigger such updates for active pets, though this has weight implications.

### b. Interaction-Driven Development (`feed_pet`, `play_with_pet`)
*   **Concept:** Owners actively develop their pets by interacting with them, primarily through using items.
*   **Implementation Idea (Conceptual extrinsics in `pallet-critter-nfts`):**
    *   **Feeding and Playing:** These actions are now primarily driven by the `user_apply_item_to_pet` extrinsic in `pallet-items`.
        *   To "feed" a pet, the user would use an item of `ItemCategory::Consumable` (e.g., "Basic Kibble") that has an `ItemEffect::AttributeBoost` targeting `PetAttributeType::HungerStatus` (e.g., value = -30, `is_permanent = true` for the hunger reduction) and potentially `PetAttributeType::MoodIndicator` (e.g., value = +5).
        *   To "play" with a pet, the user might use a "Toy" item. This could be a durable item (not consumed, future) or a consumable "Play Treat". Its effects could include `AttributeBoost` for `MoodIndicator` (increase), `EnergyStatus` (decrease), `HungerStatus` (slight increase), and `ExperiencePoints` (increase).
        *   The `pallet-items` extrinsic calls methods on `pallet-critter-nfts` (via `NftManagerForItems`) to apply these specific attribute changes defined by the item's `effects`.
*   **Events:** `pallet-items` emits `ItemUsedOnPet`. `pallet-critter-nfts` would likely emit `PetNftMetadataUpdated` as its internal `PetNft` struct fields are changed by the `NftManagerForItems` trait implementation.

### c. Leveling and Experience (XP)
*   **Concept:** Pets gain Experience Points (XP) through interactions like feeding, playing, completing quests (via `pallet-quests`), and winning battles (via `pallet-battles`).
*   **Implementation Idea (`attempt_level_up` internal function in `pallet-critter-nfts`):**
    *   Whenever a pet gains XP, this internal function is called.
    *   **XP Curve:** A defined XP curve (e.g., `next_level_xp = BASE_XP * (level ^ EXP_FACTOR)`) determines the XP needed for the next level. This could use constants defined in `Config`.
    *   **Level Up:** If XP exceeds required amount:
        *   Level increases.
        *   XP is reduced by the amount needed (excess XP carries over).
        *   **Effects of Leveling:**
            *   This is a key area for future design. Leveling might:
                *   Slightly increase dynamic stats (if a distinction between base charter stats and current dynamic stats is implemented beyond just level).
                *   Increase the *potential* or cap for dynamic stats.
                *   Unlock new abilities or slots for abilities (future system).
                *   Grant "evolution points" or "skill points" to be spent by the owner (future system).
            *   For the current conceptual stage, leveling primarily increases the `level` attribute itself, which can then be a factor in battle calculations, quest eligibility, etc.
    *   An event like `PetLeveledUp` would be emitted.

### d. Personality Trait Evolution (Conceptual)
*   **Concept:** Beyond the initial set (if any), personality traits could evolve based on long-term pet states or specific interactions.
*   **Implementation Ideas (Future, within `pallet-critter-nfts` or `apply_time_tick`):**
    *   If a pet's `mood_indicator` remains very low for an extended period, it might gain a "Grumpy" or "Sad" trait.
    *   Consistently feeding a pet specific types of "luxury" foods might lead to a "Picky" or "Refined" trait.
    *   Winning many battles could contribute to a "Brave" or "Confident" trait.
*   **Mechanics:** This would require more sophisticated tracking of historical states or specific interaction counts, adding complexity but also depth to pet individuality.

This lifecycle ensures that Pet NFTs are not static but grow and change, reflecting their journey and the care provided by their owners, making each pet more unique over time.

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]

[end of ADVANCED_FEATURES.md]
