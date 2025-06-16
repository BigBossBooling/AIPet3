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
        traits::{Currency, ExistenceRequirement, OnUnbalanced},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::Perbill; // For Perbill type for fees
    use scale_info::TypeInfo;
    // Import the NftManager trait defined above
    use super::NftManager; // This refers to the NftManager trait defined outside this module


    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    // Conceptual: Type alias for negative imbalance, used with OnUnbalanced for fee handling.
    // type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;


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

        // SYNERGY: Marketplace Fee Configuration
        // #[pallet::constant]
        // type MarketplaceFeeRate: Get<Perbill>; // e.g., Perbill::from_percent(1) for 1%
        // type FeeDestination: OnUnbalanced<NegativeImbalanceOf<Self>>; // Where fees go (e.g., Treasury, Burn)
        // Or, if simpler, an AccountId to transfer fees to:
        // type FeeCollectorAccountId: Get<Self::AccountId>;
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

            // SYNERGY: Check seller's trade_reputation_score from pallet-user-profile
            // // let seller_profile = pallet_user_profile::Pallet::<T>::user_profiles(&seller); // Requires T: pallet_user_profile::Config
            // // if seller_profile.trade_reputation_score < MIN_REP_TO_LIST_CONSTANT {
            // //     return Err(Error::<T>::SellerReputationTooLow.into()); // Conceptual error
            // // }

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

            // --- Conceptual Fee Logic ---
            // let sale_price = listing.price;
            // let fee = T::MarketplaceFeeRate::get() * sale_price; // Perbill multiplication needs careful handling of Balance type
            // let price_after_fee = sale_price.saturating_sub(fee);

            // // Option 1: Direct transfer to FeeCollectorAccountId (simpler)
            // // if fee > BalanceOf::<T>::zero() {
            // //     T::Currency::transfer(&buyer, &T::FeeCollectorAccountId::get(), fee, ExistenceRequirement::AllowDeath)?;
            // // }
            // // T::Currency::transfer(&buyer, &listing.seller, price_after_fee, ExistenceRequirement::KeepAlive)?;
            // // This means buyer pays `sale_price + fee` or pallet needs to manage an intermediate account.

            // // Option 2: Buyer pays full price to seller, then seller pays fee (more complex for seller) - not ideal.

            // // Option 3: Buyer pays full price, pallet intercepts fee.
            // // This would require the pallet to have a sovereign account or use imbalances.
            // // Total amount to be withdrawn from buyer:
            // // T::Currency::withdraw(&buyer, sale_price, WithdrawReasons::TRANSACTION_PAYMENT, ExistenceRequirement::KeepAlive)?;
            // // T::Currency::deposit_creating(&listing.seller, price_after_fee); // Simplified deposit
            // // T::FeeDestination::on_unbalanced(T::Currency::issue(fee)); // If FeeDestination handles Imbalance

            // For this conceptual stage, we'll proceed with the original direct transfer logic
            // and add a NOTE that fees would alter this flow by splitting the `listing.price`.
            // The actual implementation detail (e.g. pallet sovereign account, or buyer pays seller and then fee separately)
            // would be decided during full implementation. The critical point is acknowledging the fee.
            // A comment will suffice here to indicate where fee logic would apply.
            // NOTE ON FEE: At this point, the `listing.price` would be split.
            // `price_to_seller = listing.price - fee`. `fee` goes to `T::FeeDestination`.
            // The T::Currency::transfer below would use `price_to_seller`.
            // An additional transfer or imbalance handling would manage the `fee`.

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

            // SYNERGY: After successful trade, call pallet-user-profile to update trade stats for buyer and seller
            // // pallet_user_profile::Pallet::<T>::record_successful_trade(&buyer)?; // Requires T: pallet_user_profile::Config
            // // pallet_user_profile::Pallet::<T>::record_successful_trade(&listing.seller)?;

            // SYNERGY: (Future) Could also update buyer's/seller's trade_reputation_score based on this successful transaction
            // // pallet_user_profile::Pallet::<T>::update_trade_reputation(&buyer, POSITIVE_TRADE_REP_CHANGE)?;
            // // pallet_user_profile::Pallet::<T>::update_trade_reputation(&listing.seller, POSITIVE_TRADE_REP_CHANGE)?;


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
