#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Time, Randomness},
        sp_runtime::traits::Zero,
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_runtime::{Perbill, traits::AtLeast32BitUnsigned};
    use sp_std::{vec::Vec, prelude::*};

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    type MomentOf<T> = <<T as Config>::TimeProvider as Time>::Moment;

    /// Enum defining the types of content that can be created
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ContentType {
        CritterSkin,
        Accessory,
        ItemDesign,
        EnvironmentTheme,
        QuestTemplate,
        Other,
    }

    /// Enum defining the status of content
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ContentStatus {
        Pending,
        Approved,
        Rejected,
        Flagged,
    }

    /// Struct to hold details of content
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct ContentDetails<AccountId, Balance, Moment, BoundedString> {
        pub creator: AccountId,
        pub content_type: ContentType,
        pub name: BoundedString,
        pub description: BoundedString,
        pub uri: BoundedString,
        pub content_hash: [u8; 32],
        pub royalty_percentage: u8,
        pub status: ContentStatus,
        pub created_at: Moment,
        pub approved_at: Option<Moment>,
        pub total_earnings: Balance,
        pub purchase_count: u32,
        pub usage_count: u32,
    }

    /// Struct to hold moderation details
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct ModerationDetails<AccountId, Moment, BoundedString> {
        pub moderator: AccountId,
        pub moderated_at: Moment,
        pub reason: Option<BoundedString>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        
        /// The time provider
        type TimeProvider: Time;
        
        /// The content ID type
        type ContentId: Parameter + Member + Copy + MaybeSerializeDeserialize + MaxEncodedLen + Default + Ord + AtLeast32BitUnsigned;
        
        /// Randomness source for content ID generation
        type ContentRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
        
        /// Maximum length for content names
        #[pallet::constant]
        type MaxNameLength: Get<u32>;
        
        /// Maximum length for content descriptions
        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;
        
        /// Maximum length for content URIs
        #[pallet::constant]
        type MaxUriLength: Get<u32>;
        
        /// Maximum length for moderation reasons
        #[pallet::constant]
        type MaxReasonLength: Get<u32>;
        
        /// Required deposit for content submission (refundable if approved)
        #[pallet::constant]
        type ContentSubmissionDeposit: Get<BalanceOf<Self>>;
        
        /// Maximum allowed royalty percentage (0-100)
        #[pallet::constant]
        type MaxRoyaltyPercentage: Get<u8>;
        
        /// Account for the community treasury
        #[pallet::constant]
        type CommunityTreasuryAccountId: Get<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Type for bounded strings
    pub type BoundedName<T> = BoundedVec<u8, <T as Config>::MaxNameLength>;
    pub type BoundedDescription<T> = BoundedVec<u8, <T as Config>::MaxDescriptionLength>;
    pub type BoundedUri<T> = BoundedVec<u8, <T as Config>::MaxUriLength>;
    pub type BoundedReason<T> = BoundedVec<u8, <T as Config>::MaxReasonLength>;

    #[pallet::storage]
    #[pallet::getter(fn next_content_id)]
    pub(super) type NextContentId<T: Config> = StorageValue<_, T::ContentId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn content)]
    /// Stores content details. Maps ContentId to ContentDetails.
    pub(super) type Content<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::ContentId,
        ContentDetails<
            T::AccountId,
            BalanceOf<T>,
            MomentOf<T>,
            BoundedName<T>
        >,
    >;

    #[pallet::storage]
    #[pallet::getter(fn content_descriptions)]
    /// Stores content descriptions separately to save storage space
    pub(super) type ContentDescriptions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::ContentId,
        BoundedDescription<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn content_uris)]
    /// Stores content URIs separately to save storage space
    pub(super) type ContentUris<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::ContentId,
        BoundedUri<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn content_moderation)]
    /// Stores moderation details for content
    pub(super) type ContentModeration<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::ContentId,
        ModerationDetails<
            T::AccountId,
            MomentOf<T>,
            BoundedReason<T>
        >,
    >;

    #[pallet::storage]
    #[pallet::getter(fn creator_content)]
    /// Maps creator to their content IDs
    pub(super) type CreatorContent<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<T::ContentId, ConstU32<1000>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pending_content)]
    /// Stores content IDs that are pending moderation
    pub(super) type PendingContent<T: Config> = StorageValue<
        _,
        BoundedVec<T::ContentId, ConstU32<1000>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn approved_content)]
    /// Stores content IDs that have been approved
    pub(super) type ApprovedContent<T: Config> = StorageValue<
        _,
        BoundedVec<T::ContentId, ConstU32<10000>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn flagged_content)]
    /// Stores content IDs that have been flagged
    pub(super) type FlaggedContent<T: Config> = StorageValue<
        _,
        BoundedVec<T::ContentId, ConstU32<1000>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn content_by_type)]
    /// Maps content type to content IDs
    pub(super) type ContentByType<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ContentType,
        BoundedVec<T::ContentId, ConstU32<10000>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn moderators)]
    /// Set of accounts that have moderation privileges
    pub(super) type Moderators<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        bool,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New content has been submitted
        ContentSubmitted {
            content_id: T::ContentId,
            creator: T::AccountId,
            content_type: ContentType,
        },
        /// Content has been moderated
        ContentModerated {
            content_id: T::ContentId,
            moderator: T::AccountId,
            status: ContentStatus,
        },
        /// Content has been purchased
        ContentPurchased {
            content_id: T::ContentId,
            buyer: T::AccountId,
            price: BalanceOf<T>,
        },
        /// Content has been used
        ContentUsed {
            content_id: T::ContentId,
            user: T::AccountId,
        },
        /// Royalty has been paid to a creator
        RoyaltyPaid {
            content_id: T::ContentId,
            creator: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// A moderator has been added
        ModeratorAdded {
            account: T::AccountId,
        },
        /// A moderator has been removed
        ModeratorRemoved {
            account: T::AccountId,
        },
        /// Content has been updated
        ContentUpdated {
            content_id: T::ContentId,
            creator: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The content ID already exists
        ContentIdAlreadyExists,
        /// The content ID does not exist
        ContentIdDoesNotExist,
        /// The caller is not the creator of the content
        NotContentCreator,
        /// The caller is not a moderator
        NotModerator,
        /// The content has already been moderated
        ContentAlreadyModerated,
        /// The content is not in the pending state
        ContentNotPending,
        /// The content is not in the approved state
        ContentNotApproved,
        /// The royalty percentage is too high
        RoyaltyPercentageTooHigh,
        /// The deposit is insufficient
        InsufficientDeposit,
        /// The content hash is invalid
        InvalidContentHash,
        /// The content URI is invalid
        InvalidContentUri,
        /// The content name is invalid
        InvalidContentName,
        /// The content description is invalid
        InvalidContentDescription,
        /// Too many content items for this creator
        TooManyContentItems,
        /// Too many pending content items
        TooManyPendingItems,
        /// Too many approved content items
        TooManyApprovedItems,
        /// Too many flagged content items
        TooManyFlaggedItems,
        /// Too many content items of this type
        TooManyContentItemsOfType,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit new content
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn submit_content(
            origin: OriginFor<T>,
            content_type: ContentType,
            name: Vec<u8>,
            description: Vec<u8>,
            uri: Vec<u8>,
            content_hash: [u8; 32],
            royalty_percentage: u8,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;
            
            // Validate inputs
            ensure!(royalty_percentage <= T::MaxRoyaltyPercentage::get(), Error::<T>::RoyaltyPercentageTooHigh);
            
            // Convert to bounded types
            let name = BoundedName::<T>::try_from(name)
                .map_err(|_| Error::<T>::InvalidContentName)?;
            
            let description = BoundedDescription::<T>::try_from(description)
                .map_err(|_| Error::<T>::InvalidContentDescription)?;
            
            let uri = BoundedUri::<T>::try_from(uri)
                .map_err(|_| Error::<T>::InvalidContentUri)?;
            
            // Reserve the deposit
            T::Currency::reserve(&creator, T::ContentSubmissionDeposit::get())
                .map_err(|_| Error::<T>::InsufficientDeposit)?;
            
            // Generate a new content ID
            let content_id = NextContentId::<T>::try_mutate(|id| -> Result<T::ContentId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(&T::ContentId::from(1u32)).ok_or(Error::<T>::ContentIdAlreadyExists)?;
                Ok(current_id)
            })?;
            
            // Create content details
            let now = T::TimeProvider::now();
            let content_details = ContentDetails {
                creator: creator.clone(),
                content_type: content_type.clone(),
                name,
                description: BoundedName::<T>::default(), // Placeholder, actual description stored separately
                uri: BoundedName::<T>::default(), // Placeholder, actual URI stored separately
                content_hash,
                royalty_percentage,
                status: ContentStatus::Pending,
                created_at: now,
                approved_at: None,
                total_earnings: BalanceOf::<T>::zero(),
                purchase_count: 0,
                usage_count: 0,
            };
            
            // Store the content
            Content::<T>::insert(content_id, content_details);
            ContentDescriptions::<T>::insert(content_id, description);
            ContentUris::<T>::insert(content_id, uri);
            
            // Add to creator's content
            CreatorContent::<T>::try_mutate(&creator, |contents| -> DispatchResult {
                contents.try_push(content_id).map_err(|_| Error::<T>::TooManyContentItems)?;
                Ok(())
            })?;
            
            // Add to pending content
            PendingContent::<T>::try_mutate(|contents| -> DispatchResult {
                contents.try_push(content_id).map_err(|_| Error::<T>::TooManyPendingItems)?;
                Ok(())
            })?;
            
            // Add to content by type
            ContentByType::<T>::try_mutate(content_type.clone(), |contents| -> DispatchResult {
                contents.try_push(content_id).map_err(|_| Error::<T>::TooManyContentItemsOfType)?;
                Ok(())
            })?;
            
            // Emit event
            Self::deposit_event(Event::ContentSubmitted {
                content_id,
                creator,
                content_type,
            });
            
            Ok(())
        }
        
        /// Moderate content (approve, reject, or flag)
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn moderate_content(
            origin: OriginFor<T>,
            content_id: T::ContentId,
            status: ContentStatus,
            reason: Option<Vec<u8>>,
        ) -> DispatchResult {
            let moderator = ensure_signed(origin)?;
            
            // Ensure the caller is a moderator
            ensure!(Moderators::<T>::get(&moderator), Error::<T>::NotModerator);
            
            // Retrieve the content
            let mut content = Content::<T>::get(content_id).ok_or(Error::<T>::ContentIdDoesNotExist)?;
            
            // Ensure the content is in the pending state or flagged state
            ensure!(
                content.status == ContentStatus::Pending || content.status == ContentStatus::Flagged,
                Error::<T>::ContentNotPending
            );
            
            // Update the content status
            content.status = status.clone();
            
            // If approved, set approved_at timestamp and unreserve the deposit
            if status == ContentStatus::Approved {
                content.approved_at = Some(T::TimeProvider::now());
                T::Currency::unreserve(&content.creator, T::ContentSubmissionDeposit::get());
                
                // Remove from pending content
                PendingContent::<T>::try_mutate(|contents| -> DispatchResult {
                    if let Some(pos) = contents.iter().position(|id| *id == content_id) {
                        contents.swap_remove(pos);
                    }
                    Ok(())
                })?;
                
                // Add to approved content
                ApprovedContent::<T>::try_mutate(|contents| -> DispatchResult {
                    contents.try_push(content_id).map_err(|_| Error::<T>::TooManyApprovedItems)?;
                    Ok(())
                })?;
            } else if status == ContentStatus::Rejected {
                // If rejected, slash the deposit
                T::Currency::slash_reserved(&content.creator, T::ContentSubmissionDeposit::get());
                
                // Remove from pending content
                PendingContent::<T>::try_mutate(|contents| -> DispatchResult {
                    if let Some(pos) = contents.iter().position(|id| *id == content_id) {
                        contents.swap_remove(pos);
                    }
                    Ok(())
                })?;
            } else if status == ContentStatus::Flagged {
                // If flagged, add to flagged content
                FlaggedContent::<T>::try_mutate(|contents| -> DispatchResult {
                    contents.try_push(content_id).map_err(|_| Error::<T>::TooManyFlaggedItems)?;
                    Ok(())
                })?;
                
                // Remove from pending content if it was pending
                if content.status == ContentStatus::Pending {
                    PendingContent::<T>::try_mutate(|contents| -> DispatchResult {
                        if let Some(pos) = contents.iter().position(|id| *id == content_id) {
                            contents.swap_remove(pos);
                        }
                        Ok(())
                    })?;
                }
            }
            
            // Update the content
            Content::<T>::insert(content_id, content);
            
            // Create moderation details
            let now = T::TimeProvider::now();
            let bounded_reason = if let Some(r) = reason {
                Some(BoundedReason::<T>::try_from(r).map_err(|_| Error::<T>::InvalidContentDescription)?)
            } else {
                None
            };
            
            let moderation_details = ModerationDetails {
                moderator: moderator.clone(),
                moderated_at: now,
                reason: bounded_reason,
            };
            
            // Store moderation details
            ContentModeration::<T>::insert(content_id, moderation_details);
            
            // Emit event
            Self::deposit_event(Event::ContentModerated {
                content_id,
                moderator,
                status,
            });
            
            Ok(())
        }
        
        /// Update content (only allowed for approved content)
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn update_content(
            origin: OriginFor<T>,
            content_id: T::ContentId,
            name: Option<Vec<u8>>,
            description: Option<Vec<u8>>,
            uri: Option<Vec<u8>>,
            content_hash: Option<[u8; 32]>,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;
            
            // Retrieve the content
            let mut content = Content::<T>::get(content_id).ok_or(Error::<T>::ContentIdDoesNotExist)?;
            
            // Ensure the caller is the creator
            ensure!(content.creator == creator, Error::<T>::NotContentCreator);
            
            // Ensure the content is approved
            ensure!(content.status == ContentStatus::Approved, Error::<T>::ContentNotApproved);
            
            // Update the content details
            if let Some(new_name) = name {
                let bounded_name = BoundedName::<T>::try_from(new_name)
                    .map_err(|_| Error::<T>::InvalidContentName)?;
                content.name = bounded_name;
            }
            
            if let Some(new_description) = description {
                let bounded_description = BoundedDescription::<T>::try_from(new_description)
                    .map_err(|_| Error::<T>::InvalidContentDescription)?;
                ContentDescriptions::<T>::insert(content_id, bounded_description);
            }
            
            if let Some(new_uri) = uri {
                let bounded_uri = BoundedUri::<T>::try_from(new_uri)
                    .map_err(|_| Error::<T>::InvalidContentUri)?;
                ContentUris::<T>::insert(content_id, bounded_uri);
            }
            
            if let Some(new_content_hash) = content_hash {
                content.content_hash = new_content_hash;
            }
            
            // Update the content
            Content::<T>::insert(content_id, content);
            
            // Emit event
            Self::deposit_event(Event::ContentUpdated {
                content_id,
                creator,
            });
            
            Ok(())
        }
        
        /// Record content purchase (called by marketplace)
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn record_purchase(
            origin: OriginFor<T>,
            content_id: T::ContentId,
            buyer: T::AccountId,
            price: BalanceOf<T>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            
            // In a production system, we would verify that the caller is the marketplace pallet
            // For simplicity, we're skipping that check here
            
            // Retrieve the content
            let mut content = Content::<T>::get(content_id).ok_or(Error::<T>::ContentIdDoesNotExist)?;
            
            // Ensure the content is approved
            ensure!(content.status == ContentStatus::Approved, Error::<T>::ContentNotApproved);
            
            // Update purchase count and total earnings
            content.purchase_count = content.purchase_count.saturating_add(1);
            content.total_earnings = content.total_earnings.saturating_add(price);
            
            // Update the content
            Content::<T>::insert(content_id, content);
            
            // Emit event
            Self::deposit_event(Event::ContentPurchased {
                content_id,
                buyer,
                price,
            });
            
            Ok(())
        }
        
        /// Record content usage (called by game logic)
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn record_usage(
            origin: OriginFor<T>,
            content_id: T::ContentId,
            user: T::AccountId,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            
            // In a production system, we would verify that the caller is authorized
            // For simplicity, we're skipping that check here
            
            // Retrieve the content
            let mut content = Content::<T>::get(content_id).ok_or(Error::<T>::ContentIdDoesNotExist)?;
            
            // Ensure the content is approved
            ensure!(content.status == ContentStatus::Approved, Error::<T>::ContentNotApproved);
            
            // Update usage count
            content.usage_count = content.usage_count.saturating_add(1);
            
            // Update the content
            Content::<T>::insert(content_id, content);
            
            // Emit event
            Self::deposit_event(Event::ContentUsed {
                content_id,
                user,
            });
            
            Ok(())
        }
        
        /// Add a moderator (must be called by root)
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn add_moderator(
            origin: OriginFor<T>,
            account: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            // Add the account to moderators
            Moderators::<T>::insert(&account, true);
            
            // Emit event
            Self::deposit_event(Event::ModeratorAdded {
                account,
            });
            
            Ok(())
        }
        
        /// Remove a moderator (must be called by root)
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn remove_moderator(
            origin: OriginFor<T>,
            account: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            // Remove the account from moderators
            Moderators::<T>::remove(&account);
            
            // Emit event
            Self::deposit_event(Event::ModeratorRemoved {
                account,
            });
            
            Ok(())
        }
    }
    
    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Get content creator
        pub fn get_content_creator(content_id: &T::ContentId) -> Option<T::AccountId> {
            Content::<T>::get(content_id).map(|content| content.creator)
        }
        
        /// Get royalty percentage
        pub fn get_royalty_percentage(content_id: &T::ContentId) -> Option<u8> {
            Content::<T>::get(content_id).map(|content| content.royalty_percentage)
        }
        
        /// Pay royalty to content creator
        pub fn pay_royalty(content_id: &T::ContentId, amount: BalanceOf<T>) -> DispatchResult {
            if let Some(content) = Content::<T>::get(content_id) {
                // Calculate royalty amount
                let royalty_amount = amount.saturating_mul(content.royalty_percentage.into()) / 100u32.into();
                
                if royalty_amount > BalanceOf::<T>::zero() {
                    // Transfer royalty to creator
                    T::Currency::transfer(
                        &T::CommunityTreasuryAccountId::get(),
                        &content.creator,
                        royalty_amount,
                        ExistenceRequirement::KeepAlive
                    )?;
                    
                    // Update content earnings
                    Content::<T>::try_mutate(content_id, |content_opt| -> DispatchResult {
                        if let Some(content) = content_opt {
                            content.total_earnings = content.total_earnings.saturating_add(royalty_amount);
                        }
                        Ok(())
                    })?;
                    
                    // Emit event
                    Self::deposit_event(Event::RoyaltyPaid {
                        content_id: *content_id,
                        creator: content.creator,
                        amount: royalty_amount,
                    });
                }
                
                Ok(())
            } else {
                Err(Error::<T>::ContentIdDoesNotExist.into())
            }
        }
    }
}

// Implementation of ContentRoyaltyManager trait for integration with marketplace
impl<T: Config> crate::ContentRoyaltyManager<T::AccountId, T::ContentId, BalanceOf<T>, DispatchResult> for Pallet<T> {
    fn get_content_creator(content_id: &T::ContentId) -> Option<T::AccountId> {
        Self::get_content_creator(content_id)
    }
    
    fn get_royalty_percentage(content_id: &T::ContentId) -> Option<u8> {
        Self::get_royalty_percentage(content_id)
    }
    
    fn pay_royalty(content_id: &T::ContentId, amount: BalanceOf<T>) -> DispatchResult {
        Self::pay_royalty(content_id, amount)
    }
}

// Define the ContentRoyaltyManager trait here for reference by the marketplace pallet
pub trait ContentRoyaltyManager<AccountId, ContentId, Balance, DispatchResult> {
    fn get_content_creator(content_id: &ContentId) -> Option<AccountId>;
    fn get_royalty_percentage(content_id: &ContentId) -> Option<u8>;
    fn pay_royalty(content_id: &ContentId, amount: Balance) -> DispatchResult;
}