# CritterCraft Tokenomics: PetCoin (PTCN)

This document details the economic model, utility, supply, allocation, and flow of PetCoin (PTCN), the native utility and governance token of the CritterChain ecosystem.

## 1. Introduction: The Role of PTCN

PetCoin (PTCN) is the lifeblood of the CritterCraft universe. It is designed to:
*   Facilitate economic activity and value exchange within the game.
*   Incentivize participation in network security (staking) and governance.
*   Reward players for engagement and contributions to the ecosystem.
*   Provide a sustainable economic model for the long-term development and operation of CritterChain.

## 2. Total Supply & Initial Allocation

*   **Total Supply (Conceptual):** A fixed total supply of **1,000,000,000 PTCN** (1 Billion PTCN) will be minted at genesis. (This can be adjusted based on further modeling).
*   **Decimal Places:** 18 (standard for Substrate-based tokens, allowing for fine granularity).
*   **Initial Allocation (Illustrative Percentages):**
    *   **Staking Rewards Pool & Initial Network Incentives (35%):**
        *   To bootstrap network security via NPoS staking rewards over many years.
        *   Includes initial pools for quest rewards, battle rewards, daily claims to drive early engagement.
    *   **Ecosystem Development & Foundation/Treasury (30%):**
        *   Funds for ongoing development, grants for third-party developers, community initiatives, partnerships, marketing, and operational costs.
        *   A significant portion will be managed by the on-chain Treasury, controlled by PTCN governance.
    *   **Core Team & Advisors (20%):**
        *   To reward past and future contributions of the core development team and strategic advisors. Subject to vesting.
    *   **Early Backers/Private Sale (if applicable) (10%):**
        *   For initial funding rounds. Subject to vesting. (If no private sale, this portion could be reallocated to Ecosystem/Treasury or Community Incentives).
    *   **Community & Airdrop Pool (5%):**
        *   For strategic airdrops to target communities (e.g., Substrate users, NFT gamers), testnet participation rewards, and other early community growth initiatives.

## 3. Vesting Schedules (Conceptual)

To ensure long-term alignment, tokens allocated to the Core Team, Advisors, and any Early Backers will be subject to vesting schedules.
*   **Team & Advisors:** E.g., 4-year linear vesting with a 12-month cliff. (Meaning tokens unlock gradually over 4 years after an initial 1-year lock-up).
*   **Early Backers:** E.g., 2-3 year linear vesting with a 6-12 month cliff, potentially with a portion unlocked at Token Generation Event (TGE) if applicable.

## 4. Inflation Model (for Staking Rewards)

*   **Primary Source:** The initial "Staking Rewards Pool & Initial Network Incentives" allocation (35% of total supply) is designed to cover staking rewards for a significant period (e.g., 5-10 years, depending on emission rate).
*   **Sustainable Long-Term Inflation (Post-Initial Pool):** Once the initial pool depletes, or to ensure perpetual staking incentives, a low, controlled annual inflation rate (e.g., **2-5% of the *then-current* total supply**) can be introduced via on-chain governance. This inflation would be directed primarily to `pallet-staking` for distribution as rewards to active validators and nominators.
*   The exact inflation parameters (rate, activation) will be governable by PTCN holders.

## 5. Deflationary Mechanisms & Value Accrual

Several mechanisms are designed to create PTCN sinks, reduce circulating supply, or increase demand, contributing to value accrual:

*   **Transaction Fees:** While a portion of transaction fees might cover network operational costs (e.g., collator rewards if a parachain, or validator rewards if not fully covered by inflation), a percentage could be burned or sent to the Treasury. This will be subject to governance.
*   **Marketplace Fees (`pallet-marketplace`):** A portion of the `MarketplaceFixedFee` (if non-zero and configured by the runtime) can be burned or allocated to the Treasury. For MVP, this fee might be zero or very low.
*   **User Shop Fees (Future):** Similar to marketplace fees, a platform fee on User Shop sales could be introduced post-MVP, with a portion potentially burned or sent to the Treasury.
*   **Breeding Fees (`pallet-breeding`):** The `BreedingFee` (if non-zero and configured by the runtime) can be partially burned or sent to the Treasury. For MVP, this fee might be zero.
*   **Item Sinks (`pallet-items` - Future):**
    *   Purchasing rare/powerful items from a system-controlled shop (if implemented) could burn PTCN or send it to the Treasury.
    *   Specific high-value crafting recipes might require PTCN as a component, which is then burned.
    *   Repairing high-tier equipment (if implemented) might cost PTCN.
*   **Treasury Burns:** Governance can vote to burn a portion of unspent funds in the `pallet-treasury` periodically if the accumulated amount is deemed excessive.
*   **Governance Proposal Bonds:** PTCN bonded for proposals might be slashed (and potentially burned or sent to Treasury) if proposals are malicious, spam, or fail to meet certain criteria (e.g., not reaching quorum).
*   **Staking Lock-up:** PTCN bonded for staking (by validators and nominators) is temporarily removed from circulating supply, reducing liquid supply and potentially increasing market scarcity.
*   **Penalties/Slashing:** Staked PTCN lost due to validator misbehavior (slashing) is removed from the offender's control. A portion of this might be burned or reallocated to the Treasury, further reducing supply or funding community goods.

