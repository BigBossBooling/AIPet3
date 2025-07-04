//! # Pet Achievements System
//!
//! This module provides a system for pets to earn achievements for reaching various milestones,
//! encouraging different play styles and giving owners goals to work toward.

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

/// Represents an achievement that a pet can earn.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Achievement {
    /// The achievement ID
    pub id: u32,
    
    /// The achievement name
    pub name: Vec<u8>,
    
    /// The achievement description
    pub description: Vec<u8>,
    
    /// The achievement category
    pub category: u8,
    
    /// The achievement difficulty (0-255)
    pub difficulty: u8,
    
    /// The rewards for earning this achievement
    pub rewards: AchievementRewards,
    
    /// The requirements for earning this achievement
    pub requirements: AchievementRequirements,
}

/// Represents the rewards for earning an achievement.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AchievementRewards {
    /// The experience points awarded
    pub experience: u32,
    
    /// The mood boost awarded
    pub mood_boost: u8,
    
    /// The stat boost awarded
    pub stat_boost: u8,
    
    /// The stat that is boosted
    pub boosted_stat: u8,
    
    /// The special ability unlocked
    pub special_ability: u8,
}

/// Represents the requirements for earning an achievement.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AchievementRequirements {
    /// The minimum level required
    pub min_level: u32,
    
    /// The minimum experience points required
    pub min_experience: u32,
    
    /// The minimum stat values required
    pub min_stats: Vec<(u8, u8)>, // (stat_type, min_value)
    
    /// The skills required
    pub required_skills: Vec<(u8, u8)>, // (skill_type, min_level)
    
    /// The achievements required as prerequisites
    pub prerequisite_achievements: Vec<u32>, // achievement_ids
    
    /// The number of social interactions required
    pub required_social_interactions: u32,
    
    /// The number of environmental adaptations required
    pub required_environmental_adaptations: u32,
    
    /// The number of memories required
    pub required_memories: u32,
}

/// Achievement categories.
pub enum AchievementCategory {
    General = 0,
    Combat = 1,
    Exploration = 2,
    Social = 3,
    Crafting = 4,
    Training = 5,
    Collection = 6,
    Special = 7,
}

