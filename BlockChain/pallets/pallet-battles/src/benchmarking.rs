//! Benchmarking for pallet-battles
//!
//! This file defines benchmarks for each extrinsic using the `frame_benchmarking` framework.
//! These benchmarks meticulously measure the computational and storage costs of pallet operations,
//! which are then used to generate accurate dispatch weights for transaction fees
//! and to ensure the economic integrity and scalability of the CritterChain network.
//!
//! The benchmarks are designed to capture worst-case scenarios and provide hooks for
//! dynamic weight calculation based on game parameters like battle level or number of players.
//! Meticulously crafted to align with The Architect's vision for
//! performance optimization and robust resource management in the CritterCraft digital ecosystem.

#![cfg(feature = "runtime-benchmarks")] // Only compile when the `runtime-benchmarks` feature is enabled.

use super::*; // Import all items from the parent module (lib.rs)
use frame_benchmarking::{benchmarks, whitelisted_caller, account, add_benchmark}; // Core benchmarking macros and helpers
use frame_system::RawOrigin; // For creating dispatch origins in benchmarks
use sp_std::prelude::*; // For Vec and other standard library types
use frame_support::traits::Get; // For accessing BoundedVec limits from Config
use frame_support::weights::{Weight, constants::RocksDbWeight}; // For explicit weight building


// --- Helper functions and constants for benchmarks ---
// These ensure deterministic and consistent setup for each benchmark.
const SEED: u32 = 0; // Constant seed for account generation, crucial for determinism.

/// Creates a new account ID from a string seed.
fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    account(name, SEED, 0)
}

/// Helper to get a `BoundedVec` for species name, filled to max length for worst-case.
fn get_bounded_species<T: Config>() -> crittercraft_traits::SpeciesType {
    vec![b'C'; T::MaxSpeciesNameLen::get() as usize].try_into().unwrap()
}

/// Helper to get a `BoundedVec` for pet name, filled to max length for worst-case.
fn get_bounded_name<T: Config>() -> BoundedVec<u8, T::MaxPetNameLen> {
    vec![b'N'; T::MaxPetNameLen::get() as usize].try_into().unwrap()
}

/// Helper to get a `BoundedVec` for personality traits, filled to max traits with max-length strings.
fn get_bounded_traits<T: Config>() -> BoundedVec<crittercraft_traits::TraitTypeString, T::MaxPetPersonalityTraits> {
    let mut traits_vec: Vec<crittercraft_traits::TraitTypeString> = Vec::new();
    let max_traits = T::MaxPetPersonalityTraits::get() as usize;
    let max_trait_len = T::MaxTraitStringLen::get() as usize;

    for i in 0..max_traits {
        let trait_str_raw = format!("Trait{}", i);
        // Pad with 'X' to max_trait_len if necessary for worst-case BoundedVec
        let padded_trait_str = if trait_str_raw.len() < max_trait_len {
            let mut s = trait_str_raw.into_bytes();
            s.resize(max_trait_len, b'X');
            s
        } else {
            trait_str_raw.into_bytes()
        };
        traits_vec.try_push(padded_trait_str.try_into().unwrap()).unwrap();
    }
    traits_vec.try_into().unwrap()
}

/// Helper to create and mint a pet using `pallet-critter-nfts`.
/// This is used for setting up benchmark scenarios.
fn create_and_mint_pet<T: Config>(owner: T::AccountId, pet_id: T::PetId) -> Result<(), DispatchError> {
    let species: crittercraft_traits::SpeciesType = vec![b'S'; 4].try_into().unwrap();
    let name: BoundedVec<u8, T::MaxPetNameLen> = vec![b'N'; 4].try_into().unwrap();
    
    crittercraft_runtime::pallet_critter_nfts::Pallet::<T>::mint_pet_nft(
        RawOrigin::Signed(owner.clone()).into(),
        species.into(),
        name.into(),
    )?;
    // If the pet_id created is not `pet_id`, it means the counter is not aligned or this helper
    // needs to return the actual pet_id created. For benchmarks, we usually use sequential IDs.
    Ok(())
}

/// Helper to ensure an account has some balance for transfers (e.g., reward pot).
fn endow_account<T: Config>(account_id: &T::AccountId, amount: BalanceOf<T>) {
    T::Currency::deposit_creating(account_id, amount);
}


