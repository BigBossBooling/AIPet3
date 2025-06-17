# CritterCraft Governance Model (Conceptual Outline)

This document outlines the conceptual framework for the future decentralized governance of the CritterCraft ecosystem, powered by PetCoin (PTCN).

## 1. Core Principles

*   **Community-Driven:** The long-term development and key parameter changes in CritterCraft will be guided by PTCN token holders.
*   **Transparency:** All governance proposals, votes, and outcomes will be publicly auditable on CritterChain.
*   **Incentivization:** Active participation in governance may be incentivized in future iterations.

## 2. PetCoin (PTCN) as Governance Token

*   **Voting Power:** Each PTCN token will represent a certain amount of voting power (e.g., 1 PTCN = 1 vote).
*   **Staking for Votes (Future):** A staking mechanism might be introduced where users lock PTCN to participate in voting, potentially increasing their voting weight or earning rewards for participation.

## 3. Proposal Lifecycle (Conceptual)

The proposal lifecycle will be managed by a suite of standard FRAME governance pallets, primarily `pallet-democracy` for public proposals and `pallet-collective` for council motions and actions.

1.  **Proposal Submission:**
    *   Any PTCN holder meeting a minimum token threshold (e.g., holding 0.1% of total supply) can submit a proposal.
    *   Proposals might require a deposit of PTCN, which is returned if the proposal passes a basic sanity check or is voted upon, and slashed if it's spam or malicious.
    *   Proposals would be for specific on-chain actions (e.g., changing a runtime parameter, funding a community project from a treasury, approving a new feature set).

2.  **Voting Period:**
    *   Once a proposal is active, a defined voting period begins (e.g., 7 days).
    *   PTCN holders can cast their votes (Yes, No, Abstain) weighted by their token holdings at a specific snapshot block.

3.  **Tallying & Execution:**
    *   At the end of the voting period, votes are tallied.
    *   A proposal passes if it meets quorum (minimum total voter turnout) and a certain threshold of 'Yes' votes (e.g., simple majority, or supermajority for critical changes).
    *   If passed, the proposal can be enacted. For runtime upgrades or parameter changes, this might involve scheduling the dispatch of a root-level call.

## 4. Governable Parameters & Features (Examples)

The community could potentially vote on:

*   **Economic Parameters:**
    *   Marketplace transaction fees.
    *   Daily PTCN claim amounts.
    *   Battle reward amounts.
    *   Quest reward calibration.
*   **Game Features & Development:**
    *   Prioritization of new features (e.g., new pet species, new item categories, advanced battle modes).
    *   Approval of community-developed content or pallets.
    *   Changes to core game mechanics (e.g., pet evolution rules, battle formulas - within limits).
*   **Treasury Management (Future):**
    *   If a portion of network fees or other revenue goes to a decentralized treasury, PTCN holders would govern its use (e.g., funding development grants, community initiatives).
*   **Protocol Upgrades:**
    *   Authorizing runtime upgrades for CritterChain.

## 5. Phased Rollout

*   **Initial Conceptual Phase:** Conceptual outline, key pallet identification (`pallet-democracy`, `pallet-collective`, `pallet-treasury`), and UI placeholders were established.
*   **Future Stages:**
    *   Implementation of a basic `pallet-governance`.
    *   Introduction of simple proposal types (e.g., text-based polls).
    *   Gradual expansion to more complex on-chain parameter changes and treasury management.

This governance model aims to empower the CritterCraft community and ensure the platform's sustainable and decentralized evolution.

## 6. Key FRAME Pallets for Full Governance

To implement the full decentralized governance vision for CritterChain, the following standard FRAME pallets are planned for integration:

### a. `pallet-democracy`

*   **Role:** Enables public referenda, allowing all PTCN holders to vote on proposals. This is the primary mechanism for community-wide decision-making.
*   **Key Features & Configuration Considerations:**
    *   **Public Proposals:** PTCN holders can propose referenda (potentially requiring a bond or minimum stake).
    *   **Voting Mechanism:** Token-weighted voting (1 PTCN = 1 vote, or conviction voting where locking tokens for longer grants more voting power).
    *   **Referendum Lifecycle:** Defined periods for proposal submission, voting, enactment delays, and emergency proposals.
    *   **Launch Period:** Initial period for referenda to be submitted at a faster rate.
    *   **Minimum Deposit for Proposals:** To prevent spam.
    *   **Voting Period:** Duration for which voting is open.
    *   **Enactment Period:** Delay between a referendum passing and its execution, allowing for preparation or objection.
    *   **Cooloff Period:** Minimum time between identical proposals.
    *   **Vote Locking:** PTCN used for voting is typically locked for at least the duration of the vote.
    *   **Integration with Council:** Proposals can be fast-tracked or vetoed by a technical council (see `pallet-collective`).

### b. `pallet-collective` (The Council)

*   **Role:** Establishes one or more collective bodies, such as a Technical Council or Community Council. These bodies can have special privileges, like proposing referenda, fast-tracking proposals, vetoing malicious proposals, or even acting as an "AdminOrigin" for certain pallet extrinsics.
*   **Key Features & Configuration Considerations:**
    *   **Membership:** Defines how members are elected or appointed (often via `pallet-elections-phragmen` or a simpler `EnsureProportion` origin).
    *   **Instance Configuration:** Can configure multiple instances of `pallet-collective` for different bodies with different roles (e.g., a Technical Council focused on upgrades, a Community Council focused on treasury proposals for ecosystem growth).
    *   **Proposal Submission:** Council members can submit proposals to the collective.
    *   **Voting within the Collective:** Defines how council members vote on proposals (e.g., simple majority).
    *   **Dispatching Calls:** Approved council proposals can dispatch calls, either as the collective's origin or as Root (if configured).
    *   **Prime Member:** Optionally, a prime member can be designated to break ties or act as a default proposer.

