//! # Critter Pet Status Pallet
//!
//! This pallet manages pet status, conditions, and state changes
//! for the CritterCraft ecosystem.
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
        traits::{Currency, Randomness}, // Currency for balances
        BoundedVec, // For bounded collections, crucial for security
    };
    use frame_system::{
        pallet_prelude::*, // Provides types like BlockNumberFor, AccountId, OriginFor
        ensure_signed,     // Macro to ensure origin is a signed account
    };
    use sp_std::vec::Vec; // Standard Vec for dynamic arrays (used where not bounded)
    use scale_info::TypeInfo; // For `TypeInfo` derive macro
    use frame_support::log; // Correct way to import Substrate's logging macro
    use sp_runtime::traits::StaticLookup; // For AccountIdLookup

    // --- Type Aliases ---
    pub type PetId = u32; // Unique identifier for each pet
    pub type ConditionId = u32; // Unique identifier for each condition
    pub type StatValue = u8; // Value for pet stats (0-100)

    // --- Enum Definitions ---
    // PetMood: Defines the current mood of a pet
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum PetMood {
        Happy,
        Content,
        Neutral,
        Sad,
        Distressed,
    }

    // ConditionType: Defines the type of condition
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum ConditionType {
        Positive, // Buffs
        Negative, // Debuffs
        Neutral,  // Special states
    }

    // ConditionSeverity: Defines the severity of a condition
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum ConditionSeverity {
        Minor,
        Moderate,
        Major,
        Severe,
    }

    // StatType: Defines the different pet stats
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum StatType {
        Strength,
        Agility,
        Intelligence,
        Vitality,
        Charisma,
    }

    // NeedType: Defines the different pet needs
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum NeedType {
        Hunger,
        Energy,
        Happiness,
        Hygiene,
        Social,
    }

    // --- Struct Definitions ---
    // PetStatus: Defines the current status of a pet
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct PetStatus<T: Config> {
        pub pet_id: PetId,
        pub owner: T::AccountId,
        pub mood: PetMood,
        pub last_interaction: BlockNumberFor<T>,
        pub last_fed: BlockNumberFor<T>,
        pub last_rested: BlockNumberFor<T>,
        pub last_played: BlockNumberFor<T>,
        pub last_groomed: BlockNumberFor<T>,
        pub last_socialized: BlockNumberFor<T>,
    }

    // PetStats: Defines the stats of a pet
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct PetStats {
        pub strength: StatValue,
        pub agility: StatValue,
        pub intelligence: StatValue,
        pub vitality: StatValue,
        pub charisma: StatValue,
    }

    // PetNeeds: Defines the needs of a pet
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct PetNeeds {
        pub hunger: StatValue,
        pub energy: StatValue,
        pub happiness: StatValue,
        pub hygiene: StatValue,
        pub social: StatValue,
    }

    // Condition: Defines a condition that can affect a pet
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Condition<T: Config> {
        pub id: ConditionId,
        pub name: BoundedVec<u8, T::MaxConditionNameLen>,
        pub description: BoundedVec<u8, T::MaxConditionDescLen>,
        pub condition_type: ConditionType,
        pub severity: ConditionSeverity,
        pub duration_blocks: BlockNumberFor<T>,
        pub stat_modifiers: Vec<(StatType, i8)>, // (StatType, modifier value)
        pub need_modifiers: Vec<(NeedType, i8)>, // (NeedType, modifier value)
    }

    // PetCondition: Tracks a condition affecting a pet
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct PetCondition<T: Config> {
        pub pet_id: PetId,
        pub condition_id: ConditionId,
        pub started_at_block: BlockNumberFor<T>,
        pub expires_at_block: BlockNumberFor<T>,
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
        
        /// The randomness trait for generating random events.
        type PetRandomness: Randomness<Self::Hash, Self::BlockNumber>;
        
        /// Maximum length of a condition name (in bytes).
        #[pallet::constant]
        type MaxConditionNameLen: Get<u32>;
        
        /// Maximum length of a condition description (in bytes).
        #[pallet::constant]
        type MaxConditionDescLen: Get<u32>;
        
        /// Maximum number of conditions a pet can have simultaneously.
        #[pallet::constant]
        type MaxPetConditions: Get<u32>;
        
        /// Blocks between automatic need decay.
        #[pallet::constant]
        type NeedDecayInterval: Get<Self::BlockNumber>;
        
        /// Amount of need decay per interval.
        #[pallet::constant]
        type NeedDecayAmount: Get<StatValue>;
        
        /// Blocks before a pet becomes hungry.
        #[pallet::constant]
        type HungerInterval: Get<Self::BlockNumber>;
        
        /// Blocks before a pet becomes tired.
        #[pallet::constant]
        type TirednessInterval: Get<Self::BlockNumber>;
        
        /// Blocks before a pet becomes unhappy.
        #[pallet::constant]
        type UnhappinessInterval: Get<Self::BlockNumber>;
        
        /// Blocks before a pet becomes dirty.
        #[pallet::constant]
        type DirtinessInterval: Get<Self::BlockNumber>;
        
        /// Blocks before a pet becomes lonely.
        #[pallet::constant]
        type LonelinessInterval: Get<Self::BlockNumber>;
    }

    // --- Pallet Definition ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // --- Pallet Storage Items ---
    #[pallet::storage]
    #[pallet::getter(fn pet_status)]
    /// Stores the comprehensive PetStatus data for each PetId.
    pub(super) type PetStatuses<T: Config> = StorageMap<_, Blake2_128Concat, PetId, PetStatus<T>>;

    #[pallet::storage]
    #[pallet::getter(fn pet_stats)]
    /// Stores the PetStats data for each PetId.
    pub(super) type PetStatsStorage<T: Config> = StorageMap<_, Blake2_128Concat, PetId, PetStats>;

    #[pallet::storage]
    #[pallet::getter(fn pet_needs)]
    /// Stores the PetNeeds data for each PetId.
    pub(super) type PetNeedsStorage<T: Config> = StorageMap<_, Blake2_128Concat, PetId, PetNeeds>;

    #[pallet::storage]
    #[pallet::getter(fn conditions)]
    /// Stores the comprehensive Condition data for each ConditionId.
    pub(super) type Conditions<T: Config> = StorageMap<_, Blake2_128Concat, ConditionId, Condition<T>>;

    #[pallet::storage]
    #[pallet::getter(fn next_condition_id)]
    /// Stores the next available unique ConditionId.
    pub(super) type NextConditionId<T: Config> = StorageValue<_, ConditionId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_conditions)]
    /// Stores the conditions affecting each pet.
    pub(super) type PetConditions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<PetCondition<T>, T::MaxPetConditions>,
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn last_need_decay)]
    /// Stores the last block number when needs were decayed.
    pub(super) type LastNeedDecay<T: Config> = StorageMap<_, Blake2_128Concat, PetId, BlockNumberFor<T>>;

    // --- Pallet Events ---
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A pet's status has been initialized. [pet_id, owner]
        PetStatusInitialized { pet_id: PetId, owner: T::AccountId },
        
        /// A pet's mood has changed. [pet_id, mood]
        PetMoodChanged { pet_id: PetId, mood: PetMood },
        
        /// A pet has been fed. [pet_id, hunger_restored]
        PetFed { pet_id: PetId, hunger_restored: StatValue },
        
        /// A pet has rested. [pet_id, energy_restored]
        PetRested { pet_id: PetId, energy_restored: StatValue },
        
        /// A pet has played. [pet_id, happiness_increased]
        PetPlayed { pet_id: PetId, happiness_increased: StatValue },
        
        /// A pet has been groomed. [pet_id, hygiene_increased]
        PetGroomed { pet_id: PetId, hygiene_increased: StatValue },
        
        /// A pet has socialized. [pet_id, social_increased]
        PetSocialized { pet_id: PetId, social_increased: StatValue },
        
        /// A pet's stats have changed. [pet_id, stat_type, old_value, new_value]
        PetStatChanged { pet_id: PetId, stat_type: StatType, old_value: StatValue, new_value: StatValue },
        
        /// A pet's needs have changed. [pet_id, need_type, old_value, new_value]
        PetNeedChanged { pet_id: PetId, need_type: NeedType, old_value: StatValue, new_value: StatValue },
        
        /// A pet has developed a condition. [pet_id, condition_id, name]
        PetDevelopedCondition { pet_id: PetId, condition_id: ConditionId, name: Vec<u8> },
        
        /// A pet has recovered from a condition. [pet_id, condition_id, name]
        PetRecoveredFromCondition { pet_id: PetId, condition_id: ConditionId, name: Vec<u8> },
        
        /// A pet's needs have decayed. [pet_id]
        PetNeedsDecayed { pet_id: PetId },
    }

    // --- Pallet Errors ---
    #[pallet::error]
    pub enum Error<T> {
        /// The pet status does not exist.
        PetStatusDoesNotExist,
        
        /// The pet stats do not exist.
        PetStatsDoNotExist,
        
        /// The pet needs do not exist.
        PetNeedsDoNotExist,
        
        /// The condition does not exist.
        ConditionDoesNotExist,
        
        /// The pet already has this condition.
        PetAlreadyHasCondition,
        
        /// The pet does not have this condition.
        PetDoesNotHaveCondition,
        
        /// The pet has reached the maximum number of conditions.
        MaxPetConditionsReached,
        
        /// The pet is not owned by the sender.
        PetNotOwnedBySender,
        
        /// The next ID has overflowed.
        NextIdOverflow,
        
        /// The pet is too hungry to perform this action.
        PetTooHungry,
        
        /// The pet is too tired to perform this action.
        PetTooTired,
        
        /// The pet is too unhappy to perform this action.
        PetTooUnhappy,
        
        /// The pet is too dirty to perform this action.
        PetTooDirty,
        
        /// The pet is too lonely to perform this action.
        PetTooLonely,
        
        /// The pet was fed too recently.
        PetFedTooRecently,
        
        /// The pet rested too recently.
        PetRestedTooRecently,
        
        /// The pet played too recently.
        PetPlayedTooRecently,
        
        /// The pet was groomed too recently.
        PetGroomedTooRecently,
        
        /// The pet socialized too recently.
        PetSocializedTooRecently,
    }

    // --- Pallet Hooks ---
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            // Process need decay and condition updates
            Self::process_pet_updates(n);
            Weight::zero()
        }
    }

    // --- Pallet Extrinsics ---
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initialize a pet's status.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn initialize_pet_status(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Check if the pet status already exists.
            ensure!(!PetStatuses::<T>::contains_key(pet_id), Error::<T>::PetStatusDoesNotExist);
            
            // 2. Create the pet status.
            let current_block = frame_system::Pallet::<T>::block_number();
            let pet_status = PetStatus::<T> {
                pet_id,
                owner: owner.clone(),
                mood: PetMood::Happy,
                last_interaction: current_block,
                last_fed: current_block,
                last_rested: current_block,
                last_played: current_block,
                last_groomed: current_block,
                last_socialized: current_block,
            };
            
            // 3. Create the pet stats.
            let pet_stats = PetStats {
                strength: 10,
                agility: 10,
                intelligence: 10,
                vitality: 10,
                charisma: 10,
            };
            
            // 4. Create the pet needs.
            let pet_needs = PetNeeds {
                hunger: 100,
                energy: 100,
                happiness: 100,
                hygiene: 100,
                social: 100,
            };
            
            // 5. Store the pet status, stats, and needs.
            PetStatuses::<T>::insert(pet_id, pet_status);
            PetStatsStorage::<T>::insert(pet_id, pet_stats);
            PetNeedsStorage::<T>::insert(pet_id, pet_needs);
            LastNeedDecay::<T>::insert(pet_id, current_block);
            
            // 6. Emit the event.
            Self::deposit_event(Event::PetStatusInitialized {
                pet_id,
                owner,
            });
            
            Ok(())
        }

        /// Feed a pet.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn feed_pet(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // 1. Check if the pet status exists.
            let mut pet_status = PetStatuses::<T>::get(pet_id).ok_or(Error::<T>::PetStatusDoesNotExist)?;
            
            // 2. Check if the sender is the owner of the pet.
            ensure!(pet_status.owner == sender, Error::<T>::PetNotOwnedBySender);
            
            // 3. Check if the pet was fed too recently.
            let current_block = frame_system::Pallet::<T>::block_number();
            let blocks_since_last_fed = current_block.saturating_sub(pet_status.last_fed);
            ensure!(blocks_since_last_fed >= 10u32.into(), Error::<T>::PetFedTooRecently);
            
            // 4. Update the pet's hunger.
            let mut pet_needs = PetNeedsStorage::<T>::get(pet_id).ok_or(Error::<T>::PetNeedsDoNotExist)?;
            let old_hunger = pet_needs.hunger;
            let hunger_restored = 30;
            pet_needs.hunger = (pet_needs.hunger.saturating_add(hunger_restored)).min(100);
            
            // 5. Update the pet's last fed time.
            pet_status.last_fed = current_block;
            pet_status.last_interaction = current_block;
            
            // 6. Update the pet's mood based on needs.
            Self::update_pet_mood(&mut pet_status, &pet_needs);
            
            // 7. Store the updated pet status and needs.
            PetStatuses::<T>::insert(pet_id, pet_status.clone());
            PetNeedsStorage::<T>::insert(pet_id, pet_needs.clone());
            
            // 8. Emit the events.
            Self::deposit_event(Event::PetFed {
                pet_id,
                hunger_restored,
            });
            
            Self::deposit_event(Event::PetNeedChanged {
                pet_id,
                need_type: NeedType::Hunger,
                old_value: old_hunger,
                new_value: pet_needs.hunger,
            });
            
            Self::deposit_event(Event::PetMoodChanged {
                pet_id,
                mood: pet_status.mood,
            });
            
            Ok(())
        }

        /// Rest a pet.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn rest_pet(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // 1. Check if the pet status exists.
            let mut pet_status = PetStatuses::<T>::get(pet_id).ok_or(Error::<T>::PetStatusDoesNotExist)?;
            
            // 2. Check if the sender is the owner of the pet.
            ensure!(pet_status.owner == sender, Error::<T>::PetNotOwnedBySender);
            
            // 3. Check if the pet rested too recently.
            let current_block = frame_system::Pallet::<T>::block_number();
            let blocks_since_last_rested = current_block.saturating_sub(pet_status.last_rested);
            ensure!(blocks_since_last_rested >= 20u32.into(), Error::<T>::PetRestedTooRecently);
            
            // 4. Update the pet's energy.
            let mut pet_needs = PetNeedsStorage::<T>::get(pet_id).ok_or(Error::<T>::PetNeedsDoNotExist)?;
            let old_energy = pet_needs.energy;
            let energy_restored = 40;
            pet_needs.energy = (pet_needs.energy.saturating_add(energy_restored)).min(100);
            
            // 5. Update the pet's last rested time.
            pet_status.last_rested = current_block;
            pet_status.last_interaction = current_block;
            
            // 6. Update the pet's mood based on needs.
            Self::update_pet_mood(&mut pet_status, &pet_needs);
            
            // 7. Store the updated pet status and needs.
            PetStatuses::<T>::insert(pet_id, pet_status.clone());
            PetNeedsStorage::<T>::insert(pet_id, pet_needs.clone());
            
            // 8. Emit the events.
            Self::deposit_event(Event::PetRested {
                pet_id,
                energy_restored,
            });
            
            Self::deposit_event(Event::PetNeedChanged {
                pet_id,
                need_type: NeedType::Energy,
                old_value: old_energy,
                new_value: pet_needs.energy,
            });
            
            Self::deposit_event(Event::PetMoodChanged {
                pet_id,
                mood: pet_status.mood,
            });
            
            Ok(())
        }

        /// Play with a pet.
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn play_with_pet(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // 1. Check if the pet status exists.
            let mut pet_status = PetStatuses::<T>::get(pet_id).ok_or(Error::<T>::PetStatusDoesNotExist)?;
            
            // 2. Check if the sender is the owner of the pet.
            ensure!(pet_status.owner == sender, Error::<T>::PetNotOwnedBySender);
            
            // 3. Check if the pet played too recently.
            let current_block = frame_system::Pallet::<T>::block_number();
            let blocks_since_last_played = current_block.saturating_sub(pet_status.last_played);
            ensure!(blocks_since_last_played >= 15u32.into(), Error::<T>::PetPlayedTooRecently);
            
            // 4. Check if the pet is too tired.
            let mut pet_needs = PetNeedsStorage::<T>::get(pet_id).ok_or(Error::<T>::PetNeedsDoNotExist)?;
            ensure!(pet_needs.energy >= 20, Error::<T>::PetTooTired);
            
            // 5. Update the pet's happiness and energy.
            let old_happiness = pet_needs.happiness;
            let happiness_increased = 25;
            pet_needs.happiness = (pet_needs.happiness.saturating_add(happiness_increased)).min(100);
            pet_needs.energy = pet_needs.energy.saturating_sub(10);
            
            // 6. Update the pet's last played time.
            pet_status.last_played = current_block;
            pet_status.last_interaction = current_block;
            
            // 7. Update the pet's mood based on needs.
            Self::update_pet_mood(&mut pet_status, &pet_needs);
            
            // 8. Store the updated pet status and needs.
            PetStatuses::<T>::insert(pet_id, pet_status.clone());
            PetNeedsStorage::<T>::insert(pet_id, pet_needs.clone());
            
            // 9. Emit the events.
            Self::deposit_event(Event::PetPlayed {
                pet_id,
                happiness_increased,
            });
            
            Self::deposit_event(Event::PetNeedChanged {
                pet_id,
                need_type: NeedType::Happiness,
                old_value: old_happiness,
                new_value: pet_needs.happiness,
            });
            
            Self::deposit_event(Event::PetNeedChanged {
                pet_id,
                need_type: NeedType::Energy,
                old_value: old_happiness,
                new_value: pet_needs.energy,
            });
            
            Self::deposit_event(Event::PetMoodChanged {
                pet_id,
                mood: pet_status.mood,
            });
            
            Ok(())
        }

        /// Groom a pet.
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn groom_pet(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // 1. Check if the pet status exists.
            let mut pet_status = PetStatuses::<T>::get(pet_id).ok_or(Error::<T>::PetStatusDoesNotExist)?;
            
            // 2. Check if the sender is the owner of the pet.
            ensure!(pet_status.owner == sender, Error::<T>::PetNotOwnedBySender);
            
            // 3. Check if the pet was groomed too recently.
            let current_block = frame_system::Pallet::<T>::block_number();
            let blocks_since_last_groomed = current_block.saturating_sub(pet_status.last_groomed);
            ensure!(blocks_since_last_groomed >= 25u32.into(), Error::<T>::PetGroomedTooRecently);
            
            // 4. Update the pet's hygiene.
            let mut pet_needs = PetNeedsStorage::<T>::get(pet_id).ok_or(Error::<T>::PetNeedsDoNotExist)?;
            let old_hygiene = pet_needs.hygiene;
            let hygiene_increased = 35;
            pet_needs.hygiene = (pet_needs.hygiene.saturating_add(hygiene_increased)).min(100);
            
            // 5. Update the pet's last groomed time.
            pet_status.last_groomed = current_block;
            pet_status.last_interaction = current_block;
            
            // 6. Update the pet's mood based on needs.
            Self::update_pet_mood(&mut pet_status, &pet_needs);
            
            // 7. Store the updated pet status and needs.
            PetStatuses::<T>::insert(pet_id, pet_status.clone());
            PetNeedsStorage::<T>::insert(pet_id, pet_needs.clone());
            
            // 8. Emit the events.
            Self::deposit_event(Event::PetGroomed {
                pet_id,
                hygiene_increased,
            });
            
            Self::deposit_event(Event::PetNeedChanged {
                pet_id,
                need_type: NeedType::Hygiene,
                old_value: old_hygiene,
                new_value: pet_needs.hygiene,
            });
            
            Self::deposit_event(Event::PetMoodChanged {
                pet_id,
                mood: pet_status.mood,
            });
            
            Ok(())
        }

        /// Socialize a pet.
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn socialize_pet(
            origin: OriginFor<T>,
            pet_id: PetId,
            target_pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // 1. Check if the pet status exists.
            let mut pet_status = PetStatuses::<T>::get(pet_id).ok_or(Error::<T>::PetStatusDoesNotExist)?;
            
            // 2. Check if the sender is the owner of the pet.
            ensure!(pet_status.owner == sender, Error::<T>::PetNotOwnedBySender);
            
            // 3. Check if the target pet exists.
            ensure!(PetStatuses::<T>::contains_key(target_pet_id), Error::<T>::PetStatusDoesNotExist);
            
            // 4. Check if the pet socialized too recently.
            let current_block = frame_system::Pallet::<T>::block_number();
            let blocks_since_last_socialized = current_block.saturating_sub(pet_status.last_socialized);
            ensure!(blocks_since_last_socialized >= 30u32.into(), Error::<T>::PetSocializedTooRecently);
            
            // 5. Update the pet's social need.
            let mut pet_needs = PetNeedsStorage::<T>::get(pet_id).ok_or(Error::<T>::PetNeedsDoNotExist)?;
            let old_social = pet_needs.social;
            let social_increased = 30;
            pet_needs.social = (pet_needs.social.saturating_add(social_increased)).min(100);
            
            // 6. Update the pet's last socialized time.
            pet_status.last_socialized = current_block;
            pet_status.last_interaction = current_block;
            
            // 7. Update the pet's mood based on needs.
            Self::update_pet_mood(&mut pet_status, &pet_needs);
            
            // 8. Store the updated pet status and needs.
            PetStatuses::<T>::insert(pet_id, pet_status.clone());
            PetNeedsStorage::<T>::insert(pet_id, pet_needs.clone());
            
            // 9. Also update the target pet's social need.
            if let Some(mut target_pet_status) = PetStatuses::<T>::get(target_pet_id) {
                if let Some(mut target_pet_needs) = PetNeedsStorage::<T>::get(target_pet_id) {
                    let old_target_social = target_pet_needs.social;
                    target_pet_needs.social = (target_pet_needs.social.saturating_add(20)).min(100);
                    
                    target_pet_status.last_socialized = current_block;
                    target_pet_status.last_interaction = current_block;
                    
                    Self::update_pet_mood(&mut target_pet_status, &target_pet_needs);
                    
                    PetStatuses::<T>::insert(target_pet_id, target_pet_status.clone());
                    PetNeedsStorage::<T>::insert(target_pet_id, target_pet_needs.clone());
                    
                    Self::deposit_event(Event::PetNeedChanged {
                        pet_id: target_pet_id,
                        need_type: NeedType::Social,
                        old_value: old_target_social,
                        new_value: target_pet_needs.social,
                    });
                    
                    Self::deposit_event(Event::PetMoodChanged {
                        pet_id: target_pet_id,
                        mood: target_pet_status.mood,
                    });
                }
            }
            
            // 10. Emit the events.
            Self::deposit_event(Event::PetSocialized {
                pet_id,
                social_increased,
            });
            
            Self::deposit_event(Event::PetNeedChanged {
                pet_id,
                need_type: NeedType::Social,
                old_value: old_social,
                new_value: pet_needs.social,
            });
            
            Self::deposit_event(Event::PetMoodChanged {
                pet_id,
                mood: pet_status.mood,
            });
            
            Ok(())
        }

        /// Create a new condition (admin only).
        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_condition(
            origin: OriginFor<T>,
            name: BoundedVec<u8, T::MaxConditionNameLen>,
            description: BoundedVec<u8, T::MaxConditionDescLen>,
            condition_type: ConditionType,
            severity: ConditionSeverity,
            duration_blocks: BlockNumberFor<T>,
            stat_modifiers: Vec<(StatType, i8)>,
            need_modifiers: Vec<(NeedType, i8)>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            // 1. Get the next condition ID.
            let condition_id = Self::next_condition_id();
            let next_id = condition_id.checked_add(1).ok_or(Error::<T>::NextIdOverflow)?;
            NextConditionId::<T>::put(next_id);
            
            // 2. Create the condition.
            let condition = Condition::<T> {
                id: condition_id,
                name: name.clone(),
                description,
                condition_type,
                severity,
                duration_blocks,
                stat_modifiers,
                need_modifiers,
            };
            
            // 3. Store the condition.
            Conditions::<T>::insert(condition_id, condition);
            
            Ok(())
        }

        /// Apply a condition to a pet (admin only).
        #[pallet::call_index(7)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn apply_condition(
            origin: OriginFor<T>,
            pet_id: PetId,
            condition_id: ConditionId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            // 1. Check if the pet status exists.
            ensure!(PetStatuses::<T>::contains_key(pet_id), Error::<T>::PetStatusDoesNotExist);
            
            // 2. Check if the condition exists.
            let condition = Conditions::<T>::get(condition_id).ok_or(Error::<T>::ConditionDoesNotExist)?;
            
            // 3. Check if the pet already has this condition.
            let pet_conditions = PetConditions::<T>::get(pet_id);
            for pet_condition in pet_conditions.iter() {
                if pet_condition.condition_id == condition_id {
                    return Err(Error::<T>::PetAlreadyHasCondition.into());
                }
            }
            
            // 4. Apply the condition.
            let current_block = frame_system::Pallet::<T>::block_number();
            let expires_at_block = current_block.saturating_add(condition.duration_blocks);
            
            let pet_condition = PetCondition::<T> {
                pet_id,
                condition_id,
                started_at_block: current_block,
                expires_at_block,
            };
            
            // 5. Store the pet condition.
            PetConditions::<T>::try_mutate(pet_id, |conditions| -> DispatchResult {
                conditions.try_push(pet_condition).map_err(|_| Error::<T>::MaxPetConditionsReached)?;
                Ok(())
            })?;
            
            // 6. Apply stat modifiers.
            if let Some(mut pet_stats) = PetStatsStorage::<T>::get(pet_id) {
                for (stat_type, modifier) in condition.stat_modifiers.iter() {
                    match stat_type {
                        StatType::Strength => {
                            let old_value = pet_stats.strength;
                            if *modifier > 0 {
                                pet_stats.strength = (pet_stats.strength.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_stats.strength = pet_stats.strength.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.strength,
                            });
                        },
                        StatType::Agility => {
                            let old_value = pet_stats.agility;
                            if *modifier > 0 {
                                pet_stats.agility = (pet_stats.agility.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_stats.agility = pet_stats.agility.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.agility,
                            });
                        },
                        StatType::Intelligence => {
                            let old_value = pet_stats.intelligence;
                            if *modifier > 0 {
                                pet_stats.intelligence = (pet_stats.intelligence.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_stats.intelligence = pet_stats.intelligence.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.intelligence,
                            });
                        },
                        StatType::Vitality => {
                            let old_value = pet_stats.vitality;
                            if *modifier > 0 {
                                pet_stats.vitality = (pet_stats.vitality.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_stats.vitality = pet_stats.vitality.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.vitality,
                            });
                        },
                        StatType::Charisma => {
                            let old_value = pet_stats.charisma;
                            if *modifier > 0 {
                                pet_stats.charisma = (pet_stats.charisma.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_stats.charisma = pet_stats.charisma.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.charisma,
                            });
                        },
                    }
                }
                PetStatsStorage::<T>::insert(pet_id, pet_stats);
            }
            
            // 7. Apply need modifiers.
            if let Some(mut pet_needs) = PetNeedsStorage::<T>::get(pet_id) {
                for (need_type, modifier) in condition.need_modifiers.iter() {
                    match need_type {
                        NeedType::Hunger => {
                            let old_value = pet_needs.hunger;
                            if *modifier > 0 {
                                pet_needs.hunger = (pet_needs.hunger.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_needs.hunger = pet_needs.hunger.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetNeedChanged {
                                pet_id,
                                need_type: *need_type,
                                old_value,
                                new_value: pet_needs.hunger,
                            });
                        },
                        NeedType::Energy => {
                            let old_value = pet_needs.energy;
                            if *modifier > 0 {
                                pet_needs.energy = (pet_needs.energy.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_needs.energy = pet_needs.energy.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetNeedChanged {
                                pet_id,
                                need_type: *need_type,
                                old_value,
                                new_value: pet_needs.energy,
                            });
                        },
                        NeedType::Happiness => {
                            let old_value = pet_needs.happiness;
                            if *modifier > 0 {
                                pet_needs.happiness = (pet_needs.happiness.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_needs.happiness = pet_needs.happiness.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetNeedChanged {
                                pet_id,
                                need_type: *need_type,
                                old_value,
                                new_value: pet_needs.happiness,
                            });
                        },
                        NeedType::Hygiene => {
                            let old_value = pet_needs.hygiene;
                            if *modifier > 0 {
                                pet_needs.hygiene = (pet_needs.hygiene.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_needs.hygiene = pet_needs.hygiene.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetNeedChanged {
                                pet_id,
                                need_type: *need_type,
                                old_value,
                                new_value: pet_needs.hygiene,
                            });
                        },
                        NeedType::Social => {
                            let old_value = pet_needs.social;
                            if *modifier > 0 {
                                pet_needs.social = (pet_needs.social.saturating_add(*modifier as u8)).min(100);
                            } else {
                                pet_needs.social = pet_needs.social.saturating_sub((-*modifier) as u8);
                            }
                            Self::deposit_event(Event::PetNeedChanged {
                                pet_id,
                                need_type: *need_type,
                                old_value,
                                new_value: pet_needs.social,
                            });
                        },
                    }
                }
                PetNeedsStorage::<T>::insert(pet_id, pet_needs);
            }
            
            // 8. Update the pet's mood.
            if let Some(mut pet_status) = PetStatuses::<T>::get(pet_id) {
                if let Some(pet_needs) = PetNeedsStorage::<T>::get(pet_id) {
                    Self::update_pet_mood(&mut pet_status, &pet_needs);
                    PetStatuses::<T>::insert(pet_id, pet_status.clone());
                    
                    Self::deposit_event(Event::PetMoodChanged {
                        pet_id,
                        mood: pet_status.mood,
                    });
                }
            }
            
            // 9. Emit the event.
            Self::deposit_event(Event::PetDevelopedCondition {
                pet_id,
                condition_id,
                name: condition.name.to_vec(),
            });
            
            Ok(())
        }

        /// Remove a condition from a pet (admin only).
        #[pallet::call_index(8)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn remove_condition(
            origin: OriginFor<T>,
            pet_id: PetId,
            condition_id: ConditionId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            // 1. Check if the pet status exists.
            ensure!(PetStatuses::<T>::contains_key(pet_id), Error::<T>::PetStatusDoesNotExist);
            
            // 2. Check if the condition exists.
            let condition = Conditions::<T>::get(condition_id).ok_or(Error::<T>::ConditionDoesNotExist)?;
            
            // 3. Check if the pet has this condition.
            let mut pet_conditions = PetConditions::<T>::get(pet_id);
            let condition_index = pet_conditions.iter().position(|c| c.condition_id == condition_id)
                .ok_or(Error::<T>::PetDoesNotHaveCondition)?;
            
            // 4. Remove the condition.
            pet_conditions.swap_remove(condition_index);
            PetConditions::<T>::insert(pet_id, pet_conditions);
            
            // 5. Revert stat modifiers.
            if let Some(mut pet_stats) = PetStatsStorage::<T>::get(pet_id) {
                for (stat_type, modifier) in condition.stat_modifiers.iter() {
                    match stat_type {
                        StatType::Strength => {
                            let old_value = pet_stats.strength;
                            if *modifier > 0 {
                                pet_stats.strength = pet_stats.strength.saturating_sub(*modifier as u8);
                            } else {
                                pet_stats.strength = (pet_stats.strength.saturating_add((-*modifier) as u8)).min(100);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.strength,
                            });
                        },
                        StatType::Agility => {
                            let old_value = pet_stats.agility;
                            if *modifier > 0 {
                                pet_stats.agility = pet_stats.agility.saturating_sub(*modifier as u8);
                            } else {
                                pet_stats.agility = (pet_stats.agility.saturating_add((-*modifier) as u8)).min(100);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.agility,
                            });
                        },
                        StatType::Intelligence => {
                            let old_value = pet_stats.intelligence;
                            if *modifier > 0 {
                                pet_stats.intelligence = pet_stats.intelligence.saturating_sub(*modifier as u8);
                            } else {
                                pet_stats.intelligence = (pet_stats.intelligence.saturating_add((-*modifier) as u8)).min(100);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.intelligence,
                            });
                        },
                        StatType::Vitality => {
                            let old_value = pet_stats.vitality;
                            if *modifier > 0 {
                                pet_stats.vitality = pet_stats.vitality.saturating_sub(*modifier as u8);
                            } else {
                                pet_stats.vitality = (pet_stats.vitality.saturating_add((-*modifier) as u8)).min(100);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.vitality,
                            });
                        },
                        StatType::Charisma => {
                            let old_value = pet_stats.charisma;
                            if *modifier > 0 {
                                pet_stats.charisma = pet_stats.charisma.saturating_sub(*modifier as u8);
                            } else {
                                pet_stats.charisma = (pet_stats.charisma.saturating_add((-*modifier) as u8)).min(100);
                            }
                            Self::deposit_event(Event::PetStatChanged {
                                pet_id,
                                stat_type: *stat_type,
                                old_value,
                                new_value: pet_stats.charisma,
                            });
                        },
                    }
                }
                PetStatsStorage::<T>::insert(pet_id, pet_stats);
            }
            
            // 6. Update the pet's mood.
            if let Some(mut pet_status) = PetStatuses::<T>::get(pet_id) {
                if let Some(pet_needs) = PetNeedsStorage::<T>::get(pet_id) {
                    Self::update_pet_mood(&mut pet_status, &pet_needs);
                    PetStatuses::<T>::insert(pet_id, pet_status.clone());
                    
                    Self::deposit_event(Event::PetMoodChanged {
                        pet_id,
                        mood: pet_status.mood,
                    });
                }
            }
            
            // 7. Emit the event.
            Self::deposit_event(Event::PetRecoveredFromCondition {
                pet_id,
                condition_id,
                name: condition.name.to_vec(),
            });
            
            Ok(())
        }

        /// Update a pet's stats (admin only).
        #[pallet::call_index(9)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn update_pet_stats(
            origin: OriginFor<T>,
            pet_id: PetId,
            strength: Option<StatValue>,
            agility: Option<StatValue>,
            intelligence: Option<StatValue>,
            vitality: Option<StatValue>,
            charisma: Option<StatValue>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            // 1. Check if the pet status exists.
            ensure!(PetStatuses::<T>::contains_key(pet_id), Error::<T>::PetStatusDoesNotExist);
            
            // 2. Get the pet stats.
            let mut pet_stats = PetStatsStorage::<T>::get(pet_id).ok_or(Error::<T>::PetStatsDoNotExist)?;
            
            // 3. Update the stats.
            if let Some(strength) = strength {
                let old_value = pet_stats.strength;
                pet_stats.strength = strength;
                Self::deposit_event(Event::PetStatChanged {
                    pet_id,
                    stat_type: StatType::Strength,
                    old_value,
                    new_value: strength,
                });
            }
            
            if let Some(agility) = agility {
                let old_value = pet_stats.agility;
                pet_stats.agility = agility;
                Self::deposit_event(Event::PetStatChanged {
                    pet_id,
                    stat_type: StatType::Agility,
                    old_value,
                    new_value: agility,
                });
            }
            
            if let Some(intelligence) = intelligence {
                let old_value = pet_stats.intelligence;
                pet_stats.intelligence = intelligence;
                Self::deposit_event(Event::PetStatChanged {
                    pet_id,
                    stat_type: StatType::Intelligence,
                    old_value,
                    new_value: intelligence,
                });
            }
            
            if let Some(vitality) = vitality {
                let old_value = pet_stats.vitality;
                pet_stats.vitality = vitality;
                Self::deposit_event(Event::PetStatChanged {
                    pet_id,
                    stat_type: StatType::Vitality,
                    old_value,
                    new_value: vitality,
                });
            }
            
            if let Some(charisma) = charisma {
                let old_value = pet_stats.charisma;
                pet_stats.charisma = charisma;
                Self::deposit_event(Event::PetStatChanged {
                    pet_id,
                    stat_type: StatType::Charisma,
                    old_value,
                    new_value: charisma,
                });
            }
            
            // 4. Store the updated pet stats.
            PetStatsStorage::<T>::insert(pet_id, pet_stats);
            
            Ok(())
        }
    }

    // --- Pallet Internal Helper Functions ---
    impl<T: Config> Pallet<T> {
        /// Update a pet's mood based on its needs.
        fn update_pet_mood(pet_status: &mut PetStatus<T>, pet_needs: &PetNeeds) {
            // Calculate the average need value.
            let total_needs = pet_needs.hunger as u32 + pet_needs.energy as u32 + pet_needs.happiness as u32 + pet_needs.hygiene as u32 + pet_needs.social as u32;
            let avg_needs = total_needs / 5;
            
            // Update the mood based on the average need value.
            pet_status.mood = if avg_needs >= 80 {
                PetMood::Happy
            } else if avg_needs >= 60 {
                PetMood::Content
            } else if avg_needs >= 40 {
                PetMood::Neutral
            } else if avg_needs >= 20 {
                PetMood::Sad
            } else {
                PetMood::Distressed
            };
        }

        /// Process pet updates (need decay and condition updates).
        fn process_pet_updates(current_block: BlockNumberFor<T>) {
            // Process need decay for all pets.
            for (pet_id, _) in PetStatuses::<T>::iter() {
                if let Some(last_decay) = LastNeedDecay::<T>::get(pet_id) {
                    let blocks_since_decay = current_block.saturating_sub(last_decay);
                    
                    if blocks_since_decay >= T::NeedDecayInterval::get() {
                        // Decay the pet's needs.
                        if let Some(mut pet_needs) = PetNeedsStorage::<T>::get(pet_id) {
                            let decay_amount = T::NeedDecayAmount::get();
                            
                            let old_hunger = pet_needs.hunger;
                            let old_energy = pet_needs.energy;
                            let old_happiness = pet_needs.happiness;
                            let old_hygiene = pet_needs.hygiene;
                            let old_social = pet_needs.social;
                            
                            pet_needs.hunger = pet_needs.hunger.saturating_sub(decay_amount);
                            pet_needs.energy = pet_needs.energy.saturating_sub(decay_amount);
                            pet_needs.happiness = pet_needs.happiness.saturating_sub(decay_amount);
                            pet_needs.hygiene = pet_needs.hygiene.saturating_sub(decay_amount);
                            pet_needs.social = pet_needs.social.saturating_sub(decay_amount);
                            
                            PetNeedsStorage::<T>::insert(pet_id, pet_needs.clone());
                            LastNeedDecay::<T>::insert(pet_id, current_block);
                            
                            // Update the pet's mood.
                            if let Some(mut pet_status) = PetStatuses::<T>::get(pet_id) {
                                Self::update_pet_mood(&mut pet_status, &pet_needs);
                                PetStatuses::<T>::insert(pet_id, pet_status.clone());
                                
                                Self::deposit_event(Event::PetMoodChanged {
                                    pet_id,
                                    mood: pet_status.mood,
                                });
                            }
                            
                            // Emit need changed events.
                            if old_hunger != pet_needs.hunger {
                                Self::deposit_event(Event::PetNeedChanged {
                                    pet_id,
                                    need_type: NeedType::Hunger,
                                    old_value: old_hunger,
                                    new_value: pet_needs.hunger,
                                });
                            }
                            
                            if old_energy != pet_needs.energy {
                                Self::deposit_event(Event::PetNeedChanged {
                                    pet_id,
                                    need_type: NeedType::Energy,
                                    old_value: old_energy,
                                    new_value: pet_needs.energy,
                                });
                            }
                            
                            if old_happiness != pet_needs.happiness {
                                Self::deposit_event(Event::PetNeedChanged {
                                    pet_id,
                                    need_type: NeedType::Happiness,
                                    old_value: old_happiness,
                                    new_value: pet_needs.happiness,
                                });
                            }
                            
                            if old_hygiene != pet_needs.hygiene {
                                Self::deposit_event(Event::PetNeedChanged {
                                    pet_id,
                                    need_type: NeedType::Hygiene,
                                    old_value: old_hygiene,
                                    new_value: pet_needs.hygiene,
                                });
                            }
                            
                            if old_social != pet_needs.social {
                                Self::deposit_event(Event::PetNeedChanged {
                                    pet_id,
                                    need_type: NeedType::Social,
                                    old_value: old_social,
                                    new_value: pet_needs.social,
                                });
                            }
                            
                            Self::deposit_event(Event::PetNeedsDecayed {
                                pet_id,
                            });
                        }
                    }
                }
            }
            
            // Process condition updates for all pets.
            for (pet_id, pet_conditions) in PetConditions::<T>::iter() {
                let mut conditions_to_remove = Vec::new();
                
                // Check for expired conditions.
                for (i, pet_condition) in pet_conditions.iter().enumerate() {
                    if current_block >= pet_condition.expires_at_block {
                        conditions_to_remove.push((i, pet_condition.condition_id));
                    }
                }
                
                // Remove expired conditions.
                if !conditions_to_remove.is_empty() {
                    for (i, condition_id) in conditions_to_remove.iter().rev() {
                        if let Ok(()) = Self::remove_condition(RawOrigin::Root.into(), pet_id, *condition_id) {
                            // Condition was successfully removed.
                        }
                    }
                }
            }
        }
    }
}
