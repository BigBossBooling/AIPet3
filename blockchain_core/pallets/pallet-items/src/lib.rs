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
    pub enum ItemEffect<BlockNumberType> {
        AttributeBoost {
            attribute: PetAttributeType,
            value: i16, // Delta: can be positive or negative
            is_percentage: bool, // If true, value is e.g. 10 for +10%. If false, it's an absolute delta.
            is_permanent: bool, // If true, applies to a base stat or permanently alters a dynamic one. If false, needs duration.
            duration_blocks: Option<BlockNumberType>, // Only if !is_permanent.
        },
        GrantPersonalityTrait {
            trait_to_grant: Vec<u8>, // The personality trait string
            // Future: chance_to_grant_percent: u8, // If not guaranteed
        },
        ModifyFertility { // Example specific effect for breeding
            fertility_points_change: i16, // Direct change to a conceptual "fertility" score on PetNft
            cooldown_reduction_blocks: Option<BlockNumberType>,
        },
        // Future: Heal { amount: u32, heal_type: HealType { HP, Energy } }
        // Future: RevivePet { chance_percent: u8 }
        // SYNERGY: Effect to grant temporary access to a special battle area or quest line
        // GrantAccess { feature_id: u32, duration: BlockNumberType }
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
        type NftHandler: super::NftManagerForItems<
            Self::AccountId,
            PetId,
            PetAttributeType,
            Vec<u8>, // TraitType (for personality_traits)
            BlockNumberFor<Self>, // BlockNumberType for durations
            DispatchResult // DispatchResultType
        >;
        // Note: NftManagerForItems is a NEW conceptual trait pallet_critter_nfts would need to implement.

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
            let user = ensure_signed(origin)?;

            let item_details = ItemDefinitions::<T>::get(item_id).ok_or(Error::<T>::ItemNotFound)?;
            let current_quantity = UserItemInventory::<T>::get((&user, item_id));
            ensure!(current_quantity > 0, Error::<T>::NotEnoughItemsInInventory);

            // SYNERGY: Check UserProfile score for eligibility to use rare items
            // // if item_details.rarity_level > RARE_THRESHOLD && T::UserProfileChecker::get_user_score(&user) < item_details.min_required_score {
            // //     return Err(Error::<T>::UserScoreTooLowForItem.into());
            // // }

            // Verify pet ownership via NftHandler
            let pet_owner = T::NftHandler::get_pet_owner(&target_pet_id)
               .ok_or(Error::<T>::CannotApplyItemToTarget)?; // Pet not found or NftHandler error
            ensure!(pet_owner == user, Error::<T>::TargetPetNotOwned);

            // Consume item (if consumable or stackable)
            // Equipment "usage" might be handled by an "equip" extrinsic if it's persistent.
            // If equipment provides one-time permanent boosts on "use", it's handled like a consumable here.
            if item_details.category == ItemCategory::Consumable ||
               (item_details.category != ItemCategory::Equipment && item_details.max_stack.is_some()) { // Non-equipment stackable
                UserItemInventory::<T>::insert((&user, item_id), current_quantity.saturating_sub(1));
            } else if item_details.category == ItemCategory::Equipment {
                // For true "Equipment" that is equipped/unequipped, this extrinsic might not be the right one.
                // Or, if "using" equipment means a one-time application of its permanent effects:
                // ensure!(current_quantity > 0, Error::<T>::NotEnoughItemsInInventory); // Should already be checked
                // UserItemInventory::<T>::insert((&user, item_id), current_quantity.saturating_sub(1)); // Consume it
                // This implies that "Equipment" items providing permanent boosts are consumed on use.
                // If they are meant to be equipped and provide passive stats, a different system is needed.
                // For now, let's assume this extrinsic is for items that apply effects and are consumed or quantity reduced.
                // If it's a non-stackable, non-consumable that's not equipment (e.g. a reusable key item for a quest),
                // its quantity might not change. This logic depends on item design.
                // For now, we'll assume non-equipment items are consumed if stackable, or one-time use if not stackable.
                // The current logic for equipment is to disallow direct "use" if it means "equip".
                ensure!(item_details.category != ItemCategory::Equipment, Error::<T>::CannotUseEquipmentDirectly);
            }


            // Apply effects via T::NftHandler
            for effect in &item_details.effects {
                match effect {
                    ItemEffect::AttributeBoost { attribute, value, is_percentage, is_permanent, duration_blocks } => {
                        T::NftHandler::apply_attribute_boost_to_pet(
                            &user, &target_pet_id, *attribute, *value, *is_percentage, *is_permanent, *duration_blocks
                        ).map_err(|_| Error::<T>::ItemEffectApplicationFailed)?;
                    },
                    ItemEffect::GrantPersonalityTrait { trait_to_grant } => {
                        T::NftHandler::grant_personality_trait_to_pet(
                            &user, &target_pet_id, trait_to_grant.clone()
                        ).map_err(|_| Error::<T>::ItemEffectApplicationFailed)?;
                    },
                    ItemEffect::ModifyFertility { fertility_points_change, cooldown_reduction_blocks } => {
                        if *fertility_points_change != 0 {
                             T::NftHandler::modify_pet_fertility_value(&user, &target_pet_id, *fertility_points_change)
                                 .map_err(|_| Error::<T>::ItemEffectApplicationFailed)?;
                        }
                        if let Some(reduction) = cooldown_reduction_blocks {
                             T::NftHandler::reduce_pet_breeding_cooldown(&user, &target_pet_id, *reduction)
                                 .map_err(|_| Error::<T>::ItemEffectApplicationFailed)?;
                        }
                    },
                    // Handle other future effects like GrantAccess, Heal, RevivePet
                }
            }

            Self::deposit_event(Event::ItemUsedOnPet { user, item_id, pet_id: target_pet_id, effects_applied: item_details.effects.clone() });

            Ok(())
        }
    }
}