## 6. PTCN Utility (Consolidated)

PTCN is integral to interacting with and participating in the CritterCraft ecosystem:

*   **Staking & Network Security:**
    *   Bonding PTCN to become a validator on CritterChain.
    *   Staking PTCN to nominate validators and earn a share of staking rewards.
*   **Governance:**
    *   Voting on public referenda via `pallet-democracy` (e.g., runtime upgrades, parameter changes, treasury spending).
    *   Proposing referenda (requires a PTCN bond).
    *   (Future) Voting for council members in `pallet-elections-phragmen`.
    *   (Future) Submitting/bonding treasury proposals via `pallet-treasury`.
*   **Medium of Exchange:**
    *   Buying and selling Pet NFTs on the primary `pallet-marketplace`.
    *   (Future) Buying and selling Pet NFTs and various game items (food, equipment, cosmetics, trait modifiers, fertility boosters) in player-operated `pallet-user-shops`.
    *   (Future) Paying fees for services in `pallet-daycare` (e.g., boarding pets).
*   **Accessing Features & Gameplay:**
    *   Paying the `BreedingFee` (if non-zero) via `pallet-breeding`.
    *   (Future) Paying entry fees for special battle tournaments or arcade mini-games.
    *   (Future) Purchasing specific items or services directly from system-controlled entities (e.g., special NPC shops, advanced pet customization services).
    *   (Future) Consuming PTCN as a component in high-level item crafting recipes or for other specialized in-game actions.

## 7. PTCN Sinks & Sources (Simplified Flow)

This illustrates the primary ways PTCN enters and leaves active user circulation or the total supply.

*   **Sources (Flowing to Users/Ecosystem Participants):**
    *   Initial Allocations (Team, Advisors, Early Backers - subject to vesting schedules).
    *   Ecosystem Development Fund / Treasury disbursements (via governance proposals for grants, initiatives).
    *   Community & Airdrop Pool distributions (for early adoption and community growth).
    *   Staking Rewards (from `pallet-staking` - sourced from the initial rewards pool or future inflation).
    *   Quest Rewards (from `pallet-quests` - funded by ecosystem allocation or treasury).
    *   Battle Rewards (from `pallet-battles` - funded by ecosystem allocation or treasury).
    *   Daily PTCN Claims (from `pallet-critter-nfts` - funded by ecosystem allocation or treasury).
    *   (Future) Advanced Job Payouts (from `pallet-jobs-board` - funded by job posters or ecosystem fund).
    *   (Future) Sales revenue for users operating shops or day cares (transfers between users, less any platform fees).

*   **Sinks (Reducing Circulating Supply or Total Supply):**
    *   **Locks (Temporary Removal from Active Circulation):**
        *   Staking bonds (validator self-bonds and nominator stakes).
        *   Governance proposal bonds (for public referenda, treasury proposals).
        *   (Future) Escrow for services (e.g., advanced day care fees before payout, job bonds).
    *   **Fees (Potential for Burn/Treasury Allocation, reducing circulating supply):**
        *   `MarketplaceFixedFee` from `pallet-marketplace` (if configured).
        *   `BreedingFee` from `pallet-breeding` (if configured).
        *   (Future) User Shop platform fees.
        *   (Future) Day Care platform fees.
        *   (Future) Mini-game entry fees (a portion might go to platform/treasury).
        *   Transaction fees (a portion may go to treasury/burn).
    *   **Burns (Permanent Removal from Total Supply):**
        *   A designated portion of various fees (if governance configures this).
        *   Specific item crafting recipes requiring PTCN burn.
        *   Governance-approved treasury burns.
        *   Slashed proposal bonds (if configured to burn).
        *   Slashed staking bonds (a portion might be burned).
    *   **Spending (Transfers between users or to system for services/items - doesn't reduce total supply but affects circulation):**
        *   Purchasing NFTs/items from other players or shops.
        *   Paying for day care services.
        *   (Future) Purchasing items from system shops.

*(A visual diagram illustrating these flows, such as a Sankey diagram, would be highly beneficial for clarity in more detailed documentation).*

## 8. Future Considerations

*   **Tokenomic Parameter Adjustments:** Many parameters (fee rates, reward amounts, inflation rates, burn rates, bond amounts) will be governable by PTCN holders via the on-chain governance mechanism. This allows the community to adapt the economy as the ecosystem matures and its needs change.
*   **Liquidity & Exchange Listings:** Post-launch and after achieving sufficient decentralization and stability, strategies for ensuring PTCN liquidity on decentralized exchanges (DEXs) within the Polkadot ecosystem (via XCM) and potentially on reputable centralized exchanges (CEXs) will be explored to enhance broader accessibility and utility of PTCN.
*   **Collateralization:** As PTCN gains stability and value, it could potentially be used as collateral in DeFi protocols within the Polkadot ecosystem or even on other chains via bridges.

This tokenomic model aims to create a balanced, sustainable, and engaging economy for CritterCraft, rewarding participation, securing the network, and driving value within the ecosystem for the long term.
```
