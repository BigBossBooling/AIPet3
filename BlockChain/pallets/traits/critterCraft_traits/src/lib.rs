//! # CritterCraft Shared Traits
//!
//! This crate defines shared traits and types used across the CritterCraft ecosystem.
//! It provides a standardized interface for cross-pallet communication and integration.
//!
//! ## Overview
//!
//! The traits defined here enable:
//! - NFT management across pallets
//! - Breeding mechanics
//! - Item interactions
//! - Quest requirements
//! - Marketplace integration
//! - Governance participation
//!
//! ## Design Philosophy
//!
//! These traits follow the "Expanded KISS Principle" by providing clear, focused interfaces
//! that enable complex interactions while maintaining separation of concerns.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::Get,
    BoundedVec,
};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{prelude::*, vec::Vec};

// --- Type Definitions ---

/// Type for Pet IDs
pub type PetId = u32;

/// Type for Item IDs
pub type ItemId = u32;

/// Type for DNA hash (32 bytes)
pub type DnaHashType = [u8; 32];

/// Type for item category tags
pub type ItemCategoryTag = u8;

/// Type for personality trait strings
pub type TraitTypeString = BoundedVec<u8, ConstU32<64>>;

/// Type for species names
pub type SpeciesType = BoundedVec<u8, ConstU32<64>>;

// --- Trait Configuration Constants ---

/// Trait for shared configuration constants
pub trait TraitConfigConstants {
    /// Maximum length of a pet's species name
    type MaxSpeciesNameLen: Get<u32>;
    
    /// Maximum length of a pet's name
    type MaxPetNameLen: Get<u32>;
    
    /// Maximum length of a personality trait string
    type MaxTraitStringLen: Get<u32>;
    
    /// Maximum number of personality traits a pet can have
    type MaxPetPersonalityTraits: Get<u32>;
}

// --- Shared Data Structures ---

/// Simple genetic information for breeding
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SimpleGeneticInfo<DnaHash, Species> {
    /// The DNA hash of the pet
    pub dna_hash: DnaHash,
    /// The species of the pet
    pub species: Species,
}

/// Pet attribute type for minigames and jobs
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum AttributeType {
    Strength,
    Agility,
    Intelligence,
    Vitality,
    Elemental,
}

/// Pet statistics for gameplay
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PetStats {
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
    pub vitality: u8,
    pub elemental_affinity: u8,
    pub level: u16,
    pub experience: u32,
}

/// Enhanced pet information for advanced gameplay
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct EnhancedPetInfo<AccountId, BlockNumber> {
    pub id: PetId,
    pub owner: AccountId,
    pub stats: PetStats,
    pub mood: u8,
    pub last_interaction: BlockNumber,
    pub is_locked: bool,
}

// --- Core NFT Management Traits ---

/// Core NFT management trait for cross-pallet integration
pub trait SharedNftManager<AccountId, TokenId> {
    /// Get the owner of an NFT
    fn owner_of(token_id: &TokenId) -> Option<AccountId>;
    
    /// Check if an NFT is transferable
    fn is_transferable(token_id: &TokenId) -> bool;
    
    /// Lock an NFT to prevent transfers
    fn lock_nft(owner: &AccountId, token_id: &TokenId) -> DispatchResult;
    
    /// Unlock a previously locked NFT
    fn unlock_nft(owner: &AccountId, token_id: &TokenId) -> DispatchResult;
    
    /// Transfer an NFT between accounts
    fn transfer_nft(from: &AccountId, to: &AccountId, token_id: &TokenId) -> DispatchResult;
}

/// Extended NFT management for advanced operations
pub trait ExtendedNftManager<AccountId, TokenId>: SharedNftManager<AccountId, TokenId> {
    /// Get detailed information about an NFT
    fn get_nft_details(token_id: &TokenId) -> Option<Vec<u8>>;
    
    /// Check if an account owns a specific NFT
    fn is_owner(account: &AccountId, token_id: &TokenId) -> bool;
    
    /// Get all NFTs owned by an account
    fn get_owned_nfts(account: &AccountId) -> Vec<TokenId>;
    
    /// Burn an NFT (permanent destruction)
    fn burn_nft(owner: &AccountId, token_id: &TokenId) -> DispatchResult;
}

// --- Breeding Traits ---

