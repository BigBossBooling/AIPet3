//! Benchmarking for pallet-critter-minigames
//!
//! This file defines benchmarks for each extrinsic using the `frame_benchmarking` framework.
//! These benchmarks measure the computational and storage costs of pallet operations,
//! which are then used to generate accurate dispatch weights for transaction fees
//! and to ensure the economic integrity of the CritterChain network.
//!
//! Run `cargo test --features=runtime-benchmarks` and `cargo benchmark` to generate weights.
//! Meticulously crafted to align with The Architect's vision for
//! performance optimization and resource management in the CritterCraft digital ecosystem.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use sp_std::prelude::*;
use frame_support::traits::Get;

// Helper functions and constants
const SEED: u32 = 0;

fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    account(name, SEED, 0)
}

benchmarks! {
    // Benchmark for creating a game
    create_game {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32; // Assuming pet with ID 0 exists and is owned by caller
        
        // Mock the NftHandler trait to return true for is_owner
        // This would need to be properly implemented in the runtime
    }: {
        Pallet::<T>::create_game(RawOrigin::Signed(caller.clone()).into(), pet_id, GameType::LogicLeaper, GameDifficulty::Medium)?;
    }
    verify {
        let game_id = 0u32; // First game created should have ID 0
        assert!(GameInstances::<T>::contains_key(game_id));
        let game = GameInstances::<T>::get(game_id).unwrap();
        assert_eq!(game.pet_id, pet_id);
        assert_eq!(game.game_type, GameType::LogicLeaper);
        assert_eq!(game.difficulty, GameDifficulty::Medium);
        assert_eq!(game.status, GameStatus::InProgress);
    }

    // Benchmark for completing a game
    complete_game {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let game_id = 0u32;
        let score = 500u32;
        
        // Create a game first
        Pallet::<T>::create_game(RawOrigin::Signed(caller.clone()).into(), pet_id, GameType::LogicLeaper, GameDifficulty::Medium)?;
    }: {
        Pallet::<T>::complete_game(RawOrigin::Signed(caller.clone()).into(), game_id, score)?;
    }
    verify {
        assert!(GameInstances::<T>::contains_key(game_id));
        let game = GameInstances::<T>::get(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert_eq!(game.score, Some(score));
        
        assert!(GameResults::<T>::contains_key(game_id));
        let result = GameResults::<T>::get(game_id).unwrap();
        assert_eq!(result.score, score);
    }

    // Benchmark for abandoning a game
    abandon_game {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let game_id = 0u32;
        
        // Create a game first
        Pallet::<T>::create_game(RawOrigin::Signed(caller.clone()).into(), pet_id, GameType::LogicLeaper, GameDifficulty::Medium)?;
    }: {
        Pallet::<T>::abandon_game(RawOrigin::Signed(caller.clone()).into(), game_id)?;
    }
    verify {
        assert!(GameInstances::<T>::contains_key(game_id));
        let game = GameInstances::<T>::get(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Abandoned);
    }

    // Benchmark for completing Logic Leaper mini-game
    complete_logic_leaper {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let difficulty = GameDifficulty::Medium;
        let score = 500u32;
    }: {
        Pallet::<T>::complete_logic_leaper(RawOrigin::Signed(caller.clone()).into(), pet_id, difficulty, score)?;
    }
    verify {
        let game_id = 0u32;
        assert!(GameInstances::<T>::contains_key(game_id));
        let game = GameInstances::<T>::get(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert_eq!(game.game_type, GameType::LogicLeaper);
        assert_eq!(game.score, Some(score));
    }

    // Benchmark for completing Aura Weaving mini-game
    complete_aura_weaving {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let difficulty = GameDifficulty::Medium;
        let score = 500u32;
    }: {
        Pallet::<T>::complete_aura_weaving(RawOrigin::Signed(caller.clone()).into(), pet_id, difficulty, score)?;
    }
    verify {
        let game_id = 0u32;
        assert!(GameInstances::<T>::contains_key(game_id));
        let game = GameInstances::<T>::get(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert_eq!(game.game_type, GameType::AuraWeaving);
        assert_eq!(game.score, Some(score));
    }

    // Benchmark for completing Habitat Dash mini-game
    complete_habitat_dash {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let difficulty = GameDifficulty::Medium;
        let score = 500u32;
    }: {
        Pallet::<T>::complete_habitat_dash(RawOrigin::Signed(caller.clone()).into(), pet_id, difficulty, score)?;
    }
    verify {
        let game_id = 0u32;
        assert!(GameInstances::<T>::contains_key(game_id));
        let game = GameInstances::<T>::get(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert_eq!(game.game_type, GameType::HabitatDash);
        assert_eq!(game.score, Some(score));
    }
}

#[cfg(test)]
mod tests {
    use super::Pallet as CritterMinigames;
    frame_benchmarking::impl_benchmark_test_suite!(
        CritterMinigames,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}