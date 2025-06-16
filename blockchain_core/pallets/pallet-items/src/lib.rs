#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Assume PetId is u32, AccountId from frame_system::Config
// Assume NftManager trait from pallet_critter_nfts is accessible for applying effects
// use pallet_critter_nfts::{NftManager, PetId}; // Would be actual imports

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::Currency, // If items can be bought/sold from a system shop or have value
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_std::vec::Vec;

    // Local type aliases for clarity, assuming they match definitions elsewhere or are made generic in Config
    pub type PetId = u32; // Placeholder, ideally from T::PetId via Config or shared type
    // pub type PetAttributeType = pallet_critter_nfts::PetAttributeType; // If defined in critter_nfts
    // pub type PetTrait = Vec<u8>; // Placeholder for a personality trait string

    /// Enum defining the types of attributes a pet has that items might affect.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum PetAttributeType {
        BaseStrength, // Refers to the immutable base stat
        BaseAgility,
        BaseIntelligence,
        BaseVitality,
        CurrentStrength, // Placeholder for dynamic, current strength if tracked separately
        CurrentAgility,
        CurrentIntelligence,
        CurrentVitality,
        Level,
        ExperiencePoints,
        MoodIndicator,
        HungerStatus,
        EnergyStatus,
        Fertility, // For breeding items
    }

    /// Enum defining categories for items.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum ItemCategory {
        Consumable,    // Used once, e.g., potions, food
        Equipment,     // Can be equipped/unequipped, provides persistent bonus
        TraitModifier, // Special item to grant/change a personality trait
        FertilityBooster, // Specific to breeding
        Cosmetic,      // Changes appearance (conceptual)
    }

    /// Enum defining the possible effects an item can have.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ItemEffect<BlockNumber> { // Made generic for BlockNumber for duration
        AttributeBoost {
            attribute: PetAttributeType,
            value: i16, // Can be positive or negative
            duration: Option<BlockNumber>, // Duration in blocks, None for permanent
        },
        GrantPersonalityTrait {
            trait_to_grant: Vec<u8>, // The personality trait string
        },
        ModifyFertility {
            boost_percentage: u8, // e.g., 10 for 10%
            duration_eras: Option<u32>, // Duration in eras
        },
        // Future: ApplyCosmetic { cosmetic_id: u32 },
        // SYNERGY: Effect to grant temporary access to a special battle area or quest line
        // GrantAccess { feature_id: u32, duration: BlockNumber }
    }

    /// Struct to hold details of an item definition.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct ItemDetails<BlockNumber> { // Made generic for BlockNumber
        // item_id: ItemId, // ItemId will be the key in storage
        pub name: Vec<u8>,
        pub description: Vec<u8>,
        pub category: ItemCategory,
        pub effects: Vec<ItemEffect<BlockNumber>>, // Multiple effects possible per item, now generic
        pub max_stack: Option<u32>, // None for non-stackable (e.g., equipment), Some(count) for stackable
        // pub icon_url: Option<Vec<u8>>, // For UI
    }

    // Type alias for ItemId
    pub type ItemId = u32;
    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>; // For system shop or item values

        /// Handler for interacting with Pet NFTs to apply item effects.
        type NftHandler: super::NftManagerForItems< // Use super to access trait in parent module
            Self::AccountId,
            PetId, // Assuming PetId is u32 or defined in NftManagerForItems
            PetAttributeType, // Enum for which attribute to modify
            Vec<u8>, // For personality trait string
            BlockNumberFor<Self> // Pass BlockNumberFor<Self> for duration
        >;
        // Note: NftManagerForItems is a NEW conceptual trait pallet_critter_nfts would need to implement.
        // It would have methods like `apply_attribute_boost(pet_id, attribute, value, duration)`
        // and `grant_personality_trait(pet_id, trait_string)`.

        #[pallet::constant]
        type MaxItemNameLength: Get<u32>;
        #[pallet::constant]
        type MaxItemDescriptionLength: Get<u32>;
        #[pallet::constant]
        type MaxEffectsPerItem: Get<u32>; // BoundedVec for ItemDetails.effects would use this
        #[pallet::constant]
        type MaxTraitLength: Get<u32>; // For personality trait strings
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_item_id)]
    pub(super) type NextItemId<T: Config> = StorageValue<_, ItemId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn item_definitions)]
    /// Stores definitions of all available item types.
    pub(super) type ItemDefinitions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ItemId,
        ItemDetails<BlockNumberFor<T>>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn user_item_inventory)]
    /// Maps (AccountId, ItemId) to the quantity of that item the user owns.
    /// For non-stackable items (equipment), quantity would typically be 1.
    pub(super) type UserItemInventory<T: Config> = StorageMap<
        _,
        Twox64Concat,
        (T::AccountId, ItemId),
        u32, // Quantity
        ValueQuery,
    >;


    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ItemDefined { item_id: ItemId, name: Vec<u8>, category: ItemCategory },
        ItemUsedOnPet { user: T::AccountId, item_id: ItemId, pet_id: PetId, effects_applied: Vec<ItemEffect<BlockNumberFor<T>>> },
        ItemsTransferred { from: T::AccountId, to: T::AccountId, item_id: ItemId, quantity: u32 },
        // ItemPurchasedFromSystemShop { buyer: T::AccountId, item_id: ItemId, quantity: u32, cost: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        ItemIdOverflow,
        ItemNotFound,
        NotEnoughItemsInInventory,
        CannotApplyItemToTarget,
        TargetPetNotOwned,
        MaxStackExceeded,
        ItemEffectApplicationFailed,
        NameTooLong,
        DescriptionTooLong,
        TooManyEffects, // Should be enforced by BoundedVec on ItemDetails.effects input
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Admin function to define a new type of item available in the game.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn admin_add_item_definition(
            origin: OriginFor<T>,
            name: Vec<u8>,
            description: Vec<u8>,
            category: ItemCategory,
            effects: Vec<ItemEffect<BlockNumberFor<T>>>, // Should be BoundedVec
            max_stack: Option<u32>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(name.len() <= T::MaxItemNameLength::get() as usize, Error::<T>::NameTooLong);
            ensure!(description.len() <= T::MaxItemDescriptionLength::get() as usize, Error::<T>::DescriptionTooLong);
            // ensure!(effects.len() <= T::MaxEffectsPerItem::get() as usize, Error::<T>::TooManyEffects);
            // This ^ should be handled by using BoundedVec<_, T::MaxEffectsPerItem> in the extrinsic signature

            let item_id = NextItemId::<T>::try_mutate(|id| -> Result<ItemId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(1).ok_or(Error::<T>::ItemIdOverflow)?;
                Ok(current_id)
            })?;

            let item_details = ItemDetails {
                name: name.clone(),
                description,
                category,
                effects, // If effects is BoundedVec, this is fine
                max_stack,
            };

            ItemDefinitions::<T>::insert(item_id, item_details);
            Self::deposit_event(Event::ItemDefined { item_id, name, category });
            Ok(())
        }

        /// Allows a user to apply/use an owned item on one of their pets.
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn user_apply_item_to_pet(
            origin: OriginFor<T>,
            item_id: ItemId,
            target_pet_id: PetId,
        ) -> DispatchResult {
            let user = ensure_signed(origin)?;

            // 1. Check if item_id is valid (exists in ItemDefinitions)
            //    let item_details = ItemDefinitions::<T>::get(item_id).ok_or(Error::<T>::ItemNotFound)?;

            // SYNERGY: Check UserProfile score for eligibility to use rare items (requires Config to access pallet-user-profile)
            // if T::UserProfileChecker::get_user_score(&user) < item_details.min_required_score {
            //     ensure!(false, Error::<T>::UserScoreTooLowForItem); // Conceptual error
            // }
            // Or, more directly if pallet-user-profile is a known type (less ideal than trait):
            // // let user_profile = pallet_user_profile::Pallet::<T>::user_profiles(&user); // This assumes T is also Config for pallet_user_profile
            // // ensure!(user_profile.overall_progress_score > item_details.min_required_score_conceptual, Error::<T>::UserScoreTooLowForItem);


            // 2. Check if user owns the item (UserItemInventory) and has enough quantity (e.g. > 0)
            //    UserItemInventory::<T>::try_mutate((&user, item_id), |quantity| -> DispatchResult {
            //        ensure!(*quantity > 0, Error::<T>::NotEnoughItemsInInventory);
            //        *quantity -= 1;
            //        Ok(())
            //    })?;

            // 3. Check if user owns target_pet_id (via T::NftHandler::owner_of_pet from pallet-critter-nfts)
            //    This part of NftHandlerForItems might just be a direct call to pallet_critter_nfts owner_of.
            //    Or NftHandlerForItems might have its own owner_of if it needs specific logic.
            //    Let's assume T::NftHandler has an `owner_of(pet_id)` method.
            //    let pet_owner = T::NftHandler::owner_of_pet(&target_pet_id).ok_or(Error::<T>::CannotApplyItemToTarget)?;
            //    ensure!(pet_owner == user, Error::<T>::TargetPetNotOwned);

            // 4. Apply effects via T::NftHandler
            //    For each effect in item_details.effects:
            //    match effect {
            //        ItemEffect::AttributeBoost { attribute, value, duration } => {
            //            T::NftHandler::apply_attribute_boost(&target_pet_id, attribute, value, duration, &user)?;
            //        },
            //        ItemEffect::GrantPersonalityTrait { trait_to_grant } => {
            //            T::NftHandler::grant_personality_trait(&target_pet_id, trait_to_grant, &user)?;
            //        },
            //        ItemEffect::ModifyFertility { boost_percentage, duration_eras } => {
            //             T::NftHandler::modify_pet_fertility(&target_pet_id, boost_percentage, &user)?;
            //        }
            //    }

            // 5. Emit Event::ItemUsedOnPet
            //    Self::deposit_event(Event::ItemUsedOnPet { user, item_id, pet_id: target_pet_id, effects_applied: item_details.effects });

            Ok(())
        }
    }
}

pub trait NftManagerForItems<AccountId, PetId, AttributeType, TraitType, BlockNumber> {
    fn apply_attribute_boost(
        pet_id: &PetId,
        attribute: AttributeType,
        value: i16,
        duration: Option<BlockNumber>,
        owner_check: &AccountId
    ) -> DispatchResult;

    fn grant_personality_trait(
        pet_id: &PetId,
        trait_to_grant: TraitType,
        owner_check: &AccountId
    ) -> DispatchResult;

    fn modify_pet_fertility(
        pet_id: &PetId,
        boost_percentage: u8,
        owner_check: &AccountId
    ) -> DispatchResult;
}
