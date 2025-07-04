//! # Pet Lifecycle Events
//!
//! This module provides a system for significant lifecycle events in a pet's life,
//! creating memorable moments and unlocking new capabilities or customization options.

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::Get,
    BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;
use scale_info::TypeInfo;
use crate::{Config, Error, PetId, PetNft};

/// Represents a lifecycle event.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LifecycleEvent<T: Config> {
    /// The event type
    pub event_type: u8,
    
    /// The event name
    pub name: Vec<u8>,
    
    /// The event description
    pub description: Vec<u8>,
    
    /// The timestamp when the event occurred
    pub timestamp: BlockNumberFor<T>,
    
    /// The effects of the event
    pub effects: LifecycleEventEffects,
    
    /// The requirements for the event
    pub requirements: LifecycleEventRequirements,
}

/// Represents the effects of a lifecycle event.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LifecycleEventEffects {
    /// The experience points awarded
    pub experience: u32,
    
    /// The mood boost awarded
    pub mood_boost: u8,
    
    /// The stat boosts awarded
    pub stat_boosts: Vec<(u8, u8)>, // (stat_type, boost)
    
    /// The special abilities unlocked
    pub special_abilities: Vec<u8>, // ability_ids
    
    /// The customization options unlocked
    pub customization_options: Vec<u8>, // option_ids
}

/// Represents the requirements for a lifecycle event.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LifecycleEventRequirements {
    /// The minimum level required
    pub min_level: u32,
    
    /// The minimum experience points required
    pub min_experience: u32,
    
    /// The minimum age (in blocks) required
    pub min_age: u64,
    
    /// The minimum stat values required
    pub min_stats: Vec<(u8, u8)>, // (stat_type, min_value)
    
    /// The skills required
    pub required_skills: Vec<(u8, u8)>, // (skill_type, min_level)
    
    /// The achievements required
    pub required_achievements: Vec<u32>, // achievement_ids
    
    /// The previous lifecycle events required
    pub prerequisite_events: Vec<u8>, // event_types
}

/// Lifecycle event types.
pub enum LifecycleEventType {
    Birth = 0,
    Adolescence = 1,
    Adulthood = 2,
    Mastery = 3,
    Evolution = 4,
    Transformation = 5,
    Awakening = 6,
    Enlightenment = 7,
    Rebirth = 8,
}

/// A system for managing pet lifecycle events.
pub struct LifecycleEventSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> LifecycleEventSystem<T> {
    /// Triggers a lifecycle event for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `event_type` - The type of lifecycle event
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn trigger_lifecycle_event(
        pet_id: PetId,
        event_type: u8,
    ) -> DispatchResult {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the lifecycle event
        let event = Self::get_lifecycle_event(event_type)?;
        
        // Check if the pet meets the requirements
        Self::check_requirements(&pet, &event.requirements)?;
        
        // Check if the pet has already experienced this event
        Self::check_not_already_experienced(pet_id, event_type)?;
        
        // Apply the effects of the event
        Self::apply_event_effects(pet_id, &event.effects)?;
        
        // Record the event
        Self::record_event(pet_id, event_type)?;
        
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::LifecycleEvent {
            pet_id,
            event_type,
            timestamp: current_block,
        });
        
        // Record a memory of the event
        crate::memory::PetMemorySystem::<T>::record_memory(
            pet_id,
            crate::memory::MemoryType::LifecycleEvent as u8,
            255, // Maximum significance
            event_type.encode(), // Store the event type as associated data
        )?;
        
