#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

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

    // BalanceOf type alias needs to be accessible by the Config trait for QuestRewardAmount etc.
    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct Quest<Balance> {
        pub description: Vec<u8>, // Consider BoundedVec<u8, T::MaxDescriptionLength> for determinable weight
        pub reward_ptcn: Balance,
        // SYNERGY: Fields for prerequisites
        // pub min_user_progress_score: Option<u64 /*ScoreValue from pallet-user-profile*/>,
        // pub required_pet_charter_attribute: Option<(u32 /*PetAttributeType enum index or ID*/, u8 /*min_value*/)>,
        // pub required_pet_id_for_completion: Option<u32 /*PetId*/>, // If a specific pet must be "used" or present
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type QuestId: Parameter + Member + Copy + MaybeSerializeDeserialize + MaxEncodedLen + Default + Ord + From<u32>;
        // type AdminOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>; // For future admin roles

        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>; // Example: For BoundedVec if description length is restricted
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
        DescriptionTooLong, // Example error if description length was restricted
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
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?; // For MVP, only Root can add quests. Could be T::AdminOrigin later.

            // Example of how one might check description length if T::MaxDescriptionLength was enforced
            // ensure!(description.len() <= T::MaxDescriptionLength::get() as usize, Error::<T>::DescriptionTooLong);

            let quest_id = NextQuestId::<T>::try_mutate(|id| -> Result<T::QuestId, DispatchError> {
                let current_id = *id;
                // Use .into() to ensure type compatibility for QuestId if it's not u32 directly
                *id = id.checked_add(&(1u32.into())).ok_or(Error::<T>::QuestIdOverflow)?;
                Ok(current_id)
            })?;

            let new_quest = Quest {
                description: description.clone(), // Store a copy of the description
                reward_ptcn,
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
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;

            // Check if quest exists
            let quest = AvailableQuests::<T>::get(&quest_id).ok_or(Error::<T>::QuestNotFound)?;

            // SYNERGY: Check UserProfile scores or PetNft attributes for quest eligibility
            // This check would ideally be in `accept_quest` or a helper `can_accept_quest`
            // For `complete_quest`, we assume eligibility was met to accept it.
            // Example conceptual checks (would require T::Config to provide access to other pallets or traits):
            // if let Some(min_score) = quest.min_user_progress_score {
            //     // let user_profile = pallet_user_profile::Pallet::<T>::user_profiles(&account);
            //     // ensure!(user_profile.overall_progress_score >= min_score, Error::<T>::UserScoreTooLow);
            // }
            // if let Some((attr_type, min_val)) = quest.required_pet_charter_attribute {
            //     // let pet_details = T::NftHandler::get_pet_details(&quest.required_pet_id_for_completion.unwrap_or_default());
            //     // match attr_type { /* check specific base stat */ }
            //     // ensure!(pet_stat >= min_val, Error::<T>::PetAttributeTooLow);
            // }


            // Check if already completed by this account
            ensure!(!CompletedQuests::<T>::contains_key((&account, &quest_id)), Error::<T>::QuestAlreadyCompleted);

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

            Self::deposit_event(Event::QuestCompleted { quest_id, account, reward: quest.reward_ptcn });
            Ok(())
        }
    }
}
