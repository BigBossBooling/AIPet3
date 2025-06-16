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
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, Randomness},
    };
    use frame_system::pallet_prelude::*;
    use super::NftManager;
    use sp_std::vec::Vec;
    use scale_info::TypeInfo;

    // Define PetId type alias for clarity
    pub type PetId = u32;
    // Conceptual ItemId type alias (ideally from pallet-items)
    pub type ItemId = u32;

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
    #[scale_info(skip_type_params(T))] // Important for BoundedVec usage with generics
    pub struct PetNft<T: Config> {
        // --- Immutable Attributes ---
        pub id: PetId,
        pub dna_hash: [u8; 16],
        pub initial_species: Vec<u8>, // Consider BoundedVec in future if max length is critical early
        pub current_pet_name: Vec<u8>, // Consider BoundedVec in future

        // Explicit On-Chain Charter Attributes (Immutable after minting)
        pub base_strength: u8,
        pub base_agility: u8,
        pub base_intelligence: u8,
        pub base_vitality: u8,
        pub primary_elemental_affinity: Option<ElementType>,

        // --- Dynamic Attributes (Simplified) ---
        pub level: u32,
        pub experience_points: u32,

        // Simplified mood. Hunger & Energy are inferred from timestamps for off-chain simulation.
        pub mood_indicator: u8, // e.g., 0-Unhappy, 50-Neutral, up to T::MaxMoodValue. Updated by direct actions.

        pub last_fed_block: BlockNumberFor<T>,
        pub last_played_block: BlockNumberFor<T>, // Represents general care/interaction timestamp

        pub personality_traits: BoundedVec<BoundedVec<u8, T::MaxTraitStringLen>, T::MaxPetPersonalityTraits>,

        pub last_state_update_block: BlockNumberFor<T>, // Block of last significant on-chain state change or interaction
    }

    // Conceptual: Trait to be implemented by pallet-items for basic care item consumption
    // This allows pallet-critter-nfts to be less coupled with pallet-items' internal structure.
    pub trait BasicCareItemConsumer<AccountId, ItemId> {
        // Define ItemCategory conceptually, assuming pallet-items has something similar
        // For this example, we'll just pass a simple enum or u8 for category if needed.
        // Let's assume pallet-items will define its own ItemCategory enum.
        // For now, we'll use a placeholder type or rely on ItemId implying its category for simplicity here.
        // A better way: pallet-items defines an enum `ItemCategory { Food, Toy, Potion, ... }`
        // and `consume_care_item` takes this category.
        // For this subtask, to avoid defining pallet_items::ItemCategory here:
        fn consume_item_if_category(
            user: &AccountId,
            item_id: ItemId,
            // category_tag: u8, // e.g., 0 for Food, 1 for Toy
            // Or, if pallet-items has a public ItemCategory enum:
            category: pallet_items::ItemCategory, // Assuming pallet_items::ItemCategory exists
        ) -> DispatchResult;
    }

    // Placeholder for pallet_items::ItemCategory if not directly importing
    // This is just for compilation within this pallet if pallet_items is not a direct dep for types
    // In a real setup, this would come from pallet_items.
    pub mod pallet_items {
        #[derive(PartialEq, Clone, Copy)] // For comparison in consume_item_if_category
        pub enum ItemCategory { Food, Toy, Other } // Simplified for this context
    }


    pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        #[pallet::constant]
        type MaxOwnedPets: Get<u32>;
        type PetRandomness: Randomness<Self::Hash, Self::BlockNumber>;
        #[pallet::constant]
        type DailyClaimAmount: Get<BalanceOf<Self>>;
        #[pallet::constant]
        type ClaimCooldownPeriod: Get<Self::BlockNumber>;

        // New constants for simplified pet care
        #[pallet::constant]
        type MaxTraitStringLen: Get<u32>; // Max length of a personality trait string
        #[pallet::constant]
        type MaxPetPersonalityTraits: Get<u32>; // Max number of personality traits per pet
        #[pallet::constant]
        type MaxMoodValue: Get<u8>; // Max value for mood_indicator (e.g., 100 or 200)
        #[pallet::constant]
        type FeedMoodBoost: Get<u8>; // Mood boost from basic feeding
        #[pallet::constant]
        type PlayMoodBoost: Get<u8>; // Mood boost from basic playing
        #[pallet::constant]
        type FeedXpGain: Get<u32>;   // XP gain from basic feeding
        #[pallet::constant]
        type PlayXpGain: Get<u32>;   // XP gain from basic playing
        #[pallet::constant]
        type NeglectMoodPenalty: Get<u8>; // Penalty for neglect
        #[pallet::constant]
        type NeglectThresholdBlocks: Get<Self::BlockNumber>; // Blocks after which neglect effects might apply

        // Handler for consuming basic care items (Food, Toys)
        // This trait should be implemented by pallet-items.
        type ItemHandler: BasicCareItemConsumer<Self::AccountId, ItemId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_pet_id)]
    pub(super) type NextPetId<T: Config> = StorageValue<_, PetId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_nfts)]
    pub(super) type PetNfts<T: Config> = StorageMap<_, Blake2_128Concat, PetId, PetNft<T>>; // Changed PetNft<BlockNumberFor<T>> to PetNft<T>

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
        /// A pet was fed. [owner, pet_id, food_item_id]
        PetFed { owner: T::AccountId, pet_id: PetId, food_item_id: ItemId },
        /// A pet was played with. [owner, pet_id, toy_item_id]
        PetPlayedWith { owner: T::AccountId, pet_id: PetId, toy_item_id: ItemId },
        /// A pet leveled up. [pet_id, new_level]
        PetLeveledUp { pet_id: PetId, new_level: u32 },
        /// A pet's mood changed due to neglect. [pet_id, new_mood]
        PetNeglected { pet_id: PetId, new_mood: u8 },
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
        ClaimRewardFailed,
        /// Error from the ItemHandler (e.g., item not found, not correct category, consumption failed).
        ItemInteractionFailed,
        /// Personality trait string is too long.
        TraitStringTooLong,
        /// Pet already has the maximum number of personality traits.
        TooManyPersonalityTraits,
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
                primary_elemental_affinity,
                level: 1,
                experience_points: 0,
                mood_indicator: T::MaxMoodValue::get(), // Start at max mood
                last_fed_block: frame_system::Pallet::<T>::block_number(),
                last_played_block: frame_system::Pallet::<T>::block_number(),
                personality_traits: BoundedVec::new(), // Initialize empty BoundedVec
                last_state_update_block: frame_system::Pallet::<T>::block_number(),
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
            // Level and XP are updated by feed/play/battle actions, not directly here.
            // Mood is updated by feed/play, or neglect. Not directly here.
            // Hunger/Energy are now off-chain concepts based on timestamps.
            personality_traits: Option<BoundedVec<BoundedVec<u8, T::MaxTraitStringLen>, T::MaxPetPersonalityTraits>>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let current_owner = PetNftOwner::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(current_owner == sender, Error::<T>::NotOwner);

            PetNfts::<T>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
                let pet_nft = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

                if let Some(new_name) = name {
                    // Consider BoundedVec for name in PetNft struct and validate length here if so.
                    pet_nft.current_pet_name = new_name;
                }
                if let Some(new_traits) = personality_traits {
                    // This replaces all traits. Individual add/remove might be other extrinsics.
                    pet_nft.personality_traits = new_traits;
                }
                pet_nft.last_state_update_block = frame_system::Pallet::<T>::block_number();
                Ok(())
            })?;

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
            T::Currency::deposit_creating(&claimer, amount);

            LastClaimTime::<T>::insert(&claimer, current_block);

            Self::deposit_event(Event::DailyClaimMade {
                account: claimer,
                amount,
                claim_time: current_block,
            });

            Ok(())
        }

        /// Feed a pet with a specified food item.
        #[pallet::call_index(4)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).reads(2))] // R: Owner, Item; W: PetNft
        pub fn feed_pet(origin: OriginFor<T>, pet_id: PetId, food_item_id: ItemId) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            ensure!(Self::owner_of_pet(&pet_id) == Some(owner.clone()), Error::<T>::NotOwner);

            // Consume the food item via ItemHandler.
            // Assumes pallet_items::ItemCategory::Food exists and is the correct variant.
            T::ItemHandler::consume_item_if_category(&owner, food_item_id, pallet_items::ItemCategory::Food)
                .map_err(|_| Error::<T>::ItemInteractionFailed)?;

            PetNfts::<T>::try_mutate(&pet_id, |pet_opt| -> DispatchResult {
                let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                let current_block = frame_system::Pallet::<T>::block_number();

                pet.last_fed_block = current_block;
                pet.mood_indicator = pet.mood_indicator.saturating_add(T::FeedMoodBoost::get()).min(T::MaxMoodValue::get());
                pet.experience_points = pet.experience_points.saturating_add(T::FeedXpGain::get());
                Self::attempt_level_up(pet)?; // Internal helper
                pet.last_state_update_block = current_block;
                Ok(())
            })?;

            Self::deposit_event(Event::PetFed { owner, pet_id, food_item_id });
            Ok(())
        }

        /// Play with a pet using a specified toy item.
        #[pallet::call_index(5)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).reads(2))] // Similar to feed_pet
        pub fn play_with_pet(origin: OriginFor<T>, pet_id: PetId, toy_item_id: ItemId) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            ensure!(Self::owner_of_pet(&pet_id) == Some(owner.clone()), Error::<T>::NotOwner);

            // Consume the toy item via ItemHandler.
            // Assumes pallet_items::ItemCategory::Toy exists.
            T::ItemHandler::consume_item_if_category(&owner, toy_item_id, pallet_items::ItemCategory::Toy)
                .map_err(|_| Error::<T>::ItemInteractionFailed)?;

            PetNfts::<T>::try_mutate(&pet_id, |pet_opt| -> DispatchResult {
                let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                let current_block = frame_system::Pallet::<T>::block_number();

                pet.last_played_block = current_block; // This is the general care timestamp too
                pet.mood_indicator = pet.mood_indicator.saturating_add(T::PlayMoodBoost::get()).min(T::MaxMoodValue::get());
                pet.experience_points = pet.experience_points.saturating_add(T::PlayXpGain::get());
                Self::attempt_level_up(pet)?;
                pet.last_state_update_block = current_block;
                Ok(())
            })?;

            Self::deposit_event(Event::PetPlayedWith { owner, pet_id, toy_item_id });
            Ok(())
        }

        /// Potentially apply neglect effects if the pet hasn't been interacted with.
        /// This is a public extrinsic but might be called by an off-chain worker or by users infrequently.
        #[pallet::call_index(6)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1,1))]
        pub fn apply_neglect_check(origin: OriginFor<T>, pet_id: PetId) -> DispatchResult {
            // Anyone can call this, or it could be restricted.
            // For now, let's allow signed origin to trigger it.
            let _sender = ensure_signed(origin)?;

            PetNfts::<T>::try_mutate(&pet_id, |pet_opt| -> DispatchResult {
                let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                let current_block = frame_system::Pallet::<T>::block_number();

                // Use last_played_block as the general "last care" timestamp for neglect.
                if current_block.saturating_sub(pet.last_played_block) > T::NeglectThresholdBlocks::get() {
                    let old_mood = pet.mood_indicator;
                    pet.mood_indicator = pet.mood_indicator.saturating_sub(T::NeglectMoodPenalty::get());
                    pet.last_state_update_block = current_block;
                    if pet.mood_indicator != old_mood {
                       Self::deposit_event(Event::PetNeglected{ pet_id, new_mood: pet.mood_indicator });
                    }
                }
                Ok(())
            })
        }
    }

    impl<T: Config> Pallet<T> {
        /// Internal helper to handle level ups.
        fn attempt_level_up(pet: &mut PetNft<T>) -> DispatchResult {
            // Simplified XP curve: e.g., 100 XP for level 1, 200 for level 2, etc.
            // This can be made more complex with T::Config constants.
            let xp_needed_for_next_level = 100u32.saturating_mul(pet.level);

            if pet.experience_points >= xp_needed_for_next_level {
                pet.level = pet.level.saturating_add(1);
                pet.experience_points = pet.experience_points.saturating_sub(xp_needed_for_next_level);

                Self::deposit_event(Event::PetLeveledUp { pet_id: pet.id, new_level: pet.level });
            }
            Ok(())
        }

        // Other internal helpers or public getters can go here.
        // For example, a getter for PetNft details that might also calculate dynamic stats off-chain.
        // pub fn get_pet_details(pet_id: &PetId) -> Option<PetNft<T>> {
        //     Self::pet_nfts(pet_id)
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


// Conceptual Implementation of NftManagerForItems trait (defined in pallet-items)
// This allows pallet-items to call functions on this pallet to apply specific effects to PetNfts.
// Note: A proper dependency setup would be needed for pallet_items types.
// For this conceptual phase, we assume pallet_items::TraitName is accessible.
// This assumes pallet_items::NftManagerForItems is correctly defined in pallet_items's lib.rs
// and that pallet-critter-nfts::Config includes pallet_items::Config or necessary type aliases.
// For MVP, trait generics are simplified: AccountId, PetId, TraitTypeString (Vec<u8>), DispatchResultType
impl<T: Config> crate::pallet_items::NftManagerForItems<T::AccountId, PetId, Vec<u8>, DispatchResult> for Pallet<T>
    // where T: pallet_items::Config // This would be needed if using types from pallet_items::Config directly
{
    fn get_pet_owner(pet_id: &PetId) -> Option<T::AccountId> {
        Self::pet_nft_owner(pet_id)
    }

    fn grant_fixed_xp_to_pet(
        caller: &T::AccountId,
        pet_id: &PetId,
        amount: u32
    ) -> DispatchResult {
        // Ensure caller owns the pet (or any other permission logic if needed)
        ensure!(Self::owner_of_pet(pet_id) == Some(caller.clone()), Error::<T>::NotOwner);

        PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            pet.experience_points = pet.experience_points.saturating_add(amount);
            Self::attempt_level_up(pet)?;
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            Ok(())
        })
    }

    fn modify_mood_of_pet(
        caller: &T::AccountId,
        pet_id: &PetId,
        amount: i16
    ) -> DispatchResult {
        ensure!(Self::owner_of_pet(pet_id) == Some(caller.clone()), Error::<T>::NotOwner);

        PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            let current_mood = pet.mood_indicator as i16;
            let new_mood = current_mood.saturating_add(amount).clamp(0, T::MaxMoodValue::get() as i16) as u8;
            pet.mood_indicator = new_mood;
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            // Self::deposit_event(Event::PetNftMetadataUpdated { owner: caller.clone(), pet_id: *pet_id }); // Or a more specific mood event
            Ok(())
        })
    }

    fn grant_personality_trait_to_pet(
        caller: &T::AccountId,
        pet_id: &PetId,
        trait_to_grant: Vec<u8>,
    ) -> DispatchResult {
        ensure!(Self::owner_of_pet(pet_id) == Some(caller.clone()), Error::<T>::NotOwner);

        PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

            let bounded_trait: BoundedVec<u8, T::MaxTraitStringLen> = trait_to_grant.try_into()
                .map_err(|_| Error::<T>::TraitStringTooLong)?;

            if !pet.personality_traits.iter().any(|existing_trait| existing_trait == &bounded_trait) {
                pet.personality_traits.try_push(bounded_trait)
                    .map_err(|_| Error::<T>::TooManyPersonalityTraits)?;
            }
            // Else, trait already exists, do nothing or return specific info.

            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            // Self::deposit_event(Event::PetNftMetadataUpdated { owner: caller.clone(), pet_id: *pet_id });
            Ok(())
        })
    }

    fn apply_breeding_assist_effect_to_pet(
        caller: &T::AccountId,
        pet_id: &PetId,
        effect_type_id: u8,
        value: u32
    ) -> DispatchResult {
        ensure!(Self::owner_of_pet(pet_id) == Some(caller.clone()), Error::<T>::NotOwner);
        // This is highly conceptual for MVP as pallet-breeding is not fully defined.
        // This function would:
        // 1. Mutate PetNfts storage if the effect is stored directly on the PetNft struct
        //    (e.g., a conceptual `fertility_points: u8` field).
        // 2. OR, call a function on a `BreedingHandler` trait if breeding logic is in another pallet.
        // For now, let's assume it might update a conceptual field or just log an event.
        // Example: if effect_type_id == 0, increase conceptual fertility score by `value`.
        //          if effect_type_id == 1, reduce breeding cooldown by `value` blocks.

        // Placeholder: Just update last_state_update_block to acknowledge interaction
        PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            // Log a specific event here if desired, e.g., PetBreedingAssistApplied
            Ok(())
        })?;

        log::info!("Conceptual breeding assist effect (type: {}, value: {}) applied to pet {}", effect_type_id, value, pet_id);
        Ok(())
    }
}