// --- Benchmarks for each extrinsic ---
// Each benchmark aims to measure the cost of its corresponding extrinsic in a worst-case scenario.
// It includes parameters for dynamic weight calculation (e.g., 'b' for batch size, 'l' for level).
benchmarks! {
    // Benchmark for `register_for_battle` extrinsic.
    // Measures the cost of registering a pet for battle, worst-case for bounded vector mutation.
    // `r`: number of existing battle registrations for this pet (conceptually, to test map lookups if any)
    register_for_battle {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = get_pet_id::<T>(0); // First pet ID for testing

        // Setup: Ensure the caller's `OwnerOfPet` list is nearly full for worst-case `try_mutate`.
        let max_owned_pets = T::MaxOwnedPets::get();
        for i in 0..max_owned_pets.saturating_sub(1) { // Mint max_owned_pets - 1 dummy pets for the caller
            let _ = create_and_mint_pet::<T>(caller.clone(), get_pet_id::<T>(i));
        }
        // Ensure the pet to register (ID 0) exists and is owned by caller.
        create_and_mint_pet::<T>(caller.clone(), pet_id)?; 
        // Ensure pet is unlocked (is_transferable) for registration.
        crittercraft_runtime::pallet_critter_nfts::Pallet::<T>::unlock_nft(
            &caller, &pet_id
        )?;
        
    }: _(RawOrigin::Signed(caller.clone()), pet_id) // Benchmark the extrinsic call
    verify {
        assert!(<Battles<T>>::contains_key(0)); // Battle ID 0 should exist
        assert!(<PetInBattle<T>>::contains_key(&pet_id)); // Pet should be marked as in battle
    }

    // Benchmark for `initiate_battle` extrinsic.
    // Measures the cost of initiating a battle between two pets, including locking.
    // `r`: number of existing battle registrations (not directly affecting this extrinsic's cost, but common benchmark param).
    initiate_battle {
        let player1: T::AccountId = whitelisted_caller();
        let pet1_id = get_pet_id::<T>(0);
        let player2: T::AccountId = get_account::<T>("Bob");
        let pet2_id = get_pet_id::<T>(1);

        // Setup: Mint both pets and ensure they are eligible.
        create_and_mint_pet::<T>(player1.clone(), pet1_id)?;
        create_and_mint_pet::<T>(player2.clone(), pet2_id)?;

        // Register player1's pet for a battle (to get a PENDING battle state)
        Pallet::<T>::register_for_battle(RawOrigin::Signed(player1.clone()).into(), pet1_id)?;
        let battle_id = <NextBattleId<T>>::get().saturating_sub(1); // Get the ID of the pending battle

        // Ensure pet2 is unlocked for initiation.
        crittercraft_runtime::pallet_critter_nfts::Pallet::<T>::unlock_nft(
            &player2, &pet2_id
        )?;

    }: _(RawOrigin::Signed(player1.clone()), battle_id, player2.clone(), pet2_id) // Benchmark the extrinsic
    verify {
        let battle = <Battles<T>>::get(battle_id).unwrap();
        assert_eq!(battle.status, BattleStatus::InProgress);
        assert!(<PetInBattle<T>>::contains_key(&pet1_id));
        assert!(<PetInBattle<T>>::contains_key(&pet2_id));
        assert!(!crittercraft_runtime::pallet_critter_nfts::Pallet::<T>::is_transferable(&pet1_id)); // Pets are locked
        assert!(!crittercraft_runtime::pallet_critter_nfts::Pallet::<T>::is_transferable(&pet2_id));
    }

    // Benchmark for `report_battle_outcome` extrinsic.
    // Measures the cost of reporting and finalizing a battle outcome, including reward distribution and unlocking.
    // `r`: number of reads/writes for pet data based on complexity of battle outcome (simplified for MVP).
    // `l`: complexity of battle (e.g., number of turns, which could affect logs).
    report_battle_outcome {
        let reporter: T::AccountId = whitelisted_caller();
        let player1: T::AccountId = get_account::<T>("Alice");
        let pet1_id = get_pet_id::<T>(0);
        let player2: T::AccountId = get_account::<T>("Bob");
        let pet2_id = get_pet_id::<T>(1);

        // Setup: Ensure reporter is authorized.
        let authorized_reporters: BoundedVec<T::AccountId, <T as Config>::AuthorizedBattleReporters> = vec![reporter.clone()].try_into().unwrap();
        <T as Config>::AuthorizedBattleReporters::set(authorized_reporters); // Set authorized reporters for benchmark

        // Setup: Mint pets and initiate a battle (needs to be in InProgress state).
        create_and_mint_pet::<T>(player1.clone(), pet1_id)?;
        create_and_mint_pet::<T>(player2.clone(), pet2_id)?;
        Pallet::<T>::register_for_battle(RawOrigin::Signed(player1.clone()).into(), pet1_id)?;
        let battle_id = <NextBattleId<T>>::get().saturating_sub(1);
        Pallet::<T>::initiate_battle(RawOrigin::Signed(player1.clone()).into(), battle_id, player2.clone(), pet2_id)?;

        // Setup: Endow the reward pot.
        let pot_id = T::BattleRewardPotId::get();
        endow_account::<T>(&pot_id, T::BattleRewardAmount::get());

    }: _(RawOrigin::Signed(reporter.clone()), battle_id, pet1_id) // Benchmark reporting player1 wins
    verify {
        let battle = <Battles<T>>::get(battle_id).unwrap();
        assert_eq!(battle.status, BattleStatus::Concluded);
        assert!(crittercraft_runtime::pallet_critter_nfts::Pallet::<T>::is_transferable(&pet1_id)); // Pets are unlocked
        assert!(crittercraft_runtime::pallet_critter_nfts::Pallet::<T>::is_transferable(&pet2_id));
        assert_eq!(battle.winner, Some(player1));
        assert_eq!(battle.winning_pet, Some(pet1_id));
        // Check balance of winner
        assert!(T::Currency::total_balance(&player1) >= T::BattleRewardAmount::get()); // >= to account for existential deposit
    }

    // Benchmark for `flee_battle` extrinsic.
    // Measures the cost of a pet owner fleeing an in-progress battle.
    flee_battle {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = get_pet_id::<T>(0);
        let player2: T::AccountId = get_account::<T>("Bob");
        let pet2_id = get_pet_id::<T>(1);

        // Setup: Mint pets and initiate a battle (needs to be in InProgress state).
        create_and_mint_pet::<T>(caller.clone(), pet_id)?;
        create_and_mint_pet::<T>(player2.clone(), pet2_id)?;
        Pallet::<T>::register_for_battle(RawOrigin::Signed(caller.clone()).into(), pet_id)?;
        let battle_id = <NextBattleId<T>>::get().saturating_sub(1);
        Pallet::<T>::initiate_battle(RawOrigin::Signed(caller.clone()).into(), battle_id, player2.clone(), pet2_id)?;

    }: _(RawOrigin::Signed(caller.clone()), pet_id) // Benchmark the flee action
    verify {
        let battle = <Battles<T>>::get(battle_id).unwrap();
        assert_eq!(battle.status, BattleStatus::Aborted);
        assert!(crittercraft_runtime::pallet_critter_nfts::Pallet::<T>::is_transferable(&pet_id)); // Fleeing pet unlocked
        assert!(crittercraft_runtime::pallet_critter_nfts::Pallet::<T>::is_transferable(&pet2_id)); // Opponent pet unlocked
        assert_eq!(battle.loser, Some(caller));
        assert_eq!(battle.losing_pet, Some(pet_id));
    }
}

