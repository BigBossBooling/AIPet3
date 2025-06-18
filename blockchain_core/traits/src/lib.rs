#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use frame_support::dispatch::DispatchResult; // Common dispatch result type

// --- Conceptual Type Aliases ---
// These are placeholders. In a real setup, pallets would define their own concrete types
// or use generics that are then resolved in the runtime. For a traits crate, it's often
// better to use generics in trait definitions (e.g., <AccountId, PetIdType, ...>)
// or associated types if the trait is defining something that varies per implementation.
// For this conceptual step, we use simple type aliases to represent common identifiers.

pub type PetId = u32;
pub type ItemId = u32;
pub type DnaHashType = [u8; 16];
pub type SpeciesType = Vec<u8>;
pub type TraitTypeString = Vec<u8>;
pub type ItemCategoryTag = u8; // Placeholder: e.g., 0 for Food, 1 for Toy. Pallet-items would map this.
// pub type PetAttributeIdentifier = u8; // Placeholder: To identify pet attributes for boosts.

// --- Trait Definitions ---

/// For general NFT operations: ownership, locking, transfers.
/// Implemented by `pallet-critter-nfts`.
/// Used by `pallet-marketplace`, `pallet-battles`, `pallet-daycare`.
pub trait NftManager<AccountId, LocalPetId> {
    fn owner_of(pet_id: &LocalPetId) -> Option<AccountId>;
    fn is_transferable(pet_id: &LocalPetId) -> bool; // Checks if not locked
    fn lock_nft(owner: &AccountId, pet_id: &LocalPetId) -> DispatchResult;
    fn unlock_nft(owner: &AccountId, pet_id: &LocalPetId) -> DispatchResult;
    /// Basic transfer, assumes locks are handled by caller if necessary.
    fn transfer_nft(from: &AccountId, to: &AccountId, pet_id: &LocalPetId) -> DispatchResult;
}

/// For breeding-specific interactions with Pet NFTs.
/// Implemented by `pallet-critter-nfts`.
/// Used by `pallet-breeding`.
#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct SimpleGeneticInfo<LocalDnaHashType, LocalSpeciesType> {
    pub dna_hash: LocalDnaHashType,
    pub species: LocalSpeciesType,
    // pub level: u32, // Example: if breeding eligibility depends on level (part of PetNft struct)
    // pub is_ready_for_breeding: bool, // Example: if cooldown is managed in critter-nfts
}
pub trait NftBreedingHandler<AccountId, LocalPetId, LocalDnaHashType, LocalSpeciesType> {
    fn get_pet_simple_genetics(pet_id: &LocalPetId) -> Option<SimpleGeneticInfo<LocalDnaHashType, LocalSpeciesType>>;
    /// Mints a new pet based on breeding outcome, including setting parents.
    fn mint_pet_from_breeding(
        owner: &AccountId,
        species: LocalSpeciesType,
        dna_hash: LocalDnaHashType,
        parent1_id: LocalPetId,
        parent2_id: LocalPetId,
        initial_name: Vec<u8>,
    ) -> Result<LocalPetId, DispatchResult>; // Return PetId of new pet or error
}

/// For `pallet-items` to apply effects to Pet NFTs.
/// Implemented by `pallet-critter-nfts`.
/// Used by `pallet-items`.
pub trait NftManagerForItems<AccountId, LocalPetId, LocalTraitTypeString, BlockNumberType> {
    fn get_pet_owner_for_item_use(pet_id: &LocalPetId) -> Option<AccountId>;

    fn apply_fixed_xp_to_pet(
        caller: &AccountId,
        pet_id: &LocalPetId,
        amount: u32,
    ) -> DispatchResult;

    fn apply_mood_modification_to_pet(
        caller: &AccountId,
        pet_id: &LocalPetId,
        amount: i16,
    ) -> DispatchResult;

    fn apply_personality_trait_to_pet(
        caller: &AccountId,
        pet_id: &LocalPetId,
        trait_to_grant: LocalTraitTypeString,
    ) -> DispatchResult;

    fn apply_breeding_assist_effect(
        caller: &AccountId,
        pet_id: &LocalPetId,
        effect_type_id: u8,
        value: u32,
    ) -> DispatchResult;

    // Future methods for equipment or complex buffs:
    // fn apply_timed_attribute_boost(pet_id: &LocalPetId, attribute: PetAttributeIdentifier, amount: i16, duration: BlockNumberType) -> DispatchResult;
    // fn equip_item_on_pet(pet_id: &LocalPetId, item_id: LocalItemId, slot_id: u8) -> DispatchResult;
    // fn unequip_item_from_pet(pet_id: &LocalPetId, slot_id: u8) -> DispatchResult;
}

/// For `pallet-critter-nfts` to request consumption of basic care items.
/// Implemented by `pallet-items`.
/// Used by `pallet-critter-nfts`.
pub trait BasicCareItemConsumer<AccountId, LocalItemId> {
    /// Checks if item of category exists for user and consumes it.
    fn consume_item_of_category(
        user: &AccountId,
        item_id: &LocalItemId,
        expected_category_tag: ItemCategoryTag,
    ) -> DispatchResult;
}

/// For `pallet-quests` to check Pet NFT requirements.
/// Implemented by `pallet-critter-nfts`.
/// Used by `pallet-quests`.
pub trait QuestNftRequirementChecker<AccountId, LocalPetId> {
    fn get_pet_owner_for_quest(pet_id: &LocalPetId) -> Option<AccountId>;
    fn get_pet_level_for_quest(pet_id: &LocalPetId) -> Option<u32>;
    // fn get_pet_species_for_quest(pet_id: &LocalPetId) -> Option<LocalSpeciesType>; // Deferred for MVP quests
}

/// For `pallet-quests` to check and consume item requirements.
/// Implemented by `pallet-items`.
/// Used by `pallet-quests`.
pub trait QuestItemRequirementChecker<AccountId, LocalItemId> {
    fn check_and_consume_quest_item(
        user: &AccountId,
        item_id: &LocalItemId,
        quantity: u32,
        consume: bool,
    ) -> DispatchResult;
}

/// For `pallet-quests` to check user profile requirements.
/// Implemented by `pallet-user-profile`.
/// Used by `pallet-quests`.
pub trait QuestUserProfileRequirementChecker<AccountId> {
    fn get_battles_won_for_quest(user: &AccountId) -> Option<u32>;
    // fn get_trade_reputation_for_quest(user: &AccountId) -> Option<i32>; // Deferred for MVP quests
}

// Note:
// - AccountId, BlockNumberType, etc., would typically be generic (<T: Config> where T::AccountId)
//   or associated types within these traits when used in a full Substrate setup.
//   The consuming pallet's Config trait would then specify the concrete types.
// - `DispatchResult` is from `frame_support::dispatch`.
```