        Ok(())
    }
    
    /// Gets a lifecycle event by type.
    /// 
    /// # Parameters
    /// 
    /// * `event_type` - The type of lifecycle event
    /// 
    /// # Returns
    /// 
    /// * `Result<LifecycleEvent<T>, DispatchError>` - The lifecycle event, or an error
    fn get_lifecycle_event(
        event_type: u8,
    ) -> Result<LifecycleEvent<T>, DispatchError> {
        // In a real implementation, this would get the event from storage
        // For now, we'll just return a hardcoded event based on the type
        
        let current_block = frame_system::Pallet::<T>::block_number();
        
        match event_type {
            0 => { // Birth
                Ok(LifecycleEvent {
                    event_type,
                    name: b"Birth".to_vec(),
                    description: b"The beginning of a new life.".to_vec(),
                    timestamp: current_block,
                    effects: LifecycleEventEffects {
                        experience: 100,
                        mood_boost: 50,
                        stat_boosts: vec![
                            (0, 5), // Strength +5
                            (1, 5), // Agility +5
                            (2, 5), // Intelligence +5
                            (3, 5), // Vitality +5
                        ],
                        special_abilities: Vec::new(),
                        customization_options: Vec::new(),
                    },
                    requirements: LifecycleEventRequirements {
                        min_level: 1,
                        min_experience: 0,
                        min_age: 0,
                        min_stats: Vec::new(),
                        required_skills: Vec::new(),
                        required_achievements: Vec::new(),
                        prerequisite_events: Vec::new(),
                    },
                })
            },
            1 => { // Adolescence
                Ok(LifecycleEvent {
                    event_type,
                    name: b"Adolescence".to_vec(),
                    description: b"The transition from childhood to adulthood.".to_vec(),
                    timestamp: current_block,
                    effects: LifecycleEventEffects {
                        experience: 500,
                        mood_boost: 30,
                        stat_boosts: vec![
                            (0, 10), // Strength +10
                            (1, 10), // Agility +10
                            (2, 10), // Intelligence +10
                            (3, 10), // Vitality +10
                        ],
                        special_abilities: vec![0], // First special ability
                        customization_options: vec![0, 1], // First two customization options
                    },
                    requirements: LifecycleEventRequirements {
                        min_level: 10,
                        min_experience: 1000,
                        min_age: 1000, // Arbitrary age requirement
                        min_stats: Vec::new(),
                        required_skills: Vec::new(),
                        required_achievements: Vec::new(),
                        prerequisite_events: vec![0], // Birth
                    },
                })
            },
            2 => { // Adulthood
                Ok(LifecycleEvent {
                    event_type,
                    name: b"Adulthood".to_vec(),
                    description: b"The prime of life.".to_vec(),
                    timestamp: current_block,
                    effects: LifecycleEventEffects {
                        experience: 1000,
                        mood_boost: 20,
                        stat_boosts: vec![
                            (0, 15), // Strength +15
                            (1, 15), // Agility +15
                            (2, 15), // Intelligence +15
                            (3, 15), // Vitality +15
                        ],
                        special_abilities: vec![1, 2], // Two more special abilities
                        customization_options: vec![2, 3, 4], // Three more customization options
                    },
                    requirements: LifecycleEventRequirements {
                        min_level: 20,
                        min_experience: 5000,
                        min_age: 2000, // Arbitrary age requirement
                        min_stats: vec![
                            (0, 50), // Strength >= 50
                            (1, 50), // Agility >= 50
                            (2, 50), // Intelligence >= 50
                            (3, 50), // Vitality >= 50
                        ],
                        required_skills: vec![(0, 5)], // At least one skill at level 5
                        required_achievements: vec![1], // At least one achievement
                        prerequisite_events: vec![0, 1], // Birth and Adolescence
                    },
                })
            },
            _ => {
                // Default to a generic event
                Ok(LifecycleEvent {
                    event_type,
                    name: b"Unknown Event".to_vec(),
                    description: b"An unknown lifecycle event.".to_vec(),
                    timestamp: current_block,
                    effects: LifecycleEventEffects {
                        experience: 100,
                        mood_boost: 10,
                        stat_boosts: vec![
                            (0, 5), // Strength +5
                            (1, 5), // Agility +5
                            (2, 5), // Intelligence +5
                            (3, 5), // Vitality +5
                        ],
                        special_abilities: Vec::new(),
                        customization_options: Vec::new(),
                    },
                    requirements: LifecycleEventRequirements {
                        min_level: 1,
                        min_experience: 0,
                        min_age: 0,
                        min_stats: Vec::new(),
                        required_skills: Vec::new(),
                        required_achievements: Vec::new(),
                        prerequisite_events: Vec::new(),
                    },
                })
            }
        }
    }
    
    /// Checks if a pet meets the requirements for a lifecycle event.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `requirements` - The event requirements
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if the pet meets the requirements, Err otherwise
    fn check_requirements(
        pet: &PetNft<T>,
        requirements: &LifecycleEventRequirements,
    ) -> DispatchResult {
        // Check level requirement
        ensure!(
            pet.level >= requirements.min_level,
            Error::<T>::PetTooYoung
        );
        
        // Check experience requirement
        ensure!(
            pet.experience_points >= requirements.min_experience,
            Error::<T>::PetTooYoung
        );
        
        // Check age requirement
        let current_block = frame_system::Pallet::<T>::block_number();
        let age = current_block.saturated_into::<u64>().saturating_sub(pet.last_state_update_block.saturated_into::<u64>());
        ensure!(
            age >= requirements.min_age,
            Error::<T>::PetTooYoung
        );
        
        // Check stat requirements
        for (stat_type, min_value) in &requirements.min_stats {
            let stat_value = match stat_type {
                0 => pet.base_strength,
                1 => pet.base_agility,
                2 => pet.base_intelligence,
                3 => pet.base_vitality,
                _ => 0,
            };
            
            ensure!(
                stat_value >= *min_value,
                Error::<T>::StatTooLow
            );
        }
        
        // Check skill requirements
        if !requirements.required_skills.is_empty() {
            let skills = crate::PetSkills::<T>::get(pet.id);
            
            for (required_skill, min_level) in &requirements.required_skills {
                let mut has_skill = false;
                
                for (skill, level) in skills.iter() {
                    if *skill == *required_skill && *level >= *min_level {
                        has_skill = true;
                        break;
                    }
                }
                
                ensure!(
                    has_skill,
                    Error::<T>::SkillLevelTooLow
                );
            }
        }
        
        // Check achievement requirements
        if !requirements.required_achievements.is_empty() {
            let achievements = crate::PetAchievements::<T>::get(pet.id);
            
            for required_achievement in &requirements.required_achievements {
                ensure!(
                    achievements.iter().any(|(id, _)| *id == *required_achievement),
                    Error::<T>::AchievementRequirementsNotMet
                );
            }
        }
        
        // Check prerequisite events
        if !requirements.prerequisite_events.is_empty() {
            let events = crate::PetLifecycleEvents::<T>::get(pet.id);
            
            for prerequisite in &requirements.prerequisite_events {
                ensure!(
                    events.iter().any(|(event_type, _)| *event_type == *prerequisite),
                    Error::<T>::LifecycleEventNotAvailable
                );
            }
        }
        
        Ok(())
    }
    
    /// Checks if a pet has already experienced a lifecycle event.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `event_type` - The type of lifecycle event
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if the pet has not experienced the event, Err otherwise
    fn check_not_already_experienced(
        pet_id: PetId,
        event_type: u8,
    ) -> DispatchResult {
        let events = crate::PetLifecycleEvents::<T>::get(pet_id);
        
        ensure!(
            !events.iter().any(|(type_, _)| *type_ == event_type),
            Error::<T>::LifecycleEventAlreadyExperienced
        );
        
        Ok(())
    }
    
    /// Applies the effects of a lifecycle event to a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `effects` - The event effects
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn apply_event_effects(
        pet_id: PetId,
        effects: &LifecycleEventEffects,
    ) -> DispatchResult {
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Apply experience boost
            pet.experience_points = pet.experience_points.saturating_add(effects.experience);
            
            // Apply mood boost
            pet.mood_indicator = pet.mood_indicator
                .saturating_add(effects.mood_boost)
                .min(T::MaxMoodValue::get());
            
            // Apply stat boosts
            for (stat_type, boost) in &effects.stat_boosts {
                match stat_type {
                    0 => { // Strength
                        pet.base_strength = pet.base_strength.saturating_add(*boost);
                    },
                    1 => { // Agility
                        pet.base_agility = pet.base_agility.saturating_add(*boost);
                    },
                    2 => { // Intelligence
                        pet.base_intelligence = pet.base_intelligence.saturating_add(*boost);
                    },
                    3 => { // Vitality
                        pet.base_vitality = pet.base_vitality.saturating_add(*boost);
                    },
                    _ => {} // No boost for other stats
                }
            }
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })
    }
    
    /// Records a lifecycle event for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `event_type` - The type of lifecycle event
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn record_event(
        pet_id: PetId,
        event_type: u8,
    ) -> DispatchResult {
        crate::PetLifecycleEvents::<T>::try_mutate(pet_id, |events| -> DispatchResult {
            // Get the current block number
            let current_block = frame_system::Pallet::<T>::block_number();
            
            // Add the event
            events.try_push((event_type, current_block))
                .map_err(|_| Error::<T>::TooManyLifecycleEvents)?;
            
            Ok(())
        })
    }
    
    /// Gets all of a pet's lifecycle events.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u8, BlockNumberFor<T>)>` - The lifecycle events (event_type, timestamp)
    pub fn get_pet_lifecycle_events(
        pet_id: PetId,
    ) -> Vec<(u8, BlockNumberFor<T>)> {
        crate::PetLifecycleEvents::<T>::get(pet_id).to_vec()
    }
    
    /// Gets detailed information about a pet's lifecycle events.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<LifecycleEvent<T>>, DispatchError>` - The lifecycle events, or an error
    pub fn get_pet_lifecycle_event_details(
        pet_id: PetId,
    ) -> Result<Vec<LifecycleEvent<T>>, DispatchError> {
        let events = crate::PetLifecycleEvents::<T>::get(pet_id);
        
        let mut result = Vec::with_capacity(events.len());
        for (event_type, timestamp) in events.iter() {
            let mut event = Self::get_lifecycle_event(*event_type)?;
            event.timestamp = *timestamp;
            result.push(event);
        }
        
        Ok(result)
    }
    
    /// Gets the next available lifecycle events for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<LifecycleEvent<T>>, DispatchError>` - The next available lifecycle events, or an error
    pub fn get_next_lifecycle_events(
        pet_id: PetId,
    ) -> Result<Vec<LifecycleEvent<T>>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the pet's current lifecycle events
        let current_events = crate::PetLifecycleEvents::<T>::get(pet_id);
        
        // Get all possible lifecycle events
        let all_events = vec![
            Self::get_lifecycle_event(0)?, // Birth
            Self::get_lifecycle_event(1)?, // Adolescence
            Self::get_lifecycle_event(2)?, // Adulthood
            Self::get_lifecycle_event(3)?, // Mastery
            Self::get_lifecycle_event(4)?, // Evolution
            Self::get_lifecycle_event(5)?, // Transformation
            Self::get_lifecycle_event(6)?, // Awakening
            Self::get_lifecycle_event(7)?, // Enlightenment
            Self::get_lifecycle_event(8)?, // Rebirth
        ];
        
        // Find events the pet hasn't experienced yet
        let mut result = Vec::new();
        for event in all_events {
            // Skip events the pet has already experienced
            if current_events.iter().any(|(type_, _)| *type_ == event.event_type) {
                continue;
            }
            
            // Check if the pet meets the requirements
            if Self::check_requirements(&pet, &event.requirements).is_ok() {
                result.push(event);
            }
        }
        
        Ok(result)
    }
}