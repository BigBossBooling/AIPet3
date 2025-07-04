//! # Advanced Pet Training System
//!
//! This module provides a sophisticated system for training pets in specific skills,
//! adding depth to pet development and giving owners more ways to customize their pets.

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

/// Represents a skill that a pet can learn.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Skill {
    /// The skill type
    pub skill_type: u8,
    
    /// The skill name
    pub name: Vec<u8>,
    
    /// The skill description
    pub description: Vec<u8>,
    
    /// The primary stat used for this skill
    pub primary_stat: u8,
    
    /// The secondary stat used for this skill
    pub secondary_stat: u8,
    
    /// The difficulty of learning this skill (0-255)
    pub learning_difficulty: u8,
    
    /// The maximum level this skill can reach
    pub max_level: u8,
    
    /// The benefits provided by this skill
    pub benefits: SkillBenefits,
}

/// Represents the benefits provided by a skill.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SkillBenefits {
    /// The stat boost provided by the skill
    pub stat_boost: u8,
    
    /// The stat that is boosted
    pub boosted_stat: u8,
    
    /// The mood boost provided by the skill
    pub mood_boost: u8,
    
    /// The experience boost provided by the skill (percentage)
    pub experience_boost: u8,
    
    /// The special ability unlocked by this skill
    pub special_ability: u8,
}

/// Skill types.
pub enum SkillType {
    Hunting = 0,
    Fishing = 1,
    Gathering = 2,
    Crafting = 3,
    Mining = 4,
    Farming = 5,
    Cooking = 6,
    Healing = 7,
    Combat = 8,
    Stealth = 9,
    Diplomacy = 10,
    Leadership = 11,
    Magic = 12,
    Technology = 13,
    Music = 14,
    Art = 15,
}

/// Training intensity levels.
pub enum TrainingIntensity {
    Light = 0,
    Moderate = 1,
    Intense = 2,
    Extreme = 3,
}

/// A system for training pets in specific skills.
pub struct PetTrainingSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> PetTrainingSystem<T> {
    /// Trains a pet in a specific skill.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `skill_type` - The type of skill to train
    /// * `training_intensity` - The intensity of the training
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn train_pet(
        pet_id: PetId,
        skill_type: u8,
        training_intensity: u8,
    ) -> DispatchResult {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the skill
        let skill = Self::get_skill(skill_type)?;
        
        // Check if the pet meets the requirements for this skill
        Self::check_requirements(&pet, &skill)?;
        
        // Get the pet's current skill level
        let current_level = Self::get_skill_level(pet_id, skill_type)?;
        
        // Check if the pet has already mastered this skill
        ensure!(
            current_level < skill.max_level,
            Error::<T>::SkillAlreadyMastered
        );
        
        // Calculate training effectiveness
        let effectiveness = Self::calculate_training_effectiveness(
            &pet,
            &skill,
            training_intensity,
        )?;
        
        // Determine if the training was successful
        let success = Self::determine_training_success(
            effectiveness,
            current_level,
            skill.learning_difficulty,
        )?;
        
        if success {
            // Increase the pet's skill level
            Self::increase_skill_level(pet_id, skill_type, current_level)?;
            
            // Apply the effects of successful training
            Self::apply_training_effects(
                pet_id,
                &skill,
                current_level + 1,
                training_intensity,
            )?;
            
            // Get the current block number
            let current_block = frame_system::Pallet::<T>::block_number();
            
            // Emit an event
            crate::Pallet::<T>::deposit_event(crate::Event::PetTrainingCompleted {
                pet_id,
                skill_type,
                skill_level: current_level + 1,
                timestamp: current_block,
            });
            
            // Potentially evolve personality traits based on the training
            if current_level + 1 >= skill.max_level / 2 {
                // High skill level: evolve a relevant trait
                let trait_type = match skill_type {
                    0 => crate::personality::PersonalityTraitType::Brave as u8, // Hunting
                    1 => crate::personality::PersonalityTraitType::Patient as u8, // Fishing
                    2 => crate::personality::PersonalityTraitType::Curious as u8, // Gathering
                    3 => crate::personality::PersonalityTraitType::Creative as u8, // Crafting
                    _ => crate::personality::PersonalityTraitType::Intelligent as u8, // Default
                };
                
                crate::personality::PersonalityEvolutionSystem::<T>::evolve_personality(
                    pet_id,
                    crate::personality::EvolutionCatalyst::Training as u8,
                    200, // High intensity for mastering a skill
                )?;
            }
        } else {
            // Apply the effects of failed training
            Self::apply_failed_training_effects(
                pet_id,
                training_intensity,
            )?;
        }
        
        Ok(())
    }
    
