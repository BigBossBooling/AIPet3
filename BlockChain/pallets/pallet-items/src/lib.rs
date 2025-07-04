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
        traits::Currency,
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_std::vec::Vec;

    pub type PetId = u32;
    pub type ItemId = u32;

    // This trait is implemented by pallet-items and called by pallet-critter-nfts
    // for basic feed/play item consumption.
    pub trait BasicCareItemConsumer<AccountId, LocalItemId> {
        fn consume_specific_item(
            user: &AccountId,
            item_id: LocalItemId,
            expected_category: ItemCategory // Use local ItemCategory
        ) -> DispatchResult;
    }

    /// Enum defining categories for items (Simplified for MVP).
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum ItemCategory {
        ConsumableCare,     // For basic feed/play actions, effects handled by critter_nfts based on its Config
        ConsumableBoost,    // For direct, often permanent or simple temporary stat boosts applied by this pallet via NftManagerForItems
        QuestItem,          // Key items for quests, may not have direct effects on pets
        BreedingAssist,     // E.g., Fertility boosters
        SpecialFunctional,  // E.g., Trait modifiers, items that unlock things
        // Deferred for Post-MVP: Equipment, Cosmetic
    }

    /// Enum defining the possible effects an item can have (Simplified for MVP).
    /// BlockNumberType removed as timed buffs are deferred.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ItemEffect {
        GrantFixedXp { amount: u32 },
        ModifyMood { amount: i16 }, // Direct change to mood_indicator in PetNft
        GrantPersonalityTrait { trait_to_grant: Vec<u8> }, // BoundedVec handled by NftHandler
        ModifyBreedingRelatedValue {
            effect_type_id: u8, // To differentiate specific breeding effects (e.g., 0 for fertility score, 1 for cooldown reduction)
            value: u32, // Value for the effect (e.g. fertility points, block number reduction)
        },
        // Deferred for Post-MVP: Complex AttributeBoost with duration/percentage, ApplyPermanentCharterBoost, ApplyCosmetic
    }

    /// Struct to hold details of an item definition.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct ItemDetails {
        pub name: Vec<u8>,
        pub description: Vec<u8>,
        pub category: ItemCategory,
        pub effects: Vec<ItemEffect>, // BoundedVec in practice via extrinsic input validation
        pub max_stack: Option<u32>,
    }

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        /// Handler for interacting with Pet NFTs to apply more complex item effects.
        /// Implemented by pallet-critter-nfts.
        type NftHandler: super::NftManagerForItems<
            Self::AccountId,
            PetId, // Assuming PetId is u32 from this pallet
            Vec<u8>, // Type for personality trait string
            DispatchResult
        >;
        // Note: PetAttributeType and BlockNumberType removed from trait generics due to simplification.

        #[pallet::constant]
        type MaxItemNameLength: Get<u32>;
        #[pallet::constant]
        type MaxItemDescriptionLength: Get<u32>;
        #[pallet::constant]
        type MaxEffectsPerItem: Get<u32>;
        #[pallet::constant]
        type MaxTraitStringLen: Get<u32>; // Renamed from MaxTraitLength for consistency
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
        ItemDetails, // Removed BlockNumberFor<T> generic from ItemDetails
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
        ItemUsedOnPet { user: T::AccountId, item_id: ItemId, pet_id: PetId, effects_applied: Vec<ItemEffect> }, // Removed BlockNumberFor<T>
        ItemsTransferred { from: T::AccountId, to: T::AccountId, item_id: ItemId, quantity: u32 },
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
        TooManyEffects,
        /// Item is of ConsumableCare category and should be used via feed_pet or play_with_pet.
        UseViaDedicatedExtrinsic,
        /// Item is Equipment and cannot be "used" directly in this way.
        CannotUseEquipmentDirectly,
        /// The item category provided during consumption check does not match the item's actual category.
        ItemCategoryMismatch,
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
            effects: BoundedVec<ItemEffect, T::MaxEffectsPerItem>, // Using BoundedVec
            max_stack: Option<u32>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(name.len() <= T::MaxItemNameLength::get() as usize, Error::<T>::NameTooLong);
            ensure!(description.len() <= T::MaxItemDescriptionLength::get() as usize, Error::<T>::DescriptionTooLong);
            // BoundedVec for effects handles TooManyEffects check implicitly at type level.

            let item_id = NextItemId::<T>::try_mutate(|id| -> Result<ItemId, DispatchError> {
                let current_id = *id;
                *id = id.checked_add(1).ok_or(Error::<T>::ItemIdOverflow)?;
                Ok(current_id)
            })?;

            let item_details = ItemDetails {
                name: name.clone(),
                description,
                category,
                effects: effects.into_inner(), // Convert BoundedVec to Vec for storage if ItemDetails stores Vec
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

            let user = ensure_signed(origin)?;

            let item_details = ItemDefinitions::<T>::get(item_id).ok_or(Error::<T>::ItemNotFound)?;
            let current_quantity = UserItemInventory::<T>::get((&user, item_id));
            ensure!(current_quantity > 0, Error::<T>::NotEnoughItemsInInventory);

            // Ensure this item is not a basic care item (handled by critter_nfts pallet's extrinsics)
            ensure!(item_details.category != ItemCategory::ConsumableCare, Error::<T>::UseViaDedicatedExtrinsic);
            // For MVP, also disallow direct "use" of Equipment if it implies equipping
            // If equipment has one-time application effects, it should be ConsumableBoost
            ensure!(item_details.category != ItemCategory::Equipment, Error::<T>::CannotUseEquipmentDirectly);


            let pet_owner = T::NftHandler::get_pet_owner(&target_pet_id)
               .ok_or(Error::<T>::CannotApplyItemToTarget)?;
            ensure!(pet_owner == user, Error::<T>::TargetPetNotOwned);

            // Consume item (most item categories here are consumed on use)
            UserItemInventory::<T>::insert((&user, item_id), current_quantity.saturating_sub(1));

            // Apply effects via T::NftHandler
            for effect in &item_details.effects {
                match effect {
                    ItemEffect::GrantFixedXp { amount } => {
                        T::NftHandler::grant_fixed_xp_to_pet(&user, &target_pet_id, *amount)
                            .map_err(|_| Error::<T>::ItemEffectApplicationFailed)?;
                    },
                    ItemEffect::ModifyMood { amount } => {
                        T::NftHandler::modify_mood_of_pet(&user, &target_pet_id, *amount)
                            .map_err(|_| Error::<T>::ItemEffectApplicationFailed)?;
                    },
                    ItemEffect::GrantPersonalityTrait { trait_to_grant } => {
                        // Ensure trait string length is within limits (can also be done in NftHandler impl)
                        ensure!(trait_to_grant.len() <= T::MaxTraitStringLen::get() as usize, Error::<T>::NameTooLong); // Reusing NameTooLong, consider specific error
                        T::NftHandler::grant_personality_trait_to_pet(&user, &target_pet_id, trait_to_grant.clone())
                            .map_err(|_| Error::<T>::ItemEffectApplicationFailed)?;
                    },
                    ItemEffect::ModifyBreedingRelatedValue { effect_type_id, value } => {
                        T::NftHandler::apply_breeding_assist_effect_to_pet(&user, &target_pet_id, *effect_type_id, *value)
                            .map_err(|_| Error::<T>::ItemEffectApplicationFailed)?;
                    },
                }
            }

            Self::deposit_event(Event::ItemUsedOnPet { user, item_id, pet_id: target_pet_id, effects_applied: item_details.effects.clone() });
            Ok(())
        }
    }

    // Implementation of the BasicCareItemConsumer trait
    // This allows pallet-critter-nfts to call into this pallet to consume items.
    impl<T: Config> BasicCareItemConsumer<T::AccountId, ItemId> for Pallet<T> {
        fn consume_specific_item(
            user: &T::AccountId,
            item_id: ItemId,
            expected_category: ItemCategory
        ) -> DispatchResult {
            let item_details = Self::item_definitions(item_id).ok_or(Error::<T>::ItemNotFound)?;
            ensure!(item_details.category == expected_category, Error::<T>::ItemCategoryMismatch);

            let current_quantity = Self::user_item_inventory((user, item_id));
            ensure!(current_quantity > 0, Error::<T>::NotEnoughItemsInInventory);

            UserItemInventory::<T>::insert((user, item_id), current_quantity.saturating_sub(1));
            // Event for item consumption could be added here if needed, or rely on critter_nfts events.
            Ok(())
        }
    }
}