/// Handler for NFT breeding operations
pub trait NftBreedingHandler<AccountId, TokenId, DnaHash, Species> {
    /// Get basic genetic information for a pet
    fn get_pet_simple_genetics(token_id: &TokenId) -> Option<SimpleGeneticInfo<DnaHash, Species>>;
    
    /// Mint a new pet from breeding
    fn mint_pet_from_breeding(
        owner: &AccountId,
        species: Species,
        dna_hash: DnaHash,
        parent1_id: TokenId,
        parent2_id: TokenId,
        initial_name: BoundedVec<u8, ConstU32<64>>,
    ) -> Result<TokenId, DispatchResult>;
}

/// Advanced breeding mechanics
pub trait AdvancedBreedingMechanics<AccountId, TokenId> {
    /// Check if a pet is eligible for breeding
    fn is_breedable(token_id: &TokenId) -> bool;
    
    /// Get the breeding cooldown period
    fn get_breeding_cooldown(token_id: &TokenId) -> Option<u32>;
    
    /// Set the last breeding time
    fn set_last_breeding_time(token_id: &TokenId, block_number: u32) -> DispatchResult;
    
    /// Calculate breeding success probability
    fn calculate_breeding_success(parent1: &TokenId, parent2: &TokenId) -> u8;
    
    /// Get genetic compatibility between two pets
    fn get_genetic_compatibility(parent1: &TokenId, parent2: &TokenId) -> u8;
}

// --- Item Interaction Traits ---

/// Basic care item consumer for pet interactions
pub trait BasicCareItemConsumer<AccountId, ItemId, CategoryTag, ResultType> {
    /// Consume an item of a specific category
    fn consume_item_of_category(
        owner: &AccountId,
        item_id: &ItemId,
        category: CategoryTag,
    ) -> Result<(), ResultType>;
    
    /// Get the food category tag
    fn food_category_tag() -> CategoryTag;
    
    /// Get the toy category tag
    fn toy_category_tag() -> CategoryTag;
}

/// NFT manager for item interactions
pub trait NftManagerForItems<AccountId, TokenId, TraitString, BlockNumber> {
    /// Get the owner of a pet for item use
    fn get_pet_owner_for_item_use(token_id: &TokenId) -> Option<AccountId>;
    
    /// Apply fixed XP to a pet
    fn apply_fixed_xp_to_pet(
        caller: &AccountId,
        token_id: &TokenId,
        amount: u32,
    ) -> DispatchResult;
    
    /// Modify a pet's mood
    fn apply_mood_modification_to_pet(
        caller: &AccountId,
        token_id: &TokenId,
        amount: i16,
    ) -> DispatchResult;
    
    /// Apply a personality trait to a pet
    fn apply_personality_trait_to_pet(
        caller: &AccountId,
        token_id: &TokenId,
        trait_to_grant: TraitString,
    ) -> DispatchResult;
    
    /// Apply a breeding assist effect
    fn apply_breeding_assist_effect(
        caller: &AccountId,
        token_id: &TokenId,
        effect_type_id: u8,
        value: u32,
    ) -> DispatchResult;
}

// --- Quest Traits ---

/// Quest requirement checker for NFTs
pub trait QuestNftRequirementChecker<AccountId, TokenId, Species> {
    /// Get the owner of a pet for quest verification
    fn get_pet_owner_for_quest(token_id: &TokenId) -> Option<AccountId>;
    
    /// Get the level of a pet for quest requirements
    fn get_pet_level_for_quest(token_id: &TokenId) -> Option<u32>;
    
    /// Get the species of a pet for quest requirements
    fn get_pet_species_for_quest(token_id: &TokenId) -> Option<Species>;
}

/// Advanced quest interaction for pets
pub trait AdvancedQuestInteraction<AccountId, TokenId, QuestId> {
    /// Check if a pet meets all requirements for a quest
    fn meets_quest_requirements(token_id: &TokenId, quest_id: &QuestId) -> bool;
    
    /// Start a quest with a pet
    fn start_quest(account: &AccountId, token_id: &TokenId, quest_id: &QuestId) -> DispatchResult;
    
    /// Complete a quest with a pet
    fn complete_quest(account: &AccountId, token_id: &TokenId, quest_id: &QuestId) -> DispatchResult;
    
    /// Get active quests for a pet
    fn get_active_quests(token_id: &TokenId) -> Vec<QuestId>;
    
