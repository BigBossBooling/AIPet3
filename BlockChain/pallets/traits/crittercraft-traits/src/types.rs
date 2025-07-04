//! # Shared Data Types
//!
//! This module defines the core, shared data structures for the CritterCraft
//! ecosystem, promoting type safety and clarity over primitive types.

use super::Config;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, BoundedVec};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// A unique identifier for a pet's genetic code (e.g., a 32-byte hash).
pub type DnaHash = [u8; 32];

/// A bounded vector for storing names and other short strings.
pub type BoundedString<T> = BoundedVec<u8, <T as Config>::MaxStringLength>;

/// Represents the core genetic makeup of a pet, used in breeding.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PetDna<Species> {
    pub dna_hash: DnaHash,
    pub species: Species,
}

/// Defines the primary statistical attributes of a pet.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum StatAttribute {
    Strength,
    Agility,
    Intelligence,
    Charisma,
    Stamina,
}

/// A snapshot of a pet's current gameplay statistics.
#[derive(Clone, Encode, Decode, PartialEq, Eq, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PetStats {
    pub level: u16,
    pub experience: u32,
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
    pub charisma: u8,
    pub stamina: u8,
}