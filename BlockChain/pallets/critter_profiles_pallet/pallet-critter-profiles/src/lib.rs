//! # Critter Profiles Pallet
//!
//! This pallet manages user profiles, achievements, and gameplay progression
//! for the CritterCraft ecosystem.
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
    use sp_runtime::traits::StaticLookup; // For AccountIdLookup

    // --- Type Aliases ---
    pub type AchievementId = u32; // Unique identifier for each achievement
    pub type BadgeId = u32; // Unique identifier for each badge
    pub type ProfileLevel = u32; // User profile level

    // --- Enum Definitions ---
    // UserStatus: Defines the current status of a user
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum UserStatus {
        Online,
        Offline,
        Busy,
        Away,
        Invisible,
    }

    // AchievementCategory: Defines the category of an achievement
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum AchievementCategory {
        Exploration,
        Collection,
        Social,
        Combat,
        Crafting,
        Special,
    }

    // --- Struct Definitions ---
    // UserProfile: Defines a user's profile
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct UserProfile<T: Config> {
        pub account_id: T::AccountId,
        pub username: BoundedVec<u8, T::MaxUsernameLen>,
        pub bio: BoundedVec<u8, T::MaxBioLen>,
        pub avatar_id: Option<u32>,
        pub status: UserStatus,
        pub level: ProfileLevel,
        pub experience: u64,
        pub reputation: i32,
        pub creation_block: BlockNumberFor<T>,
        pub last_active_block: BlockNumberFor<T>,
    }

    // Achievement: Defines an achievement that users can earn
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Achievement<T: Config> {
        pub id: AchievementId,
        pub name: BoundedVec<u8, T::MaxAchievementNameLen>,
        pub description: BoundedVec<u8, T::MaxAchievementDescLen>,
        pub category: AchievementCategory,
        pub experience_reward: u64,
        pub badge_reward: Option<BadgeId>,
        pub bits_reward: BalanceOf<T>,
        pub hidden: bool,
    }

    // UserAchievement: Tracks a user's earned achievement
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct UserAchievement<T: Config> {
        pub account_id: T::AccountId,
        pub achievement_id: AchievementId,
        pub earned_at_block: BlockNumberFor<T>,
    }

    // Badge: Defines a badge that users can display on their profile
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Badge<T: Config> {
        pub id: BadgeId,
        pub name: BoundedVec<u8, T::MaxBadgeNameLen>,
        pub description: BoundedVec<u8, T::MaxBadgeDescLen>,
        pub rarity: u8, // 1-5, with 5 being the rarest
    }

    // UserBadge: Tracks a user's earned badge
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct UserBadge<T: Config> {
        pub account_id: T::AccountId,
        pub badge_id: BadgeId,
        pub equipped: bool,
    }

    // FriendRequest: Tracks a friend request between users
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct FriendRequest<T: Config> {
        pub from: T::AccountId,
        pub to: T::AccountId,
        pub message: BoundedVec<u8, T::MaxFriendRequestMessageLen>,
        pub sent_at_block: BlockNumberFor<T>,
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
        
        /// Maximum length of a username (in bytes).
        #[pallet::constant]
        type MaxUsernameLen: Get<u32>;
        
        /// Maximum length of a bio (in bytes).
        #[pallet::constant]
        type MaxBioLen: Get<u32>;
        
        /// Maximum length of an achievement name (in bytes).
        #[pallet::constant]
        type MaxAchievementNameLen: Get<u32>;
        
        /// Maximum length of an achievement description (in bytes).
        #[pallet::constant]
        type MaxAchievementDescLen: Get<u32>;
        
        /// Maximum length of a badge name (in bytes).
        #[pallet::constant]
        type MaxBadgeNameLen: Get<u32>;
        
        /// Maximum length of a badge description (in bytes).
        #[pallet::constant]
        type MaxBadgeDescLen: Get<u32>;
        
        /// Maximum length of a friend request message (in bytes).
        #[pallet::constant]
        type MaxFriendRequestMessageLen: Get<u32>;
        
        /// Maximum number of friends a user can have.
        #[pallet::constant]
        type MaxFriends: Get<u32>;
        
        /// Maximum number of badges a user can equip at once.
        #[pallet::constant]
        type MaxEquippedBadges: Get<u32>;
        
        /// Experience required for each level.
        #[pallet::constant]
        type ExperiencePerLevel: Get<u64>;
    }

    // --- Pallet Definition ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // --- Pallet Storage Items ---
    #[pallet::storage]
    #[pallet::getter(fn user_profiles)]
    /// Stores the comprehensive UserProfile data for each AccountId.
    pub(super) type UserProfiles<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, UserProfile<T>>;

    #[pallet::storage]
    #[pallet::getter(fn username_lookup)]
    /// Maps a username to an AccountId.
    pub(super) type UsernameLookup<T: Config> = StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::MaxUsernameLen>, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn achievements)]
    /// Stores the comprehensive Achievement data for each AchievementId.
    pub(super) type Achievements<T: Config> = StorageMap<_, Blake2_128Concat, AchievementId, Achievement<T>>;

    #[pallet::storage]
    #[pallet::getter(fn next_achievement_id)]
    /// Stores the next available unique AchievementId.
    pub(super) type NextAchievementId<T: Config> = StorageValue<_, AchievementId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn user_achievements)]
    /// Stores the achievements earned by each user.
    pub(super) type UserAchievements<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, T::AccountId,
        Blake2_128Concat, AchievementId,
        UserAchievement<T>
    >;

    #[pallet::storage]
    #[pallet::getter(fn badges)]
    /// Stores the comprehensive Badge data for each BadgeId.
    pub(super) type Badges<T: Config> = StorageMap<_, Blake2_128Concat, BadgeId, Badge<T>>;

    #[pallet::storage]
    #[pallet::getter(fn next_badge_id)]
    /// Stores the next available unique BadgeId.
    pub(super) type NextBadgeId<T: Config> = StorageValue<_, BadgeId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn user_badges)]
    /// Stores the badges earned by each user.
    pub(super) type UserBadges<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, T::AccountId,
        Blake2_128Concat, BadgeId,
        UserBadge<T>
    >;

    #[pallet::storage]
    #[pallet::getter(fn equipped_badges)]
    /// Stores the badges currently equipped by each user.
    pub(super) type EquippedBadges<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<BadgeId, T::MaxEquippedBadges>,
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn friend_requests)]
    /// Stores pending friend requests.
    pub(super) type FriendRequests<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, T::AccountId, // From
        Blake2_128Concat, T::AccountId, // To
        FriendRequest<T>
    >;

    #[pallet::storage]
    #[pallet::getter(fn friends)]
    /// Stores the friends of each user.
    pub(super) type Friends<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<T::AccountId, T::MaxFriends>,
        ValueQuery
    >;

    // --- Pallet Events ---
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new user profile has been created. [account_id, username]
        ProfileCreated { account_id: T::AccountId, username: Vec<u8> },
        
        /// A user profile has been updated. [account_id]
        ProfileUpdated { account_id: T::AccountId },
        
        /// A user's status has been changed. [account_id, status]
        StatusChanged { account_id: T::AccountId, status: UserStatus },
        
        /// A user has earned an achievement. [account_id, achievement_id, name]
        AchievementEarned { account_id: T::AccountId, achievement_id: AchievementId, name: Vec<u8> },
        
        /// A user has earned a badge. [account_id, badge_id, name]
        BadgeEarned { account_id: T::AccountId, badge_id: BadgeId, name: Vec<u8> },
        
        /// A user has equipped a badge. [account_id, badge_id]
        BadgeEquipped { account_id: T::AccountId, badge_id: BadgeId },
        
        /// A user has unequipped a badge. [account_id, badge_id]
        BadgeUnequipped { account_id: T::AccountId, badge_id: BadgeId },
        
        /// A friend request has been sent. [from, to]
        FriendRequestSent { from: T::AccountId, to: T::AccountId },
        
        /// A friend request has been accepted. [from, to]
        FriendRequestAccepted { from: T::AccountId, to: T::AccountId },
        
        /// A friend request has been rejected. [from, to]
        FriendRequestRejected { from: T::AccountId, to: T::AccountId },
        
        /// A friend has been removed. [account_id, friend_id]
        FriendRemoved { account_id: T::AccountId, friend_id: T::AccountId },
        
        /// A user has gained experience. [account_id, amount]
        ExperienceGained { account_id: T::AccountId, amount: u64 },
        
        /// A user has leveled up. [account_id, new_level]
        LevelUp { account_id: T::AccountId, new_level: ProfileLevel },
        
        /// A user's reputation has changed. [account_id, change, new_total]
        ReputationChanged { account_id: T::AccountId, change: i32, new_total: i32 },
    }

    // --- Pallet Errors ---
    #[pallet::error]
    pub enum Error<T> {
        /// The profile already exists.
        ProfileAlreadyExists,
        
        /// The profile does not exist.
        ProfileDoesNotExist,
        
        /// The username is already taken.
        UsernameAlreadyTaken,
        
        /// The achievement does not exist.
        AchievementDoesNotExist,
        
        /// The user has already earned this achievement.
        AchievementAlreadyEarned,
        
        /// The badge does not exist.
        BadgeDoesNotExist,
        
        /// The user has already earned this badge.
        BadgeAlreadyEarned,
        
        /// The user has already equipped this badge.
        BadgeAlreadyEquipped,
        
        /// The user has not earned this badge.
        BadgeNotEarned,
        
        /// The user has not equipped this badge.
        BadgeNotEquipped,
        
        /// The user has reached the maximum number of equipped badges.
        MaxEquippedBadgesReached,
        
        /// A friend request already exists between these users.
        FriendRequestAlreadyExists,
        
        /// The friend request does not exist.
        FriendRequestDoesNotExist,
        
        /// The users are already friends.
        AlreadyFriends,
        
        /// The user has reached the maximum number of friends.
        MaxFriendsReached,
        
        /// The users are not friends.
        NotFriends,
        
        /// Cannot send a friend request to yourself.
        CannotFriendSelf,
        
        /// The next ID has overflowed.
        NextIdOverflow,
    }

    // --- Pallet Hooks ---
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            Weight::zero()
        }
    }

    // --- Pallet Extrinsics ---
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new user profile.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_profile(
            origin: OriginFor<T>,
            username: BoundedVec<u8, T::MaxUsernameLen>,
            bio: BoundedVec<u8, T::MaxBioLen>,
            avatar_id: Option<u32>,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;
            
            // 1. Check if the profile already exists.
            ensure!(!UserProfiles::<T>::contains_key(&account_id), Error::<T>::ProfileAlreadyExists);
            
            // 2. Check if the username is already taken.
            ensure!(!UsernameLookup::<T>::contains_key(&username), Error::<T>::UsernameAlreadyTaken);
            
            // 3. Create the profile.
            let current_block = frame_system::Pallet::<T>::block_number();
            let profile = UserProfile::<T> {
                account_id: account_id.clone(),
                username: username.clone(),
                bio,
                avatar_id,
                status: UserStatus::Online,
                level: 1,
                experience: 0,
                reputation: 0,
                creation_block: current_block,
                last_active_block: current_block,
            };
            
            // 4. Store the profile.
            UserProfiles::<T>::insert(&account_id, profile);
            
            // 5. Map the username to the account ID.
            UsernameLookup::<T>::insert(&username, account_id.clone());
            
            // 6. Emit the event.
            Self::deposit_event(Event::ProfileCreated {
                account_id,
                username: username.to_vec(),
            });
            
            Ok(())
        }

        /// Update an existing user profile.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn update_profile(
            origin: OriginFor<T>,
            bio: Option<BoundedVec<u8, T::MaxBioLen>>,
            avatar_id: Option<u32>,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;
            
            // 1. Check if the profile exists.
            let mut profile = UserProfiles::<T>::get(&account_id).ok_or(Error::<T>::ProfileDoesNotExist)?;
            
            // 2. Update the profile.
            if let Some(bio) = bio {
                profile.bio = bio;
            }
            
            if avatar_id.is_some() {
                profile.avatar_id = avatar_id;
            }
            
            // 3. Update the last active block.
            let current_block = frame_system::Pallet::<T>::block_number();
            profile.last_active_block = current_block;
            
            // 4. Store the updated profile.
            UserProfiles::<T>::insert(&account_id, profile);
            
            // 5. Emit the event.
            Self::deposit_event(Event::ProfileUpdated {
                account_id,
            });
            
            Ok(())
        }

        /// Change user status.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn set_status(
            origin: OriginFor<T>,
            status: UserStatus,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;
            
            // 1. Check if the profile exists.
            let mut profile = UserProfiles::<T>::get(&account_id).ok_or(Error::<T>::ProfileDoesNotExist)?;
            
            // 2. Update the status.
            profile.status = status;
            
            // 3. Update the last active block.
            let current_block = frame_system::Pallet::<T>::block_number();
            profile.last_active_block = current_block;
            
            // 4. Store the updated profile.
            UserProfiles::<T>::insert(&account_id, profile);
            
            // 5. Emit the event.
            Self::deposit_event(Event::StatusChanged {
                account_id,
                status,
            });
            
            Ok(())
        }

        /// Create a new achievement (admin only).
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_achievement(
            origin: OriginFor<T>,
            name: BoundedVec<u8, T::MaxAchievementNameLen>,
            description: BoundedVec<u8, T::MaxAchievementDescLen>,
            category: AchievementCategory,
            experience_reward: u64,
            badge_reward: Option<BadgeId>,
            bits_reward: BalanceOf<T>,
            hidden: bool,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            // 1. Get the next achievement ID.
            let achievement_id = Self::next_achievement_id();
            let next_id = achievement_id.checked_add(1).ok_or(Error::<T>::NextIdOverflow)?;
            NextAchievementId::<T>::put(next_id);
            
            // 2. Create the achievement.
            let achievement = Achievement::<T> {
                id: achievement_id,
                name: name.clone(),
                description,
                category,
                experience_reward,
                badge_reward,
                bits_reward,
                hidden,
            };
            
            // 3. Store the achievement.
            Achievements::<T>::insert(achievement_id, achievement);
            
            Ok(())
        }

        /// Award an achievement to a user (admin only).
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn award_achievement(
            origin: OriginFor<T>,
            to: <T::Lookup as StaticLookup>::Source,
            achievement_id: AchievementId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let account_id = T::Lookup::lookup(to)?;
            
            // 1. Check if the profile exists.
            ensure!(UserProfiles::<T>::contains_key(&account_id), Error::<T>::ProfileDoesNotExist);
            
            // 2. Check if the achievement exists.
            let achievement = Achievements::<T>::get(achievement_id).ok_or(Error::<T>::AchievementDoesNotExist)?;
            
            // 3. Check if the user has already earned this achievement.
            ensure!(!UserAchievements::<T>::contains_key(&account_id, achievement_id), Error::<T>::AchievementAlreadyEarned);
            
            // 4. Award the achievement.
            let current_block = frame_system::Pallet::<T>::block_number();
            let user_achievement = UserAchievement::<T> {
                account_id: account_id.clone(),
                achievement_id,
                earned_at_block: current_block,
            };
            
            // 5. Store the user achievement.
            UserAchievements::<T>::insert(&account_id, achievement_id, user_achievement);
            
            // 6. Award experience.
            if achievement.experience_reward > 0 {
                Self::add_experience(&account_id, achievement.experience_reward)?;
            }
            
            // 7. Award BITS.
            if achievement.bits_reward > BalanceOf::<T>::zero() {
                T::Currency::deposit_creating(&account_id, achievement.bits_reward);
            }
            
            // 8. Award badge if applicable.
            if let Some(badge_id) = achievement.badge_reward {
                if let Some(badge) = Badges::<T>::get(badge_id) {
                    if !UserBadges::<T>::contains_key(&account_id, badge_id) {
                        let user_badge = UserBadge::<T> {
                            account_id: account_id.clone(),
                            badge_id,
                            equipped: false,
                        };
                        
                        UserBadges::<T>::insert(&account_id, badge_id, user_badge);
                        
                        Self::deposit_event(Event::BadgeEarned {
                            account_id: account_id.clone(),
                            badge_id,
                            name: badge.name.to_vec(),
                        });
                    }
                }
            }
            
            // 9. Emit the event.
            Self::deposit_event(Event::AchievementEarned {
                account_id,
                achievement_id,
                name: achievement.name.to_vec(),
            });
            
            Ok(())
        }

        /// Create a new badge (admin only).
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_badge(
            origin: OriginFor<T>,
            name: BoundedVec<u8, T::MaxBadgeNameLen>,
            description: BoundedVec<u8, T::MaxBadgeDescLen>,
            rarity: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            // 1. Get the next badge ID.
            let badge_id = Self::next_badge_id();
            let next_id = badge_id.checked_add(1).ok_or(Error::<T>::NextIdOverflow)?;
            NextBadgeId::<T>::put(next_id);
            
            // 2. Create the badge.
            let badge = Badge::<T> {
                id: badge_id,
                name: name.clone(),
                description,
                rarity,
            };
            
            // 3. Store the badge.
            Badges::<T>::insert(badge_id, badge);
            
            Ok(())
        }

        /// Award a badge to a user (admin only).
        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn award_badge(
            origin: OriginFor<T>,
            to: <T::Lookup as StaticLookup>::Source,
            badge_id: BadgeId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let account_id = T::Lookup::lookup(to)?;
            
            // 1. Check if the profile exists.
            ensure!(UserProfiles::<T>::contains_key(&account_id), Error::<T>::ProfileDoesNotExist);
            
            // 2. Check if the badge exists.
            let badge = Badges::<T>::get(badge_id).ok_or(Error::<T>::BadgeDoesNotExist)?;
            
            // 3. Check if the user has already earned this badge.
            ensure!(!UserBadges::<T>::contains_key(&account_id, badge_id), Error::<T>::BadgeAlreadyEarned);
            
            // 4. Award the badge.
            let user_badge = UserBadge::<T> {
                account_id: account_id.clone(),
                badge_id,
                equipped: false,
            };
            
            // 5. Store the user badge.
            UserBadges::<T>::insert(&account_id, badge_id, user_badge);
            
            // 6. Emit the event.
            Self::deposit_event(Event::BadgeEarned {
                account_id,
                badge_id,
                name: badge.name.to_vec(),
            });
            
            Ok(())
        }

        /// Equip a badge.
        #[pallet::call_index(7)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn equip_badge(
            origin: OriginFor<T>,
            badge_id: BadgeId,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;
            
            // 1. Check if the profile exists.
            ensure!(UserProfiles::<T>::contains_key(&account_id), Error::<T>::ProfileDoesNotExist);
            
            // 2. Check if the user has earned this badge.
            let mut user_badge = UserBadges::<T>::get(&account_id, badge_id).ok_or(Error::<T>::BadgeNotEarned)?;
            
            // 3. Check if the badge is already equipped.
            ensure!(!user_badge.equipped, Error::<T>::BadgeAlreadyEquipped);
            
            // 4. Check if the user has reached the maximum number of equipped badges.
            let equipped_badges = EquippedBadges::<T>::get(&account_id);
            ensure!(equipped_badges.len() < T::MaxEquippedBadges::get() as usize, Error::<T>::MaxEquippedBadgesReached);
            
            // 5. Equip the badge.
            user_badge.equipped = true;
            UserBadges::<T>::insert(&account_id, badge_id, user_badge);
            
            // 6. Add the badge to the equipped badges.
            EquippedBadges::<T>::try_mutate(&account_id, |badges| -> DispatchResult {
                badges.try_push(badge_id).map_err(|_| Error::<T>::MaxEquippedBadgesReached)?;
                Ok(())
            })?;
            
            // 7. Emit the event.
            Self::deposit_event(Event::BadgeEquipped {
                account_id,
                badge_id,
            });
            
            Ok(())
        }

        /// Unequip a badge.
        #[pallet::call_index(8)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn unequip_badge(
            origin: OriginFor<T>,
            badge_id: BadgeId,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;
            
            // 1. Check if the profile exists.
            ensure!(UserProfiles::<T>::contains_key(&account_id), Error::<T>::ProfileDoesNotExist);
            
            // 2. Check if the user has earned this badge.
            let mut user_badge = UserBadges::<T>::get(&account_id, badge_id).ok_or(Error::<T>::BadgeNotEarned)?;
            
            // 3. Check if the badge is equipped.
            ensure!(user_badge.equipped, Error::<T>::BadgeNotEquipped);
            
            // 4. Unequip the badge.
            user_badge.equipped = false;
            UserBadges::<T>::insert(&account_id, badge_id, user_badge);
            
            // 5. Remove the badge from the equipped badges.
            EquippedBadges::<T>::mutate(&account_id, |badges| {
                if let Some(pos) = badges.iter().position(|&id| id == badge_id) {
                    badges.swap_remove(pos);
                }
            });
            
            // 6. Emit the event.
            Self::deposit_event(Event::BadgeUnequipped {
                account_id,
                badge_id,
            });
            
            Ok(())
        }

        /// Send a friend request.
        #[pallet::call_index(9)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn send_friend_request(
            origin: OriginFor<T>,
            to: <T::Lookup as StaticLookup>::Source,
            message: BoundedVec<u8, T::MaxFriendRequestMessageLen>,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            let to = T::Lookup::lookup(to)?;
            
            // 1. Check if the profiles exist.
            ensure!(UserProfiles::<T>::contains_key(&from), Error::<T>::ProfileDoesNotExist);
            ensure!(UserProfiles::<T>::contains_key(&to), Error::<T>::ProfileDoesNotExist);
            
            // 2. Check if the users are the same.
            ensure!(from != to, Error::<T>::CannotFriendSelf);
            
            // 3. Check if the users are already friends.
            let friends = Friends::<T>::get(&from);
            ensure!(!friends.contains(&to), Error::<T>::AlreadyFriends);
            
            // 4. Check if a friend request already exists.
            ensure!(!FriendRequests::<T>::contains_key(&from, &to), Error::<T>::FriendRequestAlreadyExists);
            ensure!(!FriendRequests::<T>::contains_key(&to, &from), Error::<T>::FriendRequestAlreadyExists);
            
            // 5. Create the friend request.
            let current_block = frame_system::Pallet::<T>::block_number();
            let request = FriendRequest::<T> {
                from: from.clone(),
                to: to.clone(),
                message,
                sent_at_block: current_block,
            };
            
            // 6. Store the friend request.
            FriendRequests::<T>::insert(&from, &to, request);
            
            // 7. Emit the event.
            Self::deposit_event(Event::FriendRequestSent {
                from,
                to,
            });
            
            Ok(())
        }

        /// Accept a friend request.
        #[pallet::call_index(10)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn accept_friend_request(
            origin: OriginFor<T>,
            from: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let to = ensure_signed(origin)?;
            let from = T::Lookup::lookup(from)?;
            
            // 1. Check if the friend request exists.
            ensure!(FriendRequests::<T>::contains_key(&from, &to), Error::<T>::FriendRequestDoesNotExist);
            
            // 2. Check if the users have reached the maximum number of friends.
            let mut from_friends = Friends::<T>::get(&from);
            let mut to_friends = Friends::<T>::get(&to);
            
            ensure!(from_friends.len() < T::MaxFriends::get() as usize, Error::<T>::MaxFriendsReached);
            ensure!(to_friends.len() < T::MaxFriends::get() as usize, Error::<T>::MaxFriendsReached);
            
            // 3. Add the users to each other's friend lists.
            from_friends.try_push(to.clone()).map_err(|_| Error::<T>::MaxFriendsReached)?;
            to_friends.try_push(from.clone()).map_err(|_| Error::<T>::MaxFriendsReached)?;
            
            Friends::<T>::insert(&from, from_friends);
            Friends::<T>::insert(&to, to_friends);
            
            // 4. Remove the friend request.
            FriendRequests::<T>::remove(&from, &to);
            
            // 5. Emit the event.
            Self::deposit_event(Event::FriendRequestAccepted {
                from,
                to,
            });
            
            Ok(())
        }

        /// Reject a friend request.
        #[pallet::call_index(11)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn reject_friend_request(
            origin: OriginFor<T>,
            from: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let to = ensure_signed(origin)?;
            let from = T::Lookup::lookup(from)?;
            
            // 1. Check if the friend request exists.
            ensure!(FriendRequests::<T>::contains_key(&from, &to), Error::<T>::FriendRequestDoesNotExist);
            
            // 2. Remove the friend request.
            FriendRequests::<T>::remove(&from, &to);
            
            // 3. Emit the event.
            Self::deposit_event(Event::FriendRequestRejected {
                from,
                to,
            });
            
            Ok(())
        }

        /// Remove a friend.
        #[pallet::call_index(12)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn remove_friend(
            origin: OriginFor<T>,
            friend: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;
            let friend_id = T::Lookup::lookup(friend)?;
            
            // 1. Check if the users are friends.
            let mut friends = Friends::<T>::get(&account_id);
            ensure!(friends.contains(&friend_id), Error::<T>::NotFriends);
            
            // 2. Remove the friend from the user's friend list.
            if let Some(pos) = friends.iter().position(|id| id == &friend_id) {
                friends.swap_remove(pos);
            }
            
            Friends::<T>::insert(&account_id, friends);
            
            // 3. Remove the user from the friend's friend list.
            Friends::<T>::mutate(&friend_id, |friends| {
                if let Some(pos) = friends.iter().position(|id| id == &account_id) {
                    friends.swap_remove(pos);
                }
            });
            
            // 4. Emit the event.
            Self::deposit_event(Event::FriendRemoved {
                account_id,
                friend_id,
            });
            
            Ok(())
        }

        /// Add experience to a user (admin only).
        #[pallet::call_index(13)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn add_experience_admin(
            origin: OriginFor<T>,
            to: <T::Lookup as StaticLookup>::Source,
            amount: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let account_id = T::Lookup::lookup(to)?;
            
            Self::add_experience(&account_id, amount)
        }

        /// Change a user's reputation (admin only).
        #[pallet::call_index(14)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn change_reputation(
            origin: OriginFor<T>,
            to: <T::Lookup as StaticLookup>::Source,
            change: i32,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let account_id = T::Lookup::lookup(to)?;
            
            // 1. Check if the profile exists.
            let mut profile = UserProfiles::<T>::get(&account_id).ok_or(Error::<T>::ProfileDoesNotExist)?;
            
            // 2. Update the reputation.
            profile.reputation = profile.reputation.saturating_add(change);
            
            // 3. Store the updated profile.
            UserProfiles::<T>::insert(&account_id, profile.clone());
            
            // 4. Emit the event.
            Self::deposit_event(Event::ReputationChanged {
                account_id,
                change,
                new_total: profile.reputation,
            });
            
            Ok(())
        }
    }

    // --- Pallet Internal Helper Functions ---
    impl<T: Config> Pallet<T> {
        /// Add experience to a user and handle level ups.
        fn add_experience(account_id: &T::AccountId, amount: u64) -> DispatchResult {
            // 1. Check if the profile exists.
            let mut profile = UserProfiles::<T>::get(account_id).ok_or(Error::<T>::ProfileDoesNotExist)?;
            
            // 2. Add the experience.
            let old_level = profile.level;
            profile.experience = profile.experience.saturating_add(amount);
            
            // 3. Check for level up.
            let exp_per_level = T::ExperiencePerLevel::get();
            let new_level = 1 + (profile.experience / exp_per_level) as u32;
            
            // 4. Update the level if needed.
            if new_level > old_level {
                profile.level = new_level;
                
                // Emit level up event.
                Self::deposit_event(Event::LevelUp {
                    account_id: account_id.clone(),
                    new_level,
                });
            }
            
            // 5. Store the updated profile.
            UserProfiles::<T>::insert(account_id, profile);
            
            // 6. Emit the experience gained event.
            Self::deposit_event(Event::ExperienceGained {
                account_id: account_id.clone(),
                amount,
            });
            
            Ok(())
        }
    }
}