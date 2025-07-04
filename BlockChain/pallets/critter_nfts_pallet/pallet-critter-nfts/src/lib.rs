//! # Pet NFT Pallet
//!
//! This pallet manages the core Non-Fungible Tokens (NFTs) representing CritterCraft Pets.
//! It defines the immutable "charter attributes" (nature) and dynamic "development attributes" (nurture)
//! of each Pet NFT. It also handles minting, ownership, basic transfers, and
//! essential interactions that drive a pet's lifecycle.
//!
//! Meticulously crafted to align with The Architect's vision for
//! modularity, scalability, and robust operation of the CritterCraft digital ecosystem.

#![cfg_attr(not(feature = "std"), no_std)] // No standard library for Wasm compilation

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// Include the synchronization module
pub mod sync;

// Include the advanced state management module
pub mod state;

// Include the personality evolution module
pub mod personality;

// Include the social interactions module
pub mod social;

// Include the environmental adaptation module
pub mod environment;

// Include the pet training module
pub mod training;

// Include the pet memory module
pub mod memory;

// Include the mood contagion module
pub mod mood;

// Include the achievements module
pub mod achievements;

// Include the seasonal events module
pub mod seasonal;

// Include the lifecycle events module
pub mod lifecycle;

// Include the analytics module
pub mod analytics;

// Include the UI bridge module
pub mod ui_bridge;

// Include the visual representation module
pub mod visual;

// Include the interactive elements module
pub mod interactive;

// Include the updated interactive elements module with enhanced security and optimizations
pub mod interactive_updated;