### c. `pallet-treasury`

*   **Role:** Manages an on-chain treasury funded by various sources (e.g., a portion of transaction fees, marketplace fees, slashing penalties, or direct contributions). PTCN holders (via `pallet-democracy`) or the Council (via `pallet-collective`) can propose to spend these funds.
*   **Key Features & Configuration Considerations:**
    *   **Proposal Submission:** Spenders (typically the Council) can propose to allocate treasury funds for specific purposes (e.g., development grants, marketing, bug bounties, community events).
    *   **Bonding for Proposals:** Proposals usually require a bond, which is returned if the proposal is approved or slashed if rejected (to discourage spam).
    *   **Approval Process:** Proposals are typically approved or rejected by the Council. There might be a delay period for public scrutiny.
    *   **Payouts:** Approved proposals result in funds being paid out from the treasury.
    *   **Burn Mechanism:** A portion of unspent treasury funds can be burned periodically to manage token supply.
    *   **Funding Sources:** The runtime needs to be configured to direct funds (e.g., from transaction fees) into the treasury account.

### d. `pallet-elections-phragmen` (or similar)

*   **Role:** Manages the election of council members for `pallet-collective`. It uses Phragmén's algorithm or similar methods to ensure proportional representation.
*   **Key Features & Configuration Considerations:**
    *   **Candidacy Submission:** PTCN holders can declare themselves as candidates for council.
    *   **Voting for Candidates:** PTCN holders vote for their preferred candidates.
    *   **Election Process:** Periodically, an election is run to determine the new set of council members based on votes.
    *   **Term Durations:** Defines how long council members serve.

## 7. Integration and Interaction

These pallets are designed to work together:
*   `pallet-democracy` can have its proposals vetoed or fast-tracked by `pallet-collective`.
*   `pallet-collective` (as the Council) can submit proposals to `pallet-democracy` for community vote or directly propose spending from `pallet-treasury`.
*   `pallet-treasury` proposals are typically managed and approved/rejected by `pallet-collective`.
*   `pallet-elections-phragmen` populates the members of `pallet-collective`.

The specific origins (e.g., who can submit certain proposals, who can approve treasury spends) will be carefully configured in the runtime to establish the desired governance balance for CritterChain.

## 8. Conceptual User Interface for On-Chain Governance

The CritterCraft UI Wallet will provide a dedicated "Governance" section to facilitate user interaction with the on-chain governance mechanisms. This interface will aim to make participation transparent and accessible.

### a. Public Referenda & Proposals (`#democracy-module`)
*   **Display Active Referenda (`#referenda-list`):**
    *   A list of all currently active public referenda.
    *   Each item will show: Referendum ID, title/short description, proposer, voting end time/block, current tally (Yes/No votes, turnout percentage).
    *   Buttons to "Vote YES" and "Vote NO" on each referendum.
    *   A link/button to view more detailed information about the referendum (e.g., full proposal text, on-chain call data).
*   **Submitting Proposals (`#submit-proposal-area`):**
    *   A button "[Submit New Proposal (Conceptual)]" will eventually allow eligible users to initiate new public referenda. This flow would involve specifying the proposed action (e.g., a runtime call) and potentially bonding PTCN.
*   **Voting Interface:**
    *   When voting, users might be presented with options for "Conviction Voting" (locking tokens for longer to increase vote weight).
*   **Status Feedback:** A general status area (`#governance-action-status` or module-specific) will provide feedback on proposal submissions and voting transactions.

### b. CritterCraft Council Information (`#council-module`)
*   **Display Council Members (`#council-members-list`):**
    *   A list of current council members, showing their AccountIds and identities (if available).
*   **Council Motions/Proposals (`#council-motions-list`):**
    *   A feed of recent motions proposed and voted upon by the council (e.g., motions to fast-track referenda, approve treasury spends).
*   **More Council Info (`#viewCouncilInfoButton`):**
    *   A conceptual button leading to more detailed information about the council's mandate, election process, and historical decisions.

### c. Ecosystem Treasury (`#treasury-module`)
*   **Treasury Balance (`#treasury-balance`):** Display the current total balance of the on-chain treasury.
*   **Spending Proposals (`#treasury-proposals-list`):**
    *   A list of active proposals to spend funds from the treasury.
    *   Each item will show: Proposal ID, proposer, amount requested, beneficiary, short description/purpose, and current status (e.g., "Under Council Consideration," "Approved, Pending Payout," "Paid").
    *   A conceptual "[Endorse Proposal]" button might allow users to signal support, though formal approval typically rests with the Council or via a referendum.
*   **Submitting Treasury Proposals (`#submitTreasuryProposalButton`):**
    *   A conceptual button for users (or council members) to initiate new treasury spending proposals, detailing the request, amount, and beneficiary.

This UI structure aims to provide a clear overview of governance activities and enable PTCN holders to participate effectively in the decision-making processes of CritterCraft. All interactions involving extrinsics (voting, proposing) will initially be simulated in the UI development phase.
