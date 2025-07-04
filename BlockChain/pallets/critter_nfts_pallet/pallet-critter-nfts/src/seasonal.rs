//! # Seasonal Events System
//!
//! This module provides a system for seasonal events that affect all pets,
//! creating a dynamic world that changes over time and encouraging regular engagement.

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

/// Represents a seasonal event.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SeasonalEvent<T: Config> {
    /// The event ID
    pub id: u32,
    
    /// The event name
    pub name: Vec<u8>,
    
    /// The event description
    pub description: Vec<u8>,
    
    /// The start time of the event
    pub start_time: BlockNumberFor<T>,
    
    /// The end time of the event
    pub end_time: BlockNumberFor<T>,
    
    /// The effects of the event
    pub effects: SeasonalEventEffects,
}

/// Represents the effects of a seasonal event.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SeasonalEventEffects {
    /// The mood modifier
    pub mood_modifier: i8,
    
    /// The experience modifier (percentage)
    pub experience_modifier: i8,
    
    /// The stat modifiers
    pub stat_modifiers: Vec<(u8, i8)>, // (stat_type, modifier)
    
    /// The special effects
    pub special_effects: Vec<(u8, u8)>, // (effect_type, magnitude)
}

/// Seasonal event types.
pub enum SeasonalEventType {
    Spring = 0,
    Summer = 1,
    Autumn = 2,
    Winter = 3,
    Festival = 4,
    Holiday = 5,
    SpecialEvent = 6,
    Catastrophe = 7,
}

/// Special effect types.
pub enum SpecialEffectType {
    RainbowAura = 0,
    Sparkles = 1,
    Glow = 2,
    Frost = 3,
    Fire = 4,
    Leaves = 5,
    Flowers = 6,
    Ghosts = 7,
}

/// A system for managing seasonal events.
pub struct SeasonalEventSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> SeasonalEventSystem<T> {
    /// Starts a new seasonal event.
    /// 
    /// # Parameters
    /// 
    /// * `event_id` - The ID of the event
    /// * `duration` - The duration of the event in blocks
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn start_event(
        event_id: u32,
        duration: BlockNumberFor<T>,
    ) -> DispatchResult {
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Calculate the end time
        let end_time = current_block.saturating_add(duration);
        
        // Add the event to active events
        crate::ActiveSeasonalEvents::<T>::try_mutate(|events| -> DispatchResult {
            // Check if we've reached the maximum number of active events
            ensure!(
                events.len() < T::MaxActiveSeasonalEvents::get() as usize,
                Error::<T>::TooManyActiveEvents
            );
            
            // Check if the event is already active
            ensure!(
                !events.iter().any(|(id, _, _)| *id == event_id),
                Error::<T>::EventAlreadyActive
            );
            
            // Add the event
            events.try_push((event_id, current_block, end_time))
                .map_err(|_| Error::<T>::TooManyActiveEvents)?;
            
            Ok(())
        })?;
        
        // Get the event
        let event = Self::get_event(event_id)?;
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::SeasonalEventStarted {
            event_id,
            start_time: current_block,
            end_time,
        });
        