// Include the user experience module
pub mod user_experience;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*, // Provides common types and macros for pallets
        traits::{Currency, Randomness, UnixTime}, // Currency for balances, Randomness for DNA, UnixTime for timestamps (if used)
        BoundedVec, // For bounded collections, crucial for security
    };
    use frame_system::{
        pallet_prelude::*, // Provides types like BlockNumberFor, AccountId, OriginFor
        ensure_signed,     // Macro to ensure origin is a signed account
    };
    // Use shared traits and types defined in the `crittercraft-traits` crate
    use crate::traits::{
        NftManager as SharedNftManager, // For basic NFT operations (lock, unlock, transfer)
        NftBreedingHandler, // For integration with breeding logic
        NftManagerForItems, // For integration with item usage
        QuestNftRequirementChecker, // For integration with quest requirements
        BasicCareItemConsumer, // This trait defines the interface pallet-critter-nfts expects from pallet-items
        SimpleGeneticInfo, // Data structure for breeding genetics
        PetId as SharedPetId, // Using SharedPetId to distinguish from local PetId if needed (typically u32)
        ItemId as SharedItemId, // Using ItemId from `pallet-items`
        DnaHashType,        // Type for DNA hash (e.g., [u8; 32])
        SpeciesType,        // Type for Species (e.g., BoundedVec<u8, MaxSpeciesNameLen>)
        TraitTypeString,    // Type for Personality Trait strings (e.g., BoundedVec<u8, MaxTraitStringLen>)
        ItemCategoryTag,    // For `BasicCareItemConsumer` (e.g., u8 for enum variant index)
    };
    use sp_std::vec::Vec; // Standard Vec for dynamic arrays (used where not bounded)
    use scale_info::TypeInfo; // For `TypeInfo` derive macro
    use frame_support::log; // Correct way to import Substrate's logging macro
    use sp_runtime::SaturatedFrom; // For saturating arithmetic

    // --- Type Aliases ---
    // These aliases enhance clarity, aligning with "Know Your Core, Keep it Clear".
    pub type PetId = u32; // Unique identifier for each Pet NFT
    // ItemId is provided by SharedItemId from traits, no need to re-alias here if not used.

    // --- Enum Definitions ---
    // ElementType: Defines the elemental affinities of Critters.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default, Copy)]
    pub enum ElementType {
        #[default]
        Neutral,
        Fire,
        Water,
        Earth,
        Air,
        Tech,
        Nature,
        Mystic,
    }

    // --- Struct Definitions ---
    // PetNft: Defines the core attributes and state of a CritterCraft Pet NFT.
    // #[scale_info(skip_type_params(T))] is important when T is only used in BoundedVec/Storage, not directly in struct.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))] // T is used in BoundedVecs, so skip for TypeInfo on struct
    pub struct PetNft<T: Config> {
        // --- Immutable Attributes (Charter Attributes) ---
        // Set at minting and never change, defining the pet's inherent "nature".
        pub id: PetId,
        pub dna_hash: DnaHashType, // Using DnaHashType from `crittercraft-traits`
        pub initial_species: SpeciesType, // Using SpeciesType from `crittercraft-traits`
        pub current_pet_name: BoundedVec<u8, T::MaxPetNameLen>, // Using BoundedVec for security

        // Deterministically derived from dna_hash at minting.
        pub base_strength: u8,
        pub base_agility: u8,
        pub base_intelligence: u8,
        pub base_vitality: u8,
        pub primary_elemental_affinity: ElementType, // Changed to non-Option, Default is Neutral

        // --- Dynamic Attributes ---
        // These attributes change over time based on interactions and gameplay.
        pub level: u32,
        pub experience_points: u32,

        // Simplified mood. Hunger & Energy are primarily inferred off-chain from timestamps.
        pub mood_indicator: u8, // e.g., 0-Unhappy, 50-Neutral, up to T::MaxMoodValue. Updated by direct actions.

        // On-chain timestamps for off-chain state calculation and neglect checks.
        pub last_fed_block: BlockNumberFor<T>,
        pub last_played_block: BlockNumberFor<T>, // Represents general care/interaction timestamp

        // Personality traits, dynamically updated (potentially by owner-approved AI suggestions).
        pub personality_traits: BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits>, // Using TraitTypeString from `crittercraft-traits`

        pub last_state_update_block: BlockNumberFor<T>, // Block of last significant on-chain state change or interaction
        
        // --- State Versioning ---
        // Used to track changes to pet state for synchronization and concurrency control
        pub state_version: u32, // Incremented on every state change
        
        // --- Synchronization Flags ---
        // Used to track which aspects of the pet state have been synchronized with off-chain systems
        pub sync_flags: u8, // Bitfield for tracking sync status (0x01 = basic info, 0x02 = stats, 0x04 = traits, etc.)
        
        // --- Locking Status ---
        // Used to prevent interactions with the pet during certain operations
        pub is_locked: bool, // True if the pet is locked and cannot be interacted with
        
        // --- Last Interaction Time ---
        // Used for rate limiting and tracking recent activity
        pub last_interaction_time: BlockNumberFor<T>, // Block number of the last interaction
        
        // V2+: Parent IDs for breeding traceability
        // pub parent1_id: Option<PetId>,
        // pub parent2_id: Option<PetId>,
    }

    // BalanceOf<T> type alias for the pallet's currency type.
    pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // --- Pallet Configuration Trait ---
    // Defines the types and constants that the runtime must provide for this pallet to function.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// The currency trait for handling PTCN token balances.
        type Currency: Currency<Self::AccountId>;

        /// The randomness trait for generating deterministic DNA hashes.
        type PetRandomness: Randomness<Self::Hash, Self::BlockNumber>;
        /// Maximum number of Pet NFTs an account can own. Crucial for limiting state bloat.
        #[pallet::constant]
        type MaxOwnedPets: Get<u32>;
        /// Maximum length of a pet's species name (in bytes). Crucial for input validation.
        #[pallet::constant]
        type MaxSpeciesNameLen: Get<u32>;
        /// Maximum length of a pet's current name (in bytes). Crucial for input validation.
        #[pallet::constant]
        type MaxPetNameLen: Get<u32>;
        /// Maximum length of a single personality trait string (in bytes).
        #[pallet::constant]
        type MaxTraitStringLen: Get<u32>;
        /// Maximum number of personality traits a pet can have.
        #[pallet::constant]
        type MaxPetPersonalityTraits: Get<u32>;
        /// Maximum value for `mood_indicator` (e.g., 100 or 200).
        #[pallet::constant]
        type MaxMoodValue: Get<u8>;
        /// Amount of mood restored from basic feeding.
        #[pallet::constant]
        type FeedMoodBoost: Get<u8>;
        /// Amount of mood restored from basic playing.
        #[pallet::constant]
        type PlayMoodBoost: Get<u8>;
        /// XP gained from basic feeding.
        #[pallet::constant]
        type FeedXpGain: Get<u32>;
        /// XP gained from basic playing.
        #[pallet::constant]
        type PlayXpGain: Get<u32>;
        /// Mood penalty applied due to neglect.
        #[pallet::constant]
        type NeglectMoodPenalty: Get<u8>;
        /// Number of blocks after which neglect effects might apply.
        #[pallet::constant]
        type NeglectThresholdBlocks: Get<Self::BlockNumber>;
        /// Amount of PTCN claimed daily by users.
        #[pallet::constant]
        type DailyClaimAmount: Get<BalanceOf<Self>>;
        /// Cooldown period (in blocks) for daily PTCN claims.
        #[pallet::constant]
        type ClaimCooldownPeriod: Get<Self::BlockNumber>;

        /// Maximum number of execution statistics entries to store per hook.
        #[pallet::constant]
        type MaxHookExecutionStats: Get<u32>;
        
        /// Maximum number of synchronization status entries to store per pet.
        #[pallet::constant]
        type MaxSyncStatusEntries: Get<u32>;
        
        /// Maximum number of hooks that can be registered.
        #[pallet::constant]
        type MaxRegisteredHooks: Get<u32>;
        
        /// Maximum execution time for a hook in milliseconds.
        #[pallet::constant]
        type MaxHookExecutionTimeMs: Get<u32>;
        
        /// Maximum size of a pet's compressed interaction history.
        #[pallet::constant]
        type MaxInteractionHistorySize: Get<u32>;
        
        /// Maximum number of behavior predictions to store for a pet.
        #[pallet::constant]
        type MaxBehaviorPredictions: Get<u32>;
        
        /// Maximum number of state transition probabilities to store for a pet.
        #[pallet::constant]
        type MaxTransitionProbabilities: Get<u32>;
        
        /// Threshold for adaptive behavior adjustments.
        #[pallet::constant]
        type AdaptiveBehaviorThreshold: Get<u32>;
        
        /// Maximum number of memories a pet can have.
        #[pallet::constant]
        type MaxPetMemories: Get<u32>;
        
        /// Maximum significance of a memory.
        #[pallet::constant]
        type MaxMemorySignificance: Get<u8>;
        
        /// Maximum number of skills a pet can have.
        #[pallet::constant]
        type MaxPetSkills: Get<u32>;
        
        /// Maximum level of a skill.
        #[pallet::constant]
        type MaxSkillLevel: Get<u8>;
        
        /// Maximum number of achievements a pet can earn.
        #[pallet::constant]
        type MaxPetAchievements: Get<u32>;
        
        /// Maximum number of social interactions a pet can have per block.
        #[pallet::constant]
        type MaxSocialInteractionsPerBlock: Get<u32>;
        
        /// Maximum mood change from a social interaction.
        #[pallet::constant]
        type MaxMoodChangeFromSocialInteraction: Get<u8>;
        
        /// Maximum number of environments a pet can adapt to.
        #[pallet::constant]
        type MaxEnvironmentalAdaptations: Get<u32>;
        
        /// Maximum adaptation level to an environment.
        #[pallet::constant]
        type MaxAdaptationLevel: Get<u8>;
        
        /// Maximum number of seasonal events that can be active at once.
        #[pallet::constant]
        type MaxActiveSeasonalEvents: Get<u32>;
        
        /// Maximum effect magnitude of a seasonal event.
        #[pallet::constant]
        type MaxSeasonalEventEffectMagnitude: Get<u8>;
        
        /// Maximum number of lifecycle events a pet can experience.
        #[pallet::constant]
        type MaxLifecycleEvents: Get<u32>;
        
        /// Maximum size of an analytics report.
        #[pallet::constant]
        type MaxAnalyticsReportSize: Get<u32>;
        
        /// Maximum number of visual attributes a pet can have.
        #[pallet::constant]
        type MaxVisualAttributes: Get<u32>;
        
        /// Maximum number of notifications a user can have.
        #[pallet::constant]
        type MaxNotifications: Get<u32>;
        
        /// Maximum number of achievements a user can have.
        #[pallet::constant]
        type MaxAchievements: Get<u32>;
        
        /// Handler for consuming basic care items (Food, Toys).
        /// This trait is from `crate::traits` and MUST be implemented by `pallet-items`.
        /// It dictates what `pallet-items` must provide for basic care item consumption logic
        /// called by `pallet-critter-nfts`.
        type ItemHandler: BasicCareItemConsumer<Self::AccountId, SharedItemId, ItemCategoryTag, DispatchResult> + 
                        // Add associated types for category tags if they are part of the trait.
                        // Example: `food_category_tag() -> ItemCategoryTag;`
                        // `toy_category_tag() -> ItemCategoryTag;`
                        // Or if they are constants defined within the trait itself.
                        // For now, assuming direct functions are available on the trait.
                        frame_support::traits::Get<ItemCategoryTag>; // Assuming ItemHandler can provide constants or associated types for tags.
    }

    // --- Pallet Definition ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)] // Generates getter functions for storage items
    pub struct Pallet<T>(_);

    // --- Pallet Storage Items ---
    
    /// Storage for pet social interactions.
    #[pallet::storage]
    #[pallet::getter(fn pet_social_interactions)]
    pub type PetSocialInteractions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(PetId, u8, u8, BlockNumberFor<T>), T::MaxSocialInteractionsPerBlock>,
        ValueQuery,
    >;
    
    /// Storage for pet social bonds.
    #[pallet::storage]
    #[pallet::getter(fn pet_social_bonds)]
    pub type PetSocialBonds<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<social::SocialBond, T::MaxSocialInteractionsPerBlock>,
        ValueQuery,
    >;
    
    /// Storage for pet environmental adaptations.
    #[pallet::storage]
    #[pallet::getter(fn pet_environmental_adaptations)]
    pub type PetEnvironmentalAdaptations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, u8), T::MaxEnvironmentalAdaptations>,
        ValueQuery,
    >;
    
    /// Storage for pet skills.
    #[pallet::storage]
    #[pallet::getter(fn pet_skills)]
    pub type PetSkills<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, u8), T::MaxPetSkills>,
        ValueQuery,
    >;
    
    /// Storage for pet memories.
    #[pallet::storage]
    #[pallet::getter(fn pet_memories)]
    pub type PetMemories<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, u8, u64, Vec<u8>), T::MaxPetMemories>,
        ValueQuery,
    >;
    
    /// Storage for pet achievements.
    #[pallet::storage]
    #[pallet::getter(fn pet_achievements)]
    pub type PetAchievements<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u32, u64), T::MaxPetAchievements>,
        ValueQuery,
    >;
    
    /// Storage for active seasonal events.
    #[pallet::storage]
    #[pallet::getter(fn active_seasonal_events)]
    pub type ActiveSeasonalEvents<T: Config> = StorageValue<
        _,
        BoundedVec<(u32, BlockNumberFor<T>, BlockNumberFor<T>), T::MaxActiveSeasonalEvents>,
        ValueQuery,
    >;
    
    /// Storage for pet lifecycle events.
    #[pallet::storage]
    #[pallet::getter(fn pet_lifecycle_events)]
    pub type PetLifecycleEvents<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, BlockNumberFor<T>), T::MaxLifecycleEvents>,
        ValueQuery,
    >;
    
    /// Storage for pet analytics reports.
    #[pallet::storage]
    #[pallet::getter(fn pet_analytics_reports)]
    pub type PetAnalyticsReports<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<u8, T::MaxAnalyticsReportSize>,
        ValueQuery,
    >;
    
    /// Storage for pet visual attributes.
    #[pallet::storage]
    #[pallet::getter(fn pet_visual_attributes)]
    pub type PetVisualAttributes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<visual::VisualAttribute, T::MaxVisualAttributes>,
        ValueQuery,
    >;
    
    /// Storage for pet visual themes.
    #[pallet::storage]
    #[pallet::getter(fn pet_visual_theme)]
    pub type PetVisualTheme<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        u8,
        OptionQuery,
    >;
    
    /// Storage for user UX flows.
    #[pallet::storage]
    #[pallet::getter(fn user_ux_flow)]
    pub type UserUxFlow<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        (u16, u32), // (flow_id, step_id)
        ValueQuery,
    >;
    
    /// Storage for user notifications.
    #[pallet::storage]
    #[pallet::getter(fn user_notifications)]
    pub type UserNotifications<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<user_experience::UserNotification, T::MaxNotifications>,
        ValueQuery,
    >;
    
    /// Storage for user achievements.
    #[pallet::storage]
    #[pallet::getter(fn user_achievements)]
    pub type UserAchievements<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<user_experience::UserAchievement, T::MaxAchievements>,
        ValueQuery,
    >;
    
    /// Storage for the next notification ID.
    #[pallet::storage]
    #[pallet::getter(fn next_notification_id)]
    pub type NextNotificationId<T: Config> = StorageValue<_, u32, ValueQuery>;
    // These store the actual state of the CritterChain.
    #[pallet::storage]
    #[pallet::getter(fn next_pet_id)]
    /// Stores the next available unique PetId for new NFTs.
    pub(super) type NextPetId<T: Config> = StorageValue<_, PetId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_nfts)]
    /// Stores the comprehensive PetNft data for each PetId.
    pub(super) type PetNfts<T: Config> = StorageMap<_, Blake2_128Concat, PetId, PetNft<T>>;

    #[pallet::storage]
    #[pallet::getter(fn owner_of_pet)]
    /// Stores a list of PetIds owned by each AccountId.
    /// Uses `BoundedVec` for security against unbounded growth.
    pub(super) type OwnerOfPet<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<PetId, T::MaxOwnedPets>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_nft_owner)]
    /// Maps a PetId directly to its owner AccountId for quick lookups.
    pub(super) type PetNftOwner<T: Config> = StorageMap<_, Blake2_128Concat, PetId, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn locked_nfts)]
    /// Stores PetIds of NFTs that are currently locked (e.g., listed on marketplace, in battle).
    /// Using `ValueQuery` with `()` means we only care about the key's presence, not an associated value.
    pub(super) type LockedNfts<T: Config> = StorageMap<_, Blake2_128Concat, PetId, (), ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_claim_time)]
    /// Stores the block number of the last successful PTCN claim for each account.
    pub(super) type LastClaimTime<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::BlockNumber, ValueQuery>;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_state_versions)]
    /// Stores the current state version for each pet.
    /// This is used for optimistic concurrency control and synchronization.
    pub(super) type PetStateVersions<T: Config> = StorageMap<_, Blake2_128Concat, PetId, u32, ValueQuery>;
    
    #[pallet::storage]
    #[pallet::getter(fn sync_hook_registry)]
    /// Stores detailed information about registered synchronization hooks.
    /// Each hook is identified by a unique ID and contains detailed information.
    pub(super) type SyncHookRegistry<T: Config> = StorageMap<_, Blake2_128Concat, u32, sync::HookInfo<T>, OptionQuery>;
    
    #[pallet::storage]
    #[pallet::getter(fn sync_hook_execution_stats)]
    /// Stores execution statistics for synchronization hooks.
    /// This helps track hook performance and reliability.
    pub(super) type SyncHookExecutionStats<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // hook_id
        BoundedVec<(T::BlockNumber, bool, u32), T::MaxHookExecutionStats>, // (timestamp, success, execution_time_ms)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_sync_status)]
    /// Stores the synchronization status for each pet.
    /// This helps track which aspects of a pet's state have been synchronized.
    pub(super) type PetSyncStatus<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, u32, T::BlockNumber), T::MaxSyncStatusEntries>, // (change_type, version, timestamp)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_interaction_history)]
    /// Stores a compressed history of interactions for each pet.
    /// This is used for advanced state management and predictive analytics.
    pub(super) type PetInteractionHistory<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<u8, T::MaxInteractionHistorySize>, // Compressed interaction history
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_behavior_predictions)]
    /// Stores behavior predictions for each pet.
    /// This is used for adaptive behavior and predictive analytics.
    pub(super) type PetBehaviorPredictions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, u8), T::MaxBehaviorPredictions>, // (behavior_type, probability)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_transition_probabilities)]
    /// Stores state transition probabilities for each pet.
    /// This is used for adaptive behavior and predictive analytics.
    pub(super) type PetTransitionProbabilities<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, u8, u8), T::MaxTransitionProbabilities>, // (from_state, to_state, probability)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_memories)]
    /// Stores memories for each pet.
    /// This is used for the pet memory system.
    pub(super) type PetMemories<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, u8, u64, Vec<u8>), T::MaxPetMemories>, // (memory_type, significance, timestamp, associated_data)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_skills)]
    /// Stores skills for each pet.
    /// This is used for the pet training system.
    pub(super) type PetSkills<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, u8), T::MaxPetSkills>, // (skill_type, skill_level)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_achievements)]
    /// Stores achievements for each pet.
    /// This is used for the pet achievements system.
    pub(super) type PetAchievements<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u32, u64), T::MaxPetAchievements>, // (achievement_id, timestamp)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_social_interactions)]
    /// Stores recent social interactions for each pet.
    /// This is used for the cross-pet social interactions system.
    pub(super) type PetSocialInteractions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(PetId, u8, u8, T::BlockNumber), T::MaxSocialInteractionsPerBlock>, // (other_pet_id, interaction_type, outcome, timestamp)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_environmental_adaptations)]
    /// Stores environmental adaptations for each pet.
    /// This is used for the environmental adaptation system.
    pub(super) type PetEnvironmentalAdaptations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, u8), T::MaxEnvironmentalAdaptations>, // (environment_type, adaptation_level)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn active_seasonal_events)]
    /// Stores currently active seasonal events.
    /// This is used for the seasonal events system.
    pub(super) type ActiveSeasonalEvents<T: Config> = StorageValue<
        _,
        BoundedVec<(u32, T::BlockNumber, T::BlockNumber), T::MaxActiveSeasonalEvents>, // (event_id, start_time, end_time)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_lifecycle_events)]
    /// Stores lifecycle events for each pet.
    /// This is used for the lifecycle events system.
    pub(super) type PetLifecycleEvents<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<(u8, T::BlockNumber), T::MaxLifecycleEvents>, // (event_type, timestamp)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn pet_analytics_reports)]
    /// Stores analytics reports for each pet.
    /// This is used for the advanced analytics dashboard.
    pub(super) type PetAnalyticsReports<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PetId,
        BoundedVec<u8, T::MaxAnalyticsReportSize>, // Compressed analytics report
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn interactive_sessions)]
    /// Stores interactive sessions for each pet.
    /// This is used for the interactive elements system.
    pub(super) type InteractiveSessions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // session_id
        interactive::InteractiveSession,
        OptionQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn user_sessions)]
    /// Stores session IDs for each user.
    /// This is used for tracking and rate limiting sessions.
    pub(super) type UserSessions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Vec<(u32, T::BlockNumber)>, // (session_id, start_block)
        ValueQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn nonce)]
    /// A nonce for generating unique session IDs.
    pub(super) type Nonce<T: Config> = StorageValue<
        _,
        u32,
        ValueQuery
    >;


    // --- Pallet Events ---
    // Events provide transparent, auditable logs of state changes for off-chain services and UIs.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new Pet NFT has been minted with detailed information.
        /// [owner, pet_id, species, dna_hash, base_stats, timestamp]
        PetNftMinted { 
            owner: T::AccountId, 
            pet_id: PetId,
            species: SpeciesType,
            dna_hash: DnaHashType,
            base_strength: u8,
            base_agility: u8,
            base_intelligence: u8,
            base_vitality: u8,
            elemental_affinity: ElementType,
            timestamp: BlockNumberFor<T>
        },
        
        /// A Pet NFT has been transferred with detailed information.
        /// [from, to, pet_id, timestamp]
        PetNftTransferred { 
            from: T::AccountId, 
            to: T::AccountId, 
            pet_id: PetId,
            timestamp: BlockNumberFor<T>
        },
        
        /// A Pet NFT's metadata has been updated with detailed information.
        /// [owner, pet_id, new_name, new_traits, timestamp]
        PetNftMetadataUpdated { 
            owner: T::AccountId, 
            pet_id: PetId,
            new_name: Option<BoundedVec<u8, T::MaxPetNameLen>>,
            new_traits: Option<BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits>>,
            timestamp: BlockNumberFor<T>
        },
        
        /// A user has successfully claimed their daily PTCN.
        /// [account, amount, claim_time]
        DailyClaimMade { 
            account: T::AccountId, 
            amount: BalanceOf<T>, 
            claim_time: T::BlockNumber 
        },
        
        /// A pet was fed with detailed information.
        /// [owner, pet_id, food_item_id, mood_boost, xp_gain, timestamp]
        PetFed { 
            owner: T::AccountId, 
            pet_id: PetId, 
            food_item_id: SharedItemId,
            mood_boost: u8,
            xp_gain: u32,
            new_mood: u8,
            timestamp: BlockNumberFor<T>
        },
        
        /// A pet was played with detailed information.
        /// [owner, pet_id, toy_item_id, mood_boost, xp_gain, timestamp]
        PetPlayedWith { 
            owner: T::AccountId, 
            pet_id: PetId, 
            toy_item_id: SharedItemId,
            mood_boost: u8,
            xp_gain: u32,
            new_mood: u8,
            timestamp: BlockNumberFor<T>
        },
        
        /// A pet leveled up with detailed information.
        /// [pet_id, old_level, new_level, experience_points, timestamp]
        PetLeveledUp { 
            pet_id: PetId, 
            old_level: u32,
            new_level: u32,
            experience_points: u32,
            timestamp: BlockNumberFor<T>
        },
        
        /// A pet's mood changed due to neglect with detailed information.
        /// [pet_id, old_mood, new_mood, neglect_duration, timestamp]
        PetNeglected { 
            pet_id: PetId, 
            old_mood: u8,
            new_mood: u8,
            neglect_duration: BlockNumberFor<T>,
            timestamp: BlockNumberFor<T>
        },
        
        /// An NFT has been locked with detailed information.
        /// [owner, pet_id, timestamp]
        NftLocked { 
            owner: T::AccountId, 
            pet_id: PetId,
            timestamp: BlockNumberFor<T>
        },
        
        /// An NFT has been unlocked with detailed information.
        /// [owner, pet_id, timestamp]
        NftUnlocked { 
            owner: T::AccountId, 
            pet_id: PetId,
            timestamp: BlockNumberFor<T>
        },
        
        /// A pet's state has been synchronized with detailed information.
        /// This event is emitted whenever a pet's state is updated to help off-chain systems
        /// track changes and maintain consistency.
        PetStateSynchronized {
            pet_id: PetId,
            version: u32,
            timestamp: BlockNumberFor<T>,
            change_type: u8,
            successful_hooks: u32,
            failed_hooks: u32,
        },
        
        /// A synchronization hook has been registered.
        HookRegistered {
            hook_id: u32,
            account_id: T::AccountId,
            interests: u8,
            priority: u8,
        },
        
        /// A synchronization hook has been unregistered.
        HookUnregistered {
            hook_id: u32,
            account_id: T::AccountId,
        },
        
        /// A synchronization hook has been enabled.
        HookEnabled {
            hook_id: u32,
        },
        
        /// A synchronization hook has been disabled.
        HookDisabled {
            hook_id: u32,
        },
        
        /// A synchronization hook's interests have been updated.
        HookInterestsUpdated {
            hook_id: u32,
            interests: u8,
        },
        
        /// A synchronization hook execution has succeeded.
        HookExecutionSucceeded {
            hook_id: u32,
            pet_id: PetId,
            change_type: u8,
            execution_time_ms: u32,
        },
        
        /// A synchronization hook execution has failed.
        HookExecutionFailed {
            hook_id: u32,
            pet_id: PetId,
            change_type: u8,
            error: Vec<u8>,
        },
        
        /// A pet's behavior has been predicted.
        PetBehaviorPredicted {
            pet_id: PetId,
            behavior_type: u8,
            probability: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet's state transition probabilities have been calculated.
        PetTransitionProbabilitiesCalculated {
            pet_id: PetId,
            transitions_count: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// Adaptive behavior has been applied to a pet.
        AdaptiveBehaviorApplied {
            pet_id: PetId,
            adjustment_type: u8,
            adjustment_value: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet's state has been validated.
        PetStateValidated {
            pet_id: PetId,
            is_valid: bool,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet's interaction history has been compressed.
        InteractionHistoryCompressed {
            pet_id: PetId,
            original_size: u32,
            compressed_size: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet's personality trait has evolved.
        PersonalityTraitEvolved {
            pet_id: PetId,
            trait_type: u8,
            old_intensity: u8,
            new_intensity: u8,
            catalyst: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet has interacted with another pet.
        PetSocialInteraction {
            pet_id_1: PetId,
            pet_id_2: PetId,
            interaction_type: u8,
            outcome: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet has adapted to a new environment.
        EnvironmentalAdaptation {
            pet_id: PetId,
            environment_type: u8,
            adaptation_level: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet has been trained in a skill.
        PetTrainingCompleted {
            pet_id: PetId,
            skill_type: u8,
            skill_level: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet has formed a new memory.
        MemoryFormed {
            pet_id: PetId,
            memory_type: u8,
            significance: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// Mood contagion has occurred between pets.
        MoodContagion {
            pet_id_1: PetId,
            pet_id_2: PetId,
            mood_change_1: i8,
            mood_change_2: i8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet has earned an achievement.
        AchievementEarned {
            pet_id: PetId,
            achievement_id: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A seasonal event has affected a pet.
        SeasonalEventEffect {
            pet_id: PetId,
            event_id: u32,
            effect_type: u8,
            effect_magnitude: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet has experienced a lifecycle event.
        LifecycleEvent {
            pet_id: PetId,
            event_type: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// An analytics report has been generated for a pet.
        AnalyticsReportGenerated {
            pet_id: PetId,
            report_size: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet's personality evolved.
        PersonalityEvolved {
            pet_id: PetId,
            trait_type: u8,
            old_intensity: u8,
            new_intensity: u8,
            catalyst: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A social interaction occurred between two pets.
        SocialInteraction {
            pet_id_1: PetId,
            pet_id_2: PetId,
            interaction_type: u8,
            outcome: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet adapted to an environment.
        EnvironmentalAdaptation {
            pet_id: PetId,
            environment_type: u8,
            adaptation_level: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet learned or improved a skill.
        SkillLearned {
            pet_id: PetId,
            skill_type: u8,
            skill_level: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet formed a memory.
        MemoryFormed {
            pet_id: PetId,
            memory_type: u8,
            significance: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet's memory was reinforced.
        MemoryReinforced {
            pet_id: PetId,
            memory_index: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// Mood contagion occurred between two pets.
        MoodContagion {
            pet_id_1: PetId,
            pet_id_2: PetId,
            mood_change_1: i8,
            mood_change_2: i8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet earned an achievement.
        AchievementEarned {
            pet_id: PetId,
            achievement_id: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A seasonal event started.
        SeasonalEventStarted {
            event_id: u32,
            start_time: BlockNumberFor<T>,
            end_time: BlockNumberFor<T>,
        },
        
        /// A seasonal event ended.
        SeasonalEventEnded {
            event_id: u32,
            end_time: BlockNumberFor<T>,
        },
        
        /// A seasonal event affected a pet.
        SeasonalEventEffect {
            pet_id: PetId,
            event_id: u32,
            effect_type: u8,
            effect_magnitude: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A pet experienced a lifecycle event.
        LifecycleEvent {
            pet_id: PetId,
            event_type: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A visual attribute was set for a pet.
        VisualAttributeSet {
            pet_id: PetId,
            attribute_type: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A visual theme was set for a pet.
        VisualThemeSet {
            pet_id: PetId,
            theme_id: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A gesture interaction occurred with a pet.
        GestureInteraction {
            pet_id: PetId,
            gesture_id: u8,
            response_animation: u8,
            mood_effect: i8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A touch interaction occurred with a pet.
        TouchInteraction {
            pet_id: PetId,
            touch_area: u8,
            response_id: u8,
            mood_effect: i8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A UX flow was started for a user.
        UxFlowStarted {
            account_id: T::AccountId,
            flow_id: u16,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A UX flow step was advanced for a user.
        UxFlowAdvanced {
            account_id: T::AccountId,
            flow_id: u16,
            step_id: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A UX flow step was skipped for a user.
        UxFlowStepSkipped {
            account_id: T::AccountId,
            flow_id: u16,
            step_id: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A UX flow was completed for a user.
        UxFlowCompleted {
            account_id: T::AccountId,
            flow_id: u16,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A notification was added for a user.
        NotificationAdded {
            account_id: T::AccountId,
            notification_id: u32,
            notification_type: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A notification was marked as read.
        NotificationRead {
            account_id: T::AccountId,
            notification_id: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        // Interactive session events
        /// An interactive session was started.
        InteractiveSessionStarted {
            account_id: T::AccountId,
            pet_id: PetId,
            session_id: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// An interactive session was ended.
        InteractiveSessionEnded {
            account_id: T::AccountId,
            pet_id: PetId,
            session_id: u32,
            duration: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// An interaction was recorded in a session.
        SessionInteractionRecorded {
            session_id: u32,
            interaction_type: u8,
            outcome: u8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A mood change was recorded in a session.
        SessionMoodChangeRecorded {
            session_id: u32,
            change: i8,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A reward was earned in a session.
        SessionRewardEarned {
            session_id: u32,
            reward_type: u8,
            amount: u32,
            timestamp: BlockNumberFor<T>,
        },
        
        /// A multi-touch interaction was processed.
        MultiTouchInteractionProcessed {
            pet_id: PetId,
            interaction_id: u8,
            touch_count: u8,
            timestamp: BlockNumberFor<T>,
        },
    }

    // --- Pallet Errors ---
    // Custom errors provide precise feedback on why an extrinsic failed, crucial for debugging and user experience.
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
        /// The NFT is already locked and cannot be locked again.
        NftAlreadyLocked,
        /// The NFT is not locked and thus cannot be unlocked.
        NftNotLocked,
        /// The NFT is locked and cannot be transferred by standard means.
        NftLocked, // Used when transfer is attempted on a locked NFT
        /// The cooldown period for claiming daily PTCN has not yet passed.
        ClaimCooldownNotMet,
        /// Error from the ItemHandler (e.g., item not found, not correct category, consumption failed).
        ItemInteractionFailed,
        /// Personality trait string is too long (exceeds T::MaxTraitStringLen).
        TraitStringTooLong,
        /// Pet already has the maximum number of personality traits.
        TooManyPersonalityTraits,
        /// Failed to unmarshal/reconstruct personality traits (internal error).
        FailedToReconstructPersonalityTraits, // Added for trait handling robustness
        /// Custom name provided for pet exceeds MaxPetNameLen.
        PetNameTooLong,
        /// Custom species provided for pet exceeds MaxSpeciesNameLen.
        SpeciesNameTooLong,
        /// The pet's state has been modified by another transaction.
        /// This error is used for optimistic concurrency control.
        ConcurrentModification,
        /// Failed to synchronize pet state with external systems.
        SynchronizationFailed,
        /// The specified hook was not found.
        HookNotFound,
        /// The maximum number of hooks has been reached.
        TooManyHooks,
        /// The hook execution timed out.
        HookExecutionTimeout,
        /// The hook execution failed.
        HookExecutionFailed,
        /// The hook is already registered.
        HookAlreadyRegistered,
        /// The hook is disabled.
        HookDisabled,
        /// Invalid hook parameters.
        InvalidHookParameters,
        /// The maximum number of synchronization status entries has been reached.
        TooManySyncStatusEntries,
        /// The pet's state is invalid.
        InvalidState,
        /// Failed to compress the pet's interaction history.
        CompressionFailed,
        /// Failed to decompress the pet's interaction history.
        DecompressionFailed,
        /// Failed to predict the pet's behavior.
        BehaviorPredictionFailed,
        /// Failed to calculate the pet's state transition probabilities.
        TransitionCalculationFailed,
        /// Failed to apply adaptive behavior to the pet.
        AdaptiveBehaviorFailed,
        /// The pet's interaction history is too large.
        InteractionHistoryTooLarge,
        /// The maximum number of behavior predictions has been reached.
        TooManyBehaviorPredictions,
        /// The maximum number of state transition probabilities has been reached.
        TooManyTransitionProbabilities,
        /// The maximum number of personality traits has been reached.
        TooManyTraits,
        /// The pet is not in the correct state for this operation.
        InvalidPetState,
        /// The pet is not compatible with the other pet.
        IncompatiblePets,
        /// The pet is not compatible with the environment.
        IncompatibleEnvironment,
        /// The pet does not have the required skill.
        SkillNotFound,
        /// The pet's skill level is too low.
        SkillLevelTooLow,
        /// The pet does not have the required memory.
        MemoryNotFound,
        /// The pet's memory capacity is full.
        MemoryCapacityFull,
        /// The mood contagion failed.
        MoodContagionFailed,
        /// The achievement requirements have not been met.
        AchievementRequirementsNotMet,
        /// The seasonal event is not active.
        SeasonalEventNotActive,
        /// The lifecycle event is not available.
        LifecycleEventNotAvailable,
        /// The analytics report generation failed.
        AnalyticsReportGenerationFailed,
        /// The pet is too young for this operation.
        PetTooYoung,
        /// The pet is too old for this operation.
        PetTooOld,
        /// The pet is not in the correct mood for this operation.
        InvalidMood,
        /// The pet is not in the correct environment for this operation.
        InvalidEnvironment,
        /// The pet does not have the required trait.
        TraitNotFound,
        /// The pet's trait level is too low.
        TraitLevelTooLow,
        /// The pet has too many traits.
        TooManyTraits,
        /// The pets are incompatible for this interaction.
        IncompatiblePets,
        /// The pet has too many social interactions.
        TooManySocialInteractions,
        /// The pet has too many social bonds.
        TooManySocialBonds,
        /// The pet is incompatible with this environment.
        IncompatibleEnvironment,
        /// The pet has too many environmental adaptations.
        TooManyEnvironmentalAdaptations,
        /// The pet has too many skills.
        TooManySkills,
        /// The pet's skill level is too low.
        SkillLevelTooLow,
        /// The pet has too many memories.
        TooManyMemories,
        /// The memory index is out of bounds.
        MemoryIndexOutOfBounds,
        /// The pet has too many achievements.
        TooManyAchievements,
        /// The achievement requirements are not met.
        AchievementRequirementsNotMet,
        /// The achievement was not found.
        AchievementNotFound,
        /// There are too many active seasonal events.
        TooManyActiveEvents,
        /// The seasonal event is already active.
        EventAlreadyActive,
        /// The seasonal event is not active.
        EventNotActive,
        /// The pet has too many lifecycle events.
        TooManyLifecycleEvents,
        /// The lifecycle event is not available.
        LifecycleEventNotAvailable,
        /// The lifecycle event has already been experienced.
        LifecycleEventAlreadyExperienced,
        /// The pet's stat is too low.
        StatTooLow,
        /// The analytics report is too large.
        AnalyticsReportTooLarge,
        /// The analytics report was not found.
        AnalyticsReportNotFound,
        /// The analytics report could not be decoded.
        AnalyticsReportDecodingFailed,
        /// The attribute type is invalid.
        InvalidAttributeType,
        /// The value is too long.
        ValueTooLong,
        /// The pet has too many attributes.
        TooManyAttributes,
        /// The gesture is invalid.
        InvalidGesture,
        /// The requirements are not met.
        RequirementsNotMet,
        /// No touch response is available.
        NoTouchResponse,
        /// The UX flow step cannot be skipped.
        CannotSkipStep,
        /// A UX flow is already active.
        UxFlowAlreadyActive,
        /// The notification type is invalid.
        InvalidNotificationType,
        /// The notification priority is invalid.
        InvalidNotificationPriority,
        /// The title is too long.
        TitleTooLong,
        /// The message is too long.
        MessageTooLong,
        /// The icon is too long.
        IconTooLong,
        /// The action is too long.
        ActionTooLong,
        /// The user has too many notifications.
        TooManyNotifications,
        /// The notification was not found.
        NotificationNotFound,
        /// The progress value is invalid.
        InvalidProgress,
        /// The user has too many achievements.
        TooManyAchievements,
        
        // Interactive system errors
        /// Error when a session with the given ID is not found.
        SessionNotFound,
        /// Error when a session with the given ID already exists.
        SessionAlreadyExists,
        /// Error when a session has too many interactions.
        TooManySessionInteractions,
        /// Error when a session has too many mood changes.
        TooManySessionMoodChanges,
        /// Error when a session has too many rewards.
        TooManySessionRewards,
        /// Error when a touch area with the given ID is not found.
        InvalidTouchArea,
        /// Error when a UI element with the given ID is not found.
        InvalidUiElement,
        /// Error when a pattern with the given ID is not found.
        InvalidPattern,
        /// Error when a recognition result with the given ID is not found.
        InvalidRecognitionResult,
        /// Error when a multi-touch interaction with the given ID is not found.
        InvalidMultiTouchInteraction,
        /// Error when a session has expired.
        SessionExpired,
        /// Error when a session is already active.
        SessionAlreadyActive,
        /// Error when a session is not active.
        SessionNotActive,
        /// Error when a session is locked by a different account.
        SessionLockedByDifferentAccount,
        /// Error when a session has reached its maximum duration.
        SessionDurationExceeded,
        /// Error when a session has already ended.
        SessionAlreadyEnded,
        /// Error when a session has an invalid duration.
        InvalidSessionDuration,
        /// Error when an interaction has an invalid timestamp.
        InvalidInteractionTimestamp,
        /// Error when a mood change has an invalid timestamp.
        InvalidMoodChangeTimestamp,
        /// Error when a mood change has an invalid magnitude.
        InvalidMoodChangeMagnitude,
        /// Error when an interaction type is invalid.
        InvalidInteractionType,
        /// Error when a user has too many active sessions.
        TooManySessions,
        /// Error when the interaction rate limit has been exceeded.
        InteractionRateLimitExceeded,
        /// Error when the mood change rate limit has been exceeded.
        MoodChangeRateLimitExceeded,
        /// Error when the pet is unwell and cannot be interacted with.
        PetIsUnwell,
        /// Error when a session has reached its maximum number of interactions.
        SessionInteractionsExceeded,
        /// Error when a session has reached its maximum number of mood changes.
        SessionMoodChangesExceeded,
        /// Error when a session has reached its maximum number of rewards.
        SessionRewardsExceeded,
    }

    // --- Pallet Extrinsics (Callable Functions) ---
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Mint a new Pet NFT.
        /// This creates a unique digital companion on CritterChain.
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().writes(4).reads(1)))] // Basic weight, adjust as needed
        pub fn mint_pet_nft(
            origin: OriginFor<T>,
            species: Vec<u8>, // Will be converted to BoundedVec inside
            name: Vec<u8>,    // Will be converted to BoundedVec inside
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // 1. Input Validation: Enforce BoundedVec limits for species and name.
            let bounded_species: BoundedVec<u8, T::MaxSpeciesNameLen> = species.try_into()
                .map_err(|_| Error::<T>::SpeciesNameTooLong)?;
            let bounded_name: BoundedVec<u8, T::MaxPetNameLen> = name.try_into()
                .map_err(|_| Error::<T>::PetNameTooLong)?;

            // 2. Check maximum owned pets for sender.
            ensure!(
                OwnerOfPet::<T>::get(&sender).len() < T::MaxOwnedPets::get() as usize,
                Error::<T>::ExceedMaxOwnedPets
            );

            // 3. Generate PetId.
            let pet_id = NextPetId::<T>::try_mutate(|next_id| -> Result<PetId, DispatchError> {
                let current_id = *next_id;
                *next_id = next_id.checked_add(1).ok_or(Error::<T>::NextPetIdOverflow)?;
                Ok(current_id)
            })?;

            // 4. DNA Hash Generation: Uses secure on-chain randomness.
            let (dna_seed, _) = T::PetRandomness::random_seed();
            let dna_hash_data = (dna_seed, &sender, pet_id, &bounded_species, &bounded_name).encode();
            // Using a full SHA256 hash (32 bytes) for DnaHashType, not Blake2_128 (16 bytes).
            // Need to change DnaHashType to [u8; 32] in traits/mod.rs.
            let dna_hash_val = sp_io::hashing::sha256(&dna_hash_data);

            // 5. Charter Attribute Derivation from dna_hash.
            // This algorithm is deterministic.
            let base_strength = (dna_hash_val[0] % 16) + 5; // 5-20
            let base_agility = (dna_hash_val[1] % 16) + 5;
            let base_intelligence = (dna_hash_val[2] % 16) + 5;
            let base_vitality = (dna_hash_val[3] % 16) + 5;
            let primary_elemental_affinity = match dna_hash_val[4] % 8 {
                0 => ElementType::Fire, 1 => ElementType::Water, 2 => ElementType::Earth,
                3 => ElementType::Air, 4 => ElementType::Tech, 5 => ElementType::Nature,
                6 => ElementType::Mystic,
                _ => ElementType::Neutral, // Default for 7th value, if any
            };

            // 6. Initial Dynamic Attributes (set to defaults).
            let current_block_number = frame_system::Pallet::<T>::block_number();
            let initial_mood = T::MaxMoodValue::get();
            let initial_personality_traits: BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits> = Default::default(); // Start empty

            // Initialize state version and sync flags
            let initial_state_version = 1;
            let initial_sync_flags = 0; // No synchronization has occurred yet

            let new_pet = PetNft {
                id: pet_id,
                dna_hash: dna_hash_val, // Use the 32-byte SHA256 hash
                initial_species: bounded_species.clone(),
                current_pet_name: bounded_name.clone(),
                base_strength,
                base_agility,
                base_intelligence,
                base_vitality,
                primary_elemental_affinity,
                level: 1,
                experience_points: 0,
                mood_indicator: initial_mood,
                last_fed_block: current_block_number,
                last_played_block: current_block_number,
                personality_traits: initial_personality_traits.clone(),
                last_state_update_block: current_block_number,
                state_version: initial_state_version,
                sync_flags: initial_sync_flags,
            };

            // 7. Storage Operations: Insert Pet NFT and update ownership.
            PetNfts::<T>::insert(pet_id, new_pet.clone());
            OwnerOfPet::<T>::try_mutate(&sender, |owned_pets_vec| {
                owned_pets_vec.try_push(pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
            })?;
            PetNftOwner::<T>::insert(pet_id, sender.clone());
            
            // Store the initial state version
            PetStateVersions::<T>::insert(pet_id, initial_state_version);

            // 8. Emit detailed event for transparency and off-chain indexing.
            Self::deposit_event(Event::PetNftMinted { 
                owner: sender.clone(), 
                pet_id,
                species: bounded_species,
                dna_hash: dna_hash_val,
                base_strength,
                base_agility,
                base_intelligence,
                base_vitality,
                elemental_affinity: primary_elemental_affinity,
                timestamp: current_block_number
            });
            
            // 9. Notify synchronization hooks
            use crate::sync::{SyncHookManager, StateChangeType};
            SyncHookManager::<T>::notify_hooks(
                pet_id,
                StateChangeType::BasicInfo,
                initial_state_version,
                current_block_number
            ).map_err(|_| Error::<T>::SynchronizationFailed)?;

            Ok(())
        }

        /// Transfer a Pet NFT from the sender to a recipient.
        /// Adheres to "Sense the Landscape, Secure the Solution" by checking transferability.
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(2).writes(2)))] // Adjust weight
        pub fn transfer_pet_nft(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // 1. Basic validation: Sender cannot transfer to themselves.
            ensure!(sender != recipient, Error::<T>::CannotTransferToSelf);

            // 2. Verify ownership and pet existence.
            let owner = PetNftOwner::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(owner == sender, Error::<T>::NotOwner);

            // 3. Crucial check: Ensure the NFT is transferable (not locked by marketplace, battle, etc.).
            ensure!(Self::is_transferable(&pet_id), Error::<T>::NftLocked);

            // 4. Check recipient capacity.
            let recipient_pets_count = OwnerOfPet::<T>::get(&recipient).len(); // Get length directly
            ensure!(recipient_pets_count < T::MaxOwnedPets::get() as usize, Error::<T>::RecipientExceedMaxOwnedPets);

            // 5. Mutate ownership records atomically.
            OwnerOfPet::<T>::try_mutate(&sender, |sender_owned_pets| -> DispatchResult {
                // Find and remove the pet_id from sender's owned list.
                if let Some(index) = sender_owned_pets.iter().position(|id| *id == pet_id) {
                    sender_owned_pets.swap_remove(index);
                    Ok(())
                } else {
                    // This indicates an internal inconsistency if owner check passed but pet not in list.
                    log::error!(
                        target: "runtime::critter_nfts_pallet",
                        "Inconsistency: Pet {} owned by {} but not in OwnerOfPet list.",
                        pet_id,
                        sender
                    );
                    Err(Error::<T>::PetNotFound.into()) // More robust error, or panic in debug.
                }
            })?;

            OwnerOfPet::<T>::try_mutate(&recipient, |recipient_owned_pets| -> DispatchResult {
                // Add pet_id to recipient's owned list.
                recipient_owned_pets.try_push(pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
                // This error should ideally be caught by check 4, but good to have defense in depth.
            })?;

            // Update the direct owner mapping for the pet.
            PetNftOwner::<T>::insert(pet_id, recipient.clone());

            // 6. Emit event for transparency and off-chain indexing.
            Self::deposit_event(Event::PetNftTransferred { from: sender, to: recipient, pet_id });

            Ok(())
        }

        /// Update mutable metadata for a Pet NFT.
        /// Only the owner of the Pet NFT can perform this action.
        /// Fields set to `None` will not be updated.
        /// This is crucial for owner agency in pet development and AI personality integration.
        /// Uses optimistic concurrency control to prevent conflicting updates.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(3).writes(2), 0))] // Reads: PetNftOwner, PetNfts, PetStateVersions. Writes: PetNfts, PetStateVersions.
        pub fn update_pet_metadata(
            origin: OriginFor<T>,
            pet_id: PetId,
            name: Option<Vec<u8>>,
            personality_traits: Option<BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits>>,
            expected_version: u32, // For optimistic concurrency control
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let current_block_number = frame_system::Pallet::<T>::block_number();

            // 1. Verify ownership.
            let owner = PetNftOwner::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(owner == sender, Error::<T>::NotOwner);
            
            // 2. Check version for optimistic concurrency control
            let current_version = PetStateVersions::<T>::get(pet_id);
            ensure!(current_version == expected_version, Error::<T>::ConcurrentModification);
            
            // 3. Calculate the new version
            let new_version = current_version.saturating_add(1);
            
            // 4. Prepare variables to capture changes for the event
            let mut new_name_for_event: Option<BoundedVec<u8, T::MaxPetNameLen>> = None;
            let mut new_traits_for_event: Option<BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits>> = None;

            // 5. Mutate PetNft data.
            PetNfts::<T>::try_mutate(&pet_id, |pet_nft_opt| -> DispatchResult {
                let pet_nft = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                
                // Verify version again inside the transaction
                ensure!(pet_nft.state_version == expected_version, Error::<T>::ConcurrentModification);

                // Selectively update name if provided.
                if let Some(new_name) = name {
                    // Use try_into() for BoundedVec conversion and propagate error.
                    let bounded_name: BoundedVec<u8, T::MaxPetNameLen> = new_name.try_into()
                        .map_err(|_| Error::<T>::PetNameTooLong)?;
                    pet_nft.current_pet_name = bounded_name.clone();
                    new_name_for_event = Some(bounded_name);
                }

                // Selectively update personality traits if provided.
                // This replaces all existing traits with the new set.
                if let Some(new_traits) = personality_traits.clone() {
                    // COMMENT: This field is updated by the owner. For the AI Personality system
                    // (conceptually an off-chain engine detailed in AI_PERSONALITY_ENGINE.md),
                    // suggestions for new or modified personality traits would be presented to the owner.
                    // If the owner accepts these suggestions, they would use this extrinsic, providing
                    // the complete, updated list of traits. This ensures owner agency over on-chain
                    // personality changes for their Pet NFT.
                    
                    // Validate new_traits content - BoundedVec already checks overall length.
                    // Check individual trait string lengths.
                    for trait_string in new_traits.iter() {
                         ensure!(trait_string.len() <= T::MaxTraitStringLen::get() as usize, Error::<T>::TraitStringTooLong);
                    }
                    pet_nft.personality_traits = new_traits.clone();
                    new_traits_for_event = Some(new_traits);
                }

                // Update the last state update block and version
                pet_nft.last_state_update_block = current_block_number;
                pet_nft.state_version = new_version;
                
                // Set the appropriate sync flags
                if new_name_for_event.is_some() {
                    pet_nft.sync_flags |= crate::sync::state_change_to_flag(crate::sync::StateChangeType::BasicInfo);
                }
                if new_traits_for_event.is_some() {
                    pet_nft.sync_flags |= crate::sync::state_change_to_flag(crate::sync::StateChangeType::Traits);
                }
                
                Ok(())
            })?;
            
            // 6. Update the state version in storage
            PetStateVersions::<T>::insert(pet_id, new_version);

            // 7. Emit detailed event for transparency.
            Self::deposit_event(Event::PetNftMetadataUpdated { 
                owner: sender.clone(), 
                pet_id,
                new_name: new_name_for_event,
                new_traits: new_traits_for_event,
                timestamp: current_block_number
            });
            
            // 8. Notify synchronization hooks
            use crate::sync::{SyncHookManager, StateChangeType};
            if new_name_for_event.is_some() {
                SyncHookManager::<T>::notify_hooks(
                    pet_id,
                    StateChangeType::BasicInfo,
                    new_version,
                    current_block_number
                ).map_err(|_| Error::<T>::SynchronizationFailed)?;
            }
            if new_traits_for_event.is_some() {
                SyncHookManager::<T>::notify_hooks(
                    pet_id,
                    StateChangeType::Traits,
                    new_version,
                    current_block_number
                ).map_err(|_| Error::<T>::SynchronizationFailed)?;
            }
            
            Ok(())
        }

        /// Allows a user to claim their daily PTCN reward.
        /// This mechanism incentivizes consistent engagement with the CritterCraft ecosystem.
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(1).writes(2)))] // Adjust weight
        pub fn claim_daily_ptcn(origin: OriginFor<T>) -> DispatchResult {
            let claimer = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();
            let last_claim_block = LastClaimTime::<T>::get(&claimer);

            // 1. Check if the cooldown period has passed.
            ensure!(
                current_block >= last_claim_block.saturating_add(T::ClaimCooldownPeriod::get()),
                Error::<T>::ClaimCooldownNotMet
            );

            let amount = T::DailyClaimAmount::get();

            // 2. Reward the user with PTCN.
            // This uses `deposit_creating` which ensures the account exists and receives funds.
            // For existing accounts, it increases the free balance if it's not below existential deposit.
            // This is a simplified direct issuance for the purpose of this pallet.
            T::Currency::deposit_creating(&claimer, amount);

            // 3. Update the last claim time for the user.
            LastClaimTime::<T>::insert(&claimer, current_block);

            // 4. Emit event for transparency.
            Self::deposit_event(Event::DailyClaimMade {
                account: claimer,
                amount,
                claim_time: current_block,
            });

            Ok(())
        }

        /// Feed a pet with a specified food item.
        /// This promotes pet nurturing and directly impacts dynamic attributes.
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(2).writes(1)))] // R: Owner, Item; W: PetNft
        pub fn feed_pet(origin: OriginFor<T>, pet_id: PetId, food_item_id: ItemId) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            // 1. Check if the sender owns the pet.
            ensure!(Self::pet_nft_owner(pet_id) == Some(owner.clone()), Error::<T>::NotOwner);

            // 2. Consume the specified food item via the ItemHandler.
            // This interaction confirms the item exists, is of the correct category (Food),
            // and atomically deducts it from inventory, ensuring synchronized state.
            // We pass ItemHandler's internally defined FOOD_CATEGORY_TAG via its associated type if it had one,
            // or a concrete value if it's a global constant in pallet-items.
            T::ItemHandler::consume_item_of_category(&owner, &food_item_id, T::ItemHandler::food_category_tag())
                .map_err(|_| Error::<T>::ItemInteractionFailed)?; 
            
            // 3. Update pet's attributes.
            PetNfts::<T>::try_mutate(&pet_id, |pet_nft_opt| -> DispatchResult {
                let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                let current_block = frame_system::Pallet::<T>::block_number();

                // Update last fed time.
                pet.last_fed_block = current_block;
                // Boost mood, capped by MaxMoodValue.
                pet.mood_indicator = pet.mood_indicator.saturating_add(T::FeedMoodBoost::get()).min(T::MaxMoodValue::get());
                // Grant XP.
                pet.experience_points = pet.experience_points.saturating_add(T::FeedXpGain::get());
                // Attempt to level up based on new XP.
                Self::attempt_level_up(pet)?;
                // Record this interaction timestamp.
                pet.last_state_update_block = current_block;
                Ok(())
            })?;

            // 4. Emit event for transparency.
            Self::deposit_event(Event::PetFed { owner, pet_id, food_item_id });
            Ok(())
        }

        /// Play with a pet using a specified toy item.
        /// This promotes pet nurturing and directly impacts dynamic attributes.
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, T::DbWeight::get().reads(2).writes(1)))] // Similar to feed_pet
        pub fn play_with_pet(origin: OriginFor<T>, pet_id: PetId, toy_item_id: ItemId) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            // 1. Check if the sender owns the pet.
            ensure!(Self::pet_nft_owner(pet_id) == Some(owner.clone()), Error::<T>::NotOwner);

            // 2. Consume the specified toy item via the ItemHandler.
            T::ItemHandler::consume_item_of_category(&owner, &toy_item_id, T::ItemHandler::toy_category_tag())
                .map_err(|_| Error::<T>::ItemInteractionFailed)?;

            // 3. Update pet's attributes.
            PetNfts::<T>::try_mutate(&pet_id, |pet_nft_opt| -> DispatchResult {
                let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                let current_block = frame_system::Pallet::<T>::block_number();

                // Update last played time (also general care timestamp).
                pet.last_played_block = current_block;
                // Boost mood.
                pet.mood_indicator = pet.mood_indicator.saturating_add(T::PlayMoodBoost::get()).min(T::MaxMoodValue::get());
                // Grant XP.
                pet.experience_points = pet.experience_points.saturating_add(T::PlayXpGain::get());
                // Attempt to level up.
                Self::attempt_level_up(pet)?;
                // Record this interaction.
                pet.last_state_update_block = current_block;
                Ok(())
            })?;

            // 4. Emit event.
            Self::deposit_event(Event::PetPlayedWith { owner, pet_id, toy_item_id });
            Ok(())
        }

        /// Batch mint multiple Pet NFTs in a single transaction.
        /// This is more efficient than calling mint_pet_nft multiple times.
        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(10_000 * pets.len() as u64, T::DbWeight::get().writes(4 * pets.len() as u64).reads(1 * pets.len() as u64)))]
        pub fn batch_mint_pet_nfts(
            origin: OriginFor<T>,
            pets: Vec<(Vec<u8>, Vec<u8>)>, // Vector of (species, name) pairs
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // 1. Check that the sender has enough capacity for all the new pets
            let current_pet_count = OwnerOfPet::<T>::get(&sender).len();
            let max_pets = T::MaxOwnedPets::get() as usize;
            ensure!(
                current_pet_count + pets.len() <= max_pets,
                Error::<T>::ExceedMaxOwnedPets
            );
            
            // 2. Process each pet in the batch
            let mut minted_pet_ids = Vec::with_capacity(pets.len());
            let current_block_number = frame_system::Pallet::<T>::block_number();
            
            for (species, name) in pets {
                // 2.1 Input Validation: Enforce BoundedVec limits for species and name
                let bounded_species: BoundedVec<u8, T::MaxSpeciesNameLen> = species.try_into()
                    .map_err(|_| Error::<T>::SpeciesNameTooLong)?;
                let bounded_name: BoundedVec<u8, T::MaxPetNameLen> = name.try_into()
                    .map_err(|_| Error::<T>::PetNameTooLong)?;
                
                // 2.2 Generate PetId
                let pet_id = NextPetId::<T>::try_mutate(|next_id| -> Result<PetId, DispatchError> {
                    let current_id = *next_id;
                    *next_id = next_id.checked_add(1).ok_or(Error::<T>::NextPetIdOverflow)?;
                    Ok(current_id)
                })?;
                
                // 2.3 DNA Hash Generation
                let (dna_seed, _) = T::PetRandomness::random_seed();
                let dna_hash_data = (dna_seed, &sender, pet_id, &bounded_species, &bounded_name).encode();
                let dna_hash_val = sp_io::hashing::sha256(&dna_hash_data);
                
                // 2.4 Charter Attribute Derivation
                let base_strength = (dna_hash_val[0] % 16) + 5;
                let base_agility = (dna_hash_val[1] % 16) + 5;
                let base_intelligence = (dna_hash_val[2] % 16) + 5;
                let base_vitality = (dna_hash_val[3] % 16) + 5;
                let primary_elemental_affinity = match dna_hash_val[4] % 8 {
                    0 => ElementType::Fire, 1 => ElementType::Water, 2 => ElementType::Earth,
                    3 => ElementType::Air, 4 => ElementType::Tech, 5 => ElementType::Nature,
                    6 => ElementType::Mystic,
                    _ => ElementType::Neutral,
                };
                
                // 2.5 Initialize state version and sync flags
                let initial_state_version = 1;
                let initial_sync_flags = 0;
                let initial_mood = T::MaxMoodValue::get();
                let initial_personality_traits: BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits> = Default::default();
                
                // 2.6 Create the new pet
                let new_pet = PetNft {
                    id: pet_id,
                    dna_hash: dna_hash_val,
                    initial_species: bounded_species.clone(),
                    current_pet_name: bounded_name.clone(),
                    base_strength,
                    base_agility,
                    base_intelligence,
                    base_vitality,
                    primary_elemental_affinity,
                    level: 1,
                    experience_points: 0,
                    mood_indicator: initial_mood,
                    last_fed_block: current_block_number,
                    last_played_block: current_block_number,
                    personality_traits: initial_personality_traits.clone(),
                    last_state_update_block: current_block_number,
                    state_version: initial_state_version,
                    sync_flags: initial_sync_flags,
                };
                
                // 2.7 Storage Operations
                PetNfts::<T>::insert(pet_id, new_pet.clone());
                PetNftOwner::<T>::insert(pet_id, sender.clone());
                PetStateVersions::<T>::insert(pet_id, initial_state_version);
                
                // 2.8 Emit event
                Self::deposit_event(Event::PetNftMinted { 
                    owner: sender.clone(), 
                    pet_id,
                    species: bounded_species,
                    dna_hash: dna_hash_val,
                    base_strength,
                    base_agility,
                    base_intelligence,
                    base_vitality,
                    elemental_affinity: primary_elemental_affinity,
                    timestamp: current_block_number
                });
                
                // 2.9 Notify synchronization hooks
                use crate::sync::{SyncHookManager, StateChangeType};
                SyncHookManager::<T>::notify_hooks(
                    pet_id,
                    StateChangeType::BasicInfo,
                    initial_state_version,
                    current_block_number
                ).map_err(|_| Error::<T>::SynchronizationFailed)?;
                
                // 2.10 Add to minted pet IDs
                minted_pet_ids.push(pet_id);
            }
            
            // 3. Update the owner's pet list with all minted pets
            OwnerOfPet::<T>::try_mutate(&sender, |owned_pets_vec| -> DispatchResult {
                for pet_id in minted_pet_ids {
                    owned_pets_vec.try_push(pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)?;
                }
                Ok(())
            })?;
            
            Ok(())
        }
        
        /// Potentially apply neglect effects if the pet hasn't been interacted with for a long time.
        /// This is a public extrinsic, designed to be called by any account (e.g., an off-chain worker,
        /// another player as a utility function, or the owner themselves) to trigger neglect calculations.
        #[pallet::call_index(7)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))] // Reads: PetNfts. Writes: PetNfts.
        pub fn apply_neglect_check(origin: OriginFor<T>, pet_id: PetId) -> DispatchResult {
            let _sender = ensure_signed(origin)?; // Ensure the call is signed for security/spam prevention.

            // 1. Mutate the PetNft state.
            PetNfts::<T>::try_mutate(&pet_id, |pet_nft_opt| -> DispatchResult {
                let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                let current_block = frame_system::Pallet::<T>::block_number();

                // 2. Check if the neglect threshold has been passed since the last play/care interaction.
                // Using saturating_sub to prevent underflow if current_block is very low for some reason.
                if current_block.saturating_sub(pet.last_played_block) > T::NeglectThresholdBlocks::get() {
                    let old_mood = pet.mood_indicator;
                    // 3. Apply mood penalty due to neglect.
                    // Mood cannot go below 0.
                    pet.mood_indicator = pet.mood_indicator.saturating_sub(T::NeglectMoodPenalty::get());
                    // 4. Update the last state update block to reflect this change.
                    pet.last_state_update_block = current_block;

                    // 5. Emit an event if mood actually changed due to neglect.
                    if pet.mood_indicator != old_mood {
                       Self::deposit_event(Event::PetNeglected{ 
                           pet_id: pet.id, 
                           old_mood,
                           new_mood: pet.mood_indicator,
                           neglect_duration: current_block.saturating_sub(pet.last_played_block),
                           timestamp: current_block
                       });
                    }
                }
                // If neglect threshold not met, no state changes or events occur for neglect, and returns Ok.
                Ok(())
            })
        }
        
        /// Predict a pet's behavior based on its current state and interaction history.
        /// This is used for advanced state management and predictive analytics.
        #[pallet::call_index(8)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn predict_pet_behavior(origin: OriginFor<T>, pet_id: PetId) -> DispatchResult {
            let _sender = ensure_signed(origin)?; // Anyone can call this
            
            // Use the PetStateManager to predict behavior
            let behaviors = state::PetStateManager::<T>::predict_behavior(pet_id)?;
            
            // Store the predictions
            let mut predictions = Vec::with_capacity(behaviors.len());
            for behavior in &behaviors {
                predictions.push((behavior.behavior_type, behavior.intensity));
            }
            
            // Convert to BoundedVec
            let bounded_predictions: BoundedVec<(u8, u8), T::MaxBehaviorPredictions> = 
                predictions.try_into().map_err(|_| Error::<T>::TooManyBehaviorPredictions)?;
            
            // Store in storage
            PetBehaviorPredictions::<T>::insert(pet_id, bounded_predictions);
            
            // Emit event
            let current_block = frame_system::Pallet::<T>::block_number();
            if let Some(first_behavior) = behaviors.first() {
                Self::deposit_event(Event::PetBehaviorPredicted {
                    pet_id,
                    behavior_type: first_behavior.behavior_type,
                    probability: first_behavior.intensity,
                    timestamp: current_block,
                });
            }
            
            Ok(())
        }
        
        /// Calculate a pet's state transition probabilities.
        /// This is used for advanced state management and predictive analytics.
        #[pallet::call_index(9)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn calculate_pet_transitions(origin: OriginFor<T>, pet_id: PetId) -> DispatchResult {
            let _sender = ensure_signed(origin)?; // Anyone can call this
            
            // Use the PetStateManager to calculate transitions
            let transitions = state::PetStateManager::<T>::calculate_transitions(pet_id)?;
            
            // Store the transitions
            let mut probabilities = Vec::with_capacity(transitions.len());
            for transition in &transitions {
                probabilities.push((transition.from_state, transition.to_state, transition.probability));
            }
            
            // Convert to BoundedVec
            let bounded_probabilities: BoundedVec<(u8, u8, u8), T::MaxTransitionProbabilities> = 
                probabilities.try_into().map_err(|_| Error::<T>::TooManyTransitionProbabilities)?;
            
            // Store in storage
            PetTransitionProbabilities::<T>::insert(pet_id, bounded_probabilities);
            
            // Emit event
            let current_block = frame_system::Pallet::<T>::block_number();
            Self::deposit_event(Event::PetTransitionProbabilitiesCalculated {
                pet_id,
                transitions_count: transitions.len() as u32,
                timestamp: current_block,
            });
            
            Ok(())
        }
        
        /// Apply adaptive behavior adjustments to a pet.
        /// This is used for advanced state management and adaptive behavior.
        #[pallet::call_index(10)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn apply_adaptive_behavior(origin: OriginFor<T>, pet_id: PetId) -> DispatchResult {
            let _sender = ensure_signed(origin)?; // Anyone can call this
            
            // Use the PetStateManager to apply adaptive behavior
            state::PetStateManager::<T>::apply_adaptive_behavior(pet_id)?;
            
            // Emit event
            let current_block = frame_system::Pallet::<T>::block_number();
            Self::deposit_event(Event::AdaptiveBehaviorApplied {
                pet_id,
                adjustment_type: 0, // Mood adjustment
                adjustment_value: 10, // +10 to mood
                timestamp: current_block,
            });
            
            Ok(())
        }
        
        /// Validate a pet's state for integrity and consistency.
        /// This is used for advanced state management and data validation.
        #[pallet::call_index(11)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(0), 0))]
        pub fn validate_pet_state(origin: OriginFor<T>, pet_id: PetId) -> DispatchResult {
            let _sender = ensure_signed(origin)?; // Anyone can call this
            
            // Use the PetStateManager to validate state
            let result = state::PetStateManager::<T>::validate_state(pet_id);
            let is_valid = result.is_ok();
            
            // Emit event
            let current_block = frame_system::Pallet::<T>::block_number();
            Self::deposit_event(Event::PetStateValidated {
                pet_id,
                is_valid,
                timestamp: current_block,
            });
            
            // Return the validation result
            result
        }
        
        /// Register a synchronization hook.
        /// This allows other pallets to be notified of pet state changes.
        #[pallet::call_index(12)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn register_sync_hook(
            origin: OriginFor<T>,
            hook_id: u32,
            interests: u8,
            priority: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Check if we've reached the maximum number of hooks
            let hook_count = SyncHookRegistry::<T>::iter().count() as u32;
            ensure!(
                hook_count < T::MaxRegisteredHooks::get(),
                Error::<T>::TooManyHooks
            );
            
            // Check if the hook already exists
            ensure!(
                !SyncHookRegistry::<T>::contains_key(hook_id),
                Error::<T>::HookAlreadyRegistered
            );
            
            // Register the hook
            sync::SyncHookManager::<T>::register_hook(
                hook_id,
                sender,
                interests,
                priority,
            )
        }
        
        /// Unregister a synchronization hook.
        #[pallet::call_index(13)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn unregister_sync_hook(
            origin: OriginFor<T>,
            hook_id: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Check if the hook exists and is owned by the sender
            if let Some(hook_info) = SyncHookRegistry::<T>::get(hook_id) {
                ensure!(
                    hook_info.account_id == sender,
                    Error::<T>::NotOwner
                );
                
                // Unregister the hook
                sync::SyncHookManager::<T>::unregister_hook(hook_id)
            } else {
                Err(Error::<T>::HookNotFound.into())
            }
        }
        
        /// Enable or disable a synchronization hook.
        #[pallet::call_index(14)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn set_hook_enabled(
            origin: OriginFor<T>,
            hook_id: u32,
            enabled: bool,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Check if the hook exists and is owned by the sender
            if let Some(hook_info) = SyncHookRegistry::<T>::get(hook_id) {
                ensure!(
                    hook_info.account_id == sender,
                    Error::<T>::NotOwner
                );
                
                // Enable or disable the hook
                sync::SyncHookManager::<T>::set_hook_enabled(hook_id, enabled)
            } else {
                Err(Error::<T>::HookNotFound.into())
            }
        }
        
        /// Update a synchronization hook's interests.
        #[pallet::call_index(15)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn update_hook_interests(
            origin: OriginFor<T>,
            hook_id: u32,
            interests: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Check if the hook exists and is owned by the sender
            if let Some(hook_info) = SyncHookRegistry::<T>::get(hook_id) {
                ensure!(
                    hook_info.account_id == sender,
                    Error::<T>::NotOwner
                );
                
                // Update the hook's interests
                sync::SyncHookManager::<T>::update_hook_interests(hook_id, interests)
            } else {
                Err(Error::<T>::HookNotFound.into())
            }
        }
        
        /// Facilitates a social interaction between two pets.
        #[pallet::call_index(16)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(4).writes(4), 0))]
        pub fn pet_social_interaction(
            origin: OriginFor<T>,
            pet_id_1: PetId,
            pet_id_2: PetId,
            interaction_type: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns at least one of the pets
            let owner1 = Self::pet_nft_owner(&pet_id_1).ok_or(Error::<T>::PetNotFound)?;
            let owner2 = Self::pet_nft_owner(&pet_id_2).ok_or(Error::<T>::PetNotFound)?;
            
            ensure!(
                sender == owner1 || sender == owner2,
                Error::<T>::NotOwner
            );
            
            // Facilitate the interaction
            social::SocialInteractionSystem::<T>::interact(
                pet_id_1,
                pet_id_2,
                interaction_type,
            )
        }
        
        /// Evolves a pet's personality based on an interaction.
        #[pallet::call_index(17)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn evolve_pet_personality(
            origin: OriginFor<T>,
            pet_id: PetId,
            catalyst: u8,
            intensity: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Evolve the pet's personality
            personality::PersonalityEvolutionSystem::<T>::evolve_personality(
                pet_id,
                catalyst,
                intensity,
            )
        }
        
        /// Adapts a pet to a new environment.
        #[pallet::call_index(18)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(2), 0))]
        pub fn adapt_to_environment(
            origin: OriginFor<T>,
            pet_id: PetId,
            environment_type: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Adapt the pet to the environment
            environment::EnvironmentalAdaptationSystem::<T>::adapt_to_environment(
                pet_id,
                environment_type,
            )
        }
        
        /// Trains a pet in a specific skill.
        #[pallet::call_index(19)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(3).writes(2), 0))]
        pub fn train_pet(
            origin: OriginFor<T>,
            pet_id: PetId,
            skill_type: u8,
            training_intensity: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Train the pet
            training::PetTrainingSystem::<T>::train_pet(
                pet_id,
                skill_type,
                training_intensity,
            )
        }
        
        /// Records a memory for a pet.
        #[pallet::call_index(20)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn record_pet_memory(
            origin: OriginFor<T>,
            pet_id: PetId,
            memory_type: u8,
            significance: u8,
            associated_data: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Record the memory
            memory::PetMemorySystem::<T>::record_memory(
                pet_id,
                memory_type,
                significance,
                associated_data,
            )
        }
        
        /// Reinforces a memory for a pet.
        #[pallet::call_index(21)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn reinforce_pet_memory(
            origin: OriginFor<T>,
            pet_id: PetId,
            memory_index: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Reinforce the memory
            memory::PetMemorySystem::<T>::reinforce_memory(
                pet_id,
                memory_index as usize,
            )
        }
        
        /// Processes mood contagion between two pets.
        #[pallet::call_index(22)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(4).writes(2), 0))]
        pub fn process_mood_contagion(
            origin: OriginFor<T>,
            pet_id_1: PetId,
            pet_id_2: PetId,
            interaction_duration: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns at least one of the pets
            let owner1 = Self::pet_nft_owner(&pet_id_1).ok_or(Error::<T>::PetNotFound)?;
            let owner2 = Self::pet_nft_owner(&pet_id_2).ok_or(Error::<T>::PetNotFound)?;
            
            ensure!(
                sender == owner1 || sender == owner2,
                Error::<T>::NotOwner
            );
            
            // Process mood contagion
            mood::MoodContagionSystem::<T>::process_mood_contagion(
                pet_id_1,
                pet_id_2,
                interaction_duration,
            )
        }
        
        /// Checks if a pet has earned any new achievements.
        #[pallet::call_index(23)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(10).writes(2), 0))]
        pub fn check_pet_achievements(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Anyone can check achievements for any pet
            // This is a public service that benefits the ecosystem
            
            // Check achievements
            achievements::AchievementSystem::<T>::check_achievements(pet_id)
        }
        
        /// Starts a new seasonal event.
        #[pallet::call_index(24)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn start_seasonal_event(
            origin: OriginFor<T>,
            event_id: u32,
            duration: T::BlockNumber,
        ) -> DispatchResult {
            ensure_root(origin)?; // Only the root account can start seasonal events
            
            // Start the event
            seasonal::SeasonalEventSystem::<T>::start_event(event_id, duration)
        }
        
        /// Ends a seasonal event.
        #[pallet::call_index(25)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn end_seasonal_event(
            origin: OriginFor<T>,
            event_id: u32,
        ) -> DispatchResult {
            ensure_root(origin)?; // Only the root account can end seasonal events
            
            // End the event
            seasonal::SeasonalEventSystem::<T>::end_event(event_id)
        }
        
        /// Applies seasonal event effects to a pet.
        #[pallet::call_index(26)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(3).writes(1), 0))]
        pub fn apply_seasonal_effects(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Apply seasonal effects
            seasonal::SeasonalEventSystem::<T>::apply_seasonal_effects(pet_id)
        }
        
        /// Updates active seasonal events, ending those that have expired.
        #[pallet::call_index(27)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn update_seasonal_events(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?; // Anyone can update seasonal events
            
            // Update active events
            seasonal::SeasonalEventSystem::<T>::update_active_events()
        }
        
        /// Triggers a lifecycle event for a pet.
        #[pallet::call_index(28)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(5).writes(3), 0))]
        pub fn trigger_lifecycle_event(
            origin: OriginFor<T>,
            pet_id: PetId,
            event_type: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Trigger the lifecycle event
            lifecycle::LifecycleEventSystem::<T>::trigger_lifecycle_event(pet_id, event_type)
        }
        
        /// Generates an analytics report for a pet.
        #[pallet::call_index(29)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(20).writes(1), 0))]
        pub fn generate_analytics_report(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Anyone can generate an analytics report for any pet
            // This is a public service that benefits the ecosystem
            
            // Generate the report
            let _ = analytics::PetAnalyticsDashboard::<T>::generate_analytics_report(pet_id)?;
            
            Ok(())
        }
        
        /// Initiates a social interaction between two pets.
        #[pallet::call_index(30)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(5).writes(6), 0))]
        pub fn social_interact(
            origin: OriginFor<T>,
            pet_id_1: PetId,
            pet_id_2: PetId,
            interaction_type: u8,
            duration: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns at least one of the pets
            let owner1 = Self::pet_nft_owner(&pet_id_1).ok_or(Error::<T>::PetNotFound)?;
            let owner2 = Self::pet_nft_owner(&pet_id_2).ok_or(Error::<T>::PetNotFound)?;
            
            ensure!(
                sender == owner1 || sender == owner2,
                Error::<T>::NotOwner
            );
            
            // Initiate the social interaction
            social::SocialInteractionSystem::<T>::interact(
                pet_id_1,
                pet_id_2,
                interaction_type,
                duration,
            )
        }
        
        /// Adapts a pet to a new environment.
        #[pallet::call_index(31)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(3).writes(2), 0))]
        pub fn adapt_to_environment(
            origin: OriginFor<T>,
            pet_id: PetId,
            environment_type: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Adapt the pet to the environment
            environment::EnvironmentalAdaptationSystem::<T>::adapt_to_environment(
                pet_id,
                environment_type,
            )
        }
        
        /// Sets a visual attribute for a pet.
        #[pallet::call_index(32)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn set_visual_attribute(
            origin: OriginFor<T>,
            pet_id: PetId,
            attribute_type: u8,
            value: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Set the visual attribute
            visual::VisualSystem::<T>::set_visual_attribute(
                pet_id,
                attribute_type,
                value,
            )
        }
        
        /// Sets the visual theme for a pet.
        #[pallet::call_index(33)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn set_visual_theme(
            origin: OriginFor<T>,
            pet_id: PetId,
            theme_id: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Set the visual theme
            visual::VisualSystem::<T>::set_visual_theme(
                pet_id,
                theme_id,
            )
        }
        
        /// Processes a gesture interaction with a pet.
        #[pallet::call_index(34)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn process_gesture(
            origin: OriginFor<T>,
            pet_id: PetId,
            gesture_id: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Process the gesture
            let _ = interactive::InteractiveSystem::<T>::process_gesture(
                pet_id,
                gesture_id,
            )?;
            
            Ok(())
        }
        
        /// Processes a touch interaction with a pet.
        #[pallet::call_index(35)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(1), 0))]
        pub fn process_touch(
            origin: OriginFor<T>,
            pet_id: PetId,
            touch_area: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure the sender owns the pet
            let owner = Self::pet_nft_owner(&pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(sender == owner, Error::<T>::NotOwner);
            
            // Process the touch
            let _ = interactive::InteractiveSystem::<T>::process_touch(
                pet_id,
                touch_area,
            )?;
            
            Ok(())
        }
        
        /// Starts a UX flow for a user.
        #[pallet::call_index(36)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn start_ux_flow(
            origin: OriginFor<T>,
            flow_id: u16,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Start the UX flow
            let _ = user_experience::UserExperienceSystem::<T>::start_ux_flow(
                sender,
                flow_id,
            )?;
            
            Ok(())
        }
        
        /// Advances a user to the next UX flow step.
        #[pallet::call_index(37)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn advance_ux_flow(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Advance the UX flow
            let _ = user_experience::UserExperienceSystem::<T>::advance_ux_flow(
                sender,
            )?;
            
            Ok(())
        }
        
        /// Skips the current UX flow step for a user.
        #[pallet::call_index(38)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn skip_ux_flow_step(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Skip the UX flow step
            let _ = user_experience::UserExperienceSystem::<T>::skip_ux_flow_step(
                sender,
            )?;
            
            Ok(())
        }
        
        /// Marks a notification as read.
        #[pallet::call_index(39)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn mark_notification_as_read(
            origin: OriginFor<T>,
            notification_id: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Mark the notification as read
            user_experience::UserExperienceSystem::<T>::mark_notification_as_read(
                sender.clone(),
                notification_id,
            )?;
            
            // Emit event
            Self::deposit_event(Event::NotificationRead {
                account_id: sender,
                notification_id,
                timestamp: frame_system::Pallet::<T>::block_number(),
            });
            
            Ok(())
        }
        
        /// Adds a notification for a user.
        #[pallet::call_index(40)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(1).writes(1), 0))]
        pub fn add_user_notification(
            origin: OriginFor<T>,
            target_account: T::AccountId,
            notification_type: u8,
            title: Vec<u8>,
            message: Vec<u8>,
            icon: Vec<u8>,
            priority: u8,
            action: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Only allow certain accounts to add notifications for others
            // For now, we'll allow any account to add notifications
            // In a production environment, this should be restricted
            
            // Add the notification
            let _ = Self::add_notification(
                target_account,
                notification_type,
                title,
                message,
                icon,
                priority,
                action,
            )?;
            
            Ok(())
        }
        
        /// Updates achievement progress for a user.
        #[pallet::call_index(41)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(2).writes(2), 0))]
        pub fn update_user_achievement(
            origin: OriginFor<T>,
            achievement_id: u32,
            progress: u8,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Update the achievement progress
            let _ = Self::update_achievement_progress(
                sender,
                achievement_id,
                progress,
            )?;
            
            Ok(())
        }
        
        /// Gets a UI-friendly pet profile.
        #[pallet::call_index(42)]
        #[pallet::weight(Weight::from_parts(T::DbWeight::get().reads(10).writes(0), 0))]
        pub fn get_pet_profile(
            origin: OriginFor<T>,
            pet_id: PetId,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            
            // Get the pet profile
            let _ = Self::get_ui_pet_profile(pet_id)?;
            
            // This is a read-only extrinsic, so we don't need to emit an event
            // The profile will be returned in the RPC response
            
            Ok(().into())
        }
    }

    // --- Pallet Internal Helper Functions ---
    // These functions are not directly callable as extrinsics but are used internally by the pallet.
    impl<T: Config> Pallet<T> {
        /// Internal helper to handle pet level ups based on experience points.
        /// This is called after interactions that grant XP.
        fn attempt_level_up(pet: &mut PetNft<T>) -> DispatchResult {
            // 1. Define XP needed for the next level (example: 100 XP per level).
            // This calculation could be made more complex using T::Config constants for a curve.
            let xp_needed_for_next_level = 100u32.saturating_mul(pet.level);

            // 2. Check if pet has enough XP to level up.
            if pet.experience_points >= xp_needed_for_next_level && xp_needed_for_next_level > 0 { // Ensure XP needed is positive
                // 3. Increment level.
                pet.level = pet.level.saturating_add(1);
                // 4. Deduct XP used for leveling (carry over excess XP).
                pet.experience_points = pet.experience_points.saturating_sub(xp_needed_for_next_level);

                // 5. Emit event for transparency and off-chain indexing.
                Self::deposit_event(Event::PetLeveledUp { pet_id: pet.id, new_level: pet.level });
            }
            Ok(())
        }

        /// Helper function to get a UI-friendly pet profile.
        /// 
        /// # Parameters
        /// 
        /// * `pet_id` - The ID of the pet
        /// 
        /// # Returns
        /// 
        /// * `Result<ui_bridge::UiPetProfile<T>, DispatchError>` - The pet profile, or an error
        pub fn get_ui_pet_profile(pet_id: PetId) -> Result<ui_bridge::UiPetProfile<T>, DispatchError> {
            ui_bridge::UiBridge::<T>::get_pet_profile(pet_id)
        }
        
        // --- Shared NFT Manager Trait Helper ---
        // This is a helper function specifically for the SharedNftManager trait implementation.
        // It provides a centralized way to determine if a pet is transferable.
        fn is_transferable(pet_id: &PetId) -> bool {
            !LockedNfts::<T>::contains_key(pet_id)
        }
    }
}

