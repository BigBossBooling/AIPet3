//! # Pet NFT Pallet
//!
//! This pallet manages the core Non-Fungible Tokens (NFTs) representing CritterCraft Pets.
//! It defines the immutable "charter attributes" (nature) and dynamic "development attributes" (nurture)
//! of each Pet NFT. It also handles minting, ownership, basic transfers, and
//! essential interactions that drive a pet's lifecycle.
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
        traits::{Currency, Randomness, UnixTime}, // Currency for balances, Randomness for DNA, UnixTime for timestamps (if used)
        BoundedVec, // For bounded collections, crucial for security
    };
    use frame_system::{
        pallet_prelude::*, // Provides types like BlockNumberFor, AccountId, OriginFor
        ensure_signed,     // Macro to ensure origin is a signed account
    };
    // Use shared traits and types defined in the `crittercraft-traits` crate
    use crittercraft_traits::{
        SharedNftManager as NftManager, // Renamed for clarity within this pallet
        NftBreedingHandler, // For integration with breeding logic
        NftManagerForItems, // For integration with item usage
        QuestNftRequirementChecker, // For integration with quest requirements
        BasicCareItemConsumer, // This trait defines the interface pallet-critter-nfts expects from pallet-items
        SimpleGeneticInfo, // Data structure for breeding genetics
        PetId as SharedPetId, // Using SharedPetId to distinguish from local PetId if needed (typically u32)
        ItemId as SharedItemId, // Using ItemId from `pallet-items`
        DnaHashType,        // Type for DNA hash (e.g., [u8; 32])
        SpeciesType,        // Type for Species (e.g., BoundedVec<u8, MaxSpeciesNameLen>)
        TraitTypeString,    // Type for Personality Trait strings (e.g., BoundedVec<u8, MaxTraitStringLen>)
        ItemCategoryTag,    // For `BasicCareItemConsumer` (e.g., u8 for enum variant index)
        TraitConfigConstants, // For accessing constant bounds from traits crate
    };
    use sp_std::vec::Vec; // Standard Vec for dynamic arrays (used where not bounded)
    use scale_info::TypeInfo; // For `TypeInfo` derive macro
    use frame_support::log; // Correct way to import Substrate's logging macro
    use sp_runtime::SaturatedFrom; // For saturating arithmetic

    // --- Type Aliases ---
    // These aliases enhance clarity, aligning with "Know Your Core, Keep it Clear".
    pub type PetId = u32; // Unique identifier for each Pet NFT
    // ItemId is provided by SharedItemId from crittercraft-traits, no need to re-alias here.

    // --- Enum Definitions ---
    // ElementType: Defines the elemental affinities of Critters.
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

    // --- Struct Definitions ---
    // PetNft: Defines the core attributes and state of a CritterCraft Pet NFT.
    // #[scale_info(skip_type_params(T))] is important when T is only used in BoundedVec/Storage, not directly in struct.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))] // T is used in BoundedVecs, so skip for TypeInfo on struct
    pub struct PetNft<T: Config> {
        // --- Immutable Attributes (Charter Attributes) ---
        // Set at minting and never change, defining the pet's inherent "nature".
        pub id: PetId,
        pub dna_hash: DnaHashType, // Using DnaHashType from `crittercraft-traits`
        pub initial_species: SpeciesType, // Using SpeciesType from `crittercraft-traits`
        pub current_pet_name: BoundedVec<u8, T::MaxPetNameLen>, // Using BoundedVec for security

        // Deterministically derived from dna_hash at minting.
        pub base_strength: u8,
        pub base_agility: u8,
        pub base_intelligence: u8,
        pub base_vitality: u8,
        pub primary_elemental_affinity: ElementType, // Changed to non-Option, Default is Neutral

        // --- Dynamic Attributes ---
        // These attributes change over time based on interactions and gameplay.
        pub level: u32,
        pub experience_points: u32,

        // Simplified mood. Hunger & Energy are primarily inferred off-chain from timestamps.
        pub mood_indicator: u8, // e.g., 0-Unhappy, 50-Neutral, up to T::MaxMoodValue. Updated by direct actions.

        // On-chain timestamps for off-chain state calculation and neglect checks.
        pub last_fed_block: BlockNumberFor<T>,
        pub last_played_block: BlockNumberFor<T>, // Represents general care/interaction timestamp

        // Personality traits, dynamically updated (potentially by owner-approved AI suggestions).
        pub personality_traits: BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits>, // Using TraitTypeString from `crittercraft-traits`

        pub last_state_update_block: BlockNumberFor<T>, // Block of last significant on-chain state change or interaction
        
        // V2+: Parent IDs for breeding traceability
        // pub parent1_id: Option<PetId>,
        // pub parent2_id: Option<PetId>,
    }

    // BalanceOf<T> type alias for the pallet's currency type.
    pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // --- Pallet Configuration Trait ---
    // Defines the types and constants that the runtime must provide for this pallet to function.
    #[pallet::config]
    pub trait Config: frame_system::Config + TraitConfigConstants { // Inherit TraitConfigConstants
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// The currency trait for handling PTCN token balances.
        type Currency: Currency<Self::AccountId>;

        /// The randomness trait for generating deterministic DNA hashes.
        type PetRandomness: Randomness<Self::Hash, Self::BlockNumber>;
        /// Maximum number of Pet NFTs an account can own. Crucial for limiting state bloat.
        #[pallet::constant]
        type MaxOwnedPets: Get<u32>;
        // MaxSpeciesNameLen, MaxPetNameLen, etc. are now provided by TraitConfigConstants.
        
        /// Maximum value for `mood_indicator` (e.g., 100 or 200).
        #[pallet::constant]
        type MaxMoodValue: Get<u8>;
        /// Amount of mood restored from basic feeding.
        #[pallet::constant]
        type FeedMoodBoost: Get<u8>;
        /// Amount of mood restored from basic playing.
        #[pallet::constant]
        type PlayMoodBoost: Get<u8>;
        /// XP gained from basic feeding.
        #[pallet::constant]
        type FeedXpGain: Get<u32>;
        /// XP gained from basic playing.
        #[pallet::constant]
        type PlayXpGain: Get<u32>;
        /// Mood penalty applied due to neglect.
        #[pallet::constant]
        type NeglectMoodPenalty: Get<u8>;
        /// Number of blocks after which neglect effects might apply.
        #[pallet::constant]
        type NeglectThresholdBlocks: Get<Self::BlockNumber>;
        /// Amount of PTCN claimed daily by users.
        #[pallet::constant]
        type DailyClaimAmount: Get<BalanceOf<Self>>;
        /// Cooldown period (in blocks) for daily PTCN claims.
        #[pallet::constant]
        type ClaimCooldownPeriod: Get<Self::BlockNumber>;

        /// Handler for consuming basic care items (Food, Toys).
        /// This trait is from `crittercraft-traits` and MUST be implemented by `pallet-items`.
        /// It dictates what `pallet-items` must provide for basic care item consumption logic
        /// called by `pallet-critter-nfts`.
        type ItemHandler: BasicCareItemConsumer<Self::AccountId, SharedItemId, ItemCategoryTag, DispatchResult>;
    }

    // --- Pallet Definition ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)] // Generates getter functions for storage items
    pub struct Pallet<T>(_);

    // --- Pallet Storage Items ---
    // These store the actual state of the CritterChain.
    #[pallet::storage]
    #[pallet::getter(fn next_pet_id)]
    /// Stores the next available unique PetId for new NFTs.
    pub(super) type NextPetId<T: Config> = StorageValue<_, PetId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_nfts)]
    /// Stores the comprehensive PetNft data for each PetId.
    pub(super) type PetNfts<T: Config> = StorageMap<_, Blake2_128Concat, PetId, PetNft<T>>;

    #[pallet::storage]
    #[pallet::getter(fn owner_of_pet)]
    /// Stores a list of PetIds owned by each AccountId.
    /// Uses `BoundedVec` for security against unbounded growth.
    pub(super) type OwnerOfPet<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<PetId, T::MaxOwnedPets>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_nft_owner)]
    /// Maps a PetId directly to its owner AccountId for quick lookups.
    pub(super) type PetNftOwner<T: Config> = StorageMap<_, Blake2_128Concat, PetId, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn locked_nfts)]
    /// Stores PetIds of NFTs that are currently locked (e.g., listed on marketplace, in battle).
    /// Using `ValueQuery` with `()` means we only care about the key's presence, not an associated value.
    pub(super) type LockedNfts<T: Config> = StorageMap<_, Blake2_128Concat, PetId, (), ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_claim_time)]
    /// Stores the block number of the last successful PTCN claim for each account.
    pub(super) type LastClaimTime<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::BlockNumber, ValueQuery>;


    // --- Pallet Events ---
    // Events provide transparent, auditable logs of state changes for off-chain services and UIs.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new Pet NFT has been minted. [owner, pet_id]
        PetNftMinted { owner: T::AccountId, pet_id: PetId },
        /// A Pet NFT has been transferred. [from, to, pet_id]
        PetNftTransferred { from: T::AccountId, to: T::AccountId, pet_id: PetId },
        /// A Pet NFT's metadata (name, personality traits) has been updated. [owner, pet_id]
        PetNftMetadataUpdated { owner: T::AccountId, pet_id: PetId },
        /// A user has successfully claimed their daily PTCN.
        DailyClaimMade { account: T::AccountId, amount: BalanceOf<T>, claim_time: T::BlockNumber },
        /// A pet was fed. [owner, pet_id, food_item_id]
        PetFed { owner: T::AccountId, pet_id: PetId, food_item_id: SharedItemId },
        /// A pet was played with. [owner, pet_id, toy_item_id]
        PetPlayedWith { owner: T::AccountId, pet_id: PetId, toy_item_id: SharedItemId },
        /// A pet leveled up. [pet_id, new_level]
        PetLeveledUp { pet_id: PetId, new_level: u32 },
        /// A pet's mood changed due to neglect. [pet_id, new_mood_indicator_value]
        PetNeglected { pet_id: PetId, new_mood: u8 },
        /// An NFT has been locked. [owner, pet_id]
        NftLocked { owner: T::AccountId, pet_id: PetId },
        /// An NFT has been unlocked. [owner, pet_id]
        NftUnlocked { owner: T::AccountId, pet_id: PetId },
    }

    // --- Pallet Errors ---
    // Custom errors provide precise feedback on why an extrinsic failed, crucial for debugging and user experience.
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
        NftLocked, // Used when transfer is attempted on a locked NFT
        /// The cooldown period for claiming daily PTCN has not yet passed.
        ClaimCooldownNotMet,
        /// Error from the ItemHandler (e.g., item not found, not correct category, consumption failed).
        ItemInteractionFailed,
        /// Personality trait string is too long (exceeds T::MaxTraitStringLen).
        TraitStringTooLong,
        /// Pet already has the maximum number of personality traits.
        TooManyPersonalityTraits,
        /// Failed to unmarshal/reconstruct personality traits (internal error).
        FailedToReconstructPersonalityTraits, // Added for trait handling robustness
        /// Custom name provided for pet exceeds MaxPetNameLen.
        PetNameTooLong,
        /// Custom species provided for pet exceeds MaxSpeciesNameLen.
        SpeciesNameTooLong,
    }

    // --- Pallet Extrinsics (Callable Functions) ---
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Mint a new Pet NFT.
        /// This creates a unique digital companion on CritterChain.
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().writes(4).reads(1)))] // Basic weight, adjust as needed
        pub fn mint_pet_nft(
            origin: OriginFor<T>,
            species: Vec<u8>, // Will be converted to BoundedVec inside
            name: Vec<u8>,    // Will be converted to BoundedVec inside
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // 1. Input Validation: Enforce BoundedVec limits for species and name.
            let bounded_species: SpeciesType = species.try_into()
                .map_err(|_| Error::<T>::SpeciesNameTooLong)?;
            let bounded_name: BoundedVec<u8, T::MaxPetNameLen> = name.try_into()
                .map_err(|_| Error::<T>::PetNameTooLong)?;

            // 2. Check maximum owned pets for sender.
            ensure!(
                <OwnerOfPet<T>>::get(&sender).len() < T::MaxOwnedPets::get() as usize,
                Error::<T>::ExceedMaxOwnedPets
            );

            // 3. Generate PetId.
            let pet_id = NextPetId::<T>::try_mutate(|next_id| -> Result<PetId, DispatchError> {
                let current_id = *next_id;
                *next_id = next_id.checked_add(1).ok_or(Error::<T>::NextPetIdOverflow)?;
                Ok(current_id)
            })?;

            // 4. DNA Hash Generation: Uses secure on-chain randomness.
            let (dna_seed, _) = T::PetRandomness::random_seed();
            let dna_hash_data = (dna_seed, &sender, pet_id, &bounded_species, &bounded_name).encode();
            // Using a full SHA256 hash (32 bytes) for DnaHashType, not Blake2_128 (16 bytes).
            // Need to change DnaHashType to [u8; 32] in traits/mod.rs.
            let dna_hash_val = sp_io::hashing::sha256(&dna_hash_data);

            // 5. Charter Attribute Derivation from dna_hash.
            // This algorithm is deterministic.
            let base_strength = (dna_hash_val[0] % 16) + 5; // 5-20
            let base_agility = (dna_hash_val[1] % 16) + 5;
            let base_intelligence = (dna_hash_val[2] % 16) + 5;
            let base_vitality = (dna_hash_val[3] % 16) + 5;
            let primary_elemental_affinity = match dna_hash_val[4] % 8 {
                0 => ElementType::Fire, 1 => ElementType::Water, 2 => ElementType::Earth,
                3 => ElementType::Air, 4 => ElementType::Tech, 5 => ElementType::Nature,
                6 => ElementType::Mystic,
                _ => ElementType::Neutral, // Default for 7th value, if any
            };

            // 6. Initial Dynamic Attributes (set to defaults).
            let current_block_number = frame_system::Pallet::<T>::block_number();
            let initial_mood = T::MaxMoodValue::get();
            let initial_personality_traits: BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits> = Default::default(); // Start empty

            let new_pet = PetNft {
                id: pet_id,
                dna_hash: dna_hash_val, // Use the 32-byte SHA256 hash
                initial_species: bounded_species,
                current_pet_name: bounded_name,
                base_strength,
                base_agility,
                base_intelligence,
                base_vitality,
                primary_elemental_affinity,
                level: 1,
                experience_points: 0,
                mood_indicator: initial_mood,
                last_fed_block: current_block_number,
                last_played_block: current_block_number,
                personality_traits: initial_personality_traits,
                last_state_update_block: current_block_number,
            };

            // 7. Storage Operations: Insert Pet NFT and update ownership.
            PetNfts::<T>::insert(pet_id, new_pet);
            OwnerOfPet::<T>::try_mutate(&sender, |owned_pets_vec| {
                owned_pets_vec.try_push(pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
            })?;
            PetNftOwner::<T>::insert(pet_id, sender.clone());

            // 8. Emit event for transparency and off-chain indexing.
            Self::deposit_event(Event::PetNftMinted { owner: sender, pet_id });

            Ok(())
        }

        /// Transfer a Pet NFT from the sender to a recipient.
        /// Adheres to "Sense the Landscape, Secure the Solution" by checking transferability.
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(2).writes(2)))] // Adjust weight
        pub fn transfer_pet_nft(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // 1. Basic validation: Sender cannot transfer to themselves.
            ensure!(sender != recipient, Error::<T>::CannotTransferToSelf);

            // 2. Verify ownership and pet existence.
            let owner = PetNftOwner::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(owner == sender, Error::<T>::NotOwner);

            // 3. Crucial check: Ensure the NFT is transferable (not locked by marketplace, battle, etc.).
            ensure!(Self::is_transferable(&pet_id), Error::<T>::NftLocked);

            // 4. Check recipient capacity.
            let recipient_pets_count = OwnerOfPet::<T>::get(&recipient).len(); // Get length directly
            ensure!(recipient_pets_count < T::MaxOwnedPets::get() as usize, Error::<T>::RecipientExceedMaxOwnedPets);

            // 5. Mutate ownership records atomically.
            OwnerOfPet::<T>::try_mutate(&sender, |sender_owned_pets| -> DispatchResult {
                // Find and remove the pet_id from sender's owned list.
                if let Some(index) = sender_owned_pets.iter().position(|id| *id == pet_id) {
                    sender_owned_pets.swap_remove(index);
                    Ok(())
                } else {
                    // This indicates an internal inconsistency if owner check passed but pet not in list.
                    log::error!(
                        target: "runtime::critter_nfts_pallet",
                        "Inconsistency: Pet {} owned by {} but not in OwnerOfPet list.",
                        pet_id,
                        sender
                    );
                    Err(Error::<T>::PetNotFound.into()) // More robust error, or panic in debug.
                }
            })?;

            OwnerOfPet::<T>::try_mutate(&recipient, |recipient_owned_pets| -> DispatchResult {
                // Add pet_id to recipient's owned list.
                recipient_owned_pets.try_push(pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
                // This error should ideally be caught by check 4, but good to have defense in depth.
            })?;

            // Update the direct owner mapping for the pet.
            PetNftOwner::<T>::insert(pet_id, recipient.clone());

            // 6. Emit event for transparency and off-chain indexing.
            Self::deposit_event(Event::PetNftTransferred { from: sender, to: recipient, pet_id });

            Ok(())
        }

        /// Update mutable metadata for a Pet NFT.
        /// Only the owner of the Pet NFT can perform this action.
        /// Fields set to `None` will not be updated.
        /// This is crucial for owner agency in pet development and AI personality integration.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))] // Reads: PetNftOwner, PetNfts. Writes: PetNfts.
        pub fn update_pet_metadata(
            origin: OriginFor<T>,
            pet_id: PetId,
            name: Option<Vec<u8>>,
            personality_traits: Option<BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits>>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // 1. Verify ownership.
            let owner = PetNftOwner::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(owner == sender, Error::<T>::NotOwner);

            // 2. Mutate PetNft data.
            PetNfts::<T>::try_mutate(&pet_id, |pet_nft_opt| -> DispatchResult {
                let pet_nft = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

                // Selectively update name if provided.
                if let Some(new_name) = name {
                    // Use try_into() for BoundedVec conversion and propagate error.
                    pet_nft.current_pet_name = new_name.try_into()
                        .map_err(|_| Error::<T>::PetNameTooLong)?;
                }

                // Selectively update personality traits if provided.
                // This replaces all existing traits with the new set.
                if let Some(new_traits) = personality_traits {
                    // COMMENT: This field is updated by the owner. For the AI Personality system
                    // (conceptually an off-chain engine detailed in AI_PERSONALITY_ENGINE.md),
                    // suggestions for new or modified personality traits would be presented to the owner.
                    // If the owner accepts these suggestions, they would use this extrinsic, providing
                    // the complete, updated list of traits. This ensures owner agency over on-chain
                    // personality changes for their Pet NFT.
                    
                    // Validate new_traits content - BoundedVec already checks overall length.
                    // Check individual trait string lengths.
                    for trait_string in new_traits.iter() {
                         ensure!(trait_string.len() <= T::MaxTraitStringLen::get() as usize, Error::<T>::TraitStringTooLong);
                    }
                    pet_nft.personality_traits = new_traits;
                }

                // 3. Update the last state update block, ensuring "nurture" is tracked.
                pet_nft.last_state_update_block = frame_system::Pallet::<T>::block_number();
                Ok(())
            })?;

            // 4. Emit event for transparency.
            Self::deposit_event(Event::PetNftMetadataUpdated { owner: sender, pet_id });
            Ok(())
        }

        /// Allows a user to claim their daily PTCN reward.
        /// This mechanism incentivizes consistent engagement with the CritterCraft ecosystem.
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(1).writes(2)))] // Adjust weight
        pub fn claim_daily_ptcn(origin: OriginFor<T>) -> DispatchResult {
            let claimer = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();
            let last_claim_block = LastClaimTime::<T>::get(&claimer);

            // 1. Check if the cooldown period has passed.
            ensure!(
                current_block >= last_claim_block.saturating_add(T::ClaimCooldownPeriod::get()),
                Error::<T>::ClaimCooldownNotMet
            );

            let amount = T::DailyClaimAmount::get();

            // 2. Reward the user with PTCN.
            // This uses `deposit_creating` which ensures the account exists and receives funds.
            // For existing accounts, it increases the free balance if it's not below existential deposit.
            // This is a simplified direct issuance for the purpose of this pallet.
            T::Currency::deposit_creating(&claimer, amount);

            // 3. Update the last claim time for the user.
            LastClaimTime::<T>::insert(&claimer, current_block);

            // 4. Emit event for transparency.
            Self::deposit_event(Event::DailyClaimMade {
                account: claimer,
                amount,
                claim_time: current_block,
            });

            Ok(())
        }

        /// Feed a pet with a specified food item.
        /// This promotes pet nurturing and directly impacts dynamic attributes.
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(2).writes(1)))] // R: Owner, Item; W: PetNft
        pub fn feed_pet(origin: OriginFor<T>, pet_id: PetId, food_item_id: ItemId) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            // 1. Check if the sender owns the pet.
            ensure!(Self::pet_nft_owner(pet_id) == Some(owner.clone()), Error::<T>::NotOwner);

            // 2. Consume the specified food item via the ItemHandler.
            // This interaction confirms the item exists, is of the correct category (Food),
            // and atomically deducts it from inventory, ensuring synchronized state.
            // We pass ItemHandler's internally defined FOOD_CATEGORY_TAG via its associated type if it had one,
            // or a concrete value if it's a global constant in pallet-items.
            T::ItemHandler::consume_item_of_category(&owner, &food_item_id, T::ItemHandler::food_category_tag())
                .map_err(|_| Error::<T>::ItemInteractionFailed)?; 
            
            // 3. Update pet's attributes.
            PetNfts::<T>::try_mutate(&pet_id, |pet_nft_opt| -> DispatchResult {
                let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                let current_block = frame_system::Pallet::<T>::block_number();

                // Update last fed time.
                pet.last_fed_block = current_block;
                // Boost mood, capped by MaxMoodValue.
                pet.mood_indicator = pet.mood_indicator.saturating_add(T::FeedMoodBoost::get()).min(T::MaxMoodValue::get());
                // Grant XP.
                pet.experience_points = pet.experience_points.saturating_add(T::FeedXpGain::get());
                // Attempt to level up based on new XP.
                Self::attempt_level_up(pet)?;
                // Record this interaction timestamp.
                pet.last_state_update_block = current_block;
                Ok(())
            })?;

            // 4. Emit event for transparency.
            Self::deposit_event(Event::PetFed { owner, pet_id, food_item_id });
            Ok(())
        }

        /// Play with a pet using a specified toy item.
        /// This promotes pet nurturing and directly impacts dynamic attributes.
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(2).writes(1)))] // Similar to feed_pet
        pub fn play_with_pet(origin: OriginFor<T>, pet_id: PetId, toy_item_id: ItemId) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            // 1. Check if the sender owns the pet.
            ensure!(Self::pet_nft_owner(pet_id) == Some(owner.clone()), Error::<T>::NotOwner);

            // 2. Consume the specified toy item via the ItemHandler.
            T::ItemHandler::consume_item_of_category(&owner, &toy_item_id, T::ItemHandler::toy_category_tag())
                .map_err(|_| Error::<T>::ItemInteractionFailed)?;

            // 3. Update pet's attributes.
            PetNfts::<T>::try_mutate(&pet_id, |pet_nft_opt| -> DispatchResult {
                let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                let current_block = frame_system::Pallet::<T>::block_number();

                // Update last played time (also general care timestamp).
                pet.last_played_block = current_block;
                // Boost mood.
                pet.mood_indicator = pet.mood_indicator.saturating_add(T::PlayMoodBoost::get()).min(T::MaxMoodValue::get());
                // Grant XP.
                pet.experience_points = pet.experience_points.saturating_add(T::PlayXpGain::get());
                // Attempt to level up.
                Self::attempt_level_up(pet)?;
                // Record this interaction.
                pet.last_state_update_block = current_block;
                Ok(())
            })?;

            // 4. Emit event.
            Self::deposit_event(Event::PetPlayedWith { owner, pet_id, toy_item_id });
            Ok(())
        }

        /// Potentially apply neglect effects if the pet hasn't been interacted with for a long time.
        /// This is a public extrinsic, designed to be called by any account (e.g., an off-chain worker,
        /// another player as a utility function, or the owner themselves) to trigger neglect calculations.
        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))] // Reads: PetNfts. Writes: PetNfts.
        pub fn apply_neglect_check(origin: OriginFor<T>, pet_id: PetId) -> DispatchResult {
            let _sender = ensure_signed(origin)?; // Ensure the call is signed

            // 1. Mutate the PetNft state.
            PetNfts::<T>::try_mutate(&pet_id, |pet_nft_opt| -> DispatchResult {
                let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                let current_block = frame_system::Pallet::<T>::block_number();

                // 2. Check Neglect Threshold
                if current_block.saturating_sub(pet.last_played_block) > T::NeglectThresholdBlocks::get() {
                    let old_mood = pet.mood_indicator;

                    // 3. Apply Mood Penalty
                    pet.mood_indicator = pet
                        .mood_indicator
                        .saturating_sub(T::NeglectMoodPenalty::get());

                    // 4. Update Last State Update Block
                    pet.last_state_update_block = current_block;

                    // 5. Emit Event if Mood Changed
                    if pet.mood_indicator != old_mood {
                        Self::deposit_event(Event::PetNeglected {
                            pet_id: pet.id,
                            new_mood: pet.mood_indicator,
                        });
                    }
                }

                Ok(())
            })
        }
    }

    // --- Pallet Internal Helper Functions ---
    impl<T: Config> Pallet<T> {
        /// Internal helper to handle pet level ups based on experience points.
        fn attempt_level_up(pet: &mut PetNft<T>) -> DispatchResult {
            let xp_needed_for_next_level = 100u32.saturating_mul(pet.level);

            if pet.experience_points >= xp_needed_for_next_level && xp_needed_for_next_level > 0 {
                pet.level = pet.level.saturating_add(1);
                pet.experience_points = pet.experience_points.saturating_sub(xp_needed_for_next_level);

                Self::deposit_event(Event::PetLeveledUp {
                    pet_id: pet.id,
                    new_level: pet.level,
                });
            }

            Ok(())
        }

        /// Helper function to determine if a pet is transferable.
        fn is_transferable(pet_id: &PetId) -> bool {
            !<LockedNfts::<T>>::contains_key(pet_id)
        }
    }

    // --- Trait Implementations (for External Pallet Interactions) ---

    impl<T: Config> NftManager<T::AccountId, PetId> for Pallet<T> {
        fn owner_of(pet_id: &PetId) -> Option<T::AccountId> {
            <PetNftOwner<T>>::get(pet_id)
        }

        fn is_transferable(pet_id: &PetId) -> bool {
            Self::is_transferable(pet_id)
        }

        fn lock_nft(owner: &T::AccountId, pet_id: &PetId) -> DispatchResult {
            let current_owner = <PetNftOwner<T>>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(current_owner == *owner, Error::<T>::NotOwner);
            ensure!(!<LockedNfts<T>>::contains_key(pet_id), Error::<T>::NftAlreadyLocked);

            <LockedNfts<T>>::insert(pet_id, ());
            Self::deposit_event(Event::NftLocked {
                owner: owner.clone(),
                pet_id: *pet_id,
            });

            Ok(())
        }

        fn unlock_nft(owner: &T::AccountId, pet_id: &PetId) -> DispatchResult {
            let current_owner = <PetNftOwner<T>>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(current_owner == *owner, Error::<T>::NotOwner);
            ensure!(*<LockedNfts<T>>::get(pet_id) == (), Error::<T>::NftNotLocked); // Check if value is `()`

            <LockedNfts<T>>::remove(pet_id);
            Self::deposit_event(Event::NftUnlocked {
                owner: owner.clone(),
                pet_id: *pet_id,
            });

            Ok(())
        }

        fn transfer_nft(from: &T::AccountId, to: &T::AccountId, pet_id: &PetId) -> DispatchResult {
            let current_owner = <PetNftOwner<T>>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(current_owner == *from, Error::<T>::NotOwner);
            let recipient_pets_count = <OwnerOfPet<T>>::get(to).len();
            ensure!(
                recipient_pets_count < T::MaxOwnedPets::get() as usize,
                Error::<T>::RecipientExceedMaxOwnedPets
            );

            <OwnerOfPet<T>>::try_mutate(from, |sender_owned_pets| -> DispatchResult {
                if let Some(index) = sender_owned_pets.iter().position(|id| *id == *pet_id) {
                    sender_owned_pets.swap_remove(index);
                    Ok(())
                } else {
                    log::error!(
                        target: "runtime::critter_nfts_pallet",
                        "Inconsistency: Pet {} owned by {:?} but not in OwnerOfPet list for transfer.",
                        pet_id,
                        from
                    );
                    Err(Error::<T>::PetNotFound.into())
                }
            })?;

            <OwnerOfPet<T>>::try_mutate(to, |recipient_owned_pets| -> DispatchResult {
                recipient_owned_pets.try_push(*pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
            })?;

            <PetNftOwner<T>>::insert(pet_id, to.clone());
            Self::deposit_event(Event::PetNftTransferred {
                from: sender,
                to: recipient,
                pet_id,
            });

            Ok(())
        }
    }

    impl<T: Config> NftManagerForItems<T::AccountId, PetId, TraitTypeString, T::BlockNumber> for Pallet<T> {
        fn get_pet_owner_for_item_use(pet_id: &PetId) -> Option<T::AccountId> {
            <PetNftOwner<T>>::get(pet_id)
        }

        fn apply_fixed_xp_to_pet(caller: &T::AccountId, pet_id: &PetId, amount: u32) -> DispatchResult {
            ensure!(
                <PetNftOwner<T>>::get(pet_id) == Some(caller.clone()),
                Error::<T>::NotOwner
            );

            <PetNfts<T>>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
                let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

                pet.experience_points = pet.experience_points.saturating_add(amount);
                Self::attempt_level_up(pet)?;

                pet.last_state_update_block = <frame_system::Pallet<T>>::block_number();
                Ok(())
            })
        }

        fn apply_mood_modification_to_pet(caller: &T::AccountId, pet_id: &PetId, amount: i16) -> DispatchResult {
            ensure!(
                <PetNftOwner<T>>::get(pet_id) == Some(caller.clone()),
                Error::<T>::NotOwner
            );

            <PetNfts<T>>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
                let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

                let current_mood = pet.mood_indicator as i16;
                let new_mood = current_mood
                    .saturating_add(amount)
                    .clamp(0, T::MaxMoodValue::get() as i16) as u8;

                pet.mood_indicator = new_mood;
                pet.last_state_update_block = <frame_system::Pallet<T>>::block_number();

                Ok(())
            })
        }

        fn apply_personality_trait_to_pet(
            caller: &T::AccountId,
            pet_id: &PetId,
            trait_to_grant: TraitTypeString,
        ) -> DispatchResult {
            ensure!(
                <PetNftOwner<T>>::get(pet_id) == Some(caller.clone()),
                Error::<T>::NotOwner
            );

            <PetNfts<T>>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
                let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

                if !pet.personality_traits.iter().any(|existing_trait| existing_trait == &trait_to_grant) {
                    pet.personality_traits
                        .try_push(trait_to_grant)
                        .map_err(|_| Error::<T>::TooManyPersonalityTraits)?;
                } else {
                    log::info!(
                        target: "runtime::critter_nfts_pallet",
                        "Trait already exists for pet {}: {:?}",
                        pet_id,
                        pet.personality_traits
                    );
                }

                pet.last_state_update_block = <frame_system::Pallet<T>>::block_number();
                Ok(())
            })
        }

        fn apply_breeding_assist_effect(
            caller: &T::AccountId,
            pet_id: &PetId,
            effect_type_id: u8,
            value: u32,
        ) -> DispatchResult {
            ensure!(
                <PetNftOwner<T>>::get(pet_id) == Some(caller.clone()),
                Error::<T>::NotOwner
            );

            <PetNfts<T>>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
                let _pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

                // Placeholder for breeding logic
                // Implement breeding-specific mutations here

                _pet.last_state_update_block = <frame_system::Pallet<T>>::block_number(); // Use `_pet`

                Ok(())
            })?;

            log::info!(
                target: "runtime::critter_nfts_pallet",
                "Breeding assist effect applied: Type ID: {}, Value: {} to Pet ID: {} by Owner: {:?}",
                effect_type_id,
                value,
                pet_id,
                caller
            );

            Ok(())
        }
    }

    impl<T: Config> NftBreedingHandler<T::AccountId, PetId, DnaHashType, SpeciesType> for Pallet<T> {
        fn get_pet_simple_genetics(
            pet_id: &PetId,
        ) -> Option<SimpleGeneticInfo<DnaHashType, SpeciesType>> {
            if let Some(pet_nft) = <PetNfts<T>>::get(pet_id) {
                Some(SimpleGeneticInfo {
                    dna_hash: pet_nft.dna_hash,
                    species: pet_nft.initial_species.clone(),
                })
            } else {
                None
            }
        }

        fn mint_pet_from_breeding(
            owner: &T::AccountId,
            species: SpeciesType,
            dna_hash: DnaHashType,
            parent1_id: PetId,
            parent2_id: PetId,
            initial_name: BoundedVec<u8, T::MaxPetNameLen>,
        ) -> Result<PetId, DispatchResult> {
            ensure!(
                <OwnerOfPet<T>>::get(owner).len() < T::MaxOwnedPets::get() as usize,
                Error::<T>::ExceedMaxOwnedPets
            );

            let pet_id = <NextPetId<T>>::try_mutate(|next_id| -> Result<PetId, DispatchError> {
                let current_id = *next_id;
                *next_id = next_id.checked_add(1).ok_or(Error::<T>::NextPetIdOverflow)?;
                Ok(current_id)
            })?;

            let base_strength = (dna_hash[0] % 16) + 5;
            let base_agility = (dna_hash[1] % 16) + 5;
            let base_intelligence = (dna_hash[2] % 16) + 5;
            let base_vitality = (dna_hash[3] % 16) + 5;
            let primary_elemental_affinity = match dna_hash[4] % 8 {
                0 => ElementType::Fire,
                1 => ElementType::Water,
                2 => ElementType::Earth,
                3 => ElementType::Air,
                4 => ElementType::Tech,
                5 => ElementType::Nature,
                6 => ElementType::Mystic,
                _ => ElementType::Neutral,
            };

            let current_block_number = <frame_system::Pallet<T>>::block_number();
            let new_pet = PetNft {
                id: pet_id,
                dna_hash,
                initial_species: species.clone(),
                current_pet_name: initial_name,
                base_strength,
                base_agility,
                base_intelligence,
                base_vitality,
                primary_elemental_affinity,
                level: 1,
                experience_points: 0,
                mood_indicator: T::MaxMoodValue::get(),
                last_fed_block: current_block_number,
                last_played_block: current_block_number,
                personality_traits: Default::default(),
                last_state_update_block: current_block_number,
            };

            <PetNfts<T>>::insert(pet_id, new_pet);
            <OwnerOfPet<T>>::try_mutate(owner, |owned_pets_vec| {
                owned_pets_vec.try_push(pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
            })?;
            <PetNftOwner<T>>::insert(pet_id, owner.clone());

            Self::deposit_event(Event::PetNftMinted {
                owner: owner.clone(),
                pet_id,
            });

            Ok(pet_id)
        }
    }

    impl<T: Config> QuestNftRequirementChecker<T::AccountId, PetId, SpeciesType> for Pallet<T> {
        fn get_pet_owner_for_quest(pet_id: &PetId) -> Option<T::AccountId> {
            <PetNftOwner<T>>::get(pet_id)
        }

        fn get_pet_level_for_quest(pet_id: &PetId) -> Option<u32> {
            <PetNfts<T>>::get(pet_id).map(|pet| pet.level)
        }

        fn get_pet_species_for_quest(pet_id: &PetId) -> Option<SpeciesType> {
            <PetNfts<T>>::get(pet_id).map(|pet| pet.initial_species.clone())
        }
    