        Ok(())
    }
    
    /// Ends a seasonal event.
    /// 
    /// # Parameters
    /// 
    /// * `event_id` - The ID of the event
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn end_event(
        event_id: u32,
    ) -> DispatchResult {
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Remove the event from active events
        crate::ActiveSeasonalEvents::<T>::try_mutate(|events| -> DispatchResult {
            // Find the event
            let event_index = events.iter().position(|(id, _, _)| *id == event_id)
                .ok_or(Error::<T>::EventNotActive)?;
            
            // Remove the event
            events.remove(event_index);
            
            Ok(())
        })?;
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::SeasonalEventEnded {
            event_id,
            end_time: current_block,
        });
        
        Ok(())
    }
    
    /// Applies seasonal event effects to a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn apply_seasonal_effects(
        pet_id: PetId,
    ) -> DispatchResult {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get active events
        let active_events = crate::ActiveSeasonalEvents::<T>::get();
        
        // Apply effects from each active event
        for (event_id, _, _) in active_events {
            // Get the event
            let event = Self::get_event(event_id)?;
            
            // Apply the event's effects
            Self::apply_event_effects(pet_id, &event.effects)?;
            
            // Get the current block number
            let current_block = frame_system::Pallet::<T>::block_number();
            
            // Emit an event
            crate::Pallet::<T>::deposit_event(crate::Event::SeasonalEventEffect {
                pet_id,
                event_id,
                effect_type: 0, // General effect
                effect_magnitude: 100, // Full effect
                timestamp: current_block,
            });
        }
        
        Ok(())
    }
    
    /// Gets a seasonal event by ID.
    /// 
    /// # Parameters
    /// 
    /// * `event_id` - The ID of the event
    /// 
    /// # Returns
    /// 
    /// * `Result<SeasonalEvent<T>, DispatchError>` - The event, or an error
    fn get_event(
        event_id: u32,
    ) -> Result<SeasonalEvent<T>, DispatchError> {
        // In a real implementation, this would get the event from storage
        // For now, we'll just return a hardcoded event based on the ID
        
        let current_block = frame_system::Pallet::<T>::block_number();
        let end_time = current_block.saturating_add(1000.into()); // Arbitrary duration
        
        match event_id {
            0 => { // Spring
                Ok(SeasonalEvent {
                    id: event_id,
                    name: b"Spring Festival".to_vec(),
                    description: b"A celebration of new beginnings.".to_vec(),
                    start_time: current_block,
                    end_time,
                    effects: SeasonalEventEffects {
                        mood_modifier: 10,
                        experience_modifier: 5,
                        stat_modifiers: vec![
                            (1, 5), // Agility +5
                            (3, 5), // Vitality +5
                        ],
                        special_effects: vec![
                            (SpecialEffectType::Flowers as u8, 100), // Flowers effect at full intensity
                        ],
                    },
                })
            },
            1 => { // Summer
                Ok(SeasonalEvent {
                    id: event_id,
                    name: b"Summer Solstice".to_vec(),
                    description: b"The longest day of the year.".to_vec(),
                    start_time: current_block,
                    end_time,
                    effects: SeasonalEventEffects {
                        mood_modifier: 15,
                        experience_modifier: 10,
                        stat_modifiers: vec![
                            (0, 5), // Strength +5
                            (1, 5), // Agility +5
                        ],
                        special_effects: vec![
                            (SpecialEffectType::Glow as u8, 100), // Glow effect at full intensity
                        ],
                    },
                })
            },
            2 => { // Autumn
                Ok(SeasonalEvent {
                    id: event_id,
                    name: b"Autumn Harvest".to_vec(),
                    description: b"A time of abundance and preparation.".to_vec(),
                    start_time: current_block,
                    end_time,
                    effects: SeasonalEventEffects {
                        mood_modifier: 5,
                        experience_modifier: 15,
                        stat_modifiers: vec![
                            (2, 5), // Intelligence +5
                            (0, 5), // Strength +5
                        ],
                        special_effects: vec![
                            (SpecialEffectType::Leaves as u8, 100), // Leaves effect at full intensity
                        ],
                    },
                })
            },
            3 => { // Winter
                Ok(SeasonalEvent {
                    id: event_id,
                    name: b"Winter Solstice".to_vec(),
                    description: b"The longest night of the year.".to_vec(),
                    start_time: current_block,
                    end_time,
                    effects: SeasonalEventEffects {
                        mood_modifier: 0,
                        experience_modifier: 20,
                        stat_modifiers: vec![
                            (3, 10), // Vitality +10
                            (2, 5),  // Intelligence +5
                        ],
                        special_effects: vec![
                            (SpecialEffectType::Frost as u8, 100), // Frost effect at full intensity
                        ],
                    },
                })
            },
            _ => {
                // Default to a generic event
                Ok(SeasonalEvent {
                    id: event_id,
                    name: b"Special Event".to_vec(),
                    description: b"A special event.".to_vec(),
                    start_time: current_block,
                    end_time,
                    effects: SeasonalEventEffects {
                        mood_modifier: 5,
                        experience_modifier: 5,
                        stat_modifiers: vec![
                            (0, 2), // Strength +2
                            (1, 2), // Agility +2
                            (2, 2), // Intelligence +2
                            (3, 2), // Vitality +2
                        ],
                        special_effects: vec![
                            (SpecialEffectType::Sparkles as u8, 100), // Sparkles effect at full intensity
                        ],
                    },
                })
            }
        }
    }
    
    /// Applies the effects of a seasonal event to a pet.
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
        effects: &SeasonalEventEffects,
    ) -> DispatchResult {
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Apply mood modifier
            if effects.mood_modifier > 0 {
                pet.mood_indicator = pet.mood_indicator
                    .saturating_add(effects.mood_modifier as u8)
                    .min(T::MaxMoodValue::get());
            } else if effects.mood_modifier < 0 {
                pet.mood_indicator = pet.mood_indicator
                    .saturating_sub((-effects.mood_modifier) as u8);
            }
            
            // Apply stat modifiers
            for (stat_type, modifier) in &effects.stat_modifiers {
                match stat_type {
                    0 => { // Strength
                        if *modifier > 0 {
                            pet.base_strength = pet.base_strength.saturating_add(*modifier as u8);
                        } else if *modifier < 0 {
                            pet.base_strength = pet.base_strength.saturating_sub((-*modifier) as u8);
                        }
                    },
                    1 => { // Agility
                        if *modifier > 0 {
                            pet.base_agility = pet.base_agility.saturating_add(*modifier as u8);
                        } else if *modifier < 0 {
                            pet.base_agility = pet.base_agility.saturating_sub((-*modifier) as u8);
                        }
                    },
                    2 => { // Intelligence
                        if *modifier > 0 {
                            pet.base_intelligence = pet.base_intelligence.saturating_add(*modifier as u8);
                        } else if *modifier < 0 {
                            pet.base_intelligence = pet.base_intelligence.saturating_sub((-*modifier) as u8);
                        }
                    },
                    3 => { // Vitality
                        if *modifier > 0 {
                            pet.base_vitality = pet.base_vitality.saturating_add(*modifier as u8);
                        } else if *modifier < 0 {
                            pet.base_vitality = pet.base_vitality.saturating_sub((-*modifier) as u8);
                        }
                    },
                    _ => {} // No modifier for other stats
                }
            }
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })
    }
    
    /// Gets all active seasonal events.
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u32, BlockNumberFor<T>, BlockNumberFor<T>)>` - The active events (event_id, start_time, end_time)
    pub fn get_active_events() -> Vec<(u32, BlockNumberFor<T>, BlockNumberFor<T>)> {
        crate::ActiveSeasonalEvents::<T>::get().to_vec()
    }
    
    /// Gets detailed information about all active seasonal events.
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<SeasonalEvent<T>>, DispatchError>` - The active events, or an error
    pub fn get_active_event_details() -> Result<Vec<SeasonalEvent<T>>, DispatchError> {
        let active_events = crate::ActiveSeasonalEvents::<T>::get();
        
        let mut result = Vec::with_capacity(active_events.len());
        for (event_id, _, _) in active_events {
            let event = Self::get_event(event_id)?;
            result.push(event);
        }
        
        Ok(result)
    }
    
    /// Checks if a seasonal event is active.
    /// 
    /// # Parameters
    /// 
    /// * `event_id` - The ID of the event
    /// 
    /// # Returns
    /// 
    /// * `bool` - Whether the event is active
    pub fn is_event_active(
        event_id: u32,
    ) -> bool {
        let active_events = crate::ActiveSeasonalEvents::<T>::get();
        
        active_events.iter().any(|(id, _, _)| *id == event_id)
    }
    
    /// Updates active seasonal events, ending those that have expired.
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn update_active_events() -> DispatchResult {
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Get active events
        let active_events = crate::ActiveSeasonalEvents::<T>::get();
        
        // Check each event
        for (event_id, _, end_time) in active_events {
            // If the event has expired, end it
            if current_block >= end_time {
                Self::end_event(event_id)?;
            }
        }
        
        Ok(())
    }
}