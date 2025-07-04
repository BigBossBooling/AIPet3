//! # Environmental Adaptation System
//!
//! This module provides a system for pets to adapt to different environments,
//! allowing them to thrive in different "regions" of the blockchain ecosystem.

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::Get,
    BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;
use scale_info::TypeInfo;
use crate::{Config, Error, PetId, PetNft, ElementType};

/// Represents an environment that pets can adapt to.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Environment {
    /// The environment type
    pub environment_type: u8,
    
    /// The primary element of the environment
    pub primary_element: u8,
    
    /// The secondary element of the environment
    pub secondary_element: u8,
    
    /// The difficulty of adapting to this environment (0-255)
    pub adaptation_difficulty: u8,
    
    /// The benefits of adapting to this environment
    pub benefits: EnvironmentBenefits,
    
    /// The challenges of adapting to this environment
    pub challenges: EnvironmentChallenges,
}

/// Represents the benefits of adapting to an environment.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct EnvironmentBenefits {
    /// The stat boost provided by the environment
    pub stat_boost: u8,
    
    /// The stat that is boosted
    pub boosted_stat: u8,
    
    /// The mood boost provided by the environment
    pub mood_boost: u8,
    
    /// The experience boost provided by the environment (percentage)
    pub experience_boost: u8,
}

/// Represents the challenges of adapting to an environment.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct EnvironmentChallenges {
    /// The stat penalty imposed by the environment
    pub stat_penalty: u8,
    
    /// The stat that is penalized
    pub penalized_stat: u8,
    
    /// The mood penalty imposed by the environment
    pub mood_penalty: u8,
    
    /// The experience penalty imposed by the environment (percentage)
    pub experience_penalty: u8,
}

/// Environment types.
pub enum EnvironmentType {
    Forest = 0,
    Mountain = 1,
    Desert = 2,
    Ocean = 3,
    Tundra = 4,
    Volcano = 5,
    City = 6,
    Space = 7,
    Digital = 8,
    Ethereal = 9,
}

/// A system for managing pet adaptations to different environments.
pub struct EnvironmentalAdaptationSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> EnvironmentalAdaptationSystem<T> {
    /// Adapts a pet to a new environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `environment_type` - The type of environment to adapt to
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn adapt_to_environment(
        pet_id: PetId,
        environment_type: u8,
    ) -> DispatchResult {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the environment
        let environment = Self::get_environment(environment_type)?;
        
        // Check compatibility
        Self::check_compatibility(&pet, &environment)?;
        
        // Calculate adaptation level
        let adaptation_level = Self::calculate_adaptation_level(&pet, &environment)?;
        
        // Apply adaptation effects
        Self::apply_adaptation_effects(pet_id, &environment, adaptation_level)?;
        
        // Record the adaptation
        Self::record_adaptation(pet_id, environment_type, adaptation_level)?;
        
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::EnvironmentalAdaptation {
            pet_id,
            environment_type,
            adaptation_level,
            timestamp: current_block,
        });
        
        // Potentially evolve personality traits based on the adaptation
        if adaptation_level > 200 {
            // High adaptation level: evolve the "Adaptable" trait
            crate::personality::PersonalityEvolutionSystem::<T>::evolve_personality(
                pet_id,
                crate::personality::EvolutionCatalyst::EnvironmentalChange as u8,
                adaptation_level,
            )?;
        }
        
