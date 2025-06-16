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
*   **Economic Impact:** Fosters a player-driven economy, encourages entrepreneurship, provides more sinks/uses for PTCN, and allows for specialized trading hubs.

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

## 3. Treasure Hunts & Exploration

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

## . Competitive Pet Battles (New Section Conceptual Placement)

CritterCraft will feature a robust system for competitive pet battles, where Pet NFTs engage in strategic combat.

### Battle Mechanics & On-Chain Attributes:
Battle calculations will explicitly factor in each participating Pet NFT's on-chain charter attributes (`base_strength`, `base_agility`, `base_intelligence`, `base_vitality`, `primary_elemental_affinity`) as foundational stats. These will be combined with dynamic attributes (level, experience) and any temporary or permanent buffs/debuffs applied by items (from the conceptual `pallet-items`) to determine effective combat stats. Personality traits may also provide situational modifiers. The `pallet-battles` will manage battle registration, outcome reporting, and potentially reward distribution, while the core battle simulation might occur off-chain or through more complex on-chain logic in future iterations, always referencing these on-chain stats as the source of truth for a pet's capabilities.

## 8. Future Stage: Pet Breeding & Genetics

A comprehensive Pet Breeding and Genetics system is envisioned as a major future stage for CritterCraft, adding significant depth to pet collection, strategy, and the in-game economy. This system will allow players to breed their Pet NFTs to create new, potentially unique offspring.

### 1. Core Breeding Mechanic

*   **Concept:** Owners of two compatible Pet NFTs (see "Compatibility" below) can choose to breed them together to produce a new Pet NFT (an "egg" or "newborn" pet).
*   **Pallet Interaction/New Pallet (`pallet-breeding` or extend `pallet-critter-nfts`):**
    *   An extrinsic like `initiate_breeding(origin, parent1_pet_id: PetId, parent2_pet_id: PetId, fertility_item_id: Option<ItemId>)` would be called.
    *   The pallet would verify ownership of parent pets, check compatibility, consume any fertility items, and potentially place parent pets into a temporary "breeding cooldown" status.
    *   A new Pet NFT would be minted, its charter attributes and initial state determined by the genetic inheritance logic.
*   **Breeding Cooldowns:** Pets may have cooldown periods after breeding before they can breed again to prevent overpopulation and add strategic depth.

### 2. Genetic Inheritance & Charter Attributes

*   **Foundation:** The on-chain charter attributes (`base_strength`, `base_agility`, etc., and `primary_elemental_affinity`) of parent pets, along with their `dna_hash` and `initial_species`, will be primary inputs into the genetic algorithm. This algorithm, potentially influenced by fertility items (from `pallet-items`), will determine the `dna_hash`, `initial_species`, and the explicit on-chain charter attributes for the offspring. This ensures a clear lineage and heritability of core potentials.
*   **Algorithm (Conceptual):**
    *   The new pet's `dna_hash` would be a unique hash, possibly derived algorithmically from parents' DNA hashes, ensuring uniqueness while maintaining a "genetic link."
    *   `initial_species` of the offspring could be determined by parental species (e.g., same as one parent, a hybrid if cross-species breeding is enabled, or weighted random chance).
    *   The explicit **on-chain charter attributes** for the offspring (e.g., `base_strength`, `base_agility`, `primary_elemental_affinity`) would be calculated based on:
        *   A combination of the parents' corresponding charter attributes (e.g., average, weighted average, min/max ranges).
        *   Small random variations (utilizing `T::RandomnessSource` in `pallet-breeding`).
        *   Potential influence from `dna_hash` to introduce further uniqueness.
        *   Effects from any consumed **fertility items** (e.g., an item from `pallet-items` might provide a boost to certain stats in offspring or increase the chance of inheriting a rare elemental affinity).