    /// Gets a skill by type.
    /// 
    /// # Parameters
    /// 
    /// * `skill_type` - The type of skill
    /// 
    /// # Returns
    /// 
    /// * `Result<Skill, DispatchError>` - The skill, or an error
    fn get_skill(skill_type: u8) -> Result<Skill, DispatchError> {
        // In a real implementation, this would get the skill from storage
        // For now, we'll just return a hardcoded skill based on the type
        
        match skill_type {
            0 => { // Hunting
                Ok(Skill {
                    skill_type,
                    name: b"Hunting".to_vec(),
                    description: b"The ability to track and catch wild animals.".to_vec(),
                    primary_stat: 0, // Strength
                    secondary_stat: 1, // Agility
                    learning_difficulty: 100,
                    max_level: 10,
                    benefits: SkillBenefits {
                        stat_boost: 5,
                        boosted_stat: 0, // Strength
                        mood_boost: 5,
                        experience_boost: 10,
                        special_ability: 0, // None
                    },
                })
            },
            1 => { // Fishing
                Ok(Skill {
                    skill_type,
                    name: b"Fishing".to_vec(),
                    description: b"The ability to catch fish.".to_vec(),
                    primary_stat: 1, // Agility
                    secondary_stat: 2, // Intelligence
                    learning_difficulty: 75,
                    max_level: 10,
                    benefits: SkillBenefits {
                        stat_boost: 5,
                        boosted_stat: 1, // Agility
                        mood_boost: 10,
                        experience_boost: 5,
                        special_ability: 0, // None
                    },
                })
            },
            2 => { // Gathering
                Ok(Skill {
                    skill_type,
                    name: b"Gathering".to_vec(),
                    description: b"The ability to find and collect resources.".to_vec(),
                    primary_stat: 2, // Intelligence
                    secondary_stat: 1, // Agility
                    learning_difficulty: 50,
                    max_level: 10,
                    benefits: SkillBenefits {
                        stat_boost: 5,
                        boosted_stat: 2, // Intelligence
                        mood_boost: 5,
                        experience_boost: 5,
                        special_ability: 0, // None
                    },
                })
            },
            _ => {
                // Default to a generic skill
                Ok(Skill {
                    skill_type,
                    name: b"Unknown".to_vec(),
                    description: b"An unknown skill.".to_vec(),
                    primary_stat: 0, // Strength
                    secondary_stat: 0, // Strength
                    learning_difficulty: 100,
                    max_level: 10,
                    benefits: SkillBenefits {
                        stat_boost: 5,
                        boosted_stat: 0, // Strength
                        mood_boost: 5,
                        experience_boost: 5,
                        special_ability: 0, // None
                    },
                })
            }
        }
    }
    
    /// Checks if a pet meets the requirements for a skill.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `skill` - The skill
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if the pet meets the requirements, Err otherwise
    fn check_requirements(
        pet: &PetNft<T>,
        skill: &Skill,
    ) -> DispatchResult {
        // In a real implementation, this would check various requirements
        // such as level, stats, prerequisites, etc.
        // For now, we'll just do a simple check based on level
        
        // Ensure the pet is at least level 5
        ensure!(
            pet.level >= 5,
            Error::<T>::PetTooYoung
        );
        
        // Ensure the pet has sufficient primary stat
        let primary_stat = match skill.primary_stat {
            0 => pet.base_strength,
            1 => pet.base_agility,
            2 => pet.base_intelligence,
            3 => pet.base_vitality,
            _ => 0,
        };
        
        ensure!(
            primary_stat >= 10,
            Error::<T>::SkillLevelTooLow
        );
        
        Ok(())
    }
    
    /// Gets a pet's current level in a skill.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `skill_type` - The type of skill
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The skill level, or an error
    fn get_skill_level(
        pet_id: PetId,
        skill_type: u8,
    ) -> Result<u8, DispatchError> {
        let skills = crate::PetSkills::<T>::get(pet_id);
        
        for (skill, level) in skills.iter() {
            if *skill == skill_type {
                return Ok(*level);
            }
        }
        
        // If the pet doesn't have this skill yet, return level 0
        Ok(0)
    }
    
