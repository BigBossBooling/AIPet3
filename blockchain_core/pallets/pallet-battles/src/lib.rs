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
        type Currency: Currency<Self::AccountId>; // For battle rewards later
        type PetId: Parameter + Member + Copy + MaybeSerializeDeserialize + MaxEncodedLen + Default + Ord;

        // NftHandler for interacting with the NFT pallet.
        type NftHandler: pallet_critter_nfts::NftManager<Self::AccountId, Self::PetId, DispatchResult>;

        #[pallet::constant]
        type BattleRewardAmount: Get<BalanceOf<Self>>; // New constant for rewards
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

            // Check if pet is already in another battle
            ensure!(!PetInBattle::<T>::contains_key(&pet_id), Error::<T>::PetAlreadyInBattle);

            // Verify ownership and eligibility via NftHandler
            // owner_of returns Option<AccountId>
            let owner = T::NftHandler::owner_of(&pet_id).ok_or(Error::<T>::NotNftOwnerOrNftNotFound)?;
            ensure!(owner == player1, Error::<T>::NotNftOwnerOrNftNotFound);

            // is_transferable means not locked by marketplace or other logic in critter_nfts_pallet
            ensure!(T::NftHandler::is_transferable(&pet_id), Error::<T>::NftNotEligibleForBattle);

            // Generate new BattleId
            let battle_id = NextBattleId::<T>::try_mutate(|id| -> Result<BattleId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(1).ok_or(Error::<T>::BattleIdOverflow)?;
                Ok(current_id)
            })?;

            let battle_details = BattleDetails {
                player1: player1.clone(),
                pet1_id: pet_id,
                player2: None,
                pet2_id: None,
                status: BattleStatus::PendingMatch,
                winner: None,
            };

            Battles::<T>::insert(battle_id, battle_details);
            PetInBattle::<T>::insert(&pet_id, battle_id);

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

            let mut battle = Battles::<T>::get(&battle_id).ok_or(Error::<T>::BattleNotFound)?;
            ensure!(battle.status != BattleStatus::Concluded, Error::<T>::BattleAlreadyConcluded);

            // For MVP, player1 (the initiator) is authorized to report.
            ensure!(reporter == battle.player1, Error::<T>::NotAuthorizedToReportOutcome);

            // Infer loser_pet_id. This assumes a 1v1 battle context as per current BattleDetails.
            let inferred_loser_pet_id: Option<T::PetId>;
            if battle.pet1_id == winner_pet_id {
                inferred_loser_pet_id = battle.pet2_id;
            } else if battle.pet2_id.is_some() && battle.pet2_id.unwrap() == winner_pet_id {
                inferred_loser_pet_id = Some(battle.pet1_id);
            } else {
                // This means the reported winner_pet_id was not part of this battle
                // or pet2_id was None (which shouldn't happen if a winner involving pet2 is reported).
                return Err(Error::<T>::InvalidBattleParticipants.into());
            }

            // Ensure that if pet2_id was None (e.g. solo registration), winner must be pet1_id
            if battle.pet2_id.is_none() && winner_pet_id != battle.pet1_id {
                 return Err(Error::<T>::InvalidBattleParticipants.into());
            }

            let mut winner_account_final: Option<T::AccountId> = None;
            let mut loser_account_final: Option<T::AccountId> = None;
            // loser_pet_id_final is already inferred_loser_pet_id
            let mut actual_winner_pet_id_final: Option<T::PetId> = Some(winner_pet_id);
            let mut reward_given: Option<BalanceOf<T>> = None;

            if battle.pet1_id == winner_pet_id {
                winner_account_final = Some(battle.player1.clone());
                if let Some(p2_acc) = battle.player2.clone() { // player2 account might not exist if pet2_id was None
                    loser_account_final = Some(p2_acc);
                }
            } else if battle.pet2_id.is_some() && battle.pet2_id.unwrap() == winner_pet_id {
                winner_account_final = battle.player2.clone(); // player2 must exist if pet2_id is winner
                loser_account_final = Some(battle.player1.clone());
            }
            // If winner_pet_id was not found (already handled by InvalidBattleParticipants), this part is skipped.

            battle.status = BattleStatus::Concluded;
            battle.winner = winner_account_final.clone();

            // Distribute reward (fixed amount for MVP)
            if let Some(ref win_acc) = winner_account_final {
                let reward = T::BattleRewardAmount::get();
                if reward > BalanceOf::<T>::from(0u32) {
                    T::Currency::deposit_creating(win_acc, reward);
                    reward_given = Some(reward);
                }
            }

            Battles::<T>::insert(&battle_id, battle.clone());

            // Clean up PetInBattle state for both pets
            PetInBattle::<T>::remove(&battle.pet1_id);
            if let Some(pet2_id_val) = battle.pet2_id { // only remove if pet2 was actually in battle
                PetInBattle::<T>::remove(&pet2_id_val);
            }

            Self::deposit_event(Event::BattleConcluded {
                battle_id,
                winner_account: winner_account_final,
                winner_pet_id: actual_winner_pet_id_final,
                loser_account: loser_account_final,
                loser_pet_id: inferred_loser_pet_id, // Use inferred loser
                reward_amount: reward_given,
                // battle_log_hash: None, // Add when battle_log_hash param is re-introduced
            });

            Ok(())
        }
    }

    // Separate impl block for conceptual helper functions
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
