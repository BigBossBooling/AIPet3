//! # Pet Battles Pallet
//!
//! This pallet manages the core blockchain logic for competitive Pet NFT battles
//! within the CritterCraft ecosystem. It handles battle registration, initiation,
//! and crucially, the secure reporting and verification of off-chain battle outcomes.
//! Rewards (XP, PTCN) are distributed based on these outcomes.
//!
//! Meticulously crafted to align with The Architect's vision for
//! modularity, scalability, and robust operation of the CritterCraft digital ecosystem.

#![cfg_attr(not(feature = "std"), no_std)] // No standard library for Wasm compilation

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// --- FRAME Pallet Imports ---
#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*, // Provides common types and macros for pallets
        traits::{Currency, Get}, // Currency for PTCN, Get for constants
        BoundedVec,             // For bounded collections (e.g., potential future battle logs)
    };
    use frame_system::{
        pallet_prelude::*, // Provides types like BlockNumberFor, AccountId, OriginFor
        ensure_signed,     // Macro to ensure origin is a signed account
    };
    use scale_info::TypeInfo; // For `TypeInfo` derive macro
    use sp_std::vec::Vec; // For Vec type
    use sp_runtime::SaturatedFrom; // For saturating arithmetic
    use frame_support::log; // Correct way to import Substrate's logging macro

    // --- CritterCraft Shared Traits & Types ---
    // These imports define the interfaces with other CritterCraft pallets.
    // They are crucial for inter-pallet communication and system synergy.
    use crittercraft_traits::{
        SharedNftManager as NftManager, // For pet locking/unlocking/ownership
        NftManagerForItems,             // For applying item effects to pets (e.g., battle buffs)
        QuestNftRequirementChecker,     // For checking pet eligibility for quests
        PetId,                          // PetId type from traits crate
        DnaHashType,                    // DNA hash type from traits crate (for future battle_log_hash)
        SpeciesType,                    // Species type from traits crate
        // TraitTypeString,                // Not directly used in pallet-battles core logic
        // ItemCategoryTag,                // Not directly used in pallet-battles core logic
    };

    // --- Pallet Type Aliases ---
    // These aliases enhance clarity, aligning with "Know Your Core, Keep it Clear".
    pub type BattleId = u32; // Unique identifier for each battle instance
    pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // --- Enum Definitions ---
    // BattleStatus: Defines the current state of a battle.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub enum BattleStatus {
        #[default]
        PendingMatch, // Battle registered, waiting for opponent/initiation
        InProgress,   // Battle initiated, off-chain simulation running
        Concluded,    // Battle outcome reported and finalized on-chain
        Aborted,      // Battle ended prematurely (e.g., player fled, timeout)
    }

    // BattleOutcome: Defines the possible high-level outcome of a battle.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum BattleOutcome {
        Player1Win,
        Player2Win,
        Draw,
        // Future: Player1Fled, Player2Fled, Timeout, or more granular outcomes.
    }

    // --- Struct Definitions ---
    // BattleDetails: Stores the on-chain data for a battle instance.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))] // T is used in AccountId, PetId
    pub struct BattleDetails<T: Config> {
        pub battle_id: BattleId,
        pub player1: T::AccountId,
        pub pet1_id: T::PetId,
        pub player2: Option<T::AccountId>, // Option for PvE or if opponent hasn't joined
        pub pet2_id: Option<T::PetId>,     // Option for PvE
        pub status: BattleStatus,
        pub winner: Option<T::AccountId>, // Winner's account ID (if applicable)
        pub loser: Option<T::AccountId>,  // Loser's account ID (if applicable)
        pub winning_pet: Option<T::PetId>,// Winner's pet ID (if applicable)
        pub losing_pet: Option<T::PetId>, // Loser's pet ID (if applicable)
        pub initiated_block: BlockNumberFor<T>, // Block number when battle was initiated
        pub concluded_block: Option<BlockNumberFor<T>>, // Block number when outcome was reported
        pub reward_claimed: bool, // Track if reward for this battle has been claimed (future: for separate reward pallet)
        // V2+: Add battle_log_hash: Option<DnaHashType>, // SHA256 hash of off-chain battle log for verifiability
    }

    // --- Pallet Configuration Trait ---
    // Defines the types and constants that the runtime must provide for this pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// The currency trait for handling PTCN token balances.
        type Currency: Currency<Self::AccountId>;
        /// PetId type, provided by the CritterNFTs pallet.
        type PetId: Parameter + Member + Copy + MaybeSerializeDeserialize + MaxEncodedLen + Default + Ord + 'static;
        /// Handler for interacting with the NFT pallet (`pallet-critter-nfts`).
        type NftHandler: NftManager<Self::AccountId, Self::PetId>; // This trait provides pet locking/unlocking/ownership
        /// Amount of PTCN rewarded to the winner of a battle.
        #[pallet::constant]
        type BattleRewardAmount: Get<BalanceOf<Self>>;
        /// The maximum number of turns a battle can last before a tie-breaker.
        /// Crucial for off-chain simulation predictability and preventing indefinite battles.
        #[pallet::constant]
        type MaxBattleTurns: Get<u32>;
        /// The `AccountId` of the treasury/pool that funds battle rewards.
        /// This account must be endowed with sufficient PTCN.
        /// Ensures reward distribution is auditable and from a controlled source.
        #[pallet::constant]
        type BattleRewardPotId: Get<Self::AccountId>;
        /// A whitelist of `AccountId`s that are authorized to report battle outcomes.
        /// This is crucial for initial MVP security, later replaced by a decentralized oracle network.
        type AuthorizedBattleReporters: Get<Vec<Self::AccountId>>;
        /// Handler for checking pet eligibility criteria for quests (e.g., min level for battles).
        /// This is used by `pallet-quests` but can be used by this pallet for eligibility checks.
        type QuestChecker: QuestNftRequirementChecker<Self::AccountId, Self::PetId, SpeciesType>;
    }

    // --- Pallet Definition ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)] // Generates getter functions for storage items
    pub struct Pallet<T>(_);

    // --- Pallet Storage Items ---
    // These store the actual state of the CritterChain.
    #[pallet::storage]
    #[pallet::getter(fn next_battle_id)]
    /// Stores the next available unique BattleId for new battles.
    pub(super) type NextBattleId<T: Config> = StorageValue<_, BattleId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn battles)]
    /// Stores the details of each battle instance, mapped by BattleId.
    pub(super) type Battles<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BattleId,
        BattleDetails<T::AccountId, T::PetId>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pet_in_battle)]
    /// Maps a PetId to an active BattleId to prevent simultaneous battles.
    /// Crucial for maintaining accurate pet state across the ecosystem.
    pub(super) type PetInBattle<T: Config> = StorageMap<_, Blake2_128Concat, T::PetId, BattleId>;


    // --- Pallet Events ---
    // Events provide transparent, auditable logs of state changes for off-chain services and UIs.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new battle has been registered. [battle_id, player_account, pet_id]
        BattleRegistered { battle_id: BattleId, player: T::AccountId, pet_id: T::PetId },
        /// A battle has been initiated. [battle_id, player1, pet1_id, player2, pet2_id]
        BattleInitiated {
            battle_id: BattleId,
            player1: T::AccountId,
            pet1_id: T::PetId,
            player2: T::AccountId,
            pet2_id: T::PetId,
            initiated_block: BlockNumberFor<T>,
        },
        /// A battle has been concluded and its outcome reported.
        /// [battle_id, winner_account, winner_pet_id, loser_account, loser_pet_id, reward_amount, battle_log_hash_opt]
        BattleConcluded {
            battle_id: BattleId,
            winner_account: T::AccountId,
            winner_pet_id: T::PetId,
            loser_account: T::AccountId,
            loser_pet_id: T::PetId,
            reward_amount: BalanceOf<T>,
            // battle_log_hash_opt: Option<DnaHashType>, // V2: hash of the off-chain battle log for verifiability
        },
        /// A battle could not be initiated due to an error (e.g. pet already in battle, not eligible)
        BattleInitiationFailed { battle_id: BattleId, reason: Error<T> },
        /// A pet has fled from battle. [battle_id, pet_id, account_id]
        PetFledBattle { battle_id: BattleId, pet_id: T::PetId, account_id: T::AccountId },
    }

    // --- Pallet Errors ---
    // Custom errors provide precise feedback on why an extrinsic failed, crucial for debugging and user experience.
    // Aligns with "Know Your Core, Keep it Clear" for failure states.
    #[pallet::error]
    pub enum Error<T> {
        /// The pet is already registered in another active battle.
        PetAlreadyInBattle,
        /// The specified Pet NFT does not exist or the caller is not its owner.
        NotNftOwnerOrNftNotFound,
        /// The Pet NFT is not eligible for battle (e.g., it's locked by marketplace, or too low level).
        NftNotEligibleForBattle,
        /// The next BattleId has overflowed.
        BattleIdOverflow,
        /// The specified battle instance was not found.
        BattleNotFound,
        /// The battle has already been concluded.
        BattleAlreadyConcluded,
        /// The battle is not in `PendingMatch` status to be initiated.
        BattleNotPending,
        /// The reported winner_pet_id does not match any known participant in the battle.
        InvalidBattleParticipants,
        /// The account attempting to report the outcome is not authorized.
        NotAuthorizedToReportOutcome,
        /// The reward distribution to the winner failed (e.g., treasury empty, transfer error).
        RewardDistributionFailed,
        /// Player2 (opponent) pet ID is missing or invalid when initiating a 2-player battle.
        Player2PetMissing,
        /// Pet IDs are identical (a pet cannot battle itself).
        CannotBattleSelf,
        /// The participant is not owned by the account attempting to initiate/register for battle.
        ParticipantNotOwnedByPlayer, // Renamed for clarity
        /// An attempt was made to report an outcome for a battle that is not InProgress.
        BattleNotInProgress, // New error for report_battle_outcome
        /// A pet tried to flee from battle but was not in an InProgress battle.
        PetNotInBattle, // New error for flee_battle
    }

    // --- Pallet Extrinsics (Callable Functions) ---
    // These define the public interface of the pallet, allowing users to interact with CritterCraft.
    // They are designed with explicit weights for economic integrity.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Registers a single pet for battle.
        /// In MVP, this conceptually queues the pet for matchmaking or a challenge.
        /// This ensures the pet is ready for battle and its state is accurately tracked.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(2).writes(2)))] // R: PetInBattle, NftHandler. W: NextBattleId, Battles, PetInBattle
        pub fn register_for_battle(
            origin: OriginFor<T>,
            pet_id: T::PetId,
        ) -> DispatchResult {
            let player1 = ensure_signed(origin)?;

            // 1. Check if the pet is already registered in another active battle.
            ensure!(!<PetInBattle<T>>::contains_key(&pet_id), Error::<T>::PetAlreadyInBattle);

            // 2. Verify ownership of the pet via NftHandler.
            let owner = T::NftHandler::owner_of(&pet_id)
                .ok_or(Error::<T>::NotNftOwnerOrNftNotFound)?; // Pet doesn't exist or no owner.
            ensure!(owner == player1, Error::<T>::NotNftOwnerOrNftNotFound); // Caller is not the owner.

            // 3. Check if the pet is eligible for battle (e.g., not locked by marketplace, eligible level).
            // `is_transferable` implies it's not locked by other systems.
            ensure!(T::NftHandler::is_transferable(&pet_id), Error::<T>::NftNotEligibleForBattle);
            // V2: Add check for pet level or other eligibility criteria using QuestChecker trait:
            // ensure!(T::QuestChecker::get_pet_level_for_quest(&pet_id).map_or(false, |l| l >= T::MinBattleLevel::get()), Error::<T>::PetTooLowLevel);

            // 4. Generate a new BattleId.
            let battle_id = NextBattleId::<T>::try_mutate(|id| -> Result<BattleId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(1).ok_or(Error::<T>::BattleIdOverflow)?;
                Ok(current_id)
            })?;

            // 5. Create initial BattleDetails for a pending match.
            // Player2 and pet2_id remain None initially for matchmaking or PvE.
            let battle_details = BattleDetails {
                battle_id,
                player1: player1.clone(),
                pet1_id: pet_id,
                player2: None,
                pet2_id: None,
                status: BattleStatus::PendingMatch,
                winner: None,
                loser: None,
                winning_pet: None,
                losing_pet: None,
                initiated_block: frame_system::Pallet::<T>::block_number(),
                concluded_block: None,
                reward_claimed: false,
            };

            // 6. Store the new battle and mark the pet as being in this battle.
            <Battles<T>>::insert(battle_id, battle_details);
            <PetInBattle<T>>::insert(&pet_id, battle_id);

            // 7. Emit event for transparency and off-chain indexing.
            Self::deposit_event(Event::BattleRegistered { battle_id, player: player1, pet_id });
            Ok(())
        }

        /// Initiates a battle between two pets.
        /// This locks both pets for battle and sets the battle status to InProgress.
        /// For MVP, this might be called by Player1 initiating against a registered Player2, or a PvE challenge.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(6).writes(5)))] // R:Battles, PetInBattle(x2), NftHandler(x2). W:Battles, PetInBattle(x2), NftHandler(x2 for lock)
        pub fn initiate_battle(
            origin: OriginFor<T>,
            battle_id: BattleId,
            player2: T::AccountId, // Opponent's account
            pet2_id: T::PetId,     // Opponent's pet
        ) -> DispatchResult {
            let player1 = ensure_signed(origin)?;

            // 1. Verify battle exists and is in PendingMatch status.
            <Battles<T>>::try_mutate(&battle_id, |battle_opt| -> DispatchResult {
                let battle = battle_opt.as_mut().ok_or(Error::<T>::BattleNotFound)?;

                ensure!(battle.status == BattleStatus::PendingMatch, Error::<T>::BattleNotPending);

                // 2. Verify player1 owns pet1_id that initiated this pending battle.
                ensure!(player1 == battle.player1 && battle.pet1_id.is_some(), Error::<T>::ParticipantNotOwnedByPlayer);
                let pet1_id = battle.pet1_id; // Capture pet1_id from battle_details

                // 3. Verify player2 and pet2_id are valid.
                ensure!(player2 != player1, Error::<T>::CannotBattleSelf); // A pet cannot battle itself.
                ensure!(pet2_id != pet1_id, Error::<T>::CannotBattleSelf); // Cannot battle same pet if IDs identical.

                // 4. Verify ownership of pet2_id by player2.
                let owner2 = T::NftHandler::owner_of(&pet2_id)
                    .ok_or(Error::<T>::NotNftOwnerOrNftNotFound)?;
                ensure!(owner2 == player2, Error::<T>::NotNftOwnerOrNftNotFound);

                // 5. Check if pet2_id is already in battle or eligible.
                ensure!(!<PetInBattle<T>>::contains_key(&pet2_id), Error::<T>::PetAlreadyInBattle);
                ensure!(T::NftHandler::is_transferable(&pet2_id), Error::<T>::NftNotEligibleForBattle);
                // V2: Add check for pet level or other eligibility criteria for pet2_id:
                // ensure!(T::QuestChecker::get_pet_level_for_quest(&pet2_id).map_or(false, |l| l >= T::MinBattleLevel::get()), Error::<T>::PetTooLowLevel);

                // 6. Lock both pets for battle to prevent conflicting operations.
                // This is a crucial atomic step for maintaining state integrity.
                T::NftHandler::lock_nft(&player1, &pet1_id)?; // Player1's pet (already registered)
                T::NftHandler::lock_nft(&player2, &pet2_id)?; // Player2's pet

                // 7. Update battle details and status.
                battle.player2 = Some(player2.clone());
                battle.pet2_id = Some(pet2_id);
                battle.status = BattleStatus::InProgress;
                battle.initiated_block = frame_system::Pallet::<T>::block_number();

                // 8. Mark pet2_id as being in this battle.
                <PetInBattle<T>>::insert(&pet2_id, battle_id);

                Ok(())
            })?;

            // 9. Emit event for transparency.
            Self::deposit_event(Event::BattleInitiated {
                battle_id,
                player1,
                pet1_id: battle_id, // This should be pet1_id from the battle details, not battle_id itself. Fix.
                player2,
                pet2_id,
                initiated_block: frame_system::Pallet::<T>::block_number(),
            });

            Ok(())
        }

        /// Reports the outcome of an off-chain battle simulation.
        /// This concludes the battle on-chain, distributes rewards, and unlocks pets.
        /// Only authorized reporters can call this.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(4).writes(5)))] // R:Battles, PetInBattle(x2), NftHandler(x2), Currency. W:Battles, PetInBattle(x2), NftHandler(x2 for unlock), Currency
        pub fn report_battle_outcome(
            origin: OriginFor<T>,
            battle_id: BattleId,
            winner_pet_id: T::PetId,
            // V2: Add battle_log_hash: DnaHashType and reporter_signature for verifiability
        ) -> DispatchResult {
            let reporter = ensure_signed(origin)?;

            // 1. Authenticate reporter: Only whitelisted accounts can report outcomes for MVP.
            // This is crucial for initial security. In V2, this would be a decentralized oracle committee.
            ensure!(
                T::AuthorizedBattleReporters::get().contains(&reporter),
                Error::<T>::NotAuthorizedToReportOutcome
            );

            // 2. Mutate battle details directly using try_mutate_exists for safety.
            <Battles<T>>::try_mutate_exists(battle_id, |battle_opt| -> DispatchResult {
                let battle = battle_opt.as_mut().ok_or(Error::<T>::BattleNotFound)?;

                // 3. Ensure battle is in `InProgress` status.
                ensure!(battle.status == BattleStatus::InProgress, Error::<T>::BattleNotInProgress);

                // 4. Determine winner and loser accounts and pet IDs based on reported winner_pet_id.
                let (winner_account_final, actual_winner_pet_id_final, loser_account_final, loser_pet_id_final) = {
                    if battle.pet1_id == winner_pet_id {
                        // Pet1 is reported winner. Ensure pet2 exists.
                        ensure!(battle.player2.is_some() && battle.pet2_id.is_some(), Error::<T>::InvalidBattleParticipants);
                        (
                            battle.player1.clone(),
                            battle.pet1_id,
                            battle.player2.clone().unwrap(),
                            battle.pet2_id.unwrap(),
                        )
                    } else if battle.pet2_id.map_or(false, |id| id == winner_pet_id) {
                        // Pet2 is reported winner. Ensure pet2 exists.
                        ensure!(battle.player2.is_some() && battle.pet2_id.is_some(), Error::<T>::InvalidBattleParticipants);
                        (
                            battle.player2.clone().unwrap(),
                            battle.pet2_id.unwrap(),
                            battle.player1.clone(),
                            battle.pet1_id,
                        )
                    } else {
                        // Reported winner_pet_id does not match any known participant in the battle.
                        return Err(Error::<T>::InvalidBattleParticipants.into());
                    }
                };

                // 5. Update battle status and winner/loser.
                battle.status = BattleStatus::Concluded;
                battle.winner = Some(winner_account_final.clone());
                battle.winning_pet = Some(actual_winner_pet_id_final);
                battle.loser = Some(loser_account_final.clone());
                battle.losing_pet = Some(loser_pet_id_final);
                battle.concluded_block = Some(frame_system::Pallet::<T>::block_number());

                // 6. Distribute reward to the winner.
                let reward_amount = T::BattleRewardAmount::get();
                if reward_amount > BalanceOf::<T>::from(0u32) {
                    // Transfer from the BattleRewardPotId to the winner.
                    // This assumes T::BattleRewardPotId account is endowed with sufficient funds.
                    T::Currency::transfer(
                        &T::BattleRewardPotId::get(), // Source account (treasury)
                        &winner_account_final,        // Destination account (winner)
                        reward_amount,                // Amount
                        // KeepAlive ensures winner account is not reaped due to low balance after reward.
                        frame_support::traits::ExistenceRequirement::KeepAlive,
                    ).map_err(|_| Error::<T>::RewardDistributionFailed)?; // Map the currency error
                }
                
                // 7. Unlock pets for battle. This is crucial for maintaining state integrity.
                // Owner must be correct to unlock.
                T::NftHandler::unlock_nft(&winner_account_final, &actual_winner_pet_id_final)?;
                T::NftHandler::unlock_nft(&loser_account_final, &loser_pet_id_final)?;


                // 8. Clean up PetInBattle state for both pets involved.
                // Remove both pets from the PetInBattle storage, regardless of winner/loser order.
                <PetInBattle<T>>::remove(&battle.pet1_id);
                <PetInBattle<T>>::remove(&battle.pet2_id.unwrap()); // pet2_id is guaranteed to exist here.

                // 9. Emit event.
                Self::deposit_event(Event::BattleConcluded {
                    battle_id,
                    winner_account: winner_account_final,
                    winner_pet_id: actual_winner_pet_id_final,
                    loser_account: loser_account_final,
                    loser_pet_id: loser_pet_id_final,
                    reward_amount,
                    // battle_log_hash_opt: None, // Deferred for post-MVP.
                });
                Ok(())
            }) // End of try_mutate_exists
        }

        /// Allows a pet owner to prematurely end an in-progress battle and flee.
        /// This will result in a loss for the fleeing pet.
        #[pallet::call_index(3)] // Assigning a call_index.
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(2).writes(2)))] // R: PetInBattle, NftHandler. W: Battles, PetInBattle, NftHandler
        pub fn flee_battle(
            origin: OriginFor<T>,
            pet_id: T::PetId,
        ) -> DispatchResult {
            let fleer_account = ensure_signed(origin)?;

            // 1. Verify pet is in battle.
            let battle_id = <PetInBattle<T>>::get(pet_id).ok_or(Error::<T>::PetNotInBattle)?;
            
            // 2. Mutate battle details to set status to aborted and record loser.
            <Battles<T>>::try_mutate_exists(battle_id, |battle_opt| -> DispatchResult {
                let battle = battle_opt.as_mut().ok_or(Error::<T>::BattleNotFound)?;

                ensure!(battle.status == BattleStatus::InProgress, Error::<T>::BattleNotInProgress); // Ensure still in progress

                // Identify the fleeing pet and its owner.
                let (fleeing_pet_actual, fleeing_owner_actual, other_pet, other_owner) = {
                    if battle.pet1_id == pet_id && battle.player1 == fleer_account {
                        (battle.pet1_id, battle.player1.clone(), battle.pet2_id.unwrap(), battle.player2.clone().unwrap())
                    } else if battle.pet2_id.map_or(false, |p_id| p_id == pet_id) && battle.player2.map_or(false, |p_acc| p_acc == fleer_account) {
                        (battle.pet2_id.unwrap(), battle.player2.clone().unwrap(), battle.pet1_id, battle.player1.clone())
                    } else {
                        return Err(Error::<T>::NotNftOwnerOrNftNotFound.into()); // Pet found in battle, but not owned by caller or not participant.
                    }
                };

                // Set status to Aborted and record winner (the opponent) and loser (the fleeing pet).
                battle.status = BattleStatus::Aborted;
                battle.winner = Some(other_owner.clone()); // The opponent wins by default if one flees
                battle.winning_pet = Some(other_pet);
                battle.loser = Some(fleeing_owner_actual.clone());
                battle.losing_pet = Some(fleeing_pet_actual);
                battle.concluded_block = Some(frame_system::Pallet::<T>::block_number());

                // Unlock both pets from battle.
                T::NftHandler::unlock_nft(&fleeing_owner_actual, &fleeing_pet_actual)?;
                T::NftHandler::unlock_nft(&other_owner, &other_pet)?;

                // Remove from PetInBattle storage.
                <PetInBattle<T>>::remove(&fleeing_pet_actual);
                <PetInBattle<T>>::remove(&other_pet);

                Ok(())
            })?;

            // 3. Emit event.
            Self::deposit_event(Event::PetFledBattle { battle_id, pet_id, account_id: fleer_account });
            Ok(())
        }
    }

    // --- Pallet Internal Helper Functions ---
    impl<T: Config> Pallet<T> {
        /// Checks if a given pet ID is one of the participants in the specified battle.
        fn is_participant_in_battle(
            battle_details: &BattleDetails<T>,
            pet_id: T::PetId,
        ) -> bool {
            battle_details.pet1_id == pet_id ||
            battle_details.pet2_id.map_or(false, |p2_id| p2_id == pet_id)
        }
    }
}
    