//! # Breeding System Traits
//!
//! Defines the interfaces for the Echo-Synthesis (breeding) system, which
//! allows players to combine pets to create new offspring.

use super::{types::{PetDna, BoundedString}, Config, nft::NftManagement};
use frame_support::dispatch::DispatchResult;

/// A handler for all breeding-related operations.
/// This trait depends on `NftManagement` to mint the resulting offspring.
pub trait Breeding<T: Config + NftManagement<T>> {
    /// Get the core genetic information (DNA and species) for a given pet.
    fn pet_dna(pet_id: &T::PetId) -> Option<PetDna<BoundedString<T>>>;

    /// Checks if a pet is mature and not on a breeding cooldown.
    fn is_breedable(pet_id: &T::PetId) -> bool;

    /// Initiates the breeding process between two pets, consuming a catalyst item.
    /// On success, it calls `NftManagement::mint` to create the new pet.
    fn breed(
        owner: &T::AccountId,
        parent1_id: &T::PetId,
        parent2_id: &T::PetId,
        catalyst_id: &T::ItemId,
    ) -> Result<T::PetId, DispatchResult>;
}