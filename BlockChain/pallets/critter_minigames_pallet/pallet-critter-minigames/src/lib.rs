//! # Critter Mini-Games Pallet
//!
//! This pallet manages mini-games and activities for CritterCraft pets.
//! It defines the game types, rewards, and interactions that drive pet development
//! through engaging gameplay loops.
//!
//! Meticulously crafted to align with The Architect's vision for
//! modularity, scalability, and robust operation of the CritterCraft digital ecosystem.

#![cfg_attr(not(feature = "std"), no_std)] // No standard library for Wasm compilation

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*, // Provides common types and macros for pallets
        traits::{Currency, Randomness}, // Currency for balances, Randomness for game outcomes
        BoundedVec, // For bounded collections, crucial for security
    };
    use frame_system::{
        pallet_prelude::*, // Provides types like BlockNumberFor, AccountId, OriginFor
        ensure_signed,     // Macro to ensure origin is a signed account
    };
    use sp_std::vec::Vec; // Standard Vec for dynamic arrays (used where not bounded)
    use scale_info::TypeInfo; // For `TypeInfo` derive macro
    use frame_support::log; // Correct way to import Substrate's logging macro
    use sp_runtime::SaturatedFrom; // For saturating arithmetic

    // Import traits from critter-nfts pallet
    use crate::traits::{
        NftManagerForItems, // For integration with pet NFTs
        PetId,             // Using PetId from critter-nfts
        ItemId,            // Using ItemId from pallet-items
    };

    // --- Type Aliases ---
    pub type GameId = u32; // Unique identifier for each mini-game instance
    pub type ScoreType = u32; // Type for game scores

    // --- Enum Definitions ---
    // GameType: Defines the different types of mini-games available
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum GameType {
        LogicLeaper,   // Intelligence training
        AuraWeaving,   // Charisma training
        HabitatDash,   // Energy/Agility training
        CritterTactics, // Strategic duel (2-player)
        CooperativeCrafting, // Cooperative game (2-player)
    }

    // GameDifficulty: Defines the difficulty levels for mini-games
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum GameDifficulty {
        Easy,
        Medium,
        Hard,
        Expert,
    }

    // GameStatus: Defines the current status of a game instance
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum GameStatus {
        InProgress,
        Completed,
        Abandoned,
    }

    // --- Struct Definitions ---
    // GameInstance: Defines a specific instance of a mini-game
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct GameInstance<T: Config> {
        pub id: GameId,
        pub game_type: GameType,
        pub difficulty: GameDifficulty,
        pub pet_id: PetId,
        pub owner: T::AccountId,
        pub start_block: BlockNumberFor<T>,
        pub end_block: Option<BlockNumberFor<T>>,
        pub status: GameStatus,
        pub score: Option<ScoreType>,
        pub xp_reward: Option<u32>,
        pub bits_reward: Option<BalanceOf<T>>,
    }

    // GameResult: Defines the result of a completed game
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct GameResult<T: Config> {
        pub game_id: GameId,
        pub pet_id: PetId,
        pub score: ScoreType,
        pub xp_gained: u32,
        pub bits_earned: BalanceOf<T>,
        pub completion_block: BlockNumberFor<T>,
    }

    // BalanceOf<T> type alias for the pallet's currency type.
    pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // --- Pallet Configuration Trait ---
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// The currency trait for handling BITS token balances.
        type Currency: Currency<Self::AccountId>;

        /// The randomness trait for generating game outcomes.
        type GameRandomness: Randomness<Self::Hash, Self::BlockNumber>;
        
        /// Maximum number of active games an account can have.
        #[pallet::constant]
        type MaxActiveGames: Get<u32>;
        
        /// Maximum length of a game result comment (in bytes).
        #[pallet::constant]
        type MaxCommentLen: Get<u32>;
        
        /// Base XP reward for completing a game.
        #[pallet::constant]
        type BaseXpReward: Get<u32>;
        
        /// Base BITS reward for completing a game.
        #[pallet::constant]
        type BaseBitsReward: Get<BalanceOf<Self>>;
        
        /// Multiplier for XP based on difficulty.
        #[pallet::constant]
        type DifficultyXpMultiplier: Get<u32>;
        
        /// Multiplier for BITS based on difficulty.
        #[pallet::constant]
        type DifficultyBitsMultiplier: Get<u32>;
        
        /// Handler for interacting with pet NFTs.
        type NftHandler: NftManagerForItems<Self::AccountId, PetId, ItemId, DispatchResult>;
    }

    // --- Pallet Definition ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // --- Pallet Storage Items ---
    #[pallet::storage]
    #[pallet::getter(fn next_game_id)]
    /// Stores the next available unique GameId.
    pub(super) type NextGameId<T: Config> = StorageValue<_, GameId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn game_instances)]
    /// Stores the comprehensive GameInstance data for each GameId.
    pub(super) type GameInstances<T: Config> = StorageMap<_, Blake2_128Concat, GameId, GameInstance<T>>;

    #[pallet::storage]
    #[pallet::getter(fn active_games_by_owner)]
    /// Stores a list of active GameIds for each AccountId.
    pub(super) type ActiveGamesByOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<GameId, T::MaxActiveGames>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn game_results)]
    /// Stores the results of completed games.
    pub(super) type GameResults<T: Config> = StorageMap<_, Blake2_128Concat, GameId, GameResult<T>>;

    #[pallet::storage]
    #[pallet::getter(fn pet_game_history)]
    /// Stores a list of GameIds that a pet has participated in.
    pub(super) type PetGameHistory<T: Config> = StorageMap<_, Blake2_128Concat, PetId, Vec<GameId>, ValueQuery>;

    // --- Pallet Events ---
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new game instance has been created. [owner, pet_id, game_id, game_type]
        GameCreated { owner: T::AccountId, pet_id: PetId, game_id: GameId, game_type: GameType },
        
        /// A game has been completed. [owner, pet_id, game_id, score, xp_gained, bits_earned]
        GameCompleted { owner: T::AccountId, pet_id: PetId, game_id: GameId, score: ScoreType, xp_gained: u32, bits_earned: BalanceOf<T> },
        
        /// A game has been abandoned. [owner, pet_id, game_id]
        GameAbandoned { owner: T::AccountId, pet_id: PetId, game_id: GameId },
        
        /// A pet has leveled up from game rewards. [pet_id, new_level]
        PetLeveledUp { pet_id: PetId, new_level: u32 },
    }

    // --- Pallet Errors ---
    #[pallet::error]
    pub enum Error<T> {
        /// The next GameId has overflowed.
        NextGameIdOverflow,
        
        /// An account cannot have more active games than MaxActiveGames.
        ExceedMaxActiveGames,
        
        /// The specified game instance does not exist.
        GameNotFound,
        
        /// The sender is not the owner of the game instance.
        NotGameOwner,
        
        /// The game is already completed or abandoned.
        GameAlreadyFinished,
        
        /// The game is still in progress.
        GameStillInProgress,
        
        /// The pet is already participating in another game.
        PetAlreadyInGame,
        
        /// The pet does not exist or is not owned by the sender.
        PetNotOwnedBySender,
        
        /// The pet's stats are too low for the selected difficulty.
        PetStatsInsufficient,
        
        /// The score is invalid for the game type.
        InvalidScore,
        
        /// Failed to update pet's experience or stats.
        PetUpdateFailed,
        
        /// Failed to transfer BITS rewards.
        RewardTransferFailed,
    }

    // --- Pallet Hooks ---
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // --- Pallet Extrinsics ---
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new mini-game instance.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_game(
            origin: OriginFor<T>,
            pet_id: PetId,
            game_type: GameType,
            difficulty: GameDifficulty,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Check if the sender owns the pet.
            ensure!(T::NftHandler::is_owner(&owner, &pet_id), Error::<T>::PetNotOwnedBySender);
            
            // 2. Check if the account has reached the maximum number of active games.
            let active_games = ActiveGamesByOwner::<T>::get(&owner);
            ensure!(active_games.len() < T::MaxActiveGames::get() as usize, Error::<T>::ExceedMaxActiveGames);
            
            // 3. Check if the pet is already in an active game.
            for game_id in active_games.iter() {
                if let Some(game) = GameInstances::<T>::get(game_id) {
                    if game.pet_id == pet_id && game.status == GameStatus::InProgress {
                        return Err(Error::<T>::PetAlreadyInGame.into());
                    }
                }
            }
            
            // 4. Get the next game ID.
            let game_id = Self::next_game_id();
            let next_game_id = game_id.checked_add(1).ok_or(Error::<T>::NextGameIdOverflow)?;
            NextGameId::<T>::put(next_game_id);
            
            // 5. Create the game instance.
            let current_block = frame_system::Pallet::<T>::block_number();
            let game_instance = GameInstance::<T> {
                id: game_id,
                game_type,
                difficulty,
                pet_id,
                owner: owner.clone(),
                start_block: current_block,
                end_block: None,
                status: GameStatus::InProgress,
                score: None,
                xp_reward: None,
                bits_reward: None,
            };
            
            // 6. Store the game instance.
            GameInstances::<T>::insert(game_id, game_instance);
            
            // 7. Update the active games for the owner.
            ActiveGamesByOwner::<T>::try_mutate(&owner, |games| -> DispatchResult {
                games.try_push(game_id).map_err(|_| Error::<T>::ExceedMaxActiveGames)?;
                Ok(())
            })?;
            
            // 8. Update the pet's game history.
            PetGameHistory::<T>::mutate(pet_id, |games| {
                games.push(game_id);
            });
            
            // 9. Emit the event.
            Self::deposit_event(Event::GameCreated {
                owner,
                pet_id,
                game_id,
                game_type,
            });
            
            Ok(())
        }

        /// Complete a mini-game and claim rewards.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn complete_game(
            origin: OriginFor<T>,
            game_id: GameId,
            score: ScoreType,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Get the game instance.
            let mut game = GameInstances::<T>::get(game_id).ok_or(Error::<T>::GameNotFound)?;
            
            // 2. Check if the sender is the owner of the game.
            ensure!(game.owner == owner, Error::<T>::NotGameOwner);
            
            // 3. Check if the game is still in progress.
            ensure!(game.status == GameStatus::InProgress, Error::<T>::GameAlreadyFinished);
            
            // 4. Validate the score based on game type and difficulty.
            ensure!(Self::is_valid_score(game.game_type, game.difficulty, score), Error::<T>::InvalidScore);
            
            // 5. Calculate rewards based on game type, difficulty, and score.
            let (xp_reward, bits_reward) = Self::calculate_rewards(game.game_type, game.difficulty, score);
            
            // 6. Update the game instance.
            let current_block = frame_system::Pallet::<T>::block_number();
            game.end_block = Some(current_block);
            game.status = GameStatus::Completed;
            game.score = Some(score);
            game.xp_reward = Some(xp_reward);
            game.bits_reward = Some(bits_reward);
            
            // 7. Store the updated game instance.
            GameInstances::<T>::insert(game_id, game.clone());
            
            // 8. Create and store the game result.
            let game_result = GameResult::<T> {
                game_id,
                pet_id: game.pet_id,
                score,
                xp_gained: xp_reward,
                bits_earned: bits_reward,
                completion_block: current_block,
            };
            GameResults::<T>::insert(game_id, game_result);
            
            // 9. Update the pet's experience and stats.
            // This would call into the NftHandler to update the pet's XP.
            // For now, we'll just emit an event.
            
            // 10. Transfer BITS rewards to the owner.
            T::Currency::deposit_creating(&owner, bits_reward);
            
            // 11. Remove the game from active games.
            ActiveGamesByOwner::<T>::try_mutate(&owner, |games| -> DispatchResult {
                if let Some(pos) = games.iter().position(|&id| id == game_id) {
                    games.swap_remove(pos);
                }
                Ok(())
            })?;
            
            // 12. Emit the event.
            Self::deposit_event(Event::GameCompleted {
                owner,
                pet_id: game.pet_id,
                game_id,
                score,
                xp_gained: xp_reward,
                bits_earned: bits_reward,
            });
            
            Ok(())
        }

        /// Abandon a mini-game without claiming rewards.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn abandon_game(
            origin: OriginFor<T>,
            game_id: GameId,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Get the game instance.
            let mut game = GameInstances::<T>::get(game_id).ok_or(Error::<T>::GameNotFound)?;
            
            // 2. Check if the sender is the owner of the game.
            ensure!(game.owner == owner, Error::<T>::NotGameOwner);
            
            // 3. Check if the game is still in progress.
            ensure!(game.status == GameStatus::InProgress, Error::<T>::GameAlreadyFinished);
            
            // 4. Update the game instance.
            let current_block = frame_system::Pallet::<T>::block_number();
            game.end_block = Some(current_block);
            game.status = GameStatus::Abandoned;
            
            // 5. Store the updated game instance.
            GameInstances::<T>::insert(game_id, game.clone());
            
            // 6. Remove the game from active games.
            ActiveGamesByOwner::<T>::try_mutate(&owner, |games| -> DispatchResult {
                if let Some(pos) = games.iter().position(|&id| id == game_id) {
                    games.swap_remove(pos);
                }
                Ok(())
            })?;
            
            // 7. Emit the event.
            Self::deposit_event(Event::GameAbandoned {
                owner,
                pet_id: game.pet_id,
                game_id,
            });
            
            Ok(())
        }

        /// Complete the Logic Leaper mini-game (Intelligence training).
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn complete_logic_leaper(
            origin: OriginFor<T>,
            pet_id: PetId,
            difficulty: GameDifficulty,
            score: ScoreType,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Create a new game instance.
            let create_result = Self::create_game(
                RawOrigin::Signed(owner.clone()).into(),
                pet_id,
                GameType::LogicLeaper,
                difficulty,
            );
            
            // 2. If game creation failed, return the error.
            if let Err(e) = create_result {
                return Err(e);
            }
            
            // 3. Get the game ID that was just created.
            let game_id = Self::next_game_id() - 1;
            
            // 4. Complete the game.
            Self::complete_game(
                RawOrigin::Signed(owner).into(),
                game_id,
                score,
            )
        }

        /// Complete the Aura Weaving mini-game (Charisma training).
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn complete_aura_weaving(
            origin: OriginFor<T>,
            pet_id: PetId,
            difficulty: GameDifficulty,
            score: ScoreType,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Create a new game instance.
            let create_result = Self::create_game(
                RawOrigin::Signed(owner.clone()).into(),
                pet_id,
                GameType::AuraWeaving,
                difficulty,
            );
            
            // 2. If game creation failed, return the error.
            if let Err(e) = create_result {
                return Err(e);
            }
            
            // 3. Get the game ID that was just created.
            let game_id = Self::next_game_id() - 1;
            
            // 4. Complete the game.
            Self::complete_game(
                RawOrigin::Signed(owner).into(),
                game_id,
                score,
            )
        }

        /// Complete the Habitat Dash mini-game (Energy/Agility training).
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn complete_habitat_dash(
            origin: OriginFor<T>,
            pet_id: PetId,
            difficulty: GameDifficulty,
            score: ScoreType,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Create a new game instance.
            let create_result = Self::create_game(
                RawOrigin::Signed(owner.clone()).into(),
                pet_id,
                GameType::HabitatDash,
                difficulty,
            );
            
            // 2. If game creation failed, return the error.
            if let Err(e) = create_result {
                return Err(e);
            }
            
            // 3. Get the game ID that was just created.
            let game_id = Self::next_game_id() - 1;
            
            // 4. Complete the game.
            Self::complete_game(
                RawOrigin::Signed(owner).into(),
                game_id,
                score,
            )
        }
    }

    // --- Pallet Internal Helper Functions ---
    impl<T: Config> Pallet<T> {
        /// Check if a score is valid for a given game type and difficulty.
        fn is_valid_score(game_type: GameType, difficulty: GameDifficulty, score: ScoreType) -> bool {
            // For MVP, we'll just check that the score is within a reasonable range.
            // In a real implementation, this would be more sophisticated.
            match game_type {
                GameType::LogicLeaper => score <= 1000,
                GameType::AuraWeaving => score <= 1000,
                GameType::HabitatDash => score <= 1000,
                GameType::CritterTactics => score <= 100,
                GameType::CooperativeCrafting => score <= 100,
            }
        }

        /// Calculate rewards based on game type, difficulty, and score.
        fn calculate_rewards(game_type: GameType, difficulty: GameDifficulty, score: ScoreType) -> (u32, BalanceOf<T>) {
            // Base rewards
            let base_xp = T::BaseXpReward::get();
            let base_bits = T::BaseBitsReward::get();
            
            // Difficulty multiplier
            let difficulty_multiplier = match difficulty {
                GameDifficulty::Easy => 1,
                GameDifficulty::Medium => 2,
                GameDifficulty::Hard => 3,
                GameDifficulty::Expert => 4,
            };
            
            // Score factor (0.1 to 1.0 based on score)
            let score_factor = match game_type {
                GameType::LogicLeaper => (score as f32 / 1000.0).min(1.0).max(0.1),
                GameType::AuraWeaving => (score as f32 / 1000.0).min(1.0).max(0.1),
                GameType::HabitatDash => (score as f32 / 1000.0).min(1.0).max(0.1),
                GameType::CritterTactics => (score as f32 / 100.0).min(1.0).max(0.1),
                GameType::CooperativeCrafting => (score as f32 / 100.0).min(1.0).max(0.1),
            };
            
            // Calculate final rewards
            let xp_reward = (base_xp * difficulty_multiplier * (score_factor * 100.0) as u32) / 100;
            let bits_reward = BalanceOf::<T>::saturated_from(
                (base_bits.saturated_into::<u32>() * difficulty_multiplier * (score_factor * 100.0) as u32) / 100
            );
            
            (xp_reward, bits_reward)
        }
    }
}

// Define the traits module for external interfaces
pub mod traits {
    use super::*;
    use frame_support::dispatch::DispatchResult;

    // Re-export types from critter-nfts pallet
    pub type PetId = u32;
    pub type ItemId = u32;

    // Trait for interacting with pet NFTs
    pub trait NftManagerForItems<AccountId, NftId, ItemId, Result> {
        fn is_owner(owner: &AccountId, pet_id: &NftId) -> bool;
        fn add_experience(pet_id: &NftId, xp_amount: u32) -> Result;
    }
}

// Benchmarking module (empty for now)
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use super::*;
    use frame_benchmarking::{benchmarks, whitelisted_caller, account};
    use frame_system::RawOrigin;

    benchmarks! {
        // Benchmarks would be defined here
    }
}