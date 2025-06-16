#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Assume AccountId, Balance from frame_system and pallet_balances
// Assume PetId, QuestId, BattleId, etc. are u32 or defined types

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        // May need traits like Currency if scores influence economic factors directly managed here
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_std::vec::Vec; // Only if needed for specific score components

    // Define type aliases for clarity if complex types are used for scores
    pub type ScoreValue = u64; // Generic score value type
    // pub type PetId = u32; // Example, if needed by specific score logic

    /// Struct to hold various user scores and reputation metrics. (Simplified for MVP)
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct UserProfile<BlockNumber> {
        // MVP Core Metrics:
        pub total_pet_levels_sum: ScoreValue,
        pub quests_completed_count: u32,
        pub battles_won_count: u32,
        pub overall_progress_score: ScoreValue, // Derived from the above
        pub last_active_block: BlockNumber,
        // Deferred for Post-MVP:
        // pub successful_trades_count: u32,
        // pub community_contributions_score: ScoreValue,
        // pub trade_reputation_score: i32,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // MVP Score Weights:
        #[pallet::constant]
        type PetLevelScoreWeight: Get<ScoreValue>;
        #[pallet::constant]
        type QuestScoreWeight: Get<ScoreValue>;
        #[pallet::constant]
        type BattleWinScoreWeight: Get<ScoreValue>;
        // Deferred for Post-MVP:
        // #[pallet::constant] type TradeScoreWeight: Get<ScoreValue>;
        // #[pallet::constant] type CommunityContributionScoreWeight: Get<ScoreValue>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn user_profiles)]
    /// Stores the UserProfile struct for each account.
    pub(super) type UserProfiles<T: Config> = StorageMap<
        _,
        Blake2_128Concat, // AccountId is crypto hashable
        T::AccountId,
        UserProfile<BlockNumberFor<T>>, // Use BlockNumberFor<T>
        ValueQuery, // Initialize with Default::default() if not found
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A user's profile has been updated. [user, new_overall_score]
        UserProfileUpdated { user: T::AccountId, new_overall_score: ScoreValue },
        // TradeReputationChanged event removed for MVP as trade_reputation_score is deferred.
        // TradeReputationChanged { user: T::AccountId, new_reputation: i32, change_delta: i32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        // This pallet primarily updates based on calls from other pallets,
        // so direct user-facing errors might be minimal unless it exposes extrinsics later.
        CannotUpdateProfileDirectly, // If direct updates are disallowed
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // Can use on_initialize or on_finalize to update last_active_block for users who transact.
        // This is a conceptual placeholder for activity tracking.
        // A more robust activity tracking might involve a dedicated event system or specific hooks.
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            // Example: If we had a way to get the author of the block or signers of extrinsics
            // in this block, we could update their last_active_block.
            // This is complex for a simple hook here.
            // For now, last_active_block would be updated by other pallets calling a specific function.
            Weight::zero()
        }
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // This pallet might not have many direct user-callable extrinsics initially.
        // Scores are updated by other pallets calling its public Rust functions.
        // Example: A future extrinsic to set a display name or avatar if profile expands.
        // #[pallet::call_index(0)]
        // pub fn set_display_name(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
        //     let user = ensure_signed(origin)?;
        //     // ... logic to update a display_name field in UserProfile ...
        //     Ok(())
        // }
    }

    // --- Public helper functions for other pallets to call ---
    impl<T: Config> Pallet<T> {
        /// Central function to update profile, recalculate overall score, and set last active block.
        fn update_profile_and_recalculate(user: &T::AccountId, mutator: impl FnOnce(&mut UserProfile<BlockNumberFor<T>>)) {
            UserProfiles::<T>::mutate(user, |profile| {
                mutator(profile); // Apply specific changes
                profile.last_active_block = frame_system::Pallet::<T>::block_number();
                Self::recalculate_overall_score(profile); // Recalculate after specific changes
            });
            // Emit event after all changes are done and score is recalculated
            Self::deposit_event(Event::UserProfileUpdated {
                user: user.clone(),
                new_overall_score: UserProfiles::<T>::get(user).overall_progress_score,
            });
        }

        /// Called by other pallets to simply record user activity if no specific score changes.
        pub fn record_user_activity(user: &T::AccountId) -> DispatchResult {
            UserProfiles::<T>::mutate(user, |profile| {
                profile.last_active_block = frame_system::Pallet::<T>::block_number();
            });
            // No UserProfileUpdated event here unless overall score changes, which it doesn't.
            // Could have a generic ActivityRecorded event if useful.
            Ok(())
        }

        /// Called by pallet-critter-nfts when a pet levels up or total levels change for a user.
        pub fn update_pet_level_sum(user: &T::AccountId, new_total_level_sum: ScoreValue) -> DispatchResult {
            Self::update_profile_and_recalculate(user, |profile| {
                profile.total_pet_levels_sum = new_total_level_sum;
            });
            Ok(())
        }

        /// Called by pallet-quests when a quest is completed.
        pub fn record_quest_completion(user: &T::AccountId) -> DispatchResult {
            Self::update_profile_and_recalculate(user, |profile| {
                profile.quests_completed_count = profile.quests_completed_count.saturating_add(1);
                // Conceptual: Add cap for quest score contribution if desired
                // if profile.quests_completed_count > MAX_QUEST_SCORE_CONTRIBUTION_COUNT { /* don't add more to score */ }
            });
            Ok(())
        }

        /// Called by pallet-battles when a battle is won.
        pub fn record_battle_win(user: &T::AccountId) -> DispatchResult {
            Self::update_profile_and_recalculate(user, |profile| {
                profile.battles_won_count = profile.battles_won_count.saturating_add(1);
            });
            Ok(())
        }

        // /// Called by pallet-marketplace or pallet-user-shops for successful trades. (Deferred for MVP)
        // pub fn record_successful_trade(user: &T::AccountId) -> DispatchResult {
        //     Self::update_profile_and_recalculate(user, |profile| {
        //         profile.successful_trades_count = profile.successful_trades_count.saturating_add(1);
        //     });
        //     Ok(())
        // }

        // /// Called by a feedback system (future) or dispute resolution for trades. (Deferred for MVP)
        // pub fn update_trade_reputation(user: &T::AccountId, reputation_change: i32) -> DispatchResult {
        //     UserProfiles::<T>::mutate(user, |profile| {
        //         profile.trade_reputation_score = profile.trade_reputation_score.saturating_add(reputation_change);
        //         profile.last_active_block = frame_system::Pallet::<T>::block_number();
        //     });
        //     Self::deposit_event(Event::TradeReputationChanged {
        //         user: user.clone(),
        //         new_reputation: UserProfiles::<T>::get(user).trade_reputation_score,
        //         change_delta: reputation_change,
        //     });
        //     Ok(())
        // }

        // /// Called by governance or job system for community contributions. (Deferred for MVP)
        // pub fn record_community_contribution(user: &T::AccountId, contribution_score_increase: ScoreValue) -> DispatchResult {
        //     Self::update_profile_and_recalculate(user, |profile| {
        //         profile.community_contributions_score = profile.community_contributions_score.saturating_add(contribution_score_increase);
        //     });
        //     Ok(())
        // }


        // Internal helper to recalculate the overall progress score (Simplified for MVP)
        fn recalculate_overall_score(profile: &mut UserProfile<BlockNumberFor<T>>) {
            let pet_score = profile.total_pet_levels_sum
                .saturating_mul(T::PetLevelScoreWeight::get());

            const QUEST_COUNT_SCORE_CAP: u32 = 500; // Example cap, can be moved to Config if needed
            let effective_quests_count = profile.quests_completed_count.min(QUEST_COUNT_SCORE_CAP) as ScoreValue;
            let quest_score = effective_quests_count
                .saturating_mul(T::QuestScoreWeight::get());

            const BATTLE_WINS_SCORE_CAP: u32 = 1000; // Example cap
            let effective_battles_won = profile.battles_won_count.min(BATTLE_WINS_SCORE_CAP) as ScoreValue;
            let battle_score = effective_battles_won
                .saturating_mul(T::BattleWinScoreWeight::get());

            // Deferred scores (trade_activity_score, community_contributions_score) are removed from calculation for MVP.
            profile.overall_progress_score = pet_score
                .saturating_add(quest_score)
                .saturating_add(battle_score);
        }
    }
}
