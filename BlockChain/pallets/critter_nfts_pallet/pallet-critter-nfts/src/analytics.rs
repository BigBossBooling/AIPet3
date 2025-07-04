//! # Advanced Analytics Dashboard
//!
//! This module provides a comprehensive analytics system for pets,
//! giving owners deep insights into their pets' development and the broader ecosystem.

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

/// Represents an analytics report for a pet.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AnalyticsReport<T: Config> {
    /// The pet ID
    pub pet_id: PetId,
    
    /// The timestamp when the report was generated
    pub timestamp: BlockNumberFor<T>,
    
    /// The pet's basic stats
    pub basic_stats: BasicStats,
    
    /// The pet's interaction statistics
    pub interaction_stats: InteractionStats,
    
    /// The pet's skill statistics
    pub skill_stats: SkillStats,
    
    /// The pet's achievement statistics
    pub achievement_stats: AchievementStats,
    
    /// The pet's social statistics
    pub social_stats: SocialStats,
    
    /// The pet's environmental adaptation statistics
    pub environmental_stats: EnvironmentalStats,
    
    /// The pet's lifecycle statistics
    pub lifecycle_stats: LifecycleStats,
    
    /// The pet's memory statistics
    pub memory_stats: MemoryStats,
    
    /// The pet's mood statistics
    pub mood_stats: MoodStats,
    
    /// The pet's personality statistics
    pub personality_stats: PersonalityStats,
}

/// Represents a pet's basic stats.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BasicStats {
    /// The pet's level
    pub level: u32,
    
    /// The pet's experience points
    pub experience: u32,
    
    /// The pet's age (in blocks)
    pub age: u64,
    
    /// The pet's strength
    pub strength: u8,
    
    /// The pet's agility
    pub agility: u8,
    
    /// The pet's intelligence
    pub intelligence: u8,
    
    /// The pet's vitality
    pub vitality: u8,
    
    /// The pet's mood
    pub mood: u8,
    
    /// The pet's state version
    pub state_version: u32,
}

/// Represents a pet's interaction statistics.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct InteractionStats {
    /// The number of times the pet has been fed
    pub feeding_count: u32,
    
    /// The number of times the pet has been played with
    pub playing_count: u32,
    
    /// The number of times the pet has been trained
    pub training_count: u32,
    
    /// The number of times the pet has been neglected
    pub neglect_count: u32,
    
    /// The average time between interactions (in blocks)
    pub average_interaction_interval: u64,
    
    /// The longest time between interactions (in blocks)
    pub longest_interaction_interval: u64,
    
    /// The shortest time between interactions (in blocks)
    pub shortest_interaction_interval: u64,
}

/// Represents a pet's skill statistics.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SkillStats {
    /// The number of skills the pet has
    pub skill_count: u32,
    
    /// The pet's highest skill level
    pub highest_skill_level: u8,
    
    /// The pet's average skill level
    pub average_skill_level: u8,
    
    /// The pet's most recently trained skill
    pub most_recent_skill: u8,
    
    /// The pet's most frequently trained skill
    pub most_frequent_skill: u8,
}

/// Represents a pet's achievement statistics.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AchievementStats {
    /// The number of achievements the pet has earned
    pub achievement_count: u32,
    
    /// The pet's most recent achievement
    pub most_recent_achievement: u32,
    
    /// The pet's rarest achievement
    pub rarest_achievement: u32,
    
    /// The pet's achievement categories
    pub achievement_categories: Vec<(u8, u32)>, // (category, count)
}

/// Represents a pet's social statistics.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SocialStats {
    /// The number of social interactions the pet has had
    pub interaction_count: u32,
    
    /// The number of unique pets the pet has interacted with
    pub unique_interaction_count: u32,
    
    /// The pet's most frequent interaction type
    pub most_frequent_interaction_type: u8,
    
    /// The pet's most frequent interaction outcome
    pub most_frequent_interaction_outcome: u8,
    
    /// The pet's average interaction outcome
    pub average_interaction_outcome: u8,
}

