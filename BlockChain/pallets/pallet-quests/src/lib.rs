//! # Quest Pallet
//!
//! ## The Architect's Vision
//!
//! This pallet provides a generic and extensible engine for managing quests in the
//! Critter-Craft ecosystem. Guided by the Expanded KISS Principle, it is completely
//! decoupled from the specific logic of quest requirements.
//!
//! It uses a `QuestRequirement` enum to define objectives and a `RequirementVerifier`
//! trait to dispatch verification logic to the runtime, allowing for a highly
//! scalable and maintainable system where new quest types can be added without
//! modifying this pallet's core code.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// --- Trait Definitions for Verification Logic ---

/// A trait that defines the logic for verifying a single quest requirement.
/// (S) - This is the core of our systematic, scalable design. The runtime will
/// implement this trait to provide the actual verification logic.
pub trait RequirementVerifier<AccountId, PetId, ItemId> {
    fn verify(
        &self,
        user: &AccountId,
        context: &VerificationContext<PetId>,
    ) -> DispatchResult;
}

/// Provides contextual information for verification, such as the pet selected by the user.
#[derive(Default, Clone)]
pub struct VerificationContext<PetId> {
    pub target_pet_id: Option<PetId>,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_std::vec::Vec;

    // --- Type Aliases ---
    pub type PetId = u32;
    pub type ItemId = u32;
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // --- Core Data Structures ---

    /// (K) - A rich enum defining all possible quest requirements.
    /// This is far clearer and more extensible than multiple `Option` fields.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum QuestRequirement<ItemId> {
        /// User must possess a specific pet that has reached a certain level.
        PetLevel { pet_id_placeholder: u32, min_level: u32 },
        /// User must have a certain quantity of a specific item.
        HasItem { item_id: ItemId, amount: u32, consume: bool },
        /// User must have won a minimum number of battles.
        BattlesWon { min_wins: u32 },
        // (S) - New requirement types can be added here without changing pallet logic.
        // Example: Must have a specific reputation score.
        // Reputation { min_reputation: i32 },
    }

    /// The core Quest struct, now holding a vector of generic requirements.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Quest<T: Config> {
        pub creator: T::AccountId,
        pub description: BoundedVec<u8, T::MaxDescriptionLength>,
        pub reward: BalanceOf<T>,
        pub requirements: BoundedVec<QuestRequirement<ItemId>, T::MaxRequirementsPerQuest>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type QuestId: Parameter + Member + Copy + Default + MaxEncodedLen + From<u32>;

        /// (S) - The verifier that knows how to check all `QuestRequirement` variants.
        /// This will be implemented in the runtime, giving it access to all other pallets.
        type Verifier: RequirementVerifier<Self::AccountId, PetId, ItemId>;

        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;
        #[pallet::constant]
        type MaxRequirementsPerQuest: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // --- Storage ---
    #[pallet::storage]
    pub type NextQuestId<T: Config> = StorageValue<_, T::QuestId, ValueQuery>;

    #[pallet::storage]
    pub type AvailableQuests<T: Config> = StorageMap<_, Blake2_128Concat, T::QuestId, Quest<T>>;

    #[pallet::storage]
    pub type CompletedQuests<T: Config> =
        StorageMap<_, Twox64Concat, (T::AccountId, T::QuestId), (), OptionQuery>;

    // --- Events ---
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        QuestCreated { quest_id: T::QuestId, creator: T::AccountId, reward: BalanceOf<T> },
        QuestCompleted { quest_id: T::QuestId, user: T::AccountId, reward: BalanceOf<T> },
    }

    // --- Errors ---
    #[pallet::error]
    pub enum Error<T> {
        QuestNotFound,
        QuestAlreadyCompleted,
        QuestIdOverflow,
        DescriptionTooLong,
        TooManyRequirements,
        /// An error was returned from the requirement verifier in the runtime.
        VerificationFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new quest with a set of requirements.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_quest(
            origin: OriginFor<T>,
            description: BoundedVec<u8, T::MaxDescriptionLength>,
            reward: BalanceOf<T>,
            requirements: BoundedVec<QuestRequirement<ItemId>, T::MaxRequirementsPerQuest>,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;

            let quest_id = NextQuestId::<T>::try_mutate(|id| -> Result<T::QuestId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(&(1u32.into())).ok_or(Error::<T>::QuestIdOverflow)?;
                Ok(current_id)
            })?;

            let new_quest = Quest { creator: creator.clone(), description, reward, requirements };

            AvailableQuests::<T>::insert(quest_id, new_quest);
            Self::deposit_event(Event::QuestCreated { quest_id, creator, reward });
            Ok(())
        }

        /// Allow a user to complete a quest if all requirements are met.
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn complete_quest(
            origin: OriginFor<T>,
            quest_id: T::QuestId,
            // The user provides context, like which pet to use for a check.
            context: VerificationContext<PetId>,
        ) -> DispatchResult {
            let user = ensure_signed(origin)?;
            let quest = AvailableQuests::<T>::get(&quest_id).ok_or(Error::<T>::QuestNotFound)?;
            ensure!(!CompletedQuests::<T>::contains_key((&user, &quest_id)), Error::<T>::QuestAlreadyCompleted);

            // --- (S) Systematic Verification ---
            // The pallet iterates through the requirements and dispatches them to the
            // runtime-configured verifier. It doesn't need to know the details.
            for requirement in quest.requirements {
                T::Verifier::verify(&requirement, &user, &context)
                    .map_err(|_| Error::<T>::VerificationFailed)?;
            }

            // Distribute reward
            if quest.reward > 0u32.into() {
                T::Currency::deposit_creating(&user, quest.reward);
            }

            // Mark as completed
            CompletedQuests::<T>::insert((user.clone(), quest_id), ());

            // SYNERGY: Trigger a score update in the profile pallet.
            // T::ProfileHandler::trigger_score_update(&user)?; // Assumes a handler in Config

            Self::deposit_event(Event::QuestCompleted { quest_id, user, reward: quest.reward });
            Ok(())
        }
    }
}