// --- NftManagerForItems Trait Definition (Simplified for MVP) ---
// This trait defines how pallet-items interacts with pallet-critter-nfts for applying specific effects.
// pallet-critter-nfts would implement this.
// Removed LocalPetAttributeType and BlockNumberType from generics as effects are more direct or don't involve complex durations on-chain for MVP.
pub trait NftManagerForItems<AccountId, PetId, TraitTypeString, DispatchResultType> {
    fn get_pet_owner(pet_id: &PetId) -> Option<AccountId>; // Still useful for verification

    fn grant_fixed_xp_to_pet(
        caller: &AccountId,
        pet_id: &PetId,
        amount: u32
    ) -> DispatchResultType;

    fn modify_mood_of_pet(
        caller: &AccountId,
        pet_id: &PetId,
        amount: i16 // Can be positive or negative
    ) -> DispatchResultType;

    // apply_permanent_charter_boost_to_pet is deferred for MVP to avoid easy modification of base stats.

    fn grant_personality_trait_to_pet(
        caller: &AccountId,
        pet_id: &PetId,
        trait_to_grant: TraitTypeString, // e.g., Vec<u8>, BoundedVec handled by impl
    ) -> DispatchResultType;

    fn apply_breeding_assist_effect_to_pet( // Generic handler for breeding-related effects
        caller: &AccountId,
        pet_id: &PetId,
        effect_type_id: u8, // Pallet-critter-nfts impl will know how to interpret this
        value: u32
    ) -> DispatchResultType;
}
