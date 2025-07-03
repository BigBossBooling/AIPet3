//! # NFT Management Traits
//!
//! Defines the core interface for creating, managing, and transferring
//! Non-Fungible Tokens (pets) within the CritterCraft ecosystem.

use super::{types::PetStats, Config};
use frame_support::dispatch::DispatchResult;

/// A unified trait for all core NFT management operations.
/// (I) - Consolidates `SharedNftManager` and `ExtendedNftManager` into one clear interface.
pub trait NftManagement<T: Config> {
    /// Get the owner of a pet NFT. ReturAns `None` if the pet does not exist.
    fn owner_of(pet_id: &T::PetId) -> Option<T::AccountId>;

    /// Transfer a pet NFT from one account to another.
    fn transfer(from: &T::AccountId, to: &T::AccountId, pet_id: &T::PetId) -> DispatchResult;

    /// Checks if a pet is "locked" by another pallet (e.g., listed on the
    /// marketplace or currently in a battle) and cannot be transferred.
    fn is_locked(pet_id: &T::PetId) -> bool;

    /// Get the current stats of a specific pet.
    fn pet_stats(pet_id: &T::PetId) -> Option<PetStats>;

    /// Mint a new pet NFT and assign it to an owner.
    /// This is the foundational function for creating all new pets.
    fn mint(owner: &T::AccountId, dna: [u8; 32], stats: PetStats) -> Result<T::PetId, DispatchResult>;
}
