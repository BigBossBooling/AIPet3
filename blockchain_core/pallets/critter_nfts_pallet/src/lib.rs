#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// Definition of the NftManager trait
// This trait will be implemented by Pallet<T>
// It's defined here to be in the same crate as its implementor.
// In a multi-crate workspace, this might live in a shared traits crate,
// or be part of pallet-critter-nfts's public API for other pallets to depend on.
pub trait NftManager<AccountId, PetId, DispatchResult> {
    fn owner_of(pet_id: &PetId) -> Option<AccountId>;
    fn is_transferable(pet_id: &PetId) -> bool;
    fn lock_nft(owner: &AccountId, pet_id: &PetId) -> DispatchResult;
    fn unlock_nft(owner: &AccountId, pet_id: &PetId) -> DispatchResult;
    fn transfer_nft(from: &AccountId, to: &AccountId, pet_id: &PetId) -> DispatchResult;
}


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult, // Ensure DispatchResult is in scope for the trait impl
        pallet_prelude::*,
        traits::{Currency, Randomness},
    };
    use frame_system::pallet_prelude::*;
    // Import the NftManager trait from the parent module (crate level)
    use super::NftManager;
    use sp_std::vec::Vec; // For Vec<u8>
    use scale_info::TypeInfo; // For TypeInfo trait

    // Define PetId type alias for clarity
    pub type PetId = u32;

    // Add this enum definition, e.g., before the PetNft struct
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default, Copy)]
    pub enum ElementType {
        #[default]
        Neutral,
        Fire,
        Water,
        Earth,
        Air,
        Tech,
        Nature,
        Mystic,
    }

    // Define the PetNft struct
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    // #[scale_info(skip_type_params(T))] // Not needed if AccountId is not part of PetNft struct directly
    pub struct PetNft { // Removed AccountId generic here as it's not used in the struct fields
        pub id: PetId,
        pub dna_hash: [u8; 16], // 16 bytes for DNA hash
        pub initial_species: Vec<u8>,
        pub current_pet_name: Vec<u8>,
        // New Explicit Charter Attributes (Immutable after minting)
        pub base_strength: u8,
        pub base_agility: u8,
        pub base_intelligence: u8,
        pub base_vitality: u8,
        pub primary_elemental_affinity: Option<ElementType>, // Optional for neutrality
        // Existing mutable/dynamic attributes
        pub level: u32,
        pub experience_points: u32,
        pub mood_indicator: u8, // e.g., 0=Sad, 1=Neutral, 2=Happy, 3=Playful
        pub hunger_status: u8,  // Numerical value
        pub energy_status: u8,  // Numerical value
        pub personality_traits: Vec<Vec<u8>>, // New field for storing personality traits as Vec of strings (Vec<u8>)
        // pub owner: AccountId, // Considering if owner should be part of the struct or only in maps
        // SYNERGY: For battle champion status influencing breeding or other perks
        // pub battle_champion_eras: u32,
        // SYNERGY: For tracking if pet is equipped with items from pallet-items
        // pub equipped_item_slots: BoundedVec<Option<u32 /*ItemId from pallet-items*/>, ConstU32</*MaxEquippedItems*/>>,
        // CONCEPTUAL: For pet development lifecycle
        // pub last_tick_applied_block: BlockNumberFor<T>, // Requires PetNft to be generic over T or BlockNumberFor<T> to be accessible
    }

    // Conceptual struct for effective stats, returned by a helper
    // pub struct EffectivePetStats {
    //     pub strength: u8,
    //     pub agility: u8,
    //     // ... other stats
    // }

    // Type alias for balance, needs to be accessible by Config trait for DailyClaimAmount
    // So, it's better defined directly or within the pallet module but before Config.
    // For now, it's here, meaning `Config` will use `BalanceOf<Self>`
    pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency type for this pallet.
        type Currency: Currency<Self::AccountId>;

        /// Maximum number of pets an account can own.
        #[pallet::constant]
        type MaxOwnedPets: Get<u32>;

        /// Access to a source of randomness for DNA hash generation
        type PetRandomness: Randomness<Self::Hash, Self::BlockNumber>;

        /// The amount of PTCN to be claimed daily.
        #[pallet::constant]
        type DailyClaimAmount: Get<BalanceOf<Self>>; // Using BalanceOf<Self> which refers to Self::Currency

        /// The cooldown period (in blocks) for daily claims.
        #[pallet::constant]
        type ClaimCooldownPeriod: Get<Self::BlockNumber>;

        // SYNERGY: Conceptual handler for item interactions if this pallet manages them directly
        // type ItemHandler: pallet_items::ItemManager<Self::AccountId, u32 /*ItemId*/>;
        // Or this pallet calls pallet-items which then calls back via NftManagerForItems
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_pet_id)]
    pub(super) type NextPetId<T: Config> = StorageValue<_, PetId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_nfts)]
    pub(super) type PetNfts<T: Config> = StorageMap<_, Blake2_128Concat, PetId, PetNft>; // Changed PetNft<T::AccountId> to PetNft

    #[pallet::storage]
    #[pallet::getter(fn owner_of_pet)]
    // Stores a list of pet IDs owned by an account. BoundedVec ensures it doesn't grow indefinitely.
    pub(super) type OwnerOfPet<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<PetId, T::MaxOwnedPets>>;

    #[pallet::storage]
    #[pallet::getter(fn pet_nft_owner)]
    // Maps a PetId directly to its owner AccountId for quick lookups.
    pub(super) type PetNftOwner<T: Config> = StorageMap<_, Blake2_128Concat, PetId, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn locked_nfts)]
    /// Stores PetIds of NFTs that are currently locked (e.g., listed on marketplace).
    /// Using ValueQuery with () means we only care about the key's presence.
    pub(super) type LockedNfts<T: Config> = StorageMap<_, Blake2_128Concat, PetId, (), ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_claim_time)]
    /// Stores the block number of the last successful PTCN claim for each account.
    pub(super) type LastClaimTime<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::BlockNumber, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new Pet NFT has been minted. [owner, pet_id]
        PetNftMinted { owner: T::AccountId, pet_id: PetId },
        /// A Pet NFT has been transferred. [from, to, pet_id]
        PetNftTransferred { from: T::AccountId, to: T::AccountId, pet_id: PetId },
        /// A Pet NFT's metadata has been updated. [owner, pet_id]
        PetNftMetadataUpdated { owner: T::AccountId, pet_id: PetId },
        /// A user has successfully claimed their daily PTCN.
        DailyClaimMade { account: T::AccountId, amount: BalanceOf<T>, claim_time: T::BlockNumber },
        // SYNERGY / CONCEPTUAL: Events for pet development
        // PetFed { owner: T::AccountId, pet_id: PetId, food_item_id: u32 /*ItemId*/ },
        // PetPlayedWith { owner: T::AccountId, pet_id: PetId, toy_item_id: u32 /*ItemId*/ },
        // PetLeveledUp { pet_id: PetId, new_level: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The next PetId has overflowed.
        NextPetIdOverflow,
        /// An account cannot own more pets than MaxOwnedPets (when minting).
        ExceedMaxOwnedPets,
        /// The specified Pet NFT does not exist.
        PetNotFound, // Renamed from PetNftNotFound for consistency
        /// The sender is not the owner of the Pet NFT.
        NotOwner,
        /// The recipient of a transfer cannot hold more pets.
        RecipientExceedMaxOwnedPets,
        /// An attempt was made to transfer a pet to its current owner.
        CannotTransferToSelf,
        /// The NFT is already locked and cannot be locked again.
        NftAlreadyLocked,
        /// The NFT is not locked and thus cannot be unlocked.
        NftNotLocked,
        /// The NFT is locked and cannot be transferred by standard means.
        NftLocked,
        /// The cooldown period for claiming daily PTCN has not yet passed.
        ClaimCooldownNotMet,
        /// The attempt to reward the user with PTCN failed (e.g., currency issuance error).
        ClaimRewardFailed, // This error might be too generic depending on Currency trait used.
        // SYNERGY / CONCEPTUAL: Errors for pet development extrinsics
        // ItemNotFood,
        // ItemNotToy,
        // ItemInteractionFailed, // General error for when ItemHandler calls fail
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Mint a new Pet NFT.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(4).reads(1))] // Basic weight, adjust as needed
        pub fn mint_pet_nft(
            origin: OriginFor<T>,
            species: Vec<u8>,
            name: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Generate PetId
            let pet_id = NextPetId::<T>::try_mutate(|next_id| -> Result<PetId, DispatchError> {
                let current_id = *next_id;
                *next_id = next_id.checked_add(1).ok_or(Error::<T>::NextPetIdOverflow)?;
                Ok(current_id)
            })?;

            // Generate a simple DNA hash (placeholder - can be improved with randomness)
            // let dna_hash = T::PetRandomness::random(&species).0.into(); // Example using randomness
            let (dna_seed, _) = T::PetRandomness::random_seed();
            let dna_hash_data = (dna_seed, &sender, pet_id, &species, &name).encode();
            let dna_hash = frame_support::Hashable::blake2_128(&dna_hash_data);

            // --- Charter Attribute Derivation from dna_hash ---
            // This illustrative algorithm aims for a spread of values.
            // Each u8 in dna_hash ranges from 0-255.
            // Base stats range, e.g., 5-20 (16 possible values). Max u8 for stat is 255.
            // We can use modulo and scaling.

            // Example: Use pairs of bytes from dna_hash for more entropy per stat group.
            // Strength & Agility from first 4 bytes. Intelligence & Vitality from next 4.
            // Elemental Affinity from another byte.

            // Strength (5-20): dna_hash[0] & dna_hash[1]
            // Combine two bytes for a wider initial range (0-65535), then scale.
            let val_s = ((dna_hash[0] as u16) << 8 | dna_hash[1] as u16) % 100; // 0-99
            let base_strength = (5 + (val_s * 15) / 99) as u8; // Scales 0-99 to 0-15, then adds 5 -> 5-20

            // Agility (5-20): dna_hash[2] & dna_hash[3]
            let val_a = ((dna_hash[2] as u16) << 8 | dna_hash[3] as u16) % 100;
            let base_agility = (5 + (val_a * 15) / 99) as u8;

            // Intelligence (5-20): dna_hash[4] & dna_hash[5]
            let val_i = ((dna_hash[4] as u16) << 8 | dna_hash[5] as u16) % 100;
            let base_intelligence = (5 + (val_i * 15) / 99) as u8;

            // Vitality (5-20): dna_hash[6] & dna_hash[7]
            let val_v = ((dna_hash[6] as u16) << 8 | dna_hash[7] as u16) % 100;
            let base_vitality = (5 + (val_v * 15) / 99) as u8;

            // Primary Elemental Affinity: dna_hash[8] using Option<ElementType>
            let primary_elemental_affinity = match dna_hash[8] % 8 { // 7 defined types + None
                0 => Some(ElementType::Fire),
                1 => Some(ElementType::Water),
                2 => Some(ElementType::Earth),
                3 => Some(ElementType::Air),
                4 => Some(ElementType::Tech),
                5 => Some(ElementType::Nature),
                6 => Some(ElementType::Mystic),
                _ => None, // Represents Neutral or no strong affinity
            };

            // Create new PetNft instance
            let new_pet = PetNft {
                id: pet_id,
                dna_hash,
                initial_species: species.clone(),
                current_pet_name: name.clone(),
                base_strength,
                base_agility,
                base_intelligence,
                base_vitality,
                primary_elemental_affinity, // This is the Option<ElementType> version
                level: 1,
                experience_points: 0,
                mood_indicator: 100, // Example: Start at 100 (Happy/Content)
                hunger_status: 50,   // Example: Start at 50 (Not hungry, not full)
                energy_status: 100,  // Example: Start at 100 (Full energy)
                personality_traits: Vec::new(),
                // last_tick_applied_block: frame_system::Pallet::<T>::block_number(), // Initialize tick block
            };

            // Store the new PetNft
            PetNfts::<T>::insert(pet_id, new_pet);

            // Update ownership records
            OwnerOfPet::<T>::try_mutate(&sender, |owned_pets_vec| {
                owned_pets_vec.try_push(pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
            })?;

            PetNftOwner::<T>::insert(pet_id, sender.clone());

            // Deposit an event
            Self::deposit_event(Event::PetNftMinted { owner: sender, pet_id });

            Ok(())
        }

        /// Transfer a Pet NFT from the sender to a recipient.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(3).reads(2))] // Adjust weight
        pub fn transfer_pet_nft(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Ensure sender is not transferring to themselves
            ensure!(sender != recipient, Error::<T>::CannotTransferToSelf);

            // Check if the pet exists and sender is the owner
            let current_owner = PetNftOwner::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(current_owner == sender, Error::<T>::NotOwner);

            // Check if the NFT is locked
            ensure!(Self::is_transferable(&pet_id), Error::<T>::NftLocked);

            // Check if recipient can receive another pet
            OwnerOfPet::<T>::try_mutate(&recipient, |recipient_pets| {
                recipient_pets.try_push(pet_id).map_err(|_| Error::<T>::RecipientExceedMaxOwnedPets)
            })?;

            // Remove pet from sender's ownership list
            OwnerOfPet::<T>::try_mutate(&sender, |sender_pets| {
                if let Some(index) = sender_pets.iter().position(|&id| id == pet_id) {
                    sender_pets.swap_remove(index); // Efficient removal if order doesn't matter
                    Ok(())
                } else {
                    // This case should ideally not happen if PetNftOwner is consistent
                    // but as a safeguard or if logic changes, it's good to consider.
                    // For now, we assume PetNftOwner is the source of truth for ownership.
                    Err(Error::<T>::NotOwner) // Or a more specific error
                }
            })?;

            // Update the direct owner mapping
            PetNftOwner::<T>::insert(pet_id, recipient.clone());

            // Deposit an event
            Self::deposit_event(Event::PetNftTransferred { from: sender, to: recipient, pet_id });

            Ok(())
        }

        /// Update mutable metadata for a Pet NFT.
        /// Only the owner of the Pet NFT can perform this action.
        /// Fields set to `None` will not be updated.
        #[pallet::call_index(2)]
        #[pallet::weight(T::DbWeight::get().reads_writes(2, 1))] // Reads: PetNftOwner, PetNfts. Writes: PetNfts.
        pub fn update_pet_metadata(
            origin: OriginFor<T>,
            pet_id: PetId,
            name: Option<Vec<u8>>,
            level: Option<u32>,
            experience_points: Option<u32>,
            mood_indicator: Option<u8>,
            hunger_status: Option<u8>,
            energy_status: Option<u8>,
            personality_traits: Option<Vec<Vec<u8>>>, // New parameter
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Verify ownership
            let current_owner = PetNftOwner::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(current_owner == sender, Error::<T>::NotOwner);

            // Get the pet NFT and update its fields selectively
            PetNfts::<T>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
                let pet_nft = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

                if let Some(new_name) = name {
                    pet_nft.current_pet_name = new_name;
                }
                if let Some(new_level) = level {
                    // Add validation logic here if needed, e.g., level cannot decrease
                    pet_nft.level = new_level;
                }
                if let Some(new_xp) = experience_points {
                    pet_nft.experience_points = new_xp;
                }
                if let Some(new_mood) = mood_indicator {
                    pet_nft.mood_indicator = new_mood;
                }
                if let Some(new_hunger) = hunger_status {
                    pet_nft.hunger_status = new_hunger;
                }
                if let Some(new_energy) = energy_status {
                    pet_nft.energy_status = new_energy;
                }
                if let Some(new_traits) = personality_traits {
                    pet_nft.personality_traits = new_traits;
                }
                Ok(())
            })?;

            // Deposit an event
            Self::deposit_event(Event::PetNftMetadataUpdated { owner: sender, pet_id });

            Ok(())
        }

        /// Allows a user to claim their daily PTCN reward.
        #[pallet::call_index(3)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2))] // R: LastClaimTime, W: Currency, W: LastClaimTime
        pub fn claim_daily_ptcn(origin: OriginFor<T>) -> DispatchResult {
            let claimer = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();
            let last_claim_block = LastClaimTime::<T>::get(&claimer);

            // Check if the cooldown period has passed
            ensure!(
                current_block >= last_claim_block.saturating_add(T::ClaimCooldownPeriod::get()),
                Error::<T>::ClaimCooldownNotMet
            );

            let amount = T::DailyClaimAmount::get();

            // Reward the user with PTCN.
            // NOTE: This uses `deposit_creating` which is primarily for ensuring an account exists
            // and giving it an initial balance. If the account already exists, this specific call
            // might not be the most semantically correct for simply adding funds, depending on
            // the `Currency` trait implementation (e.g., pallet-balances).
            // A more robust system might involve this pallet having its own treasury/pot
            // from which to transfer, or having explicit minting capabilities.
            // For this subtask, we proceed with `deposit_creating` as a simplified mechanism.
            // Some `Currency` implementations might make `deposit_creating` a no-op or error if
            // the account already has > existential deposit.
            // A more robust approach for adding balance:
            // `T::Currency::make_free_balance_be(&claimer, T::Currency::free_balance(&claimer).saturating_add(amount)).map_err(|_| Error::<T>::ClaimRewardFailed)?;`
            // However, make_free_balance_be often requires special privileges.
            // We'll use `deposit_creating` and acknowledge its limitations for this example.
            // If `T::Currency` is `pallet-balances`, `deposit_creating` will ensure ED is met,
            // but won't simply add to an existing balance if it's already above ED.
            // A simple `T::Currency::transfer` from a pallet account would be better if one was set up.
            // For simplicity, let's assume deposit_creating has the desired effect of increasing balance
            // or that the `Currency` trait has an `issue_to` or `mint_into` method if this pallet had minting rights.
            // The chosen `deposit_creating` might not return a `DispatchResult`.
            // Let's use a placeholder for the actual rewarding mechanism which would need to be robust.
            // For the purpose of this exercise, we'll assume a successful deposit or a dedicated reward function.
            // This often translates to:
            // `T::Currency::deposit_creating(&claimer, amount);`
            // If the goal is to simply increase the balance, and `deposit_creating` doesn't do that for existing accounts,
            // this part of the logic would need refinement based on the specific `Currency` implementation.
            // For now, we assert the intent. A real implementation would need a funding mechanism.
            // A common pattern is for the pallet to hold funds in a sovereign account and transfer.
            // Or, if it's truly "minting", it needs that capability.
            // Let's proceed with a simplified `deposit_creating` and note the assumption.

            // Attempt to increase the free balance of the claimer.
            // This is a common way to mint/issue if the pallet has such capabilities
            // or if the Currency trait supports this kind of "deposit".
            // The exact mechanism can vary. `deposit_creating` ensures the account exists.
            // If it already exists, its behavior for just adding funds might differ.
            // We assume for this subtask that this results in the user getting the `amount`.
            T::Currency::deposit_creating(&claimer, amount);
            // A more robust check would be to verify the balance actually increased,
            // or use a currency method that returns a Result.
            // For instance, if `T::Currency` was `pallet_balances::Pallet<T>` this would be more complex.
            // We are relying on the simplicity of the `Currency` trait here.


            // Update the last claim time for the user
            LastClaimTime::<T>::insert(&claimer, current_block);

            // Emit an event
            Self::deposit_event(Event::DailyClaimMade {
                account: claimer,
                amount,
                claim_time: current_block,
            });

            Ok(())
        }

        // --- Conceptual Extrinsics for Pet Development ---

        // #[pallet::call_index(4)] // Ensure unique index, e.g., after claim_daily_ptcn
        // pub fn feed_pet(origin: OriginFor<T>, pet_id: PetId, food_item_id: u32 /*ItemId*/) -> DispatchResult {
        //     let owner = ensure_signed(origin)?;
        //     // 1. Verify owner owns pet_id (using Self::owner_of or PetNftOwner)
        //     //    ensure!(Self::owner_of_pet(&owner).map_or(false, |pets| pets.contains(&pet_id)), Error::<T>::NotOwner);
        //     // 2. Call T::ItemHandler::consume_item(&owner, food_item_id, ItemCategory::Food) // ItemCategory would be part of ItemHandler trait
        //     //    -> This would verify item ownership, type, and decrement from inventory.
        //     //    -> It could return item_effects (e.g., hunger_reduction, mood_boost).
        //     //    -> This requires T::Config to have an ItemHandler.
        //     // let food_effect = T::ItemHandler::consume_food_item(&owner, food_item_id).map_err(|_| Error::<T>::ItemInteractionFailed)?;

        //     // 3. Mutate PetNft:
        //     //    PetNfts::<T>::try_mutate(&pet_id, |pet_opt| -> DispatchResult {
        //     //        let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
        //     //        pet.hunger_status = pet.hunger_status.saturating_sub(food_effect.hunger_reduction).max(0); // Assuming food_effect struct
        //     //        pet.mood_indicator = pet.mood_indicator.saturating_add(food_effect.mood_boost).min(100);
        //     //        pet.experience_points = pet.experience_points.saturating_add(food_effect.xp_gain);
        //     //        Self::attempt_level_up(pet)?;
        //     //        // (Future) Potentially add a personality trait based on food type
        //     //        Ok(())
        //     //    })?;
        //     // Self::deposit_event(Event::PetFed { owner, pet_id, food_item_id });
        //     Ok(())
        // }

        // #[pallet::call_index(5)] // Ensure unique index
        // pub fn play_with_pet(origin: OriginFor<T>, pet_id: PetId, toy_item_id: u32 /*ItemId*/) -> DispatchResult {
        //     let owner = ensure_signed(origin)?;
        //     // 1. Verify owner owns pet_id
        //     // 2. Call T::ItemHandler::use_toy_item(&owner, toy_item_id) -> returns effects
        //     //    let toy_effect = T::ItemHandler::consume_toy_item(&owner, toy_item_id).map_err(|_| Error::<T>::ItemInteractionFailed)?;

        //     // 3. Mutate PetNft:
        //     //    PetNfts::<T>::try_mutate(&pet_id, |pet_opt| -> DispatchResult {
        //     //        let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
        //     //        pet.energy_status = pet.energy_status.saturating_sub(toy_effect.energy_cost).max(0); // Playing costs energy
        //     //        pet.mood_indicator = pet.mood_indicator.saturating_add(toy_effect.mood_increase).min(100);
        //     //        pet.hunger_status = pet.hunger_status.saturating_add(toy_effect.hunger_increase).min(100); // Playing makes hungry
        //     //        pet.experience_points = pet.experience_points.saturating_add(toy_effect.xp_gain);
        //     //        Self::attempt_level_up(pet)?;
        //     //        // (Future) Potentially add a personality trait based on toy type or play outcome
        //     //        Ok(())
        //     //    })?;
        //     // Self::deposit_event(Event::PetPlayedWith { owner, pet_id, toy_item_id });
        //     Ok(())
        // }
    }

    // SYNERGY: Public functions callable by other pallets
    // Separate impl block for organization is fine, or merge into the one with #[pallet::call]
    impl<T: Config> Pallet<T> {
        // --- Conceptual Internal Pet Development Functions ---

        // fn attempt_level_up(pet: &mut PetNft) -> DispatchResult { // PetNft needs to be generic if used in pallet struct
        //     // Define XP curve, e.g., next_level_xp = BASE_XP_PER_LEVEL * (pet.level ^ LEVEL_EXPONENT_FACTOR)
        //     // const BASE_XP_PER_LEVEL: u32 = 100; // Could be T::Config constant
        //     // const LEVEL_EXPONENT_FACTOR: f32 = 1.5; // Needs careful handling of floats or use integer math

        //     // Simplified integer curve: next_level_xp = 100 * pet.level + (50 * pet.level * pet.level / 10)
        //     let xp_needed_for_next_level = 100u32.saturating_mul(pet.level).saturating_add(
        //         50u32.saturating_mul(pet.level).saturating_mul(pet.level) / 10
        //     );

        //     if pet.experience_points >= xp_needed_for_next_level {
        //         pet.level = pet.level.saturating_add(1);
        //         pet.experience_points = pet.experience_points.saturating_sub(xp_needed_for_next_level); // Carry over excess XP

        //         // What happens on level up?
        //         // - Small permanent increase to dynamic stats (conceptual, if dynamic stats have a base separate from charter)
        //         // - OR, increase potential for dynamic stats.
        //         // - OR, grant skill points (future system).
        //         // - For now, just level up.
        //         // Self::deposit_event(Event::PetLeveledUp { pet_id: pet.id, new_level: pet.level });
        //     }
        //     Ok(())
        // }

        // Conceptual internal function for time-based state changes
        // fn apply_time_tick(pet_id: &PetId, blocks_since_last_tick: BlockNumberFor<T>) -> DispatchResult {
        //     PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
        //         let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

        //         // 1. Increase Hunger
        //         // Example: Hunger increases by 1 point every N blocks (e.g., 100 blocks)
        //         // let hunger_increase_rate: BlockNumberFor<T> = 100u32.into(); // Placeholder
        //         // let hunger_points_to_add = blocks_since_last_tick / hunger_increase_rate;
        //         // pet.hunger_status = pet.hunger_status.saturating_add(hunger_points_to_add as u8).min(100); // Max 100

        //         // 2. Decrease Energy
        //         // Example: Energy decreases by 1 point every M blocks (e.g., 50 blocks)
        //         // let energy_decrease_rate: BlockNumberFor<T> = 50u32.into(); // Placeholder
        //         // let energy_points_to_lose = blocks_since_last_tick / energy_decrease_rate;
        //         // pet.energy_status = pet.energy_status.saturating_sub(energy_points_to_lose as u8);

        //         // 3. Update Mood based on Hunger and Energy
        //         // if pet.hunger_status >= 80 { pet.mood_indicator = pet.mood_indicator.saturating_sub(20); } // Very hungry, mood drops
        //         // else if pet.hunger_status <= 20 { pet.mood_indicator = pet.mood_indicator.saturating_add(10); } // Well fed
        //         // if pet.energy_status <= 20 { pet.mood_indicator = pet.mood_indicator.saturating_sub(20); } // Very tired
        //         // else if pet.energy_status >= 80 { pet.mood_indicator = pet.mood_indicator.saturating_add(10); } // Energetic
        //         // pet.mood_indicator = pet.mood_indicator.clamp(0, 100); // Mood 0-100

        //         // 4. (Future) Personality Trait Evolution based on long-term state or random events
        //         // if pet.mood_indicator < 10 for X consecutive ticks { pet.personality_traits.push("Grumpy".into()); }

        //         // 5. Update Pet's last_tick_applied_block or similar to track this.
        //         // pet.last_tick_applied_block = frame_system::Pallet::<T>::block_number();
        //         Ok(())
        //     })
        // }

        // // SYNERGY: Function to be called by pallet-battles to update champion status
        // pub fn set_battle_champion_status(pet_id: &PetId, eras: u32) -> DispatchResult {
        //     // Ensure pet exists, then update a new field PetNft.battle_champion_eras
        //     // PetNfts::<T>::try_mutate(...)
        //     Ok(())
        // }

        // // SYNERGY: Functions for pallet-items to equip/unequip items
        // // These would require ItemId type, MaxEquippedItems const in Config, and equipped_item_slots in PetNft
        // pub fn equip_item_to_pet(owner: &T::AccountId, pet_id: &PetId, item_id: u32 /*ItemId*/, slot_index: u8) -> DispatchResult {
        //     // Verify ownership, pet exists, item exists (via a trait call to pallet-items perhaps), slot_index valid
        //     // Update PetNft.equipped_item_slots[slot_index] = Some(item_id);
        //     // Potentially apply item's permanent effects if it's equipment with on-equip stat boosts
        //     Ok(())
        // }
        // pub fn unequip_item_from_pet(owner: &T::AccountId, pet_id: &PetId, slot_index: u8) -> DispatchResult {
        //     // Verify ownership, pet exists, slot_index valid and has an item
        //     // PetNft.equipped_item_slots[slot_index] = None;
        //     // Potentially remove item's permanent effects
        //     Ok(())
        // }

        // // SYNERGY: Function for pallet-battles or pallet-items to get current effective stats
        // // This would read base stats, level, mood, and factor in equipped items.
        // pub fn get_effective_pet_stats(pet_id: &PetId) -> Option<super::EffectivePetStats> { // Assuming EffectivePetStats is defined above
        //     // Fetch PetNft
        //     // Fetch equipped items (e.g., from PetNft.equipped_item_slots)
        //     // For each equipped item, fetch its stat boosts (would need a trait call to pallet-items)
        //     // Calculate effective stats based on base stats, level, mood, item boosts.
        //     // Return Some(EffectivePetStats { ... }) or None if pet not found
        //     None // Placeholder
        // }

        // // SYNERGY: Helper for pallet-breeding to get all relevant data for parent, including charter stats
        // pub fn get_pet_breeding_data(pet_id: &PetId) -> Option<PetNft> { // Could return a more specific struct
        //     Self::pet_nfts(pet_id) // Returns the whole PetNft struct which now includes charter attributes
        // }
    }
}

