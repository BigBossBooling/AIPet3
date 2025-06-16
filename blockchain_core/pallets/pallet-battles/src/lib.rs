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

    // Conceptual struct for passing comprehensive pet stats to battle logic
    // This would be populated by fetching PetNft data and active item effects via NftHandler.
    // pub struct BattlePetStats<PetId, AccountId, ElementType> { // Made generic for ElementType
    //     pet_id: PetId,
    //     owner: AccountId,
    //     // Base Charter Attributes
    //     base_strength: u8,
    //     base_agility: u8,
    //     base_intelligence: u8,
    //     base_vitality: u8,
    //     primary_elemental_affinity: Option<ElementType>, // Assuming ElementType from critter_nfts
    //     // Dynamic Attributes
    //     level: u32,
    //     current_hp: u32, // Calculated from vitality, level, items for the battle instance
    //     current_mood: u8, // Mood might influence battle performance
    //     // Effective Combat Stats (derived from base, level, items, temporary effects)
    //     effective_attack: u32,
    //     effective_defense: u32,
    //     effective_speed: u32, // Determines attack order, evasion
    //     // Other relevant info
    //     personality_traits: Vec<Vec<u8>>,
    //     // equipped_items: Vec<u32 /*ItemId*/>, // If equipment provides passive bonuses
    //     // active_buffs_debuffs: Vec<u32 /*StatusEffectId*/>, // If items grant temporary effects
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
        ) -> DispatchResult {
            let reporter = ensure_signed(origin)?;

            let mut battle = Battles::<T>::get(&battle_id).ok_or(Error::<T>::BattleNotFound)?;
            ensure!(battle.status != BattleStatus::Concluded, Error::<T>::BattleAlreadyConcluded);

            // For MVP, player1 (the initiator) is authorized to report.
            // This would need to be more robust (e.g. oracle, both players agree, or signed game server report).
            ensure!(reporter == battle.player1, Error::<T>::NotAuthorizedToReportOutcome);

            // --- BATTLE LOGIC (Conceptual - Likely Off-Chain or complex on-chain) ---
            // The actual determination of `winner_pet_id` would come from a more complex system.
            // For this conceptual update, we assume `winner_pet_id` is correctly reported.
            // The logic below then assigns winner/loser accounts based on this.
            //
            // A full battle simulation would:
            // 1. Fetch full PetNft details for battle.pet1_id and battle.pet2_id (if applicable)
            //    This would be done via T::NftHandler, which would need a method like `get_pet_details(pet_id)`.
            //    Pet details would include:
            //    - Explicit on-chain charter attributes: base_strength, base_agility, etc.
            //    - Dynamic attributes: level, current_mood, current_energy.
            //    - Personality traits.
            //    - (Crucially) Any active temporary attribute boosts from consumed items
            //      or bonuses from equipped items. This implies NftHandler needs to expose these,
            //      or this pallet needs to be aware of `pallet-items`.
            //      (e.g., T::NftHandler::get_effective_stats(pet_id) -> EffectiveStats, or
            //       this pallet queries pallet-items for active effects on a pet).
            //
            // 2. The battle algorithm would use these comprehensive stats:
            //    - `effective_strength = base_strength + level_bonus + item_bonus_strength ...`
            //    - `effective_agility = base_agility + ...`
            //    - Elemental affinities (from PetNft.primary_elemental_affinity) would play a role.
            //    - Personality traits might give situational advantages/disadvantages.
            //    - Randomness (from a T::RandomnessSource if on-chain) for hit chances, critical hits, etc.
            //
            // 3. Simulate turns, damage calculation, status effects.
            //
            // 4. Determine the actual winner_pet_id based on this simulation.
            //    For now, `winner_pet_id` is an input to this extrinsic.

            // Before this extrinsic is called, the battle simulation (as conceptualized in calculate_battle_simulation)
            // would have occurred (e.g., off-chain, or via a trusted oracle that runs this logic).
            // The extrinsic then receives the `winner_pet_id` (and potentially `loser_pet_id`, `battle_log_hash`).

            // 1. Fetch PetNft details for winner_pet_id and (inferred) loser_pet_id using T::NftHandler
            //    This confirms their existence and ownership by players in the battle.
            //    This data would also be used if the on-chain extrinsic itself performed a simplified final check or applied on-chain effects.
            //    (e.g. `let winner_details = T::NftHandler::get_pet_details(&winner_pet_id).ok_or(...)?;`)

            // 2. (Future) If a battle_log_hash is provided, it could be stored on-chain for verifiability.
            //    `CurrentBattleLogHash::<T>::put(battle_id, battle_log_hash);`

            let mut winner_account_final: Option<T::AccountId> = None;
            let mut loser_account_final: Option<T::AccountId> = None;
            let mut loser_pet_id_final: Option<T::PetId> = None;
            let mut actual_winner_pet_id_final: Option<T::PetId> = None; // Store the actual winner pet id
            let mut reward_given: Option<BalanceOf<T>> = None;

            if battle.pet1_id == winner_pet_id {
                winner_account_final = Some(battle.player1.clone());
                actual_winner_pet_id_final = Some(battle.pet1_id);
                if let (Some(p2), Some(p2_pet)) = (battle.player2.clone(), battle.pet2_id) {
                    loser_account_final = Some(p2);
                    loser_pet_id_final = Some(p2_pet);
                }
            } else if battle.pet2_id.is_some() && battle.pet2_id.unwrap() == winner_pet_id {
                // player2 must exist if pet2_id is the winner
                winner_account_final = battle.player2.clone();
                actual_winner_pet_id_final = battle.pet2_id;
                loser_account_final = Some(battle.player1.clone());
                loser_pet_id_final = Some(battle.pet1_id);
            } else {
                // If winner_pet_id is not pet1_id and (player2 doesn't exist or winner_pet_id is not pet2_id)
                return Err(Error::<T>::InvalidBattleParticipants.into());
            }

            battle.status = BattleStatus::Concluded;
            battle.winner = winner_account_final.clone();

            // Distribute reward
            if let Some(ref win_acc) = winner_account_final {
                let reward = T::BattleRewardAmount::get();
                if reward > BalanceOf::<T>::from(0u32) {
                    // Using deposit_creating with same caveats as daily claim in critter_nfts_pallet.
                    // Assumes this pallet has a way to source/mint these funds.
                    T::Currency::deposit_creating(win_acc, reward);
                    // Note: A more robust implementation would handle potential errors from deposit_creating,
                    // or use a transfer from a pallet sovereign account, or use an Imbalance.
                    // For now, we assume success or internal panic if Currency can't fulfill.
                    reward_given = Some(reward);
                }
            }

            Battles::<T>::insert(&battle_id, battle.clone());

            // Clean up PetInBattle state for both pets
            PetInBattle::<T>::remove(&battle.pet1_id);
            if let Some(pet2_id_val) = battle.pet2_id {
                PetInBattle::<T>::remove(&pet2_id_val);
            }

            // 3. Distribute Rewards (already part of the extrinsic's existing logic)
            //    Rewards could be scaled based on level difference or Pet "rank" (future UserProfile synergy).

            // 4. (Future SYNERGY) Update PetNft stats (if battles grant XP or change personality traits)
            //    `T::NftHandler::grant_battle_xp(&winner_pet_id, XP_AMOUNT_PLACEHOLDER)?;`
            //    `T::NftHandler::update_personality_from_battle(&winner_pet_id, BattleOutcome::Win)?;` // Conceptual
            //    `if let Some(ref loser_id) = loser_pet_id_final { T::NftHandler::update_personality_from_battle(loser_id, BattleOutcome::Loss)?; }`

            // 5. (Future SYNERGY) Update UserProfile stats for players
            //    `if let Some(ref winner_acc) = winner_account_final { pallet_user_profile::Pallet::<T>::record_battle_win(winner_acc)?; }`
            //    `if let Some(ref loser_acc) = loser_account_final { pallet_user_profile::Pallet::<T>::record_battle_loss(loser_acc)?; }`


            Self::deposit_event(Event::BattleConcluded {
                battle_id,
                winner_account: winner_account_final,
                winner_pet_id: actual_winner_pet_id_final,
                loser_account: loser_account_final,
                loser_pet_id: loser_pet_id_final,
                reward_amount: reward_given,
            });

            Ok(())
        }
    }

    // Separate impl block for conceptual helper functions
    impl<T: Config> Pallet<T> {
        // fn calculate_battle_simulation(
        //     pet1_stats: &BattlePetStats<T::PetId, T::AccountId, pallet_critter_nfts::ElementType>, // Assume BattlePetStats defined above
        //     pet2_stats: &BattlePetStats<T::PetId, T::AccountId, pallet_critter_nfts::ElementType>,
        //     random_seed: T::Hash, // For RNG aspects
        // ) -> (T::PetId, T::PetId) { // Returns (winner_pet_id, loser_pet_id)

            // --- Conceptual Battle Logic ---
            // This simulation is illustrative. A real system could be far more complex and likely off-chain.
            // const MAX_BATTLE_TURNS: u32 = 50; // Example constant

            // 0. Initialize Battle State:
            //    - let mut pet1_current_hp = pet1_stats.current_hp;
            //    - let mut pet2_current_hp = pet2_stats.current_hp;
            //    - (Future) Apply pre-battle passive abilities or item effects.

            // 1. Determine Attack Order (Example):
            //    - Higher `effective_speed` attacks first. If equal, use randomness or a tie-breaker.
            //    - let (mut current_attacker_stats, mut current_defender_stats, mut current_attacker_hp, mut current_defender_hp) =
            //    -   if pet1_stats.effective_speed >= pet2_stats.effective_speed { // Handle ties with logic or randomness
            //    -       (pet1_stats, pet2_stats, pet1_current_hp, pet2_current_hp)
            //    -   } else {
            //    -       (pet2_stats, pet1_stats, pet2_current_hp, pet1_current_hp)
            //    -   };

            // 2. Simulate Turns (Max turns or until one pet's HP is <= 0):
            //    `for _turn in 0..MAX_BATTLE_TURNS { ... }`
            //    In each turn, attacker performs an action (e.g., basic attack).
            //    (Future: Pets could have multiple abilities/moves to choose from).

            //    a. Calculate Hit Chance (Example):
            //       - `let base_hit_chance: u8 = 90;` // Percentage
            //       - `let accuracy_factor = current_attacker_stats.effective_speed.saturating_div(current_defender_stats.effective_speed.max(1));` // Avoid div by zero
            //       - `let mut hit_chance = base_hit_chance.saturating_mul(accuracy_factor.min(2) as u8);` // Cap factor effect
            //       - `hit_chance = hit_chance.min(99);` // Max 99% hit
            //       // Use random_seed part for this turn's randomness
            //       // `if (random_value_from_seed_for_this_turn % 100) >= hit_chance { /* Miss */ /* Swap roles and continue */ }`

            //    b. Calculate Damage (Example):
            //       - `let base_damage = current_attacker_stats.effective_attack;`
            //       - `let defense_reduction = current_defender_stats.effective_defense / 2;` // example
            //       - `let raw_damage = base_damage.saturating_sub(defense_reduction);`
            //       - `let min_damage = 1u32;`
            //       - `let mut damage = raw_damage.max(min_damage);`

            //    c. Apply Elemental Modifier (Example):
            //       - `let elemental_multiplier = Self::get_elemental_multiplier(&current_attacker_stats.primary_elemental_affinity, &current_defender_stats.primary_elemental_affinity);` // e.g., Fire > Nature = 1.5x, Fire < Water = 0.75x (fixed point math needed)
            //       // `damage = (damage as u64 * elemental_multiplier_fixed_point / FIXED_POINT_DENOMINATOR) as u32;`

            //    d. Critical Hit Chance (Example):
            //       - `let crit_chance = 5u8.saturating_add(current_attacker_stats.effective_agility / 10);`
            //       // `if (random_value_from_seed_for_crit % 100) < crit_chance { damage = damage.saturating_mul(3) / 2; /* Critical Hit! 1.5x */ }`

            //    e. Personality Trait Influence (Example):
            //       - `if current_attacker_stats.personality_traits.contains(&"Brave".encode()) && current_attacker_hp < (current_attacker_stats.current_hp / 4) { damage = damage.saturating_mul(11) / 10; /* Brave last stand +10% */ }`
            //       // `if current_defender_stats.personality_traits.contains(&"Timid".encode()) && (random_value_from_seed_for_flinch % 10) == 0 { /* Timid pet might "flinch" and miss its next turn - conceptual */ }`

            //    f. Apply Damage:
            //       - `current_defender_hp = current_defender_hp.saturating_sub(damage);`
            //       - `if current_defender_hp == 0 { return (current_attacker_stats.pet_id, current_defender_stats.pet_id); }`

            //    g. Swap attacker/defender roles for next iteration of the turn.
            //       // `core::mem::swap(&mut current_attacker_stats, &mut current_defender_stats);`
            //       // `core::mem::swap(&mut current_attacker_hp, &mut current_defender_hp);`
            //    `}` // End of turn loop

            // 3. Determine Winner if MAX_BATTLE_TURNS reached:
            //    - // E.g., pet with higher remaining HP percentage wins.
            //    - // Or, if HP is equal, pet with higher total effective stats, or random.
            //    // `if pet1_current_hp * 100 / pet1_stats.current_hp.max(1) >= pet2_current_hp * 100 / pet2_stats.current_hp.max(1) { (pet1_stats.pet_id, pet2_stats.pet_id) } else { (pet2_stats.pet_id, pet1_stats.pet_id) }`

            // Placeholder return, actual logic above would return earlier.
            // (pet1_stats.pet_id, pet2_stats.pet_id) // Default or error case
        // }

        // fn get_elemental_multiplier(
        //     _attacker_affinity: &Option<pallet_critter_nfts::ElementType>,
        //     _defender_affinity: &Option<pallet_critter_nfts::ElementType>,
        // ) -> u64 { // Should return a fixed-point multiplier (e.g., 100 for 1.0x, 150 for 1.5x)
        //     100 // Placeholder for 1.0x
        // }
    }
}