        Ok(())
    }
    
    /// Gets an environment by type.
    /// 
    /// # Parameters
    /// 
    /// * `environment_type` - The type of environment
    /// 
    /// # Returns
    /// 
    /// * `Result<Environment, DispatchError>` - The environment, or an error
    fn get_environment(environment_type: u8) -> Result<Environment, DispatchError> {
        // In a real implementation, this would get the environment from storage
        // For now, we'll just return a hardcoded environment based on the type
        
        match environment_type {
            0 => { // Forest
                Ok(Environment {
                    environment_type,
                    primary_element: ElementType::Nature as u8,
                    secondary_element: ElementType::Water as u8,
                    adaptation_difficulty: 50,
                    benefits: EnvironmentBenefits {
                        stat_boost: 10,
                        boosted_stat: 1, // Agility
                        mood_boost: 5,
                        experience_boost: 10,
                    },
                    challenges: EnvironmentChallenges {
                        stat_penalty: 5,
                        penalized_stat: 0, // Strength
                        mood_penalty: 0,
                        experience_penalty: 0,
                    },
                })
            },
            1 => { // Mountain
                Ok(Environment {
                    environment_type,
                    primary_element: ElementType::Earth as u8,
                    secondary_element: ElementType::Air as u8,
                    adaptation_difficulty: 100,
                    benefits: EnvironmentBenefits {
                        stat_boost: 15,
                        boosted_stat: 0, // Strength
                        mood_boost: 0,
                        experience_boost: 15,
                    },
                    challenges: EnvironmentChallenges {
                        stat_penalty: 10,
                        penalized_stat: 1, // Agility
                        mood_penalty: 5,
                        experience_penalty: 0,
                    },
                })
            },
            2 => { // Desert
                Ok(Environment {
                    environment_type,
                    primary_element: ElementType::Fire as u8,
                    secondary_element: ElementType::Earth as u8,
                    adaptation_difficulty: 150,
                    benefits: EnvironmentBenefits {
                        stat_boost: 20,
                        boosted_stat: 3, // Vitality
                        mood_boost: 0,
                        experience_boost: 20,
                    },
                    challenges: EnvironmentChallenges {
                        stat_penalty: 15,
                        penalized_stat: 2, // Intelligence
                        mood_penalty: 10,
                        experience_penalty: 0,
                    },
                })
            },
            _ => {
                // Default to a generic environment
                Ok(Environment {
                    environment_type,
                    primary_element: ElementType::Neutral as u8,
                    secondary_element: ElementType::Neutral as u8,
                    adaptation_difficulty: 100,
                    benefits: EnvironmentBenefits {
                        stat_boost: 10,
                        boosted_stat: 0, // Strength
                        mood_boost: 5,
                        experience_boost: 10,
                    },
                    challenges: EnvironmentChallenges {
                        stat_penalty: 5,
                        penalized_stat: 1, // Agility
                        mood_penalty: 5,
                        experience_penalty: 0,
                    },
                })
            }
        }
    }
    
    /// Checks if a pet is compatible with an environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `environment` - The environment
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if compatible, Err otherwise
    fn check_compatibility(
        pet: &PetNft<T>,
        environment: &Environment,
    ) -> DispatchResult {
        // In a real implementation, this would check various factors
        // such as elemental affinity, personality traits, etc.
        // For now, we'll just do a simple check based on elemental affinity
        
        // Pets with opposite elemental affinities to the environment might not be compatible
        if (pet.primary_elemental_affinity as u8 + environment.primary_element) % 8 == 4 {
            // 50% chance of incompatibility for opposite elements
            let (random_seed, _) = T::PetRandomness::random_seed();
            let random_value = random_seed.using_encoded(|encoded| {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&encoded[0..4]);
                u32::from_le_bytes(buf)
            });
            
            if random_value % 2 == 0 {
                return Err(Error::<T>::IncompatibleEnvironment.into());
            }
        }
        
        Ok(())
    }
    
    /// Calculates a pet's adaptation level to an environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `environment` - The environment
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The adaptation level (0-255), or an error
    fn calculate_adaptation_level(
        pet: &PetNft<T>,
        environment: &Environment,
    ) -> Result<u8, DispatchError> {
        // In a real implementation, this would calculate the adaptation level
        // based on various factors such as elemental affinity, personality traits, etc.
        // For now, we'll use a simple algorithm
        
        // Base adaptation level
        let mut adaptation_level = 128; // 50%
        
        // Adjust based on elemental affinity
        if pet.primary_elemental_affinity as u8 == environment.primary_element {
            // Same primary element: +20% adaptation
            adaptation_level = adaptation_level.saturating_add(51);
        } else if pet.primary_elemental_affinity as u8 == environment.secondary_element {
            // Same secondary element: +10% adaptation
            adaptation_level = adaptation_level.saturating_add(25);
        } else if (pet.primary_elemental_affinity as u8 + environment.primary_element) % 8 == 4 {
            // Opposite primary element: -20% adaptation
            adaptation_level = adaptation_level.saturating_sub(51);
        }
        
        // Adjust based on pet level
        if pet.level > 10 {
            // High level: +10% adaptation
            adaptation_level = adaptation_level.saturating_add(25);
        } else if pet.level < 5 {
            // Low level: -10% adaptation
            adaptation_level = adaptation_level.saturating_sub(25);
        }
        
        // Adjust based on environment difficulty
        let difficulty_adjustment = environment.adaptation_difficulty / 2;
        adaptation_level = adaptation_level.saturating_sub(difficulty_adjustment);
        
        Ok(adaptation_level)
    }
    
    /// Applies the effects of adapting to an environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `environment` - The environment
    /// * `adaptation_level` - The pet's adaptation level to the environment
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn apply_adaptation_effects(
        pet_id: PetId,
        environment: &Environment,
        adaptation_level: u8,
    ) -> DispatchResult {
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Calculate the effectiveness of benefits and challenges based on adaptation level
            let benefit_effectiveness = adaptation_level as u16 * 100 / 255;
            let challenge_effectiveness = (255 - adaptation_level) as u16 * 100 / 255;
            
            // Apply stat boost
            match environment.benefits.boosted_stat {
                0 => { // Strength
                    let boost = (environment.benefits.stat_boost as u16 * benefit_effectiveness / 100) as u8;
                    pet.base_strength = pet.base_strength.saturating_add(boost);
                },
                1 => { // Agility
                    let boost = (environment.benefits.stat_boost as u16 * benefit_effectiveness / 100) as u8;
                    pet.base_agility = pet.base_agility.saturating_add(boost);
                },
                2 => { // Intelligence
                    let boost = (environment.benefits.stat_boost as u16 * benefit_effectiveness / 100) as u8;
                    pet.base_intelligence = pet.base_intelligence.saturating_add(boost);
                },
                3 => { // Vitality
                    let boost = (environment.benefits.stat_boost as u16 * benefit_effectiveness / 100) as u8;
                    pet.base_vitality = pet.base_vitality.saturating_add(boost);
                },
                _ => {} // No boost for other stats
            }
            
            // Apply stat penalty
            match environment.challenges.penalized_stat {
                0 => { // Strength
                    let penalty = (environment.challenges.stat_penalty as u16 * challenge_effectiveness / 100) as u8;
                    pet.base_strength = pet.base_strength.saturating_sub(penalty);
                },
                1 => { // Agility
                    let penalty = (environment.challenges.stat_penalty as u16 * challenge_effectiveness / 100) as u8;
                    pet.base_agility = pet.base_agility.saturating_sub(penalty);
                },
                2 => { // Intelligence
                    let penalty = (environment.challenges.stat_penalty as u16 * challenge_effectiveness / 100) as u8;
                    pet.base_intelligence = pet.base_intelligence.saturating_sub(penalty);
                },
                3 => { // Vitality
                    let penalty = (environment.challenges.stat_penalty as u16 * challenge_effectiveness / 100) as u8;
                    pet.base_vitality = pet.base_vitality.saturating_sub(penalty);
                },
                _ => {} // No penalty for other stats
            }
            
            // Apply mood boost
            let mood_boost = (environment.benefits.mood_boost as u16 * benefit_effectiveness / 100) as u8;
            pet.mood_indicator = pet.mood_indicator
                .saturating_add(mood_boost)
                .min(T::MaxMoodValue::get());
            
            // Apply mood penalty
            let mood_penalty = (environment.challenges.mood_penalty as u16 * challenge_effectiveness / 100) as u8;
            pet.mood_indicator = pet.mood_indicator.saturating_sub(mood_penalty);
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })
    }
    
    /// Records an adaptation for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `environment_type` - The type of environment
    /// * `adaptation_level` - The pet's adaptation level to the environment
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn record_adaptation(
        pet_id: PetId,
        environment_type: u8,
        adaptation_level: u8,
    ) -> DispatchResult {
        crate::PetEnvironmentalAdaptations::<T>::try_mutate(pet_id, |adaptations| -> DispatchResult {
            // Check if the pet already has an adaptation to this environment
            for i in 0..adaptations.len() {
                if let Some((env_type, _)) = adaptations.get(i) {
                    if *env_type == environment_type {
                        // Update the existing adaptation
                        adaptations.set(i, (environment_type, adaptation_level))?;
                        return Ok(());
                    }
                }
            }
            
            // Add the new adaptation
            adaptations.try_push((environment_type, adaptation_level))
                .map_err(|_| Error::<T>::TooManyEnvironmentalAdaptations)?;
            
            Ok(())
        })
    }
    
    /// Gets a pet's adaptation level to an environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `environment_type` - The type of environment
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The adaptation level (0-255), or an error
    pub fn get_adaptation_level(
        pet_id: PetId,
        environment_type: u8,
    ) -> Result<u8, DispatchError> {
        let adaptations = crate::PetEnvironmentalAdaptations::<T>::get(pet_id);
        
        for (env_type, adaptation_level) in adaptations.iter() {
            if *env_type == environment_type {
                return Ok(*adaptation_level);
            }
        }
        
        // If the pet doesn't have an adaptation to this environment,
        // calculate a base adaptation level
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        let environment = Self::get_environment(environment_type)?;
        
        Self::calculate_adaptation_level(&pet, &environment)
    }
    
    /// Gets all of a pet's environmental adaptations.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u8, u8)>` - The environmental adaptations (environment_type, adaptation_level)
    pub fn get_all_adaptations(
        pet_id: PetId,
    ) -> Vec<(u8, u8)> {
        crate::PetEnvironmentalAdaptations::<T>::get(pet_id).to_vec()
    }
}//! # Environmental Adaptation System
//!
//! This module provides a system for pets to adapt to different environments,
//! allowing them to thrive in different "regions" of the blockchain ecosystem.

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::Get,
    BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;
