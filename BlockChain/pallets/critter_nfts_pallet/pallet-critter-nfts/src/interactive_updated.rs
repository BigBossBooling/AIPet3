//! # Interactive Elements System (Updated)
//!
//! This module provides an enhanced system for managing interactive elements in the UI,
//! including gestures, touch responses, and dynamic UI elements that respond
//! to user input and pet state changes. This updated version includes improved security,
//! optimizations, and anti-abuse measures.
//!
//! ## Features
//! 
//! - **Gesture Recognition**: Advanced pattern recognition for user gestures
//! - **Touch Interactions**: Responsive touch-based interactions with pets
//! - **Dynamic UI Elements**: UI components that adapt to pet state
//! - **Session Management**: Tracking and managing interactive sessions
//! - **Reward System**: Balanced reward distribution with anti-abuse measures
//! - **Security**: Validation and rate limiting to prevent exploitation
//! - **Performance Optimizations**: Efficient data structures and algorithms
//!
//! ## Module Organization
//!
//! The module is organized into several logical sections:
//! - Core data structures and enums
//! - Gesture recognition and processing
//! - Touch interaction handling
//! - Dynamic UI element management
//! - Interactive session tracking and rewards
//! - Pattern matching algorithms
//! - Security and anti-abuse systems
//!
//! ## Implementation Notes
//!
//! - All vector types use BoundedVec to prevent DoS attacks
//! - Helper methods validate data integrity before processing
//! - Functional programming patterns are used for cleaner code
//! - Constants are used for configuration to ensure consistency

// ============================================================================
// Constants for the interactive session system
// ============================================================================

/// Session duration limits
pub const MAX_SESSION_DURATION: u64 = 86400; // 24 hours in seconds

/// Interaction limits
pub const MAX_INTERACTIONS_PER_SESSION: u32 = 100; // Maximum number of interactions per session
pub const MAX_MOOD_CHANGES_PER_SESSION: u32 = 50; // Maximum number of mood changes per session
pub const MAX_MOOD_CHANGE_PER_INTERACTION: i8 = 10; // Maximum mood change per interaction

/// Reward limits
pub const MAX_EXPERIENCE_PER_SESSION: u32 = 500; // Maximum experience points per session
pub const MAX_REWARDS_PER_SESSION: u32 = 8; // Maximum number of rewards per session

/// Rate limiting
pub const MAX_SESSIONS_PER_USER: u32 = 5; // Maximum number of active sessions per user
pub const SESSION_RATE_LIMIT_BLOCKS: u32 = 300; // 5 minutes in blocks (assuming 1 block per second)

use frame_support::{
    dispatch::{DispatchResult, DispatchError},
    ensure,
    pallet_prelude::*,
    BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_std::{vec::Vec, prelude::*};
use sp_runtime::{
    traits::{SaturatedConversion, Saturating},
    ArithmeticError,
};
use scale_info::TypeInfo;
use codec::{Encode, Decode, MaxEncodedLen};
use crate::{Config, Error, PetId, PetNft};

/// Standardized bounds for various vector types
pub mod bounds {
    use super::*;
    
    /// Maximum length for name strings
    pub const MAX_NAME_LENGTH: u32 = 32;
    
    /// Maximum length for description strings
    pub const MAX_DESCRIPTION_LENGTH: u32 = 128;
    
    /// Maximum length for icon strings
    pub const MAX_ICON_LENGTH: u32 = 64;
    
    /// Maximum number of effects
    pub const MAX_EFFECTS: u32 = 8;
    
    /// Maximum number of requirements
    pub const MAX_REQUIREMENTS: u32 = 4;
    
    /// Type alias for name vectors
    pub type NameVec = BoundedVec<u8, ConstU32<MAX_NAME_LENGTH>>;
    
    /// Type alias for description vectors
    pub type DescriptionVec = BoundedVec<u8, ConstU32<MAX_DESCRIPTION_LENGTH>>;
    
    /// Type alias for icon vectors
    pub type IconVec = BoundedVec<u8, ConstU32<MAX_ICON_LENGTH>>;
    
    /// Type alias for effects vectors
    pub type EffectsVec<T> = BoundedVec<T, ConstU32<MAX_EFFECTS>>;
    
    /// Type alias for requirements vectors
    pub type RequirementsVec<T> = BoundedVec<T, ConstU32<MAX_REQUIREMENTS>>;
}

// ============================================================================
// Core Data Structures and Enums
// ============================================================================

/// Represents a gesture effect with its type and magnitude
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct GestureEffect {
    /// The effect type (0 = mood, 1 = experience, etc.)
    pub effect_type: u8,
    
    /// The effect magnitude (positive or negative)
    pub magnitude: i8,
}

/// Represents a gesture requirement
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct GestureRequirement {
    /// The requirement type (0 = mood, 1 = level, etc.)
    pub requirement_type: u8,
    
    /// The required value
    pub value: u8,
}

/// Represents an interactive gesture that users can perform.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct InteractiveGesture {
    /// The gesture ID
    pub gesture_id: u8,
    
    /// The gesture name
    pub name: BoundedVec<u8, ConstU32<32>>,
    
    /// The gesture description
    pub description: BoundedVec<u8, ConstU32<128>>,
    
    /// The gesture icon
    pub icon: BoundedVec<u8, ConstU32<64>>,
    
    /// The gesture effects
    pub effects: BoundedVec<GestureEffect, ConstU32<8>>,
    
    /// The gesture requirements
    pub requirements: BoundedVec<GestureRequirement, ConstU32<4>>,
}

/// Represents a touch response for a pet.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TouchResponse {
    /// The response ID
    pub response_id: u8,
    
    /// The touch area
    pub touch_area: u8,
    
    /// The pet mood requirement
    pub mood_requirement: u8,
    
    /// The response animation
    pub animation: u8,
    
    /// The response sound
    pub sound: u8,
    
    /// The mood effect
    pub mood_effect: i8,
}

/// Represents a state condition for a UI element
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct StateCondition {
    /// The condition type (0 = mood, 1 = health, etc.)
    pub condition_type: u8,
    
    /// The comparison operator (0 = equal, 1 = not equal, etc.)
    pub comparison_operator: u8,
    
    /// The comparison value
    pub value: u8,
}

/// Represents a visual property for a UI element
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct VisualProperty {
    /// The property type (0 = color, 1 = size, etc.)
    pub property_type: u8,
    
    /// The property value
    pub value: BoundedVec<u8, ConstU32<64>>,
}

/// Represents a dynamic UI element that responds to pet state.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DynamicUiElement {
    /// The element ID
    pub element_id: u8,
    
    /// The element name
    pub name: BoundedVec<u8, ConstU32<32>>,
    
    /// The element description
    pub description: BoundedVec<u8, ConstU32<128>>,
    
    /// The element type
    pub element_type: u8,
    
    /// The element state conditions
    pub state_conditions: BoundedVec<StateCondition, ConstU32<8>>,
    
    /// The element visual properties
    pub visual_properties: BoundedVec<VisualProperty, ConstU32<8>>,
}

// ============================================================================
// Enums
// ============================================================================

/// Gesture types.
pub enum GestureType {
    Pet = 0,
    Tickle = 1,
    Stroke = 2,
    Tap = 3,
    Swipe = 4,
    Pinch = 5,
    Shake = 6,
    Hold = 7,
    CircularMotion = 8,
    DoubleTap = 9,
    TripleTap = 10,
    LongPress = 11,
    QuickSwipe = 12,
    ZigZag = 13,
    DrawHeart = 14,
    DrawStar = 15,
    DrawCircle = 16,
    DrawSquare = 17,
    TwoFingerTap = 18,
    ThreeFingerTap = 19,
    FourFingerTap = 20,
    SpreadFingers = 21,
    PinchIn = 22,
    PinchOut = 23,
    Rotate = 24,
    Flick = 25,
    Toss = 26,
    Bounce = 27,
    Spin = 28,
    Wave = 29,
    Clap = 30,
    Snap = 31,
}

