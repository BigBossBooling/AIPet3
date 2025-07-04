//! # Mood Contagion System
//!
//! This module provides a system for pets to influence each other's moods when they interact,
//! creating interesting social dynamics where happy pets can cheer up sad ones (or vice versa).

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

/// A system for managing mood contagion between pets.
pub struct MoodContagionSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> MoodContagionSystem<T> {
    /// Processes mood contagion between two pets.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id_1` - The ID of the first pet
    /// * `pet_id_2` - The ID of the second pet
    /// * `interaction_duration` - The duration of the interaction
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn process_mood_contagion(
        pet_id_1: PetId,
        pet_id_2: PetId,
        interaction_duration: u32,
    ) -> DispatchResult {
        // Ensure the pets are different
        ensure!(pet_id_1 != pet_id_2, Error::<T>::InvalidPetState);
        
        // Get the pets from storage
        let pet1 = crate::PetNfts::<T>::get(pet_id_1).ok_or(Error::<T>::PetNotFound)?;
        let pet2 = crate::PetNfts::<T>::get(pet_id_2).ok_or(Error::<T>::PetNotFound)?;
        
        // Calculate the mood contagion
        let (mood_change_1, mood_change_2) = Self::calculate_mood_contagion(
            &pet1,
            &pet2,
            interaction_duration,
        )?;
        
        // Apply the mood changes
        Self::apply_mood_change(pet_id_1, mood_change_1)?;
        Self::apply_mood_change(pet_id_2, mood_change_2)?;
        
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::MoodContagion {
            pet_id_1,
            pet_id_2,
            mood_change_1,
            mood_change_2,
            timestamp: current_block,
        });
        
        Ok(())
    }
    
    /// Calculates the mood contagion between two pets.
    /// 
    /// # Parameters
    /// 
    /// * `pet1` - The first pet
    /// * `pet2` - The second pet
    /// * `interaction_duration` - The duration of the interaction
    /// 
    /// # Returns
    /// 
    /// * `Result<(i8, i8), DispatchError>` - The mood changes for pet1 and pet2, or an error
    fn calculate_mood_contagion(
        pet1: &PetNft<T>,
        pet2: &PetNft<T>,
        interaction_duration: u32,
    ) -> Result<(i8, i8), DispatchError> {
        // In a real implementation, this would use a complex algorithm
        // to calculate mood contagion based on various factors
        // For now, we'll use a simple algorithm
        
        // Calculate the mood difference
        let mood_diff = pet1.mood_indicator as i16 - pet2.mood_indicator as i16;
        
        // Calculate the contagion strength
        let contagion_strength = (interaction_duration as u16).min(255) as u8;
        
        // Calculate the mood changes
        let mut mood_change_1 = 0i8;
        let mut mood_change_2 = 0i8;
        
        if mood_diff > 50 {
            // Pet1 is happier than pet2
            // Pet1's mood decreases slightly, pet2's mood increases
            mood_change_1 = -((contagion_strength as u16 * 10 / 255) as i8).min(T::MaxMoodChangeFromSocialInteraction::get() as i8);
            mood_change_2 = ((contagion_strength as u16 * 20 / 255) as i8).min(T::MaxMoodChangeFromSocialInteraction::get() as i8);
        } else if mood_diff < -50 {
            // Pet2 is happier than pet1
            // Pet2's mood decreases slightly, pet1's mood increases
            mood_change_1 = ((contagion_strength as u16 * 20 / 255) as i8).min(T::MaxMoodChangeFromSocialInteraction::get() as i8);
            mood_change_2 = -((contagion_strength as u16 * 10 / 255) as i8).min(T::MaxMoodChangeFromSocialInteraction::get() as i8);
        } else {
            // Pets have similar moods
            // Both pets' moods converge slightly
            let average_mood = (pet1.mood_indicator as u16 + pet2.mood_indicator as u16) / 2;
            let pet1_diff = average_mood as i16 - pet1.mood_indicator as i16;
            let pet2_diff = average_mood as i16 - pet2.mood_indicator as i16;
            
            mood_change_1 = ((pet1_diff * contagion_strength as i16 / 255) as i8).min(T::MaxMoodChangeFromSocialInteraction::get() as i8);
            mood_change_2 = ((pet2_diff * contagion_strength as i16 / 255) as i8).min(T::MaxMoodChangeFromSocialInteraction::get() as i8);
        }
        
        Ok((mood_change_1, mood_change_2))
    }
    
    /// Applies a mood change to a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `mood_change` - The change in mood
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn apply_mood_change(
        pet_id: PetId,
        mood_change: i8,
    ) -> DispatchResult {
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Apply mood change
            if mood_change > 0 {
                pet.mood_indicator = pet.mood_indicator
                    .saturating_add(mood_change as u8)
                    .min(T::MaxMoodValue::get());
            } else if mood_change < 0 {
                pet.mood_indicator = pet.mood_indicator
                    .saturating_sub((-mood_change) as u8);
            }
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })
    }
    
    /// Calculates the mood contagion susceptibility of a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The susceptibility (0-255), or an error
    pub fn calculate_susceptibility(
        pet_id: PetId,
    ) -> Result<u8, DispatchError> {
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // In a real implementation, this would calculate susceptibility
        // based on various factors such as personality traits, etc.
        // For now, we'll use a simple algorithm
        
        // Base susceptibility
        let mut susceptibility = 128; // 50%
        
        // Adjust based on intelligence
        if pet.base_intelligence > 20 {
            // High intelligence: -20% susceptibility (more resistant)
            susceptibility = susceptibility.saturating_sub(51);
        } else if pet.base_intelligence < 10 {
            // Low intelligence: +20% susceptibility (more susceptible)
            susceptibility = susceptibility.saturating_add(51);
        }
        
        // Adjust based on mood
        if pet.mood_indicator > 200 {
            // Very happy: -10% susceptibility (more resistant)
            susceptibility = susceptibility.saturating_sub(25);
        } else if pet.mood_indicator < 100 {
            // Unhappy: +10% susceptibility (more susceptible)
            susceptibility = susceptibility.saturating_add(25);
        }
        
        Ok(susceptibility)
    }
    
    /// Calculates the mood contagion influence of a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The influence (0-255), or an error
    pub fn calculate_influence(
        pet_id: PetId,
    ) -> Result<u8, DispatchError> {
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // In a real implementation, this would calculate influence
        // based on various factors such as personality traits, etc.
        // For now, we'll use a simple algorithm
        
        // Base influence
        let mut influence = 128; // 50%
        
        // Adjust based on charisma (using vitality as a proxy for now)
        if pet.base_vitality > 20 {
            // High charisma: +20% influence
            influence = influence.saturating_add(51);
        } else if pet.base_vitality < 10 {
            // Low charisma: -20% influence
            influence = influence.saturating_sub(51);
        }
        
        // Adjust based on mood
        if pet.mood_indicator > 200 {
            // Very happy: +20% influence
            influence = influence.saturating_add(51);
        } else if pet.mood_indicator < 100 {
            // Unhappy: -20% influence
            influence = influence.saturating_sub(51);
        }
        
        Ok(influence)
    }
}