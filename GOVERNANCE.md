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

A new pallet, `pallet-governance` (or similar), will manage the proposal lifecycle.

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

*   **Stage 4 (Current Focus - Hooks):** Conceptual outline and UI placeholder. No on-chain voting pallet yet.
*   **Future Stages:**
    *   Implementation of a basic `pallet-governance`.
    *   Introduction of simple proposal types (e.g., text-based polls).
    *   Gradual expansion to more complex on-chain parameter changes and treasury management.

This governance model aims to empower the CritterCraft community and ensure the platform's sustainable and decentralized evolution.