/// Touch areas.
pub enum TouchArea {
    Head = 0,
    Back = 1,
    Belly = 2,
    Paws = 3,
    Tail = 4,
    Face = 5,
    Ears = 6,
    Wings = 7,
}

/// UI element types.
pub enum UiElementType {
    MoodIndicator = 0,
    HealthBar = 1,
    ExperienceBar = 2,
    ActionButton = 3,
    StatusIcon = 4,
    BackgroundElement = 5,
    ForegroundEffect = 6,
    InteractiveObject = 7,
}

/// Condition types.
pub enum ConditionType {
    Mood = 0,
    Health = 1,
    Experience = 2,
    Level = 3,
    ElementalAffinity = 4,
    EnvironmentType = 5,
    TimeOfDay = 6,
    SeasonalEvent = 7,
}

/// Comparison operators.
pub enum ComparisonOperator {
    Equal = 0,
    NotEqual = 1,
    GreaterThan = 2,
    LessThan = 3,
    GreaterThanOrEqual = 4,
    LessThanOrEqual = 5,
    Between = 6,
    NotBetween = 7,
}

/// Represents a touch point with coordinates and timestamp
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TouchPoint {
    /// X coordinate
    pub x: i16,
    
    /// Y coordinate
    pub y: i16,
    
    /// Timestamp
    pub time: u32,
}

/// Represents a recognition parameter
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RecognitionParameter {
    /// Parameter type (0 = threshold, 1 = tolerance, etc.)
    pub param_type: u8,
    
    /// Parameter value
    pub value: u8,
}

/// Represents a gesture pattern for recognition.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct GesturePattern {
    /// The pattern ID
    pub pattern_id: u8,
    
    /// The gesture type
    pub gesture_type: u8,
    
    /// The pattern name
    pub name: BoundedVec<u8, ConstU32<32>>,
    
    /// The pattern description
    pub description: BoundedVec<u8, ConstU32<128>>,
    
    /// The pattern points (x, y, time)
    pub points: BoundedVec<TouchPoint, ConstU32<32>>,
    
    /// The pattern recognition parameters
    pub recognition_params: BoundedVec<RecognitionParameter, ConstU32<8>>,
}

/// Represents a gesture parameter
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct GestureParameter {
    /// Parameter type (0 = intensity, 1 = speed, 2 = area, etc.)
    pub param_type: u8,
    
    /// Parameter value
    pub value: u16,
}

/// Represents a gesture recognition result.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct GestureRecognitionResult {
    /// The recognized gesture type
    pub gesture_type: u8,
    
    /// The confidence level (0-100)
    pub confidence: u8,
    
    /// The recognition time in milliseconds
    pub recognition_time: u32,
    
    /// The gesture parameters
    pub parameters: BoundedVec<GestureParameter, ConstU32<8>>,
}

/// Represents an interaction in a session
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SessionInteraction {
    /// Interaction type (0 = pet, 1 = feed, 2 = play, etc.)
    pub interaction_type: u8,
    
    /// Timestamp when the interaction occurred
    pub timestamp: u64,
    
    /// Outcome of the interaction (0 = neutral, 1 = positive, 2 = negative, etc.)
    pub outcome: u8,
}

/// Represents a mood change in a session
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct MoodChange {
    /// The change in mood (positive or negative)
    pub change: i8,
    
    /// Timestamp when the mood change occurred
    pub timestamp: u64,
}

/// Reward types
pub enum RewardType {
    /// Experience points reward
    Experience = 0,
    /// Item reward
    Item = 1,
    /// Currency reward
    Currency = 2,
    /// Consistency bonus reward
    ConsistencyBonus = 3,
    /// Achievement reward
    Achievement = 4,
    /// Special event reward
    SpecialEvent = 5,
    /// Loyalty reward
    Loyalty = 6,
    /// Milestone reward
    Milestone = 7,
}

/// Represents a reward earned in a session
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SessionReward {
    /// Reward type (see RewardType enum)
    pub reward_type: u8,
    
    /// Amount of the reward
    pub amount: u32,
    
    /// Checks if this is a significant reward
    pub fn is_significant(&self) -> bool {
        self.amount >= 50
    }
}

/// Represents an interactive session with a pet.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct InteractiveSession {
    /// The session ID
    pub session_id: u32,
    
    /// The pet ID
    pub pet_id: PetId,
    
    /// The session start time
    pub start_time: u64,
    
    /// The session duration in seconds
    pub duration: u32,
    
    /// The interactions performed
    pub interactions: BoundedVec<SessionInteraction, ConstU32<MAX_INTERACTIONS_PER_SESSION>>,
    
    /// The mood changes
    pub mood_changes: BoundedVec<MoodChange, ConstU32<MAX_MOOD_CHANGES_PER_SESSION>>,
    
    /// The rewards earned
    pub rewards: BoundedVec<SessionReward, ConstU32<MAX_REWARDS_PER_SESSION>>,
    
    /// Calculates the total reward amount
    pub fn total_reward(&self) -> u32 {
        self.rewards.iter()
            .map(|r| r.amount)
            .sum::<u32>()
            .min(MAX_EXPERIENCE_PER_SESSION)
    }
    
    /// Checks if the session is active
    pub fn is_active(&self, current_time: u64) -> bool {
        self.duration == 0 && 
        current_time - self.start_time <= MAX_SESSION_DURATION
    }
}

/// Maximum pressure value
pub const MAX_PRESSURE: u8 = 255;

/// Represents a touch point with pressure
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PressureTouchPoint {
    /// X coordinate (0 to MAX_SCREEN_WIDTH)
    pub x: i16,
    
    /// Y coordinate (0 to MAX_SCREEN_HEIGHT)
    pub y: i16,
    
    /// Pressure (0-255)
    pub pressure: u8,
    
    /// Validates that the touch point is within screen bounds and has valid pressure
    pub fn is_valid(&self) -> bool {
        self.x >= coordinates::MIN_COORDINATE && 
        self.x <= coordinates::MAX_SCREEN_WIDTH &&
        self.y >= coordinates::MIN_COORDINATE && 
        self.y <= coordinates::MAX_SCREEN_HEIGHT &&
        self.pressure <= MAX_PRESSURE
    }
}

/// Maximum number of touch points in a multi-touch interaction
pub const MAX_TOUCH_POINTS: u32 = 10;

/// Multi-touch interaction types
pub enum MultiTouchType {
    /// Pinch interaction
    Pinch = 0,
    /// Spread interaction
    Spread = 1,
    /// Rotate interaction
    Rotate = 2,
    /// Swipe interaction
    Swipe = 3,
    /// Tap interaction
    Tap = 4,
    /// Hold interaction
    Hold = 5,
    /// Drag interaction
    Drag = 6,
    /// Custom interaction
    Custom = 7,
}

/// Represents a multi-touch interaction.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct MultiTouchInteraction {
    /// The interaction ID
    pub interaction_id: u8,
    
    /// The number of touch points
    pub touch_count: u8,
    
    /// The touch points with pressure
    pub touch_points: BoundedVec<PressureTouchPoint, ConstU32<MAX_TOUCH_POINTS>>,
    
    /// The interaction type (see MultiTouchType enum)
    pub interaction_type: u8,
    
    /// The interaction parameters
    pub parameters: BoundedVec<GestureParameter, ConstU32<MAX_GESTURE_PARAMS>>,
    
    /// Validates the multi-touch interaction
    pub fn is_valid(&self) -> bool {
        // Check if all touch points are valid
        self.touch_points.iter().all(|point| point.is_valid()) &&
        // Check if touch_count matches the actual number of touch points
        self.touch_count as usize == self.touch_points.len() &&
        // Check if we have at least one touch point
        !self.touch_points.is_empty() &&
        // Check if the interaction type is valid
        self.interaction_type <= 7
    }
}

