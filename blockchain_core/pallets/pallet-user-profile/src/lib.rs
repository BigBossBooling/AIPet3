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

    /// Struct to hold various user scores and reputation metrics.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct UserProfile<BlockNumber> { // Made generic for BlockNumber
        pub total_pet_levels_sum: ScoreValue,
        pub quests_completed_count: u32,
        pub battles_won_count: u32,
        pub successful_trades_count: u32,    // e.g., from marketplace
        pub community_contributions_score: ScoreValue, // e.g., from future governance participation or job system
        pub overall_progress_score: ScoreValue, // Could be a weighted combination of other scores
        pub trade_reputation_score: i32, // Can go up/down based on trade feedback (future)
        pub last_active_block: BlockNumber,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // No direct currency needed if this pallet only aggregates scores.
        // Dependencies on other pallets for score updates will be via direct calls or a trait system.

        // Constants for calculating overall_progress_score (weights)
        #[pallet::constant]
        type PetLevelScoreWeight: Get<ScoreValue>;
        #[pallet::constant]
        type QuestScoreWeight: Get<ScoreValue>;
        #[pallet::constant]
        type BattleWinScoreWeight: Get<ScoreValue>;
        #[pallet::constant]
        type TradeScoreWeight: Get<ScoreValue>;
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
        /// Trade reputation for a user changed. [user, new_reputation, change_delta]
        TradeReputationChanged { user: T::AccountId, new_reputation: i32, change_delta: i32 },
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
        /// Called by other pallets to record user activity and update their last_active_block.
        pub fn record_user_activity(user: &T::AccountId) -> DispatchResult {
            let current_block = frame_system::Pallet::<T>::block_number();
            UserProfiles::<T>::mutate(user, |profile| {
                profile.last_active_block = current_block;
            });
            // Potentially emit an event if needed, or just update.
            Ok(())
        }

        /// Called by pallet-critter-nfts when a pet levels up or total levels change.
        pub fn update_pet_level_sum(user: &T::AccountId, new_total_level_sum: ScoreValue) -> DispatchResult {
            UserProfiles::<T>::mutate(user, |profile| {
                profile.total_pet_levels_sum = new_total_level_sum;
                Self::recalculate_overall_score(profile);
                profile.last_active_block = frame_system::Pallet::<T>::block_number();
            });
            Self::deposit_event(Event::UserProfileUpdated {
                user: user.clone(),
                new_overall_score: UserProfiles::<T>::get(user).overall_progress_score
            });
            Ok(())
        }

        /// Called by pallet-quests when a quest is completed.
        pub fn record_quest_completion(user: &T::AccountId) -> DispatchResult {
            UserProfiles::<T>::mutate(user, |profile| {
                profile.quests_completed_count = profile.quests_completed_count.saturating_add(1);
                Self::recalculate_overall_score(profile);
                profile.last_active_block = frame_system::Pallet::<T>::block_number();
            });
            Self::deposit_event(Event::UserProfileUpdated {
                user: user.clone(),
                new_overall_score: UserProfiles::<T>::get(user).overall_progress_score
            });
            Ok(())
        }

        /// Called by pallet-battles when a battle is won.
        pub fn record_battle_win(user: &T::AccountId) -> DispatchResult {
            UserProfiles::<T>::mutate(user, |profile| {
                profile.battles_won_count = profile.battles_won_count.saturating_add(1);
                Self::recalculate_overall_score(profile);
                profile.last_active_block = frame_system::Pallet::<T>::block_number();
            });
            Self::deposit_event(Event::UserProfileUpdated {
                user: user.clone(),
                new_overall_score: UserProfiles::<T>::get(user).overall_progress_score
            });
            Ok(())
        }

        /// Called by pallet-marketplace or pallet-user-shops for successful trades.
        pub fn record_successful_trade(user: &T::AccountId) -> DispatchResult {
            UserProfiles::<T>::mutate(user, |profile| {
                profile.successful_trades_count = profile.successful_trades_count.saturating_add(1);
                Self::recalculate_overall_score(profile);
                profile.last_active_block = frame_system::Pallet::<T>::block_number();
            });
            Self::deposit_event(Event::UserProfileUpdated {
                user: user.clone(),
                new_overall_score: UserProfiles::<T>::get(user).overall_progress_score
            });
            Ok(())
        }

        /// Called by a feedback system (future) or dispute resolution for trades.
        pub fn update_trade_reputation(user: &T::AccountId, reputation_change: i32) -> DispatchResult {
            UserProfiles::<T>::mutate(user, |profile| {
                profile.trade_reputation_score = profile.trade_reputation_score.saturating_add(reputation_change);
                profile.last_active_block = frame_system::Pallet::<T>::block_number();
                // Overall score might not directly depend on reputation, or it could.
            });
            Self::deposit_event(Event::TradeReputationChanged {
                user: user.clone(),
                new_reputation: UserProfiles::<T>::get(user).trade_reputation_score,
                change_delta: reputation_change,
            });
            Ok(())
        }

        /// Called by governance or job system for community contributions.
        pub fn record_community_contribution(user: &T::AccountId, contribution_score_increase: ScoreValue) -> DispatchResult {
            UserProfiles::<T>::mutate(user, |profile| {
                profile.community_contributions_score = profile.community_contributions_score.saturating_add(contribution_score_increase);
                Self::recalculate_overall_score(profile);
                profile.last_active_block = frame_system::Pallet::<T>::block_number();
            });
             Self::deposit_event(Event::UserProfileUpdated {
                user: user.clone(),
                new_overall_score: UserProfiles::<T>::get(user).overall_progress_score
            });
            Ok(())
        }


        // Internal helper to recalculate the overall progress score
        fn recalculate_overall_score(profile: &mut UserProfile<BlockNumberFor<T>>) { // Ensure generic matches
            profile.overall_progress_score =
                profile.total_pet_levels_sum.saturating_mul(T::PetLevelScoreWeight::get()) +
                (profile.quests_completed_count as ScoreValue).saturating_mul(T::QuestScoreWeight::get()) +
                (profile.battles_won_count as ScoreValue).saturating_mul(T::BattleWinScoreWeight::get()) +
                (profile.successful_trades_count as ScoreValue).saturating_mul(T::TradeScoreWeight::get()) +
                profile.community_contributions_score; // Community score added directly for now
        }
    }
}
