#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// --- Conceptual Handler Traits (What pallet-quests needs from other pallets) ---
// These would typically be defined in the respective pallets (critter_nfts, items, user-profile)
// and then pallet-quests would depend on those pallets via its Config trait.
// For this conceptual outline, we define them here to show what pallet-quests expects.

// From pallet-critter-nfts (subset of NftManager or a new specific trait)
pub trait QuestNftRequirementChecker<AccountId, PetId, PetSpeciesType> {
    fn get_pet_owner(pet_id: &PetId) -> Option<AccountId>;
    fn get_pet_level(pet_id: &PetId) -> Option<u32>;
    fn get_pet_species(pet_id: &PetId) -> Option<PetSpeciesType>; // PetSpeciesType is Vec<u8>
}

// From pallet-items (conceptual)
pub trait QuestItemRequirementChecker<AccountId, ItemId> {
    fn check_and_consume_item(
        user: &AccountId,
        item_id: &ItemId,
        quantity: u32,
    ) -> frame_support::dispatch::DispatchResult;
}

// From pallet-user-profile (conceptual)
pub trait QuestUserProfileRequirementChecker<AccountId, ScoreValue> {
    fn get_battles_won(user: &AccountId) -> Option<u32>;
    fn get_trade_reputation(user: &AccountId) -> Option<i32>;
    // Add other specific score getters if needed by quests
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
    pub type PetSpeciesType = Vec<u8>;
    pub type ScoreValue = u64; // Placeholder

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct Quest<Balance> {
        pub description: Vec<u8>,
        pub reward_ptcn: Balance,
        // New fields for advanced criteria (all Optional)
        pub required_pet_level: Option<u32>,
        pub required_pet_id_for_level_check: Option<PetId>,
        pub required_pet_species: Option<PetSpeciesType>,
        pub required_pet_id_for_species_check: Option<PetId>,
        pub required_item_id: Option<ItemId>,
        pub required_item_quantity: Option<u32>,
        pub consume_item_on_completion: bool,
        pub min_battles_won_for_user: Option<u32>,
        pub min_trade_reputation_for_user: Option<i32>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type QuestId: Parameter + Member + Copy + MaybeSerializeDeserialize + MaxEncodedLen + Default + Ord + From<u32>;

        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;

        // Handlers for interacting with other pallets
        type NftChecker: QuestNftRequirementChecker<Self::AccountId, PetId, PetSpeciesType>;
        type ItemChecker: QuestItemRequirementChecker<Self::AccountId, ItemId>;
        type UserProfileChecker: QuestUserProfileRequirementChecker<Self::AccountId, ScoreValue>;
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
        // SYNERGY & ADVANCED CRITERIA: New Error variants
        QuestCriteriaRequiresPetSelection, // If user needs to specify which pet meets criteria
        CriteriaPetNotFound,        // If specified PetID for criteria check doesn't exist
        CriteriaPetNotOwned,        // If user doesn't own the specified PetID
        PetLevelTooLow,
        IncorrectPetSpecies,
        RequiredItemNotFoundOrInsufficient, // Covers both missing item or not enough quantity
        UserProfileDataUnavailable,     // If profile data can't be fetched
        NotEnoughBattlesWon,
        TradeReputationTooLow,
        QuestPrerequisitesNotMet,     // Generic for other types of checks
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
            // New optional parameters for criteria
            required_pet_level: Option<u32>,
            required_pet_id_for_level_check: Option<PetId>,
            required_pet_species: Option<PetSpeciesType>,
            required_pet_id_for_species_check: Option<PetId>,
            required_item_id: Option<ItemId>,
            required_item_quantity: Option<u32>,
            consume_item_on_completion: Option<bool>,
            min_battles_won_for_user: Option<u32>,
            min_trade_reputation_for_user: Option<i32>,
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
                required_pet_id_for_level_check,
                required_pet_species,
                required_pet_id_for_species_check,
                required_item_id,
                required_item_quantity,
                consume_item_on_completion: consume_item_on_completion.unwrap_or(if required_item_id.is_some() { true } else { false }),
                min_battles_won_for_user,
                min_trade_reputation_for_user,
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

            // --- Advanced Criteria Verification (Conceptual Logic) ---
            if let Some(req_level) = quest.required_pet_level {
                let pet_id_to_check = quest.required_pet_id_for_level_check.or(maybe_target_pet_id)
                                         .ok_or(Error::<T>::QuestCriteriaRequiresPetSelection)?;
                ensure!(T::NftChecker::get_pet_owner(&pet_id_to_check) == Some(account.clone()), Error::<T>::CriteriaPetNotOwned);
                let pet_level = T::NftChecker::get_pet_level(&pet_id_to_check).ok_or(Error::<T>::CriteriaPetNotFound)?;
                ensure!(pet_level >= req_level, Error::<T>::PetLevelTooLow);
            }

            if let Some(ref req_species) = quest.required_pet_species {
                let pet_id_to_check = quest.required_pet_id_for_species_check.or(maybe_target_pet_id)
                                         .ok_or(Error::<T>::QuestCriteriaRequiresPetSelection)?;
                ensure!(T::NftChecker::get_pet_owner(&pet_id_to_check) == Some(account.clone()), Error::<T>::CriteriaPetNotOwned);
                let pet_species = T::NftChecker::get_pet_species(&pet_id_to_check).ok_or(Error::<T>::CriteriaPetNotFound)?;
                ensure!(pet_species == *req_species, Error::<T>::IncorrectPetSpecies);
            }

            if let (Some(item_id), Some(req_quantity)) = (quest.required_item_id, quest.required_item_quantity) {
                if req_quantity > 0 {
                    T::ItemChecker::check_and_consume_item(&account, &item_id, req_quantity)
                        .map_err(|_| Error::<T>::RequiredItemNotFoundOrInsufficient)?;
                        // Assumes check_and_consume_item correctly uses consume_item_on_completion from quest struct
                        // or the flag is passed to it if it's a separate parameter in the trait method.
                        // If consume_item_on_completion is true, this method consumes. If false, it only checks.
                        // For simplicity, assume check_and_consume_item handles the consume_item_on_completion logic.
                }
            }

            if let Some(req_battles_won) = quest.min_battles_won_for_user {
               let battles_won = T::UserProfileChecker::get_battles_won(&account)
                                     .ok_or(Error::<T>::UserProfileDataUnavailable)?;
               ensure!(battles_won >= req_battles_won, Error::<T>::NotEnoughBattlesWon);
            }

            if let Some(req_trade_rep) = quest.min_trade_reputation_for_user {
                let trade_rep = T::UserProfileChecker::get_trade_reputation(&account)
                                      .ok_or(Error::<T>::UserProfileDataUnavailable)?;
                ensure!(trade_rep >= req_trade_rep, Error::<T>::TradeReputationTooLow);
            }
            // --- End of Criteria Verification ---

            // Distribute reward (simplified, as in other pallets)
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
