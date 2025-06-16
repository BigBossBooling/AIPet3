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

    pub type BattleId = u32;
    // PetId will come from T::PetId

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


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
}
