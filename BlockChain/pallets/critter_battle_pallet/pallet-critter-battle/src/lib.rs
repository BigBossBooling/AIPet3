//! # CritterCraft Battle Pallet
//!
//! A pallet that manages pet battles in the CritterCraft ecosystem.
//!
//! ## Overview
//!
//! The battle pallet provides the following features:
//! - Pet vs. Pet battles with turn-based mechanics
//! - Battle matchmaking and ranking system
//! - Battle rewards and experience
//! - Special moves and elemental advantages
//! - Tournament system
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `create_challenge` - Challenge another pet to a battle
//! * `accept_challenge` - Accept a battle challenge
//! * `decline_challenge` - Decline a battle challenge
//! * `execute_move` - Execute a battle move during a battle
//! * `forfeit_battle` - Forfeit an ongoing battle
//! * `claim_rewards` - Claim rewards from a completed battle
//! * `enter_tournament` - Enter a pet into a tournament
//! * `set_battle_params` - Update battle parameters
, and 
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crittercraft_traits::{
        AdvancedPetManagement, AttributeType, BattleSystemIntegration, PetId, PetStats, SharedNftManager,
    };
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, Get, Randomness, ReservableCurrency},
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::{
        traits::{AccountIdConversion, CheckedAdd, CheckedSub, Zero, Saturating},
        Perbill,
    };
    use sp_std::{prelude::*, vec::Vec};

    // Define the battle ID type
    pub type BattleId = u32;

    // Define the move ID type
    pub type MoveId = u8;

    // Define the tournament ID type
    pub type TournamentId = u32;

    // Define the battle status enum
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum BattleStatus {
        Challenged,
        Active,
        Completed,
        Forfeited,
        Expired,
    }

    // Define the battle outcome enum
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum BattleOutcome {
        Pet1Win,
        Pet2Win,
        Draw,
        Forfeited,
    }

    // Define the battle move enum
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum BattleMove {
        Attack,
        Defend,
        SpecialAttack,
        Heal,
        Dodge,
        ElementalAttack,
        StatusEffect,
        Combo,
        Ultimate,
    }
    
    // Define the battle move result enum
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum BattleMoveResult {
        Hit(u8),           // Damage dealt
        Miss,              // Attack missed
        Critical(u8),      // Critical hit with damage
        Heal(u8),          // Health restored
        StatusApplied(u8), // Status effect applied with ID
        Combo(u8, u8),     // Combo hits and total damage
    }
    
    // Define the status effect enum
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum StatusEffect {
        Burn(u8),      // Damage per turn, remaining turns
        Freeze(u8),    // Skip turns chance, remaining turns
        Poison(u8),    // Damage per turn, remaining turns
        Stun(u8),      // Skip turn chance, remaining turns
        Strengthen(u8), // Attack boost, remaining turns
        Shield(u8),    // Damage reduction, remaining turns
    }

    // Define the battle move history entry
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct BattleMoveHistoryEntry {
        pub turn: u8,
        pub pet_id: PetId,
        pub move_type: BattleMove,
        pub result: BattleMoveResult,
    }

    // Define the battle struct
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Battle<AccountId, BlockNumber> {
        pub id: BattleId,
        pub pet1_id: PetId,
        pub pet2_id: PetId,
        pub pet1_owner: AccountId,
        pub pet2_owner: AccountId,
        pub status: BattleStatus,
        pub current_turn: u8,
        pub pet1_health: u8,
        pub pet2_health: u8,
        pub pet1_energy: u8,
        pub pet2_energy: u8,
        pub pet1_status_effects: BoundedVec<StatusEffect, ConstU32<5>>,
        pub pet2_status_effects: BoundedVec<StatusEffect, ConstU32<5>>,
        pub last_move_pet1: Option<BattleMove>,
        pub last_move_pet2: Option<BattleMove>,
        pub last_move_result: Option<BattleMoveResult>,
        pub combo_counter_pet1: u8,
        pub combo_counter_pet2: u8,
        pub outcome: Option<BattleOutcome>,
        pub created_at: BlockNumber,
        pub updated_at: BlockNumber,
        pub completed_at: Option<BlockNumber>,
        pub reward_claimed: bool,
        pub battle_rating: Option<u16>, // For matchmaking and ranking
    }

    // Define the tournament struct
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Tournament<BlockNumber> {
        pub id: TournamentId,
        pub name: BoundedVec<u8, ConstU32<64>>,
        pub description: BoundedVec<u8, ConstU32<256>>,
        pub max_participants: u32,
        pub current_participants: u32,
        pub min_pet_level: u16,
        pub max_pet_level: u16,
        pub entry_fee: BalanceOf<T>,
        pub prize_pool: BalanceOf<T>,
        pub status: TournamentStatus,
        pub start_block: BlockNumber,
        pub end_block: Option<BlockNumber>,
        pub winner_pet_id: Option<PetId>,
    }

    // Define the tournament status enum
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum TournamentStatus {
        Registration,
        InProgress,
        Completed,
        Cancelled,
    }

    // Define the battle parameters struct
    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct BattleParameters<Balance> {
        pub challenge_bond: Balance,
        pub forfeit_penalty: Balance,
        pub base_reward: Balance,
        pub challenge_expiry_blocks: u32,
        pub max_turns: u8,
        pub base_experience_reward: u32,
        pub elemental_advantage_multiplier: Perbill,
        pub critical_hit_chance: Perbill,
        pub critical_hit_multiplier: Perbill,
        pub combo_threshold: u8,
        pub combo_bonus_multiplier: Perbill,
        pub status_effect_duration: u8,
        pub initial_energy: u8,
        pub energy_per_turn: u8,
        pub ultimate_move_energy_cost: u8,
        pub matchmaking_rating_change: u16,
    }

    // Define the pallet's configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency used for battle bonds and rewards
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The randomness source for battle outcomes
        type BattleRandomness: Randomness<Self::Hash, Self::BlockNumber>;

        /// The NFT manager for pet ownership verification
        type NftManager: SharedNftManager<Self::AccountId, PetId>;

        /// The pet manager for pet stats and experience
        type PetManager: AdvancedPetManagement<Self::AccountId, Self::BlockNumber>;

        /// The maximum number of active battles per account
        #[pallet::constant]
        type MaxActiveBattles: Get<u32>;

        /// The maximum number of active tournaments
        #[pallet::constant]
        type MaxActiveTournaments: Get<u32>;

        /// The maximum number of participants in a tournament
        #[pallet::constant]
        type MaxTournamentParticipants: Get<u32>;

        /// The origin that can update battle parameters
        type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    // Define the pallet's events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A battle challenge has been created. [battle_id, challenger, challenged, pet1_id, pet2_id]
        BattleChallengeCreated(BattleId, T::AccountId, T::AccountId, PetId, PetId),
        /// A battle challenge has been accepted. [battle_id]
        BattleChallengeAccepted(BattleId),
        /// A battle challenge has been declined. [battle_id]
        BattleChallengeDeclined(BattleId),
        /// A battle move has been executed. [battle_id, player, pet_id, move]
        BattleMoveExecuted(BattleId, T::AccountId, PetId, BattleMove),
        /// A battle move result. [battle_id, pet_id, result]
        BattleMoveResult(BattleId, PetId, BattleMoveResult),
        /// A status effect has been applied. [battle_id, pet_id, effect]
        StatusEffectApplied(BattleId, PetId, StatusEffect),
        /// A status effect has expired. [battle_id, pet_id, effect]
        StatusEffectExpired(BattleId, PetId, StatusEffect),
        /// A combo has been triggered. [battle_id, pet_id, combo_count]
        ComboTriggered(BattleId, PetId, u8),
        /// An ultimate move has been used. [battle_id, pet_id]
        UltimateMoveUsed(BattleId, PetId),
        /// A battle has been completed. [battle_id, outcome]
        BattleCompleted(BattleId, BattleOutcome),
        /// A battle has been forfeited. [battle_id, forfeiter]
        BattleForfeited(BattleId, T::AccountId),
        /// Battle rewards have been claimed. [battle_id, claimer, amount]
        BattleRewardsClaimed(BattleId, T::AccountId, BalanceOf<T>),
        /// A pet has entered a tournament. [tournament_id, owner, pet_id]
        TournamentEntered(TournamentId, T::AccountId, PetId),
        /// A tournament has started. [tournament_id]
        TournamentStarted(TournamentId),
        /// A tournament has ended. [tournament_id, winner_pet_id]
        TournamentEnded(TournamentId, PetId),
        /// Battle parameters have been updated.
        BattleParametersUpdated,
        /// A pet has been added to the matchmaking queue. [pet_id, owner, rating]
        PetAddedToMatchmaking(PetId, T::AccountId, u16),
        /// A pet has been removed from the matchmaking queue. [pet_id, owner]
        PetRemovedFromMatchmaking(PetId, T::AccountId),
        /// A matchmaking battle has been created. [battle_id, pet1_id, pet2_id]
        MatchmakingBattleCreated(BattleId, PetId, PetId),
        /// A pet's battle rating has changed. [pet_id, old_rating, new_rating]
        PetBattleRatingChanged(PetId, u16, u16),
    }

    // Define the pallet's errors
    #[pallet::error]
    pub enum Error<T> {
        /// The battle does not exist
        BattleNotFound,
        /// The tournament does not exist
        TournamentNotFound,
        /// The account has too many active battles
        TooManyActiveBattles,
        /// The pet is already in a battle
        PetAlreadyInBattle,
        /// The pet is not owned by the account
        NotPetOwner,
        /// The battle is not in the correct status
        InvalidBattleStatus,
        /// The tournament is not in the correct status
        InvalidTournamentStatus,
        /// The account is not a participant in the battle
        NotBattleParticipant,
        /// It's not the account's turn in the battle
        NotYourTurn,
        /// The battle has expired
        BattleExpired,
        /// The battle has already been completed
        BattleAlreadyCompleted,
        /// The rewards have already been claimed
        RewardsAlreadyClaimed,
        /// The pet does not meet the tournament requirements
        PetDoesNotMeetRequirements,
        /// The tournament is full
        TournamentFull,
        /// The pet is already in the tournament
        PetAlreadyInTournament,
        /// The account has insufficient balance
        InsufficientBalance,
        /// Invalid battle parameters
        InvalidBattleParameters,
        /// The battle ID has overflowed
        BattleIdOverflow,
        /// The tournament ID has overflowed
        TournamentIdOverflow,
        /// Not enough energy for the move
        InsufficientEnergy,
        /// The pet is affected by a status effect that prevents this action
        PreventedByStatusEffect,
        /// The pet already has the maximum number of status effects
        TooManyStatusEffects,
        /// The pet is already in the matchmaking queue
        AlreadyInMatchmakingQueue,
        /// The pet is not in the matchmaking queue
        NotInMatchmakingQueue,
        /// The matchmaking queue is empty
        MatchmakingQueueEmpty,
        /// No suitable match found in the matchmaking queue
        NoSuitableMatchFound,
        /// The battle history is too long
        BattleHistoryTooLong,
        /// The combo counter has reached its maximum
        ComboCounterMaximum,
        /// The move is not available to this pet
        MoveNotAvailable,
        /// The battle move result is invalid
        InvalidBattleMoveResult,
    }

    // Define the pallet's storage items
    #[pallet::storage]
    #[pallet::getter(fn battles)]
    pub type Battles<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BattleId,
        Battle<T::AccountId, T::BlockNumber>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn battle_count)]
    pub type BattleCount<T: Config> = StorageValue<_, BattleId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_active_battle)]
    pub type PetActiveBattle<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BattleId,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn account_active_battles)]
    pub type AccountActiveBattles<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<BattleId, T::MaxActiveBattles>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn tournaments)]
    pub type Tournaments<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        TournamentId,
        Tournament<T::BlockNumber>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn tournament_count)]
    pub type TournamentCount<T: Config> = StorageValue<_, TournamentId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn tournament_participants)]
    pub type TournamentParticipants<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        TournamentId,
        Blake2_128Concat,
        PetId,
        T::AccountId,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pet_active_tournament)]
    pub type PetActiveTournament<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        TournamentId,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn battle_parameters)]
    pub type BattleParams<T: Config> = StorageValue<_, BattleParameters<BalanceOf<T>>, OptionQuery>;
    
    #[pallet::storage]
    #[pallet::getter(fn battle_history)]
    pub type BattleHistory<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BattleId,
        BoundedVec<BattleMoveHistoryEntry, ConstU32<100>>,
        ValueQuery,
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_battle_stats)]
    pub type PetBattleStats<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        (u32, u32, u32, u16), // (wins, losses, draws, rating)
        ValueQuery,
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn battle_matchmaking_queue)]
    pub type BattleMatchmakingQueue<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        (T::AccountId, u16, T::BlockNumber), // (owner, rating, enqueue_time)
        OptionQuery,
    >;

    // Define the pallet itself
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Define the balance type
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Define the pallet's call (dispatchable functions)
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a battle challenge
        #[pallet::weight(T::WeightInfo::create_challenge())]
        pub fn create_challenge(
            origin: OriginFor<T>,
            pet_id: PetId,
            target_pet_id: PetId,
        ) -> DispatchResultWithPostInfo {
            let challenger = ensure_signed(origin)?;
            
            // Ensure the challenger owns the pet
            ensure!(
                T::NftManager::owner_of(&pet_id) == Some(challenger.clone()),
                Error::<T>::NotPetOwner
            );
            
            // Get the target pet owner
            let target_owner = T::NftManager::owner_of(&target_pet_id).ok_or(Error::<T>::NotPetOwner)?;
            
            // Ensure the pets are not the same
            ensure!(pet_id != target_pet_id, Error::<T>::InvalidBattleStatus);
            
            // Ensure the challenger is not challenging themselves
            ensure!(challenger != target_owner, Error::<T>::InvalidBattleStatus);
            
            // Ensure the challenger doesn't have too many active battles
            ensure!(
                AccountActiveBattles::<T>::get(&challenger).len() < T::MaxActiveBattles::get() as usize,
                Error::<T>::TooManyActiveBattles
            );
            
            // Ensure the pets are not already in battles
            ensure!(
                !PetActiveBattle::<T>::contains_key(&pet_id),
                Error::<T>::PetAlreadyInBattle
            );
            ensure!(
                !PetActiveBattle::<T>::contains_key(&target_pet_id),
                Error::<T>::PetAlreadyInBattle
            );
            
            // Get the battle parameters
            let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
            
            // Reserve the challenge bond
            T::Currency::reserve(&challenger, params.challenge_bond)?;
            
            // Create the battle
            let battle_id = Self::next_battle_id()?;
            let now = <frame_system::Pallet<T>>::block_number();
            
            // Get pet stats
            let pet1_stats = T::PetManager::get_pet_attributes(&pet_id)
                .ok_or(Error::<T>::NotPetOwner)?;
            let pet2_stats = T::PetManager::get_pet_attributes(&target_pet_id)
                .ok_or(Error::<T>::NotPetOwner)?;
            
            // Calculate initial health based on vitality
            let pet1_health = pet1_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Vitality { Some(*val) } else { None })
                .unwrap_or(50);
            
            let pet2_health = pet2_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Vitality { Some(*val) } else { None })
                .unwrap_or(50);
            
            // Create empty status effect vectors
            let pet1_status_effects: BoundedVec<StatusEffect, ConstU32<5>> = BoundedVec::default();
            let pet2_status_effects: BoundedVec<StatusEffect, ConstU32<5>> = BoundedVec::default();
            
            let battle = Battle {
                id: battle_id,
                pet1_id: pet_id,
                pet2_id: target_pet_id,
                pet1_owner: challenger.clone(),
                pet2_owner: target_owner.clone(),
                status: BattleStatus::Challenged,
                current_turn: 0,
                pet1_health,
                pet2_health,
                pet1_energy: params.initial_energy,
                pet2_energy: params.initial_energy,
                pet1_status_effects,
                pet2_status_effects,
                last_move_pet1: None,
                last_move_pet2: None,
                last_move_result: None,
                combo_counter_pet1: 0,
                combo_counter_pet2: 0,
                outcome: None,
                created_at: now,
                updated_at: now,
                completed_at: None,
                reward_claimed: false,
                battle_rating: None,
            };
            
            // Store the battle
            Battles::<T>::insert(battle_id, battle);
            BattleCount::<T>::put(battle_id + 1);
            
            // Update pet active battles
            PetActiveBattle::<T>::insert(&pet_id, battle_id);
            PetActiveBattle::<T>::insert(&target_pet_id, battle_id);
            
            // Update account active battles
            AccountActiveBattles::<T>::try_mutate(&challenger, |battles| {
                battles.try_push(battle_id).map_err(|_| Error::<T>::TooManyActiveBattles)
            })?;
            
            // Emit event
            Self::deposit_event(Event::BattleChallengeCreated(
                battle_id,
                challenger,
                target_owner,
                pet_id,
                target_pet_id,
            ));
            
            Ok(().into())
        }
        
        /// Accept a battle challenge
        #[pallet::weight(T::WeightInfo::accept_challenge())]
        pub fn accept_challenge(
            origin: OriginFor<T>,
            battle_id: BattleId,
        ) -> DispatchResultWithPostInfo {
            let acceptor = ensure_signed(origin)?;
            
            // Get the battle
            let mut battle = Self::battles(battle_id).ok_or(Error::<T>::BattleNotFound)?;
            
            // Ensure the battle is in the challenged state
            ensure!(battle.status == BattleStatus::Challenged, Error::<T>::InvalidBattleStatus);
            
            // Ensure the acceptor is the target pet owner
            ensure!(battle.pet2_owner == acceptor, Error::<T>::NotBattleParticipant);
            
            // Ensure the battle hasn't expired
            let now = <frame_system::Pallet<T>>::block_number();
            let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
            ensure!(
                now <= battle.created_at + params.challenge_expiry_blocks.into(),
                Error::<T>::BattleExpired
            );
            
            // Update battle status
            battle.status = BattleStatus::Active;
            battle.current_turn = 1; // Pet1 goes first
            battle.updated_at = now;
            
            // Update the battle
            Battles::<T>::insert(battle_id, battle.clone());
            
            // Update account active battles
            AccountActiveBattles::<T>::try_mutate(&acceptor, |battles| {
                battles.try_push(battle_id).map_err(|_| Error::<T>::TooManyActiveBattles)
            })?;
            
            // Emit event
            Self::deposit_event(Event::BattleChallengeAccepted(battle_id));
            
            Ok(().into())
        }
        
        /// Decline a battle challenge
        #[pallet::weight(T::WeightInfo::decline_challenge())]
        pub fn decline_challenge(
            origin: OriginFor<T>,
            battle_id: BattleId,
        ) -> DispatchResultWithPostInfo {
            let decliner = ensure_signed(origin)?;
            
            // Get the battle
            let mut battle = Self::battles(battle_id).ok_or(Error::<T>::BattleNotFound)?;
            
            // Ensure the battle is in the challenged state
            ensure!(battle.status == BattleStatus::Challenged, Error::<T>::InvalidBattleStatus);
            
            // Ensure the decliner is the target pet owner
            ensure!(battle.pet2_owner == decliner, Error::<T>::NotBattleParticipant);
            
            // Update battle status
            battle.status = BattleStatus::Expired;
            battle.updated_at = <frame_system::Pallet<T>>::block_number();
            
            // Update the battle
            Battles::<T>::insert(battle_id, battle.clone());
            
            // Remove pet active battles
            PetActiveBattle::<T>::remove(&battle.pet1_id);
            PetActiveBattle::<T>::remove(&battle.pet2_id);
            
            // Unreserve the challenger's bond
            let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
            T::Currency::unreserve(&battle.pet1_owner, params.challenge_bond);
            
            // Emit event
            Self::deposit_event(Event::BattleChallengeDeclined(battle_id));
            
            Ok(().into())
        }
        
        /// Execute a battle move
        #[pallet::weight(T::WeightInfo::execute_move())]
        pub fn execute_move(
            origin: OriginFor<T>,
            battle_id: BattleId,
            move_type: BattleMove,
        ) -> DispatchResultWithPostInfo {
            let player = ensure_signed(origin)?;
            
            // Get the battle
            let mut battle = Self::battles(battle_id).ok_or(Error::<T>::BattleNotFound)?;
            
            // Ensure the battle is active
            ensure!(battle.status == BattleStatus::Active, Error::<T>::InvalidBattleStatus);
            
            // Determine whose turn it is
            let is_pet1_turn = battle.current_turn % 2 == 1;
            let (active_pet_id, active_owner) = if is_pet1_turn {
                (battle.pet1_id, battle.pet1_owner.clone())
            } else {
                (battle.pet2_id, battle.pet2_owner.clone())
            };
            
            // Ensure it's the player's turn
            ensure!(active_owner == player, Error::<T>::NotYourTurn);
            
            // Process the move
            if is_pet1_turn {
                battle.last_move_pet1 = Some(move_type.clone());
                Self::process_pet1_move(&mut battle, &move_type)?;
            } else {
                battle.last_move_pet2 = Some(move_type.clone());
                Self::process_pet2_move(&mut battle, &move_type)?;
            }
            
            // Increment turn counter
            battle.current_turn += 1;
            battle.updated_at = <frame_system::Pallet<T>>::block_number();
            
            // Check if the battle is over
            let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
            if battle.pet1_health == 0 || battle.pet2_health == 0 || battle.current_turn > params.max_turns {
                // Determine the outcome
                let outcome = if battle.pet1_health == 0 && battle.pet2_health == 0 {
                    BattleOutcome::Draw
                } else if battle.pet1_health == 0 {
                    BattleOutcome::Pet2Win
                } else if battle.pet2_health == 0 {
                    BattleOutcome::Pet1Win
                } else if battle.pet1_health > battle.pet2_health {
                    BattleOutcome::Pet1Win
                } else if battle.pet2_health > battle.pet1_health {
                    BattleOutcome::Pet2Win
                } else {
                    BattleOutcome::Draw
                };
                
                // Update battle status
                battle.status = BattleStatus::Completed;
                battle.outcome = Some(outcome.clone());
                battle.completed_at = Some(battle.updated_at);
                
                // Award experience to pets
                let xp_reward = params.base_experience_reward;
                match outcome {
                    BattleOutcome::Pet1Win => {
                        let _ = T::PetManager::add_experience(&battle.pet1_id, xp_reward);
                        let _ = T::PetManager::add_experience(&battle.pet2_id, xp_reward / 2);
                    },
                    BattleOutcome::Pet2Win => {
                        let _ = T::PetManager::add_experience(&battle.pet2_id, xp_reward);
                        let _ = T::PetManager::add_experience(&battle.pet1_id, xp_reward / 2);
                    },
                    BattleOutcome::Draw => {
                        let _ = T::PetManager::add_experience(&battle.pet1_id, xp_reward / 2);
                        let _ = T::PetManager::add_experience(&battle.pet2_id, xp_reward / 2);
                    },
                    _ => {},
                }
                
                // Remove pet active battles
                PetActiveBattle::<T>::remove(&battle.pet1_id);
                PetActiveBattle::<T>::remove(&battle.pet2_id);
                
                // Emit battle completed event
                Self::deposit_event(Event::BattleCompleted(battle_id, outcome));
            }
            
            // Update the battle
            Battles::<T>::insert(battle_id, battle.clone());
            
            // Emit move executed event
            Self::deposit_event(Event::BattleMoveExecuted(
                battle_id,
                player,
                active_pet_id,
                move_type,
            ));
            
            Ok(().into())
        }
        
        /// Forfeit a battle
        #[pallet::weight(T::WeightInfo::forfeit_battle())]
        pub fn forfeit_battle(
            origin: OriginFor<T>,
            battle_id: BattleId,
        ) -> DispatchResultWithPostInfo {
            let forfeiter = ensure_signed(origin)?;
            
            // Get the battle
            let mut battle = Self::battles(battle_id).ok_or(Error::<T>::BattleNotFound)?;
            
            // Ensure the battle is active
            ensure!(battle.status == BattleStatus::Active, Error::<T>::InvalidBattleStatus);
            
            // Ensure the forfeiter is a participant
            ensure!(
                battle.pet1_owner == forfeiter || battle.pet2_owner == forfeiter,
                Error::<T>::NotBattleParticipant
            );
            
            // Determine the outcome
            let outcome = if battle.pet1_owner == forfeiter {
                BattleOutcome::Pet2Win
            } else {
                BattleOutcome::Pet1Win
            };
            
            // Update battle status
            battle.status = BattleStatus::Forfeited;
            battle.outcome = Some(outcome.clone());
            battle.completed_at = Some(<frame_system::Pallet<T>>::block_number());
            
            // Apply forfeit penalty
            let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
            T::Currency::slash_reserved(&forfeiter, params.forfeit_penalty);
            
            // Award experience to the winner
            let xp_reward = params.base_experience_reward;
            match outcome {
                BattleOutcome::Pet1Win => {
                    let _ = T::PetManager::add_experience(&battle.pet1_id, xp_reward);
                },
                BattleOutcome::Pet2Win => {
                    let _ = T::PetManager::add_experience(&battle.pet2_id, xp_reward);
                },
                _ => {},
            }
            
            // Remove pet active battles
            PetActiveBattle::<T>::remove(&battle.pet1_id);
            PetActiveBattle::<T>::remove(&battle.pet2_id);
            
            // Update the battle
            Battles::<T>::insert(battle_id, battle.clone());
            
            // Emit events
            Self::deposit_event(Event::BattleForfeited(battle_id, forfeiter));
            Self::deposit_event(Event::BattleCompleted(battle_id, outcome));
            
            Ok(().into())
        }
        
        /// Claim battle rewards
        #[pallet::weight(T::WeightInfo::claim_rewards())]
        pub fn claim_rewards(
            origin: OriginFor<T>,
            battle_id: BattleId,
        ) -> DispatchResultWithPostInfo {
            let claimer = ensure_signed(origin)?;
            
            // Get the battle
            let mut battle = Self::battles(battle_id).ok_or(Error::<T>::BattleNotFound)?;
            
            // Ensure the battle is completed
            ensure!(
                battle.status == BattleStatus::Completed || battle.status == BattleStatus::Forfeited,
                Error::<T>::InvalidBattleStatus
            );
            
            // Ensure rewards haven't been claimed yet
            ensure!(!battle.reward_claimed, Error::<T>::RewardsAlreadyClaimed);
            
            // Ensure the claimer is the winner
            let (winner, reward_amount) = match battle.outcome {
                Some(BattleOutcome::Pet1Win) => {
                    ensure!(battle.pet1_owner == claimer, Error::<T>::NotBattleParticipant);
                    (battle.pet1_owner.clone(), Self::calculate_reward(&battle)?)
                },
                Some(BattleOutcome::Pet2Win) => {
                    ensure!(battle.pet2_owner == claimer, Error::<T>::NotBattleParticipant);
                    (battle.pet2_owner.clone(), Self::calculate_reward(&battle)?)
                },
                Some(BattleOutcome::Draw) => {
                    ensure!(
                        battle.pet1_owner == claimer || battle.pet2_owner == claimer,
                        Error::<T>::NotBattleParticipant
                    );
                    let half_reward = Self::calculate_reward(&battle)? / 2u32.into();
                    (claimer.clone(), half_reward)
                },
                _ => {
                    return Err(Error::<T>::InvalidBattleStatus.into());
                }
            };
            
            // Mark rewards as claimed
            battle.reward_claimed = true;
            Battles::<T>::insert(battle_id, battle);
            
            // Unreserve the challenger's bond
            let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
            T::Currency::unreserve(&battle.pet1_owner, params.challenge_bond);
            
            // Transfer the reward
            T::Currency::deposit_creating(&winner, reward_amount);
            
            // Emit event
            Self::deposit_event(Event::BattleRewardsClaimed(battle_id, claimer, reward_amount));
            
            Ok(().into())
        }
        
        /// Enter a tournament
        #[pallet::weight(T::WeightInfo::enter_tournament())]
        pub fn enter_tournament(
            origin: OriginFor<T>,
            tournament_id: TournamentId,
            pet_id: PetId,
        ) -> DispatchResultWithPostInfo {
            let participant = ensure_signed(origin)?;
            
            // Ensure the participant owns the pet
            ensure!(
                T::NftManager::owner_of(&pet_id) == Some(participant.clone()),
                Error::<T>::NotPetOwner
            );
            
            // Get the tournament
            let mut tournament = Self::tournaments(tournament_id).ok_or(Error::<T>::TournamentNotFound)?;
            
            // Ensure the tournament is in registration phase
            ensure!(
                tournament.status == TournamentStatus::Registration,
                Error::<T>::InvalidTournamentStatus
            );
            
            // Ensure the tournament isn't full
            ensure!(
                tournament.current_participants < tournament.max_participants,
                Error::<T>::TournamentFull
            );
            
            // Ensure the pet isn't already in a tournament
            ensure!(
                !PetActiveTournament::<T>::contains_key(&pet_id),
                Error::<T>::PetAlreadyInTournament
            );
            
            // Ensure the pet isn't in an active battle
            ensure!(
                !PetActiveBattle::<T>::contains_key(&pet_id),
                Error::<T>::PetAlreadyInBattle
            );
            
            // Ensure the pet meets the level requirements
            let pet_level = T::PetManager::get_pet_level(&pet_id).ok_or(Error::<T>::NotPetOwner)?;
            ensure!(
                pet_level >= tournament.min_pet_level && pet_level <= tournament.max_pet_level,
                Error::<T>::PetDoesNotMeetRequirements
            );
            
            // Collect the entry fee
            T::Currency::transfer(
                &participant,
                &Self::account_id(),
                tournament.entry_fee,
                ExistenceRequirement::KeepAlive,
            )?;
            
            // Update the tournament
            tournament.current_participants += 1;
            tournament.prize_pool = tournament.prize_pool.saturating_add(tournament.entry_fee);
            Tournaments::<T>::insert(tournament_id, tournament);
            
            // Register the participant
            TournamentParticipants::<T>::insert(tournament_id, pet_id, participant.clone());
            PetActiveTournament::<T>::insert(pet_id, tournament_id);
            
            // Emit event
            Self::deposit_event(Event::TournamentEntered(tournament_id, participant, pet_id));
            
            Ok(().into())
        }
        
        /// Create a tournament
        #[pallet::weight(T::WeightInfo::create_tournament())]
        pub fn create_tournament(
            origin: OriginFor<T>,
            name: Vec<u8>,
            description: Vec<u8>,
            max_participants: u32,
            min_pet_level: u16,
            max_pet_level: u16,
            entry_fee: BalanceOf<T>,
            start_block: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            
            // Validate parameters
            ensure!(max_participants > 0, Error::<T>::InvalidBattleParameters);
            ensure!(max_participants <= T::MaxTournamentParticipants::get(), Error::<T>::InvalidBattleParameters);
            ensure!(min_pet_level <= max_pet_level, Error::<T>::InvalidBattleParameters);
            ensure!(start_block > <frame_system::Pallet<T>>::block_number(), Error::<T>::InvalidBattleParameters);
            
            // Create bounded vectors
            let bounded_name: BoundedVec<u8, ConstU32<64>> = name.try_into()
                .map_err(|_| Error::<T>::InvalidBattleParameters)?;
            let bounded_description: BoundedVec<u8, ConstU32<256>> = description.try_into()
                .map_err(|_| Error::<T>::InvalidBattleParameters)?;
            
            // Generate tournament ID
            let tournament_id = Self::next_tournament_id()?;
            
            // Create the tournament
            let tournament = Tournament {
                id: tournament_id,
                name: bounded_name,
                description: bounded_description,
                max_participants,
                current_participants: 0,
                min_pet_level,
                max_pet_level,
                entry_fee,
                prize_pool: Zero::zero(),
                status: TournamentStatus::Registration,
                start_block,
                end_block: None,
                winner_pet_id: None,
            };
            
            // Store the tournament
            Tournaments::<T>::insert(tournament_id, tournament);
            TournamentCount::<T>::put(tournament_id + 1);
            
            Ok(().into())
        }
        
        /// Set battle parameters
        #[pallet::weight(T::WeightInfo::set_battle_params())]
        pub fn set_battle_params(
            origin: OriginFor<T>,
            params: BattleParameters<BalanceOf<T>>,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            
            // Validate parameters
            ensure!(params.max_turns > 0, Error::<T>::InvalidBattleParameters);
            ensure!(params.challenge_expiry_blocks > 0, Error::<T>::InvalidBattleParameters);
            ensure!(params.initial_energy > 0, Error::<T>::InvalidBattleParameters);
            ensure!(params.energy_per_turn > 0, Error::<T>::InvalidBattleParameters);
            ensure!(params.ultimate_move_energy_cost > 0, Error::<T>::InvalidBattleParameters);
            
            // Update parameters
            BattleParams::<T>::put(params);
            
            // Emit event
            Self::deposit_event(Event::BattleParametersUpdated);
            
            Ok(().into())
        }
        
        /// Enter matchmaking queue
        #[pallet::weight(T::WeightInfo::enter_matchmaking())]
        pub fn enter_matchmaking(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;
            
            // Ensure the owner owns the pet
            ensure!(
                T::NftManager::owner_of(&pet_id) == Some(owner.clone()),
                Error::<T>::NotPetOwner
            );
            
            // Ensure the pet is not already in a battle
            ensure!(
                !PetActiveBattle::<T>::contains_key(&pet_id),
                Error::<T>::PetAlreadyInBattle
            );
            
            // Ensure the pet is not already in the matchmaking queue
            ensure!(
                !BattleMatchmakingQueue::<T>::contains_key(&pet_id),
                Error::<T>::AlreadyInMatchmakingQueue
            );
            
            // Get or initialize pet battle stats
            let (wins, losses, draws, rating) = PetBattleStats::<T>::get(&pet_id);
            let rating = if rating == 0 { 1000 } else { rating }; // Default rating is 1000
            
            // Add pet to matchmaking queue
            let now = <frame_system::Pallet<T>>::block_number();
            BattleMatchmakingQueue::<T>::insert(&pet_id, (owner.clone(), rating, now));
            
            // Emit event
            Self::deposit_event(Event::PetAddedToMatchmaking(pet_id, owner, rating));
            
            // Try to find a match
            Self::try_matchmaking(pet_id)?;
            
            Ok(().into())
        }
        
        /// Leave matchmaking queue
        #[pallet::weight(T::WeightInfo::leave_matchmaking())]
        pub fn leave_matchmaking(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;
            
            // Ensure the owner owns the pet
            ensure!(
                T::NftManager::owner_of(&pet_id) == Some(owner.clone()),
                Error::<T>::NotPetOwner
            );
            
            // Ensure the pet is in the matchmaking queue
            ensure!(
                BattleMatchmakingQueue::<T>::contains_key(&pet_id),
                Error::<T>::NotInMatchmakingQueue
            );
            
            // Remove pet from matchmaking queue
            BattleMatchmakingQueue::<T>::remove(&pet_id);
            
            // Emit event
            Self::deposit_event(Event::PetRemovedFromMatchmaking(pet_id, owner));
            
            Ok(().into())
        }
        
        /// Get battle history
        #[pallet::weight(T::WeightInfo::get_battle_history())]
        pub fn get_battle_history(
            origin: OriginFor<T>,
            battle_id: BattleId,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            
            // Ensure the battle exists
            ensure!(
                Battles::<T>::contains_key(battle_id),
                Error::<T>::BattleNotFound
            );
            
            // The history is retrieved from storage and returned via an event
            // This is just a query function, so we don't need to modify any state
            
            Ok(().into())
        }
        
        /// Apply status effect
        #[pallet::weight(T::WeightInfo::apply_status_effect())]
        pub fn apply_status_effect(
            origin: OriginFor<T>,
            battle_id: BattleId,
            target_pet_id: PetId,
            effect: StatusEffect,
        ) -> DispatchResultWithPostInfo {
            let player = ensure_signed(origin)?;
            
            // Get the battle
            let mut battle = Self::battles(battle_id).ok_or(Error::<T>::BattleNotFound)?;
            
            // Ensure the battle is active
            ensure!(battle.status == BattleStatus::Active, Error::<T>::InvalidBattleStatus);
            
            // Determine whose turn it is
            let is_pet1_turn = battle.current_turn % 2 == 1;
            let (active_pet_id, active_owner) = if is_pet1_turn {
                (battle.pet1_id, battle.pet1_owner.clone())
            } else {
                (battle.pet2_id, battle.pet2_owner.clone())
            };
            
            // Ensure it's the player's turn
            ensure!(active_owner == player, Error::<T>::NotYourTurn);
            
            // Ensure the target pet is in the battle
            ensure!(
                battle.pet1_id == target_pet_id || battle.pet2_id == target_pet_id,
                Error::<T>::NotBattleParticipant
            );
            
            // Apply the status effect
            if target_pet_id == battle.pet1_id {
                ensure!(
                    battle.pet1_status_effects.len() < 5,
                    Error::<T>::TooManyStatusEffects
                );
                battle.pet1_status_effects.try_push(effect.clone()).map_err(|_| Error::<T>::TooManyStatusEffects)?;
            } else {
                ensure!(
                    battle.pet2_status_effects.len() < 5,
                    Error::<T>::TooManyStatusEffects
                );
                battle.pet2_status_effects.try_push(effect.clone()).map_err(|_| Error::<T>::TooManyStatusEffects)?;
            }
            
            // Update the battle
            battle.updated_at = <frame_system::Pallet<T>>::block_number();
            Battles::<T>::insert(battle_id, battle);
            
            // Emit event
            Self::deposit_event(Event::StatusEffectApplied(battle_id, target_pet_id, effect));
            
            Ok(().into())
        }
        
        /// Use ultimate move
        #[pallet::weight(T::WeightInfo::use_ultimate_move())]
        pub fn use_ultimate_move(
            origin: OriginFor<T>,
            battle_id: BattleId,
        ) -> DispatchResultWithPostInfo {
            let player = ensure_signed(origin)?;
            
            // Get the battle
            let mut battle = Self::battles(battle_id).ok_or(Error::<T>::BattleNotFound)?;
            
            // Ensure the battle is active
            ensure!(battle.status == BattleStatus::Active, Error::<T>::InvalidBattleStatus);
            
            // Determine whose turn it is
            let is_pet1_turn = battle.current_turn % 2 == 1;
            let (active_pet_id, active_owner, active_energy, target_pet_id) = if is_pet1_turn {
                (battle.pet1_id, battle.pet1_owner.clone(), battle.pet1_energy, battle.pet2_id)
            } else {
                (battle.pet2_id, battle.pet2_owner.clone(), battle.pet2_energy, battle.pet1_id)
            };
            
            // Ensure it's the player's turn
            ensure!(active_owner == player, Error::<T>::NotYourTurn);
            
            // Get battle parameters
            let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
            
            // Ensure the pet has enough energy
            ensure!(
                active_energy >= params.ultimate_move_energy_cost,
                Error::<T>::InsufficientEnergy
            );
            
            // Check for status effects that prevent actions
            if is_pet1_turn {
                for effect in &battle.pet1_status_effects {
                    match effect {
                        StatusEffect::Freeze(turns) | StatusEffect::Stun(turns) if *turns > 0 => {
                            return Err(Error::<T>::PreventedByStatusEffect.into());
                        },
                        _ => {},
                    }
                }
            } else {
                for effect in &battle.pet2_status_effects {
                    match effect {
                        StatusEffect::Freeze(turns) | StatusEffect::Stun(turns) if *turns > 0 => {
                            return Err(Error::<T>::PreventedByStatusEffect.into());
                        },
                        _ => {},
                    }
                }
            }
            
            // Execute the ultimate move (high damage based on pet stats)
            let pet_stats = T::PetManager::get_pet_attributes(&active_pet_id)
                .ok_or(Error::<T>::NotPetOwner)?;
            
            let strength = pet_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Strength { Some(*val) } else { None })
                .unwrap_or(50);
            
            let intelligence = pet_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Intelligence { Some(*val) } else { None })
                .unwrap_or(50);
            
            // Calculate damage based on both strength and intelligence
            let damage = 20 + (strength / 5) + (intelligence / 10);
            
            // Apply damage to target
            if target_pet_id == battle.pet1_id {
                battle.pet1_health = battle.pet1_health.saturating_sub(damage);
            } else {
                battle.pet2_health = battle.pet2_health.saturating_sub(damage);
            }
            
            // Consume energy
            if is_pet1_turn {
                battle.pet1_energy = battle.pet1_energy.saturating_sub(params.ultimate_move_energy_cost);
                battle.last_move_pet1 = Some(BattleMove::Ultimate);
            } else {
                battle.pet2_energy = battle.pet2_energy.saturating_sub(params.ultimate_move_energy_cost);
                battle.last_move_pet2 = Some(BattleMove::Ultimate);
            }
            
            // Record the move result
            let result = BattleMoveResult::Critical(damage);
            battle.last_move_result = Some(result.clone());
            
            // Add to battle history
            Self::add_to_battle_history(
                battle_id,
                BattleMoveHistoryEntry {
                    turn: battle.current_turn,
                    pet_id: active_pet_id,
                    move_type: BattleMove::Ultimate,
                    result: result.clone(),
                },
            )?;
            
            // Increment turn counter
            battle.current_turn += 1;
            battle.updated_at = <frame_system::Pallet<T>>::block_number();
            
            // Check if the battle is over
            if battle.pet1_health == 0 || battle.pet2_health == 0 {
                Self::finalize_battle(&mut battle)?;
            }
            
            // Update the battle
            Battles::<T>::insert(battle_id, battle);
            
            // Emit events
            Self::deposit_event(Event::UltimateMoveUsed(battle_id, active_pet_id));
            Self::deposit_event(Event::BattleMoveResult(battle_id, active_pet_id, result));
            
            Ok(().into())
        }
    }

    // Define hooks for the pallet
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Called at the beginning of a new block
        fn on_initialize(n: T::BlockNumber) -> Weight {
            // Check for tournaments that should start
            for (tournament_id, tournament) in Tournaments::<T>::iter() {
                if tournament.status == TournamentStatus::Registration && n >= tournament.start_block {
                    // Start the tournament
                    if tournament.current_participants >= 2 {
                        let mut updated_tournament = tournament;
                        updated_tournament.status = TournamentStatus::InProgress;
                        Tournaments::<T>::insert(tournament_id, updated_tournament);
                        Self::deposit_event(Event::TournamentStarted(tournament_id));
                    } else {
                        // Not enough participants, cancel the tournament
                        let mut updated_tournament = tournament;
                        updated_tournament.status = TournamentStatus::Cancelled;
                        Tournaments::<T>::insert(tournament_id, updated_tournament);
                        
                        // Refund entry fees
                        for (pet_id, participant) in TournamentParticipants::<T>::iter_prefix(tournament_id) {
                            T::Currency::transfer(
                                &Self::account_id(),
                                &participant,
                                updated_tournament.entry_fee,
                                ExistenceRequirement::KeepAlive,
                            ).ok();
                            PetActiveTournament::<T>::remove(pet_id);
                        }
                    }
                }
            }
            
            // Check for expired battle challenges
            let params = if let Some(p) = BattleParams::<T>::get() { p } else { return Weight::zero() };
            
            for (battle_id, battle) in Battles::<T>::iter() {
                if battle.status == BattleStatus::Challenged {
                    if n > battle.created_at + params.challenge_expiry_blocks.into() {
                        // Expire the challenge
                        let mut updated_battle = battle;
                        updated_battle.status = BattleStatus::Expired;
                        updated_battle.updated_at = n;
                        Battles::<T>::insert(battle_id, updated_battle.clone());
                        
                        // Remove pet active battles
                        PetActiveBattle::<T>::remove(&updated_battle.pet1_id);
                        PetActiveBattle::<T>::remove(&updated_battle.pet2_id);
                        
                        // Unreserve the challenger's bond
                        T::Currency::unreserve(&updated_battle.pet1_owner, params.challenge_bond);
                    }
                } else if battle.status == BattleStatus::Active {
                    // Process status effects for active battles
                    let mut updated_battle = battle;
                    
                    // Process status effects for both pets
                    if let Err(_) = Self::process_status_effects(&mut updated_battle, updated_battle.pet1_id) {
                        // If there's an error, just continue to the next battle
                        continue;
                    }
                    
                    if let Err(_) = Self::process_status_effects(&mut updated_battle, updated_battle.pet2_id) {
                        // If there's an error, just continue to the next battle
                        continue;
                    }
                    
                    // Add energy each turn (only at the start of a new turn)
                    if n > updated_battle.updated_at {
                        updated_battle.pet1_energy = (updated_battle.pet1_energy + params.energy_per_turn).min(100);
                        updated_battle.pet2_energy = (updated_battle.pet2_energy + params.energy_per_turn).min(100);
                    }
                    
                    // Check if the battle is over due to status effects
                    if updated_battle.pet1_health == 0 || updated_battle.pet2_health == 0 {
                        if let Err(_) = Self::finalize_battle(&mut updated_battle) {
                            // If there's an error, just continue to the next battle
                            continue;
                        }
                    }
                    
                    // Update the battle
                    Battles::<T>::insert(battle_id, updated_battle);
                }
            }
            
            // Process matchmaking queue
            // Every 10 blocks, try to match pets that have been waiting the longest
            if n % 10u32.into() == 0u32.into() {
                let mut processed = 0;
                let mut queue: Vec<(PetId, T::AccountId, u16, T::BlockNumber)> = Vec::new();
                
                // Collect all pets in the queue
                for (pet_id, (owner, rating, enqueue_time)) in BattleMatchmakingQueue::<T>::iter() {
                    queue.push((pet_id, owner, rating, enqueue_time));
                }
                
                // Sort by enqueue time (oldest first)
                queue.sort_by(|a, b| a.3.cmp(&b.3));
                
                // Process up to 10 pets
                for (pet_id, _, _, _) in queue.iter().take(10) {
                    if let Err(_) = Self::try_matchmaking(*pet_id) {
                        // If there's an error, just continue to the next pet
                        continue;
                    }
                    processed += 1;
                }
            }
            
            Weight::zero()
        }
    }

    // Define the genesis configuration for the pallet
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub battle_parameters: BattleParameters<BalanceOf<T>>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                battle_parameters: BattleParameters {
                    challenge_bond: BalanceOf::<T>::from(100u32),
                    forfeit_penalty: BalanceOf::<T>::from(50u32),
                    base_reward: BalanceOf::<T>::from(200u32),
                    challenge_expiry_blocks: 100,
                    max_turns: 10,
                    base_experience_reward: 100,
                    elemental_advantage_multiplier: Perbill::from_percent(25),
                    critical_hit_chance: Perbill::from_percent(15),
                    critical_hit_multiplier: Perbill::from_percent(200),
                    combo_threshold: 3,
                    combo_bonus_multiplier: Perbill::from_percent(150),
                    status_effect_duration: 3,
                    initial_energy: 50,
                    energy_per_turn: 10,
                    ultimate_move_energy_cost: 40,
                    matchmaking_rating_change: 25,
                },
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            BattleParams::<T>::put(&self.battle_parameters);
            BattleCount::<T>::put(0);
            TournamentCount::<T>::put(0);
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Get the next battle ID
        fn next_battle_id() -> Result<BattleId, Error<T>> {
            let battle_id = Self::battle_count();
            let next_id = battle_id.checked_add(1).ok_or(Error::<T>::BattleIdOverflow)?;
            Ok(battle_id)
        }
        
        /// Get the next tournament ID
        fn next_tournament_id() -> Result<TournamentId, Error<T>> {
            let tournament_id = Self::tournament_count();
            let next_id = tournament_id.checked_add(1).ok_or(Error::<T>::TournamentIdOverflow)?;
            Ok(tournament_id)
        }
        
        /// Add an entry to battle history
        fn add_to_battle_history(
            battle_id: BattleId,
            entry: BattleMoveHistoryEntry,
        ) -> Result<(), Error<T>> {
            BattleHistory::<T>::try_mutate(battle_id, |history| {
                history.try_push(entry).map_err(|_| Error::<T>::BattleHistoryTooLong)
            })
        }
        
        /// Try to find a match for a pet in the matchmaking queue
        fn try_matchmaking(pet_id: PetId) -> Result<(), Error<T>> {
            // Get the pet's rating
            let (pet_owner, pet_rating, _) = BattleMatchmakingQueue::<T>::get(&pet_id)
                .ok_or(Error::<T>::NotInMatchmakingQueue)?;
            
            // Find a suitable match
            let mut best_match: Option<(PetId, T::AccountId, u16, T::BlockNumber)> = None;
            let mut best_rating_diff = u16::MAX;
            
            for (queue_pet_id, (queue_owner, queue_rating, queue_time)) in BattleMatchmakingQueue::<T>::iter() {
                // Skip self
                if queue_pet_id == pet_id {
                    continue;
                }
                
                // Skip pets owned by the same player
                if queue_owner == pet_owner {
                    continue;
                }
                
                // Calculate rating difference
                let rating_diff = if queue_rating > pet_rating {
                    queue_rating - pet_rating
                } else {
                    pet_rating - queue_rating
                };
                
                // Find the closest match
                if rating_diff < best_rating_diff {
                    best_rating_diff = rating_diff;
                    best_match = Some((queue_pet_id, queue_owner.clone(), queue_rating, queue_time));
                }
            }
            
            // If a match is found, create a battle
            if let Some((match_pet_id, match_owner, match_rating, _)) = best_match {
                // Only match if rating difference is reasonable (within 200 points)
                if best_rating_diff <= 200 {
                    // Create the battle
                    let battle_id = Self::next_battle_id()?;
                    let now = <frame_system::Pallet<T>>::block_number();
                    let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
                    
                    // Get pet stats
                    let pet1_stats = T::PetManager::get_pet_attributes(&pet_id)
                        .ok_or(Error::<T>::NotPetOwner)?;
                    let pet2_stats = T::PetManager::get_pet_attributes(&match_pet_id)
                        .ok_or(Error::<T>::NotPetOwner)?;
                    
                    // Calculate initial health based on vitality
                    let pet1_health = pet1_stats.iter()
                        .find_map(|(attr, val)| if *attr == AttributeType::Vitality { Some(*val) } else { None })
                        .unwrap_or(50);
                    
                    let pet2_health = pet2_stats.iter()
                        .find_map(|(attr, val)| if *attr == AttributeType::Vitality { Some(*val) } else { None })
                        .unwrap_or(50);
                    
                    // Create empty status effect vectors
                    let pet1_status_effects: BoundedVec<StatusEffect, ConstU32<5>> = BoundedVec::default();
                    let pet2_status_effects: BoundedVec<StatusEffect, ConstU32<5>> = BoundedVec::default();
                    
                    let battle = Battle {
                        id: battle_id,
                        pet1_id: pet_id,
                        pet2_id: match_pet_id,
                        pet1_owner: pet_owner.clone(),
                        pet2_owner: match_owner.clone(),
                        status: BattleStatus::Active, // Matchmaking battles start immediately
                        current_turn: 1, // Pet1 goes first
                        pet1_health,
                        pet2_health,
                        pet1_energy: params.initial_energy,
                        pet2_energy: params.initial_energy,
                        pet1_status_effects,
                        pet2_status_effects,
                        last_move_pet1: None,
                        last_move_pet2: None,
                        last_move_result: None,
                        combo_counter_pet1: 0,
                        combo_counter_pet2: 0,
                        outcome: None,
                        created_at: now,
                        updated_at: now,
                        completed_at: None,
                        reward_claimed: false,
                        battle_rating: Some((pet_rating + match_rating) / 2), // Average rating
                    };
                    
                    // Store the battle
                    Battles::<T>::insert(battle_id, battle);
                    BattleCount::<T>::put(battle_id + 1);
                    
                    // Update pet active battles
                    PetActiveBattle::<T>::insert(&pet_id, battle_id);
                    PetActiveBattle::<T>::insert(&match_pet_id, battle_id);
                    
                    // Update account active battles
                    AccountActiveBattles::<T>::try_mutate(&pet_owner, |battles| {
                        battles.try_push(battle_id).map_err(|_| Error::<T>::TooManyActiveBattles)
                    })?;
                    AccountActiveBattles::<T>::try_mutate(&match_owner, |battles| {
                        battles.try_push(battle_id).map_err(|_| Error::<T>::TooManyActiveBattles)
                    })?;
                    
                    // Remove pets from matchmaking queue
                    BattleMatchmakingQueue::<T>::remove(&pet_id);
                    BattleMatchmakingQueue::<T>::remove(&match_pet_id);
                    
                    // Emit events
                    Self::deposit_event(Event::PetRemovedFromMatchmaking(pet_id, pet_owner.clone()));
                    Self::deposit_event(Event::PetRemovedFromMatchmaking(match_pet_id, match_owner.clone()));
                    Self::deposit_event(Event::MatchmakingBattleCreated(battle_id, pet_id, match_pet_id));
                }
            }
            
            Ok(())
        }
        
        /// Finalize a battle (determine outcome, update stats)
        fn finalize_battle(battle: &mut Battle<T::AccountId, T::BlockNumber>) -> Result<(), Error<T>> {
            // Ensure the battle is active
            ensure!(battle.status == BattleStatus::Active, Error::<T>::InvalidBattleStatus);
            
            // Determine the outcome
            let outcome = if battle.pet1_health == 0 && battle.pet2_health == 0 {
                BattleOutcome::Draw
            } else if battle.pet1_health == 0 {
                BattleOutcome::Pet2Win
            } else if battle.pet2_health == 0 {
                BattleOutcome::Pet1Win
            } else if battle.pet1_health > battle.pet2_health {
                BattleOutcome::Pet1Win
            } else if battle.pet2_health > battle.pet1_health {
                BattleOutcome::Pet2Win
            } else {
                BattleOutcome::Draw
            };
            
            // Update battle status
            battle.status = BattleStatus::Completed;
            battle.outcome = Some(outcome.clone());
            battle.completed_at = Some(battle.updated_at);
            
            // Get battle parameters
            let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
            
            // Award experience to pets
            let xp_reward = params.base_experience_reward;
            match outcome {
                BattleOutcome::Pet1Win => {
                    let _ = T::PetManager::add_experience(&battle.pet1_id, xp_reward);
                    let _ = T::PetManager::add_experience(&battle.pet2_id, xp_reward / 2);
                    
                    // Update battle stats
                    Self::update_battle_stats(battle.pet1_id, true, false, false, battle.battle_rating)?;
                    Self::update_battle_stats(battle.pet2_id, false, true, false, battle.battle_rating)?;
                },
                BattleOutcome::Pet2Win => {
                    let _ = T::PetManager::add_experience(&battle.pet2_id, xp_reward);
                    let _ = T::PetManager::add_experience(&battle.pet1_id, xp_reward / 2);
                    
                    // Update battle stats
                    Self::update_battle_stats(battle.pet1_id, false, true, false, battle.battle_rating)?;
                    Self::update_battle_stats(battle.pet2_id, true, false, false, battle.battle_rating)?;
                },
                BattleOutcome::Draw => {
                    let _ = T::PetManager::add_experience(&battle.pet1_id, xp_reward / 2);
                    let _ = T::PetManager::add_experience(&battle.pet2_id, xp_reward / 2);
                    
                    // Update battle stats
                    Self::update_battle_stats(battle.pet1_id, false, false, true, battle.battle_rating)?;
                    Self::update_battle_stats(battle.pet2_id, false, false, true, battle.battle_rating)?;
                },
                _ => {},
            }
            
            // Remove pet active battles
            PetActiveBattle::<T>::remove(&battle.pet1_id);
            PetActiveBattle::<T>::remove(&battle.pet2_id);
            
            Ok(())
        }
        
        /// Update battle stats for a pet
        fn update_battle_stats(
            pet_id: PetId,
            is_win: bool,
            is_loss: bool,
            is_draw: bool,
            battle_rating: Option<u16>,
        ) -> Result<(), Error<T>> {
            PetBattleStats::<T>::try_mutate(pet_id, |(wins, losses, draws, rating)| {
                if is_win {
                    *wins = wins.saturating_add(1);
                }
                if is_loss {
                    *losses = losses.saturating_add(1);
                }
                if is_draw {
                    *draws = draws.saturating_add(1);
                }
                
                // Update rating if available
                if let Some(battle_rating) = battle_rating {
                    let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
                    let old_rating = *rating;
                    
                    if is_win {
                        *rating = rating.saturating_add(params.matchmaking_rating_change);
                    } else if is_loss {
                        *rating = rating.saturating_sub(params.matchmaking_rating_change);
                    }
                    
                    // Ensure minimum rating of 100
                    if *rating < 100 {
                        *rating = 100;
                    }
                    
                    // Emit rating change event if it changed
                    if old_rating != *rating {
                        Self::deposit_event(Event::PetBattleRatingChanged(pet_id, old_rating, *rating));
                    }
                }
                
                Ok(())
            })
        }
        
        /// Process status effects for a pet
        fn process_status_effects(
            battle: &mut Battle<T::AccountId, T::BlockNumber>,
            pet_id: PetId,
        ) -> Result<(), Error<T>> {
            let is_pet1 = pet_id == battle.pet1_id;
            
            // Get the status effects
            let status_effects = if is_pet1 {
                &mut battle.pet1_status_effects
            } else {
                &mut battle.pet2_status_effects
            };
            
            // Process each status effect
            let mut i = 0;
            while i < status_effects.len() {
                let mut effect = status_effects[i].clone();
                let mut remove = false;
                
                match &mut effect {
                    StatusEffect::Burn(turns) => {
                        // Apply damage
                        if is_pet1 {
                            battle.pet1_health = battle.pet1_health.saturating_sub(5);
                        } else {
                            battle.pet2_health = battle.pet2_health.saturating_sub(5);
                        }
                        
                        // Decrement turns
                        *turns = turns.saturating_sub(1);
                        if *turns == 0 {
                            remove = true;
                            Self::deposit_event(Event::StatusEffectExpired(battle.id, pet_id, effect.clone()));
                        }
                    },
                    StatusEffect::Poison(turns) => {
                        // Apply damage
                        if is_pet1 {
                            battle.pet1_health = battle.pet1_health.saturating_sub(3);
                        } else {
                            battle.pet2_health = battle.pet2_health.saturating_sub(3);
                        }
                        
                        // Decrement turns
                        *turns = turns.saturating_sub(1);
                        if *turns == 0 {
                            remove = true;
                            Self::deposit_event(Event::StatusEffectExpired(battle.id, pet_id, effect.clone()));
                        }
                    },
                    StatusEffect::Freeze(turns) | StatusEffect::Stun(turns) => {
                        // Decrement turns
                        *turns = turns.saturating_sub(1);
                        if *turns == 0 {
                            remove = true;
                            Self::deposit_event(Event::StatusEffectExpired(battle.id, pet_id, effect.clone()));
                        }
                    },
                    StatusEffect::Strengthen(turns) | StatusEffect::Shield(turns) => {
                        // Decrement turns
                        *turns = turns.saturating_sub(1);
                        if *turns == 0 {
                            remove = true;
                            Self::deposit_event(Event::StatusEffectExpired(battle.id, pet_id, effect.clone()));
                        }
                    },
                }
                
                if remove {
                    status_effects.swap_remove(i);
                } else {
                    status_effects[i] = effect;
                    i += 1;
                }
            }
            
            Ok(())
        }
        
        /// Process a move from pet 1
        fn process_pet1_move(battle: &mut Battle<T::AccountId, T::BlockNumber>, move_type: &BattleMove) -> DispatchResult {
            // Get pet stats
            let pet1_stats = T::PetManager::get_pet_attributes(&battle.pet1_id)
                .ok_or(Error::<T>::NotPetOwner)?;
            let pet2_stats = T::PetManager::get_pet_attributes(&battle.pet2_id)
                .ok_or(Error::<T>::NotPetOwner)?;
            
            // Get strength and elemental values
            let pet1_strength = pet1_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Strength { Some(*val) } else { None })
                .unwrap_or(50);
            
            let pet1_elemental = pet1_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Elemental { Some(*val) } else { None })
                .unwrap_or(1);
            
            let pet2_elemental = pet2_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Elemental { Some(*val) } else { None })
                .unwrap_or(1);
            
            // Get randomness for move outcome
            let (random_seed, _) = T::BattleRandomness::random_seed();
            let random_value = (random_seed.as_ref()[0] % 100) as u8;
            
            // Process the move
            match move_type {
                BattleMove::Attack => {
                    // Basic attack: 5-15 damage based on strength
                    let base_damage = 5 + (pet1_strength / 10);
                    let damage = if random_value < 20 {
                        // Critical hit (20% chance)
                        base_damage * 2
                    } else {
                        base_damage
                    };
                    
                    battle.pet2_health = battle.pet2_health.saturating_sub(damage);
                },
                BattleMove::Defend => {
                    // Defend: Recover 5-10 health
                    let heal_amount = 5 + (random_value % 6);
                    battle.pet1_health = (battle.pet1_health + heal_amount).min(100);
                },
                BattleMove::SpecialAttack => {
                    // Special attack: High damage but can miss
                    if random_value < 70 {
                        // 70% chance to hit
                        let damage = 15 + (pet1_strength / 5);
                        battle.pet2_health = battle.pet2_health.saturating_sub(damage);
                    }
                },
                BattleMove::Heal => {
                    // Heal: Recover 10-20 health
                    let heal_amount = 10 + (random_value % 11);
                    battle.pet1_health = (battle.pet1_health + heal_amount).min(100);
                },
                BattleMove::Dodge => {
                    // Dodge: Small heal and increased chance to avoid next attack
                    let heal_amount = 3 + (random_value % 4);
                    battle.pet1_health = (battle.pet1_health + heal_amount).min(100);
                    // The dodge effect is handled in the next turn
                },
                BattleMove::ElementalAttack => {
                    // Elemental attack: Damage based on elemental advantage
                    let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
                    let base_damage = 10 + (pet1_strength / 8);
                    
                    // Calculate elemental advantage
                    let elemental_advantage = Self::calculate_elemental_advantage(pet1_elemental, pet2_elemental);
                    let damage = if elemental_advantage {
                        // Apply elemental advantage multiplier
                        params.elemental_advantage_multiplier.mul_floor(base_damage.into()) as u8
                    } else {
                        base_damage
                    };
                    
                    battle.pet2_health = battle.pet2_health.saturating_sub(damage);
                },
            }
            
            Ok(())
        }
        
        /// Process a move from pet 2
        fn process_pet2_move(battle: &mut Battle<T::AccountId, T::BlockNumber>, move_type: &BattleMove) -> DispatchResult {
            // Get pet stats
            let pet1_stats = T::PetManager::get_pet_attributes(&battle.pet1_id)
                .ok_or(Error::<T>::NotPetOwner)?;
            let pet2_stats = T::PetManager::get_pet_attributes(&battle.pet2_id)
                .ok_or(Error::<T>::NotPetOwner)?;
            
            // Get strength and elemental values
            let pet2_strength = pet2_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Strength { Some(*val) } else { None })
                .unwrap_or(50);
            
            let pet1_elemental = pet1_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Elemental { Some(*val) } else { None })
                .unwrap_or(1);
            
            let pet2_elemental = pet2_stats.iter()
                .find_map(|(attr, val)| if *attr == AttributeType::Elemental { Some(*val) } else { None })
                .unwrap_or(1);
            
            // Get randomness for move outcome
            let (random_seed, _) = T::BattleRandomness::random_seed();
            let random_value = (random_seed.as_ref()[0] % 100) as u8;
            
            // Check if pet1 used dodge in the previous turn
            let dodge_bonus = if let Some(BattleMove::Dodge) = battle.last_move_pet1 {
                30 // 30% additional chance to miss
            } else {
                0
            };
            
            // Process the move
            match move_type {
                BattleMove::Attack => {
                    // Basic attack: 5-15 damage based on strength
                    if random_value >= dodge_bonus {
                        let base_damage = 5 + (pet2_strength / 10);
                        let damage = if random_value < 20 {
                            // Critical hit (20% chance)
                            base_damage * 2
                        } else {
                            base_damage
                        };
                        
                        battle.pet1_health = battle.pet1_health.saturating_sub(damage);
                    }
                },
                BattleMove::Defend => {
                    // Defend: Recover 5-10 health
                    let heal_amount = 5 + (random_value % 6);
                    battle.pet2_health = (battle.pet2_health + heal_amount).min(100);
                },
                BattleMove::SpecialAttack => {
                    // Special attack: High damage but can miss
                    if random_value < (70 - dodge_bonus) {
                        // 70% chance to hit (reduced by dodge bonus)
                        let damage = 15 + (pet2_strength / 5);
                        battle.pet1_health = battle.pet1_health.saturating_sub(damage);
                    }
                },
                BattleMove::Heal => {
                    // Heal: Recover 10-20 health
                    let heal_amount = 10 + (random_value % 11);
                    battle.pet2_health = (battle.pet2_health + heal_amount).min(100);
                },
                BattleMove::Dodge => {
                    // Dodge: Small heal and increased chance to avoid next attack
                    let heal_amount = 3 + (random_value % 4);
                    battle.pet2_health = (battle.pet2_health + heal_amount).min(100);
                    // The dodge effect is handled in the next turn
                },
                BattleMove::ElementalAttack => {
                    // Elemental attack: Damage based on elemental advantage
                    if random_value >= dodge_bonus {
                        let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
                        let base_damage = 10 + (pet2_strength / 8);
                        
                        // Calculate elemental advantage
                        let elemental_advantage = Self::calculate_elemental_advantage(pet2_elemental, pet1_elemental);
                        let damage = if elemental_advantage {
                            // Apply elemental advantage multiplier
                            params.elemental_advantage_multiplier.mul_floor(base_damage.into()) as u8
                        } else {
                            base_damage
                        };
                        
                        battle.pet1_health = battle.pet1_health.saturating_sub(damage);
                    }
                },
            }
            
            Ok(())
        }
        
        /// Calculate elemental advantage
        fn calculate_elemental_advantage(attacker: u8, defender: u8) -> bool {
            // Simple elemental advantage calculation:
            // 1 (Fire) > 2 (Water) > 3 (Earth) > 1 (Fire)
            // 4 (Air) > 5 (Tech) > 6 (Nature) > 4 (Air)
            // 7 (Mystic) has no advantages or disadvantages
            // 0 (Neutral) has no advantages or disadvantages
            
            match (attacker % 8, defender % 8) {
                (1, 3) | (2, 1) | (3, 2) => true, // First cycle
                (4, 6) | (5, 4) | (6, 5) => true, // Second cycle
                _ => false,
            }
        }
        
        /// Calculate battle reward
        fn calculate_reward(battle: &Battle<T::AccountId, T::BlockNumber>) -> Result<BalanceOf<T>, Error<T>> {
            let params = Self::battle_parameters().ok_or(Error::<T>::InvalidBattleParameters)?;
            
            // Base reward plus any additional based on battle duration
            let base_reward = params.base_reward;
            let turns_bonus = BalanceOf::<T>::from((battle.current_turn as u32).min(params.max_turns as u32));
            
            Ok(base_reward.saturating_add(turns_bonus))
        }
        
        /// Get the account ID for the pallet
        pub fn account_id() -> T::AccountId {
            T::AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
                .expect("infinite length input; no invalid inputs for type; qed")
        }
    }

    // Implement BattleSystemIntegration trait
    impl<T: Config> BattleSystemIntegration<T::AccountId, PetId, BattleId> for Pallet<T> {
        /// Check if a pet is eligible for battle
        fn is_battle_eligible(pet_id: &PetId) -> bool {
            !PetActiveBattle::<T>::contains_key(pet_id)
        }
        
        /// Get pet battle stats
        fn get_battle_stats(pet_id: &PetId) -> Option<PetStats> {
            T::PetManager::get_enhanced_pet_info(pet_id).map(|info| info.stats)
        }
        
        /// Start a battle
        fn start_battle(
            initiator: &T::AccountId,
            pet_id: &PetId,
            opponent_pet_id: &PetId,
        ) -> Result<BattleId, DispatchError> {
            // This is a simplified version that delegates to the create_challenge function
            // In a real implementation, you would handle the logic here
            
            // Ensure the initiator owns the pet
            ensure!(
                T::NftManager::owner_of(pet_id) == Some(initiator.clone()),
                Error::<T>::NotPetOwner
            );
            
            // Ensure the pets are not already in battles
            ensure!(
                !PetActiveBattle::<T>::contains_key(pet_id),
                Error::<T>::PetAlreadyInBattle
            );
            ensure!(
                !PetActiveBattle::<T>::contains_key(opponent_pet_id),
                Error::<T>::PetAlreadyInBattle
            );
            
            // Create the battle
            let battle_id = Self::next_battle_id()?;
            
            // Return the battle ID
            Ok(battle_id)
        }
        
        /// Execute a battle move
        fn execute_battle_move(
            account: &T::AccountId,
            battle_id: &BattleId,
            move_id: u8,
        ) -> DispatchResult {
            // Get the battle
            let battle = Self::battles(*battle_id).ok_or(Error::<T>::BattleNotFound)?;
            
            // Ensure the battle is active
            ensure!(battle.status == BattleStatus::Active, Error::<T>::InvalidBattleStatus);
            
            // Ensure the account is a participant
            ensure!(
                battle.pet1_owner == *account || battle.pet2_owner == *account,
                Error::<T>::NotBattleParticipant
            );
            
            // Convert move_id to BattleMove
            let move_type = match move_id {
                0 => BattleMove::Attack,
                1 => BattleMove::Defend,
                2 => BattleMove::SpecialAttack,
                3 => BattleMove::Heal,
                4 => BattleMove::Dodge,
                5 => BattleMove::ElementalAttack,
                _ => BattleMove::Attack, // Default to Attack for invalid move_id
            };
            
            // Execute the move (simplified)
            // In a real implementation, you would handle the logic here
            
            Ok(())
        }
        
        /// Get battle outcome
        fn get_battle_outcome(battle_id: &BattleId) -> Option<(PetId, PetId, bool)> {
            Self::battles(*battle_id).and_then(|battle| {
                if battle.status == BattleStatus::Completed || battle.status == BattleStatus::Forfeited {
                    match battle.outcome {
                        Some(BattleOutcome::Pet1Win) => Some((battle.pet1_id, battle.pet2_id, true)),
                        Some(BattleOutcome::Pet2Win) => Some((battle.pet1_id, battle.pet2_id, false)),
                        _ => Some((battle.pet1_id, battle.pet2_id, false)), // Draw or other outcomes
                    }
                } else {
                    None
                }
            })
        }
        
        /// Get pet battle history
        fn get_pet_battle_history(pet_id: &PetId) -> Vec<BattleId> {
            let mut battle_ids = Vec::new();
            
            // Iterate through all battles to find those involving the pet
            for (battle_id, battle) in Battles::<T>::iter() {
                if battle.pet1_id == *pet_id || battle.pet2_id == *pet_id {
                    battle_ids.push(battle_id);
                }
            }
            
            battle_ids
        }
    }

    // Define the weight information trait
    pub trait WeightInfo {
        fn create_challenge() -> Weight;
        fn accept_challenge() -> Weight;
        fn decline_challenge() -> Weight;
        fn execute_move() -> Weight;
        fn forfeit_battle() -> Weight;
        fn claim_rewards() -> Weight;
        fn enter_tournament() -> Weight;
        fn create_tournament() -> Weight;
        fn set_battle_params() -> Weight;
        fn enter_matchmaking() -> Weight;
        fn leave_matchmaking() -> Weight;
        fn get_battle_history() -> Weight;
        fn apply_status_effect() -> Weight;
        fn use_ultimate_move() -> Weight;
    }
}