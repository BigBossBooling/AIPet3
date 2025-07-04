//! # Critter Daycare Pallet
//!
//! This pallet manages the "Zoologist's Lodge" daycare system for CritterCraft pets.
//! It allows players to leave their pets at the daycare when they are offline,
//! and hire other players as temporary caregivers.
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
        traits::{Currency, ReservableCurrency}, // Currency for balances
        BoundedVec, // For bounded collections, crucial for security
    };
    use frame_system::{
        pallet_prelude::*, // Provides types like BlockNumberFor, AccountId, OriginFor
        ensure_signed,     // Macro to ensure origin is a signed account
    };
    use sp_std::vec::Vec; // Standard Vec for dynamic arrays (used where not bounded)
    use scale_info::TypeInfo; // For `TypeInfo` derive macro
    use frame_support::log; // Correct way to import Substrate's logging macro
    use sp_runtime::SaturatedFrom; // For saturating arithmetic

    // Import traits from critter-nfts pallet
    use crate::traits::{
        NftManagerForDaycare, // For integration with pet NFTs
        PetId,               // Using PetId from critter-nfts
    };

    // --- Type Aliases ---
    pub type DaycareId = u32; // Unique identifier for each daycare instance

    // --- Enum Definitions ---
    // DaycareStatus: Defines the current status of a daycare instance
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum DaycareStatus {
        Open,
        Closed,
    }

    // --- Struct Definitions ---
    // Daycare: Defines a daycare instance
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Daycare<T: Config> {
        pub id: DaycareId,
        pub owner: T::AccountId,
        pub name: BoundedVec<u8, T::MaxDaycareNameLen>,
        pub description: BoundedVec<u8, T::MaxDaycareDescriptionLen>,
        pub status: DaycareStatus,
        pub fee_per_block: BalanceOf<T>,
        pub creation_block: BlockNumberFor<T>,
    }

    // DaycareListing: Defines a pet listing in a daycare
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct DaycareListing<T: Config> {
        pub daycare_id: DaycareId,
        pub pet_id: PetId,
        pub owner: T::AccountId,
        pub caregiver: Option<T::AccountId>,
        pub start_block: BlockNumberFor<T>,
        pub end_block: Option<BlockNumberFor<T>>,
        pub fee_paid: BalanceOf<T>,
    }

    // BalanceOf<T> type alias for the pallet's currency type.
    pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // --- Pallet Configuration Trait ---
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// The currency trait for handling BITS token balances.
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        
        /// Maximum number of pets a daycare can host.
        #[pallet::constant]
        type MaxPetsPerDaycare: Get<u32>;
        
        /// Maximum length of a daycare name (in bytes).
        #[pallet::constant]
        type MaxDaycareNameLen: Get<u32>;
        
        /// Maximum length of a daycare description (in bytes).
        #[pallet::constant]
        type MaxDaycareDescriptionLen: Get<u32>;
        
        /// Minimum fee per block for daycare services.
        #[pallet::constant]
        type MinFeePerBlock: Get<BalanceOf<Self>>;
        
        /// Maximum fee per block for daycare services.
        #[pallet::constant]
        type MaxFeePerBlock: Get<BalanceOf<Self>>;
        
        /// Percentage of fee that goes to the platform (e.g., 5 for 5%).
        #[pallet::constant]
        type PlatformFeePercent: Get<u8>;
        
        /// Handler for interacting with pet NFTs.
        type NftHandler: NftManagerForDaycare<Self::AccountId, PetId, DispatchResult>;
    }

    // --- Pallet Definition ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // --- Pallet Storage Items ---
    #[pallet::storage]
    #[pallet::getter(fn next_daycare_id)]
    /// Stores the next available unique DaycareId.
    pub(super) type NextDaycareId<T: Config> = StorageValue<_, DaycareId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn daycares)]
    /// Stores the comprehensive Daycare data for each DaycareId.
    pub(super) type Daycares<T: Config> = StorageMap<_, Blake2_128Concat, DaycareId, Daycare<T>>;

    #[pallet::storage]
    #[pallet::getter(fn daycare_by_owner)]
    /// Maps an AccountId to their DaycareId.
    pub(super) type DaycareByOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, DaycareId>;

    #[pallet::storage]
    #[pallet::getter(fn pets_in_daycare)]
    /// Stores a list of PetIds in each daycare.
    pub(super) type PetsInDaycare<T: Config> = StorageMap<_, Blake2_128Concat, DaycareId, BoundedVec<PetId, T::MaxPetsPerDaycare>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn daycare_listings)]
    /// Stores the DaycareListing data for each PetId.
    pub(super) type DaycareListings<T: Config> = StorageMap<_, Blake2_128Concat, PetId, DaycareListing<T>>;

    #[pallet::storage]
    #[pallet::getter(fn pets_by_caregiver)]
    /// Maps a caregiver AccountId to a list of PetIds they are caring for.
    pub(super) type PetsByCaregiver<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<PetId>, ValueQuery>;

    // --- Pallet Events ---
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new daycare has been created. [owner, daycare_id, name]
        DaycareCreated { owner: T::AccountId, daycare_id: DaycareId, name: Vec<u8> },
        
        /// A daycare's details have been updated. [owner, daycare_id]
        DaycareUpdated { owner: T::AccountId, daycare_id: DaycareId },
        
        /// A daycare's status has been changed. [owner, daycare_id, status]
        DaycareStatusChanged { owner: T::AccountId, daycare_id: DaycareId, status: DaycareStatus },
        
        /// A pet has been listed in a daycare. [owner, pet_id, daycare_id]
        PetListed { owner: T::AccountId, pet_id: PetId, daycare_id: DaycareId },
        
        /// A pet has been removed from a daycare. [owner, pet_id, daycare_id]
        PetRemoved { owner: T::AccountId, pet_id: PetId, daycare_id: DaycareId },
        
        /// A caregiver has been assigned to a pet. [caregiver, pet_id, daycare_id]
        CaregiverAssigned { caregiver: T::AccountId, pet_id: PetId, daycare_id: DaycareId },
        
        /// A caregiver has been unassigned from a pet. [caregiver, pet_id, daycare_id]
        CaregiverUnassigned { caregiver: T::AccountId, pet_id: PetId, daycare_id: DaycareId },
        
        /// A caregiver has interacted with a pet. [caregiver, pet_id, interaction_type]
        CaregiverInteraction { caregiver: T::AccountId, pet_id: PetId, interaction_type: Vec<u8> },
        
        /// A payment has been made for daycare services. [from, to, amount]
        DaycarePayment { from: T::AccountId, to: T::AccountId, amount: BalanceOf<T> },
    }

    // --- Pallet Errors ---
    #[pallet::error]
    pub enum Error<T> {
        /// The next DaycareId has overflowed.
        NextDaycareIdOverflow,
        
        /// The account already owns a daycare.
        AlreadyOwnsDaycare,
        
        /// The specified daycare does not exist.
        DaycareNotFound,
        
        /// The sender is not the owner of the daycare.
        NotDaycareOwner,
        
        /// The daycare is closed.
        DaycareClosed,
        
        /// The daycare is full.
        DaycareFull,
        
        /// The pet is already in a daycare.
        PetAlreadyInDaycare,
        
        /// The pet is not in a daycare.
        PetNotInDaycare,
        
        /// The pet is not in the specified daycare.
        PetNotInThisDaycare,
        
        /// The pet does not exist or is not owned by the sender.
        PetNotOwnedBySender,
        
        /// The pet already has a caregiver.
        PetAlreadyHasCaregiver,
        
        /// The pet does not have a caregiver.
        PetDoesNotHaveCaregiver,
        
        /// The sender is not the caregiver of the pet.
        NotPetCaregiver,
        
        /// The fee is outside the allowed range.
        InvalidFee,
        
        /// The sender does not have enough balance.
        InsufficientBalance,
        
        /// Failed to update pet's state.
        PetUpdateFailed,
        
        /// Failed to transfer funds.
        FundsTransferFailed,
    }

    // --- Pallet Hooks ---
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // --- Pallet Extrinsics ---
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new daycare.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_daycare(
            origin: OriginFor<T>,
            name: BoundedVec<u8, T::MaxDaycareNameLen>,
            description: BoundedVec<u8, T::MaxDaycareDescriptionLen>,
            fee_per_block: BalanceOf<T>,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Check if the account already owns a daycare.
            ensure!(!DaycareByOwner::<T>::contains_key(&owner), Error::<T>::AlreadyOwnsDaycare);
            
            // 2. Check if the fee is within the allowed range.
            ensure!(
                fee_per_block >= T::MinFeePerBlock::get() && fee_per_block <= T::MaxFeePerBlock::get(),
                Error::<T>::InvalidFee
            );
            
            // 3. Get the next daycare ID.
            let daycare_id = Self::next_daycare_id();
            let next_daycare_id = daycare_id.checked_add(1).ok_or(Error::<T>::NextDaycareIdOverflow)?;
            NextDaycareId::<T>::put(next_daycare_id);
            
            // 4. Create the daycare.
            let current_block = frame_system::Pallet::<T>::block_number();
            let daycare = Daycare::<T> {
                id: daycare_id,
                owner: owner.clone(),
                name: name.clone(),
                description,
                status: DaycareStatus::Open,
                fee_per_block,
                creation_block: current_block,
            };
            
            // 5. Store the daycare.
            Daycares::<T>::insert(daycare_id, daycare);
            
            // 6. Map the owner to the daycare.
            DaycareByOwner::<T>::insert(&owner, daycare_id);
            
            // 7. Emit the event.
            Self::deposit_event(Event::DaycareCreated {
                owner,
                daycare_id,
                name: name.to_vec(),
            });
            
            Ok(())
        }

        /// Update daycare details.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn update_daycare(
            origin: OriginFor<T>,
            name: Option<BoundedVec<u8, T::MaxDaycareNameLen>>,
            description: Option<BoundedVec<u8, T::MaxDaycareDescriptionLen>>,
            fee_per_block: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Get the daycare ID for the owner.
            let daycare_id = DaycareByOwner::<T>::get(&owner).ok_or(Error::<T>::DaycareNotFound)?;
            
            // 2. Get the daycare.
            let mut daycare = Daycares::<T>::get(daycare_id).ok_or(Error::<T>::DaycareNotFound)?;
            
            // 3. Check if the sender is the owner of the daycare.
            ensure!(daycare.owner == owner, Error::<T>::NotDaycareOwner);
            
            // 4. Update the daycare details.
            if let Some(name) = name {
                daycare.name = name;
            }
            
            if let Some(description) = description {
                daycare.description = description;
            }
            
            if let Some(fee_per_block) = fee_per_block {
                ensure!(
                    fee_per_block >= T::MinFeePerBlock::get() && fee_per_block <= T::MaxFeePerBlock::get(),
                    Error::<T>::InvalidFee
                );
                daycare.fee_per_block = fee_per_block;
            }
            
            // 5. Store the updated daycare.
            Daycares::<T>::insert(daycare_id, daycare);
            
            // 6. Emit the event.
            Self::deposit_event(Event::DaycareUpdated {
                owner,
                daycare_id,
            });
            
            Ok(())
        }

        /// Change daycare status (open/closed).
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn set_daycare_status(
            origin: OriginFor<T>,
            status: DaycareStatus,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Get the daycare ID for the owner.
            let daycare_id = DaycareByOwner::<T>::get(&owner).ok_or(Error::<T>::DaycareNotFound)?;
            
            // 2. Get the daycare.
            let mut daycare = Daycares::<T>::get(daycare_id).ok_or(Error::<T>::DaycareNotFound)?;
            
            // 3. Check if the sender is the owner of the daycare.
            ensure!(daycare.owner == owner, Error::<T>::NotDaycareOwner);
            
            // 4. Update the daycare status.
            daycare.status = status;
            
            // 5. Store the updated daycare.
            Daycares::<T>::insert(daycare_id, daycare);
            
            // 6. Emit the event.
            Self::deposit_event(Event::DaycareStatusChanged {
                owner,
                daycare_id,
                status,
            });
            
            Ok(())
        }

        /// List a pet in a daycare.
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn list_pet(
            origin: OriginFor<T>,
            pet_id: PetId,
            daycare_id: DaycareId,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Check if the sender owns the pet.
            ensure!(T::NftHandler::is_owner(&owner, &pet_id), Error::<T>::PetNotOwnedBySender);
            
            // 2. Check if the pet is already in a daycare.
            ensure!(!DaycareListings::<T>::contains_key(pet_id), Error::<T>::PetAlreadyInDaycare);
            
            // 3. Get the daycare.
            let daycare = Daycares::<T>::get(daycare_id).ok_or(Error::<T>::DaycareNotFound)?;
            
            // 4. Check if the daycare is open.
            ensure!(daycare.status == DaycareStatus::Open, Error::<T>::DaycareClosed);
            
            // 5. Check if the daycare is full.
            let pets_in_daycare = PetsInDaycare::<T>::get(daycare_id);
            ensure!(pets_in_daycare.len() < T::MaxPetsPerDaycare::get() as usize, Error::<T>::DaycareFull);
            
            // 6. Create the daycare listing.
            let current_block = frame_system::Pallet::<T>::block_number();
            let listing = DaycareListing::<T> {
                daycare_id,
                pet_id,
                owner: owner.clone(),
                caregiver: None,
                start_block: current_block,
                end_block: None,
                fee_paid: BalanceOf::<T>::zero(),
            };
            
            // 7. Store the daycare listing.
            DaycareListings::<T>::insert(pet_id, listing);
            
            // 8. Add the pet to the daycare.
            PetsInDaycare::<T>::try_mutate(daycare_id, |pets| -> DispatchResult {
                pets.try_push(pet_id).map_err(|_| Error::<T>::DaycareFull)?;
                Ok(())
            })?;
            
            // 9. Emit the event.
            Self::deposit_event(Event::PetListed {
                owner,
                pet_id,
                daycare_id,
            });
            
            Ok(())
        }

        /// Remove a pet from a daycare.
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn remove_pet(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Check if the pet is in a daycare.
            let listing = DaycareListings::<T>::get(pet_id).ok_or(Error::<T>::PetNotInDaycare)?;
            
            // 2. Check if the sender is the owner of the pet.
            ensure!(listing.owner == owner, Error::<T>::PetNotOwnedBySender);
            
            // 3. If the pet has a caregiver, unassign them.
            if let Some(caregiver) = listing.caregiver {
                // Remove the pet from the caregiver's list.
                PetsByCaregiver::<T>::mutate(&caregiver, |pets| {
                    if let Some(pos) = pets.iter().position(|&id| id == pet_id) {
                        pets.swap_remove(pos);
                    }
                });
                
                // Emit the event.
                Self::deposit_event(Event::CaregiverUnassigned {
                    caregiver,
                    pet_id,
                    daycare_id: listing.daycare_id,
                });
            }
            
            // 4. Remove the pet from the daycare.
            PetsInDaycare::<T>::mutate(listing.daycare_id, |pets| {
                if let Some(pos) = pets.iter().position(|&id| id == pet_id) {
                    pets.swap_remove(pos);
                }
            });
            
            // 5. Remove the daycare listing.
            DaycareListings::<T>::remove(pet_id);
            
            // 6. Emit the event.
            Self::deposit_event(Event::PetRemoved {
                owner,
                pet_id,
                daycare_id: listing.daycare_id,
            });
            
            Ok(())
        }

        /// Become a caregiver for a pet.
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn become_caregiver(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let caregiver = ensure_signed(origin)?;
            
            // 1. Check if the pet is in a daycare.
            let mut listing = DaycareListings::<T>::get(pet_id).ok_or(Error::<T>::PetNotInDaycare)?;
            
            // 2. Check if the pet already has a caregiver.
            ensure!(listing.caregiver.is_none(), Error::<T>::PetAlreadyHasCaregiver);
            
            // 3. Get the daycare.
            let daycare = Daycares::<T>::get(listing.daycare_id).ok_or(Error::<T>::DaycareNotFound)?;
            
            // 4. Check if the daycare is open.
            ensure!(daycare.status == DaycareStatus::Open, Error::<T>::DaycareClosed);
            
            // 5. Check if the caregiver has enough balance to cover the fee.
            let fee = daycare.fee_per_block;
            ensure!(T::Currency::free_balance(&caregiver) >= fee, Error::<T>::InsufficientBalance);
            
            // 6. Reserve the fee from the caregiver.
            T::Currency::reserve(&caregiver, fee)?;
            
            // 7. Update the daycare listing.
            listing.caregiver = Some(caregiver.clone());
            DaycareListings::<T>::insert(pet_id, listing.clone());
            
            // 8. Add the pet to the caregiver's list.
            PetsByCaregiver::<T>::mutate(&caregiver, |pets| {
                pets.push(pet_id);
            });
            
            // 9. Emit the event.
            Self::deposit_event(Event::CaregiverAssigned {
                caregiver,
                pet_id,
                daycare_id: listing.daycare_id,
            });
            
            Ok(())
        }

        /// Stop being a caregiver for a pet.
        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn stop_being_caregiver(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let caregiver = ensure_signed(origin)?;
            
            // 1. Check if the pet is in a daycare.
            let mut listing = DaycareListings::<T>::get(pet_id).ok_or(Error::<T>::PetNotInDaycare)?;
            
            // 2. Check if the pet has a caregiver.
            let current_caregiver = listing.caregiver.clone().ok_or(Error::<T>::PetDoesNotHaveCaregiver)?;
            
            // 3. Check if the sender is the caregiver of the pet.
            ensure!(current_caregiver == caregiver, Error::<T>::NotPetCaregiver);
            
            // 4. Get the daycare.
            let daycare = Daycares::<T>::get(listing.daycare_id).ok_or(Error::<T>::DaycareNotFound)?;
            
            // 5. Calculate the fee based on the time spent as caregiver.
            let current_block = frame_system::Pallet::<T>::block_number();
            let blocks_as_caregiver = current_block.saturating_sub(listing.start_block);
            let fee = daycare.fee_per_block.saturating_mul(blocks_as_caregiver.saturated_into());
            
            // 6. Unreserve the fee from the caregiver.
            T::Currency::unreserve(&caregiver, fee);
            
            // 7. Transfer the fee to the daycare owner.
            let platform_fee = fee.saturating_mul(T::PlatformFeePercent::get().into()).saturating_div(100u8.into());
            let owner_fee = fee.saturating_sub(platform_fee);
            
            // Transfer to the daycare owner.
            T::Currency::transfer(&caregiver, &daycare.owner, owner_fee, frame_support::traits::ExistenceRequirement::KeepAlive)?;
            
            // 8. Update the daycare listing.
            listing.caregiver = None;
            listing.fee_paid = listing.fee_paid.saturating_add(fee);
            listing.end_block = Some(current_block);
            DaycareListings::<T>::insert(pet_id, listing.clone());
            
            // 9. Remove the pet from the caregiver's list.
            PetsByCaregiver::<T>::mutate(&caregiver, |pets| {
                if let Some(pos) = pets.iter().position(|&id| id == pet_id) {
                    pets.swap_remove(pos);
                }
            });
            
            // 10. Emit the events.
            Self::deposit_event(Event::CaregiverUnassigned {
                caregiver: caregiver.clone(),
                pet_id,
                daycare_id: listing.daycare_id,
            });
            
            Self::deposit_event(Event::DaycarePayment {
                from: caregiver,
                to: daycare.owner,
                amount: owner_fee,
            });
            
            Ok(())
        }

        /// Interact with a pet as a caregiver.
        #[pallet::call_index(7)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn interact_with_pet(
            origin: OriginFor<T>,
            pet_id: PetId,
            interaction_type: Vec<u8>,
        ) -> DispatchResult {
            let caregiver = ensure_signed(origin)?;
            
            // 1. Check if the pet is in a daycare.
            let listing = DaycareListings::<T>::get(pet_id).ok_or(Error::<T>::PetNotInDaycare)?;
            
            // 2. Check if the pet has a caregiver.
            let current_caregiver = listing.caregiver.clone().ok_or(Error::<T>::PetDoesNotHaveCaregiver)?;
            
            // 3. Check if the sender is the caregiver of the pet.
            ensure!(current_caregiver == caregiver, Error::<T>::NotPetCaregiver);
            
            // 4. Interact with the pet.
            // This would call into the NftHandler to update the pet's state.
            // For now, we'll just emit an event.
            
            // 5. Emit the event.
            Self::deposit_event(Event::CaregiverInteraction {
                caregiver,
                pet_id,
                interaction_type,
            });
            
            Ok(())
        }
    }
}

// Define the traits module for external interfaces
pub mod traits {
    use super::*;
    use frame_support::dispatch::DispatchResult;

    // Re-export types from critter-nfts pallet
    pub type PetId = u32;

    // Trait for interacting with pet NFTs
    pub trait NftManagerForDaycare<AccountId, NftId, Result> {
        fn is_owner(owner: &AccountId, pet_id: &NftId) -> bool;
        fn update_pet_state(pet_id: &NftId, interaction_type: &[u8]) -> Result;
    }
}

// Benchmarking module (empty for now)
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use super::*;
    use frame_benchmarking::{benchmarks, whitelisted_caller, account};
    use frame_system::RawOrigin;

    benchmarks! {
        // Benchmarks would be defined here
    }
}