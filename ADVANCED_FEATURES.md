# CritterCraft: Advanced Features & Gameplay Loops (Conceptual Outline)

This document provides a high-level conceptual outline for advanced economic loops and gameplay features planned for future stages of CritterCraft development, building upon the foundations of Stage 5.

## 1. Sophisticated User Shops

*   **Concept:** Allow players to set up their own persistent, customizable shops within the CritterCraft ecosystem to sell Pet NFTs, items (food, clothes, equipment - from a future Item Pallet), or even services (e.g., "pet training," "battle assistance").
*   **Potential Pallet(s):** Could be an enhancement to `pallet-marketplace` or a new `pallet-user-shops`.
*   **Key Features (Conceptual):**
    *   **Shopfront NFTs:** Users might mint a "Shopfront NFT" to create and customize their shop.
    *   **Inventory Management:** On-chain or off-chain linked inventory of items/NFTs for sale.
    *   **Pricing & Fees:** Users set prices in PTCN. The system might take a small transaction fee.
    *   **Customization:** Options for shop appearance (if a visual frontend is developed) or description.
    *   **Reputation System:** Ratings/reviews for shops and sellers.
*   **Economic Impact:** Creates a player-driven economy, encourages entrepreneurship, and provides more sinks/uses for PTCN and items.

## 2. Advanced Blockchain Support Jobs

*   **Concept:** Expand beyond simple daily claims to more involved tasks that genuinely contribute to the ecosystem's health, data curation, or community moderation, rewarding users with PTCN or other benefits.
*   **Potential Pallet(s):** A new `pallet-jobs-board` or integration with governance/community pallets.
*   **Key Features (Conceptual):**
    *   **Job Listings:** A board where tasks are listed with requirements and rewards.
    *   **Data Curation/Validation:** E.g., verifying user-generated content (if applicable in the future), tagging items, or validating off-chain game results for specific events.
    *   **Light Node Operation Incentives:** Rewards for users running light client nodes that support network decentralization.
    *   **Community Moderation Tasks:** If social features are added, trusted users could be rewarded for moderation.
    *   **Oracle Participation:** If off-chain data is needed (e.g., for specific quest types or dynamic events), users could be incentivized to act as oracle providers.
*   **Economic Impact:** Provides more diverse earning opportunities, incentivizes useful work for the ecosystem.

## 3. Treasure Hunts & Exploration

*   **Concept:** Engage players in complex, multi-stage quests or puzzles that may involve on-chain and off-chain elements (e.g., solving riddles, finding hidden information, interacting with specific game locations or NFTs). Rewards would be rare NFTs or significant PTCN bounties.
*   **Potential Pallet(s):** Could extend `pallet-quests` or be a new `pallet-treasure-hunts`.
*   **Key Features (Conceptual):**
    *   **Multi-Stage Puzzles:** Requiring specific on-chain actions or item combinations to progress.
    *   **Location-Based Elements (Future - Mobile):** If a mobile component is ever developed, could involve AR or real-world location check-ins.
    *   **Time-Limited Events:** Special treasure hunts available for a limited duration.
    *   **Unique NFT Rewards:** Exclusive Pet NFTs, items, or cosmetic "bragging rights" NFTs.
*   **Economic Impact:** Creates excitement, drives demand for specific items/NFTs needed for quests, and provides high-value rewards.

## 4. Mini-Games & Arcade

*   **Concept:** Introduce a variety of simple, engaging mini-games where players can compete or play solo for fun and PTCN/item rewards.
*   **Potential Pallet(s):** A `pallet-arcade` or individual mini-game pallets.
*   **Key Features (Conceptual):**
    *   **On-Chain Verifiable Games:** Simple games where outcomes can be determined and verified on-chain (e.g., rock-paper-scissors with commit-reveal, dice rolls, simple card games).
    *   **Off-Chain Games with On-Chain Rewards:** More complex games played off-chain, with results reported to a pallet that distributes rewards (similar to the battle system's oracle model).
    *   **Leaderboards:** On-chain leaderboards for high scores or win streaks, potentially with periodic PTCN rewards for top players.
    *   **Entry Fees & Prize Pools:** Some games might have small PTCN entry fees that contribute to prize pools.
*   **Economic Impact:** Provides entertainment, more earning avenues, and PTCN sinks/recirculation.

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