use scale_info::TypeInfo;
use crate::{Config, Error, PetId, PetNft, ElementType};

/// Represents an environment that pets can adapt to.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Environment {
    /// The environment type
    pub environment_type: u8,
    
    /// The primary element of the environment
    pub primary_element: u8,
    
    /// The secondary element of the environment
    pub secondary_element: u8,
    
    /// The difficulty of adapting to this environment (0-255)
    pub adaptation_difficulty: u8,
    
    /// The benefits of adapting to this environment
    pub benefits: EnvironmentBenefits,
    
    /// The challenges of adapting to this environment
    pub challenges: EnvironmentChallenges,
}

/// Represents the benefits of adapting to an environment.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct EnvironmentBenefits {
    /// The stat boost provided by the environment
    pub stat_boost: u8,
    
    /// The stat that is boosted
    pub boosted_stat: u8,
    
    /// The mood boost provided by the environment
    pub mood_boost: u8,
    
    /// The experience boost provided by the environment (percentage)
    pub experience_boost: u8,
}

/// Represents the challenges of adapting to an environment.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct EnvironmentChallenges {
    /// The stat penalty imposed by the environment
    pub stat_penalty: u8,
    
    /// The stat that is penalized
    pub penalized_stat: u8,
    
    /// The mood penalty imposed by the environment
    pub mood_penalty: u8,
    
    /// The experience penalty imposed by the environment (percentage)
    pub experience_penalty: u8,
}

