# Interactive System Cleanup Implementation

## Overview

This document provides the implementation details for cleaning up the `interactive.rs` file. The goal is to move duplicate implementations to separate subsystems and update the `InteractiveSystem` to delegate to these subsystems.

## Implementation

### 1. Create Subsystem Structs

```rust
/// A system for managing interactive sessions.
pub struct InteractiveSessionSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

/// A system for gesture recognition.
pub struct GestureRecognitionSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

/// A system for touch interaction.
pub struct TouchInteractionSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

/// A system for dynamic UI elements.
pub struct DynamicUiSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

/// A system for pattern matching.
pub struct PatternMatchingSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}
```

### 2. Move Functions to Subsystems

#### InteractiveSessionSystem

```rust
impl<T: Config> InteractiveSessionSystem<T> {
    /// Starts an interactive session with a pet.
    pub fn start_interactive_session(pet_id: PetId) -> Result<InteractiveSession, DispatchError> {
        // Ensure the pet exists
        ensure!(crate::PetNfts::<T>::contains_key(pet_id), Error::<T>::PetNotFound);
        
        // Generate a session ID
        let session_id = frame_system::Pallet::<T>::block_number().saturated_into::<u32>();
        
        // Get the current timestamp
        let start_time = frame_system::Pallet::<T>::block_number().saturated_into::<u64>();
        
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
        
        Ok(session)
    }
    
    /// Ends an interactive session with a pet.
    pub fn end_interactive_session(session_id: u32) -> Result<InteractiveSession, DispatchError> {
        // Get the session
        let mut session = crate::InteractiveSessions::<T>::get(session_id).ok_or(Error::<T>::SessionNotFound)?;
        
        // Calculate the duration
        let end_time = frame_system::Pallet::<T>::block_number().saturated_into::<u64>();
        session.duration = (end_time - session.start_time) as u32;
        
        // Calculate rewards based on the session
        let mut rewards = Vec::new();
        
        // Reward based on duration
        let duration_reward = session.duration / 60; // 1 point per minute
        rewards.push((0, duration_reward));
        
        // Reward based on interactions
        let interaction_reward = session.interactions.len() as u32 * 2; // 2 points per interaction
        rewards.push((1, interaction_reward));
        
        // Reward based on mood changes
        let mut total_mood_change = 0;
        for (change, _) in session.mood_changes.iter() {
            total_mood_change += *change;
        }
        
        let mood_reward = if total_mood_change > 0 {
            total_mood_change as u32 * 3 // 3 points per positive mood change
        } else {
            0
        };
        rewards.push((2, mood_reward));
        
        // Update the session rewards
        session.rewards = rewards.try_into().map_err(|_| Error::<T>::TooManyAttributes)?;
        
        // Update the session in storage
        crate::InteractiveSessions::<T>::insert(session_id, session.clone());
        
        Ok(session)
    }
    
    /// Records an interaction in a session.
    pub fn record_interaction(
        session_id: u32,
        interaction_type: u8,
        outcome: u8,
    ) -> Result<(), DispatchError> {
        // Get the session
        let mut session = crate::InteractiveSessions::<T>::get(session_id).ok_or(Error::<T>::SessionNotFound)?;
        
        // Get the current timestamp
        let timestamp = frame_system::Pallet::<T>::block_number().saturated_into::<u64>();
        
        // Add the interaction
        session.interactions.try_push((interaction_type, timestamp, outcome))
            .map_err(|_| Error::<T>::TooManyAttributes)?;
        
        // Update the session in storage
        crate::InteractiveSessions::<T>::insert(session_id, session);
        
        Ok(())
    }
    
    /// Records a mood change in a session.
    pub fn record_mood_change(
        session_id: u32,
        change: i8,
    ) -> Result<(), DispatchError> {
        // Get the session
        let mut session = crate::InteractiveSessions::<T>::get(session_id).ok_or(Error::<T>::SessionNotFound)?;
        
        // Get the current timestamp
        let timestamp = frame_system::Pallet::<T>::block_number().saturated_into::<u64>();
        
        // Add the mood change
        session.mood_changes.try_push((change, timestamp))
            .map_err(|_| Error::<T>::TooManyAttributes)?;
        
        // Update the session in storage
        crate::InteractiveSessions::<T>::insert(session_id, session);
        
        Ok(())
    }
}
```

