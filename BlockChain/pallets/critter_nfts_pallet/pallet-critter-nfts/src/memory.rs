//! # Pet Memory System
//!
//! This module provides a system for pets to remember significant events and interactions,
//! creating emotional connections as pets "remember" important moments with their owners.

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

/// Represents a memory that a pet has formed.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Memory<T: Config> {
    /// The memory type
    pub memory_type: u8,
    
    /// The significance of the memory (0-255)
    pub significance: u8,
    
    /// The timestamp when the memory was formed
    pub timestamp: BlockNumberFor<T>,
    
    /// The emotional impact of the memory (-128 to 127)
    pub emotional_impact: i8,
    
    /// The clarity of the memory (0-255)
    /// This decreases over time unless the memory is reinforced
    pub clarity: u8,
    
    /// Additional data associated with the memory
    pub associated_data: BoundedVec<u8, T::MaxMemoryDataSize>,
}

/// Memory types.
pub enum MemoryType {
    Feeding = 0,
    Playing = 1,
    Training = 2,
    SocialInteraction = 3,
    EnvironmentalAdaptation = 4,
    Achievement = 5,
    LifecycleEvent = 6,
    OwnerBonding = 7,
    Neglect = 8,
    Trauma = 9,
    Joy = 10,
    Discovery = 11,
}

/// A system for managing pet memories.
pub struct PetMemorySystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> PetMemorySystem<T> {
    /// Records a new memory for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `memory_type` - The type of memory
    /// * `significance` - The significance of the memory
    /// * `associated_data` - Additional data associated with the memory
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn record_memory(
        pet_id: PetId,
        memory_type: u8,
        significance: u8,
        associated_data: Vec<u8>,
    ) -> DispatchResult {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Calculate the emotional impact of the memory
        let emotional_impact = Self::calculate_emotional_impact(memory_type, significance, &pet);
        
        // Create the memory
        let memory_data = (
            memory_type,
            significance,
            current_block.saturated_into::<u64>(),
            associated_data.clone(),
        );
        
        // Store the memory
        crate::PetMemories::<T>::try_mutate(pet_id, |memories| -> DispatchResult {
            // Check if we've reached the maximum number of memories
            if memories.len() >= T::MaxPetMemories::get() as usize {
                // Find the least significant memory
                let mut least_significant_index = 0;
                let mut least_significance = u8::MAX;
                
                for i in 0..memories.len() {
                    if let Some((_, sig, _, _)) = memories.get(i) {
                        if *sig < least_significance {
                            least_significant_index = i;
                            least_significance = *sig;
                        }
                    }
                }
                
                // If the new memory is more significant than the least significant one,
                // replace it
                if significance > least_significance {
                    memories.set(least_significant_index, memory_data)?;
                } else {
                    // Otherwise, don't store the new memory
                    return Ok(());
                }
            } else {
                // Add the new memory
                memories.try_push(memory_data)
                    .map_err(|_| Error::<T>::MemoryCapacityFull)?;
            }
            
            Ok(())
        })?;
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::MemoryFormed {
            pet_id,
            memory_type,
            significance,
            timestamp: current_block,
        });
        
        // Potentially evolve personality traits based on the memory
        if significance > T::MaxMemorySignificance::get() / 2 {
            // Significant memory: evolve a relevant trait
            let trait_type = match memory_type {
                0 => crate::personality::PersonalityTraitType::Grateful as u8, // Feeding
                1 => crate::personality::PersonalityTraitType::Playful as u8, // Playing
                2 => crate::personality::PersonalityTraitType::Intelligent as u8, // Training
                3 => crate::personality::PersonalityTraitType::Friendly as u8, // Social Interaction
                _ => crate::personality::PersonalityTraitType::Adaptable as u8, // Default
            };
            
            crate::personality::PersonalityEvolutionSystem::<T>::evolve_personality(
                pet_id,
                crate::personality::EvolutionCatalyst::LifecycleEvent as u8,
                significance,
            )?;
        }
        
