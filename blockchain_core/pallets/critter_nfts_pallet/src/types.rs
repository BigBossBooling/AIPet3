//! # Critter NFT Types
//!
//! This module defines common type aliases, enums, and structs
//! used across the `pallet-critter-nfts` and by its interacting traits.
//! It centralizes type definitions for pet attributes, elemental affinities,
//! and data structures, enhancing clarity and maintainability for the pallet.
//!
//! Meticulously crafted to align with The Architect's vision for
//! modularity, scalability, and robust operation of the CritterCraft digital ecosystem.

#![cfg_attr(not(feature = "std"), no_std)] // No standard library for Wasm compilation

use frame_support::{
    pallet_prelude::*, // Provides common types and macros for pallets
    BoundedVec,        // For bounded collections, crucial for security
};
use scale_info::TypeInfo; // For `TypeInfo` derive macro
use sp_runtime::traits::AtLeast32BitUnsigned; // For PetId to ensure it's a valid numerical type
use sp_std::vec::Vec; // For `Vec` where `BoundedVec` not strictly necessary or for conceptual types

// --- Enum Definitions ---
// ElementType: Defines the elemental affinities of Critters.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default, Copy)]
#[scale_info(skip_type_params(T))] // If T is used in a generic struct, apply here. Not needed for this enum.
pub enum ElementType {
    #[default]
    Neutral,
    Fire,
    Water,
    Earth,
    Air,
    Tech,
    Nature,
    Mystic,
}

// --- Struct Definitions ---
// PetAttributes: Represents the type for immutable charter attributes.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PetAttributes {
    pub base_strength: u8, // BaseStat is not a trait/type in Substrate, use u8 directly.
    pub base_agility: u8,
    pub base_intelligence: u8,
    pub base_vitality: u8,
    pub primary_elemental_affinity: ElementType,
}

// PetDevelopment: Represents the type for mutable nurture attributes.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))] // If using BoundedVec/BlockNumberFor, need T.
pub struct PetDevelopment<T: frame_system::Config + crate::pallet::Config> { // Add T: Config bound
    pub level: u32, // Level is not a trait/type, use u32.
    pub experience_points: u32, // ExperiencePoints is not a trait/type, use u32.
    pub mood_indicator: u8, // MoodIndicator is not a trait/type, use u8.
    pub last_fed_block: BlockNumberFor<T>, // From frame_system
    pub last_played_block: BlockNumberFor<T>, // From frame_system
    pub personality_traits: BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits>, // From crittercraft-traits and pallet Config
    pub last_state_update_block: BlockNumberFor<T>, // From frame_system
}

// --- Extrinsic Input/Output Structs (for internal helpers or RPCs) ---
/// Represents the type for pet NFT metadata update.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))] // If using BoundedVec, needs T.
pub struct PetMetadataUpdate<T: frame_system::Config + crate::pallet::Config> { // Add T: Config bound
    pub name: Option<BoundedVec<u8, T::MaxPetNameLen>>, // Use BoundedVec
    pub personality_traits: Option<BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits>>, // Use BoundedVec
}

/// Represents the type for pet NFT transfer (for event or internal use).
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))] // Needs T for AccountId
pub struct PetTransfer<T: frame_system::Config> { // Add T: Config bound
    pub from: T::AccountId, // Use AccountId
    pub to: T::AccountId,   // Use AccountId
    pub pet_id: PetId,
}

/// Represents the type for pet NFT interaction (for event or internal use).
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))] // Needs T for BlockNumberFor
pub struct PetInteraction<T: frame_system::Config> { // Add T: Config bound
    pub pet_id: PetId,
    pub item_id: ItemId,
    pub category: ItemCategoryTag,
    pub timestamp: BlockNumberFor<T>, // Using BlockNumberFor for consistent time tracking
}

/// Represents the type for pet NFT neglect check (for event or internal use).
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))] // Needs T for BlockNumberFor
pub struct PetNeglectCheck<T: frame_system::Config> { // Add T: Config bound
    pub pet_id: PetId,
    pub timestamp: BlockNumberFor<T>, // Use BlockNumberFor
    pub mood_penalty: u8, // MoodIndicator is not a trait/type, use u8.
}

/// Represents the type for pet NFT claim (for event or internal use).
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))] // Needs T for AccountId, Balance
pub struct PetClaim<T: frame_system::Config + crate::pallet::Config> { // Add T: Config bound
    pub account: T::AccountId,
    pub amount: <T as crate::pallet::Config>::BalanceOf, // Use BalanceOf from pallet Config
    pub timestamp: BlockNumberFor<T>, // Use BlockNumberFor
}