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

## 5. (Future Consideration) IoT Device Integration

*   **Concept:** Allow real-world data from Internet of Things (IoT) devices (e.g., smart pet feeders, activity trackers for real pets if users have them and want to link) to influence in-game pet stats or trigger on-chain events/rewards.
*   **Technical Challenges:** Requires secure oracle solutions to bring IoT data on-chain reliably.
*   **Impact:** Blurs the lines between the virtual and real world, offering unique engagement. This is a highly speculative and long-term idea.

These advanced features aim to create a deeply engaging, economically vibrant, and evolving world for CritterCraft players. Each will require careful design and phased implementation.

## 6. Staking UI V2 - Enhanced Interactions (Conceptual)

Building upon the initial staking UI, future enhancements will provide a more comprehensive and interactive experience, reflecting the detailed NPoS mechanics outlined in `CONSENSUS_MIGRATION.md`.

### a. My Staking Dashboard Enhancements

*   **Era Information:**
    *   Display current era index.
    *   Show estimated time until the next era (based on average block time and `SessionsPerEra` / `EpochDuration`).
    *   Display session progress within the current era.
*   **Nomination Management:**
    *   **Change Nominations:** Allow users to easily change their set of nominated validators by calling `staking::nominate` with a new list.
    *   **Stop Nomination:** UI to call `staking::chill` to stop nominating and make the user inactive in the next era (a prerequisite for full unbonding if they wish to withdraw all funds).
*   **Staking Rewards:**
    *   **Payout Information:** Display if nominated validators have recently triggered payouts.
    *   **Claim Payouts Button:** If `payout_stakers` can be called by the nominator for their own rewards (or for any staker for a given validator/era), this button would trigger it. It would need to take `validator_stash` and `era` as parameters. (This depends on `pallet-staking`'s configuration and helper pallets like `staking-rewards-collector`). For now, it's a placeholder for a more active reward management if applicable.
    *   Display historical reward points and payouts if queryable.
*   **Bonding & Unbonding:**
    *   **Bond More Funds:** UI to call `staking::bond_extra`.
    *   **Unbond Funds:** The existing UI for `staking::unbond` will be maintained.
    *   **Rebond Funds:** UI to call `staking::rebond` for funds that are currently unbonding but not yet withdrawable.
    *   **Withdraw Unbonded:** The existing UI for `staking::withdraw_unbonded` will be maintained, clearly showing when chunks are withdrawable.
    *   Display `BondingDuration` and `SlashDeferDuration` dynamically from chain constants.
*   **Slashing Information:**
    *   Display warnings about slashing risks.
    *   (Advanced) If an account has been slashed, display information about the event and amount if available from `pallet-staking` or `pallet-offences` events.

### b. Validator List Enhancements

*   **Detailed Stats:** Show more detailed validator stats like era points, number of nominators, commission history, and any past offences.
*   **Filtering & Sorting:** Allow users to sort/filter validators by commission, total stake, identity, etc.
*   **Nomination Interface:** Improve the nomination process, perhaps allowing users to select multiple validators and submit a single `nominate` transaction.
*   **Validator Alerts:** (Future) Allow users to subscribe to alerts if their nominated validators change commission significantly or get slashed.

### c. Becoming a Validator (Informational)

*   Provide more detailed information or links to guides on how to become a validator, including hardware requirements, key management, and the `validate` and `set_keys` extrinsics.

These V2 UI enhancements aim to make staking more transparent, manageable, and user-friendly, empowering PTCN holders to effectively participate in network security and governance.