        Ok(())
    }
    
    /// Calculates the emotional impact of a memory.
    /// 
    /// # Parameters
    /// 
    /// * `memory_type` - The type of memory
    /// * `significance` - The significance of the memory
    /// * `pet` - The pet
    /// 
    /// # Returns
    /// 
    /// * `i8` - The emotional impact of the memory (-128 to 127)
    fn calculate_emotional_impact(
        memory_type: u8,
        significance: u8,
        pet: &PetNft<T>,
    ) -> i8 {
        // In a real implementation, this would calculate the emotional impact
        // based on various factors such as pet personality, mood, etc.
        // For now, we'll use a simple algorithm
        
        // Base emotional impact
        let mut impact = 0i8;
        
        // Adjust based on memory type
        match memory_type {
            0 => impact += 20, // Feeding: positive
            1 => impact += 30, // Playing: very positive
            2 => impact += 10, // Training: slightly positive
            3 => impact += 20, // Social Interaction: positive
            4 => impact += 10, // Environmental Adaptation: slightly positive
            5 => impact += 40, // Achievement: extremely positive
            6 => impact += 30, // Lifecycle Event: very positive
            7 => impact += 50, // Owner Bonding: extremely positive
            8 => impact -= 30, // Neglect: very negative
            9 => impact -= 50, // Trauma: extremely negative
            10 => impact += 40, // Joy: extremely positive
            11 => impact += 20, // Discovery: positive
            _ => {} // No adjustment for other types
        }
        
        // Adjust based on significance
        let significance_factor = (significance as i16 - 128) / 4;
        impact = impact.saturating_add(significance_factor as i8);
        
        // Adjust based on pet mood
        let mood_factor = (pet.mood_indicator as i16 - 128) / 8;
        impact = impact.saturating_add(mood_factor as i8);
        
        impact
    }
    
    /// Reinforces a memory, increasing its clarity and potentially its significance.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `memory_index` - The index of the memory to reinforce
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn reinforce_memory(
        pet_id: PetId,
        memory_index: usize,
    ) -> DispatchResult {
        crate::PetMemories::<T>::try_mutate(pet_id, |memories| -> DispatchResult {
            // Ensure the memory exists
            ensure!(
                memory_index < memories.len(),
                Error::<T>::MemoryNotFound
            );
            
            // Get the memory
            let (memory_type, significance, timestamp, associated_data) = 
                memories.get(memory_index).ok_or(Error::<T>::MemoryNotFound)?.clone();
            
            // Increase the significance
            let new_significance = significance.saturating_add(10).min(T::MaxMemorySignificance::get());
            
            // Update the memory
            memories.set(memory_index, (memory_type, new_significance, timestamp, associated_data))?;
            
            Ok(())
        })
    }
    
    /// Fades memories over time, reducing their clarity.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn fade_memories(
        pet_id: PetId,
    ) -> DispatchResult {
        crate::PetMemories::<T>::try_mutate(pet_id, |memories| -> DispatchResult {
            // Get the current block number
            let current_block = frame_system::Pallet::<T>::block_number();
            
            // Fade each memory
            for i in 0..memories.len() {
                if let Some((memory_type, significance, timestamp, associated_data)) = memories.get(i) {
                    // Calculate the age of the memory
                    let age = current_block.saturated_into::<u64>().saturating_sub(*timestamp);
                    
                    // Calculate the fade amount based on age and significance
                    let fade_amount = (age / 1000) as u8; // Arbitrary fade rate
                    let fade_resistance = *significance / 10; // Higher significance = slower fading
                    let actual_fade = fade_amount.saturating_sub(fade_resistance);
                    
                    // Calculate the new significance
                    let new_significance = significance.saturating_sub(actual_fade);
                    
                    // Update the memory
                    memories.set(i, (*memory_type, new_significance, *timestamp, associated_data.clone()))?;
                }
            }
            
            Ok(())
        })
    }
    
    /// Retrieves a specific memory.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `memory_index` - The index of the memory to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Result<(u8, u8, u64, Vec<u8>), DispatchError>` - The memory, or an error
    pub fn get_memory(
        pet_id: PetId,
        memory_index: usize,
    ) -> Result<(u8, u8, u64, Vec<u8>), DispatchError> {
        let memories = crate::PetMemories::<T>::get(pet_id);
        
        // Ensure the memory exists
        ensure!(
            memory_index < memories.len(),
            Error::<T>::MemoryNotFound
        );
        
        // Get the memory
        let (memory_type, significance, timestamp, associated_data) = 
            memories.get(memory_index).ok_or(Error::<T>::MemoryNotFound)?.clone();
        
        Ok((memory_type, significance, timestamp, associated_data.to_vec()))
    }
    
    /// Gets all of a pet's memories.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u8, u8, u64, Vec<u8>)>` - The memories
    pub fn get_all_memories(
        pet_id: PetId,
    ) -> Vec<(u8, u8, u64, Vec<u8>)> {
        let memories = crate::PetMemories::<T>::get(pet_id);
        
        let mut result = Vec::with_capacity(memories.len());
        for (memory_type, significance, timestamp, associated_data) in memories.iter() {
            result.push((*memory_type, *significance, *timestamp, associated_data.to_vec()));
        }
        
        result
    }
    
    /// Gets a pet's most significant memories.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `count` - The maximum number of memories to return
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u8, u8, u64, Vec<u8>)>` - The memories
    pub fn get_significant_memories(
        pet_id: PetId,
        count: usize,
    ) -> Vec<(u8, u8, u64, Vec<u8>)> {
        let memories = crate::PetMemories::<T>::get(pet_id);
        
        // Sort memories by significance (highest first)
        let mut sorted_memories = memories.to_vec();
        sorted_memories.sort_by(|(_, a, _, _), (_, b, _, _)| b.cmp(a));
        
        // Take the top `count` memories
        let mut result = Vec::with_capacity(count.min(sorted_memories.len()));
        for (memory_type, significance, timestamp, associated_data) in sorted_memories.iter().take(count) {
            result.push((*memory_type, *significance, *timestamp, associated_data.to_vec()));
        }
        
        result
    }
    
    /// Gets a pet's memories of a specific type.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `memory_type` - The type of memories to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u8, u8, u64, Vec<u8>)>` - The memories
    pub fn get_memories_by_type(
        pet_id: PetId,
        memory_type: u8,
    ) -> Vec<(u8, u8, u64, Vec<u8>)> {
        let memories = crate::PetMemories::<T>::get(pet_id);
        
        let mut result = Vec::new();
        for (mem_type, significance, timestamp, associated_data) in memories.iter() {
            if *mem_type == memory_type {
                result.push((*mem_type, *significance, *timestamp, associated_data.to_vec()));
            }
        }
        
        result
    }
}