/// Environment types.
pub enum EnvironmentType {
    Forest = 0,
    Mountain = 1,
    Desert = 2,
    Ocean = 3,
    Tundra = 4,
    Volcano = 5,
    City = 6,
    Space = 7,
    Digital = 8,
    Ethereal = 9,
}

/// A system for managing pet adaptations to different environments.
pub struct EnvironmentalAdaptationSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> EnvironmentalAdaptationSystem<T> {
    /// Adapts a pet to a new environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `environment_type` - The type of environment to adapt to
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn adapt_to_environment(
        pet_id: PetId,
        environment_type: u8,
    ) -> DispatchResult {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the environment
        let environment = Self::get_environment(environment_type)?;
        
        // Check compatibility
        Self::check_compatibility(&pet, &environment)?;
        
        // Calculate adaptation level
        let adaptation_level = Self::calculate_adaptation_level(&pet, &environment)?;
        
        // Apply adaptation effects
        Self::apply_adaptation_effects(pet_id, &environment, adaptation_level)?;
        
        // Record the adaptation
        Self::record_adaptation(pet_id, environment_type, adaptation_level)?;
        
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::EnvironmentalAdaptation {
            pet_id,
            environment_type,
            adaptation_level,
            timestamp: current_block,
        });
        
        // Potentially evolve personality traits based on the adaptation
        if adaptation_level > 200 {
            // High adaptation level: evolve the "Adaptable" trait
            crate::personality::PersonalityEvolutionSystem::<T>::evolve_personality(
                pet_id,
                crate::personality::EvolutionCatalyst::EnvironmentalChange as u8,
                adaptation_level,
            )?;
        }
        
