# Interactive System Cleanup Instructions

## Overview

The `interactive.rs` file contains several duplicate implementations of functions that should be moved to separate subsystems. This document provides instructions for cleaning up the file.

## Steps

1. **Create Subsystem Structs**

   The following subsystem structs should be created:

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

2. **Move Functions to Subsystems**

   The following functions should be moved to the appropriate subsystems:

   - **InteractiveSessionSystem**:
     - `start_interactive_session`
     - `end_interactive_session`
     - `record_interaction`
     - `record_mood_change`

   - **GestureRecognitionSystem**:
     - `get_all_gestures`
     - `check_gesture_requirements`
     - `process_gesture`
     - `recognize_gesture`

   - **TouchInteractionSystem**:
     - `get_all_touch_responses`
     - `process_touch`
     - `process_multi_touch`

   - **DynamicUiSystem**:
     - `get_all_ui_elements`
     - `check_element_conditions`
     - `get_dynamic_ui_elements`
     - `get_advanced_dynamic_ui_elements`
     - `get_context_specific_elements`
     - `get_mood_specific_elements`
     - `get_elemental_affinity_elements`
     - `get_time_of_day_elements`
     - `get_seasonal_elements`

   - **PatternMatchingSystem**:
     - `match_pattern`
     - `calculate_pattern_similarity`

3. **Update InteractiveSystem to Delegate**

   The `InteractiveSystem` should be updated to delegate to the appropriate subsystems:

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

4. **Remove Duplicate Implementations**

   After moving the functions to the appropriate subsystems and updating the `InteractiveSystem` to delegate, remove the duplicate implementations from the file.

## Benefits

This cleanup will result in a more modular, maintainable, and testable codebase. Each subsystem will have a clear responsibility, and changes to one subsystem won't affect others.