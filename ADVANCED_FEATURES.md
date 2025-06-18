# CritterCraft: Advanced Features & Gameplay Loops (Conceptual Outline)

This document provides a high-level conceptual outline for advanced economic loops and gameplay features planned for future stages of CritterCraft development, building upon the foundations of Stage 5.

## 1. User Shops

*   **Concept:** Allow players to set up their own persistent, customizable shops within the CritterCraft ecosystem to sell Pet NFTs and, in the future, other items.
*   **Pallet Name:** `pallet-user-shops`
*   **Pallet Interactions:**
    *   **`crittercraft-traits::NftManager`:** Implemented by `pallet-critter-nfts`. Used by `pallet-user-shops` to verify ownership of Pet NFTs being listed and to facilitate the transfer of NFTs upon a successful purchase.
    *   **`frame_support::traits::Currency`:** For handling PTCN payments from buyers to sellers, typically implemented by `pallet-balances`.
    *   **(Future) `crittercraft-traits::ItemManager` (or similar):** If/when shops support fungible items or non-NFT items, an interaction trait for `pallet-items` would be needed.
*   **Core On-Chain Logic/Data (for `pallet-user-shops` - MVP Focus):**
    *   **`ShopId`**: For MVP, `ShopId` is simply the `AccountId` of the owner (one shop per user).
    *   **`ShopStatus` Enum:** `Open`, `ClosedTemporarily`.
    *   **`Shop<BoundedName, BoundedDescription>` Struct:**
        *   `owner: AccountId`
        *   `name: BoundedVec<u8, T::MaxShopNameLen>`
        *   `description: BoundedVec<u8, T::MaxShopDescriptionLen>`
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
        *   `ShopOwnedListings<AccountId, BoundedVec<ListingId, T::MaxListingsPerShop>>`: Maps a `ShopId` (owner's `AccountId`) to a list of `ListingId`s they own.
        *   `AllListings<ListingId, Listing<BalanceOf<T>, T::NftId>>`: Maps a `ListingId` to its `Listing` details.
        *   `NextListingId<ListingId>`: A counter to generate unique `ListingId`s.
*   **Extrinsics (Conceptual Signatures for `pallet-user-shops`):**
    *   `create_shop(origin, name: BoundedVec<u8, T::MaxShopNameLen>, description: BoundedVec<u8, T::MaxShopDescriptionLen>)`: Creates a shop for the `origin`. Fails if shop already exists for the account.
    *   `update_shop_details(origin, name: Option<BoundedVec<u8, T::MaxShopNameLen>>, description: Option<BoundedVec<u8, T::MaxShopDescriptionLen>>)`: Allows owner to update shop details.
    *   `set_shop_status(origin, status: ShopStatus)`: Allows owner to open or temporarily close their shop.
    *   `close_shop_permanently(origin)`: Allows owner to permanently close their shop (removes shop and all its listings).
    *   `list_item(origin, item_id: T::NftId, price: BalanceOf<T>)`: Lists a Pet NFT for sale. Verifies ownership via `NftManager`, locks the NFT, and creates a new listing. (MVP: quantity is 1).
    *   `unlist_item(origin, listing_id: ListingId)`: Removes a listing. Unlocks the NFT via `NftManager`.
    *   `buy_item(origin, listing_id: ListingId)`: Purchases an item. Transfers PTCN from buyer to seller, transfers NFT from seller to buyer via `NftManager`, and removes the listing. (MVP: quantity is 1).
*   **Economic Model & Impact:**
    *   Fosters a player-driven economy.
    *   **Fee Mechanism:** `pallet-user-shops` itself **does not implement an intrinsic fee mechanism** for MVP. Any platform fees on sales would need to be implemented externally (e.g., by runtime logic wrapping the `buy_item` call, or by a separate fee-collection pallet that `pallet-user-shops` could optionally integrate with post-MVP). This keeps the core shop pallet lean.
    *   Shop creation and listing are free for MVP to encourage participation.
*   **NFT Interaction:** Uses the `crittercraft-traits::NftManager` for all NFT-related operations like checking ownership, locking NFTs upon listing, unlocking upon unlisting, and transferring upon successful purchase.

    #### Conceptual User Interface for User Shops

    The "User Shops District" in the UI Wallet will be the central hub for player-to-player commerce.

    *   **Browsing and Searching Shops (`#browse-user-shops`):**
        *   A main view will allow users to see a list of active user shops (`#user-shop-list`).
        *   A search bar (`#search-shops-input`) will enable finding shops by name or owner `AccountId`.
        *   Each shop in the list will display its name, owner `AccountId`, description, and a "Visit Shop" button.

    *   **Viewing an Individual Shop (`#view-individual-shop`):**
        *   Clicking "Visit Shop" navigates to a dedicated view for that shop.
        *   Displays shop name, owner, description, and status.
        *   A list (`#shop-item-list`) shows all Pet NFTs currently for sale in that shop, with their prices.
        *   Each item has a "Buy Item" button (conceptually triggers `buy_item`). A status area (`#buy-from-shop-status`) provides feedback.

    *   **Managing Own Shop (`#manage-my-shop`):**
        *   If the user doesn't have a shop: "Create My Shop!" button (triggers `create_shop`).
        *   If shop exists:
            *   "Shop Details" form (`#shop-details-form`) to update name/description (triggers `update_shop_details`).
            *   "Shop Status" control (triggers `set_shop_status`).
            *   "Manage Listings" area:
                *   Form (`#list-item-form`) to select owned Pet NFTs, set price (triggers `list_item`).
                *   View current listings (`#my-shop-listings-list`) with "Unlist Item" buttons (triggers `unlist_item`).
            *   "Close Shop Permanently" button (triggers `close_shop_permanently`).
        *   Status paragraphs provide feedback on management actions.

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

    #### Conceptual User Interface for Jobs Board

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
    *   **`pallet-critter-nfts` (via `T::NftChecker` trait):** To verify pet-related quest criteria (level, species, ownership). The `QuestNftRequirementChecker` trait (conceptually defined in `pallet-quests` or a shared trait crate) would be implemented by `pallet-critter-nfts` and would need to expose methods like `get_pet_owner`, `get_pet_level`, and `get_pet_species`.
    *   **`pallet-items` (via `T::ItemChecker` trait):** To verify and consume items required for quests. The `QuestItemRequirementChecker` trait would be implemented by `pallet-items` and needs a method like `check_and_consume_item`.
    *   **`pallet-user-profile` (via `T::UserProfileChecker` trait):** To verify user-specific stats (e.g., battles won, reputation) as prerequisites. The `QuestUserProfileRequirementChecker` trait would be implemented by `pallet-user-profile` and needs methods like `get_battles_won` and `get_trade_reputation`.
    *   **`pallet-balances` (via `Currency` trait):** For disbursing PTCN quest rewards.
*   **Core On-Chain Logic/Data (Enhancements):**
    *   **`Quest` Struct & Advanced Criteria:**
        *   The `Quest` struct in `pallet-quests` is enhanced to include several optional fields for defining diverse on-chain verifiable completion criteria:
            *   `description: Vec<u8>`, `reward_ptcn: BalanceOf<T>` (existing).
            *   **Pet-Specific Criteria:** `required_pet_level: Option<u32>`, `required_pet_id_for_level_check: Option<PetId>`, `required_pet_species: Option<PetSpeciesType>`, `required_pet_id_for_species_check: Option<PetId>`. These allow quests to demand, for instance, that a user's specific pet (or any one of their pets if the ID field is `None` and user supplies one via `maybe_target_pet_id`) has reached a certain level or is of a particular species.
            *   **Item-Specific Criteria:** `required_item_id: Option<ItemId>`, `required_item_quantity: Option<u32>`, `consume_item_on_completion: bool`. Quests can require users to possess certain items, and optionally consume them upon completion.
            *   **User Profile Criteria:** `min_battles_won_for_user: Option<u32>`, `min_trade_reputation_for_user: Option<i32>`. These link quest eligibility to a user's broader achievements and standing in the ecosystem.
        *   The flexibility of `Option` fields means not all quests need to use all criteria; many can remain simple.
    *   **Extrinsic Updates & Verification Logic:**
        *   **`add_quest` (Admin):** The signature is updated to include all new optional criteria fields (e.g., `required_pet_level`, `required_item_id`, `min_battles_won_for_user`, etc.), allowing administrators to define complex quests. The `consume_item_on_completion` field defaults appropriately if not specified.
        *   **`complete_quest` (User):**
            *   The user can now optionally pass `maybe_target_pet_id` if the quest has pet-related criteria but doesn't hardcode a specific pet ID in its definition, allowing the user to nominate which of their pets fulfills the requirement.
            *   Before distributing rewards, the extrinsic performs **On-Chain Verification** by calling methods on the handler traits defined in its `Config`:
                *   `T::NftChecker::get_pet_owner`, `T::NftChecker::get_pet_level`, `T::NftChecker::get_pet_species` are used for pet criteria.
                *   `T::ItemChecker::check_and_consume_item` is used for item criteria (this method itself would handle the `consume_item_on_completion` logic based on the quest's flag).
                *   `T::UserProfileChecker::get_battles_won`, `T::UserProfileChecker::get_trade_reputation` are used for user profile criteria.
            *   If any criterion is not met, the extrinsic fails with a specific error (e.g., `PetLevelTooLow`, `RequiredItemNotFoundOrInsufficient`, `NotEnoughBattlesWon`).
            *   Upon successful verification of all criteria, the quest reward is distributed, the quest is marked as completed for the user, and `pallet-user-profile::Pallet::<T>::record_quest_completion(&account)` is called to update the user's profile score.
*   **Economic Impact:** Quests serve as a primary mechanism for PTCN distribution (rewards) and can also act as item sinks if `consume_item_on_completion` is true. Complex quests requiring rare items or high pet/user stats can drive demand for those prerequisites.

    This makes quests more integrated with other systems and allows for more varied and challenging objectives.

## 4. Treasure Hunts & Exploration
    // Note: The previous H2 "3. Treasure Hunts & Exploration" is being replaced by the detailed "3. Quest System".
    // The content below is the original "Treasure Hunts & Exploration" now re-numbered as H2 "4.".
    // If "Treasure Hunts" were to be a sub-type of "Quests", this would be structured differently.

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

    #### Conceptual User Interface for Treasure Hunts

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

## 5. Mini-Games & Arcade

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

    #### Conceptual User Interface for Mini-Games & Arcade

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

## 6. IoT Device Integration

*   **Concept:** Allow real-world data from Internet of Things (IoT) devices (e.g., smart pet feeders, activity trackers for real pets if users have them and want to link) to influence in-game pet stats or trigger on-chain events/rewards.
*   **Technical Challenges:** Requires secure oracle solutions to bring IoT data on-chain reliably.
*   **Impact:** Blurs the lines between the virtual and real world, offering unique engagement. This is a highly speculative and long-term idea.

These advanced features aim to create a deeply engaging, economically vibrant, and evolving world for CritterCraft players. Each will require careful design and phased implementation.

## 7. Pet NFT Charter Attributes (Foundational Traits)

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

### Dynamic Attributes & Simplifications (Recap from Pet Development Lifecycle)

*   **`level: u32`**, **`experience_points: u32`**: Standard dynamic attributes increased through interactions.
*   **`mood_indicator: u8`**: Simplified on-chain mood, directly affected by `feed_pet`, `play_with_pet`, and `apply_neglect_check`. Max value defined by `T::MaxMoodValue`.
*   **Hunger & Energy (Off-Chain Focus):** These are primarily calculated **off-chain** by UIs/game clients based on:
    *   `last_fed_block: BlockNumberFor<T>`
    *   `last_played_block: BlockNumberFor<T>`
    This significantly reduces on-chain storage and transaction frequency for these rapidly changing stats.
*   **`personality_traits: BoundedVec<BoundedVec<u8, T::MaxTraitStringLen>, T::MaxPetPersonalityTraits>`**: A bounded list of strings representing the pet's personality traits. `MaxTraitStringLen` and `MaxPetPersonalityTraits` are defined in `Config`. For MVP, these are typically set at minting or via rare items, not dynamically evolving through simple care.
*   **`last_state_update_block: BlockNumberFor<T>`**: Timestamp of the last significant on-chain interaction or state change.

### Further Derived or Gameplay-Relevant Attributes (from `dna_hash` & On-Chain Base Stats):

While the above are stored on-chain, the `dna_hash` and explicit base stats can still be used to imply or derive further nuanced attributes for off-chain game logic (e.g., in the Python MVP or a future game server) or more complex on-chain systems:

*   **Stat Growth Potential:** How base stats influence the potential maximums or growth curves for dynamic stats like current strength (derived from base + level + items).
*   **Secondary Elemental Affinities/Resistances:** More detailed elemental interactions.
*   **Hidden Talents/Abilities:** Specific rare abilities that might only become apparent or unlockable if certain base stats or affinities are present.
*   **Cosmetic Trait Predispositions:** The `dna_hash` could still influence rare base patterns or color variations not covered by dynamic cosmetics.
*   **Breeding Values:** These on-chain charter attributes will be critical inputs for the genetic algorithm in the future Pet Breeding system, determining the potential traits of offspring.

This combination of explicit on-chain charter attributes and the richer information derivable from the `dna_hash` provides a robust foundation for unique, developable, and breedable Pet NFTs, while the simplified handling of dynamic stats like hunger/energy streamlines on-chain operations for an MVP.

## 14. Staking UI V2 - Enhanced Interactions (Conceptual)

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

## 13. Competitive Pet Battles

CritterCraft will feature a robust system for competitive pet battles, where Pet NFTs engage in strategic combat. `pallet-battles` is the core on-chain component for managing battle registration, state, and outcomes, simplified for an MVP.

    ### 1. Core Concepts (Recap from pallet-battles design - MVP Focus)
    *   **Registration:** Players register one of their eligible Pet NFTs for battle.
    *   **Matchmaking:** (Conceptual) For MVP, this is likely a simple queue or direct challenge system. ELO ratings or complex brackets are post-MVP.
    *   **Outcome Reporting (Simplified for MVP):** A designated reporter (e.g., player 1, or a future trusted oracle) reports only the `winner_pet_id`. The `loser_pet_id` is inferred on-chain.
    *   **Rewards (Simplified for MVP):** A fixed PTCN amount (`BattleRewardAmount` from `Config`) is distributed to the winner.

    ### 2. Core On-Chain Logic/Data (`pallet-battles` - MVP Focus)
    *   **`BattleDetails` Struct:** Stores `player1`, `pet1_id`, `player2` (Option), `pet2_id` (Option), `status` (`PendingMatch`, `Concluded`), `winner` (Option<AccountId>).
    *   **`PetInBattle` Storage:** Tracks if a `PetId` is currently in an active `BattleId`.
    *   **`register_for_battle` Extrinsic:**
        *   Allows `player1` to register their `pet1_id`.
        *   Performs checks: pet not already in battle, player owns pet, pet is eligible (e.g., not locked elsewhere via `NftHandler::is_transferable`).
        *   Creates a new battle record with `status = PendingMatch`, `player2` and `pet2_id` as `None`.
        *   Marks `pet1_id` in `PetInBattle`.
    *   **`report_battle_outcome` Extrinsic (Simplified for MVP):**
        *   Takes `battle_id` and `winner_pet_id` as inputs.
        *   **Authority (MVP):** Only `player1` of the battle can report the outcome. (Future: Oracle or consensus from both players).
        *   **Logic:**
            *   Retrieves `BattleDetails`. Ensures battle exists and is not already `Concluded`.
            *   Determines `winner_account`, `loser_account`, and `loser_pet_id` based on the provided `winner_pet_id` and the stored `BattleDetails`.
            *   Updates `BattleDetails` status to `Concluded` and records the `winner_account`.
            *   If a `winner_account` is determined and `BattleRewardAmount` (from `Config`) is greater than zero, the reward is distributed to the `winner_account` (e.g., via `T::Currency::deposit_creating`).
            *   Clears `PetInBattle` entries for both `pet1_id` and `pet2_id` (if `pet2_id` was Some).
            *   Emits a `BattleConcluded` event with all relevant details (winner/loser accounts and pet IDs, reward amount).
        *   Storing a `battle_log_hash` is deferred for post-MVP.

    ### 3. Conceptual Battle Mechanics & Formulas (Inputs to Off-Chain Simulation - MVP Focus)

    The actual battle simulation (determining `winner_pet_id`) is performed **off-chain** for MVP. The result is then submitted to `report_battle_outcome`. The on-chain pet attributes from `pallet-critter-nfts` (obtained via `NftHandler`) serve as the primary input to this simulation.

    *   **Input Pet Data (MVP Focus):** For each participating pet, the off-chain simulation will primarily use:
        *   **`level`**: From `PetNft`.
        *   **Core Charter Attributes** from `PetNft`: `base_strength`, `base_agility`, `base_vitality`.
        *   **`primary_elemental_affinity: Option<ElementType>`**: From `PetNft`.
        *   **Deferred/Simplified for MVP Simulation:**
            *   `base_intelligence`: Might be deferred if not used for special moves/abilities in the MVP simulation.
            *   Detailed `personality_traits` influence: For MVP, this could be simplified to a few key traits having a predefined minor impact, or deferred entirely. Complex emergent behaviors from many traits are post-MVP.
            *   Complex item buffs active during battle: These are deferred for MVP. The simulation would use the pet's base and charter stats.
            *   `mood_indicator`: Could provide a minor percentage buff/debuff to effective stats (e.g., Happy = +5% attack/defense, Unhappy = -5%) if desired for MVP's simulation.
    *   **Effective Combat Stats (Calculated in Off-Chain Simulation):**
        *   **Effective HP:** Derived primarily from `base_vitality` and `level`. Example: `(base_vitality * VITALITY_HP_MULTIPLIER_CONST) + (level * LEVEL_HP_BONUS_CONST)`. (Constants are part of simulation balancing).
        *   **Effective Attack:** Derived primarily from `base_strength` and `level`.
        *   **Effective Defense:** Derived primarily from `base_vitality` (or a dedicated defense stat if added to charter attributes) and `level`.
        *   **Effective Speed:** Derived primarily from `base_agility` and `level`.
    *   **Turn-Based Logic (Conceptual Example for Off-Chain Simulation - MVP Simplification):**
        *   **Attack Order:** Determined by `Effective Speed`. Randomness (from a shared seed if deterministic off-chain simulation is desired) for ties.
        *   **Actions per Turn:** Basic "Attack" action for MVP. More complex moves/abilities are post-MVP.
        *   **Hit Chance (Simplified):** A base hit chance (e.g., 85-90%) is used. Complex accuracy vs. evasion calculations based on agility are deferred for MVP's core calculation. A simple random number generation against the base chance determines hit/miss.
            *   Example: `if (random_value % 100) < base_hit_chance { /* Hit */ } else { /* Miss */ }`
        *   **Damage Calculation (Simplified):** A straightforward formula, e.g., `damage = Attacker_Effective_Attack - (Defender_Effective_Defense / DEFENSE_FACTOR_CONST)`. Ensure minimum damage (e.g., 1) if attack > defense part.
        *   **Elemental Modifiers:** Retained as a core mechanic. Damage is adjusted based on `primary_elemental_affinity` matchups (e.g., Fire deals 1.5x to Nature, Water deals 0.75x to Fire). A predefined matrix of multipliers would be used.
        *   **Critical Hits & Advanced Effects (Deferred for MVP):** Complex critical hit systems (beyond a simple, flat random chance if desired for the simulation) and multi-layered status effects are post-MVP.
    *   **Winning Condition:** First pet to reach 0 HP loses. If a maximum number of turns is reached (to prevent stalemates), the winner might be determined by higher remaining HP percentage or other tie-breaker rules.
    *   **Battle Log (Off-Chain):** The off-chain simulation should ideally produce a log of turns, actions, and outcomes. While the hash of this log is not stored on-chain for MVP, the log itself can be useful for debugging, community interest, and potential future dispute resolution systems.

    This simplified approach for MVP ensures `pallet-battles` remains lean by focusing on registering participants and recording outcomes, while the computationally intensive battle simulation occurs off-chain using on-chain pet data as the source of truth.

## 11. Pet Breeding & Genetics

A comprehensive Pet Breeding and Genetics system is envisioned for CritterCraft, adding depth to pet collection and strategy. For an MVP, this system is simplified, focusing on core mechanics and deferring more complex genetic influences.

### 1. Core Breeding Mechanic (MVP Focus)

*   **Concept:** Owners of two compatible Pet NFTs can breed them to produce a new Pet NFT (an "egg" or "newborn" pet).
*   **Pallet Interaction (`pallet-breeding`):**
    *   The `initiate_breeding(origin, parent1_pet_id, parent2_pet_id)` extrinsic is called.
    *   The pallet verifies ownership, checks compatibility (e.g., not same pet, cooldowns met), and potentially consumes a basic prerequisite "breeding consent" item if designed.
    *   It then generates the `OffspringDetails` (containing determined species and DNA hash) and stores it.
    *   Parent pets are placed into a breeding cooldown.
*   **Breeding Cooldowns:** Pets have fixed cooldown periods after breeding.

### 2. Genetic Inheritance & Charter Attributes (MVP Simplification)

    *   **Simplified Genetics in `pallet-breeding` (MVP Focus):**
        *   **Parental Data Retrieval:** `pallet-breeding` calls a method on the `NftBreedingHandler` trait (implemented by `pallet-critter-nfts`), like `get_pet_simple_genetics(&parent_id)`, to fetch essential genetic material: the `dna_hash` (`[u8;16]`) and `species` (`Vec<u8>`) of each parent pet.
        *   **Offspring Species Determination (Conceptual Algorithm in `pallet-breeding`):**
            *   If both parents are of the same species, the offspring inherits that species directly.
            *   If parents are of different species and cross-species breeding is enabled (via a `Config` flag `AllowCrossSpeciesBreeding` in `pallet-breeding`): The offspring has a 50/50 chance of inheriting the species of either parent, determined using `T::RandomnessSource`.
            *   For MVP, new "hybrid" species are not generated from cross-breeding; the offspring will be one of the parent species.
            *   *(Future: Simple fertility items might slightly bias this 50/50 chance).*
        *   **Offspring DNA Hash Generation (Conceptual Algorithm in `pallet-breeding`):**
            *   A new, unique `determined_dna_hash` (`[u8;16]`) is generated for the offspring.
            *   The algorithm combines the `dna_hash` values of both parents with randomness from `T::RandomnessSource`. An example approach: iterate through each of the 16 bytes, for each byte, randomly select the corresponding byte from parent 1, parent 2, or an average-like combination of the two, then XOR this chosen byte with a random byte. This ensures variation even if parents are similar and makes the offspring's DNA distinct.
            *   *(Future: Advanced fertility items could introduce specific "gene splicing" mechanics or increase mutation chances for rare traits by influencing this DNA generation process, but this is post-MVP).*
        *   **`OffspringDetails` Storage:** `pallet-breeding` stores this `determined_dna_hash` and `determined_species` (along with parent IDs and breeder info) in its `PendingOffspring` storage.
    *   **Delegated Stat Derivation in `pallet-critter-nfts` (MVP Focus):**
        *   (As previously documented) When `claim_offspring` is called in `pallet-breeding`, it triggers `T::NftHandler::mint_pet_from_breeding(...)` in `pallet-critter-nfts`, passing the `determined_species` and `determined_dna_hash`.
        *   `pallet-critter-nfts` then uses this specific `determined_dna_hash` (and `determined_species`) with its *existing internal DNA interpretation logic* (the same algorithm used in its regular `mint_pet_nft` extrinsic that derives charter attributes like base stats and elemental affinity from a DNA hash) to generate all the foundational charter attributes for the new Pet NFT. It also records the lineage.
    *   This approach significantly simplifies `pallet-breeding` by making it responsible only for generating the core genetic blueprint (species and DNA), while `pallet-critter-nfts` consistently handles the expression of that blueprint into concrete stats.

### 3. Breeding Scores / Genetic Fitness (Deferred for Post-MVP)

*   **Concept:** Complex systems for "Breeding Scores" or "Genetic Fitness" ratings that influence breeding success or offspring quality are deferred.
*   **MVP Implication:** Breeding success is assumed if prerequisites (ownership, cooldowns) are met. Offspring quality is purely a result of the simplified genetic algorithm above.

### 4. Breeding Tree / Lineage Tracking

*   **Concept:** The `PetNft` struct in `pallet-critter-nfts` (as previously designed) would store lineage information (`parent1_id: Option<PetId>`, `parent2_id: Option<PetId>`, `generation_number: u32`).
*   **Implementation:** This is handled by `pallet-critter-nfts`'s `mint_pet_from_breeding` function, which receives parent IDs.
*   **UI Impact:** Remains valuable for displaying lineage.

### 5. Cross-Species Breeding (MVP Scope)

*   **Concept:** Can be allowed for MVP if a `Config` flag `AllowCrossSpeciesBreeding: Get<bool>` is set in `pallet-breeding`.
*   **Outcome (Simplified):** As per "Offspring Species Determination" above – 50/50 chance of inheriting either parent's species. No new hybrid species.

### 6. Fertility Items & Consumables (MVP Simplification)

*   **Concept:** Most complex fertility items directly influencing genetic outcomes are deferred.
*   **MVP Scope:**
    *   A very basic "Fertility Consent Form" item (from `pallet-items`, category `BreedingAssist`) might be a prerequisite for initiating breeding, consumed by `pallet-breeding` via `T::ItemHandler` (if an `ItemHandler` is kept in `pallet-breeding::Config`). This is optional for MVP.
    *   Items that simply reduce breeding cooldowns could be implemented. These would be used via `pallet-items`'s `user_apply_item_to_pet`, which calls `NftManagerForItems::apply_breeding_assist_effect_to_pet` on `pallet-critter-nfts`. `pallet-critter-nfts` would then need logic to update its cooldown storage (if it manages cooldowns directly) or call back to `pallet-breeding` if cooldowns are managed there.
    *   **Deferral:** Items that directly boost chances of specific stats, traits, or elemental affinities in offspring are post-MVP.
*   **Economic Model & Impact:**
    *   **Breeding Fees (MVP):** A flat PTCN fee (`BreedingFee` in `pallet-breeding::Config`) can be configured. For an MVP, this fee **can be set to zero** in the runtime configuration to encourage participation and simplify the initial economy. If a non-zero fee is used, it's a fixed flat amount, and complex tiered fees or those influenced by fertility items are deferred. The fee, if any, is directed to `BreedingFeeDestination`.
    *   **Fertility Items (MVP Simplification):** Most complex fertility items directly influencing genetic outcomes are deferred. For MVP, basic items like "Fertility Consent Forms" (if implemented as a prerequisite for breeding, consumed via `ItemHandler`) or cooldown reduction items (used via `pallet-items` calling `NftManagerForItems::apply_breeding_assist_effect_to_pet` on `critter-nfts`) would be the primary consumables. These create minor item sinks. The economic impact of more advanced genetic-influencing items is a post-MVP consideration.

### 7. Pallet & System Interactions (MVP Focus)

*   **`pallet-critter-nfts`:**
    *   Implements the `NftBreedingHandler` trait (defined in `pallet-breeding`'s scope).
        *   `get_pet_simple_genetics`: Provides parent DNA and species to `pallet-breeding`.
        *   `mint_pet_from_breeding`: Receives determined DNA and species from `pallet-breeding`, then uses its internal logic to derive all base charter stats, mints the new pet, and records lineage.
    *   May manage breeding cooldowns internally or expose functions for `pallet-breeding` to update them if cooldowns are stored in `pallet-breeding`. (Simpler if `pallet-breeding` manages its own cooldowns based on `PetId`).
*   **`pallet-breeding`:**
    *   Calls `T::NftHandler` (implemented by `critter-nfts`) for parent data and final offspring minting.
    *   Manages `OffspringDetails` and the incubation period.
    *   Manages breeding cooldowns for Pet IDs (e.g., `PetBreedingCooldowns<T: Config> StorageMap<PetId, T::BlockNumber>`).
*   **`pallet-items` (MVP):** Provides basic items like cooldown reducers or optional "consent" items. Interaction for cooldowns is likely: User uses item via `pallet-items` -> `pallet-items` calls `NftManagerForItems::apply_breeding_assist_effect_to_pet` on `critter-nfts` -> `critter-nfts` updates its own cooldown state or calls `pallet-breeding` to update cooldown.
*   **UI Wallet:** UI for selecting pets, initiating breeding, viewing pending offspring (eggs), and claiming them. Lineage display remains relevant.

This simplified Pet Breeding system for MVP focuses `pallet-breeding` on the core mechanics of parent selection, DNA/species determination, and offspring incubation, while leveraging `pallet-critter-nfts` for the actual stat derivation from DNA, ensuring consistency with normally minted pets. Advanced genetic modifiers are deferred.

## 12. Pet Day Cares & Caregiver Roles

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
    *   **Economic Model (MVP Simplification):**
        *   **Service Fees (MVP):** For an MVP, Day Care operators will set a **fixed PTCN fee for a fixed session duration** (e.g., X PTCN for Y blocks of care). This fee is paid upfront by the pet owner upon enrolling their pet. Complex per-block calculations or dynamically variable fees based on caregiver stats (beyond simple existence) are deferred to post-MVP.
        *   **Fee Management (MVP):** `pallet-daycare` (conceptual) will manage the direct transfer of the fixed fee from the pet owner to the day care operator upon successful enrollment. More complex escrow mechanisms and settlement/payout schedules are post-MVP considerations.
        *   **Platform Fees (MVP):** Platform fees for day care services are **deferred** for MVP to simplify the initial economic model and encourage usage.

    #### Conceptual User Interface for Pet Day Cares

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

## 9. Item System (`pallet-items`)

A dedicated Item System, likely managed by a `pallet-items`, will introduce a variety of usable and equippable objects that can affect Pet NFTs, gameplay, and the economy. These items can be earned, crafted (future), or traded.

### 1. Core Item Concepts (MVP Simplification)
*   **Item Definitions:** Each item type will have a definition including its name, description, category, effects, and stackability.
*   **Simplified `ItemCategory` Enum (in `pallet-items`):**
    *   `ConsumableCare`: For basic feed/play items (e.g., "Basic Kibble," "Simple Toy"). Their direct effects (mood/XP boost) are defined in `pallet-critter-nfts::Config` and applied by `feed_pet`/`play_with_pet` extrinsics. `pallet-items` just consumes these items via `BasicCareItemConsumer` trait.
    *   `ConsumableBoost`: For items that provide direct, often permanent or simple temporary stat boosts (e.g., "XP Potion," "Mood Candy"). These are applied by `pallet-items` calling `NftManagerForItems` trait methods on `pallet-critter-nfts`.
    *   `QuestItem`: Key items for quests, may not have direct pet effects but are checked by `pallet-quests`.
    *   `BreedingAssist`: E.g., fertility boosters, items influencing offspring traits. Effects are applied via `NftManagerForItems` calling specific breeding-related functions on `pallet-critter-nfts` or a dedicated breeding pallet.
    *   `SpecialFunctional`: E.g., trait modifiers, items that unlock specific one-time events or features.
    *   **Deferred for Post-MVP:** `Equipment` (implying persistent equipped state and passive bonuses), `Cosmetic` (implying visual changes).
*   **Simplified `ItemEffect` Enum (in `pallet-items`):**
    *   Focuses on effects manageable for MVP, deferring complex temporary buffs with on-chain duration tracking or direct modification of base charter stats.
    *   Examples:
        *   `GrantFixedXp { amount: u32 }`
        *   `ModifyMood { amount: i16 }` (direct change to `mood_indicator`)
        *   `GrantPersonalityTrait { trait_to_grant: Vec<u8> }`
        *   `ModifyBreedingRelatedValue { effect_type_id: u8, value: u32 }` (for specific breeding-related effects handled by `NftManagerForItems` trait on `critter-nfts`)
    *   **Deferred for Post-MVP:** `AttributeBoost` with complex duration/percentage, `ApplyPermanentCharterBoost` (modifying base stats like Strength - should be extremely rare if ever implemented), `ApplyCosmetic`.
*   **User Inventories:** Each user will have an on-chain inventory tracking the quantity of each `ItemId` they own.

### 2. Pallet Structure (`pallet-items` Conceptual Outline - MVP Focus)
*   **`Config` Trait:**
    *   Dependencies: `Currency`.
    *   Requires `type NftHandler: NftManagerForItems<...>` (implemented by `pallet-critter-nfts` for applying specific effects of `ConsumableBoost`, `SpecialFunctional`, `BreedingAssist` items).
    *   The pallet itself implements the `BasicCareItemConsumer<AccountId, ItemId>` trait, providing the `consume_specific_item` function for `pallet-critter-nfts` to call.
    *   Constants: `MaxItemNameLength`, `MaxItemDescriptionLength`, `MaxEffectsPerItem`, `MaxTraitStringLen`.
*   **Storage:**
    *   `NextItemId`: Counter for unique item type IDs.
    *   `ItemDefinitions<ItemId, ItemDetails>`: Stores the properties of each defined item type.
    *   `UserItemInventory<(AccountId, ItemId), Quantity>`: Tracks how many of each item a user owns.
*   **Events:**
    *   `ItemDefined`: When a new item type is added by an admin.
    *   `ItemUsedOnPet`: When a user successfully applies an item (of type `ConsumableBoost`, `SpecialFunctional`, etc.) to their pet via `user_apply_item_to_pet`.
    *   `ItemsTransferred`: When items are moved between users.
    *   `CareItemConsumed`: (Optional, if distinct event needed when `consume_specific_item` is called by `critter-nfts`).
*   **Errors:** Standard errors plus `UseViaDedicatedExtrinsic` (for trying to use `ConsumableCare` items with `user_apply_item_to_pet`), `ItemCategoryMismatch`.
*   **Extrinsics:**
    *   `admin_add_item_definition(...)`: Admin-only, defines new item types with their simplified categories and effects.
    *   `user_apply_item_to_pet(origin, item_id, target_pet_id)`:
        *   For items NOT in `ItemCategory::ConsumableCare`.
        *   Verifies ownership of item and pet.
        *   Consumes the item from `UserItemInventory`.
        *   Iterates through the item's defined `effects` (e.g., `GrantFixedXp`, `ModifyMood`) and calls the corresponding simplified methods on `T::NftHandler` (implemented by `pallet-critter-nfts`).
        *   Emits `ItemUsedOnPet`.
    *   (Internal function, not extrinsic) `consume_specific_item(user, item_id, expected_category)`: Implements the `BasicCareItemConsumer` trait. Called by `pallet-critter-nfts`'s `feed_pet`/`play_with_pet`. Verifies item category and consumes it.

### 3. Interaction with Other Systems (MVP Focus)
*   **`pallet-critter-nfts`:**
    *   **Implements `NftManagerForItems` (Simplified):** Provides methods like `grant_fixed_xp_to_pet`, `modify_mood_of_pet`, `grant_personality_trait_to_pet`, `apply_breeding_assist_effect_to_pet`. These methods are called by `pallet-items`'s `user_apply_item_to_pet` extrinsic for items like `ConsumableBoost` or `SpecialFunctional`. They directly modify the `PetNft` struct.
    *   **Calls `BasicCareItemConsumer` (Implemented by `pallet-items`):**
        *   Its `feed_pet` and `play_with_pet` extrinsics take a `food_item_id` or `toy_item_id`.
        *   They call `T::ItemHandler::consume_specific_item(user, item_id, ItemCategory::ConsumableCare)` on `pallet-items` to verify and consume the item.
        *   After successful consumption, `feed_pet`/`play_with_pet` apply their own defined mood/XP effects from `pallet-critter-nfts::Config` (e.g., `T::FeedMoodBoost`, `T::FeedXpGain`). This decouples basic care effects from item definitions in `pallet-items` for MVP.
*   **Shops & Marketplace (`pallet-marketplace`, `pallet-user-shops`):** All item types will be listable and tradable.
*   **Quests (`pallet-quests`):** `QuestItem` category items can be quest objectives or rewards. `pallet-quests` would interact with `pallet-items` to check/take these items.
*   **Breeding (`pallet-breeding` or logic within `critter-nfts`):** `BreedingAssist` items have their effects applied via `NftManagerForItems::apply_breeding_assist_effect_to_pet`.

This simplified Item System for MVP focuses on core functionalities, clear separation of concerns for basic care versus special item effects, and defers more complex mechanics like equipment and timed buffs.

    #### Conceptual User Interface for Item System

    The UI Wallet will provide an "My Item Inventory" section (`#item-inventory-section`) for users to manage and use their items.

    *   **Displaying Item Inventory (`#item-inventory-list`):**
        *   A list will display all items owned by the user, grouped by `ItemId`.
        *   Each entry will show: Item Name, ID, Quantity (if stackable), Category, Description, and a summary of its Effects.
    *   **Using Items:**
        *   For usable items (e.g., consumables), each item entry will have a dropdown (`.item-target-pet-select`) allowing the user to select one of their own Pet NFTs as the target.
        *   A "Use Item" button (`.use-item-button`) next to the selector would (conceptually) trigger the `user_apply_item_to_pet` extrinsic.
        *   For items like "Equipment," the button might be "Equip (Conceptual)," indicating a more complex equipping system for future development (potentially involving dedicated equipment slots on pets).
    *   **Status Feedback (`#item-action-status`):** Provides feedback on item usage attempts.

## 10. User Score & Reputation System (`pallet-user-profile`)

To quantify user engagement, achievements, and trustworthiness within the CritterCraft ecosystem, a `pallet-user-profile` will be introduced. This pallet will maintain various scores for each user, derived from their on-chain activities, simplified for an MVP.

### 1. Core Concepts (MVP Focus)
*   **Derived Scores:** User scores are calculated by the system when users perform specific actions in other integrated pallets (e.g., completing quests, winning battles, pet development).
*   **Progression & Recognition:** Scores serve as a measure of a user's overall progress and contribution to the MVP ecosystem.
*   **Activity Tracking:** The profile stores the `last_active_block` for a user.

### 2. `UserProfile` Struct Components (MVP Focus)
The core data structure, `UserProfile<BlockNumber>`, for MVP will store:
*   **`total_pet_levels_sum: ScoreValue`**: Aggregate sum of levels of all pets owned by the user.
*   **`quests_completed_count: u32`**: Total number of unique quests completed.
*   **`battles_won_count: u32`**: Total number of battles won.
*   **`overall_progress_score: ScoreValue`**: A weighted composite score derived from the above MVP metrics.
*   **`last_active_block: BlockNumber`**: The block number of the user's last recorded significant on-chain activity.
*   **Deferred for Post-MVP:** `successful_trades_count`, `community_contributions_score`, `trade_reputation_score`. These systems are not part of the core MVP loop or require more complex feedback mechanisms.

### 3. Pallet Structure (`pallet-user-profile` Conceptual Outline - MVP Focus)
*   **`Config` Trait (MVP):**
    *   Includes `RuntimeEvent`.
    *   Defines constants for MVP score weights: `PetLevelScoreWeight`, `QuestScoreWeight`, `BattleWinScoreWeight`. Weights for deferred scores (like `TradeScoreWeight`) are removed.
*   **Storage:**
    *   `UserProfiles<AccountId, UserProfile<BlockNumberFor<T>>>`: Stores the profile data for each user.
*   **Events (MVP):**
    *   `UserProfileUpdated { user, new_overall_score }`. The `TradeReputationChanged` event is removed as the related score is deferred.
*   **Internal Functions (Callable by Other Pallets - MVP Focus):**
    *   The central private helper `update_profile_and_recalculate(user, mutator_fn)` remains.
    *   Public functions for MVP scores: `update_pet_level_sum`, `record_quest_completion`, `record_battle_win`.
    *   `record_user_activity` (to update `last_active_block`) also remains.
    *   Functions related to deferred scores (e.g., `record_successful_trade`, `update_trade_reputation`, `record_community_contribution`) are removed or commented out in the pallet code.
*   **Score Calculation (`recalculate_overall_score` internal function - MVP Focus):**
    *   The formula is simplified to only use MVP score components:
        *   `pet_score = total_pet_levels_sum * PetLevelScoreWeight`
        *   `quest_score = min(quests_completed_count, QUEST_COUNT_SCORE_CAP) * QuestScoreWeight`
        *   `battle_score = min(battles_won_count, BATTLE_WINS_SCORE_CAP) * BattleWinScoreWeight`
        *   `overall_score = pet_score + quest_score + battle_score`
    *   Terms for `trade_activity_score` and `community_contributions_score` are removed from the sum for MVP.
*   **Extrinsics:** None are planned for the MVP of this pallet, as it's primarily service-oriented.

### 4. Impact and Future Uses (Retained Vision)
*   **Gamification:** Provides clear progression metrics for users.
*   **Access Control/Perks:** High scores or specific achievements could grant access to exclusive content, events, quests, or titles.
*   **Governance Influence:** Could be a factor in future advanced governance models (e.g., reputation-weighted voting, eligibility for council).
*   **Matchmaking:** In battles or other systems, scores could be used for fairer or more targeted matchmaking.
*   **Personalized Experiences:** User profile data can help tailor in-game events or offers.

The User Score & Reputation System aims to create a richer, more rewarding experience by recognizing and quantifying player contributions and achievements across the entire CritterCraft platform.

    #### Conceptual User Interface for User Profile & Scores

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

## 8. Pet Development Lifecycle (Conceptual)

Beyond the immutable charter attributes set at minting, Pet NFTs in CritterCraft are dynamic entities that grow and change based on time, owner interactions, and experiences. This lifecycle is primarily managed within `pallet-critter-nfts`, potentially influenced by `pallet-items`.

### a. Time-Based State Changes & Dynamic Attribute Simplification
*   **Concept & Simplification:** Pets' core needs (hunger, energy) are now primarily calculated **off-chain** for UI display, based on on-chain timestamps. Explicit on-chain ticking for these attributes is removed to simplify on-chain state.
*   **On-Chain Timestamps:** The `PetNft` struct in `pallet-critter-nfts` stores:
    *   `last_fed_block: BlockNumberFor<T>`: Updated when `feed_pet` is successfully called.
    *   `last_played_block: BlockNumberFor<T>`: Updated when `play_with_pet` is successfully called. This also serves as a general "last care" timestamp.
    *   `last_state_update_block: BlockNumberFor<T>`: Tracks any significant on-chain interaction.
*   **Off-Chain Calculation:** The UI or game client will use these timestamps to estimate current hunger and energy levels. For example:
    *   `current_hunger = (current_block - last_fed_block) * HUNGER_RATE_PER_BLOCK` (capped).
    *   `current_energy = MAX_ENERGY - (current_block - last_played_block) * ENERGY_DECAY_RATE_PER_BLOCK` (floored at 0).
*   **Simplified On-Chain Mood:**
    *   `mood_indicator: u8` remains in `PetNft` (e.g., 0-Unhappy, up to `T::MaxMoodValue`).
    *   It's primarily updated by direct interactions (`feed_pet`, `play_with_pet` provide boosts defined in `Config`).
    *   **Neglect Check:** A simplified on-chain function `apply_neglect_check` (callable extrinsic) can be triggered (e.g., by user, or potentially an off-chain worker in the future). If `current_block - pet.last_played_block > T::NeglectThresholdBlocks::get()`, `mood_indicator` decreases by `T::NeglectMoodPenalty::get()`. This is not a continuous tick.

### b. Interaction-Driven Development (`feed_pet`, `play_with_pet`)
*   **Concept:** Owners actively develop their pets by interacting with them, primarily by providing basic care items (food, toys).
*   **Implementation (Simplified Extrinsics in `pallet-critter-nfts`):**
    *   `feed_pet(origin, pet_id, food_item_id: ItemId)`:
        *   Verifies ownership.
        *   Calls `T::ItemHandler::consume_item_if_category(&owner, food_item_id, pallet_items::ItemCategory::Food)`. This trait, implemented by `pallet_items`, verifies the item is appropriate (e.g., is food) and consumes it from the user's inventory.
        *   Updates `pet.last_fed_block` to the current block.
        *   Increases `pet.mood_indicator` by `T::FeedMoodBoost::get()` (capped at `T::MaxMoodValue::get()`).
        *   Grants `pet.experience_points` by `T::FeedXpGain::get()`.
        *   Calls `attempt_level_up(pet)`.
        *   Updates `pet.last_state_update_block`.
    *   `play_with_pet(origin, pet_id, toy_item_id: ItemId)`:
        *   Similar flow: Verifies ownership, calls `T::ItemHandler::consume_item_if_category` (for `ItemCategory::Toy`).
        *   Updates `pet.last_played_block`.
        *   Increases `pet.mood_indicator` by `T::PlayMoodBoost::get()`.
        *   Grants `pet.experience_points` by `T::PlayXpGain::get()`.
        *   Calls `attempt_level_up(pet)`.
        *   Updates `pet.last_state_update_block`.
*   **Complex Item Effects:** More complex stat-boosting items or items granting specific abilities would still use the more detailed `NftManagerForItems` trait, called by `pallet-items` when such an item is used, allowing for targeted effects beyond basic care.

### c. Leveling and Experience (XP)
*   **Concept:** Pets gain Experience Points (XP) through interactions like simplified feeding and playing (as above), completing quests (via `pallet-quests`), and winning battles (via `pallet-battles`).
*   **Implementation Idea (`attempt_level_up` internal function in `pallet-critter-nfts`):**
    *   Whenever a pet gains XP, this internal function is called.
    *   **XP Curve:** A defined XP curve (e.g., `next_level_xp = BASE_XP_PER_LEVEL * pet.level` or a more complex formula using `Config` constants) determines the XP needed for the next level.
    *   **Level Up:** If XP exceeds required amount:
        *   Level increases.
        *   XP is reduced by the amount needed (excess XP carries over).
        *   **Effects of Leveling:** For MVP, leveling primarily increases the `level` attribute itself. This `level` is then a crucial input for battle calculations, quest eligibility, breeding eligibility, etc. Future enhancements could see leveling grant skill points or minor base stat increases.
    *   An event like `PetLeveledUp { pet_id, new_level }` is emitted.

    ### d. Personality Trait Evolution (Driven by AI Personality Engine)
    *   **Concept:** Personality traits (`personality_traits` field in `PetNft`) are not just static but can evolve based on a pet's cumulative experiences and interactions. This is facilitated by a conceptual **off-chain AI Personality Engine** (see `AI_PERSONALITY_ENGINE.md` for full details).
    *   **Engine Function:** This engine analyzes a pet's comprehensive on-chain history (care interactions, battle performance, quests completed, items used, etc.) to identify patterns and suggest new traits or modifications to existing ones (e.g., gaining "Brave" after many difficult battle wins, or "Picky" if only fed high-quality food). The engine considers factors like frequency, significance, and type of experiences.
    *   **Integration with `pallet-critter-nfts` (Owner-Approved for MVP):**
        *   For the MVP, the AI engine's primary role is to provide *suggestions* for personality changes to the pet owner (e.g., via the UI Wallet or a companion application).
        *   The owner retains agency and chooses whether to accept these suggestions.
        *   If accepted, the owner calls the existing `update_pet_metadata` extrinsic in `pallet-critter-nfts`, providing the new, full list of `personality_traits` for their pet. This method ensures player control over on-chain trait modifications while benefiting from AI-driven insights.
    *   **Data Sources for Engine:** The AI engine would require access to historical event data from multiple pallets (`pallet-critter-nfts`, `pallet-battles`, `pallet-quests`, `pallet-items`, etc.), likely through blockchain indexers.
    *   **Impact:** This system aims to make pet personalities feel genuinely emergent, deeply personalized, and reflective of their unique journey and the owner's play style within CritterCraft.

This simplified lifecycle focuses on essential on-chain updates (timestamps, mood boosts, XP) for core interactions, while enabling richer off-chain calculations for dynamic states like hunger and energy, reducing on-chain storage and transaction load for the MVP.
```
