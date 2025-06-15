#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, Randomness}, // Added Randomness for DNA hash
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec; // For Vec<u8>
    use scale_info::TypeInfo; // For TypeInfo trait

    // Define PetId type alias for clarity
    pub type PetId = u32;

    // Define the PetNft struct
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    // #[scale_info(skip_type_params(T))] // Not needed if AccountId is not part of PetNft struct directly
    pub struct PetNft { // Removed AccountId generic here as it's not used in the struct fields
        pub id: PetId,
        pub dna_hash: [u8; 16], // 16 bytes for DNA hash
        pub initial_species: Vec<u8>,
        pub current_pet_name: Vec<u8>,
        pub level: u32,
        pub experience_points: u32,
        pub mood_indicator: u8, // e.g., 0=Sad, 1=Neutral, 2=Happy, 3=Playful
        pub hunger_status: u8,  // Numerical value
        pub energy_status: u8,  // Numerical value
        // pub owner: AccountId, // Considering if owner should be part of the struct or only in maps
    }

    // Type alias for balance, primarily for Currency trait
    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new Pet NFT has been minted. [owner, pet_id]
        PetNftMinted { owner: T::AccountId, pet_id: PetId },
        /// A Pet NFT has been transferred. [from, to, pet_id]
        PetNftTransferred { from: T::AccountId, to: T::AccountId, pet_id: PetId },
        // TODO: Add other events like AttributeUpdated, etc.
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


            // Create new PetNft instance
            let new_pet = PetNft {
                id: pet_id,
                dna_hash,
                initial_species: species.clone(),
                current_pet_name: name.clone(),
                level: 1,
                experience_points: 0,
                mood_indicator: 1, // Neutral
                hunger_status: 50, // Default
                energy_status: 50, // Default
                // owner: sender.clone(), // Not storing owner in struct directly to avoid data duplication
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
    }
}