    /// Get completed quests for a pet
    fn get_completed_quests(token_id: &TokenId) -> Vec<QuestId>;
}

// --- Marketplace Traits ---

/// Marketplace integration for NFTs
pub trait MarketplaceIntegration<AccountId, TokenId, Balance> {
    /// List an NFT for sale
    fn list_for_sale(owner: &AccountId, token_id: &TokenId, price: Balance) -> DispatchResult;
    
    /// Cancel a listing
    fn cancel_listing(owner: &AccountId, token_id: &TokenId) -> DispatchResult;
    
    /// Buy a listed NFT
    fn buy_listed_nft(buyer: &AccountId, token_id: &TokenId) -> DispatchResult;
    
    /// Get the current price of a listed NFT
    fn get_listing_price(token_id: &TokenId) -> Option<Balance>;
    
    /// Check if an NFT is listed for sale
    fn is_listed(token_id: &TokenId) -> bool;
    
    /// Get all NFTs listed by an account
    fn get_account_listings(account: &AccountId) -> Vec<TokenId>;
}

// --- Governance Traits ---

/// Governance participation for NFT holders
pub trait GovernanceParticipation<AccountId, TokenId, ProposalId, VoteWeight> {
    /// Get the voting weight for an NFT
    fn get_nft_voting_weight(token_id: &TokenId) -> VoteWeight;
    
    /// Vote on a proposal with an NFT
    fn vote_with_nft(
        owner: &AccountId,
        token_id: &TokenId,
        proposal_id: &ProposalId,
        approve: bool,
    ) -> DispatchResult;
    
    /// Get the total voting weight of an account's NFTs
    fn get_account_total_voting_weight(account: &AccountId) -> VoteWeight;
    
    /// Check if an NFT has voted on a proposal
    fn has_nft_voted(token_id: &TokenId, proposal_id: &ProposalId) -> bool;
    
    /// Get all proposals an NFT has voted on
    fn get_nft_voting_history(token_id: &TokenId) -> Vec<ProposalId>;
}

// --- Advanced Pet Management Traits ---

/// Advanced pet management for gameplay
pub trait AdvancedPetManagement<AccountId, BlockNumber> {
    /// Get enhanced pet information
    fn get_enhanced_pet_info(pet_id: &PetId) -> Option<EnhancedPetInfo<AccountId, BlockNumber>>;
    
    /// Update pet stats
    fn update_pet_stats(pet_id: &PetId, stats: PetStats) -> DispatchResult;
    
    /// Add experience to a pet
    fn add_experience(pet_id: &PetId, experience: u32) -> DispatchResult;
    
    /// Get pet level
    fn get_pet_level(pet_id: &PetId) -> Option<u16>;
    
    /// Get pet attributes
    fn get_pet_attributes(pet_id: &PetId) -> Option<Vec<(AttributeType, u8)>>;
    
    /// Update pet state
    fn update_pet_state(pet_id: &PetId) -> DispatchResult;
    
    /// Check if a pet is neglected
    fn is_pet_neglected(pet_id: &PetId) -> bool;
    
    /// Get pet evolution eligibility
    fn is_eligible_for_evolution(pet_id: &PetId) -> bool;
    
    /// Evolve a pet
    fn evolve_pet(owner: &AccountId, pet_id: &PetId) -> DispatchResult;
}

// --- Pet Status Management Traits ---

/// Pet status management for care actions
pub trait PetStatusManager<TokenId> {
    /// Feed a pet
    fn feed_pet(pet_id: &TokenId) -> DispatchResult;
    
    /// Rest a pet
    fn rest_pet(pet_id: &TokenId) -> DispatchResult;
    
    /// Play with a pet
    fn play_with_pet(pet_id: &TokenId) -> DispatchResult;
    
    /// Groom a pet
    fn groom_pet(pet_id: &TokenId) -> DispatchResult;
    
    /// Socialize a pet with another pet
    fn socialize_pet(pet_id: &TokenId, target_pet_id: &TokenId) -> DispatchResult;
}

// --- Battle System Traits ---

/// Battle system integration for pets
pub trait BattleSystemIntegration<AccountId, TokenId, BattleId> {
    /// Check if a pet is eligible for battle
    fn is_battle_eligible(pet_id: &TokenId) -> bool;
    