        Ok(())
    }
    
    /// Gets an environment by type.
    /// 
    /// # Parameters
    /// 
    /// * `environment_type` - The type of environment
    /// 
    /// # Returns
    /// 
    /// * `Result<Environment, DispatchError>` - The environment, or an error
    fn get_environment(environment_type: u8) -> Result<Environment, DispatchError> {
        // In a real implementation, this would get the environment from storage
        // For now, we'll just return a hardcoded environment based on the type
        
        match environment_type {
            0 => { // Forest
                Ok(Environment {
                    environment_type,
                    primary_element: ElementType::Nature as u8,
                    secondary_element: ElementType::Water as u8,
                    adaptation_difficulty: 50,
                    benefits: EnvironmentBenefits {
                        stat_boost: 10,
                        boosted_stat: 1, // Agility
                        mood_boost: 5,
                        experience_boost: 10,
                    },
                    challenges: EnvironmentChallenges {
                        stat_penalty: 5,
                        penalized_stat: 0, // Strength
                        mood_penalty: 0,
                        experience_penalty: 0,
                    },
                })
            },
            1 => { // Mountain
                Ok(Environment {
                    environment_type,
                    primary_element: ElementType::Earth as u8,
                    secondary_element: ElementType::Air as u8,
                    adaptation_difficulty: 100,
                    benefits: EnvironmentBenefits {
                        stat_boost: 15,
                        boosted_stat: 0, // Strength
                        mood_boost: 0,
                        experience_boost: 15,
                    },
                    challenges: EnvironmentChallenges {
                        stat_penalty: 10,
                        penalized_stat: 1, // Agility
                        mood_penalty: 5,
                        experience_penalty: 0,
                    },
                })
            },
            2 => { // Desert
                Ok(Environment {
                    environment_type,
                    primary_element: ElementType::Fire as u8,
                    secondary_element: ElementType::Earth as u8,
                    adaptation_difficulty: 150,
                    benefits: EnvironmentBenefits {
                        stat_boost: 20,
                        boosted_stat: 3, // Vitality
                        mood_boost: 0,
                        experience_boost: 20,
                    },
                    challenges: EnvironmentChallenges {
                        stat_penalty: 15,
                        penalized_stat: 2, // Intelligence
                        mood_penalty: 10,
                        experience_penalty: 0,
                    },
                })
            },
            _ => {
                // Default to a generic environment
                Ok(Environment {
                    environment_type,
                    primary_element: ElementType::Neutral as u8,
                    secondary_element: ElementType::Neutral as u8,
                    adaptation_difficulty: 100,
                    benefits: EnvironmentBenefits {
                        stat_boost: 10,
                        boosted_stat: 0, // Strength
                        mood_boost: 5,
                        experience_boost: 10,
                    },
                    challenges: EnvironmentChallenges {
                        stat_penalty: 5,
                        penalized_stat: 1, // Agility
                        mood_penalty: 5,
                        experience_penalty: 0,
                    },
                })
            }
        }
    }
    
    /// Checks if a pet is compatible with an environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `environment` - The environment
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if compatible, Err otherwise
    fn check_compatibility(
        pet: &PetNft<T>,
        environment: &Environment,
    ) -> DispatchResult {
        // In a real implementation, this would check various factors
        // such as elemental affinity, personality traits, etc.
        // For now, we'll just do a simple check based on elemental affinity
        
        // Pets with opposite elemental affinities to the environment might not be compatible
        if (pet.primary_elemental_affinity as u8 + environment.primary_element) % 8 == 4 {
            // 50% chance of incompatibility for opposite elements
            let (random_seed, _) = T::PetRandomness::random_seed();
            let random_value = random_seed.using_encoded(|encoded| {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&encoded[0..4]);
                u32::from_le_bytes(buf)
            });
            
            if random_value % 2 == 0 {
                return Err(Error::<T>::IncompatibleEnvironment.into());
            }
        }
        
        Ok(())
    }
    
    /// Calculates a pet's adaptation level to an environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// * `environment` - The environment
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The adaptation level (0-255), or an error
    fn calculate_adaptation_level(
        pet: &PetNft<T>,
        environment: &Environment,
    ) -> Result<u8, DispatchError> {
        // In a real implementation, this would calculate the adaptation level
        // based on various factors such as elemental affinity, personality traits, etc.
        // For now, we'll use a simple algorithm
        
        // Base adaptation level
        let mut adaptation_level = 128; // 50%
        
        // Adjust based on elemental affinity
        if pet.primary_elemental_affinity as u8 == environment.primary_element {
            // Same primary element: +20% adaptation
            adaptation_level = adaptation_level.saturating_add(51);
        } else if pet.primary_elemental_affinity as u8 == environment.secondary_element {
            // Same secondary element: +10% adaptation
            adaptation_level = adaptation_level.saturating_add(25);
        } else if (pet.primary_elemental_affinity as u8 + environment.primary_element) % 8 == 4 {
            // Opposite primary element: -20% adaptation
            adaptation_level = adaptation_level.saturating_sub(51);
        }
        
        // Adjust based on pet level
        if pet.level > 10 {
            // High level: +10% adaptation
            adaptation_level = adaptation_level.saturating_add(25);
        } else if pet.level < 5 {
            // Low level: -10% adaptation
            adaptation_level = adaptation_level.saturating_sub(25);
        }
        
        // Adjust based on environment difficulty
        let difficulty_adjustment = environment.adaptation_difficulty / 2;
        adaptation_level = adaptation_level.saturating_sub(difficulty_adjustment);
        
        Ok(adaptation_level)
    }
    
    /// Applies the effects of adapting to an environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `environment` - The environment
    /// * `adaptation_level` - The pet's adaptation level to the environment
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn apply_adaptation_effects(
        pet_id: PetId,
        environment: &Environment,
        adaptation_level: u8,
    ) -> DispatchResult {
        crate::PetNfts::<T>::try_mutate(pet_id, |pet_opt| -> DispatchResult {
            let pet = pet_opt.as_mut().ok_or(Error::<T>::PetNotFound)?;
            
            // Calculate the effectiveness of benefits and challenges based on adaptation level
            let benefit_effectiveness = adaptation_level as u16 * 100 / 255;
            let challenge_effectiveness = (255 - adaptation_level) as u16 * 100 / 255;
            
            // Apply stat boost
            match environment.benefits.boosted_stat {
                0 => { // Strength
                    let boost = (environment.benefits.stat_boost as u16 * benefit_effectiveness / 100) as u8;
                    pet.base_strength = pet.base_strength.saturating_add(boost);
                },
                1 => { // Agility
                    let boost = (environment.benefits.stat_boost as u16 * benefit_effectiveness / 100) as u8;
                    pet.base_agility = pet.base_agility.saturating_add(boost);
                },
                2 => { // Intelligence
                    let boost = (environment.benefits.stat_boost as u16 * benefit_effectiveness / 100) as u8;
                    pet.base_intelligence = pet.base_intelligence.saturating_add(boost);
                },
                3 => { // Vitality
                    let boost = (environment.benefits.stat_boost as u16 * benefit_effectiveness / 100) as u8;
                    pet.base_vitality = pet.base_vitality.saturating_add(boost);
                },
                _ => {} // No boost for other stats
            }
            
            // Apply stat penalty
            match environment.challenges.penalized_stat {
                0 => { // Strength
                    let penalty = (environment.challenges.stat_penalty as u16 * challenge_effectiveness / 100) as u8;
                    pet.base_strength = pet.base_strength.saturating_sub(penalty);
                },
                1 => { // Agility
                    let penalty = (environment.challenges.stat_penalty as u16 * challenge_effectiveness / 100) as u8;
                    pet.base_agility = pet.base_agility.saturating_sub(penalty);
                },
                2 => { // Intelligence
                    let penalty = (environment.challenges.stat_penalty as u16 * challenge_effectiveness / 100) as u8;
                    pet.base_intelligence = pet.base_intelligence.saturating_sub(penalty);
                },
                3 => { // Vitality
                    let penalty = (environment.challenges.stat_penalty as u16 * challenge_effectiveness / 100) as u8;
                    pet.base_vitality = pet.base_vitality.saturating_sub(penalty);
                },
                _ => {} // No penalty for other stats
            }
            
            // Apply mood boost
            let mood_boost = (environment.benefits.mood_boost as u16 * benefit_effectiveness / 100) as u8;
            pet.mood_indicator = pet.mood_indicator
                .saturating_add(mood_boost)
                .min(T::MaxMoodValue::get());
            
            // Apply mood penalty
            let mood_penalty = (environment.challenges.mood_penalty as u16 * challenge_effectiveness / 100) as u8;
            pet.mood_indicator = pet.mood_indicator.saturating_sub(mood_penalty);
            
            // Update the pet's state version
            pet.state_version = pet.state_version.saturating_add(1);
            
            // Update the last state update block
            pet.last_state_update_block = frame_system::Pallet::<T>::block_number();
            
            Ok(())
        })
    }
    
    /// Records an adaptation for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `environment_type` - The type of environment
    /// * `adaptation_level` - The pet's adaptation level to the environment
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn record_adaptation(
        pet_id: PetId,
        environment_type: u8,
        adaptation_level: u8,
    ) -> DispatchResult {
        crate::PetEnvironmentalAdaptations::<T>::try_mutate(pet_id, |adaptations| -> DispatchResult {
            // Check if the pet already has an adaptation to this environment
            for i in 0..adaptations.len() {
                if let Some((env_type, _)) = adaptations.get(i) {
                    if *env_type == environment_type {
                        // Update the existing adaptation
                        adaptations.set(i, (environment_type, adaptation_level))?;
                        return Ok(());
                    }
                }
            }
            
            // Add the new adaptation
            adaptations.try_push((environment_type, adaptation_level))
                .map_err(|_| Error::<T>::TooManyEnvironmentalAdaptations)?;
            
            Ok(())
        })
    }
    
    /// Gets a pet's adaptation level to an environment.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `environment_type` - The type of environment
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The adaptation level (0-255), or an error
    pub fn get_adaptation_level(
        pet_id: PetId,
        environment_type: u8,
    ) -> Result<u8, DispatchError> {
        let adaptations = crate::PetEnvironmentalAdaptations::<T>::get(pet_id);
        
        for (env_type, adaptation_level) in adaptations.iter() {
            if *env_type == environment_type {
                return Ok(*adaptation_level);
            }
        }
        
        // If the pet doesn't have an adaptation to this environment,
        // calculate a base adaptation level
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        let environment = Self::get_environment(environment_type)?;
        
        Self::calculate_adaptation_level(&pet, &environment)
    }
    
    /// Gets all of a pet's environmental adaptations.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u8, u8)>` - The environmental adaptations (environment_type, adaptation_level)
    pub fn get_all_adaptations(
        pet_id: PetId,
    ) -> Vec<(u8, u8)> {
        crate::PetEnvironmentalAdaptations::<T>::get(pet_id).to_vec()
    }
}