/// A system for managing pet achievements.
pub struct AchievementSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> AchievementSystem<T> {
    /// Checks if a pet has earned any new achievements.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn check_achievements(
        pet_id: PetId,
    ) -> DispatchResult {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get all available achievements
        let achievements = Self::get_all_achievements();
        
        // Get the pet's current achievements
        let current_achievements = crate::PetAchievements::<T>::get(pet_id);
        
        // Check each achievement
        for achievement in achievements {
            // Skip achievements the pet already has
            if current_achievements.iter().any(|(id, _)| *id == achievement.id) {
                continue;
            }
            
            // Check if the pet meets the requirements
            if Self::meets_requirements(&pet, &achievement.requirements)? {
                // Award the achievement
                Self::award_achievement(pet_id, achievement.id, &achievement.rewards)?;
            }
        }
        
        Ok(())
    }
    
    /// Gets all available achievements.
    /// 
    /// # Returns
    /// 
    /// * `Vec<Achievement>` - All available achievements
    fn get_all_achievements() -> Vec<Achievement> {
        // In a real implementation, this would get all achievements from storage
        // For now, we'll just return a few hardcoded achievements
        
        vec![
            Achievement {
                id: 1,
                name: b"First Steps".to_vec(),
                description: b"Reach level 5.".to_vec(),
                category: AchievementCategory::General as u8,
                difficulty: 50,
                rewards: AchievementRewards {
                    experience: 100,
                    mood_boost: 10,
                    stat_boost: 5,
                    boosted_stat: 0, // Strength
                    special_ability: 0, // None
                },
                requirements: AchievementRequirements {
                    min_level: 5,
                    min_experience: 0,
                    min_stats: Vec::new(),
                    required_skills: Vec::new(),
                    prerequisite_achievements: Vec::new(),
                    required_social_interactions: 0,
                    required_environmental_adaptations: 0,
                    required_memories: 0,
                },
            },
            Achievement {
                id: 2,
                name: b"Skilled Apprentice".to_vec(),
                description: b"Learn a skill to level 5.".to_vec(),
                category: AchievementCategory::Training as u8,
                difficulty: 100,
                rewards: AchievementRewards {
                    experience: 200,
                    mood_boost: 15,
                    stat_boost: 10,
                    boosted_stat: 2, // Intelligence
                    special_ability: 0, // None
                },
                requirements: AchievementRequirements {
                    min_level: 0,
                    min_experience: 0,
                    min_stats: Vec::new(),
                    required_skills: vec![(0, 5)], // Any skill at level 5
                    prerequisite_achievements: Vec::new(),
                    required_social_interactions: 0,
                    required_environmental_adaptations: 0,
                    required_memories: 0,
                },
            },
            Achievement {
                id: 3,
                name: b"Social Butterfly".to_vec(),
                description: b"Interact with 10 different pets.".to_vec(),
                category: AchievementCategory::Social as u8,
                difficulty: 150,
                rewards: AchievementRewards {
                    experience: 300,
                    mood_boost: 20,
                    stat_boost: 10,
                    boosted_stat: 3, // Vitality
                    special_ability: 0, // None
                },
                requirements: AchievementRequirements {
                    min_level: 0,
                    min_experience: 0,
                    min_stats: Vec::new(),
                    required_skills: Vec::new(),
                    prerequisite_achievements: Vec::new(),
                    required_social_interactions: 10,
                    required_environmental_adaptations: 0,
                    required_memories: 0,
                },
            },
        ]
    }
    
    /// Checks if a pet meets the requirements for an achievement.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `requirements` - The achievement requirements
    /// 
    /// # Returns
    /// 
    /// * `Result<bool, DispatchError>` - Whether the pet meets the requirements, or an error
    fn meets_requirements(
        pet: &PetNft<T>,
        requirements: &AchievementRequirements,
    ) -> Result<bool, DispatchError> {
        // Check level requirement
        if pet.level < requirements.min_level {
            return Ok(false);
        }
        
        // Check experience requirement
        if pet.experience_points < requirements.min_experience {
            return Ok(false);
        }
        
        // Check stat requirements
        for (stat_type, min_value) in &requirements.min_stats {
            let stat_value = match stat_type {
                0 => pet.base_strength,
                1 => pet.base_agility,
                2 => pet.base_intelligence,
                3 => pet.base_vitality,
                _ => 0,
            };
            
            if stat_value < *min_value {
                return Ok(false);
            }
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
                
                if !has_skill {
                    return Ok(false);
                }
            }
        }
        
        // Check prerequisite achievements
        if !requirements.prerequisite_achievements.is_empty() {
            let achievements = crate::PetAchievements::<T>::get(pet.id);
            
            for prerequisite in &requirements.prerequisite_achievements {
                if !achievements.iter().any(|(id, _)| *id == *prerequisite) {
                    return Ok(false);
                }
            }
        }
        
        // Check social interaction requirement
        if requirements.required_social_interactions > 0 {
            let interactions = crate::PetSocialInteractions::<T>::get(pet.id);
            
            // Count unique pets interacted with
            let mut unique_pets = Vec::new();
            for (other_pet_id, _, _, _) in interactions.iter() {
                if !unique_pets.contains(other_pet_id) {
                    unique_pets.push(*other_pet_id);
                }
            }
            
            if unique_pets.len() < requirements.required_social_interactions as usize {
                return Ok(false);
            }
        }
        
        // Check environmental adaptation requirement
        if requirements.required_environmental_adaptations > 0 {
            let adaptations = crate::PetEnvironmentalAdaptations::<T>::get(pet.id);
            
            if adaptations.len() < requirements.required_environmental_adaptations as usize {
                return Ok(false);
            }
        }
        
        // Check memory requirement
        if requirements.required_memories > 0 {
            let memories = crate::PetMemories::<T>::get(pet.id);
            
            if memories.len() < requirements.required_memories as usize {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Awards an achievement to a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `achievement_id` - The ID of the achievement
    /// * `rewards` - The rewards for the achievement
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn award_achievement(
        pet_id: PetId,
        achievement_id: u32,
        rewards: &AchievementRewards,
    ) -> DispatchResult {
        // Record the achievement
        crate::PetAchievements::<T>::try_mutate(pet_id, |achievements| -> DispatchResult {
            // Get the current block number
            let current_block = frame_system::Pallet::<T>::block_number();
            
            // Add the achievement
            achievements.try_push((achievement_id, current_block.saturated_into::<u64>()))
                .map_err(|_| Error::<T>::TooManyAchievements)?;
            
            Ok(())
        })?;
        
        // Apply the rewards
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Apply experience reward
            pet.experience_points = pet.experience_points.saturating_add(rewards.experience);
            
            // Apply mood boost
            pet.mood_indicator = pet.mood_indicator
                .saturating_add(rewards.mood_boost)
                .min(T::MaxMoodValue::get());
            
            // Apply stat boost
            match rewards.boosted_stat {
                0 => { // Strength
                    pet.base_strength = pet.base_strength.saturating_add(rewards.stat_boost);
                },
                1 => { // Agility
                    pet.base_agility = pet.base_agility.saturating_add(rewards.stat_boost);
                },
                2 => { // Intelligence
                    pet.base_intelligence = pet.base_intelligence.saturating_add(rewards.stat_boost);
                },
                3 => { // Vitality
                    pet.base_vitality = pet.base_vitality.saturating_add(rewards.stat_boost);
                },
                _ => {} // No boost for other stats
            }
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })?;
        
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::AchievementEarned {
            pet_id,
            achievement_id,
            timestamp: current_block,
        });
        
        // Record a memory of the achievement
        crate::memory::PetMemorySystem::<T>::record_memory(
            pet_id,
            crate::memory::MemoryType::Achievement as u8,
            200, // High significance
            achievement_id.encode(), // Store the achievement ID as associated data
        )?;
        
        Ok(())
    }
    
    /// Gets an achievement by ID.
    /// 
    /// # Parameters
    /// 
    /// * `achievement_id` - The ID of the achievement
    /// 
    /// # Returns
    /// 
    /// * `Result<Achievement, DispatchError>` - The achievement, or an error
    pub fn get_achievement(
        achievement_id: u32,
    ) -> Result<Achievement, DispatchError> {
        // In a real implementation, this would get the achievement from storage
        // For now, we'll just return a hardcoded achievement based on the ID
        
        let achievements = Self::get_all_achievements();
        
        for achievement in achievements {
            if achievement.id == achievement_id {
                return Ok(achievement);
            }
        }
        
        Err(Error::<T>::AchievementNotFound.into())
    }
    
    /// Gets all of a pet's achievements.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u32, u64)>` - The achievements (achievement_id, timestamp)
    pub fn get_pet_achievements(
        pet_id: PetId,
    ) -> Vec<(u32, u64)> {
        crate::PetAchievements::<T>::get(pet_id).to_vec()
    }
    
    /// Gets detailed information about a pet's achievements.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<(Achievement, u64)>, DispatchError>` - The achievements and timestamps, or an error
    pub fn get_pet_achievement_details(
        pet_id: PetId,
    ) -> Result<Vec<(Achievement, u64)>, DispatchError> {
        let achievements = crate::PetAchievements::<T>::get(pet_id);
        
        let mut result = Vec::with_capacity(achievements.len());
        for (achievement_id, timestamp) in achievements.iter() {
            let achievement = Self::get_achievement(*achievement_id)?;
            result.push((achievement, *timestamp));
        }
        
        Ok(result)
    }
    
    /// Gets the achievements a pet is close to earning.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<Achievement>, DispatchError>` - The achievements, or an error
    pub fn get_pet_achievement_progress(
        pet_id: PetId,
    ) -> Result<Vec<Achievement>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get all available achievements
        let achievements = Self::get_all_achievements();
        
        // Get the pet's current achievements
        let current_achievements = crate::PetAchievements::<T>::get(pet_id);
        
        // Find achievements the pet is close to earning
        let mut result = Vec::new();
        for achievement in achievements {
            // Skip achievements the pet already has
            if current_achievements.iter().any(|(id, _)| *id == achievement.id) {
                continue;
            }
            
            // Check if the pet is close to meeting the requirements
            if pet.level >= achievement.requirements.min_level.saturating_sub(2) &&
               pet.experience_points >= achievement.requirements.min_experience.saturating_sub(100) {
                result.push(achievement);
            }
        }
        
        Ok(result)
    }
}