// --- Trait Implementations (for External Pallet Interactions) ---
// These implementations allow other pallets to interact with pallet-critter-nfts
// through well-defined interfaces, promoting modularity and decoupling.

// Implementation of the unified `NftManagement` trait from crittercraft-traits
// This provides a standardized interface for all NFT operations across the ecosystem
use crittercraft_traits::{nft::NftManagement, types::{PetStats, DnaHash}};

impl<T: Config> NftManagement<crittercraft_traits::Config> for Pallet<T> 
where
    T::AccountId: From<<crittercraft_traits::Config as crittercraft_traits::Config>::AccountId> + Into<<crittercraft_traits::Config as crittercraft_traits::Config>::AccountId>,
    PetId: From<<crittercraft_traits::Config as crittercraft_traits::Config>::PetId> + Into<<crittercraft_traits::Config as crittercraft_traits::Config>::PetId>,
{
    /// Get the owner of a pet NFT. Returns `None` if the pet does not exist.
    fn owner_of(pet_id: &<crittercraft_traits::Config as crittercraft_traits::Config>::PetId) -> Option<<crittercraft_traits::Config as crittercraft_traits::Config>::AccountId> {
        let local_pet_id: PetId = (*pet_id).into();
        Self::pet_nft_owner(&local_pet_id).map(|account| account.into())
    }

    /// Transfer a pet NFT from one account to another.
    fn transfer(
        from: &<crittercraft_traits::Config as crittercraft_traits::Config>::AccountId, 
        to: &<crittercraft_traits::Config as crittercraft_traits::Config>::AccountId, 
        pet_id: &<crittercraft_traits::Config as crittercraft_traits::Config>::PetId
    ) -> DispatchResult {
        let local_from: T::AccountId = (*from).into();
        let local_to: T::AccountId = (*to).into();
        let local_pet_id: PetId = (*pet_id).into();
        
        // 1. Verify 'from' is the current owner.
        let current_owner = Self::pet_nft_owner(&local_pet_id).ok_or(Error::<T>::PetNotFound)?;
        ensure!(current_owner == local_from, Error::<T>::NotOwner);

        // 2. Check recipient capacity (important for inter-pallet transfers).
        let recipient_pets_count = OwnerOfPet::<T>::get(&local_to).len();
        ensure!(recipient_pets_count < T::MaxOwnedPets::get() as usize, Error::<T>::RecipientExceedMaxOwnedPets);

        // 3. Mutate ownership records atomically.
        // Remove pet_id from sender's owned list.
        OwnerOfPet::<T>::try_mutate(&local_from, |sender_owned_pets| -> DispatchResult {
            if let Some(index) = sender_owned_pets.iter().position(|id| *id == local_pet_id) {
                sender_owned_pets.swap_remove(index);
                Ok(())
            } else {
                // This indicates an internal inconsistency if owner check passed but pet not in list.
                log::error!(
                    target: "runtime::critter_nfts_pallet",
                    "Inconsistency: Pet {} owned by {:?} but not in OwnerOfPet list for transfer.",
                    local_pet_id,
                    local_from
                );
                Err(Error::<T>::PetNotFound.into()) // Return a consistent error, or panic in debug builds.
            }
        })?;

        // Add pet_id to recipient's owned list.
        OwnerOfPet::<T>::try_mutate(&local_to, |recipient_owned_pets| -> DispatchResult {
            recipient_owned_pets.try_push(local_pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
        })?;

        // 4. Update the direct owner mapping for the pet.
        PetNftOwner::<T>::insert(&local_pet_id, local_to.clone());

        // 5. Emit event for transparency and off-chain indexing.
        Self::deposit_event(Event::PetNftTransferred { 
            from: local_from, 
            to: local_to, 
            pet_id: local_pet_id 
        });
        
        Ok(())
    }

    /// Checks if a pet is "locked" by another pallet and cannot be transferred.
    fn is_locked(pet_id: &<crittercraft_traits::Config as crittercraft_traits::Config>::PetId) -> bool {
        let local_pet_id: PetId = (*pet_id).into();
        LockedNfts::<T>::contains_key(&local_pet_id)
    }

    /// Get the current stats of a specific pet.
    fn pet_stats(pet_id: &<crittercraft_traits::Config as crittercraft_traits::Config>::PetId) -> Option<PetStats> {
        let local_pet_id: PetId = (*pet_id).into();
        Self::pet_nfts(&local_pet_id).map(|pet| {
            PetStats {
                level: pet.level as u16,
                experience: pet.experience_points,
                strength: pet.base_strength,
                agility: pet.base_agility,
                intelligence: pet.base_intelligence,
                charisma: 0, // Not tracked in our current implementation
                stamina: pet.base_vitality, // Using vitality as stamina
            }
        })
    }

    /// Mint a new pet NFT and assign it to an owner.
    fn mint(
        owner: &<crittercraft_traits::Config as crittercraft_traits::Config>::AccountId, 
        dna: [u8; 32], 
        stats: PetStats
    ) -> Result<<crittercraft_traits::Config as crittercraft_traits::Config>::PetId, DispatchResult> {
        let local_owner: T::AccountId = (*owner).into();
        
        // 1. Check maximum owned pets for owner.
        ensure!(
            OwnerOfPet::<T>::get(&local_owner).len() < T::MaxOwnedPets::get() as usize,
            Error::<T>::ExceedMaxOwnedPets
        );

        // 2. Generate PetId.
        let pet_id = NextPetId::<T>::try_mutate(|next_id| -> Result<PetId, DispatchError> {
            let current_id = *next_id;
            *next_id = next_id.checked_add(1).ok_or(Error::<T>::NextPetIdOverflow)?;
            Ok(current_id)
        })?;

        // 3. Create default species and name (can be updated later)
        let default_species: BoundedVec<u8, T::MaxSpeciesNameLen> = 
            b"Default Species".to_vec().try_into().map_err(|_| Error::<T>::SpeciesNameTooLong)?;
        let default_name: BoundedVec<u8, T::MaxPetNameLen> = 
            b"New Pet".to_vec().try_into().map_err(|_| Error::<T>::PetNameTooLong)?;

        // 4. Create the new pet with provided DNA and stats
        let current_block_number = frame_system::Pallet::<T>::block_number();
        let new_pet = PetNft {
            id: pet_id,
            dna_hash: dna,
            initial_species: default_species,
            current_pet_name: default_name,
            base_strength: stats.strength,
            base_agility: stats.agility,
            base_intelligence: stats.intelligence,
            base_vitality: stats.stamina, // Using stamina as vitality
            primary_elemental_affinity: ElementType::Neutral, // Default elemental type
            level: stats.level as u32,
            experience_points: stats.experience,
            mood_indicator: T::MaxMoodValue::get(),
            last_fed_block: current_block_number,
            last_played_block: current_block_number,
            personality_traits: Default::default(), // Start empty
            last_state_update_block: current_block_number,
        };

        // 5. Storage Operations: Insert Pet NFT and update ownership.
        PetNfts::<T>::insert(pet_id, new_pet);
        OwnerOfPet::<T>::try_mutate(&local_owner, |owned_pets_vec| {
            owned_pets_vec.try_push(pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
        })?;
        PetNftOwner::<T>::insert(pet_id, local_owner.clone());

        // 6. Emit event for transparency and off-chain indexing.
        Self::deposit_event(Event::PetNftMinted { owner: local_owner, pet_id });

        Ok(pet_id.into())
    }
}

// Implementation of the `SharedNftManager` trait for `pallet-critter-nfts`.
// This provides core NFT management operations to other pallets like `pallet-marketplace`.
// Maintained for backward compatibility with existing pallets
impl<T: Config> SharedNftManager<T::AccountId, PetId> for Pallet<T> {
    /// Get the owner of an NFT.
    /// Used by other pallets (e.g., marketplace) to verify ownership.
    fn owner_of(pet_id: &PetId) -> Option<T::AccountId> {
        Self::pet_nft_owner(pet_id)
    }

    /// Check if an NFT is transferable (i.e., not locked).
    /// Used by other pallets to verify if a pet can be moved.
    fn is_transferable(pet_id: &PetId) -> bool {
        // Delegates to the internal helper function to avoid code duplication.
        Self::is_transferable(pet_id)
    }

    /// Lock an NFT, preventing transfers (e.g., when listed on marketplace or in battle).
    /// This is crucial for maintaining state synchronization across the ecosystem.
    fn lock_nft(owner: &T::AccountId, pet_id: &PetId) -> DispatchResult {
        // 1. Verify the `owner` is the actual owner of the `pet_id`.
        let current_owner = Self::pet_nft_owner(pet_id).ok_or(Error::<T>::PetNotFound)?;
        ensure!(current_owner == *owner, Error::<T>::NotOwner);

        // 2. Ensure the NFT is not already locked.
        ensure!(!LockedNfts::<T>::contains_key(pet_id), Error::<T>::NftAlreadyLocked);

        // 3. Add the `pet_id` to the `LockedNfts` storage.
        LockedNfts::<T>::insert(pet_id, ());
        Self::deposit_event(Event::NftLocked { owner: owner.clone(), pet_id: *pet_id }); // Emit event
        Ok(())
    }

    /// Unlock an NFT, allowing transfers.
    /// This is called when an NFT is no longer actively participating in an exclusive state.
    fn unlock_nft(owner: &T::AccountId, pet_id: &PetId) -> DispatchResult {
        // 1. Verify the `owner` is the actual owner of the `pet_id`.
        let current_owner = Self::pet_nft_owner(pet_id).ok_or(Error::<T>::PetNotFound)?;
        ensure!(current_owner == *owner, Error::<T>::NotOwner);

        // 2. Ensure the NFT is currently locked.
        ensure!(LockedNfts::<T>::contains_key(pet_id), Error::<T>::NftNotLocked);

        // 3. Remove the `pet_id` from the `LockedNfts` storage.
        LockedNfts::<T>::remove(pet_id);
        Self::deposit_event(Event::NftUnlocked { owner: owner.clone(), pet_id: *pet_id }); // Emit event
        Ok(())
    }

    /// Transfer an NFT from one account to another.
    /// Note: This is a direct transfer, typically called by another pallet (e.g., marketplace after a sale).
    /// It assumes any necessary lock/unlock logic specific to the calling context (like marketplace listing)
    /// has been handled by the caller. This function itself does not check `is_transferable`.
    fn transfer_nft(from: &T::AccountId, to: &T::AccountId, pet_id: &PetId) -> DispatchResult { // DispatchResultType is DispatchResult
        // 1. Verify 'from' is the current owner.
        let current_owner = Self::pet_nft_owner(pet_id).ok_or(Error::<T>::PetNotFound)?;
        ensure!(current_owner == *from, Error::<T>::NotOwner);

        // 2. Check recipient capacity (important for inter-pallet transfers).
        let recipient_pets_count = OwnerOfPet::<T>::get(to).len();
        ensure!(recipient_pets_count < T::MaxOwnedPets::get() as usize, Error::<T>::RecipientExceedMaxOwnedPets);

        // 3. Mutate ownership records atomically.
        // Remove pet_id from sender's owned list.
        OwnerOfPet::<T>::try_mutate(from, |sender_owned_pets| -> DispatchResult {
            if let Some(index) = sender_owned_pets.iter().position(|id| *id == *pet_id) {
                sender_owned_pets.swap_remove(index);
                Ok(())
            } else {
                // This indicates an internal inconsistency if owner check passed but pet not in list.
                log::error!(
                    target: "runtime::critter_nfts_pallet",
                    "Inconsistency: Pet {} owned by {:?} but not in OwnerOfPet list for transfer.",
                    pet_id,
                    from
                );
                Err(Error::<T>::PetNotFound.into()) // Return a consistent error, or panic in debug builds.
            }
        })?;

        // Add pet_id to recipient's owned list.
        OwnerOfPet::<T>::try_mutate(to, |recipient_owned_pets| -> DispatchResult {
            recipient_owned_pets.try_push(*pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
        })?;

        // 4. Update the direct owner mapping for the pet.
        PetNftOwner::<T>::insert(pet_id, to.clone());

        // Note: No event is emitted here by default for inter-pallet transfers via trait.
        // The calling pallet (e.g., marketplace) is responsible for emitting its own relevant event (e.g., NftSold).
        // The user-facing `transfer_pet_nft` extrinsic in this pallet *does* emit `PetNftTransferred`.
        Ok(())
    }
}


// Implementation of NftManagerForItems trait (now defined in `crate::traits`)
// This trait allows `pallet-items` to apply specific effects to Pet NFTs.
impl<T: Config> NftManagerForItems<T::AccountId, PetId, TraitTypeString, BlockNumberFor<T>> for Pallet<T> {
    /// Get the owner of a pet. Called by pallet-items to verify item use permissions.
    fn get_pet_owner_for_item_use(pet_id: &PetId) -> Option<T::AccountId> { // Renamed in trait
        Self::pet_nft_owner(pet_id)
    }

    /// Grant a fixed amount of XP to a pet.
    fn apply_fixed_xp_to_pet( // Renamed in trait
        caller: &T::AccountId,
        pet_id: &PetId,
        amount: u32
    ) -> DispatchResult {
        // 1. Ensure the caller owns the pet
        ensure!(Self::pet_nft_owner(pet_id) == Some(caller.clone()), Error::<T>::NotOwner);

        // 2. Mutate the PetNft to update XP and potentially level.
        PetNfts::<T>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
            let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

            pet.experience_points = pet.experience_points.saturating_add(amount);
            Self::attempt_level_up(pet)?; // Call internal helper to check for level up.

            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            // Note: Event::PetNftMetadataUpdated or a more specific XP event could be emitted here or in attempt_level_up.
            Ok(())
        })
    }

    /// Modify the mood indicator of a pet.
    /// Amount can be positive (increase) or negative (decrease).
    fn apply_mood_modification_to_pet( // Renamed in trait
        caller: &T::AccountId,
        pet_id: &PetId,
        amount: i16 // Positive to increase mood, negative to decrease.
    ) -> DispatchResult {
        ensure!(Self::pet_nft_owner(pet_id) == Some(caller.clone()), Error::<T>::NotOwner);

        PetNfts::<T>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
            let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

            let current_mood = pet.mood_indicator as i16;
            // Add or subtract, then clamp to valid range [0, T::MaxMoodValue].
            let new_mood = current_mood.saturating_add(amount).clamp(0, T::MaxMoodValue::get() as i16) as u8;
            pet.mood_indicator = new_mood;

            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            // Consider emitting PetNftMetadataUpdated or a specific PetMoodChanged event.
            Ok(())
        })
    }

    /// Grant a new personality trait to a pet.
    /// Ensures trait string is bounded and not a duplicate.
    fn apply_personality_trait_to_pet( // Renamed in trait
        caller: &T::AccountId,
        pet_id: &PetId,
        trait_to_grant: TraitTypeString, // The trait string (BoundedVec<u8, MaxTraitStringLen>)
    ) -> DispatchResult {
        ensure!(Self::pet_nft_owner(pet_id) == Some(caller.clone()), Error::<T>::NotOwner);

        PetNfts::<T>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
            let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;

            // Check if pet already has this trait.
            if !pet.personality_traits.iter().any(|existing_trait| existing_trait == &trait_to_grant) {
                // Try to push new trait, mapping potential PushError to TooManyPersonalityTraits.
                pet.personality_traits.try_push(trait_to_grant)
                    .map_err(|_| Error::<T>::TooManyPersonalityTraits)?; // Error if max traits reached.
            } else {
                // Trait already exists, so no change needed. This is an Ok() case.
                log::info!(
                    target: "runtime::critter_nfts_pallet",
                    "Trait already exists for pet {}: {:?}",
                    pet_id,
                    trait_to_grant
                );
            }

            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            // Consider emitting PetNftMetadataUpdated or PetPersonalityTraitAdded event.
            Ok(())
        })
    }

    /// Apply a generic breeding-assist effect to a pet.
    /// This can influence breeding-specific fields or call `pallet-breeding`.
    fn apply_breeding_assist_effect( // Renamed in trait
        caller: &T::AccountId,
        pet_id: &PetId,
        effect_type_id: u8, // Identifier for the type of breeding effect.
        value: u32          // Value associated with the effect.
    ) -> DispatchResult {
        ensure!(Self::pet_nft_owner(pet_id) == Some(caller.clone()), Error::<T>::NotOwner);

        PetNfts::<T>::try_mutate(pet_id, |pet_nft_opt| -> DispatchResult {
            let pet = pet_nft_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            // Actual logic for applying breeding assist effect would be here,
            // potentially interacting with breeding-specific fields on PetNft or calling pallet-breeding.
            // For now, just updating timestamp and logging.
            Ok(())
        })?;

        log::info!(
            target: "runtime::critter_nfts_pallet",
            "Conceptual breeding assist effect (type ID: {}, value: {}) applied to pet ID: {} by owner: {:?}",
            effect_type_id, value, pet_id, caller
        );
        Ok(())
    }
}