### 3. Update InteractiveSystem to Delegate

```rust
impl<T: Config> InteractiveSystem<T> {
    // Session Management Functions
    pub fn start_interactive_session(pet_id: PetId) -> Result<InteractiveSession, DispatchError> {
        InteractiveSessionSystem::<T>::start_interactive_session(pet_id)
    }

    pub fn end_interactive_session(session_id: u32) -> Result<InteractiveSession, DispatchError> {
        InteractiveSessionSystem::<T>::end_interactive_session(session_id)
    }

    pub fn record_interaction(session_id: u32, interaction_type: u8, outcome: u8) -> Result<(), DispatchError> {
        InteractiveSessionSystem::<T>::record_interaction(session_id, interaction_type, outcome)
    }

    pub fn record_mood_change(session_id: u32, change: i8) -> Result<(), DispatchError> {
        InteractiveSessionSystem::<T>::record_mood_change(session_id, change)
    }

    // Gesture Recognition Functions
    pub fn get_all_gestures() -> Vec<InteractiveGesture> {
        GestureRecognitionSystem::<T>::get_all_gestures()
    }

    pub fn check_gesture_requirements(pet: &PetNft<T>, gesture: &InteractiveGesture) -> bool {
        GestureRecognitionSystem::<T>::check_gesture_requirements(pet, gesture)
    }

    pub fn process_gesture(pet_id: PetId, gesture_id: u8) -> Result<(u8, i8), DispatchError> {
        GestureRecognitionSystem::<T>::process_gesture(pet_id, gesture_id)
    }

    pub fn recognize_gesture(pet_id: PetId, touch_points: Vec<(i16, i16, u32)>) -> Result<GestureRecognitionResult, DispatchError> {
        GestureRecognitionSystem::<T>::recognize_gesture(pet_id, touch_points)
    }

    // Touch Interaction Functions
    pub fn get_all_touch_responses() -> Vec<TouchResponse> {
        TouchInteractionSystem::<T>::get_all_touch_responses()
    }

    pub fn process_touch(pet_id: PetId, touch_area: u8, pressure: u8) -> Result<u8, DispatchError> {
        TouchInteractionSystem::<T>::process_touch(pet_id, touch_area, pressure)
    }

    pub fn process_multi_touch(pet_id: PetId, touch_points: Vec<(i16, i16, u8)>) -> Result<u8, DispatchError> {
        TouchInteractionSystem::<T>::process_multi_touch(pet_id, touch_points)
    }

    // Dynamic UI Functions
    pub fn get_all_ui_elements() -> Vec<DynamicUiElement> {
        DynamicUiSystem::<T>::get_all_ui_elements()
    }

    pub fn check_element_conditions(pet: &PetNft<T>, element: &DynamicUiElement) -> bool {
        DynamicUiSystem::<T>::check_element_conditions(pet, element)
    }

    pub fn get_dynamic_ui_elements(pet_id: PetId) -> Result<Vec<DynamicUiElement>, DispatchError> {
        DynamicUiSystem::<T>::get_dynamic_ui_elements(pet_id)
    }

    pub fn get_advanced_dynamic_ui_elements(pet_id: PetId, context: Vec<u8>, screen_size: (u16, u16), theme: u8) -> Result<Vec<DynamicUiElement>, DispatchError> {
        DynamicUiSystem::<T>::get_advanced_dynamic_ui_elements(pet_id, context, screen_size, theme)
    }

    // Pattern Matching Functions
    pub fn match_pattern(pattern1: &[(i16, i16, u32)], pattern2: &[(i16, i16, u32)]) -> u8 {
        PatternMatchingSystem::<T>::match_pattern(pattern1, pattern2)
    }

    pub fn calculate_pattern_similarity(pattern1: &[(i16, i16, u32)], pattern2: &[(i16, i16, u32)]) -> u8 {
        PatternMatchingSystem::<T>::calculate_pattern_similarity(pattern1, pattern2)
    }
}
```

### 4. Remove Duplicate Implementations

After moving the functions to the appropriate subsystems and updating the `InteractiveSystem` to delegate, remove the duplicate implementations from the file.

## Benefits

This cleanup will result in a more modular, maintainable, and testable codebase. Each subsystem will have a clear responsibility, and changes to one subsystem won't affect others.