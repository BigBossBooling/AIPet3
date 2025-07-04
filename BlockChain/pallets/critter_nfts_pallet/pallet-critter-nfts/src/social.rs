//! # Cross-Pet Social Interactions
//!
//! This module provides a system for pets to interact with each other,
//! forming friendships, rivalries, and other social bonds that add depth to the ecosystem.

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

/// Represents a social interaction between two pets.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SocialInteraction<T: Config> {
    /// The ID of the first pet
    pub pet_id_1: PetId,
    
    /// The ID of the second pet
    pub pet_id_2: PetId,
    
    /// The type of interaction
    pub interaction_type: u8,
    
    /// The outcome of the interaction
    pub outcome: u8,
    
    /// The timestamp when the interaction occurred
    pub timestamp: BlockNumberFor<T>,
    
    /// The duration of the interaction
    pub duration: u32,
    
    /// The mood change for the first pet
    pub mood_change_1: i8,
    
    /// The mood change for the second pet
    pub mood_change_2: i8,
}

/// Represents a social bond between two pets.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SocialBond {
    /// The ID of the other pet
    pub other_pet_id: PetId,
    
    /// The type of bond
    pub bond_type: u8,
    
    /// The strength of the bond (0-255)
    pub bond_strength: u8,
    
    /// The number of interactions that have occurred
    pub interaction_count: u32,
    
    /// The timestamp of the last interaction
    pub last_interaction: u64,
}

/// Social interaction types.
pub enum InteractionType {
    Play = 0,
    Greet = 1,
    Share = 2,
    Compete = 3,
    Teach = 4,
    Learn = 5,
    Comfort = 6,
    Protect = 7,
}

/// Social bond types.
pub enum BondType {
    Friend = 0,
    Rival = 1,
    Mentor = 2,
    Student = 3,
    Partner = 4,
    Guardian = 5,
    Dependent = 6,
    Neutral = 7,
}

/// Interaction outcomes.
pub enum InteractionOutcome {
    VeryPositive = 0,
    Positive = 1,
    Neutral = 2,
    Negative = 3,
    VeryNegative = 4,
}

/// A system for managing social interactions between pets.
pub struct SocialInteractionSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> SocialInteractionSystem<T> {
    /// Initiates a social interaction between two pets.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id_1` - The ID of the first pet
    /// * `pet_id_2` - The ID of the second pet
    /// * `interaction_type` - The type of interaction
    /// * `duration` - The duration of the interaction
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn interact(
        pet_id_1: PetId,
        pet_id_2: PetId,
        interaction_type: u8,
        duration: u32,
    ) -> DispatchResult {
        // Ensure the pets are different
        ensure!(pet_id_1 != pet_id_2, Error::<T>::InvalidPetState);
        
        // Get the pets from storage
        let pet1 = crate::PetNfts::<T>::get(pet_id_1).ok_or(Error::<T>::PetNotFound)?;
        let pet2 = crate::PetNfts::<T>::get(pet_id_2).ok_or(Error::<T>::PetNotFound)?;
        
        // Check compatibility
        Self::check_compatibility(&pet1, &pet2, interaction_type)?;
        
        // Calculate the outcome of the interaction
        let (outcome, mood_change_1, mood_change_2) = Self::calculate_outcome(
            &pet1,
            &pet2,
            interaction_type,
            duration,
        )?;
        
        // Apply the effects of the interaction
        Self::apply_interaction_effects(
            pet_id_1,
            pet_id_2,
            interaction_type,
            outcome,
            mood_change_1,
            mood_change_2,
        )?;
        
        // Update the social bond between the pets
        Self::update_social_bond(
            pet_id_1,
            pet_id_2,
            interaction_type,
            outcome,
        )?;
        
        // Record the interaction
        Self::record_interaction(
            pet_id_1,
            pet_id_2,
            interaction_type,
            outcome,
            duration,
            mood_change_1,
            mood_change_2,
        )?;
        
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::SocialInteraction {
            pet_id_1,
            pet_id_2,
            interaction_type,
            outcome,
            timestamp: current_block,
        });
        