// ============================================================================
// Main Interactive System
// ============================================================================

/// Error types for the interactive system
pub enum InteractiveError {
    /// Pet not found
    PetNotFound,
    /// Pet is locked
    PetLocked,
    /// Session not found
    SessionNotFound,
    /// Too many sessions
    TooManySessions,
    /// Invalid session data
    InvalidSessionData,
    /// Rate limited
    RateLimited,
    /// Invalid interaction
    InvalidInteraction,
    /// Unauthorized
    Unauthorized,
    /// Storage error
    StorageError,
}

/// Result type for interactive operations
pub type InteractiveResult<T> = Result<T, InteractiveError>;

/// A system for managing interactive elements in the UI.
/// This is the main entry point for all interactive functionality.
pub struct InteractiveSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

/// A system for managing interactive sessions.
pub struct InteractiveSessionSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

// Using constants defined at the top of the file

/// Maximum number of recent sessions allowed per user
pub const MAX_RECENT_SESSIONS: usize = 3;

impl<T: Config> InteractiveSessionSystem<T> {
    /// Starts an interactive session with a pet.
    /// 
    /// # Parameters
    /// 
    /// * `origin` - The origin of the call
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<InteractiveSession, DispatchError>` - The session, or an error
    
    /// Validates that the user owns the pet and the pet is not locked
    fn validate_pet_ownership(origin: &T::AccountId, pet_id: PetId) -> DispatchResult {
        // Ensure the pet exists and is not locked
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        ensure!(!pet.is_locked, Error::<T>::PetIsLocked);
        
        // Ensure the caller is the owner of the pet
        let pet_owner = crate::OwnerOf::<T>::get(pet_id).ok_or(Error::<T>::NotOwner)?;
        ensure!(pet_owner == *origin, Error::<T>::NotOwner);
        
        Ok(())
    }
    
    /// Checks if the user has hit the rate limit for starting sessions
    fn check_rate_limit(origin: &T::AccountId, block_number: T::BlockNumber) -> DispatchResult {
        let user_sessions = crate::UserSessions::<T>::get(origin);
        let recent_sessions = user_sessions.iter()
            .filter(|&(_, start_block)| {
                block_number.saturating_sub(*start_block) < T::BlockNumber::from(SESSION_RATE_LIMIT_BLOCKS)
            })
            .count();
        
        ensure!(recent_sessions < MAX_RECENT_SESSIONS, Error::<T>::TooManySessions);
        Ok(())
    }
    
    /// Generates a unique session ID
    fn generate_session_id(block_number: T::BlockNumber, origin: &T::AccountId, pet_id: PetId) -> u32 {
        let nonce = crate::Nonce::<T>::get();
        let session_id_data = (block_number, origin, pet_id, nonce).encode();
        let session_id_hash = sp_io::hashing::blake2_256(&session_id_data);
        
        // Increment the nonce for next time
        crate::Nonce::<T>::mutate(|n| *n = n.wrapping_add(1));
        
        // Use first 4 bytes of hash as session ID
        u32::from_be_bytes([
            session_id_hash[0], session_id_hash[1], session_id_hash[2], session_id_hash[3]
        ])
    }
    
    /// Updates the user's session tracking
    fn update_user_sessions(origin: &T::AccountId, session_id: u32, block_number: T::BlockNumber) {
        crate::UserSessions::<T>::mutate(origin, |sessions| {
            sessions.push((session_id, block_number));
            
            // Keep only the most recent 10 sessions
            if sessions.len() > 10 {
                sessions.sort_by_key(|&(_, block)| block);
                sessions.reverse();
                sessions.truncate(10);
            }
        });
    }
    
    /// Updates the pet's state after starting a session
    fn update_pet_state(pet_id: PetId, block_number: T::BlockNumber) -> DispatchResult {
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            pet.last_interaction_time = block_number;
            pet.state_version = pet.state_version.saturating_add(1);
            Ok(())
        })
    }
    
    pub fn start_interactive_session(origin: T::AccountId, pet_id: PetId) -> Result<InteractiveSession, DispatchError> {
        // Validate pet ownership in a single operation
        Self::validate_pet_ownership(&origin, pet_id)?;
        
        // Get the current block number
        let block_number = frame_system::Pallet::<T>::block_number();
        
        // Rate limiting: Check if the user has started too many sessions recently
        Self::check_rate_limit(&origin, block_number)?;
        
        // Generate a unique session ID
        let session_id = Self::generate_session_id(block_number, &origin, pet_id);
        
        // Ensure the session ID doesn't already exist
        ensure!(!crate::InteractiveSessions::<T>::contains_key(session_id), Error::<T>::SessionAlreadyExists);
        
        // Get the current timestamp
        let start_time = block_number.saturated_into::<u64>();
        
        // Create a new session
        let session = InteractiveSession {
            session_id,
            pet_id,
            start_time,
            duration: 0, // Will be updated when the session ends
            interactions: BoundedVec::default(),
            mood_changes: BoundedVec::default(),
            rewards: BoundedVec::default(),
        };
        
        // Store the session
        crate::InteractiveSessions::<T>::insert(session_id, session.clone());
        
        // Update user sessions tracking
        Self::update_user_sessions(&origin, session_id, block_number);
        
        // Update pet's state
        Self::update_pet_state(pet_id, block_number)?;
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::InteractiveSessionStarted {
            account_id: origin,
            pet_id,
            session_id,
            timestamp: block_number,
        });
        
        Ok(session)
    }
    
    /// Ends an interactive session with a pet.
    /// 
    /// # Parameters
    /// 
    /// * `origin` - The origin of the call
    /// * `session_id` - The ID of the session
    /// 
    /// # Returns
    /// 
    /// * `Result<InteractiveSession, DispatchError>` - The updated session, or an error
    
    /// Validates a session before ending it
    fn validate_session_for_ending(
        origin: &T::AccountId, 
        session: &InteractiveSession,
        block_number: T::BlockNumber
    ) -> DispatchResult {
        // Check if the session has already been ended
        ensure!(session.duration == 0, Error::<T>::SessionAlreadyEnded);
        
        // Ensure the caller is the owner of the pet
        let pet_owner = crate::OwnerOf::<T>::get(session.pet_id).ok_or(Error::<T>::NotOwner)?;
        ensure!(pet_owner == *origin, Error::<T>::NotOwner);
        
        // Get the current timestamp
        let end_time = block_number.saturated_into::<u64>();
        
        // Check if the session has expired
        ensure!(
            end_time - session.start_time <= MAX_SESSION_DURATION,
            Error::<T>::SessionExpired
        );
        
        Ok(())
    }
    
    /// Calculates the duration of a session
    fn calculate_session_duration(session: &mut InteractiveSession, block_number: T::BlockNumber) {
        let end_time = block_number.saturated_into::<u64>();
        session.duration = (end_time - session.start_time) as u32;
    }
    
    pub fn end_interactive_session(origin: T::AccountId, session_id: u32) -> Result<InteractiveSession, DispatchError> {
        // Get the current block number
        let block_number = frame_system::Pallet::<T>::block_number();
        
        // Get the session and perform all validations in a single storage operation
        let session = crate::InteractiveSessions::<T>::try_mutate_exists(
            session_id, 
            |session_opt| -> Result<InteractiveSession, DispatchError> {
                // Get the session
                let mut session = session_opt.take().ok_or(Error::<T>::SessionNotFound)?;
                
                // Validate the session
                Self::validate_session_for_ending(&origin, &session, block_number)?;
                
                // Calculate session duration
                Self::calculate_session_duration(&mut session, block_number);
                
                // Validate session data for security
                Self::validate_session_data(&session)?;
                
                // Calculate rewards based on the session with anti-abuse measures
                let mut rewards = Vec::new();
                
                // Reward based on duration (capped at 60 minutes to prevent abuse)
                let capped_duration = session.duration.min(3600); // Cap at 1 hour (3600 seconds)
                let duration_reward = capped_duration / 60; // 1 point per minute
                
                let duration_reward_obj = SessionReward {
                    reward_type: 0, // Duration reward
                    amount: duration_reward,
                };
                rewards.push(duration_reward_obj);
        
        // Reward based on interactions (capped to prevent abuse)
        let interaction_count = session.interactions.len().min(MAX_INTERACTIONS_PER_SESSION as usize) as u32;
        
        // Apply diminishing returns for interactions
        let interaction_reward = if interaction_count <= 10 {
            interaction_count * 2 // 2 points per interaction for first 10
        } else {
            20 + ((interaction_count - 10) * 1) // 1 point per interaction after first 10
        };
        
        let interaction_reward_obj = SessionReward {
            reward_type: 1, // Interaction reward
            amount: interaction_reward,
        };
        rewards.push(interaction_reward_obj);
        
        // Reward based on mood changes with anti-gaming measures - optimized algorithm
        let mut total_mood_change = 0;
        let mut last_change_time = 0;
        let mut rapid_changes = 0;
        
        // Process mood changes chronologically - use fold for more functional approach
        if !session.mood_changes.is_empty() {
            // Sort mood changes by timestamp for chronological analysis
            let mut sorted_changes = session.mood_changes.to_vec();
            sorted_changes.sort_by_key(|change| change.timestamp);
            
            // Process changes with a fold operation for better readability
            let (_, _, total) = sorted_changes.iter().fold(
                (0u64, 0u32, 0i8), // (last_time, rapid_count, total)
                |(last_time, rapid_count, total), change| {
                    // Detect rapid changes
                    let new_rapid_count = if last_time > 0 && change.timestamp - last_time < 3 {
                        rapid_count + 1
                    } else {
                        rapid_count
                    };
                    
                    // Apply diminishing returns based on rapid count
                    let change_value = match new_rapid_count {
                        0..=3 => change.change,
                        4..=10 => change.change / 2, // 50% effectiveness
                        _ => change.change / 4,      // 25% effectiveness
                    };
                    
                    (change.timestamp, new_rapid_count, total + change_value)
                }
            );
            
            total_mood_change = total;
            
            // Extract rapid_changes for later use in consistency bonus
            rapid_changes = sorted_changes.windows(2)
                .filter(|pair| pair[1].timestamp - pair[0].timestamp < 3)
                .count() as u32;
        }
        
        // Cap mood reward to prevent abuse
        let capped_mood_change = total_mood_change.max(-50).min(50);
        let mood_reward = if capped_mood_change > 0 {
            capped_mood_change as u32 * 3 // 3 points per positive mood change
        } else {
            0
        };
        
        let mood_reward_obj = SessionReward {
            reward_type: 2, // Mood reward
            amount: mood_reward,
        };
        rewards.push(mood_reward_obj);
        
        // Add bonus reward for consistent interaction patterns
        if session.interactions.len() >= 5 && rapid_changes <= 2 && session.duration >= 300 {
            let consistency_bonus = SessionReward {
                reward_type: 3, // Consistency bonus
                amount: 15,
            };
            rewards.push(consistency_bonus);
        }
        
        // Update the session rewards
        session.rewards = rewards.try_into().map_err(|_| Error::<T>::TooManySessionRewards)?;
        
        // Calculate total reward with anti-abuse cap
        let total_reward: u32 = session.rewards.iter()
            .map(|r| r.amount)
            .sum::<u32>()
            .min(MAX_EXPERIENCE_PER_SESSION);
        
        // Update the pet's experience points and stats
        crate::PetNfts::<T>::try_mutate(session.pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Add experience points
            let old_xp = pet.experience_points;
            pet.experience_points = pet.experience_points.saturating_add(total_reward);
            
            // Check if the pet has leveled up
            let old_level = pet.level;
            let new_level = (pet.experience_points / 100).saturating_add(1); // Simple level formula: XP/100 + 1
            
            if new_level > old_level {
                // Apply level-up bonuses
                pet.level = new_level;
                
                // Increase base stats based on level gain
                let level_diff = new_level - old_level;
                
                // Apply stat increases based on pet's primary elemental affinity - optimized approach
                // First apply base increases to all stats
                let base_increase = level_diff as u8;
                pet.base_strength = pet.base_strength.saturating_add(base_increase);
                pet.base_agility = pet.base_agility.saturating_add(base_increase);
                pet.base_intelligence = pet.base_intelligence.saturating_add(base_increase);
                pet.base_vitality = pet.base_vitality.saturating_add(base_increase);
                
                // Then apply bonus increase based on affinity
                match pet.primary_elemental_affinity {
                    0 => pet.base_strength = pet.base_strength.saturating_add(base_increase),     // Fire - strength
                    1 => pet.base_intelligence = pet.base_intelligence.saturating_add(base_increase), // Water - intelligence
                    2 => pet.base_vitality = pet.base_vitality.saturating_add(base_increase),     // Earth - vitality
                    3 => pet.base_agility = pet.base_agility.saturating_add(base_increase),       // Air - agility
                    _ => {} // Balanced - already applied base increases
                }
                
                // Emit level up event
                crate::Pallet::<T>::deposit_event(crate::Event::PetLeveledUp {
                    pet_id: session.pet_id,
                    old_level,
                    new_level,
                    experience_points: pet.experience_points,
                    timestamp: block_number,
                });
                
                // Check for achievements after leveling up - don't block on errors
                let _ = crate::achievements::AchievementSystem::<T>::check_achievements(session.pet_id)
                    .map_err(|e| log::warn!("Failed to check achievements: {:?}", e));
            }
            
            // Update pet's last interaction time
            pet.last_interaction_time = block_number;
            
            // Increment the state version for synchronization
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Emit pet state synchronized event
            crate::Pallet::<T>::deposit_event(crate::Event::PetStateSynchronized {
                pet_id: session.pet_id,
                version: pet.state_version,
                timestamp: block_number,
                change_type: 2, // Interactive session
                successful_hooks: 0,
                failed_hooks: 0,
            });
            
            Ok(())
        })?;
        
        // Update the session in storage
        crate::InteractiveSessions::<T>::insert(session_id, session.clone());
        
        // Clean up old sessions if needed
        Self::prune_old_sessions(&origin);
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::InteractiveSessionEnded {
            account_id: origin.clone(),
            pet_id: session.pet_id,
            session_id,
            duration: session.duration,
            timestamp: block_number,
        });
        
        // Emit reward events
        for reward in session.rewards.iter() {
            crate::Pallet::<T>::deposit_event(crate::Event::SessionRewardEarned {
                session_id,
                reward_type: reward.reward_type,
                amount: reward.amount,
                timestamp: block_number,
            });
        }
        
        Ok(session)
    }
    
    /// Validates session data for security and consistency
    /// Validates that a timestamp is within the session's time range
    fn is_timestamp_valid(session: &InteractiveSession, timestamp: u64) -> bool {
        timestamp >= session.start_time && 
        timestamp <= session.start_time + session.duration as u64
    }
    
    /// Validates that a mood change is within allowed limits
    fn is_mood_change_valid(change: i8) -> bool {
        change >= -MAX_MOOD_CHANGE_PER_INTERACTION && 
        change <= MAX_MOOD_CHANGE_PER_INTERACTION
    }
    
    /// Validates session data for security and consistency
    fn validate_session_data(session: &InteractiveSession) -> Result<(), DispatchError> {
        // Validate session duration
        ensure!(
            session.duration <= MAX_SESSION_DURATION as u32, 
            Error::<T>::InvalidSessionDuration
        );
        
        // Validate interaction count
        ensure!(
            session.interactions.len() <= MAX_INTERACTIONS_PER_SESSION as usize,
            Error::<T>::TooManySessionInteractions
        );
        
        // Validate mood change count
        ensure!(
            session.mood_changes.len() <= MAX_MOOD_CHANGES_PER_SESSION as usize,
            Error::<T>::TooManySessionMoodChanges
        );
        
        // Validate interaction timestamps
        let all_interactions_valid = session.interactions.iter()
            .all(|interaction| Self::is_timestamp_valid(session, interaction.timestamp));
            
        ensure!(all_interactions_valid, Error::<T>::InvalidInteractionTimestamp);
        
        // Validate mood changes
        let all_mood_changes_valid = session.mood_changes.iter()
            .all(|mood_change| 
                Self::is_timestamp_valid(session, mood_change.timestamp) && 
                Self::is_mood_change_valid(mood_change.change)
            );
            
        ensure!(all_mood_changes_valid, Error::<T>::InvalidMoodChangeTimestamp);
        
        Ok(())
    }
    
    /// Number of days to keep session data
    pub const SESSION_RETENTION_DAYS: u32 = 30;
    
    /// Number of seconds in a day
    pub const SECONDS_PER_DAY: u32 = 86400; // 24 * 60 * 60
    
    /// Prunes old sessions to keep storage optimized
    fn prune_old_sessions(account: &T::AccountId) {
        let current_block = frame_system::Pallet::<T>::block_number();
        let retention_period = T::BlockNumber::from(SESSION_RETENTION_DAYS * SECONDS_PER_DAY);
        
        // Get and update user sessions in a single storage operation
        crate::UserSessions::<T>::mutate(account, |user_sessions| {
            // Partition user sessions into old and current
            let (old_sessions, current_sessions): (Vec<_>, Vec<_>) = user_sessions
                .iter()
                .partition(|&(_, block)| current_block.saturating_sub(*block) > retention_period);
            
            // Remove old sessions from storage
            for &(session_id, _) in &old_sessions {
                crate::InteractiveSessions::<T>::remove(session_id);
                
                // Emit event for session cleanup
                crate::Pallet::<T>::deposit_event(crate::Event::SessionDataPruned {
                    account_id: account.clone(),
                    session_id,
                    timestamp: current_block,
                });
            }
            
            // Replace user sessions with only current ones
            *user_sessions = current_sessions;
        });
    }
    
    /// Records an interaction in a session.
    /// 
    /// # Parameters
    /// 
    /// * `origin` - The origin of the call
    /// * `session_id` - The ID of the session
    /// * `interaction_type` - The type of interaction
    /// * `outcome` - The outcome of the interaction
    /// * `parameters` - Optional parameters for the interaction
    /// 
    /// # Returns
    /// 
    /// * `Result<(), DispatchError>` - Ok if successful, Err otherwise
    pub fn record_interaction(
        origin: T::AccountId,
        session_id: u32,
        interaction_type: u8,
        outcome: u8,
        parameters: Option<Vec<(u8, u16)>>,
    ) -> Result<(), DispatchError> {
        // Get the session and perform all validations in a single storage operation
        crate::InteractiveSessions::<T>::try_mutate(session_id, |session_opt| -> Result<(), DispatchError> {
            // Get the session
            let session = session_opt.as_mut().ok_or(Error::<T>::SessionNotFound)?;
            
            // Ensure the session hasn't ended yet
            ensure!(session.duration == 0, Error::<T>::SessionAlreadyEnded);
            
            // Ensure the caller is the owner of the pet
            let pet_owner = crate::OwnerOf::<T>::get(session.pet_id).ok_or(Error::<T>::NotOwner)?;
            ensure!(pet_owner == origin, Error::<T>::NotOwner);
            
            // Get the pet to check its state
            let pet = crate::PetNfts::<T>::get(session.pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(!pet.is_locked, Error::<T>::PetIsLocked);
            
            // Check if the session is active
            let block_number = frame_system::Pallet::<T>::block_number();
            let current_time = block_number.saturated_into::<u64>();
            
            // Check if the session has expired
            ensure!(
                current_time - session.start_time <= MAX_SESSION_DURATION,
                Error::<T>::SessionExpired
            );
            
            // Check if the session has too many interactions
            ensure!(
                session.interactions.len() < MAX_INTERACTIONS_PER_SESSION as usize,
                Error::<T>::TooManySessionInteractions
            );
        
        // Validate interaction type
        ensure!(
            interaction_type <= 20, // Limit to defined interaction types
            Error::<T>::InvalidInteractionType
        );
        
        // Rate limiting: Check if there are too many interactions in a short time
        let recent_interactions = session.interactions.iter()
            .filter(|i| current_time - i.timestamp < 3) // Less than 3 seconds apart
            .count();
        
        ensure!(
            recent_interactions < 5, // No more than 5 interactions in 3 seconds
            Error::<T>::InteractionRateLimitExceeded
        );
        
        // Create the interaction
        let interaction = SessionInteraction {
            interaction_type,
            timestamp: current_time,
            outcome,
        };
        
        // Add the interaction
        session.interactions.try_push(interaction)
            .map_err(|_| Error::<T>::TooManySessionInteractions)?;
        
        // Process interaction effects based on type and outcome
        let (mood_effect, xp_effect) = Self::calculate_interaction_effects(
            interaction_type, 
            outcome, 
            parameters.as_ref()
        );
        
        // Apply mood effect if any
        if mood_effect != 0 {
            Self::record_mood_change_internal(
                &origin,
                session_id,
                &mut session,
                mood_effect,
                current_time,
                block_number,
            )?;
        }
        
        // Apply experience effect if any
        if xp_effect > 0 {
            crate::PetNfts::<T>::try_mutate(session.pet_id, |pet_opt| -> DispatchResult {
                let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
                
                // Add a small amount of XP for the interaction
                pet.experience_points = pet.experience_points.saturating_add(xp_effect);
                
                // Update last interaction time
                pet.last_interaction_time = block_number;
                
                Ok(())
            })?;
        }
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::SessionInteractionRecorded {
            session_id,
            interaction_type,
            outcome,
            timestamp: block_number,
        });
        
        Ok(())
    })?; // End of try_mutate
        
    Ok(())
    }
    
    /// Records a mood change in a session.
    /// 
    /// # Parameters
    /// 
    /// * `origin` - The origin of the call
    /// * `session_id` - The ID of the session
    /// * `change` - The mood change
    /// 
    /// # Returns
    /// 
    /// * `Result<(), DispatchError>` - Ok if successful, Err otherwise
    pub fn record_mood_change(
        origin: T::AccountId,
        session_id: u32,
        change: i8,
    ) -> Result<(), DispatchError> {
        // Get the session and perform all validations in a single storage operation
        crate::InteractiveSessions::<T>::try_mutate(session_id, |session_opt| -> Result<(), DispatchError> {
            // Get the session
            let session = session_opt.as_mut().ok_or(Error::<T>::SessionNotFound)?;
            
            // Ensure the session hasn't ended yet
            ensure!(session.duration == 0, Error::<T>::SessionAlreadyEnded);
            
            // Ensure the caller is the owner of the pet
            let pet_owner = crate::OwnerOf::<T>::get(session.pet_id).ok_or(Error::<T>::NotOwner)?;
            ensure!(pet_owner == origin, Error::<T>::NotOwner);
            
            // Get the pet to check its state
            let pet = crate::PetNfts::<T>::get(session.pet_id).ok_or(Error::<T>::PetNotFound)?;
            ensure!(!pet.is_locked, Error::<T>::PetIsLocked);
            
            // Check if the session is active
            let block_number = frame_system::Pallet::<T>::block_number();
            let current_time = block_number.saturated_into::<u64>();
            
            // Call the internal function to record the mood change
            Self::record_mood_change_internal(
                &origin,
                session_id,
                session,
                change,
                current_time,
                block_number,
            )
        })
    }
    
    /// Internal function to record a mood change
    fn record_mood_change_internal(
        origin: &T::AccountId,
        session_id: u32,
        session: &mut InteractiveSession,
        change: i8,
        current_time: u64,
        block_number: T::BlockNumber,
    ) -> Result<(), DispatchError> {
        // Check if the session has expired
        ensure!(
            current_time - session.start_time <= MAX_SESSION_DURATION,
            Error::<T>::SessionExpired
        );
        
        // Check if the session has too many mood changes
        ensure!(
            session.mood_changes.len() < MAX_MOOD_CHANGES_PER_SESSION as usize,
            Error::<T>::TooManySessionMoodChanges
        );
        
        // Rate limiting: Check if there are too many mood changes in a short time
        let recent_changes = session.mood_changes.iter()
            .filter(|m| current_time - m.timestamp < 5) // Less than 5 seconds apart
            .count();
        
        ensure!(
            recent_changes < 3, // No more than 3 mood changes in 5 seconds
            Error::<T>::MoodChangeRateLimitExceeded
        );
        
        // Limit the mood change to prevent abuse
        let capped_change = change.max(-MAX_MOOD_CHANGE_PER_INTERACTION).min(MAX_MOOD_CHANGE_PER_INTERACTION);
        
        // Create the mood change
        let mood_change = MoodChange {
            change: capped_change,
            timestamp: current_time,
        };
        
        // Add the mood change
        session.mood_changes.try_push(mood_change)
            .map_err(|_| Error::<T>::TooManySessionMoodChanges)?;
        
        // Update the pet's mood
        crate::PetNfts::<T>::try_mutate(session.pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Calculate new mood with bounds checking
            let old_mood = pet.mood_indicator;
            let new_mood = old_mood.saturating_add_signed(capped_change);
            
            // Apply mood bounds
            pet.mood_indicator = new_mood.min(T::MaxMoodValue::get());
            
            // Update last interaction time
            pet.last_interaction_time = block_number;
            
            // Increment the state version for synchronization
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Check for mood-based achievements if mood improved significantly
            if capped_change >= 5 {
                if let Err(e) = crate::achievements::AchievementSystem::<T>::check_achievements(session.pet_id) {
                    // Log error but don't fail the transaction
                    log::warn!("Failed to check achievements: {:?}", e);
                }
            }
            
            Ok(())
        })?;
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::SessionMoodChangeRecorded {
            session_id,
            change: capped_change,
            timestamp: block_number,
        });
        
        Ok(())
    }
    
    /// Calculate interaction effects based on type and outcome
    fn calculate_interaction_effects(
        interaction_type: u8,
        outcome: u8,
        parameters: Option<&Vec<(u8, u16)>>,
    ) -> (i8, u32) {
        // Default effects
        let mut mood_effect = 0;
        let mut xp_effect = 0;
        
        // Calculate effects based on interaction type
        match interaction_type {
            0 => { // Pet
                mood_effect = 3;
                xp_effect = 1;
            },
            1 => { // Feed
                mood_effect = 5;
                xp_effect = 2;
            },
            2 => { // Play
                mood_effect = 7;
                xp_effect = 3;
            },
            3 => { // Train
                mood_effect = 1;
                xp_effect = 5;
            },
            4 => { // Groom
                mood_effect = 4;
                xp_effect = 2;
            },
            5 => { // Exercise
                mood_effect = 2;
                xp_effect = 4;
            },
            // Add more interaction types as needed
            _ => {
                // Default for unknown types
                mood_effect = 1;
                xp_effect = 1;
            }
        }
        
        // Adjust effects based on outcome
        match outcome {
            0 => { // Neutral outcome
                // No adjustment
            },
            1 => { // Positive outcome
                mood_effect = (mood_effect * 3) / 2; // 50% boost
                xp_effect = (xp_effect * 3) / 2; // 50% boost
            },
            2 => { // Negative outcome
                mood_effect = -mood_effect / 2; // Convert to negative and reduce
                xp_effect = xp_effect / 2; // Reduce
            },
            _ => {
                // Unknown outcome, use defaults
            }
        }
        
        // Apply parameter modifiers if provided
        if let Some(params) = parameters {
            for (param_type, value) in params {
                match param_type {
                    0 => { // Intensity modifier
                        let modifier = (*value as f32 / 100.0) as i8;
                        mood_effect = mood_effect.saturating_add(modifier);
                    },
                    1 => { // Duration modifier
                        let modifier = (*value / 10) as u32;
                        xp_effect = xp_effect.saturating_add(modifier);
                    },
                    // Add more parameter types as needed
                    _ => {}
                }
            }
        }
        
        // Apply final bounds
        mood_effect = mood_effect.max(-MAX_MOOD_CHANGE_PER_INTERACTION).min(MAX_MOOD_CHANGE_PER_INTERACTION);
        xp_effect = xp_effect.min(20); // Cap XP gain per interaction
        
        (mood_effect, xp_effect)
    }
    
    /// Processes a multi-touch interaction.
    /// 
    /// # Parameters
    /// 
    /// * `origin` - The origin of the call
    /// * `pet_id` - The ID of the pet
    /// * `interaction` - The multi-touch interaction
    /// 
    /// # Returns
    /// 
    /// * `Result<(), DispatchError>` - Ok if successful, Err otherwise
    pub fn process_multi_touch_interaction(
        origin: T::AccountId,
        pet_id: PetId,
        interaction: MultiTouchInteraction,
    ) -> Result<(), DispatchError> {
        // Ensure the pet exists and is not locked
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        ensure!(!pet.is_locked, Error::<T>::PetIsLocked);
        
        // Ensure the caller is the owner of the pet
        let pet_owner = crate::OwnerOf::<T>::get(pet_id).ok_or(Error::<T>::NotOwner)?;
        ensure!(pet_owner == origin, Error::<T>::NotOwner);
        
        // Validate the interaction
        ensure!(
            interaction.touch_count > 0 && interaction.touch_count <= 10,
            Error::<T>::InvalidMultiTouchInteraction
        );
        
        ensure!(
            interaction.touch_points.len() == interaction.touch_count as usize,
            Error::<T>::InvalidMultiTouchInteraction
        );
        
        // Validate interaction type
        ensure!(
            interaction.interaction_type <= 10, // Limit to defined multi-touch interaction types
            Error::<T>::InvalidInteractionType
        );
        
        // Rate limiting: Check if there have been too many multi-touch interactions recently
        let current_block = frame_system::Pallet::<T>::block_number();
        let last_interaction_time = pet.last_interaction_time;
        
        // Ensure there's at least a 2-second gap between multi-touch interactions
        ensure!(
            current_block.saturating_sub(last_interaction_time) >= T::BlockNumber::from(2),
            Error::<T>::InteractionRateLimitExceeded
        );
        
        // Calculate effects based on the multi-touch interaction
        let (mood_effect, xp_effect) = Self::calculate_multi_touch_effects(&interaction);
        
        // Apply the effects to the pet
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Update mood with bounds checking
            if mood_effect != 0 {
                let new_mood = pet.mood_indicator.saturating_add_signed(mood_effect);
                pet.mood_indicator = new_mood.min(T::MaxMoodValue::get());
            }
            
            // Add experience points
            if xp_effect > 0 {
                pet.experience_points = pet.experience_points.saturating_add(xp_effect);
            }
            
            // Update last interaction time
            pet.last_interaction_time = current_block;
            
            // Increment the state version for synchronization
            pet.state_version = pet.state_version.saturating_add(1);
            
            Ok(())
        })?;
        
        // Find or create an active session for this interaction
        let session_id = Self::find_or_create_session(origin.clone(), pet_id)?;
        
        // Record the interaction in the session
        if let Ok(mut session) = crate::InteractiveSessions::<T>::try_get(session_id) {
            // Only record if the session is still active
            if session.duration == 0 {
                // Create the interaction record
                let session_interaction = SessionInteraction {
                    interaction_type: interaction.interaction_type + 100, // Use 100+ range for multi-touch
                    timestamp: current_block.saturated_into::<u64>(),
                    outcome: interaction.touch_count, // Use touch count as outcome for multi-touch
                };
                
                // Try to add the interaction to the session
                if let Ok(_) = session.interactions.try_push(session_interaction) {
                    // If mood effect is significant, record it in the session
                    if mood_effect.abs() >= 3 {
                        let mood_change = MoodChange {
                            change: mood_effect,
                            timestamp: current_block.saturated_into::<u64>(),
                        };
                        
                        // Try to add the mood change to the session
                        let _ = session.mood_changes.try_push(mood_change);
                    }
                    
                    // Update the session in storage
                    crate::InteractiveSessions::<T>::insert(session_id, session);
                }
            }
        }
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::MultiTouchInteractionProcessed {
            pet_id,
            interaction_id: interaction.interaction_id,
            touch_count: interaction.touch_count,
            timestamp: current_block,
        });
        
        Ok(())
    }
    
    /// Calculate effects for multi-touch interactions
    fn calculate_multi_touch_effects(interaction: &MultiTouchInteraction) -> (i8, u32) {
        let mut mood_effect = 0;
        let mut xp_effect = 0;
        
        // Base effects depend on the number of touch points
        let base_mood = interaction.touch_count as i8;
        let base_xp = interaction.touch_count as u32;
        
        // Apply effects based on interaction type
        match interaction.interaction_type {
            0 => { // Pet with multiple fingers
                mood_effect = base_mood + 2;
                xp_effect = base_xp;
            },
            1 => { // Circular motion with multiple fingers
                mood_effect = base_mood + 1;
                xp_effect = base_xp + 2;
            },
            2 => { // Pinch with multiple fingers
                mood_effect = base_mood;
                xp_effect = base_xp + 3;
            },
            3 => { // Spread with multiple fingers
                mood_effect = base_mood + 3;
                xp_effect = base_xp + 1;
            },
            4 => { // Tap pattern with multiple fingers
                mood_effect = base_mood + 2;
                xp_effect = base_xp + 2;
            },
            // Add more interaction types as needed
            _ => {
                // Default for unknown types
                mood_effect = base_mood;
                xp_effect = base_xp;
            }
        }
        
        // Apply complexity bonus for interactions with many touch points
        if interaction.touch_count >= 5 {
            mood_effect += 2;
            xp_effect += 3;
        }
        
        // Apply pressure sensitivity if available
        let avg_pressure = interaction.touch_points.iter()
            .map(|point| point.pressure as u32)
            .sum::<u32>() / interaction.touch_count as u32;
        
        // Adjust effects based on pressure (higher pressure = stronger effect)
        if avg_pressure > 200 { // High pressure
            mood_effect = (mood_effect * 3) / 2;
            xp_effect = (xp_effect * 3) / 2;
        } else if avg_pressure < 50 { // Light touch
            mood_effect = (mood_effect * 2) / 3;
            xp_effect = (xp_effect * 2) / 3;
        }
        
        // Apply final bounds
        mood_effect = mood_effect.max(-MAX_MOOD_CHANGE_PER_INTERACTION).min(MAX_MOOD_CHANGE_PER_INTERACTION);
        xp_effect = xp_effect.min(30); // Cap XP gain per multi-touch interaction
        
        (mood_effect, xp_effect)
    }
    
    /// Find an active session or create a new one - optimized version
    fn find_or_create_session(origin: T::AccountId, pet_id: PetId) -> Result<u32, DispatchError> {
        // Get the current block number
        let block_number = frame_system::Pallet::<T>::block_number();
        let current_time = block_number.saturated_into::<u64>();
        
        // Get user sessions and find active session in a more functional way
        let user_sessions = crate::UserSessions::<T>::get(&origin);
        
        // Use find_map for more concise code
        let active_session_id = user_sessions.iter()
            .filter_map(|(session_id, _)| {
                // Try to get the session and check if it's active for this pet
                crate::InteractiveSessions::<T>::try_get(session_id).ok()
                    .filter(|session| 
                        session.pet_id == pet_id && 
                        session.duration == 0 && 
                        current_time - session.start_time <= MAX_SESSION_DURATION
                    )
                    .map(|_| *session_id)
            })
            .next();
        
        // Return existing session ID or create a new one
        match active_session_id {
            Some(id) => Ok(id),
            None => {
                let session = Self::start_interactive_session(origin, pet_id)?;
                Ok(session.session_id)
            }
        }
    }
}

impl<T: Config> InteractiveSystem<T> {
    /// Gets the available gestures for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<InteractiveGesture>, DispatchError>` - The available gestures, or an error
    pub fn get_available_gestures(pet_id: PetId) -> Result<Vec<InteractiveGesture>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get all gestures
        let all_gestures = Self::get_all_gestures();
        
