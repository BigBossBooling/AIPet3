// --- File: traits/src/items.rs ---
use super::Config;
use frame_support::dispatch::DispatchResult;

/// A trait for managing the consumption of items.
pub trait ItemConsumer<T: Config> {
    /// Consumes a single item from an account's inventory.
    fn consume(owner: &T::AccountId, item_id: &T::ItemId) -> DispatchResult;
}

// --- File: traits/src/battle.rs ---
use super::{types::PetStats, Config};

/// A trait for providing battle-related information about a pet.
pub trait BattleReady<T: Config> {
    /// Checks if a pet has enough stamina and is not in a non-battle state.
    fn can_battle(pet_id: &T::PetId) -> bool;
    /// Retrieves the stats relevant for a battle encounter.
    fn battle_stats(pet_id: &T::PetId) -> Option<PetStats>;
}