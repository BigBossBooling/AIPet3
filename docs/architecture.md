# CritterCraftUniverse: Architectural Overview

## 1. Core Principles

* **Modularity:** System components are decoupled for independent development and scalability.
* **Decentralization:** Leveraging blockchain for ownership, governance, and data.
* **AI-Native:** AI is integrated at fundamental levels, not as an add-on.
* **Hybrid On-Chain/Off-Chain:** Critical state and logic on-chain, performance-intensive elements off-chain.

## 2. High-Level Architecture

CritterCraftUniverse is composed of several interconnected layers:

* **User Interface Layer (Frontend):** React-based web application for pet interaction and game display.
* **Application Backend Layer:** Python services managing game logic, pet state, and interaction with blockchain/DDS.
* **Decentralized Storage Layer (DDS):** For storing pet metadata, game assets, and user-generated content (leveraging DigiSocialBlock's DDS).
* **Blockchain Layer (L1):** EmPower1 Blockchain for core ledger, tokenomics, and NFT ownership.
* **Governance & Integrity Layer:** QRASL (Zoologist's Guild, Reputation, Moderation, Dispute Resolution).
* **AI Compute Layer:** EchoSphere AI-vCPU for advanced AI behaviors and cognitive processing.

## 3. Data Flow & Interconnections

* **User Interaction:** Frontend -> Application Backend -> Blockchain/DDS.
* **Pet State:** Pet data in backend, with critical metadata (e.g., ownership, core traits) on-chain as NFT attributes.
* **Game Assets:** Stored on DDS, referenced by CIDs in smart contracts.
* **AI Decisions:** AI-vCPU informs backend logic, influences pet behavior, and provides game insights.
* **Governance Decisions:** On-chain votes/verdicts from QRASL influence game rules, treasury, and user reputation.

## 4. Key Modules & Their Roles

* **`pet/`:** Core pet logic, stats, interactions (Python).
* **`pallets/`:** Substrate pallets for blockchain logic (Rust).
    * `pallet-battles`: Critter Tactics game logic.
    * `pallet-contracts`: Caregiver Contracts.
    * `pallet-economy`: $QRASL tokenomics.
    * `pallet-governance`: Zoologist's Guild DAO.
    * `pallet-reputation`: On-Chain Reputation System.
* **`frontend/`:** React application.
* **`pkg/dds/`:** Decentralized Data Storage components (Go).
* **`pkg/identity/`:** User identity and wallet management (Go).
* **`pkg/ledger/`:** Core blockchain ledger (Go).
* **`EchoSphere AI-vCPU` (Conceptual):** Provides specialized AI processing.
* **`Project Hyperion` (Conceptual):** Provides advanced security intelligence (e.g., for user account security).
* **`Prometheus Protocol` (Conceptual):** Provides AI prompting capabilities for LLM interactions.
* **`Privacy Protocol` (Conceptual):** Manages user consent and data privacy.