        // Filter gestures based on pet state and requirements
        let available_gestures = all_gestures.into_iter()
            .filter(|gesture| Self::check_gesture_requirements(&pet, gesture))
            .collect();
        
        Ok(available_gestures)
    }
    
    /// Checks if a pet meets the requirements for a gesture.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `gesture` - The gesture
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the pet meets the requirements, false otherwise
    fn check_gesture_requirements(pet: &PetNft<T>, gesture: &InteractiveGesture) -> bool {
        // If there are no requirements, the pet meets them by default
        if gesture.requirements.is_empty() {
            return true;
        }
        
        // Use all() for more functional approach and early return
        gesture.requirements.iter().all(|requirement| {
            match requirement.requirement_type {
                0 => pet.mood_indicator >= requirement.value,                // Mood requirement
                1 => pet.level >= requirement.value as u32,                  // Level requirement
                2 => pet.base_strength >= requirement.value,                 // Strength requirement
                3 => pet.base_agility >= requirement.value,                  // Agility requirement
                4 => pet.base_intelligence >= requirement.value,             // Intelligence requirement
                5 => pet.base_vitality >= requirement.value,                 // Vitality requirement
                6 => pet.primary_elemental_affinity as u8 == requirement.value, // Elemental affinity requirement
                _ => false, // Unknown requirement type, consider it not met
            }
        })
    }
    
    /// Gets all possible gestures.
    /// 
    /// # Returns
    /// 
    /// * `Vec<InteractiveGesture>` - All gestures
    fn get_all_gestures() -> Vec<InteractiveGesture> {
        // In a real implementation, this would get the gestures from storage
        // For now, we'll just return hardcoded gestures with proper error handling
        
        let mut gestures = Vec::new();
        
        // Pet gesture
        if let Ok(name) = b"Pet".to_vec().try_into() {
            if let Ok(description) = b"Gently pet your critter to show affection.".to_vec().try_into() {
                if let Ok(icon) = b"pet_icon".to_vec().try_into() {
                    if let Ok(effects) = vec![
                        GestureEffect { effect_type: 0, magnitude: 5 },
                        GestureEffect { effect_type: 1, magnitude: 2 },
                    ].try_into() {
                        if let Ok(requirements) = Vec::new().try_into() {
                            gestures.push(InteractiveGesture {
                                gesture_id: GestureType::Pet as u8,
                                name,
                                description,
                                icon,
                                effects,
                                requirements,
                            });
                        }
                    }
                }
            }
        }
        
        // Tickle gesture
        if let Ok(name) = b"Tickle".to_vec().try_into() {
            if let Ok(description) = b"Tickle your critter to make it laugh.".to_vec().try_into() {
                if let Ok(icon) = b"tickle_icon".to_vec().try_into() {
                    if let Ok(effects) = vec![
                        GestureEffect { effect_type: 0, magnitude: 8 },
                        GestureEffect { effect_type: 1, magnitude: 1 },
                    ].try_into() {
                        if let Ok(requirements) = vec![
                            GestureRequirement { requirement_type: 0, value: 100 },
                        ].try_into() {
                            gestures.push(InteractiveGesture {
                                gesture_id: GestureType::Tickle as u8,
                                name,
                                description,
                                icon,
                                effects,
                                requirements,
                            });
                        }
                    }
                }
            }
        }
        
        // Add more gestures as needed...
        
        gestures
    }
    
    /// Gets the touch responses for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `touch_area` - The touch area
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<TouchResponse>, DispatchError>` - The touch responses, or an error
    pub fn get_touch_responses(pet_id: PetId, touch_area: u8) -> Result<Vec<TouchResponse>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Validate the touch area
        ensure!(
            touch_area <= TouchArea::Wings as u8,
            Error::<T>::InvalidTouchArea
        );
        
        // Get all touch responses for the area
        let all_responses = Self::get_all_touch_responses(touch_area);
        
        // Filter responses based on pet mood
        let available_responses = all_responses.into_iter()
            .filter(|response| pet.mood_indicator >= response.mood_requirement)
            .collect();
        
        Ok(available_responses)
    }
    
    /// Gets all possible touch responses for a touch area.
    /// 
    /// # Parameters
    /// 
    /// * `touch_area` - The touch area
    /// 
    /// # Returns
    /// 
    /// * `Vec<TouchResponse>` - All touch responses for the area
    fn get_all_touch_responses(touch_area: u8) -> Vec<TouchResponse> {
        // In a real implementation, this would get the responses from storage
        // For now, we'll just return hardcoded responses
        
        match touch_area {
            0 => { // Head
                vec![
                    TouchResponse {
                        response_id: 1,
                        touch_area: TouchArea::Head as u8,
                        mood_requirement: 0,
                        animation: 1,
                        sound: 1,
                        mood_effect: 2,
                    },
                    TouchResponse {
                        response_id: 2,
                        touch_area: TouchArea::Head as u8,
                        mood_requirement: 100,
                        animation: 2,
                        sound: 2,
                        mood_effect: 5,
                    },
                ]
            },
            1 => { // Back
                vec![
                    TouchResponse {
                        response_id: 3,
                        touch_area: TouchArea::Back as u8,
                        mood_requirement: 0,
                        animation: 3,
                        sound: 3,
                        mood_effect: 1,
                    },
                ]
            },
            // Add more touch areas as needed...
            _ => Vec::new(),
        }
    }
    
    /// Gets the dynamic UI elements for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<DynamicUiElement>, DispatchError>` - The dynamic UI elements, or an error
    pub fn get_dynamic_ui_elements(pet_id: PetId) -> Result<Vec<DynamicUiElement>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get all UI elements
        let all_elements = Self::get_all_ui_elements();
        
        // Filter elements based on pet state and conditions
        let available_elements = all_elements.into_iter()
            .filter(|element| Self::check_ui_element_conditions(&pet, element))
            .collect();
        
        Ok(available_elements)
    }
    
    /// Checks if a pet meets the conditions for a UI element.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `element` - The UI element
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the pet meets the conditions, false otherwise
    fn check_ui_element_conditions(pet: &PetNft<T>, element: &DynamicUiElement) -> bool {
        // If there are no conditions, the pet meets them by default
        if element.state_conditions.is_empty() {
            return true;
        }
        
        // Check each condition
        for condition in element.state_conditions.iter() {
            let pet_value = match condition.condition_type {
                0 => pet.mood_indicator as u8, // Mood
                1 => 100, // Health (placeholder)
                2 => (pet.experience_points % 100) as u8, // Experience progress
                3 => pet.level as u8, // Level
                4 => pet.primary_elemental_affinity as u8, // Elemental affinity
                _ => 0, // Unknown condition type
            };
            
            let meets_condition = match condition.comparison_operator {
                0 => pet_value == condition.value, // Equal
                1 => pet_value != condition.value, // Not equal
                2 => pet_value > condition.value, // Greater than
                3 => pet_value < condition.value, // Less than
                4 => pet_value >= condition.value, // Greater than or equal
                5 => pet_value <= condition.value, // Less than or equal
                _ => false, // Unknown comparison operator
            };
            
            if !meets_condition {
                return false;
            }
        }
        
        // All conditions met
        true
    }
    
    /// Gets all possible UI elements.
    /// 
    /// # Returns
    /// 
    /// * `Vec<DynamicUiElement>` - All UI elements
    fn get_all_ui_elements() -> Vec<DynamicUiElement> {
        // In a real implementation, this would get the elements from storage
        // For now, we'll just return hardcoded elements with proper error handling
        
        let mut elements = Vec::new();
        
        // Mood indicator
        if let Ok(name) = b"Mood Indicator".to_vec().try_into() {
            if let Ok(description) = b"Shows the pet's current mood.".to_vec().try_into() {
                if let Ok(state_conditions) = Vec::new().try_into() {
                    if let Ok(visual_properties) = vec![
                        VisualProperty {
                            property_type: 0,
                            value: b"#00FF00".to_vec().try_into().unwrap_or_default(),
                        },
                    ].try_into() {
                        elements.push(DynamicUiElement {
                            element_id: 1,
                            name,
                            description,
                            element_type: UiElementType::MoodIndicator as u8,
                            state_conditions,
                            visual_properties,
                        });
                    }
                }
            }
        }
        
        // Add more UI elements as needed...
        
        elements
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, Test, Origin, CritterNfts};
    use frame_support::{assert_ok, assert_noop};
    
    #[test]
    fn test_interactive_session_lifecycle() {
        new_test_ext().execute_with(|| {
            // Create a test account
            let account = 1;
            
            // Mint a pet for testing
            assert_ok!(CritterNfts::mint_pet(
                Origin::signed(account),
                b"TestPet".to_vec(),
                b"TestSpecies".to_vec(),
            ));
            
            // Start an interactive session
            let session = InteractiveSessionSystem::<Test>::start_interactive_session(
                account,
                0 // First pet has ID 0
            ).unwrap();
            
            // Verify session was created correctly
            assert_eq!(session.pet_id, 0);
            assert_eq!(session.duration, 0);
            assert_eq!(session.interactions.len(), 0);
            assert_eq!(session.mood_changes.len(), 0);
            assert_eq!(session.rewards.len(), 0);
            
            // Record an interaction
            assert_ok!(InteractiveSessionSystem::<Test>::record_interaction(
                account,
                session.session_id,
                1, // Feed interaction
                1, // Positive outcome
                None
            ));
            
            // Record a mood change
            assert_ok!(InteractiveSessionSystem::<Test>::record_mood_change(
                account,
                session.session_id,
                5 // Positive mood change
            ));
            
            // End the session
            let updated_session = InteractiveSessionSystem::<Test>::end_interactive_session(
                account,
                session.session_id
            ).unwrap();
            
            // Verify session was updated correctly
            assert!(updated_session.duration > 0);
            assert_eq!(updated_session.interactions.len(), 1);
            assert_eq!(updated_session.mood_changes.len(), 1);
            assert!(updated_session.rewards.len() > 0);
            
            // Verify pet state was updated
            let pet = CritterNfts::pet_nfts(0).unwrap();
            assert!(pet.experience_points > 0);
        });
    }
    
    #[test]
    fn test_multi_touch_interaction() {
        new_test_ext().execute_with(|| {
            // Create a test account
            let account = 1;
            
            // Mint a pet for testing
            assert_ok!(CritterNfts::mint_pet(
                Origin::signed(account),
                b"TestPet".to_vec(),
                b"TestSpecies".to_vec(),
            ));
            
            // Create a multi-touch interaction
            let touch_points = vec![
                PressureTouchPoint { x: 100, y: 100, pressure: 128 },
                PressureTouchPoint { x: 200, y: 200, pressure: 128 }
            ].try_into().unwrap();
            
            let interaction = MultiTouchInteraction {
                interaction_id: 1,
                touch_count: 2,
                touch_points,
                interaction_type: 0 // Pet with multiple fingers
            };
            
            // Process the multi-touch interaction
            assert_ok!(InteractiveSessionSystem::<Test>::process_multi_touch_interaction(
                account,
                0, // First pet has ID 0
                interaction
            ));
            
            // Verify pet state was updated
            let pet = CritterNfts::pet_nfts(0).unwrap();
            assert!(pet.experience_points > 0);
        });
    }
}