    /// Get pet battle stats
    fn get_battle_stats(pet_id: &TokenId) -> Option<PetStats>;
    
    /// Start a battle
    fn start_battle(
        initiator: &AccountId,
        pet_id: &TokenId,
        opponent_pet_id: &TokenId,
    ) -> Result<BattleId, DispatchError>;
    
    /// Execute a battle move
    fn execute_battle_move(
        account: &AccountId,
        battle_id: &BattleId,
        move_id: u8,
    ) -> DispatchResult;
    
    /// Get battle outcome
    fn get_battle_outcome(battle_id: &BattleId) -> Option<(TokenId, TokenId, bool)>;
    
    /// Get pet battle history
    fn get_pet_battle_history(pet_id: &TokenId) -> Vec<BattleId>;
}

// --- Daycare System Traits ---

/// Daycare system integration for pets
pub trait DaycareSystemIntegration<AccountId, TokenId, DaycareId, ListingId> {
    /// Create a daycare
    fn create_daycare(
        owner: &AccountId,
        name: Vec<u8>,
        description: Vec<u8>,
        fee_per_block: u32,
    ) -> Result<DaycareId, DispatchError>;
    
    /// Create a listing
    fn create_listing(
        owner: &AccountId,
        daycare_id: &DaycareId,
        pet_id: &TokenId,
        duration: u32,
    ) -> Result<ListingId, DispatchError>;
    
    /// Accept a listing
    fn accept_listing(caregiver: &AccountId, listing_id: &ListingId) -> DispatchResult;
    
    /// Perform a care action
    fn perform_care_action(
        caregiver: &AccountId,
        listing_id: &ListingId,
        action: u8,
        target_pet_id: Option<TokenId>,
    ) -> DispatchResult;
    
    /// Complete a listing
    fn complete_listing(owner: &AccountId, listing_id: &ListingId) -> DispatchResult;
    
    /// Get active listings for a pet
    fn get_pet_active_listings(pet_id: &TokenId) -> Vec<ListingId>;
}

// --- Minigame System Traits ---

/// Minigame system integration for pets
pub trait MinigameSystemIntegration<AccountId, TokenId, GameId> {
    /// Start a minigame
    fn start_game(
        player: &AccountId,
        pet_id: &TokenId,
        game_type: u8,
        difficulty: u8,
    ) -> Result<GameId, DispatchError>;
    
    /// Submit a score for a minigame
    fn submit_score(player: &AccountId, game_id: &GameId, score: u32) -> DispatchResult;
    
    /// Get pet minigame history
    fn get_pet_minigame_history(pet_id: &TokenId) -> Vec<(GameId, u32)>;
    
    /// Get pet high scores
    fn get_pet_high_scores(pet_id: &TokenId) -> Vec<(u8, u32)>;
    
    /// Check if a pet is eligible for a specific minigame
    fn is_eligible_for_minigame(pet_id: &TokenId, game_type: u8) -> bool;
}

// --- Job System Traits ---

/// Job system integration for pets
pub trait JobSystemIntegration<AccountId, TokenId, JobId> {
    /// Start a job
    fn start_job(
        owner: &AccountId,
        pet_id: &TokenId,
        job_type: u8,
        duration: u32,
    ) -> Result<JobId, DispatchError>;
    
    /// Complete a job
    fn complete_job(owner: &AccountId, job_id: &JobId) -> DispatchResult;
    
    /// Get pet job history
    fn get_pet_job_history(pet_id: &TokenId) -> Vec<JobId>;
    
    /// Check if a pet is eligible for a specific job
    fn is_eligible_for_job(pet_id: &TokenId, job_type: u8) -> bool;
    
    /// Get active jobs for a pet
    fn get_pet_active_jobs(pet_id: &TokenId) -> Vec<JobId>;
}

// --- Profile System Traits ---

/// Profile system integration for accounts
pub trait ProfileSystemIntegration<AccountId, ProfileId, BadgeId> {
    /// Create a profile
    fn create_profile(
        account: &AccountId,
        username: Vec<u8>,
        bio: Vec<u8>,
        avatar_uri: Vec<u8>,
    ) -> Result<ProfileId, DispatchError>;
    
    /// Update a profile
    fn update_profile(
        account: &AccountId,
        username: Option<Vec<u8>>,
        bio: Option<Vec<u8>>,
        avatar_uri: Option<Vec<u8>>,
    ) -> DispatchResult;
    
