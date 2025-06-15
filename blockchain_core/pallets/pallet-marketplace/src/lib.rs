#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Conceptual NftManager trait (defined outside the pallet module or inside for now)
// For the purpose of this file structure, it's defined here.
// In a multi-crate workspace, it might be in its own crate or in the NFT pallet's public interface.
/// A trait to handle NFT operations, expected to be implemented by the NFT pallet.
pub trait NftManager<AccountId, PetId, DispatchResult> {
    fn owner_of(pet_id: &PetId) -> Option<AccountId>;
    fn is_transferable(pet_id: &PetId) -> bool;
    fn lock_nft(owner: &AccountId, pet_id: &PetId) -> DispatchResult;
    fn unlock_nft(owner: &AccountId, pet_id: &PetId) -> DispatchResult; // Added for completeness, though not used in list_nft
    fn transfer_nft(from: &AccountId, to: &AccountId, pet_id: &PetId) -> DispatchResult;
}


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement}, // Added ExistenceRequirement
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    // Import the NftManager trait defined above
    use super::NftManager; // This refers to the NftManager trait defined outside this module


    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct ListingDetails<AccountId, Balance> {
        pub seller: AccountId,
        pub price: Balance,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type PetId: Parameter + Member + Copy + MaybeSerializeDeserialize + MaxEncodedLen + Default + Ord;

        /// The handler for NFT operations, bridging to the NFT pallet.
        type NftHandler: NftManager<Self::AccountId, Self::PetId, DispatchResult>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn listings)]
    /// Stores active listings. Maps PetId to its ListingDetails.
    pub(super) type Listings<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::PetId, // Using T::PetId from Config
        ListingDetails<T::AccountId, BalanceOf<T>>,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An NFT has been listed for sale.
        NftListed { seller: T::AccountId, pet_id: T::PetId, price: BalanceOf<T> },
        /// An NFT has been unlisted from sale.
        NftUnlisted { seller: T::AccountId, pet_id: T::PetId },
        /// An NFT has been successfully bought and sold.
        NftSold { buyer: T::AccountId, seller: T::AccountId, pet_id: T::PetId, price: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The specified Pet NFT was not found (possibly never minted or burned).
        PetNotFound,
        /// The caller is not the owner of the Pet NFT they are trying to list.
        NotNftOwner,
        /// This Pet NFT is already listed for sale.
        NftAlreadyListed,
        /// The Pet NFT is not transferable (e.g., it's locked or soul-bound).
        NftNotTransferable,
        /// The attempt to lock the NFT via the NftHandler failed.
        LockNftFailed,
        /// The price for listing an NFT must be greater than zero.
        PriceMustBeGreaterThanZero,
        /// The specified listing was not found.
        ListingNotFound,
        /// The caller is not the seller of the listed NFT.
        NotSeller,
        /// The attempt to unlock the NFT via the NftHandler failed.
        UnlockNftFailed,
        /// A user cannot buy their own listed NFT.
        BuyerIsSeller,
        /// The buyer does not have enough balance to purchase the NFT.
        InsufficientBalance, // Note: T::Currency::transfer handles this, but explicit error can be useful
        /// The transfer of currency or NFT failed.
        TransferFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,1) + T::DbWeight::get().reads(2))] // Reads: ListingsMap, NftHandler::owner_of, NftHandler::is_transferable. Writes: ListingsMap, NftHandler::lock_nft
        pub fn list_nft_for_sale(
            origin: OriginFor<T>,
            pet_id: T::PetId,
            price: BalanceOf<T>,
        ) -> DispatchResult {
            let seller = ensure_signed(origin)?;

            // Ensure price is not zero
            ensure!(price > BalanceOf::<T>::from(0u32), Error::<T>::PriceMustBeGreaterThanZero);

            // Check if already listed
            ensure!(!Listings::<T>::contains_key(&pet_id), Error::<T>::NftAlreadyListed);

            // Verify ownership using the NftHandler
            let owner = T::NftHandler::owner_of(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(owner == seller, Error::<T>::NotNftOwner);

            // Check if transferable/lockable via NftHandler
            ensure!(T::NftHandler::is_transferable(&pet_id), Error::<T>::NftNotTransferable);

            // Attempt to lock the NFT via NftHandler
            T::NftHandler::lock_nft(&seller, &pet_id).map_err(|_dispatch_err| Error::<T>::LockNftFailed)?;
            // Note: if lock_nft returns a DispatchError, we're converting it.
            // A more robust error handling might involve specific errors from NftHandler if it defines its own error type.

            let listing_details = ListingDetails {
                seller: seller.clone(),
                price,
            };

            Listings::<T>::insert(&pet_id, listing_details);

            Self::deposit_event(Event::NftListed { seller, pet_id, price });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1) + T::DbWeight::get().writes(1))] // Reads: Listings, Writes: Listings, NftHandler::unlock_nft
        pub fn unlist_nft_from_sale(
            origin: OriginFor<T>,
            pet_id: T::PetId,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;

            // Retrieve the listing
            let listing_details = Listings::<T>::get(&pet_id).ok_or(Error::<T>::ListingNotFound)?;

            // Verify that the caller is the seller
            ensure!(listing_details.seller == signer, Error::<T>::NotSeller);

            // Attempt to unlock the NFT via NftHandler
            // Pass listing_details.seller as it's the verified owner who locked it.
            T::NftHandler::unlock_nft(&listing_details.seller, &pet_id).map_err(|_dispatch_err| Error::<T>::UnlockNftFailed)?;

            // Remove the listing from storage
            Listings::<T>::remove(&pet_id);

            // Deposit an event
            Self::deposit_event(Event::NftUnlisted { seller: signer, pet_id });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1) + T::DbWeight::get().reads_writes(1,1) + T::DbWeight::get().writes(1))] // Placeholder: R:Listings, W:Currency, W:NftHandler, W:Listings
        pub fn buy_nft(
            origin: OriginFor<T>,
            pet_id: T::PetId,
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;

            // Retrieve the listing
            let listing = Listings::<T>::get(&pet_id).ok_or(Error::<T>::ListingNotFound)?;

            // Ensure buyer is not the seller
            ensure!(buyer != listing.seller, Error::<T>::BuyerIsSeller);

            // Perform currency transfer from buyer to seller
            T::Currency::transfer(&buyer, &listing.seller, listing.price, ExistenceRequirement::KeepAlive)
                .map_err(|_dispatch_err| {
                    // Even though T::Currency::transfer returns its own error (often a TokenError),
                    // for simplicity in the marketplace pallet, we map it to a generic TransferFailed.
                    // A more advanced implementation might inspect _dispatch_err or have a more specific
                    // InsufficientBalance error if distinguishable.
                    Error::<T>::TransferFailed
                })?;

            // Perform NFT transfer from seller to buyer using NftHandler
            // This assumes NftHandler's transfer_nft also handles unlocking if applicable.
            T::NftHandler::transfer_nft(&listing.seller, &buyer, &pet_id)
                .map_err(|_dispatch_err| Error::<T>::TransferFailed)?;

            // Remove the listing from storage
            Listings::<T>::remove(&pet_id);

            // Deposit an event
            Self::deposit_event(Event::NftSold {
                buyer,
                seller: listing.seller,
                pet_id,
                price: listing.price
            });

            Ok(())
        }
    }
}