// Implementation of the NftManager trait for our Pallet
impl<T: Config> NftManager<T::AccountId, PetId, DispatchResult> for Pallet<T> {
    fn owner_of(pet_id: &PetId) -> Option<T::AccountId> {
        // Uses the getter defined for PetNftOwner storage map
        Self::pet_nft_owner(pet_id)
    }

    fn is_transferable(pet_id: &PetId) -> bool {
        // An NFT is transferable if it's NOT in LockedNfts.
        !LockedNfts::<T>::contains_key(pet_id)
    }

    fn lock_nft(owner: &T::AccountId, pet_id: &PetId) -> DispatchResult {
        // Verify owner actually owns pet_id
        let current_owner = Self::pet_nft_owner(pet_id).ok_or(Error::<T>::PetNotFound)?;
        ensure!(current_owner == *owner, Error::<T>::NotOwner);

        // Ensure it's not already locked
        ensure!(!LockedNfts::<T>::contains_key(pet_id), Error::<T>::NftAlreadyLocked);

        // Insert into locked set
        LockedNfts::<T>::insert(pet_id, ());
        Ok(())
    }

    fn unlock_nft(owner: &T::AccountId, pet_id: &PetId) -> DispatchResult {
        // Verify owner actually owns pet_id. This check ensures that only the entity
        // that has control over the owner (like the marketplace acting on seller's behalf
        // or the owner themself) can initiate an unlock.
        let current_owner = Self::pet_nft_owner(pet_id).ok_or(Error::<T>::PetNotFound)?;
        ensure!(current_owner == *owner, Error::<T>::NotOwner);

        // Ensure it IS locked
        ensure!(LockedNfts::<T>::contains_key(pet_id), Error::<T>::NftNotLocked);

        // Remove from locked set
        LockedNfts::<T>::remove(pet_id);
        Ok(())
    }