/// Represents a pet's environmental adaptation statistics.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct EnvironmentalStats {
    /// The number of environments the pet has adapted to
    pub adaptation_count: u32,
    
    /// The pet's highest adaptation level
    pub highest_adaptation_level: u8,
    
    /// The pet's average adaptation level
    pub average_adaptation_level: u8,
    
    /// The pet's most recent adaptation
    pub most_recent_adaptation: u8,
}

/// Represents a pet's lifecycle statistics.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LifecycleStats {
    /// The number of lifecycle events the pet has experienced
    pub event_count: u32,
    
    /// The pet's most recent lifecycle event
    pub most_recent_event: u8,
    
    /// The pet's current lifecycle stage
    pub current_stage: u8,
    
    /// The pet's next available lifecycle event
    pub next_event: u8,
}

/// Represents a pet's memory statistics.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct MemoryStats {
    /// The number of memories the pet has
    pub memory_count: u32,
    
    /// The pet's most significant memory
    pub most_significant_memory: u8,
    
    /// The pet's most recent memory
    pub most_recent_memory: u8,
    
    /// The pet's memory categories
    pub memory_categories: Vec<(u8, u32)>, // (category, count)
}

/// Represents a pet's mood statistics.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct MoodStats {
    /// The pet's average mood
    pub average_mood: u8,
    
    /// The pet's mood volatility
    pub mood_volatility: u8,
    
    /// The pet's mood trend
    pub mood_trend: i8,
    
    /// The pet's mood contagion susceptibility
    pub mood_contagion_susceptibility: u8,
    
    /// The pet's mood contagion influence
    pub mood_contagion_influence: u8,
}

/// Represents a pet's personality statistics.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PersonalityStats {
    /// The number of personality traits the pet has
    pub trait_count: u32,
    
    /// The pet's dominant trait
    pub dominant_trait: u8,
    
    /// The pet's most recently evolved trait
    pub most_recent_trait: u8,
    
    /// The pet's trait stability
    pub trait_stability: u8,
    
    /// The pet's trait diversity
    pub trait_diversity: u8,
}

/// A system for generating and managing pet analytics.
pub struct PetAnalyticsDashboard<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> PetAnalyticsDashboard<T> {
    /// Generates a comprehensive analytics report for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<u8>, DispatchError>` - The compressed analytics report, or an error
    pub fn generate_analytics_report(
        pet_id: PetId,
    ) -> Result<Vec<u8>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Generate the report
        let report = AnalyticsReport {
            pet_id,
            timestamp: current_block,
            basic_stats: Self::generate_basic_stats(&pet)?,
            interaction_stats: Self::generate_interaction_stats(pet_id)?,
            skill_stats: Self::generate_skill_stats(pet_id)?,
            achievement_stats: Self::generate_achievement_stats(pet_id)?,
            social_stats: Self::generate_social_stats(pet_id)?,
            environmental_stats: Self::generate_environmental_stats(pet_id)?,
            lifecycle_stats: Self::generate_lifecycle_stats(pet_id)?,
            memory_stats: Self::generate_memory_stats(pet_id)?,
            mood_stats: Self::generate_mood_stats(pet_id)?,
            personality_stats: Self::generate_personality_stats(pet_id)?,
        };
        
        // Encode the report
        let encoded_report = report.encode();
        
        // Store the report
        Self::store_analytics_report(pet_id, &encoded_report)?;
        
        // Emit an event
        crate::Pallet::<T>::deposit_event(crate::Event::AnalyticsReportGenerated {
            pet_id,
            report_size: encoded_report.len() as u32,
            timestamp: current_block,
        });
        