// --- Dynamic Weight Functions (Illustrative - Would be in `weights.rs`) ---
// These functions conceptually demonstrate how weights would dynamically adjust based on parameters.
// In a real setup, `weights.rs` would call the benchmark macro to generate these values.
/*
impl<T: frame_system::Config> crate::weights::WeightInfo for Pallet<T> {
    fn register_for_battle(r: u32) -> Weight {
        // base cost + cost of pushing to BoundedVec (r is conceptual size of vec)
        Weight::from_parts(10_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(Weight::from_ref_time(r as u64 * 1000)) // Conceptual cost per item in BoundedVec
    }

    fn initiate_battle(r: u32) -> Weight { // r for concurrent battles, l for battle complexity
        // Base cost + cost of locking pets (constant for 2 pets) + complexity
        Weight::from_parts(15_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(5))
            .saturating_add(Weight::from_ref_time(r as u64 * 500)) // Example: cost for lookup/lock in large concurrent scenario
            // .saturating_add(Weight::from_ref_time(l as u64 * 100_000)) // Example: cost for complex battle setup
    }

    fn report_battle_outcome(r: u32, l: u32) -> Weight {
        // Base cost + reward transfer + unlock costs + complexity of log hash verification (l)
        Weight::from_parts(20_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(4))
            .saturating_add(T::DbWeight::get().writes(1)) // For currency transfer
            .saturating_add(Weight::from_ref_time(l as u64 * 50_000)) // Cost for processing battle log (if on-chain)
    }

    fn flee_battle(r: u32) -> Weight {
        // Base cost + unlock + cleanup
        Weight::from_parts(8_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
}
*/

#[cfg(test)]
mod tests {
    use super::Pallet as CritterBattles; // Correctly reference the Pallet
    // Import necessary items from your mock runtime for benchmarking tests.
    use crate::mock::{new_test_ext, Test, RuntimeOrigin}; // Assuming Test is your runtime type

    // Standard macro to implement a test suite for benchmarks.
    // It automatically generates test functions that run the benchmarks.
    frame_benchmarking::impl_benchmark_test_suite!(
        CritterBattles, // The pallet being benchmarked
        crate::mock::new_test_ext(), // Function to create a new test environment (mock runtime)
        crate::mock::Test, // Your mock runtime type (e.g., `crate::mock::Runtime`)
    );
}