    fn transfer_nft(from: &T::AccountId, to: &T::AccountId, pet_id: &PetId) -> DispatchResult {
        // 1. Verify 'from' is the current owner
        let current_owner = Self::pet_nft_owner(pet_id).ok_or(Error::<T>::PetNotFound)?;
        ensure!(current_owner == *from, Error::<T>::NotOwner);

        // This simplified transfer does not check for `is_transferable` (i.e. !is_locked).
        // It's assumed the caller (e.g., marketplace) handles the lock state appropriately
        // (e.g., calls unlock_nft just before or as part of the sale transaction flow,
        // or the lock is specific to marketplace logic and doesn't prevent underlying transfer by authorized pallet).
        // For a direct inter-pallet call like this, the NFT should ideally be in a state
        // where it *can* be transferred (e.g. marketplace has called unlock_nft).

        // 2. Update OwnerOfPet for the sender ('from')
        OwnerOfPet::<T>::try_mutate(from, |owned_pets_vec| {
            if let Some(index) = owned_pets_vec.iter().position(|&id| id == *pet_id) {
                owned_pets_vec.swap_remove(index);
                Ok(())
            } else {
                // This indicates an inconsistency if current_owner was correct.
                Err(Error::<T>::PetNotFound)
            }
        })?;

        // 3. Update OwnerOfPet for the recipient ('to')
        OwnerOfPet::<T>::try_mutate(to, |owned_pets_vec| {
            owned_pets_vec.try_push(*pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
        })?;

        // 4. Update PetNftOwner mapping
        PetNftOwner::<T>::insert(pet_id, to.clone());

        // Note: No event is emitted here. The calling context (e.g., marketplace)
        // is responsible for emitting its own relevant event (e.g., NftSold).
        // The main `transfer_pet_nft` extrinsic in this pallet would still emit `PetNftTransferred`.
        Ok(())
    }
}

// SYNERGY: Conceptual Implementation of NftManagerForItems trait (defined in pallet-items)
// use pallet_items; // This would be an actual import if pallet-items was a dependency

// impl<T: Config + pallet_items::Config> pallet_items::NftManagerForItems<
//     T::AccountId,
//     PetId, // Assuming PetId = u32 is consistent
//     pallet_items::PetAttributeType, // Type defined in pallet-items
//     Vec<u8>, // TraitType for personality_traits
//     BlockNumberFor<T>, // BlockNumber type consistent with runtime
//     DispatchResult
// > for Pallet<T>
// {
//     fn get_pet_owner(pet_id: &PetId) -> Option<T::AccountId> {
//         Self::pet_nft_owner(pet_id)
//     }

//     fn apply_attribute_boost_to_pet(
//         _caller: &T::AccountId, // Ownership already verified by pallet-items before calling this
//         pet_id: &PetId,
//         attribute: pallet_items::PetAttributeType,
//         value: i16,
//         _is_percentage: bool, // Percentage logic would be more complex here
//         is_permanent: bool,
//         _duration_blocks: Option<BlockNumberFor<T>>, // Temporary buffs require more infrastructure
//     ) -> DispatchResult {
//         PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
//             let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

//             if !is_permanent {
//                 // CONCEPTUAL: Temporary buffs would need a new storage item like:
//                 // TemporaryBuffs: StorageMap<(PetId, pallet_items::PetAttributeType), (value: i16, expiry_block: BlockNumberFor<T>)>
//                 // For now, we'll only conceptualize permanent effects or direct modifications.
//                 // return Err(Error::<T>::TemporaryEffectsNotImplementedYet.into()); // Placeholder
//             }

//             match attribute {
//                 pallet_items::PetAttributeType::ExperiencePoints => {
//                     if value > 0 {
//                         pet.experience_points = pet.experience_points.saturating_add(value as u32);
//                         // Self::attempt_level_up(pet)?; // Call internal helper
//                     }
//                 },
//                 pallet_items::PetAttributeType::MoodIndicator => {
//                     pet.mood_indicator = (pet.mood_indicator as i16).saturating_add(value).clamp(0, 100) as u8;
//                 },
//                 pallet_items::PetAttributeType::HungerStatus => {
//                     pet.hunger_status = (pet.hunger_status as i16).saturating_add(value).clamp(0, 100) as u8;
//                 },
//                 pallet_items::PetAttributeType::EnergyStatus => {
//                     pet.energy_status = (pet.energy_status as i16).saturating_add(value).clamp(0, 100) as u8;
//                 },
//                 // IMPORTANT: Modifying base charter attributes should be highly restricted or disallowed
//                 // pallet_items::PetAttributeType::BaseStrength => if is_permanent { /* pet.base_strength = ... */ } else { /* error or temp buff */ },
//                 // For now, assume item effects primarily target dynamic/status attributes or grant XP.
//                 _ => { /* return Err(Error::<T>::UnsupportedItemEffectTarget.into()); */ }
//             }
//             // Self::deposit_event(Event::PetNftMetadataUpdated { owner: _caller.clone(), pet_id: *pet_id }); // Or a more specific event
//             Ok(())
//         })
//     }

//     fn grant_personality_trait_to_pet(
//         _caller: &T::AccountId,
//         pet_id: &PetId,
//         trait_to_grant: Vec<u8>,
//     ) -> DispatchResult {
//         PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
//             let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
//             // Ensure trait is not too long (e.g. T::MaxTraitLength from pallet-items if shared, or local const)
//             // Ensure pet doesn't have too many traits (e.g. BoundedVec for pet.personality_traits)
//             if !pet.personality_traits.contains(&trait_to_grant) {
//                 // pet.personality_traits.try_push(trait_to_grant).map_err(|_| Error::<T>::TooManyPersonalityTraits)?; // If BoundedVec
//                 pet.personality_traits.push(trait_to_grant); // If unbounded Vec for now
//             }
//             // Self::deposit_event(Event::PetNftMetadataUpdated { owner: _caller.clone(), pet_id: *pet_id });
//             Ok(())
//         })
//     }

//     fn modify_pet_fertility_value(
//         _caller: &T::AccountId,
//         _pet_id: &PetId,
//         _fertility_points_change: i16,
//     ) -> DispatchResult {
//         // CONCEPTUAL: PetNft would need a `fertility_score: u8` field.
//         // PetNfts::<T>::try_mutate(...)
//         Ok(())
//     }

//     fn reduce_pet_breeding_cooldown(
//         _caller: &T::AccountId,
//         _pet_id: &PetId,
//         _reduction_blocks: BlockNumberFor<T>,
//     ) -> DispatchResult {
//         // CONCEPTUAL: Requires pallet-breeding to expose a way to modify cooldowns,
//         // or this pallet needs to manage breeding cooldowns if they are part of PetNft struct.
//         // More likely, pallet-breeding has its own cooldown storage. This function might then
//         // call a function in pallet-breeding.
//         Ok(())
//     }
// }
