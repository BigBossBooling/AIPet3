#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// Placeholder for NftManager trait, assuming it's accessible from pallet-critter-nfts
// In a real setup, pallet-breeding would depend on pallet-critter-nfts.
// use pallet_critter_nfts::NftManager;
// For now, we'll define a local conceptual version or assume it's in scope for T::NftHandler.

// Placeholder for ItemManager trait if fertility items are used
// pub trait ItemManager<AccountId, ItemId, DispatchResult> {
//     fn item_exists(item_id: &ItemId) -> bool;
//     fn use_item(owner: &AccountId, item_id: &ItemId) -> DispatchResult;
//     fn get_item_fertility_boost(item_id: &ItemId) -> Option<u32>; // e.g., boost percentage
// }


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, Randomness, Time}, // Time for incubation/cooldowns
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_std::vec::Vec;
    // Assuming PetId is u32 as in pallet-critter-nfts
    // This would ideally come from a shared type definition or pallet_critter_nfts::PetId
    pub type PetId = u32;
    pub type OffspringId = u32; // Could also be PetId if new pets get PetIds immediately

    // Assuming NftManager is correctly defined and accessible via T::NftHandler
    // For this conceptual outline, we assume pallet_critter_nfts::NftManager exists
    // and pallet_critter_nfts::PetNft exists for offspring data.

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Details of a pending offspring.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    // Ensure ElementType is correctly pathed or made generic if defined elsewhere and not imported.
    // For this conceptual outline, we assume pallet_critter_nfts::ElementType is accessible.
    // Simplified for MVP: pallet-breeding determines DNA and species.
    // pallet-critter-nfts derives actual base stats from this DNA upon minting.
    pub struct OffspringDetails<AccountId, BlockNumber, PetDnaHash> {
        pub parents: (PetId, PetId),
        pub breeder: AccountId, // Account that initiated breeding & can claim
        pub birth_block: BlockNumber,
        pub ready_at_block: BlockNumber,
        pub determined_dna_hash: PetDnaHash, // Key output for new pet's genetics, e.g., [u8;16]
        pub determined_species: Vec<u8>,     // Key output for new pet's species
    }


    /// Details of an ongoing breeding attempt or cooldown.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct BreedingAttempt<BlockNumber> {
        pub initiated_at: BlockNumber,
        pub cooldown_until_parent1: BlockNumber,
        pub cooldown_until_parent2: BlockNumber,
        // pub resulting_offspring_id: Option<OffspringId>, // Filled when offspring is generated
    }


    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>; // For potential breeding fees or item interactions
        type TimeProvider: Time; // For incubation periods and cooldowns
        type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>; // For genetic algorithm

        /// Handler for interacting with Pet NFTs.
        /// Needs methods for:
        /// - Checking ownership.
        /// - Getting pet details relevant for breeding (species, DNA hash).
        /// - Minting a new pet from breeding results (species, DNA hash, parent IDs).
        type NftHandler: NftBreedingHandler<Self::AccountId, PetId, DispatchResult>; // Using the new conceptual trait

        // /// Handler for interacting with Items (e.g., fertility items). Placeholder for MVP.
        // type ItemHandler: super::ItemManager<Self::AccountId, u32, DispatchResult>; // Assuming ItemId is u32

        #[pallet::constant]
        type IncubationDuration: Get<Self::BlockNumber>; // Blocks for an egg to "hatch"
        #[pallet::constant]
        type BreedingCooldownDuration: Get<Self::BlockNumber>; // Blocks a pet must wait after breeding
        #[pallet::constant]
        type MaxPendingOffspringPerAccount: Get<u32>; // Limit pending claims

        // SYNERGY: Economic Logic - Breeding Fee (Can be set to 0 for MVP in runtime config)
        #[pallet::constant]
        type BreedingFee: Get<BalanceOf<Self>>;
        type BreedingFeeDestination: OnUnbalanced<NegativeImbalanceOf<T>>; // Where fees go (e.g., Treasury) if fee > 0
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_offspring_id)]
    /// Tracks the next available ID for a pending offspring.
    pub(super) type NextOffspringId<T: Config> = StorageValue<_, OffspringId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_breeding_cooldowns)]
    /// Stores the block number until which a PetId is in breeding cooldown.
    pub(super) type PetBreedingCooldowns<T: Config> = StorageMap<_, Blake2_128Concat, PetId, T::BlockNumber>;

    #[pallet::storage]
    #[pallet::getter(fn pending_offspring)]
    /// Stores details of offspring that have been conceived but not yet "claimed" or "hatched".
    /// Maps an OffspringId to its details.
    pub(super) type PendingOffspring<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        OffspringId,
        OffspringDetails<T::AccountId, BlockNumberFor<T>, [u8;16]>, // [u8;16] is assumed DNA Hash type
    >;

    #[pallet::storage]
    #[pallet::getter(fn account_pending_offspring_count)]
    /// Tracks how many pending offspring an account has, to limit claims.
    pub(super) type AccountPendingOffspringCount<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;


    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BreedingInitiated {
            breeder: T::AccountId,
            parent1_id: PetId,
            parent2_id: PetId,
            offspring_id: OffspringId,
            ready_at_block: T::BlockNumber,
        },
        OffspringClaimed {
            claimer: T::AccountId,
            offspring_id: OffspringId,
            new_pet_id: PetId,
        },
        // FertilityItemUsed { account: T::AccountId, item_id: u32, parent1: PetId, parent2: PetId },
    }

    #[pallet::error]
    pub enum Error<T> {
        ParentsMustBeDifferentPets,
        PetNotOwned,
        PetInBreedingCooldown,
        PetTooYoungOrIneligible, // General eligibility check
        ParentPetDataNotFound,   // New error for when NftHandler can't find parent data
        // ItemErrors
        // FertilityItemNotFound,
        // FertilityItemNotOwned,
        // FertilityItemUseFailed,
        OffspringIdOverflow,
        PendingOffspringNotFound,
        OffspringNotReadyYet,
        CannotClaimOthersOffspring, // If breeder != claimer and not allowed
        MaxPendingOffspringReached,
        NftMintingFailed, // If interaction with NftHandler fails
        CrossSpeciesBreedingNotAllowed, // If attempted when T::AllowCrossSpeciesBreeding is false
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initiate breeding between two owned Pet NFTs.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)] // Placeholder weight
        pub fn initiate_breeding(
            origin: OriginFor<T>,
            parent1_id: PetId,
            parent2_id: PetId,
            // fertility_item_id: Option<u32>, // Placeholder for item usage
        ) -> DispatchResult {
            let breeder = ensure_signed(origin)?;
            ensure!(parent1_id != parent2_id, Error::<T>::ParentsMustBeDifferentPets);

            // SYNERGY: Take breeding fee
            // For MVP, if T::BreedingFee::get() is zero, this logic will be skipped.
            let fee = T::BreedingFee::get();
            if fee > BalanceOf::<T>::from(0u32) {
               // Conceptual fee collection logic (e.g., transfer to a fee collector or treasury).
               // Example:
               // let imbalance = T::Currency::withdraw(&breeder, fee, WithdrawReasons::FEE, ExistenceRequirement::KeepAlive)?;
               // T::BreedingFeeDestination::on_unbalanced(imbalance);
               // For this conceptual pass, we assume this step succeeds if fee > 0.
            }

            // --- Verification Phase ---
            // 1. Check ownership of parent1 and parent2 by breeder via T::NftHandler
            //    ensure!(T::NftHandler::owner_of(&parent1_id) == Some(breeder.clone()), Error::<T>::PetNotOwned);
            //    ensure!(T::NftHandler::owner_of(&parent2_id) == Some(breeder.clone()), Error::<T>::PetNotOwned);

            // 2. Check breeding cooldowns for parent1 and parent2
            //    let current_block = T::TimeProvider::now().as_secs().into(); // Example, needs correct conversion
            //    if let Some(cooldown_until) = PetBreedingCooldowns::<T>::get(&parent1_id) {
            //        ensure!(current_block >= cooldown_until, Error::<T>::PetInBreedingCooldown);
            //    } // Same for parent2

            // 3. Check other eligibility (e.g., pet level, "adult" status - via NftHandler or local logic)
            //    ensure!(Self::is_pet_eligible_for_breeding(&parent1_id), Error::<T>::PetTooYoungOrIneligible);
            //    ensure!(Self::is_pet_eligible_for_breeding(&parent2_id), Error::<T>::PetTooYoungOrIneligible);

            // 4. (If using items) Verify and consume fertility_item_id via T::ItemHandler

            // 5. Check MaxPendingOffspring for the breeder
            //    let current_pending_count = AccountPendingOffspringCount::<T>::get(&breeder);
            //    ensure!(current_pending_count < T::MaxPendingOffspringPerAccount::get(), Error::<T>::MaxPendingOffspringReached);

            // --- Genetic Algorithm & Offspring Generation ---
            // a. Fetch necessary data from parents via T::NftHandler
            let parent1_genetic_info = T::NftHandler::get_pet_simple_genetics(&parent1_id)
                .ok_or(Error::<T>::ParentPetDataNotFound)?;
            let parent2_genetic_info = T::NftHandler::get_pet_simple_genetics(&parent2_id)
                .ok_or(Error::<T>::ParentPetDataNotFound)?;

            // Ensure cross-species breeding is allowed if species are different
            if parent1_genetic_info.species != parent2_genetic_info.species && !T::AllowCrossSpeciesBreeding::get() {
                ensure!(false, Error::<T>::CrossSpeciesBreedingNotAllowed);
            }

            // b. Determine Fertility Boost (Conceptual - not used in determine_offspring_genetics_mvp for now)
            // let fertility_item_effect_conceptual: Option<ConceptualFertilityBoost> = None;

            // c. & d. Call the conceptual helper function for DNA and Species
            let (determined_dna_hash, determined_species) = Self::determine_offspring_genetics_mvp(
                &parent1_genetic_info.dna_hash,
                &parent1_genetic_info.species,
                &parent2_genetic_info.dna_hash,
                &parent2_genetic_info.species,
                &T::RandomnessSource::random_seed().0, // Pass the raw random output
                T::AllowCrossSpeciesBreeding::get(),
            );

            // e. Create OffspringDetails with determined_dna_hash and determined_species.
            // Actual base stats will be derived by pallet-critter-nfts from this determined_dna_hash.
            let offspring_id = NextOffspringId::<T>::try_mutate(|id| -> Result<OffspringId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(1).ok_or(Error::<T>::OffspringIdOverflow)?;
                Ok(current_id)
            })?;

            let current_block = frame_system::Pallet::<T>::block_number();
            let ready_at_block = current_block.saturating_add(T::IncubationDuration::get());

            let new_offspring_details = OffspringDetails {
                parents: (parent1_id, parent2_id),
                breeder: breeder.clone(),
                birth_block: current_block,
                ready_at_block,
                determined_dna_hash,
                determined_species,
            };
            PendingOffspring::<T>::insert(offspring_id, new_offspring_details);
            AccountPendingOffspringCount::<T>::mutate(&breeder, |count| *count = count.saturating_add(1));

            // --- Record Keeping & Event ---
            // let offspring_id = NextOffspringId::<T>::try_mutate(|id| -> Result<OffspringId, DispatchError> {
            //     let current_id = *id;
            //     *id = id.checked_add(1).ok_or(Error::<T>::OffspringIdOverflow)?;
            //     Ok(current_id)
            // })?;
            // let current_block = frame_system::Pallet::<T>::block_number(); // Use frame_system for block number
            // let ready_at_block = current_block.saturating_add(T::IncubationDuration::get());
            //
            // let new_offspring_details = OffspringDetails {
            //     parents: (parent1_id, parent2_id),
            //     breeder: breeder.clone(),
            //     birth_block: current_block,
            //     ready_at_block,
            //     determined_dna_hash, // From step c
            //     determined_species,  // From step b
            // };
            // PendingOffspring::<T>::insert(offspring_id, new_offspring_details);
            // AccountPendingOffspringCount::<T>::mutate(&breeder, |count| *count = count.saturating_add(1));

            // Set cooldowns for parents
            // let cooldown_end = current_block.saturating_add(T::BreedingCooldownDuration::get());
            // PetBreedingCooldowns::<T>::insert(parent1_id, cooldown_end);
            // PetBreedingCooldowns::<T>::insert(parent2_id, cooldown_end);

            // Self::deposit_event(Event::BreedingInitiated { ... });

            // For subtask, return Ok(()) as logic is conceptual
            Ok(())
        }

        /// Claim a ready/hatched offspring, minting it as a new Pet NFT.
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)] // Placeholder weight
        pub fn claim_offspring(
            origin: OriginFor<T>,
            offspring_id: OffspringId,
        ) -> DispatchResult {
            let claimer = ensure_signed(origin)?;

            // 1. Retrieve PendingOffspring details. Ensure it exists.
            //    let offspring_details = PendingOffspring::<T>::get(offspring_id).ok_or(Error::<T>::PendingOffspringNotFound)?;

            // 2. Check if current block >= offspring_details.ready_at_block
            //    ensure!(current_block >= offspring_details.ready_at_block, Error::<T>::OffspringNotReadyYet);

            // 3. Verify claimer (breeder) is the one to claim.
            //    (Need to store breeder with OffspringDetails - which it does now).
            //    ensure!(offspring_details.breeder == claimer, Error::<T>::CannotClaimOthersOffspring);

            // 4. Mint the new Pet NFT using T::NftHandler.
            //    The NftBreedingHandler trait needs a method like `mint_pet_from_breeding`.
            //    This method in pallet-critter-nfts will use the provided DNA and species,
            //    derive all other charter stats (base_strength, etc.) from this DNA,
            //    and link parents.
            //    `let new_pet_id = T::NftHandler::mint_pet_from_breeding(`
            //        `&claimer,`
            //        `offspring_details.determined_species.clone(),`
            //        `offspring_details.determined_dna_hash,`
            //        `offspring_details.parents.0,`
            //        `offspring_details.parents.1`
            //        // Initial name (e.g., "Newborn <Species>") and other defaults (level 1, XP 0, mood, timestamps)
            //        // would be handled by the `mint_pet_from_breeding` implementation in pallet-critter-nfts.
            //    `)?;`

            // 5. Clean up: Remove from PendingOffspring, decrement AccountPendingOffspringCount.
            //    PendingOffspring::<T>::remove(offspring_id);
            //    AccountPendingOffspringCount::<T>::mutate(&claimer, |count| *count = count.saturating_sub(1));

            // Self::deposit_event(Event::OffspringClaimed { claimer, offspring_id, new_pet_id });

            // For subtask, return Ok(())
            Ok(())
        }

    }
}

