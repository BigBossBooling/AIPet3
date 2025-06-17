#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// We assume pallet_critter_nfts exposes its NftManager trait.
// In a real workspace, Cargo.toml for this pallet would depend on pallet-critter-nfts.

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::Currency,
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    // This import assumes that pallet_critter_nfts is a sibling crate/module
    // and NftManager is a public trait within it.
    // The actual path might differ based on project structure.
    // For this tool, we assume the type checker would find it if properly configured.
    // use pallet_critter_nfts::NftManager; // This would be the ideal way with correct dependencies.
    // use pallet_critter_nfts; // To access ElementType, or define locally/share via common types crate
    // use sp_std::vec::Vec; // For Vec in BattlePetStats

    pub type BattleId = u32;
    // PetId will come from T::PetId

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Conceptual struct for MvpBattlePetStats (inputs to off-chain simulation)
    // This illustrates the simplified data set NftHandler would need to provide for MVP.
    // pub struct MvpBattlePetStats<PetId, AccountId, ElementType> { // ElementType from pallet_critter_nfts
    //     pet_id: PetId,
    //     owner: AccountId,
    //     // Core attributes for MVP battle calculation:
    //     level: u32,
    //     base_strength: u8,
    //     base_agility: u8,
    //     base_vitality: u8, // Directly influences HP
    //     primary_elemental_affinity: Option<ElementType>, // Assuming ElementType from critter_nfts
    //     // Deferred for post-MVP: base_intelligence (for special moves), personality_traits, complex item effects.
    //     // current_mood: u8, // Could have a minor influence if desired for MVP (e.g. small % buff/debuff).
    // }


    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub enum BattleStatus {
        #[default]
        PendingMatch,
        Concluded,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct BattleDetails<AccountId, PetId> {
        pub player1: AccountId,
        pub pet1_id: PetId,
        pub player2: Option<AccountId>,
        pub pet2_id: Option<PetId>,
        pub status: BattleStatus,
        pub winner: Option<AccountId>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type PetId: Parameter + Member + Copy + MaybeSerializeDeserialize + MaxEncodedLen + Default + Ord;

        /// Handler for interacting with the NFT pallet (pallet-critter-nfts).
        type NftHandler: pallet_critter_nfts::NftManager<Self::AccountId, Self::PetId>; // Corrected: Removed DispatchResult generic from trait bound

        #[pallet::constant]
        type BattleRewardAmount: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_battle_id)]
    pub(super) type NextBattleId<T: Config> = StorageValue<_, BattleId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn battles)]
    pub(super) type Battles<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BattleId,
        BattleDetails<T::AccountId, T::PetId>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pet_in_battle)]
    /// Maps a PetId to an active BattleId to prevent multiple registrations.
    pub(super) type PetInBattle<T: Config> = StorageMap<_, Blake2_128Concat, T::PetId, BattleId>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BattleRegistered { battle_id: BattleId, player: T::AccountId, pet_id: T::PetId },
        BattleConcluded {
            battle_id: BattleId,
            winner_account: Option<T::AccountId>,
            winner_pet_id: Option<T::PetId>,
            loser_account: Option<T::AccountId>,
            loser_pet_id: Option<T::PetId>,
            reward_amount: Option<BalanceOf<T>>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        PetAlreadyInBattle,
        NotNftOwnerOrNftNotFound,
        NftNotEligibleForBattle, // e.g., if it's locked by marketplace or other means
        BattleIdOverflow,
        BattleNotFound,
        BattleAlreadyConcluded,
        InvalidBattleParticipants, // If reported winner/loser pets don't match battle
        NotAuthorizedToReportOutcome, // If reporter is not authorized
        RewardDistributionFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,3))] // Reads: PetInBattle, NftHandler. Writes: NextBattleId, Battles, PetInBattle
        pub fn register_for_battle(
            origin: OriginFor<T>,
            pet_id: T::PetId,
        ) -> DispatchResult {
            let player1 = ensure_signed(origin)?;

            // 1. Check if the pet is already registered in another active battle.
            ensure!(!PetInBattle::<T>::contains_key(&pet_id), Error::<T>::PetAlreadyInBattle);

            // 2. Verify ownership of the pet via NftHandler.
            let owner = T::NftHandler::owner_of(&pet_id)
                .ok_or(Error::<T>::NotNftOwnerOrNftNotFound)?; // Pet doesn't exist or no owner.
            ensure!(owner == player1, Error::<T>::NotNftOwnerOrNftNotFound); // Caller is not the owner.

            // 3. Check if the pet is eligible for battle (e.g., not locked by marketplace).
            // `is_transferable` implies it's not locked by other systems.
            ensure!(T::NftHandler::is_transferable(&pet_id), Error::<T>::NftNotEligibleForBattle);

            // 4. Generate a new BattleId.
            let battle_id = NextBattleId::<T>::try_mutate(|id| -> Result<BattleId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(1).ok_or(Error::<T>::BattleIdOverflow)?;
                Ok(current_id)
            })?;

            // 5. Create initial BattleDetails.
            // For MVP, a battle is registered by player1. Player2 would need a separate `join_battle` extrinsic (not in this subtask's scope).
            // So, player2 and pet2_id remain None initially.
            let battle_details = BattleDetails {
                player1: player1.clone(),
                pet1_id: pet_id,
                player2: None,         // Player2 joins via a separate action (e.g., join_battle extrinsic).
                pet2_id: None,         // Pet2 joins when Player2 joins.
                status: BattleStatus::PendingMatch,
                winner: None,          // Winner determined after outcome is reported.
            };

            // 6. Store the new battle and mark the pet as being in this battle.
            Battles::<T>::insert(battle_id, battle_details);
            PetInBattle::<T>::insert(&pet_id, battle_id);

            // 7. Emit event.
            Self::deposit_event(Event::BattleRegistered { battle_id, player: player1, pet_id });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,3) + T::DbWeight::get().writes(1))] // R:Battles, W:Battles,PetInBattle(x2), Currency
        pub fn report_battle_outcome(
            origin: OriginFor<T>,
            battle_id: BattleId,
            winner_pet_id: T::PetId,
            // loser_pet_id is inferred.
            // battle_log_hash is deferred for post-MVP.
        ) -> DispatchResult {
            let reporter = ensure_signed(origin)?;

            // 1. Mutate battle details directly using try_mutate_exists for safety.
            Battles::<T>::try_mutate_exists(battle_id, |battle_opt| -> DispatchResult {
                let battle = battle_opt.as_mut().ok_or(Error::<T>::BattleNotFound)?;

                // 2. Ensure battle is not already concluded.
                ensure!(battle.status != BattleStatus::Concluded, Error::<T>::BattleAlreadyConcluded);

                // 3. MVP Authority Check: Assume only player1 (initiator) can report the outcome.
                // A more robust system would involve oracles or consensus from both players.
                ensure!(reporter == battle.player1, Error::<T>::NotAuthorizedToReportOutcome);

                // 4. Determine winner and loser accounts and pet IDs.
                let mut winner_account_final: Option<T::AccountId> = None;
                let mut loser_account_final: Option<T::AccountId> = None;
                let mut actual_winner_pet_id_final: Option<T::PetId> = None; // To store the confirmed winner pet ID.
                let mut loser_pet_id_final: Option<T::PetId> = None;

                if battle.pet1_id == winner_pet_id {
                    winner_account_final = Some(battle.player1.clone());
                    actual_winner_pet_id_final = Some(battle.pet1_id);
                    // If player2 exists, they are the loser.
                    if let (Some(p2_acc), Some(p2_pet)) = (&battle.player2, battle.pet2_id) {
                        loser_account_final = Some(p2_acc.clone());
                        loser_pet_id_final = Some(p2_pet);
                    }
                    // If player2 is None (e.g., player1 won a conceptual solo challenge or opponent fled before registering pet2),
                    // loser_account and loser_pet_id remain None.
                } else if battle.pet2_id.is_some() && battle.pet2_id.unwrap() == winner_pet_id {
                    // pet2_id must exist and match winner_pet_id for player2 to be the winner.
                    winner_account_final = battle.player2.clone(); // battle.player2 must be Some.
                    actual_winner_pet_id_final = battle.pet2_id;
                    loser_account_final = Some(battle.player1.clone());
                    loser_pet_id_final = Some(battle.pet1_id);
                } else {
                    // Reported winner_pet_id does not match any known participant in the battle.
                    return Err(Error::<T>::InvalidBattleParticipants.into());
                }

                // 5. Update battle status and winner.
                battle.status = BattleStatus::Concluded;
                battle.winner = winner_account_final.clone();

                // 6. Distribute reward to the winner.
                let mut reward_given: Option<BalanceOf<T>> = None;
                if let Some(ref win_acc) = winner_account_final {
                    let reward_amount = T::BattleRewardAmount::get();
                    if reward_amount > BalanceOf::<T>::from(0u32) {
                        // Using deposit_creating with same caveats as in other pallets (MVP simplification).
                        // A more robust system might use a treasury or a dedicated rewards pool.
                        T::Currency::deposit_creating(win_acc, reward_amount);
                        // TODO: Consider map_err for RewardDistributionFailed if deposit_creating can error and needs handling.
                        reward_given = Some(reward_amount);
                    }
                }

                // 7. Clean up PetInBattle state for both pets involved.
                PetInBattle::<T>::remove(&battle.pet1_id);
                if let Some(pet2_id_val) = battle.pet2_id {
                    PetInBattle::<T>::remove(&pet2_id_val);
                }

                // 8. Emit event.
                Self::deposit_event(Event::BattleConcluded {
                    battle_id,
                    winner_account: winner_account_final,
                    winner_pet_id: actual_winner_pet_id_final,
                    loser_account: loser_account_final,
                    loser_pet_id: loser_pet_id_final,
                    reward_amount: reward_given,
                    // battle_log_hash: None, // Deferred for post-MVP.
                });
                Ok(())
            }) // End of try_mutate_exists
        }
    }

    // Separate impl block for conceptual helper functions (battle simulation logic is off-chain for MVP)
    impl<T: Config> Pallet<T> {
        // fn calculate_battle_simulation(
        //     pet1_stats: &MvpBattlePetStats<T::PetId, T::AccountId, pallet_critter_nfts::ElementType>,
        //     pet2_stats: &MvpBattlePetStats<T::PetId, T::AccountId, pallet_critter_nfts::ElementType>,
        //     random_seed: T::Hash,
        // ) -> (T::PetId, T::PetId) {

            // --- Conceptual Battle Logic (Simplified for MVP) ---
            // This simulation is illustrative. A real system would likely be off-chain.
            // Inputs: level, base_strength, base_agility, base_vitality, primary_elemental_affinity for each pet.
            // const MAX_BATTLE_TURNS: u32 = 50;

            // a. Calculate Effective HP, Attack, Defense, Speed for each pet for this battle instance
            //    - Effective HP = (pet_stats.base_vitality * VITALITY_HP_MULTIPLIER_CONST) + (pet_stats.level * LEVEL_HP_BONUS_CONST)
            //    - Effective Attack = (pet_stats.base_strength * STRENGTH_ATTACK_MULTIPLIER_CONST) + (pet_stats.level * LEVEL_ATTACK_BONUS_CONST)
            //    - Effective Defense = (pet_stats.base_vitality * VITALITY_DEF_MULTIPLIER_CONST) + (pet_stats.level * LEVEL_DEF_BONUS_CONST)
            //    - Effective Speed = pet_stats.base_agility // For simplicity, or add level bonus
            //    (Constants like VITALITY_HP_MULTIPLIER_CONST are part of the off-chain simulation's balancing)

            // b. Determine Attack Order: Higher `Effective Speed` attacks first. Use random_seed for ties.

            // c. Simulate Turns:
            //    In each turn:
            //    i. Hit Chance (Simplified for MVP):
            //       - `base_hit_chance = 85%` (Configurable constant for simulation)
            //       - `if (random_value_from_seed % 100) >= base_hit_chance { /* Miss */ }`
            //         (No complex accuracy vs evasion from agility for MVP's core calc).
            //
            //    ii. Damage Calculation (Simplified for MVP):
            //        - `damage = attacker.effective_attack.saturating_sub(defender.effective_defense / 2)`
            //          (Or another simple formula. Defense divisor can be a balancing constant).
            //        - `min_damage = 1` (Ensure at least 1 damage on hit).
            //
            //    iii. Apply Elemental Modifier (Core mechanic, keep):
            //         - Fetch elemental matchup (e.g., Fire vs. Nature: Fire deals 1.5x).
            //         - `damage = damage * elemental_multiplier;` (Using fixed-point math if necessary).
            //
            //    iv. Critical Hits & Personality Traits (Deferred for MVP simulation's core logic):
            //        These add layers of complexity. For MVP, the core simulation might omit these,
            //        or only include very simple, predefined effects if essential for basic balance.
            //        E.g., a "Strong Willed" trait giving +5% to defense could be factored into Effective Defense.
            //
            //    v. Apply Damage: `defender_hp -= damage;`
            //    vi. Check for Winner: If defender_hp <= 0.
            //    vii. Swap roles if no winner.

            // d. If MAX_BATTLE_TURNS reached, determine winner by HP percentage or other tie-breaker.

            // Placeholder return
            // (pet1_stats.pet_id, pet2_stats.pet_id)
        // }

        // fn get_elemental_multiplier(
        //     _attacker_affinity: &Option<pallet_critter_nfts::ElementType>,
        //     _defender_affinity: &Option<pallet_critter_nfts::ElementType>,
        // ) -> u64 {
        //     100
        // }
    }
}
