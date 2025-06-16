#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// --- Conceptual Handler Traits (Simplified for MVP) ---

// From pallet-critter-nfts
pub trait QuestNftRequirementChecker<AccountId, PetId> { // Removed PetSpeciesType for MVP
    fn get_pet_owner(pet_id: &PetId) -> Option<AccountId>;
    fn get_pet_level(pet_id: &PetId) -> Option<u32>;
    // fn get_pet_species(pet_id: &PetId) -> Option<PetSpeciesType>; // Deferred for MVP
}

// From pallet-items
pub trait QuestItemRequirementChecker<AccountId, ItemId> {
    fn check_and_consume_item(
        user: &AccountId,
        item_id: &ItemId,
        quantity: u32,
        consume: bool, // New parameter to control consumption based on quest.consume_item_on_completion
    ) -> frame_support::dispatch::DispatchResult;
}

// From pallet-user-profile
pub trait QuestUserProfileRequirementChecker<AccountId> { // Removed ScoreValue for MVP as only u32 is used
    fn get_battles_won(user: &AccountId) -> Option<u32>;
    // fn get_trade_reputation(user: &AccountId) -> Option<i32>; // Deferred for MVP
}
// --- End of Conceptual Handler Traits ---


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::Currency,
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_std::vec::Vec;

    // Import conceptual traits defined above at crate level
    use super::{QuestNftRequirementChecker, QuestItemRequirementChecker, QuestUserProfileRequirementChecker};

    // Type Aliases (assuming these are defined or made available appropriately)
    pub type PetId = u32;
    pub type ItemId = u32;
    // PetSpeciesType removed for MVP from local alias
    // ScoreValue removed for MVP from local alias

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct Quest<Balance> {
        pub description: Vec<u8>, // Consider BoundedVec<u8, T::MaxDescriptionLength>
        pub reward_ptcn: Balance,
        // Simplified MVP Criteria (all Optional)
        pub required_pet_level: Option<u32>, // User must specify which pet meets this when completing
        pub required_item_id: Option<ItemId>,
        pub required_item_quantity: Option<u32>,
        pub consume_item_on_completion: bool,
        pub min_battles_won_for_user: Option<u32>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type QuestId: Parameter + Member + Copy + MaybeSerializeDeserialize + MaxEncodedLen + Default + Ord + From<u32>;

        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;

        // Handlers for interacting with other pallets (Simplified for MVP)
        type NftChecker: QuestNftRequirementChecker<Self::AccountId, PetId>; // PetSpeciesType removed
        type ItemChecker: QuestItemRequirementChecker<Self::AccountId, ItemId>;
        type UserProfileChecker: QuestUserProfileRequirementChecker<Self::AccountId>; // ScoreValue removed
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_quest_id)]
    pub(super) type NextQuestId<T: Config> = StorageValue<_, T::QuestId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn available_quests)]
    pub(super) type AvailableQuests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::QuestId,
        Quest<BalanceOf<T>>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn completed_quests)]
    /// Stores (AccountId, QuestId) to signify a quest is completed by an account.
    pub(super) type CompletedQuests<T: Config> = StorageMap<
        _,
        Twox64Concat, // Twox64Concat is efficient for composite keys like tuples
        (T::AccountId, T::QuestId),
        (), // Value is just a marker to indicate presence
        OptionQuery, // Use OptionQuery to easily check for key existence (Some(()) vs None)
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        QuestAdded { quest_id: T::QuestId, description: Vec<u8>, reward: BalanceOf<T> },
        QuestCompleted { quest_id: T::QuestId, account: T::AccountId, reward: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        QuestNotFound,
        QuestAlreadyCompleted,
        RewardDistributionFailed, // Placeholder for currency interaction issues
        QuestIdOverflow,
        DescriptionTooLong,
        QuestCriteriaRequiresPetSelection,
        CriteriaPetNotFound,
        CriteriaPetNotOwned,
        PetLevelTooLow,
        // IncorrectPetSpecies, // Deferred for MVP
        RequiredItemNotFoundOrInsufficient,
        UserProfileDataUnavailable,
        NotEnoughBattlesWon,
        // TradeReputationTooLow, // Deferred for MVP
        QuestPrerequisitesNotMet,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        // Tuples of (description, reward_ptcn)
        pub initial_quests: Vec<(Vec<u8>, BalanceOf<T>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { initial_quests: Vec::new() }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let mut next_id_counter: u32 = 0;
            for (description, reward) in &self.initial_quests {
                let quest_id: T::QuestId = next_id_counter.into();
                let quest = Quest {
                    description: description.clone(),
                    reward_ptcn: *reward,
                };
                AvailableQuests::<T>::insert(quest_id, quest);
                next_id_counter = next_id_counter.checked_add(1).expect("Max quests reached during genesis");
            }
            NextQuestId::<T>::put(next_id_counter.into());
        }
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new quest.
        /// Callable by an authorized origin (e.g., Root or a configured AdminOrigin).
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2))] // Placeholder weight
        pub fn add_quest(
            origin: OriginFor<T>,
            description: Vec<u8>,
            reward_ptcn: BalanceOf<T>,
            // Simplified MVP criteria
            required_pet_level: Option<u32>,
            required_item_id: Option<ItemId>,
            required_item_quantity: Option<u32>,
            consume_item_on_completion: Option<bool>, // True if item should be consumed, false if only checked
            min_battles_won_for_user: Option<u32>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(description.len() <= T::MaxDescriptionLength::get() as usize, Error::<T>::DescriptionTooLong);

            let quest_id = NextQuestId::<T>::try_mutate(|id| -> Result<T::QuestId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(&(1u32.into())).ok_or(Error::<T>::QuestIdOverflow)?;
                Ok(current_id)
            })?;

            let new_quest = Quest {
                description: description.clone(),
                reward_ptcn,
                required_pet_level,
                required_item_id,
                required_item_quantity,
                consume_item_on_completion: consume_item_on_completion.unwrap_or(required_item_id.is_some()), // Default to true if item is required
                min_battles_won_for_user,
            };

            AvailableQuests::<T>::insert(quest_id, new_quest);
            Self::deposit_event(Event::QuestAdded { quest_id, description, reward: reward_ptcn });
            Ok(().into())
        }

        /// Allow a user to claim completion of a quest and receive a reward.
        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2) + T::DbWeight::get().reads(1))] // R:AvailableQuests, R:CompletedQuests, W:Currency, W:CompletedQuests
        pub fn complete_quest(
            origin: OriginFor<T>,
            quest_id: T::QuestId,
            maybe_target_pet_id: Option<PetId>,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            let quest = AvailableQuests::<T>::get(&quest_id).ok_or(Error::<T>::QuestNotFound)?;
            ensure!(!CompletedQuests::<T>::contains_key((&account, &quest_id)), Error::<T>::QuestAlreadyCompleted);

            // --- Simplified MVP Criteria Verification ---
            if let Some(req_level) = quest.required_pet_level {
                let pet_id_to_check = maybe_target_pet_id.ok_or(Error::<T>::QuestCriteriaRequiresPetSelection)?;
                ensure!(T::NftChecker::get_pet_owner(&pet_id_to_check) == Some(account.clone()), Error::<T>::CriteriaPetNotOwned);
                let pet_level = T::NftChecker::get_pet_level(&pet_id_to_check).ok_or(Error::<T>::CriteriaPetNotFound)?;
                ensure!(pet_level >= req_level, Error::<T>::PetLevelTooLow);
            }

            // Pet Species Check Deferred for MVP

            if let (Some(item_id), Some(req_quantity)) = (quest.required_item_id, quest.required_item_quantity) {
                if req_quantity > 0 {
                    // Pass the consume_item_on_completion flag from the quest struct to the ItemChecker method.
                    T::ItemChecker::check_and_consume_item(&account, &item_id, req_quantity, quest.consume_item_on_completion)
                        .map_err(|_| Error::<T>::RequiredItemNotFoundOrInsufficient)?;
                }
            }

            if let Some(req_battles_won) = quest.min_battles_won_for_user {
               let battles_won = T::UserProfileChecker::get_battles_won(&account)
                                     .ok_or(Error::<T>::UserProfileDataUnavailable)?; // Ensure UserProfileChecker handles Option correctly
               ensure!(battles_won >= req_battles_won, Error::<T>::NotEnoughBattlesWon);
            }

            // Trade Reputation Check Deferred for MVP
            // --- End of Criteria Verification ---

            // Distribute reward
            if quest.reward_ptcn > BalanceOf::<T>::from(0u32) {
                // Using deposit_creating with same caveats as daily claim / battle rewards.
                // Assumes this pallet has a way to source/mint these funds.
                T::Currency::deposit_creating(&account, quest.reward_ptcn);
                // Note: A more robust implementation would handle potential errors from deposit_creating,
                // or use a transfer from a pallet sovereign account, or use an Imbalance.
            }

            // Mark as completed for this account and quest
            CompletedQuests::<T>::insert((account.clone(), quest_id), ());

            // SYNERGY: Call pallet-user-profile to record quest completion for scoring
            // pallet_user_profile::Pallet::<T>::record_quest_completion(&account)?; // Requires T: pallet_user_profile::Config

            Self::deposit_event(Event::QuestCompleted { quest_id, account, reward: quest.reward_ptcn });
            Ok(())
        }
    }
}