*   **Personality Trait Inheritance:** Dynamic `personality_traits` might also have a chance to be influenced by parents, or the newborn pet could start with a neutral set to be developed by the owner. The focus for genetic inheritance is primarily on the immutable charter attributes.

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
*   **Economic Loop:** These items would be craftable, earnable through quests/battles, or purchasable from NPC shops or User Shops, creating demand and PTCN sinks.

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

## 9. Future Feature: Pet Day Cares & Caregiver Roles

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

## 10. Item System (`pallet-items`)

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
    *   `AttributeBoost`: Temporarily or permanently modify a pet's specific attribute (e.g., `base_strength`, `mood_indicator`, `fertility`). Requires `PetAttributeType` enum.
    *   `GrantPersonalityTrait`: Adds a new personality trait to a pet.
    *   `ModifyFertility`: Affects breeding parameters.
*   **User Inventories:** Each user will have an on-chain inventory tracking the quantity of each `ItemId` they own.

### 2. Pallet Structure (`pallet-items` Conceptual Outline)
*   **`Config` Trait:**
    *   Dependencies: `Currency` (for item costs if bought from system), `NftHandlerForItems` (a new trait interface for `pallet-critter-nfts` to apply item effects to pets).
    *   Constants: `MaxItemNameLength`, `MaxItemDescriptionLength`, `MaxEffectsPerItem`.
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
    *   `admin_add_item_definition(...)`: Admin-only call to create new item types.
    *   `user_apply_item_to_pet(origin, item_id, target_pet_id)`: Allows a user to use an item from their inventory on one of their pets. This extrinsic will:
        1.  Verify item ownership and quantity.
        2.  Verify pet ownership (via `NftHandlerForItems`).
        3.  Consume the item (if consumable or stackable).
        4.  Call appropriate functions on `T::NftHandlerForItems` to apply the item's `effects` to the `target_pet_id`.
    *   Future: `transfer_item`, `buy_item_from_system_shop`.

### 3. Interaction with Other Systems
*   **`pallet-critter-nfts`:** Will need to implement the `NftHandlerForItems` trait, providing functions like `apply_attribute_boost(pet_id, attribute, value, duration)`, `grant_personality_trait(pet_id, trait)`, `modify_pet_fertility(pet_id, boost)` which would internally call `update_pet_metadata` or specialized logic.
*   **Shops & Marketplace:** Items will be tradable in User Shops and potentially the general Marketplace.
*   **Quests & Battles:** Items can be given as rewards or be required for certain quests/battle conditions.
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

## 11. User Score & Reputation System (`pallet-user-profile`)

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
*   `overall_progress_score: ScoreValue`: A weighted composite score calculated from other metrics, providing a single measure of overall engagement and achievement.
*   `trade_reputation_score: i32`: A score that can increase or decrease based on feedback from trading partners (requires a future feedback mechanism).
*   `last_active_block: BlockNumber`: The block number of the user's last recorded significant on-chain activity.

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
    *   Conceptual `on_initialize` hook placeholder for potential general activity tracking (though specific updates via functions are more direct).
*   **Internal Functions (Callable by Other Pallets):**
    *   The pallet will expose public Rust functions like `record_user_activity(user)`, `update_pet_level_sum(user, new_sum)`, `record_quest_completion(user)`, `record_battle_win(user)`, `record_successful_trade(user)`, `update_trade_reputation(user, change)`, and `record_community_contribution(user, score_increase)`.
    *   These functions will be called by `pallet-critter-nfts`, `pallet-quests`, `pallet-battles`, `pallet-marketplace`, governance pallets, and job systems respectively, upon relevant events occurring in those pallets. Each of these functions will also update the user's `last_active_block`.
    *   An internal `recalculate_overall_score` function will update the composite score whenever a contributing metric changes.
*   **Extrinsics:** Initially, this pallet may have few or no direct user-callable extrinsics for modifying scores. Future extrinsics might allow users to set cosmetic profile elements (e.g., display name, avatar link) if the `UserProfile` struct is expanded.

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