// --- NftManagerForItems Trait Definition ---
// This trait defines how pallet-items interacts with pallet-critter-nfts.
// pallet-critter-nfts would then implement this.
pub trait NftManagerForItems<AccountId, PetId, LocalPetAttributeType, TraitTypeString, BlockNumberType, DispatchResultType> {
    fn get_pet_owner(pet_id: &PetId) -> Option<AccountId>;

    fn apply_attribute_boost_to_pet(
        caller: &AccountId, // The user applying the item
        pet_id: &PetId,
        attribute: LocalPetAttributeType, // This is pallet_items::PetAttributeType
        value: i16,
        is_percentage: bool,
        is_permanent: bool,
        duration_blocks: Option<BlockNumberType>,
    ) -> DispatchResultType;

    fn grant_personality_trait_to_pet(
        caller: &AccountId,
        pet_id: &PetId,
        trait_to_grant: TraitTypeString, // TraitTypeString is Vec<u8>
    ) -> DispatchResultType;

    fn modify_pet_fertility_value( // Example for a specific fertility stat
        caller: &AccountId,
        pet_id: &PetId,
        fertility_points_change: i16,
    ) -> DispatchResultType;

    fn reduce_pet_breeding_cooldown( // Example
        caller: &AccountId,
        pet_id: &PetId,
        reduction_blocks: BlockNumberType,
    ) -> DispatchResultType;

    // Future: fn apply_cosmetic_to_pet(caller: &AccountId, pet_id: &PetId, cosmetic_id: u32) -> DispatchResultType;
    // Future: fn equip_item_to_pet_slot(caller: &AccountId, pet_id: &PetId, item_id: ItemId, slot_type: u8) -> DispatchResultType;
    // Future: fn unequip_item_from_pet_slot(caller: &AccountId, pet_id: &PetId, slot_type: u8) -> DispatchResultType;
}