        // Potentially evolve personality traits based on the interaction
        if outcome == InteractionOutcome::VeryPositive as u8 || outcome == InteractionOutcome::VeryNegative as u8 {
            // Significant outcome: evolve the "Social" trait
            crate::personality::PersonalityEvolutionSystem::<T>::evolve_personality(
                pet_id_1,
                crate::personality::EvolutionCatalyst::SocialInteraction as u8,
                if outcome == InteractionOutcome::VeryPositive as u8 { 200 } else { 100 },
            )?;
            
            crate::personality::PersonalityEvolutionSystem::<T>::evolve_personality(
                pet_id_2,
                crate::personality::EvolutionCatalyst::SocialInteraction as u8,
                if outcome == InteractionOutcome::VeryPositive as u8 { 200 } else { 100 },
            )?;
        }
        
        Ok(())
    }
    
    /// Checks if two pets are compatible for a social interaction.
    /// 
    /// # Parameters
    /// 
    /// * `pet1` - The first pet
    /// * `pet2` - The second pet
    /// * `interaction_type` - The type of interaction
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if compatible, Err otherwise
    fn check_compatibility(
        pet1: &PetNft<T>,
        pet2: &PetNft<T>,
        interaction_type: u8,
    ) -> DispatchResult {
        // In a real implementation, this would check various factors
        // such as personality traits, elemental affinities, etc.
        // For now, we'll just do a simple check
        
        // Check if the pets have opposite elemental affinities
        if (pet1.primary_elemental_affinity as u8 + pet2.primary_elemental_affinity as u8) % 8 == 4 {
            // Opposite elements: 50% chance of incompatibility for competitive interactions
            if interaction_type == InteractionType::Compete as u8 {
                let (random_seed, _) = T::PetRandomness::random_seed();
                let random_value = random_seed.using_encoded(|encoded| {
                    let mut buf = [0u8; 4];
                    buf.copy_from_slice(&encoded[0..4]);
                    u32::from_le_bytes(buf)
                });
                
                if random_value % 2 == 0 {
                    return Err(Error::<T>::IncompatiblePets.into());
                }
            }
        }
        
        // Check if the pets have the same elemental affinity
        if pet1.primary_elemental_affinity as u8 == pet2.primary_elemental_affinity as u8 {
            // Same element: 50% chance of incompatibility for competitive interactions
            if interaction_type == InteractionType::Compete as u8 {
                let (random_seed, _) = T::PetRandomness::random_seed();
                let random_value = random_seed.using_encoded(|encoded| {
                    let mut buf = [0u8; 4];
                    buf.copy_from_slice(&encoded[0..4]);
                    u32::from_le_bytes(buf)
                });
                
                if random_value % 2 == 0 {
                    return Err(Error::<T>::IncompatiblePets.into());
                }
            }
        }
        
        Ok(())
    }
    
    /// Calculates the outcome of a social interaction.
    /// 
    /// # Parameters
    /// 
    /// * `pet1` - The first pet
    /// * `pet2` - The second pet
    /// * `interaction_type` - The type of interaction
    /// * `duration` - The duration of the interaction
    /// 
    /// # Returns
    /// 
    /// * `Result<(u8, i8, i8), DispatchError>` - The outcome, mood change for pet1, and mood change for pet2, or an error
    fn calculate_outcome(
        pet1: &PetNft<T>,
        pet2: &PetNft<T>,
        interaction_type: u8,
        duration: u32,
    ) -> Result<(u8, i8, i8), DispatchError> {
        // In a real implementation, this would use a complex algorithm
        // to calculate the outcome based on various factors
        // For now, we'll use a simple algorithm
        
        // Base outcome
        let mut outcome = InteractionOutcome::Neutral as u8;
        
        // Adjust based on mood
        if pet1.mood_indicator > 200 && pet2.mood_indicator > 200 {
            // Both pets are very happy
            outcome = outcome.saturating_sub(1); // More positive
        } else if pet1.mood_indicator < 100 && pet2.mood_indicator < 100 {
            // Both pets are unhappy
            outcome = outcome.saturating_add(1); // More negative
        }
        
        // Adjust based on elemental affinity
        if pet1.primary_elemental_affinity as u8 == pet2.primary_elemental_affinity as u8 {
            // Same element: more positive for cooperative interactions, more negative for competitive
            if interaction_type == InteractionType::Play as u8 || 
               interaction_type == InteractionType::Share as u8 ||
               interaction_type == InteractionType::Comfort as u8 {
                outcome = outcome.saturating_sub(1); // More positive
            } else if interaction_type == InteractionType::Compete as u8 {
                outcome = outcome.saturating_add(1); // More negative
            }
        } else if (pet1.primary_elemental_affinity as u8 + pet2.primary_elemental_affinity as u8) % 8 == 4 {
            // Opposite elements: more negative for cooperative interactions, more positive for competitive
            if interaction_type == InteractionType::Play as u8 || 
               interaction_type == InteractionType::Share as u8 ||
               interaction_type == InteractionType::Comfort as u8 {
                outcome = outcome.saturating_add(1); // More negative
            } else if interaction_type == InteractionType::Compete as u8 {
                outcome = outcome.saturating_sub(1); // More positive
            }
        }
        
        // Adjust based on interaction type
        match interaction_type {
            0 => { // Play
                // Playing is generally positive
                outcome = outcome.saturating_sub(1); // More positive
            },
            1 => { // Greet
                // Greeting is neutral to positive
                if pet1.mood_indicator > 150 && pet2.mood_indicator > 150 {
                    outcome = outcome.saturating_sub(1); // More positive
                }
            },
            2 => { // Share
                // Sharing is generally positive
                outcome = outcome.saturating_sub(1); // More positive
            },
            3 => { // Compete
                // Competition can be positive or negative
                let (random_seed, _) = T::PetRandomness::random_seed();
                let random_value = random_seed.using_encoded(|encoded| {
                    let mut buf = [0u8; 4];
                    buf.copy_from_slice(&encoded[0..4]);
                    u32::from_le_bytes(buf)
                });
                
                if random_value % 2 == 0 {
                    outcome = outcome.saturating_sub(1); // More positive
                } else {
                    outcome = outcome.saturating_add(1); // More negative
                }
            },
            _ => {} // Other interaction types are neutral
        }
        
        // Calculate mood changes
        let (mood_change_1, mood_change_2) = match outcome {
            0 => { // Very positive
                let base_change = (duration as u16 * 20 / 100) as i8;
                (base_change, base_change)
            },
            1 => { // Positive
                let base_change = (duration as u16 * 10 / 100) as i8;
                (base_change, base_change)
            },
            2 => { // Neutral
                (0, 0)
            },
            3 => { // Negative
                let base_change = -((duration as u16 * 10 / 100) as i8);
                (base_change, base_change)
            },
            4 => { // Very negative
                let base_change = -((duration as u16 * 20 / 100) as i8);
                (base_change, base_change)
            },
            _ => (0, 0), // Default
        };
        
        Ok((outcome, mood_change_1, mood_change_2))
    }
    
    /// Applies the effects of a social interaction.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id_1` - The ID of the first pet
    /// * `pet_id_2` - The ID of the second pet
    /// * `interaction_type` - The type of interaction
    /// * `outcome` - The outcome of the interaction
    /// * `mood_change_1` - The mood change for the first pet
    /// * `mood_change_2` - The mood change for the second pet
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn apply_interaction_effects(
        pet_id_1: PetId,
        pet_id_2: PetId,
        interaction_type: u8,
        outcome: u8,
        mood_change_1: i8,
        mood_change_2: i8,
    ) -> DispatchResult {
        // Apply effects to the first pet
        crate::PetNfts::<T>::try_mutate(pet_id_1, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Apply mood change
            if mood_change_1 > 0 {
                pet.mood_indicator = pet.mood_indicator
                    .saturating_add(mood_change_1 as u8)
                    .min(T::MaxMoodValue::get());
            } else if mood_change_1 < 0 {
                pet.mood_indicator = pet.mood_indicator
                    .saturating_sub((-mood_change_1) as u8);
            }
            
            // Apply experience gain
            let xp_gain = match outcome {
                0 => 50, // Very positive
                1 => 30, // Positive
                2 => 10, // Neutral
                _ => 5,  // Negative or very negative
            };
            
            pet.experience_points = pet.experience_points.saturating_add(xp_gain);
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })?;
        
        // Apply effects to the second pet
        crate::PetNfts::<T>::try_mutate(pet_id_2, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Apply mood change
            if mood_change_2 > 0 {
                pet.mood_indicator = pet.mood_indicator
                    .saturating_add(mood_change_2 as u8)
                    .min(T::MaxMoodValue::get());
            } else if mood_change_2 < 0 {
                pet.mood_indicator = pet.mood_indicator
                    .saturating_sub((-mood_change_2) as u8);
            }
            
            // Apply experience gain
            let xp_gain = match outcome {
                0 => 50, // Very positive
                1 => 30, // Positive
                2 => 10, // Neutral
                _ => 5,  // Negative or very negative
            };
            
            pet.experience_points = pet.experience_points.saturating_add(xp_gain);
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })?;
        
        Ok(())
    }
    
    /// Updates the social bond between two pets.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id_1` - The ID of the first pet
    /// * `pet_id_2` - The ID of the second pet
    /// * `interaction_type` - The type of interaction
    /// * `outcome` - The outcome of the interaction
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn update_social_bond(
        pet_id_1: PetId,
        pet_id_2: PetId,
        interaction_type: u8,
        outcome: u8,
    ) -> DispatchResult {
        // Update the bond from pet1 to pet2
        Self::update_one_way_bond(pet_id_1, pet_id_2, interaction_type, outcome)?;
        
        // Update the bond from pet2 to pet1
        Self::update_one_way_bond(pet_id_2, pet_id_1, interaction_type, outcome)?;
        
        Ok(())
    }
    
    /// Updates a one-way social bond between two pets.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet whose bond is being updated
    /// * `other_pet_id` - The ID of the other pet
    /// * `interaction_type` - The type of interaction
    /// * `outcome` - The outcome of the interaction
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn update_one_way_bond(
        pet_id: PetId,
        other_pet_id: PetId,
        interaction_type: u8,
        outcome: u8,
    ) -> DispatchResult {
        crate::PetSocialBonds::<T>::try_mutate(pet_id, |bonds| -> DispatchResult {
            // Find the bond with the other pet
            let bond_index = bonds.iter().position(|bond| bond.other_pet_id == other_pet_id);
            
            // Get the current timestamp
            let current_time = frame_system::Pallet::<T>::block_number().saturated_into::<u64>();
            
            if let Some(index) = bond_index {
                // Update the existing bond
                let bond = &mut bonds[index];
                
                // Update the bond type based on the interaction
                bond.bond_type = Self::determine_bond_type(
                    bond.bond_type,
                    interaction_type,
                    outcome,
                );
                
                // Update the bond strength
                bond.bond_strength = Self::calculate_bond_strength(
                    bond.bond_strength,
                    outcome,
                );
                
                // Update the interaction count
                bond.interaction_count = bond.interaction_count.saturating_add(1);
                
                // Update the last interaction timestamp
                bond.last_interaction = current_time;
            } else {
                // Create a new bond
                let bond_type = Self::determine_initial_bond_type(
                    interaction_type,
                    outcome,
                );
                
                let bond_strength = Self::calculate_initial_bond_strength(
                    outcome,
                );
                
                let new_bond = SocialBond {
                    other_pet_id,
                    bond_type,
                    bond_strength,
                    interaction_count: 1,
                    last_interaction: current_time,
                };
                
                // Add the new bond
                bonds.try_push(new_bond)
                    .map_err(|_| Error::<T>::TooManySocialBonds)?;
            }
            
            Ok(())
        })
    }
    
    /// Determines the initial bond type based on the interaction.
    /// 
    /// # Parameters
    /// 
    /// * `interaction_type` - The type of interaction
    /// * `outcome` - The outcome of the interaction
    /// 
    /// # Returns
    /// 
    /// * `u8` - The bond type
    fn determine_initial_bond_type(
        interaction_type: u8,
        outcome: u8,
    ) -> u8 {
        match (interaction_type, outcome) {
            (0, 0) | (0, 1) | (2, 0) | (2, 1) => BondType::Friend as u8, // Play or Share with positive outcome
            (3, 0) | (3, 1) => BondType::Rival as u8, // Compete with positive outcome
            (4, 0) | (4, 1) => BondType::Mentor as u8, // Teach with positive outcome
            (5, 0) | (5, 1) => BondType::Student as u8, // Learn with positive outcome
            (6, 0) | (6, 1) => BondType::Guardian as u8, // Comfort with positive outcome
            (7, 0) | (7, 1) => BondType::Protector as u8, // Protect with positive outcome
            (_, 3) | (_, 4) => BondType::Rival as u8, // Any interaction with negative outcome
            _ => BondType::Neutral as u8, // Default
        }
    }
    
    /// Determines the bond type based on the current bond, interaction, and outcome.
    /// 
    /// # Parameters
    /// 
    /// * `current_bond_type` - The current bond type
    /// * `interaction_type` - The type of interaction
    /// * `outcome` - The outcome of the interaction
    /// 
    /// # Returns
    /// 
    /// * `u8` - The new bond type
    fn determine_bond_type(
        current_bond_type: u8,
        interaction_type: u8,
        outcome: u8,
    ) -> u8 {
        // In a real implementation, this would use a complex algorithm
        // to determine the bond type based on the current bond, interaction, and outcome
        // For now, we'll just return the current bond type
        current_bond_type
    }
    
    /// Calculates the initial bond strength based on the outcome.
    /// 
    /// # Parameters
    /// 
    /// * `outcome` - The outcome of the interaction
    /// 
    /// # Returns
    /// 
    /// * `u8` - The bond strength
    fn calculate_initial_bond_strength(
        outcome: u8,
    ) -> u8 {
        match outcome {
            0 => 100, // Very positive
            1 => 75,  // Positive
            2 => 50,  // Neutral
            3 => 25,  // Negative
            4 => 10,  // Very negative
            _ => 50,  // Default
        }
    }
    
    /// Calculates the bond strength based on the current strength and outcome.
    /// 
    /// # Parameters
    /// 
    /// * `current_strength` - The current bond strength
    /// * `outcome` - The outcome of the interaction
    /// 
    /// # Returns
    /// 
    /// * `u8` - The new bond strength
    fn calculate_bond_strength(
        current_strength: u8,
        outcome: u8,
    ) -> u8 {
        match outcome {
            0 => current_strength.saturating_add(20).min(255), // Very positive
            1 => current_strength.saturating_add(10).min(255), // Positive
            2 => current_strength, // Neutral
            3 => current_strength.saturating_sub(10), // Negative
            4 => current_strength.saturating_sub(20), // Very negative
            _ => current_strength, // Default
        }
    }
    
    /// Records a social interaction.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id_1` - The ID of the first pet
    /// * `pet_id_2` - The ID of the second pet
    /// * `interaction_type` - The type of interaction
    /// * `outcome` - The outcome of the interaction
    /// * `duration` - The duration of the interaction
    /// * `mood_change_1` - The mood change for the first pet
    /// * `mood_change_2` - The mood change for the second pet
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn record_interaction(
        pet_id_1: PetId,
        pet_id_2: PetId,
        interaction_type: u8,
        outcome: u8,
        duration: u32,
        mood_change_1: i8,
        mood_change_2: i8,
    ) -> DispatchResult {
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Record the interaction for the first pet
        crate::PetSocialInteractions::<T>::try_mutate(pet_id_1, |interactions| -> DispatchResult {
            // Add the interaction
            interactions.try_push((pet_id_2, interaction_type, outcome, current_block))
                .map_err(|_| Error::<T>::TooManySocialInteractions)?;
            
            Ok(())
        })?;
        
        // Record the interaction for the second pet
        crate::PetSocialInteractions::<T>::try_mutate(pet_id_2, |interactions| -> DispatchResult {
            // Add the interaction
            interactions.try_push((pet_id_1, interaction_type, outcome, current_block))
                .map_err(|_| Error::<T>::TooManySocialInteractions)?;
            
            Ok(())
        })?;
        
        // Record a memory of the interaction for both pets
        crate::memory::PetMemorySystem::<T>::record_memory(
            pet_id_1,
            crate::memory::MemoryType::SocialInteraction as u8,
            if outcome <= 1 { 150 } else if outcome >= 3 { 100 } else { 50 }, // Significance based on outcome
            pet_id_2.encode(), // Store the other pet's ID as associated data
        )?;
        
        crate::memory::PetMemorySystem::<T>::record_memory(
            pet_id_2,
            crate::memory::MemoryType::SocialInteraction as u8,
            if outcome <= 1 { 150 } else if outcome >= 3 { 100 } else { 50 }, // Significance based on outcome
            pet_id_1.encode(), // Store the other pet's ID as associated data
        )?;
        
        Ok(())
    }
    
    /// Gets all social bonds for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Vec<SocialBond>` - The social bonds
    pub fn get_social_bonds(
        pet_id: PetId,
    ) -> Vec<SocialBond> {
        crate::PetSocialBonds::<T>::get(pet_id).to_vec()
    }
    
    /// Gets all social interactions for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Vec<(PetId, u8, u8, BlockNumberFor<T>)>` - The social interactions (other_pet_id, interaction_type, outcome, timestamp)
    pub fn get_social_interactions(
        pet_id: PetId,
    ) -> Vec<(PetId, u8, u8, BlockNumberFor<T>)> {
        crate::PetSocialInteractions::<T>::get(pet_id).to_vec()
    }
    
    /// Gets the social bond between two pets.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `other_pet_id` - The ID of the other pet
    /// 
    /// # Returns
    /// 
    /// * `Option<SocialBond>` - The social bond, or None if no bond exists
    pub fn get_social_bond(
        pet_id: PetId,
        other_pet_id: PetId,
    ) -> Option<SocialBond> {
        let bonds = crate::PetSocialBonds::<T>::get(pet_id);
        
        bonds.iter()
            .find(|bond| bond.other_pet_id == other_pet_id)
            .cloned()
    }
    
    /// Gets the compatibility between two pets.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id_1` - The ID of the first pet
    /// * `pet_id_2` - The ID of the second pet
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The compatibility (0-255), or an error
    pub fn get_compatibility(
        pet_id_1: PetId,
        pet_id_2: PetId,
    ) -> Result<u8, DispatchError> {
        // Get the pets from storage
        let pet1 = crate::PetNfts::<T>::get(pet_id_1).ok_or(Error::<T>::PetNotFound)?;
        let pet2 = crate::PetNfts::<T>::get(pet_id_2).ok_or(Error::<T>::PetNotFound)?;
        
        // In a real implementation, this would calculate compatibility
        // based on various factors such as personality traits, elemental affinities, etc.
        // For now, we'll use a simple algorithm
        
        // Base compatibility
        let mut compatibility = 128; // 50%
        
        // Adjust based on elemental affinity
        if pet1.primary_elemental_affinity as u8 == pet2.primary_elemental_affinity as u8 {
            // Same element: +20% compatibility
            compatibility = compatibility.saturating_add(51);
        } else if (pet1.primary_elemental_affinity as u8 + pet2.primary_elemental_affinity as u8) % 8 == 4 {
            // Opposite elements: -20% compatibility
            compatibility = compatibility.saturating_sub(51);
        }
        
        // Adjust based on mood
        if pet1.mood_indicator > 200 && pet2.mood_indicator > 200 {
            // Both pets are very happy: +10% compatibility
            compatibility = compatibility.saturating_add(25);
        } else if pet1.mood_indicator < 100 && pet2.mood_indicator < 100 {
            // Both pets are unhappy: -10% compatibility
            compatibility = compatibility.saturating_sub(25);
        }
        
        Ok(compatibility)
    }
}