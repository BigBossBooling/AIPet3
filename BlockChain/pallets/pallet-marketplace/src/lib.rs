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
        traits::{Currency, ExistenceRequirement, OnUnbalanced, Imbalance},
    };
    use frame_system::pallet_prelude::*;
    // Perbill commented out as MarketplaceFeeRate is deferred for MVP
    // use sp_runtime::Perbill;
    use scale_info::TypeInfo;
    use super::NftManager;


    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    // Conceptual: Type alias for negative imbalance, used with OnUnbalanced for fee handling.
    type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;


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

        // MVP Fee Configuration: Fixed fee or zero fee.
        #[pallet::constant]
        type MarketplaceFixedFee: Get<BalanceOf<Self>>;
        // type MarketplaceFeeRate: Get<Perbill>; // Deferred for MVP

        /// AccountId for the fee destination (e.g., Treasury account).
        /// Used if MarketplaceFixedFee > 0.
        #[pallet::constant]
        type FeeDestinationAccountId: Get<Self::AccountId>;
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
        NftAlreadyListed,
        NftNotTransferable,
        LockNftFailed,
        PriceMustBeGreaterThanZero,
        ListingNotFound,
        NotSeller,
        UnlockNftFailed,
        BuyerIsSeller,
        InsufficientBalance, // Although T::Currency::transfer handles this, an explicit error can be useful for UI.
        TransferFailed, // Generic failure for currency or NFT transfer.
        FeePaymentFailed, // If the seller cannot pay the marketplace fee from the sale proceeds.
        PriceTooLowToCoverFeeAndSellerPayment, // If price <= fee, meaning seller gets nothing or negative.
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

            // 1. Ensure price is greater than zero.
            ensure!(price > BalanceOf::<T>::from(0u32), Error::<T>::PriceMustBeGreaterThanZero);

            // 2. Check if the NFT is already listed.
            ensure!(!Listings::<T>::contains_key(&pet_id), Error::<T>::NftAlreadyListed);

            // 3. Verify ownership of the NFT.
            let owner = T::NftHandler::owner_of(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(owner == seller, Error::<T>::NotNftOwner);

            // 4. Check if the NFT is transferable (not locked by other means).
            ensure!(T::NftHandler::is_transferable(&pet_id), Error::<T>::NftNotTransferable);

            // 5. Lock the NFT to prevent transfers while listed.
            T::NftHandler::lock_nft(&seller, &pet_id).map_err(|_| Error::<T>::LockNftFailed)?;

            // 6. Create listing details.
            let listing_details = ListingDetails {
                seller: seller.clone(),
                price,
            };

            // 7. Store the listing.
            Listings::<T>::insert(&pet_id, listing_details);

            // 8. Emit event.
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

            // 1. Retrieve the listing.
            let listing_details = Listings::<T>::get(&pet_id).ok_or(Error::<T>::ListingNotFound)?;

            // 2. Verify that the caller is the seller.
            ensure!(listing_details.seller == signer, Error::<T>::NotSeller);

            // 3. Attempt to unlock the NFT via NftHandler.
            // The owner passed to unlock_nft should be the original seller who locked it.
            T::NftHandler::unlock_nft(&listing_details.seller, &pet_id)
                .map_err(|_| Error::<T>::UnlockNftFailed)?;

            // 4. Remove the listing from storage.
            Listings::<T>::remove(&pet_id);

            // 5. Emit event.
            Self::deposit_event(Event::NftUnlisted { seller: signer, pet_id });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1) + T::DbWeight::get().reads_writes(1,1) + T::DbWeight::get().writes(1))]
        pub fn buy_nft(
            origin: OriginFor<T>,
            pet_id: T::PetId,
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;

            // 1. Retrieve the listing.
            let listing = Listings::<T>::get(&pet_id).ok_or(Error::<T>::ListingNotFound)?;

            // 2. Ensure buyer is not the seller.
            ensure!(buyer != listing.seller, Error::<T>::BuyerIsSeller);

            // 3. Handle fees and calculate amount for seller.
            let sale_price = listing.price;
            let fixed_fee = T::MarketplaceFixedFee::get();
            let amount_to_seller;

            if fixed_fee > BalanceOf::<T>::from(0u32) {
                // Ensure the sale price can at least cover the fee, and ideally leave something for the seller.
                // For MVP, we'll ensure price > fee for the transaction to proceed if a fee is active.
                // This means seller must get a non-zero amount.
                ensure!(sale_price > fixed_fee, Error::<T>::PriceTooLowToCoverFeeAndSellerPayment);
                amount_to_seller = sale_price.saturating_sub(fixed_fee);
            } else {
                // No fee or zero fee.
                amount_to_seller = sale_price;
            }

            // 4. Perform currency transfer: Buyer pays `sale_price`.
            // Seller receives `amount_to_seller`. Fee (if any) goes to `FeeDestinationAccountId`.

            // Step 4a: Transfer the main portion of the sale price from buyer to seller.
            T::Currency::transfer(&buyer, &listing.seller, amount_to_seller, ExistenceRequirement::KeepAlive)
                .map_err(|_| Error::<T>::TransferFailed)?; // This covers buyer's insufficient balance for amount_to_seller.

            // Step 4b: If there's a fee, transfer it from buyer to FeeDestinationAccountId.
            // This model means the buyer explicitly pays the fee on top of the amount that goes to the seller.
            // The total cost for the buyer is `amount_to_seller + fixed_fee`, which equals `sale_price`.
            if fixed_fee > BalanceOf::<T>::from(0u32) {
                 T::Currency::transfer(&buyer, &T::FeeDestinationAccountId::get(), fixed_fee, ExistenceRequirement::KeepAlive)
                    .map_err(|_| Error::<T>::FeePaymentFailed)?; // Error if buyer cannot cover the fee.
            }
            // Note: The above logic ensures buyer pays the full 'sale_price' which is then split.
            // If buyer did not have `sale_price` initially, the first transfer for `amount_to_seller` might fail,
            // or if they had just enough for `amount_to_seller` but not the `fixed_fee`, the second transfer would fail.
            // A single withdrawal from buyer to pallet, then split, is more robust but adds pallet account complexity.
            // The current model is: Buyer needs `amount_to_seller + fixed_fee` (i.e. `sale_price`).
            // The previous model (buyer pays seller, seller pays fee) is also viable but shifts risk.
            // Let's stick to: Buyer pays seller `amount_to_seller`, buyer pays fee collector `fixed_fee`. Total buyer cost = `sale_price`.

            // 5. Perform NFT transfer from seller to buyer using NftHandler.
            // This assumes NftHandler's transfer_nft also handles unlocking (or that marketplace pallet calls unlock first if needed).
            // Based on NftManager trait, transfer_nft does not manage locks itself.
            // The NFT was locked at listing. It should be unlocked here before transfer by NftHandler,
            // or NftHandler::transfer_nft must be capable of transferring a locked NFT if called by an authorized pallet like this one.
            // For MVP, we assume NftHandler::transfer_nft will succeed if the NFT is locked by this marketplace.
            // A stricter flow:
            // T::NftHandler::unlock_nft(&listing.seller, &pet_id).map_err(|_| Error::<T>::UnlockNftFailed)?; // Unlock before transfer
            // T::NftHandler::transfer_nft(&listing.seller, &buyer, &pet_id).map_err(|_| Error::<T>::TransferFailed)?;
            // However, our NftManager::transfer_nft assumes caller handles locks.
            // The lock made by list_nft_for_sale needs to be undone.
            // The NftHandler::transfer_nft should ideally be called on an unlocked NFT.
            // So, this pallet must call unlock_nft first.
            T::NftHandler::unlock_nft(&listing.seller, &pet_id).map_err(|_| Error::<T>::UnlockNftFailed)?;
            T::NftHandler::transfer_nft(&listing.seller, &buyer, &pet_id)
                .map_err(|_| Error::<T>::TransferFailed)?;

            // 5. Remove the listing from storage.
            Listings::<T>::remove(&pet_id);

            // 6. Emit event.
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