        Ok(encoded_report)
    }
    
    /// Generates basic stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// 
    /// # Returns
    /// 
    /// * `Result<BasicStats, DispatchError>` - The basic stats, or an error
    fn generate_basic_stats(
        pet: &PetNft<T>,
    ) -> Result<BasicStats, DispatchError> {
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number();
        
        // Calculate the pet's age
        let age = current_block.saturated_into::<u64>().saturating_sub(pet.last_state_update_block.saturated_into::<u64>());
        
        Ok(BasicStats {
            level: pet.level,
            experience: pet.experience_points,
            age,
            strength: pet.base_strength,
            agility: pet.base_agility,
            intelligence: pet.base_intelligence,
            vitality: pet.base_vitality,
            mood: pet.mood_indicator,
            state_version: pet.state_version,
        })
    }
    
    /// Generates interaction stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<InteractionStats, DispatchError>` - The interaction stats, or an error
    fn generate_interaction_stats(
        pet_id: PetId,
    ) -> Result<InteractionStats, DispatchError> {
        // In a real implementation, this would analyze the pet's interaction history
        // For now, we'll just return placeholder stats
        
        Ok(InteractionStats {
            feeding_count: 10,
            playing_count: 15,
            training_count: 5,
            neglect_count: 2,
            average_interaction_interval: 100,
            longest_interaction_interval: 200,
            shortest_interaction_interval: 50,
        })
    }
    
    /// Generates skill stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<SkillStats, DispatchError>` - The skill stats, or an error
    fn generate_skill_stats(
        pet_id: PetId,
    ) -> Result<SkillStats, DispatchError> {
        // Get the pet's skills
        let skills = crate::PetSkills::<T>::get(pet_id);
        
        // Calculate skill stats
        let skill_count = skills.len() as u32;
        
        let mut highest_skill_level = 0;
        let mut total_skill_level = 0;
        
        for (_, level) in skills.iter() {
            highest_skill_level = highest_skill_level.max(*level);
            total_skill_level += *level as u32;
        }
        
        let average_skill_level = if skill_count > 0 {
            (total_skill_level / skill_count) as u8
        } else {
            0
        };
        
        // In a real implementation, we would determine the most recent and most frequent skills
        // For now, we'll just use placeholders
        let most_recent_skill = if let Some((skill, _)) = skills.last() {
            *skill
        } else {
            0
        };
        
        let most_frequent_skill = if let Some((skill, _)) = skills.first() {
            *skill
        } else {
            0
        };
        
        Ok(SkillStats {
            skill_count,
            highest_skill_level,
            average_skill_level,
            most_recent_skill,
            most_frequent_skill,
        })
    }
    
    /// Generates achievement stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<AchievementStats, DispatchError>` - The achievement stats, or an error
    fn generate_achievement_stats(
        pet_id: PetId,
    ) -> Result<AchievementStats, DispatchError> {
        // Get the pet's achievements
        let achievements = crate::PetAchievements::<T>::get(pet_id);
        
        // Calculate achievement stats
        let achievement_count = achievements.len() as u32;
        
        let most_recent_achievement = if let Some((id, _)) = achievements.last() {
            *id
        } else {
            0
        };
        
        // In a real implementation, we would determine the rarest achievement
        // For now, we'll just use a placeholder
        let rarest_achievement = if let Some((id, _)) = achievements.first() {
            *id
        } else {
            0
        };
        
        // In a real implementation, we would categorize achievements
        // For now, we'll just use placeholders
        let achievement_categories = vec![
            (0, achievement_count), // All achievements in category 0
        ];
        
        Ok(AchievementStats {
            achievement_count,
            most_recent_achievement,
            rarest_achievement,
            achievement_categories,
        })
    }
    
    /// Generates social stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<SocialStats, DispatchError>` - The social stats, or an error
    fn generate_social_stats(
        pet_id: PetId,
    ) -> Result<SocialStats, DispatchError> {
        // Get the pet's social interactions
        let interactions = crate::PetSocialInteractions::<T>::get(pet_id);
        
        // Calculate social stats
        let interaction_count = interactions.len() as u32;
        
        // Count unique pets
        let mut unique_pets = Vec::new();
        for (other_pet_id, _, _, _) in interactions.iter() {
            if !unique_pets.contains(other_pet_id) {
                unique_pets.push(*other_pet_id);
            }
        }
        
        let unique_interaction_count = unique_pets.len() as u32;
        
        // In a real implementation, we would determine the most frequent interaction type and outcome
        // For now, we'll just use placeholders
        let most_frequent_interaction_type = if let Some((_, interaction_type, _, _)) = interactions.first() {
            *interaction_type
        } else {
            0
        };
        
        let most_frequent_interaction_outcome = if let Some((_, _, outcome, _)) = interactions.first() {
            *outcome
        } else {
            0
        };
        
        // Calculate average outcome
        let mut total_outcome = 0;
        for (_, _, outcome, _) in interactions.iter() {
            total_outcome += *outcome as u32;
        }
        
        let average_interaction_outcome = if interaction_count > 0 {
            (total_outcome / interaction_count) as u8
        } else {
            0
        };
        
        Ok(SocialStats {
            interaction_count,
            unique_interaction_count,
            most_frequent_interaction_type,
            most_frequent_interaction_outcome,
            average_interaction_outcome,
        })
    }
    
    /// Generates environmental stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<EnvironmentalStats, DispatchError>` - The environmental stats, or an error
    fn generate_environmental_stats(
        pet_id: PetId,
    ) -> Result<EnvironmentalStats, DispatchError> {
        // Get the pet's environmental adaptations
        let adaptations = crate::PetEnvironmentalAdaptations::<T>::get(pet_id);
        
        // Calculate environmental stats
        let adaptation_count = adaptations.len() as u32;
        
        let mut highest_adaptation_level = 0;
        let mut total_adaptation_level = 0;
        
        for (_, level) in adaptations.iter() {
            highest_adaptation_level = highest_adaptation_level.max(*level);
            total_adaptation_level += *level as u32;
        }
        
        let average_adaptation_level = if adaptation_count > 0 {
            (total_adaptation_level / adaptation_count) as u8
        } else {
            0
        };
        
        let most_recent_adaptation = if let Some((environment_type, _)) = adaptations.last() {
            *environment_type
        } else {
            0
        };
        
        Ok(EnvironmentalStats {
            adaptation_count,
            highest_adaptation_level,
            average_adaptation_level,
            most_recent_adaptation,
        })
    }
    
    /// Generates lifecycle stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<LifecycleStats, DispatchError>` - The lifecycle stats, or an error
    fn generate_lifecycle_stats(
        pet_id: PetId,
    ) -> Result<LifecycleStats, DispatchError> {
        // Get the pet's lifecycle events
        let events = crate::PetLifecycleEvents::<T>::get(pet_id);
        
        // Calculate lifecycle stats
        let event_count = events.len() as u32;
        
        let most_recent_event = if let Some((event_type, _)) = events.last() {
            *event_type
        } else {
            0
        };
        
        // Determine the pet's current lifecycle stage
        let current_stage = if events.iter().any(|(event_type, _)| *event_type == 2) {
            2 // Adulthood
        } else if events.iter().any(|(event_type, _)| *event_type == 1) {
            1 // Adolescence
        } else if events.iter().any(|(event_type, _)| *event_type == 0) {
            0 // Birth
        } else {
            0 // Default to Birth
        };
        
        // Determine the pet's next available lifecycle event
        let next_event = if current_stage == 0 {
            1 // Adolescence
        } else if current_stage == 1 {
            2 // Adulthood
        } else if current_stage == 2 {
            3 // Mastery
        } else {
            0 // Default to Birth
        };
        
        Ok(LifecycleStats {
            event_count,
            most_recent_event,
            current_stage,
            next_event,
        })
    }
    
    /// Generates memory stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<MemoryStats, DispatchError>` - The memory stats, or an error
    fn generate_memory_stats(
        pet_id: PetId,
    ) -> Result<MemoryStats, DispatchError> {
        // Get the pet's memories
        let memories = crate::PetMemories::<T>::get(pet_id);
        
        // Calculate memory stats
        let memory_count = memories.len() as u32;
        
        // Find the most significant memory
        let mut most_significant_memory = 0;
        let mut highest_significance = 0;
        
        for (memory_type, significance, _, _) in memories.iter() {
            if *significance > highest_significance {
                highest_significance = *significance;
                most_significant_memory = *memory_type;
            }
        }
        
        let most_recent_memory = if let Some((memory_type, _, _, _)) = memories.last() {
            *memory_type
        } else {
            0
        };
        
        // Categorize memories
        let mut categories = Vec::new();
        let mut category_counts = [0u32; 12]; // 12 memory types
        
        for (memory_type, _, _, _) in memories.iter() {
            if *memory_type < 12 {
                category_counts[*memory_type as usize] += 1;
            }
        }
        
        for i in 0..12 {
            if category_counts[i] > 0 {
                categories.push((i as u8, category_counts[i]));
            }
        }
        
        Ok(MemoryStats {
            memory_count,
            most_significant_memory,
            most_recent_memory,
            memory_categories: categories,
        })
    }
    
    /// Generates mood stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<MoodStats, DispatchError>` - The mood stats, or an error
    fn generate_mood_stats(
        pet_id: PetId,
    ) -> Result<MoodStats, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // In a real implementation, this would analyze the pet's mood history
        // For now, we'll just use the current mood and some placeholders
        
        let average_mood = pet.mood_indicator;
        let mood_volatility = 50; // Placeholder
        let mood_trend = 0; // Placeholder
        
        // Calculate mood contagion stats
        let mood_contagion_susceptibility = crate::mood::MoodContagionSystem::<T>::calculate_susceptibility(pet_id)?;
        let mood_contagion_influence = crate::mood::MoodContagionSystem::<T>::calculate_influence(pet_id)?;
        
        Ok(MoodStats {
            average_mood,
            mood_volatility,
            mood_trend,
            mood_contagion_susceptibility,
            mood_contagion_influence,
        })
    }
    
    /// Generates personality stats for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<PersonalityStats, DispatchError>` - The personality stats, or an error
    fn generate_personality_stats(
        pet_id: PetId,
    ) -> Result<PersonalityStats, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Calculate personality stats
        let trait_count = pet.personality_traits.len() as u32;
        
        // In a real implementation, we would determine the dominant and most recent traits
        // For now, we'll just use placeholders
        let dominant_trait = if let Some(trait_str) = pet.personality_traits.first() {
            if let Some(&trait_type) = trait_str.get(0) {
                trait_type
            } else {
                0
            }
        } else {
            0
        };
        
        let most_recent_trait = if let Some(trait_str) = pet.personality_traits.last() {
            if let Some(&trait_type) = trait_str.get(0) {
                trait_type
            } else {
                0
            }
        } else {
            0
        };
        
        // In a real implementation, we would calculate trait stability and diversity
        // For now, we'll just use placeholders
        let trait_stability = 128; // 50%
        let trait_diversity = if trait_count > 0 {
            (trait_count * 255 / T::MaxPetPersonalityTraits::get()) as u8
        } else {
            0
        };
        
        Ok(PersonalityStats {
            trait_count,
            dominant_trait,
            most_recent_trait,
            trait_stability,
            trait_diversity,
        })
    }
    
    /// Stores an analytics report for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `report` - The encoded analytics report
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    fn store_analytics_report(
        pet_id: PetId,
        report: &[u8],
    ) -> DispatchResult {
        // Ensure the report isn't too large
        ensure!(
            report.len() <= T::MaxAnalyticsReportSize::get() as usize,
            Error::<T>::AnalyticsReportTooLarge
        );
        
        // Store the report
        let bounded_report: BoundedVec<u8, T::MaxAnalyticsReportSize> = 
            report.to_vec().try_into().map_err(|_| Error::<T>::AnalyticsReportTooLarge)?;
        
        crate::PetAnalyticsReports::<T>::insert(pet_id, bounded_report);
        
        Ok(())
    }
    
    /// Gets the latest analytics report for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<u8>, DispatchError>` - The encoded analytics report, or an error
    pub fn get_analytics_report(
        pet_id: PetId,
    ) -> Result<Vec<u8>, DispatchError> {
        let report = crate::PetAnalyticsReports::<T>::get(pet_id);
        
        ensure!(
            !report.is_empty(),
            Error::<T>::AnalyticsReportNotFound
        );
        
        Ok(report.to_vec())
    }
    
    /// Decodes an analytics report.
    /// 
    /// # Parameters
    /// 
    /// * `encoded_report` - The encoded analytics report
    /// 
    /// # Returns
    /// 
    /// * `Result<AnalyticsReport<T>, DispatchError>` - The decoded analytics report, or an error
    pub fn decode_analytics_report(
        encoded_report: &[u8],
    ) -> Result<AnalyticsReport<T>, DispatchError> {
        AnalyticsReport::<T>::decode(&mut &encoded_report[..])
            .map_err(|_| Error::<T>::AnalyticsReportDecodingFailed.into())
    }
}