// Conceptual trait definition for NftHandler interactions specific to breeding
// This trait would be implemented by pallet-critter-nfts.
pub trait NftBreedingHandler<AccountId, PetId, DispatchResultType> {
    /// Gets minimal genetic material (DNA hash, species) from a parent pet.
    fn get_pet_simple_genetics(pet_id: &PetId) -> Option<SimpleGeneticInfo>;

    /// Mints a new pet based on breeding results.
    /// pallet-critter-nfts handles deriving base stats from the given dna_hash.
    fn mint_pet_from_breeding(
        owner: &AccountId,
        species: Vec<u8>,
        dna_hash: [u8;16], // The DNA determined by pallet-breeding
        parent1_id: PetId,
        parent2_id: PetId,
    ) -> Result<PetId, DispatchResultType>; // Returns the new PetId

    // Potentially, methods to check pet eligibility or apply breeding cooldowns if not managed here
    // fn check_breeding_eligibility(pet_id: &PetId) -> bool;
    // fn apply_breeding_cooldown(pet_id: &PetId, cooldown_until: BlockNumber) -> DispatchResultType;
}

// Conceptual struct for returning parent genetic info
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub struct SimpleGeneticInfo {
    pub dna_hash: [u8;16],
    pub species: Vec<u8>,
}
