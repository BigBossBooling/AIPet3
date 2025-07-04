//! # Profile Pallet
//!
//! ## The Architect's Vision
//!
//! This pallet acts as the core of the "Zoologist's Passport," serving as a generic
//! and scalable reputation engine. Guided by the Expanded KISS Principle, it is
//! completely decoupled from the specific game mechanics of other pallets.
//!
//! It uses a `ScoreContributor` trait to aggregate metrics from any number of
//! external sources, allowing for a flexible and maintainable progression system
//! that can evolve without changes to this core pallet's logic.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// --- Trait Definition for Score Contributors ---

/// A generic trait that other pallets implement to contribute to the overall user score.
/// (K) - This is the key to decoupling. This pallet doesn't care what a 'quest' or
/// 'battle' is, only that something can contribute a score.
pub trait ScoreContributor<AccountId> {
    /// Returns the weighted score contribution for a given user from this source.
    fn get_score_contribution(user: &AccountId) -> u64;
}

/// A no-op implementation for an empty tuple.
impl<AccountId> ScoreContributor<AccountId> for () {
    fn get_score_contribution(_user: &AccountId) -> u64 {
        0
    }
}

/// Implementation for a tuple of contributors.
/// (S) - This allows the runtime to be configured with any number of score sources.
impl<AccountId, A, B> ScoreContributor<AccountId> for (A, B)
where
    A: ScoreContributor<AccountId>,
    B: ScoreContributor<AccountId>,
{
    fn get_score_contribution(user: &AccountId) -> u64 {
        A::get_score_contribution(user).saturating_add(B::get_score_contribution(user))
    }
}
// Note: This can be extended with a macro for larger tuples if needed.

#[frame_support::pallet]
pub mod pallet {
    use super::ScoreContributor;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;

    pub type ScoreValue = u64;

    /// A simplified struct to hold a user's aggregated score and metadata.
    /// (K) - It no longer duplicates state from other pallets.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct UserProfile<BlockNumber> {
        pub overall_score: ScoreValue,
        pub last_active_block: BlockNumber,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// (S) A tuple of all pallets that can contribute to the user's score.
        /// Example in runtime: `type ScoreContributors = (Quests, Battles, NftLevels);`
        type ScoreContributors: ScoreContributor<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn user_profiles)]
    /// Stores the UserProfile struct for each account.
    pub type UserProfiles<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        UserProfile<BlockNumberFor<T>>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A user's profile score has been updated. [user, new_overall_score]
        ProfileScoreUpdated { user: T::AccountId, new_score: ScoreValue },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The profile could not be found. Should not happen with `ValueQuery`.
        ProfileNotFound,
    }

    // `Hooks` can be used for passive updates, but an explicit trigger is more robust.

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // This pallet has no direct user-facing calls. It's an internal engine.
    }

    // --- Public API for Other Pallets ---
    impl<T: Config> Pallet<T> {
        /// (I) - A single, intuitive entry point for other pallets to trigger a score update.
        /// This is the *only* function other pallets need to know about.
        pub fn trigger_score_update(user: &T::AccountId) -> DispatchResult {
            let new_score = T::ScoreContributors::get_score_contribution(user);
            let current_block = frame_system::Pallet::<T>::block_number();

            UserProfiles::<T>::mutate(user, |profile| {
                profile.overall_score = new_score;
                profile.last_active_block = current_block;
            });

            Self::deposit_event(Event::ProfileScoreUpdated {
                user: user.clone(),
                new_score,
            });

            Ok(())
        }
    }
}