    /// Add a friend
    fn add_friend(account: &AccountId, friend: &AccountId) -> DispatchResult;
    
    /// Remove a friend
    fn remove_friend(account: &AccountId, friend: &AccountId) -> DispatchResult;
    
    /// Award a badge
    fn award_badge(account: &AccountId, badge_id: &BadgeId) -> DispatchResult;
    
    /// Equip a badge
    fn equip_badge(account: &AccountId, badge_id: &BadgeId) -> DispatchResult;
    
    /// Unequip a badge
    fn unequip_badge(account: &AccountId, badge_id: &BadgeId) -> DispatchResult;
    
    /// Get profile information
    fn get_profile_info(account: &AccountId) -> Option<Vec<u8>>;
}

// --- Treasury System Traits ---

/// Treasury system integration
pub trait TreasurySystemIntegration<AccountId, ProposalId, Balance> {
    /// Propose a treasury spend
    fn propose_spend(
        proposer: &AccountId,
        amount: Balance,
        beneficiary: &AccountId,
        description: Vec<u8>,
    ) -> Result<ProposalId, DispatchError>;
    
    /// Approve a treasury spend
    fn approve_spend(proposal_id: &ProposalId) -> DispatchResult;
    
    /// Reject a treasury spend
    fn reject_spend(proposal_id: &ProposalId) -> DispatchResult;
    
    /// Execute a treasury spend
    fn execute_spend(proposal_id: &ProposalId) -> DispatchResult;
    
    /// Get treasury balance
    fn get_treasury_balance() -> Balance;
    
    /// Get proposal details
    fn get_proposal_details(proposal_id: &ProposalId) -> Option<Vec<u8>>;
    
    /// Get all active proposals
    fn get_active_proposals() -> Vec<ProposalId>;
}

// --- Node Rewards System Traits ---

/// Node rewards system integration
pub trait NodeRewardsSystemIntegration<AccountId, NodeId, Balance> {
    /// Register a node
    fn register_node(operator: &AccountId) -> Result<NodeId, DispatchError>;
    
    /// Unregister a node
    fn unregister_node(operator: &AccountId, node_id: &NodeId) -> DispatchResult;
    
    /// Report node metrics
    fn report_metrics(
        operator: &AccountId,
        node_id: &NodeId,
        uptime: u32,
        blocks_produced: u32,
        response_time: u32,
    ) -> DispatchResult;
    
    /// Report a validator as offline
    fn report_offline(reporter: &AccountId, validator: &AccountId) -> DispatchResult;
    
    /// Claim rewards
    fn claim_rewards(claimer: &AccountId) -> Result<Balance, DispatchError>;
    
    /// Get pending rewards
    fn get_pending_rewards(account: &AccountId) -> Balance;
    
    /// Get node information
    fn get_node_info(node_id: &NodeId) -> Option<Vec<u8>>;
}

// --- Governance System Traits ---

/// Governance system integration
pub trait GovernanceSystemIntegration<AccountId, ProposalId, VoteWeight> {
    /// Create a proposal
    fn create_proposal(
        proposer: &AccountId,
        proposal_hash: [u8; 32],
        voting_period: u32,
        description: Vec<u8>,
    ) -> Result<ProposalId, DispatchError>;
    
    /// Second a proposal
    fn second_proposal(seconder: &AccountId, proposal_id: &ProposalId) -> DispatchResult;
    
    /// Vote on a proposal
    fn vote_on_proposal(
        voter: &AccountId,
        proposal_id: &ProposalId,
        approve: bool,
        vote_amount: VoteWeight,
    ) -> DispatchResult;
    
    /// Close a vote
    fn close_vote(proposal_id: &ProposalId) -> DispatchResult;
    
    /// Execute a proposal
    fn execute_proposal(proposal_id: &ProposalId) -> DispatchResult;
    
    /// Delegate voting power
    fn delegate_voting(delegator: &AccountId, delegate: &AccountId) -> DispatchResult;
    
    /// Undelegate voting power
    fn undelegate_voting(delegator: &AccountId) -> DispatchResult;
    
    /// Get proposal details
    fn get_proposal_details(proposal_id: &ProposalId) -> Option<Vec<u8>>;
    
    /// Get all active proposals
    fn get_active_proposals() -> Vec<ProposalId>;
}