// Implementation of NftBreedingHandler trait (now defined in `crate::traits`)
impl<T: Config> NftBreedingHandler<T::AccountId, PetId, DnaHashType, SpeciesType> for Pallet<T> {
    /// Get basic genetic information for a pet, used by `pallet-breeding`.
    fn get_pet_simple_genetics(pet_id: &PetId) -> Option<SimpleGeneticInfo<DnaHashType, SpeciesType>> {
        if let Some(pet_nft) = Self::pet_nfts(pet_id) {
            Some(SimpleGeneticInfo {
                dna_hash: pet_nft.dna_hash,
                species: pet_nft.initial_species.clone(), // Assuming initial_species is BoundedVec
            })
        } else {
            None
        }
    }

    /// Mint a new pet from a breeding outcome, called by `pallet-breeding`.
    /// This method ensures secure, synchronized creation of new pets based on derived DNA.
    fn mint_pet_from_breeding(
        owner: &T::AccountId,
        species: SpeciesType, // BoundedVec from Breeding pallet
        dna_hash: DnaHashType, // [u8; 32] from Breeding pallet
        parent1_id: PetId,
        parent2_id: PetId,
        initial_name: BoundedVec<u8, T::MaxPetNameLen>, // BoundedVec for name
    ) -> Result<PetId, DispatchResult> {
        // This logic is similar to `mint_pet_nft` but uses provided DNA and species.

        // 1. Check maximum owned pets for owner.
        ensure!(
            OwnerOfPet::<T>::get(owner).len() < T::MaxOwnedPets::get() as usize,
            Error::<T>::ExceedMaxOwnedPets
        );

        // 2. Generate PetId.
        let pet_id = NextPetId::<T>::try_mutate(|next_id| -> Result<PetId, DispatchError> {
            let current_id = *next_id;
            *next_id = next_id.checked_add(1).ok_or(Error::<T>::NextPetIdOverflow)?;
            Ok(current_id)
        })?;

        // 3. Derive Charter Attributes from the given dna_hash (same deterministic logic as in `mint_pet_nft`).
        let base_strength = (dna_hash[0] % 16) + 5; // 5-20
        let base_agility = (dna_hash[1] % 16) + 5;
        let base_intelligence = (dna_hash[2] % 16) + 5;
        let base_vitality = (dna_hash[3] % 16) + 5;
        let primary_elemental_affinity = match dna_hash[4] % 8 {
            0 => ElementType::Fire, 1 => ElementType::Water, 2 => ElementType::Earth,
            3 => ElementType::Air, 4 => ElementType::Tech, 5 => ElementType::Nature,
            6 => ElementType::Mystic,
            _ => ElementType::Neutral,
        };

        // 4. Initial Dynamic Attributes (set to defaults).
        let current_block_number = frame_system::Pallet::<T>::block_number();
        let new_pet = PetNft {
            id: pet_id,
            dna_hash, // Use the provided dna_hash
            initial_species: species.clone(), // Use provided species
            current_pet_name: initial_name, // Use provided initial name
            base_strength,
            base_agility,
            base_intelligence,
            base_vitality,
            primary_elemental_affinity,
            level: 1,
            experience_points: 0,
            mood_indicator: T::MaxMoodValue::get(),
            last_fed_block: current_block_number,
            last_played_block: current_block_number,
            personality_traits: Default::default(), // Start empty for newly bred pets
            last_state_update_block: current_block_number,
            // TODO: Add parent1_id, parent2_id fields to PetNft struct and set them here (V2+ enhancement)
        };

        // 5. Storage Operations: Insert Pet NFT and update ownership.
        PetNfts::<T>::insert(pet_id, new_pet);
        OwnerOfPet::<T>::try_mutate(owner, |owned_pets_vec| {
            owned_pets_vec.try_push(pet_id).map_err(|_| Error::<T>::ExceedMaxOwnedPets)
        })?;
        PetNftOwner::<T>::insert(pet_id, owner.clone());

        // 6. Emit event.
        Self::deposit_event(Event::PetNftMinted { owner: owner.clone(), pet_id });
        Ok(pet_id) // Return the ID of the newly minted pet
    }
}

// Implementation of QuestNftRequirementChecker trait (now defined in `crate::traits`)
impl<T: Config> QuestNftRequirementChecker<T::AccountId, PetId, SpeciesType> for Pallet<T> {
    /// Get the owner of a pet. Used by `pallet-quests` to verify pet eligibility for quests.
    fn get_pet_owner_for_quest(pet_id: &PetId) -> Option<T::AccountId> {
        Self::pet_nft_owner(pet_id)
    }

    /// Get the level of a pet. Used by `pallet-quests` to verify level requirements.
    fn get_pet_level_for_quest(pet_id: &PetId) -> Option<u32> {
        Self::pet_nfts(pet_id).map(|pet| pet.level)
    }

    /// Get the species of a pet. (Deferred in trait for MVP).
    /// This method would provide the pet's species for quest requirements.
    fn get_pet_species_for_quest(pet_id: &PetId) -> Option<SpeciesType> {
        Self::pet_nfts(pet_id).map(|pet| pet.initial_species.clone())
    }