    /// Calculates the effectiveness of training.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `skill` - The skill
    /// * `training_intensity` - The intensity of the training
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The training effectiveness (0-255), or an error
    fn calculate_training_effectiveness(
        pet: &PetNft<T>,
        skill: &Skill,
        training_intensity: u8,
    ) -> Result<u8, DispatchError> {
        // In a real implementation, this would calculate the effectiveness
        // based on various factors such as pet stats, mood, etc.
        // For now, we'll use a simple algorithm
        
        // Base effectiveness
        let mut effectiveness = 128; // 50%
        
        // Adjust based on primary stat
        let primary_stat = match skill.primary_stat {
            0 => pet.base_strength,
            1 => pet.base_agility,
            2 => pet.base_intelligence,
            3 => pet.base_vitality,
            _ => 0,
        };
        
        if primary_stat > 20 {
            // High primary stat: +20% effectiveness
            effectiveness = effectiveness.saturating_add(51);
        } else if primary_stat < 10 {
            // Low primary stat: -20% effectiveness
            effectiveness = effectiveness.saturating_sub(51);
        }
        
        // Adjust based on secondary stat
        let secondary_stat = match skill.secondary_stat {
            0 => pet.base_strength,
            1 => pet.base_agility,
            2 => pet.base_intelligence,
            3 => pet.base_vitality,
            _ => 0,
        };
        
        if secondary_stat > 20 {
            // High secondary stat: +10% effectiveness
            effectiveness = effectiveness.saturating_add(25);
        } else if secondary_stat < 10 {
            // Low secondary stat: -10% effectiveness
            effectiveness = effectiveness.saturating_sub(25);
        }
        
        // Adjust based on mood
        if pet.mood_indicator > 200 {
            // Very happy: +10% effectiveness
            effectiveness = effectiveness.saturating_add(25);
        } else if pet.mood_indicator < 100 {
            // Unhappy: -10% effectiveness
            effectiveness = effectiveness.saturating_sub(25);
        }
        
        // Adjust based on training intensity
        match training_intensity {
            0 => { // Light
                // -10% effectiveness, but less strain on the pet
                effectiveness = effectiveness.saturating_sub(25);
            },
            1 => { // Moderate
                // No adjustment
            },
            2 => { // Intense
                // +10% effectiveness, but more strain on the pet
                effectiveness = effectiveness.saturating_add(25);
            },
            3 => { // Extreme
                // +20% effectiveness, but much more strain on the pet
                effectiveness = effectiveness.saturating_add(51);
            },
            _ => {} // No adjustment for other intensities
        }
        
        Ok(effectiveness)
    }
    
    /// Determines if training was successful.
    /// 
    /// # Parameters
    /// 
    /// * `effectiveness` - The training effectiveness
    /// * `current_level` - The pet's current level in the skill
    /// * `learning_difficulty` - The difficulty of learning the skill
    /// 
    /// # Returns
    /// 
    /// * `Result<bool, DispatchError>` - Whether the training was successful, or an error
    fn determine_training_success(
        effectiveness: u8,
        current_level: u8,
        learning_difficulty: u8,
    ) -> Result<bool, DispatchError> {
        // In a real implementation, this would use a complex algorithm
        // to determine if the training was successful
        // For now, we'll use a simple algorithm
        
        // Calculate the success chance
        let level_factor = current_level * 10; // Higher levels are harder to improve
        let difficulty_factor = learning_difficulty / 2; // Higher difficulty reduces success chance
        let effectiveness_factor = effectiveness / 2; // Higher effectiveness increases success chance
        
        let success_chance = 100
            .saturating_sub(level_factor)
            .saturating_sub(difficulty_factor)
            .saturating_add(effectiveness_factor);
        
        // Get a random value
        let (random_seed, _) = T::PetRandomness::random_seed();
        let random_value = random_seed.using_encoded(|encoded| {
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&encoded[0..4]);
            u32::from_le_bytes(buf)
        });
        
        // Determine if the training was successful
        Ok((random_value % 100) < success_chance as u32)
    }
    
    /// Increases a pet's skill level.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `skill_type` - The type of skill
    /// * `current_level` - The pet's current level in the skill
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn increase_skill_level(
        pet_id: PetId,
        skill_type: u8,
        current_level: u8,
    ) -> DispatchResult {
        crate::PetSkills::<T>::try_mutate(pet_id, |skills| -> DispatchResult {
            // Check if the pet already has this skill
            for i in 0..skills.len() {
                if let Some((skill, level)) = skills.get(i) {
                    if *skill == skill_type {
                        // Update the existing skill
                        skills.set(i, (skill_type, current_level + 1))?;
                        return Ok(());
                    }
                }
            }
            
            // Add the new skill
            skills.try_push((skill_type, current_level + 1))
                .map_err(|_| Error::<T>::TooManySkills)?;
            
            Ok(())
        })
    }
    
    /// Applies the effects of successful training.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `skill` - The skill
    /// * `new_level` - The pet's new level in the skill
    /// * `training_intensity` - The intensity of the training
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn apply_training_effects(
        pet_id: PetId,
        skill: &Skill,
        new_level: u8,
        training_intensity: u8,
    ) -> DispatchResult {
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Apply stat boost
            match skill.benefits.boosted_stat {
                0 => { // Strength
                    let boost = skill.benefits.stat_boost * new_level / skill.max_level;
                    pet.base_strength = pet.base_strength.saturating_add(boost);
                },
                1 => { // Agility
                    let boost = skill.benefits.stat_boost * new_level / skill.max_level;
                    pet.base_agility = pet.base_agility.saturating_add(boost);
                },
                2 => { // Intelligence
                    let boost = skill.benefits.stat_boost * new_level / skill.max_level;
                    pet.base_intelligence = pet.base_intelligence.saturating_add(boost);
                },
                3 => { // Vitality
                    let boost = skill.benefits.stat_boost * new_level / skill.max_level;
                    pet.base_vitality = pet.base_vitality.saturating_add(boost);
                },
                _ => {} // No boost for other stats
            }
            
            // Apply mood boost
            let mood_boost = skill.benefits.mood_boost * new_level / skill.max_level;
            pet.mood_indicator = pet.mood_indicator
                .saturating_add(mood_boost)
                .min(T::MaxMoodValue::get());
            
            // Apply experience boost
            let experience_boost = skill.benefits.experience_boost * new_level / skill.max_level;
            pet.experience_points = pet.experience_points.saturating_add(experience_boost as u32 * 10);
            
            // Apply mood penalty based on training intensity
            match training_intensity {
                0 => {}, // Light: no penalty
                1 => { // Moderate
                    pet.mood_indicator = pet.mood_indicator.saturating_sub(5);
                },
                2 => { // Intense
                    pet.mood_indicator = pet.mood_indicator.saturating_sub(10);
                },
                3 => { // Extreme
                    pet.mood_indicator = pet.mood_indicator.saturating_sub(20);
                },
                _ => {} // No penalty for other intensities
            }
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })
    }
    
    /// Applies the effects of failed training.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `training_intensity` - The intensity of the training
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn apply_failed_training_effects(
        pet_id: PetId,
        training_intensity: u8,
    ) -> DispatchResult {
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Apply mood penalty based on training intensity
            match training_intensity {
                0 => { // Light
                    pet.mood_indicator = pet.mood_indicator.saturating_sub(5);
                },
                1 => { // Moderate
                    pet.mood_indicator = pet.mood_indicator.saturating_sub(10);
                },
                2 => { // Intense
                    pet.mood_indicator = pet.mood_indicator.saturating_sub(20);
                },
                3 => { // Extreme
                    pet.mood_indicator = pet.mood_indicator.saturating_sub(40);
                },
                _ => {} // No penalty for other intensities
            }
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })
    }
    
    /// Gets all of a pet's skills.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u8, u8)>` - The skills (skill_type, level)
    pub fn get_all_skills(
        pet_id: PetId,
    ) -> Vec<(u8, u8)> {
        crate::PetSkills::<T>::get(pet_id).to_vec()
    }
    
    /// Gets detailed information about a pet's skill.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `skill_type` - The type of skill
    /// 
    /// # Returns
    /// 
    /// * `Result<(Skill, u8), DispatchError>` - The skill and the pet's level in it, or an error
    pub fn get_skill_details(
        pet_id: PetId,
        skill_type: u8,
    ) -> Result<(Skill, u8), DispatchError> {
        let skill = Self::get_skill(skill_type)?;
        let level = Self::get_skill_level(pet_id, skill_type)?;
        
        Ok((skill, level))
    }
}