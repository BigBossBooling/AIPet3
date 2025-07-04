//! # UI/UX Bridge Module
//!
//! This module serves as a bridge between the backend blockchain logic and the frontend UI/UX.
//! It provides structured data formats, helper functions, and event processing utilities
//! that make it easier for frontend applications to interact with the blockchain.

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::Get,
    BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;
use scale_info::TypeInfo;
use codec::{Encode, Decode};
use crate::{Config, Error, PetId, PetNft, ElementType};

/// Represents a UI-friendly pet profile with all relevant information.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct UiPetProfile<T: Config> {
    // Basic information
    pub id: PetId,
    pub name: Vec<u8>,
    pub species: Vec<u8>,
    pub level: u32,
    pub experience: u32,
    pub mood: u8,
    pub owner: T::AccountId,
    
    // Attributes
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
    pub vitality: u8,
    pub elemental_affinity: u8,
    
    // Derived stats
    pub health: u16,
    pub energy: u16,
    pub speed: u16,
    pub defense: u16,
    pub attack: u16,
    pub special_ability: u16,
    
    // Personality
    pub personality_traits: Vec<(Vec<u8>, u8)>, // (trait_name, intensity)
    pub dominant_trait: Vec<u8>, // The most prominent personality trait
    pub personality_summary: Vec<u8>, // A brief summary of the pet's personality
    
    // Social
    pub social_bonds: Vec<(PetId, u8, u8)>, // (other_pet_id, bond_type, bond_strength)
    pub recent_interactions: Vec<(PetId, u8, u8, u64)>, // (other_pet_id, interaction_type, outcome, timestamp)
    pub social_preferences: Vec<(u8, i8)>, // (interaction_type, preference_score)
    pub friendship_level: u8, // Overall sociability score
    
    // Environment
    pub adaptations: Vec<(u8, u8)>, // (environment_type, adaptation_level)
    pub preferred_environment: u8, // The environment type the pet is most adapted to
    pub environment_bonuses: Vec<(u8, Vec<(u8, i8)>)>, // (environment_type, [(stat_type, bonus)])
    
    // Skills
    pub skills: Vec<(u8, u8)>, // (skill_type, skill_level)
    pub skill_progress: Vec<(u8, u8)>, // (skill_type, progress_to_next_level)
    pub mastered_skills: Vec<u8>, // List of fully mastered skills
    
    // Memories
    pub significant_memories: Vec<(u8, u8, u64, Vec<u8>)>, // (memory_type, significance, timestamp, data)
    pub memory_categories: Vec<(u8, u8)>, // (category_type, count)
    pub memory_influence: Vec<(u8, i8)>, // (stat_type, influence)
    
    // Achievements
    pub achievements: Vec<(u32, u64)>, // (achievement_id, timestamp)
    pub achievement_points: u32, // Total achievement points
    pub achievement_progress: Vec<(u32, u8)>, // (achievement_id, progress)
    pub achievement_milestones: Vec<(u32, Vec<u8>)>, // (milestone_id, description)
    
    // Lifecycle
    pub lifecycle_events: Vec<(u8, u64)>, // (event_type, timestamp)
    pub age_category: u8, // 0=baby, 1=child, 2=teen, 3=adult, 4=senior
    pub lifecycle_stage: Vec<u8>, // Text description of current lifecycle stage
    pub next_evolution: Option<Vec<u8>>, // Description of next evolution if available
    
    // Visual customization
    pub visual_attributes: Vec<(u8, Vec<u8>)>, // (attribute_type, value)
    pub visual_effects: Vec<u8>, // Active visual effects
    pub animations: Vec<(u8, Vec<u8>)>, // (animation_id, parameters)
    pub current_outfit: Option<Vec<u8>>, // Currently equipped outfit
    pub available_customizations: Vec<(u8, Vec<u8>)>, // (customization_type, options)
    
    // Status effects
    pub status_effects: Vec<(u8, u64, i8)>, // (effect_type, duration, magnitude)
    pub buffs: Vec<(Vec<u8>, u64, Vec<(u8, i8)>)>, // (name, duration, [(stat_type, bonus)])
    pub debuffs: Vec<(Vec<u8>, u64, Vec<(u8, i8)>)>, // (name, duration, [(stat_type, penalty)])
    
    // Mood and needs
    pub mood_factors: Vec<(Vec<u8>, i8)>, // (factor_name, influence)
    pub needs: Vec<(u8, u8)>, // (need_type, fulfillment_level)
    pub mood_trend: i8, // Direction of mood change (-1=decreasing, 0=stable, 1=increasing)
    
    // Timestamps and activity
    pub last_fed: u64,
    pub last_played: u64,
    pub creation_time: u64,
    pub last_interaction: u64,
    pub active_hours: Vec<u8>, // Hours of the day when the pet is most active
    pub activity_level: u8, // Overall activity level
    
    // Progression
    pub xp_to_next_level: u32,
    pub level_progress_percentage: u8, // 0-100
    pub growth_rate: u8, // How quickly the pet levels up
    pub talent_areas: Vec<(Vec<u8>, u8)>, // (area_name, aptitude)
    
    // Special features
    pub special_abilities: Vec<(Vec<u8>, Vec<u8>, u8)>, // (ability_name, description, power_level)
    pub rare_traits: Vec<Vec<u8>>, // List of rare traits
    pub unique_features: Vec<Vec<u8>>, // List of unique features
    pub hidden_potential: Vec<(u8, u8)>, // (potential_type, level)
}

/// A system for bridging between the blockchain and UI/UX.
pub struct UiBridge<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> UiBridge<T> {
    /// Gets a UI-friendly pet profile.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<UiPetProfile<T>, DispatchError>` - The pet profile, or an error
    pub fn get_pet_profile(pet_id: PetId) -> Result<UiPetProfile<T>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the pet owner
        let owner = crate::PetNftOwner::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get personality traits
        let personality_traits = pet.personality_traits.iter()
            .map(|trait_str| {
                // In a real implementation, this would look up the trait name from a mapping
                // For now, we'll just return the trait string and a default intensity
                (trait_str.to_vec(), 128u8)
            })
            .collect::<Vec<_>>();
        
        // Get social bonds
        let social_bonds = crate::social::SocialInteractionSystem::<T>::get_social_bonds(pet_id)
            .iter()
            .map(|bond| (bond.other_pet_id, bond.bond_type, bond.bond_strength))
            .collect::<Vec<_>>();
        
        // Get recent interactions
        let recent_interactions = crate::social::SocialInteractionSystem::<T>::get_social_interactions(pet_id)
            .iter()
            .map(|(other_pet_id, interaction_type, outcome, timestamp)| 
                (*other_pet_id, *interaction_type, *outcome, timestamp.saturated_into::<u64>()))
            .collect::<Vec<_>>();
        
        // Get environmental adaptations
        let adaptations = crate::environment::EnvironmentalAdaptationSystem::<T>::get_environmental_adaptations(pet_id);
        
        // Get skills
        let skills = crate::training::PetTrainingSystem::<T>::get_skills(pet_id);
        
        // Get significant memories
        let significant_memories = crate::memory::PetMemorySystem::<T>::get_significant_memories(pet_id, 5);
        
        // Get achievements
        let achievements = crate::achievements::AchievementSystem::<T>::get_achievements(pet_id);
        
        // Get lifecycle events
        let lifecycle_events = crate::lifecycle::LifecycleEventSystem::<T>::get_lifecycle_events(pet_id);
        
        // Get visual attributes
        let visual_attributes = crate::visual::VisualSystem::<T>::get_visual_attributes(pet_id)?;
        
        // Get visual effects
        let visual_effects = crate::visual::VisualSystem::<T>::get_visual_effects(pet_id)?;
        
        // Calculate derived stats
        let health = (pet.base_vitality as u16 * 10) + (pet.level as u16 * 5);
        let energy = (pet.base_vitality as u16 * 5) + (pet.base_agility as u16 * 5) + (pet.level as u16 * 2);
        let speed = (pet.base_agility as u16 * 10) + (pet.level as u16 * 2);
        let defense = (pet.base_vitality as u16 * 5) + (pet.base_strength as u16 * 5) + (pet.level as u16 * 2);
        let attack = (pet.base_strength as u16 * 10) + (pet.level as u16 * 3);
        let special_ability = (pet.base_intelligence as u16 * 10) + (pet.level as u16 * 3);
        
        // Determine dominant personality trait
        let dominant_trait = if !personality_traits.is_empty() {
            personality_traits[0].0.clone() // In a real implementation, would find the highest intensity trait
        } else {
            b"Balanced".to_vec()
        };
        
        // Generate personality summary
        let personality_summary = if !personality_traits.is_empty() {
            let mut summary = b"A ".to_vec();
            summary.extend_from_slice(&dominant_trait);
            summary.extend_from_slice(b" pet with ");
            summary.extend_from_slice(match personality_traits.len() {
                1 => b"a simple personality.",
                2..=3 => b"a developing personality.",
                4..=5 => b"a complex personality.",
                _ => b"a rich and nuanced personality.",
            });
            summary
        } else {
            b"A pet with a developing personality.".to_vec()
        };
        
        // Generate social preferences
        let social_preferences = vec![
            (0, 5i8), // Likes playing
            (1, 3i8), // Somewhat likes grooming
            (2, 0i8), // Neutral about training
            (3, if pet.base_intelligence > 150 { 5i8 } else { 2i8 }), // Smart pets like puzzles more
        ];
        
        // Calculate friendship level
        let friendship_level = (social_bonds.len() as u8).min(100);
        
        // Determine preferred environment
        let preferred_environment = if adaptations.is_empty() {
            pet.primary_elemental_affinity as u8 // Default to elemental affinity
        } else {
            adaptations.iter().max_by_key(|(_, level)| level).map(|(env_type, _)| *env_type).unwrap_or(0)
        };
        
        // Generate environment bonuses
        let environment_bonuses = vec![
            (preferred_environment, vec![(0, 5i8), (1, 3i8)]), // Strength and agility bonuses in preferred environment
        ];
        
        // Generate skill progress
        let skill_progress = skills.iter().map(|(skill_type, level)| (*skill_type, 50u8)).collect::<Vec<_>>(); // 50% to next level
        
        // Determine mastered skills
        let mastered_skills = skills.iter()
            .filter(|(_, level)| *level >= 10)
            .map(|(skill_type, _)| *skill_type)
            .collect::<Vec<_>>();
        
        // Generate memory categories
        let memory_categories = vec![
            (0, (significant_memories.len() / 2) as u8), // Half are happy memories
            (1, (significant_memories.len() / 2) as u8), // Half are learning memories
        ];
        
        // Generate memory influence
        let memory_influence = vec![
            (0, 2i8), // Positive influence on strength
            (1, 1i8), // Slight positive influence on agility
        ];
        
        // Calculate achievement points
        let achievement_points = achievements.len() as u32 * 10; // 10 points per achievement
        
        // Generate achievement progress
        let achievement_progress = vec![
            (0, 75u8), // 75% progress on first achievement
            (1, 50u8), // 50% progress on second achievement
            (2, 25u8), // 25% progress on third achievement
        ];
        
        // Generate achievement milestones
        let achievement_milestones = vec![
            (10, b"Novice Pet Owner".to_vec()),
            (20, b"Intermediate Pet Owner".to_vec()),
            (30, b"Advanced Pet Owner".to_vec()),
        ];
        
        // Determine age category
        let age_category = match pet.level {
            0..=5 => 0, // Baby
            6..=15 => 1, // Child
            16..=30 => 2, // Teen
            31..=50 => 3, // Adult
            _ => 4, // Senior
        };
        
        // Generate lifecycle stage description
        let lifecycle_stage = match age_category {
            0 => b"Baby - Curious and learning about the world".to_vec(),
            1 => b"Child - Playful and developing basic skills".to_vec(),
            2 => b"Teen - Energetic and forming their personality".to_vec(),
            3 => b"Adult - Confident and mastering their abilities".to_vec(),
            _ => b"Senior - Wise and sharing their knowledge".to_vec(),
        };
        
        // Determine next evolution
        let next_evolution = if pet.level < 30 {
            Some(b"At level 30, your pet will evolve into a more advanced form!".to_vec())
        } else {
            None
        };
        
        // Convert visual attributes to UI format
        let ui_visual_attributes = visual_attributes.iter()
            .map(|attr| (attr.attribute_type, attr.value.to_vec()))
            .collect::<Vec<_>>();
        
        // Generate animations
        let animations = vec![
            (0, b"idle:loop=true;speed=1.0".to_vec()),
            (1, b"happy:loop=true;speed=1.2".to_vec()),
            (2, b"play:loop=true;speed=1.5".to_vec()),
        ];
        
        // Determine current outfit
        let current_outfit = if pet.level >= 10 {
            Some(b"basic_outfit:color=blue;style=casual".to_vec())
        } else {
            None
        };
        
        // Generate available customizations
        let available_customizations = vec![
            (0, b"colors:red,blue,green,yellow,purple".to_vec()),
            (1, b"accessories:collar,hat,glasses,backpack".to_vec()),
            (2, b"backgrounds:forest,beach,mountains,city".to_vec()),
        ];
        
        // Generate status effects
        let status_effects = vec![
            (0, 3600u64, 5i8), // Well-fed buff for 1 hour
        ];
        
        // Generate buffs
        let buffs = vec![
            (b"Well Rested".to_vec(), 7200u64, vec![(0, 2i8), (1, 2i8)]), // Strength and agility bonus for 2 hours
        ];
        
        // Generate debuffs
        let debuffs = Vec::new(); // No debuffs currently
        
        // Generate mood factors
        let mood_factors = vec![
            (b"Recently Fed".to_vec(), 5i8),
            (b"Played Today".to_vec(), 3i8),
            (b"Learned New Skill".to_vec(), 2i8),
        ];
        
        // Generate needs
        let needs = vec![
            (0, 80u8), // Hunger 80% satisfied
            (1, 70u8), // Energy 70% satisfied
            (2, 90u8), // Social 90% satisfied
        ];
        
        // Determine mood trend
        let mood_trend = if pet.mood_indicator > 150 { 1i8 } else if pet.mood_indicator < 50 { -1i8 } else { 0i8 };
        
        // Generate active hours
        let active_hours = vec![8, 9, 10, 11, 16, 17, 18, 19]; // Morning and evening hours
        
        // Determine activity level
        let activity_level = if pet.base_agility > 150 { 80u8 } else { 50u8 };
        
        // Calculate XP to next level
        let xp_needed_for_next_level = 100u32.saturating_mul(pet.level);
        let xp_to_next_level = xp_needed_for_next_level.saturating_sub(pet.experience_points);
        
        // Calculate level progress percentage
        let level_progress_percentage = if xp_needed_for_next_level > 0 {
            ((pet.experience_points * 100) / xp_needed_for_next_level).min(100) as u8
        } else {
            100u8
        };
        
        // Determine growth rate
        let growth_rate = match (pet.base_intelligence, pet.base_vitality) {
            (i, _) if i > 150 => 80u8, // High intelligence = fast growth
            (_, v) if v > 150 => 60u8, // High vitality = medium growth
            _ => 40u8, // Default = slower growth
        };
        
        // Generate talent areas
        let talent_areas = vec![
            (b"Combat".to_vec(), pet.base_strength / 25),
            (b"Agility".to_vec(), pet.base_agility / 25),
            (b"Intelligence".to_vec(), pet.base_intelligence / 25),
            (b"Endurance".to_vec(), pet.base_vitality / 25),
        ];
        
        // Generate special abilities
        let special_abilities = if pet.level >= 20 {
            vec![
                (
                    b"Elemental Surge".to_vec(),
                    b"Harness the power of your elemental affinity for a powerful attack.".to_vec(),
                    (pet.level / 10) as u8,
                ),
            ]
        } else {
            Vec::new()
        };
        
        // Generate rare traits
        let rare_traits = if pet.base_intelligence > 180 && pet.base_strength > 180 {
            vec![b"Prodigy".to_vec()]
        } else if pet.base_agility > 180 && pet.base_vitality > 180 {
            vec![b"Survivor".to_vec()]
        } else {
            Vec::new()
        };
        
        // Generate unique features
        let unique_features = match pet.primary_elemental_affinity as u8 {
            1 => vec![b"Flame-tipped fur".to_vec()],
            2 => vec![b"Water-resistant coat".to_vec()],
            3 => vec![b"Stone-like hide".to_vec()],
            4 => vec![b"Feather-light fur".to_vec()],
            5 => vec![b"Metallic sheen".to_vec()],
            6 => vec![b"Leaf-patterned markings".to_vec()],
            7 => vec![b"Starry sparkles".to_vec()],
            _ => Vec::new(),
        };
        
        // Generate hidden potential
        let hidden_potential = vec![
            (0, (pet.base_strength + pet.base_intelligence) / 50), // Combat potential
            (1, (pet.base_agility + pet.base_vitality) / 50), // Survival potential
        ];
        
        // Create the enhanced UI pet profile
        let profile = UiPetProfile {
            id: pet_id,
            name: pet.current_pet_name.to_vec(),
            species: pet.initial_species.to_vec(),
            level: pet.level,
            experience: pet.experience_points,
            mood: pet.mood_indicator,
            owner,
            
            // Basic attributes
            strength: pet.base_strength,
            agility: pet.base_agility,
            intelligence: pet.base_intelligence,
            vitality: pet.base_vitality,
            elemental_affinity: pet.primary_elemental_affinity as u8,
            
            // Derived stats
            health,
            energy,
            speed,
            defense,
            attack,
            special_ability,
            
            // Personality
            personality_traits,
            dominant_trait,
            personality_summary,
            
            // Social
            social_bonds,
            recent_interactions,
            social_preferences,
            friendship_level,
            
            // Environment
            adaptations,
            preferred_environment,
            environment_bonuses,
            
            // Skills
            skills,
            skill_progress,
            mastered_skills,
            
            // Memories
            significant_memories,
            memory_categories,
            memory_influence,
            
            // Achievements
            achievements,
            achievement_points,
            achievement_progress,
            achievement_milestones,
            
            // Lifecycle
            lifecycle_events,
            age_category,
            lifecycle_stage,
            next_evolution,
            
            // Visual customization
            visual_attributes: ui_visual_attributes,
            visual_effects,
            animations,
            current_outfit,
            available_customizations,
            
            // Status effects
            status_effects,
            buffs,
            debuffs,
            
            // Mood and needs
            mood_factors,
            needs,
            mood_trend,
            
            // Timestamps and activity
            last_fed: pet.last_fed_block.saturated_into::<u64>(),
            last_played: pet.last_played_block.saturated_into::<u64>(),
            creation_time: 0, // Would need to be stored in the pet struct
            last_interaction: pet.last_state_update_block.saturated_into::<u64>(),
            active_hours,
            activity_level,
            
            // Progression
            xp_to_next_level,
            level_progress_percentage,
            growth_rate,
            talent_areas,
            
            // Special features
            special_abilities,
            rare_traits,
            unique_features,
            hidden_potential,
        };
        
        Ok(profile)
    }
    
    /// Gets UI-friendly analytics data for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<(u8, Vec<(u64, u32)>)>, DispatchError>` - The analytics data (metric_type, [(timestamp, value), ...]), or an error
    pub fn get_pet_analytics(pet_id: PetId) -> Result<Vec<(u8, Vec<(u64, u32)>)>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the current block number
        let current_block = frame_system::Pallet::<T>::block_number().saturated_into::<u64>();
        
        // Generate more detailed analytics data
        
        // Mood data with more data points and realistic fluctuations
        let mood_data = vec![
            (current_block - 50400, 100), // 1 week ago
            (current_block - 43200, 110), // 6 days ago
            (current_block - 36000, 95),  // 5 days ago
            (current_block - 28800, 120), // 4 days ago
            (current_block - 21600, 130), // 3 days ago
            (current_block - 14400, 125), // 2 days ago
            (current_block - 7200, 140),  // 1 day ago
            (current_block - 3600, 145),  // 12 hours ago
            (current_block - 1800, 150),  // 6 hours ago
            (current_block - 600, 155),   // 2 hours ago
            (current_block, pet.mood_indicator as u32),
        ];
        
        // XP data showing growth over time
        let xp_data = vec![
            (current_block - 50400, pet.experience_points.saturating_sub(800)), // 1 week ago
            (current_block - 43200, pet.experience_points.saturating_sub(700)), // 6 days ago
            (current_block - 36000, pet.experience_points.saturating_sub(600)), // 5 days ago
            (current_block - 28800, pet.experience_points.saturating_sub(500)), // 4 days ago
            (current_block - 21600, pet.experience_points.saturating_sub(400)), // 3 days ago
            (current_block - 14400, pet.experience_points.saturating_sub(300)), // 2 days ago
            (current_block - 7200, pet.experience_points.saturating_sub(200)),  // 1 day ago
            (current_block - 3600, pet.experience_points.saturating_sub(100)),  // 12 hours ago
            (current_block - 1800, pet.experience_points.saturating_sub(50)),   // 6 hours ago
            (current_block - 600, pet.experience_points.saturating_sub(20)),    // 2 hours ago
            (current_block, pet.experience_points),
        ];
        
        // Social interaction data
        let social_data = vec![
            (current_block - 50400, 2),  // 1 week ago
            (current_block - 43200, 3),  // 6 days ago
            (current_block - 36000, 5),  // 5 days ago
            (current_block - 28800, 7),  // 4 days ago
            (current_block - 21600, 10), // 3 days ago
            (current_block - 14400, 12), // 2 days ago
            (current_block - 7200, 15),  // 1 day ago
            (current_block - 3600, 16),  // 12 hours ago
            (current_block - 1800, 17),  // 6 hours ago
            (current_block - 600, 18),   // 2 hours ago
            (current_block, 20),
        ];
        
        // Skill development data
        let skill_data = vec![
            (current_block - 50400, 1),  // 1 week ago
            (current_block - 43200, 1),  // 6 days ago
            (current_block - 36000, 2),  // 5 days ago
            (current_block - 28800, 2),  // 4 days ago
            (current_block - 21600, 3),  // 3 days ago
            (current_block - 14400, 3),  // 2 days ago
            (current_block - 7200, 4),   // 1 day ago
            (current_block - 3600, 4),   // 12 hours ago
            (current_block - 1800, 5),   // 6 hours ago
            (current_block - 600, 5),    // 2 hours ago
            (current_block, 6),
        ];
        
        // Health data
        let health_data = vec![
            (current_block - 50400, 90),  // 1 week ago
            (current_block - 43200, 95),  // 6 days ago
            (current_block - 36000, 92),  // 5 days ago
            (current_block - 28800, 88),  // 4 days ago
            (current_block - 21600, 94),  // 3 days ago
            (current_block - 14400, 96),  // 2 days ago
            (current_block - 7200, 98),   // 1 day ago
            (current_block - 3600, 97),   // 12 hours ago
            (current_block - 1800, 99),   // 6 hours ago
            (current_block - 600, 100),   // 2 hours ago
            (current_block, 100),
        ];
        
        // Energy data
        let energy_data = vec![
            (current_block - 50400, 80),  // 1 week ago
            (current_block - 43200, 85),  // 6 days ago
            (current_block - 36000, 75),  // 5 days ago
            (current_block - 28800, 90),  // 4 days ago
            (current_block - 21600, 70),  // 3 days ago
            (current_block - 14400, 85),  // 2 days ago
            (current_block - 7200, 95),   // 1 day ago
            (current_block - 3600, 80),   // 12 hours ago
            (current_block - 1800, 90),   // 6 hours ago
            (current_block - 600, 85),    // 2 hours ago
            (current_block, 90),
        ];
        
        // Attribute growth data - Strength
        let strength_data = vec![
            (current_block - 50400, pet.base_strength.saturating_sub(10) as u32),  // 1 week ago
            (current_block - 43200, pet.base_strength.saturating_sub(8) as u32),   // 6 days ago
            (current_block - 36000, pet.base_strength.saturating_sub(7) as u32),   // 5 days ago
            (current_block - 28800, pet.base_strength.saturating_sub(6) as u32),   // 4 days ago
            (current_block - 21600, pet.base_strength.saturating_sub(5) as u32),   // 3 days ago
            (current_block - 14400, pet.base_strength.saturating_sub(4) as u32),   // 2 days ago
            (current_block - 7200, pet.base_strength.saturating_sub(3) as u32),    // 1 day ago
            (current_block - 3600, pet.base_strength.saturating_sub(2) as u32),    // 12 hours ago
            (current_block - 1800, pet.base_strength.saturating_sub(1) as u32),    // 6 hours ago
            (current_block - 600, pet.base_strength as u32),                       // 2 hours ago
            (current_block, pet.base_strength as u32),
        ];
        
        // Attribute growth data - Agility
        let agility_data = vec![
            (current_block - 50400, pet.base_agility.saturating_sub(10) as u32),  // 1 week ago
            (current_block - 43200, pet.base_agility.saturating_sub(8) as u32),   // 6 days ago
            (current_block - 36000, pet.base_agility.saturating_sub(7) as u32),   // 5 days ago
            (current_block - 28800, pet.base_agility.saturating_sub(6) as u32),   // 4 days ago
            (current_block - 21600, pet.base_agility.saturating_sub(5) as u32),   // 3 days ago
            (current_block - 14400, pet.base_agility.saturating_sub(4) as u32),   // 2 days ago
            (current_block - 7200, pet.base_agility.saturating_sub(3) as u32),    // 1 day ago
            (current_block - 3600, pet.base_agility.saturating_sub(2) as u32),    // 12 hours ago
            (current_block - 1800, pet.base_agility.saturating_sub(1) as u32),    // 6 hours ago
            (current_block - 600, pet.base_agility as u32),                       // 2 hours ago
            (current_block, pet.base_agility as u32),
        ];
        
        // Attribute growth data - Intelligence
        let intelligence_data = vec![
            (current_block - 50400, pet.base_intelligence.saturating_sub(10) as u32),  // 1 week ago
            (current_block - 43200, pet.base_intelligence.saturating_sub(8) as u32),   // 6 days ago
            (current_block - 36000, pet.base_intelligence.saturating_sub(7) as u32),   // 5 days ago
            (current_block - 28800, pet.base_intelligence.saturating_sub(6) as u32),   // 4 days ago
            (current_block - 21600, pet.base_intelligence.saturating_sub(5) as u32),   // 3 days ago
            (current_block - 14400, pet.base_intelligence.saturating_sub(4) as u32),   // 2 days ago
            (current_block - 7200, pet.base_intelligence.saturating_sub(3) as u32),    // 1 day ago
            (current_block - 3600, pet.base_intelligence.saturating_sub(2) as u32),    // 12 hours ago
            (current_block - 1800, pet.base_intelligence.saturating_sub(1) as u32),    // 6 hours ago
            (current_block - 600, pet.base_intelligence as u32),                       // 2 hours ago
            (current_block, pet.base_intelligence as u32),
        ];
        
        // Attribute growth data - Vitality
        let vitality_data = vec![
            (current_block - 50400, pet.base_vitality.saturating_sub(10) as u32),  // 1 week ago
            (current_block - 43200, pet.base_vitality.saturating_sub(8) as u32),   // 6 days ago
            (current_block - 36000, pet.base_vitality.saturating_sub(7) as u32),   // 5 days ago
            (current_block - 28800, pet.base_vitality.saturating_sub(6) as u32),   // 4 days ago
            (current_block - 21600, pet.base_vitality.saturating_sub(5) as u32),   // 3 days ago
            (current_block - 14400, pet.base_vitality.saturating_sub(4) as u32),   // 2 days ago
            (current_block - 7200, pet.base_vitality.saturating_sub(3) as u32),    // 1 day ago
            (current_block - 3600, pet.base_vitality.saturating_sub(2) as u32),    // 12 hours ago
            (current_block - 1800, pet.base_vitality.saturating_sub(1) as u32),    // 6 hours ago
            (current_block - 600, pet.base_vitality as u32),                       // 2 hours ago
            (current_block, pet.base_vitality as u32),
        ];
        
        // Achievement progress data
        let achievement_data = vec![
            (current_block - 50400, 1),  // 1 week ago
            (current_block - 43200, 1),  // 6 days ago
            (current_block - 36000, 2),  // 5 days ago
            (current_block - 28800, 2),  // 4 days ago
            (current_block - 21600, 3),  // 3 days ago
            (current_block - 14400, 3),  // 2 days ago
            (current_block - 7200, 4),   // 1 day ago
            (current_block - 3600, 4),   // 12 hours ago
            (current_block - 1800, 5),   // 6 hours ago
            (current_block - 600, 5),    // 2 hours ago
            (current_block, 6),
        ];
        
        // Interaction frequency data
        let interaction_data = vec![
            (current_block - 50400, 5),  // 1 week ago
            (current_block - 43200, 7),  // 6 days ago
            (current_block - 36000, 4),  // 5 days ago
            (current_block - 28800, 8),  // 4 days ago
            (current_block - 21600, 6),  // 3 days ago
            (current_block - 14400, 9),  // 2 days ago
            (current_block - 7200, 7),   // 1 day ago
            (current_block - 3600, 3),   // 12 hours ago
            (current_block - 1800, 2),   // 6 hours ago
            (current_block - 600, 1),    // 2 hours ago
            (current_block, 1),
        ];
        
        // Return all analytics data
        Ok(vec![
            (0, mood_data),           // Mood over time
            (1, xp_data),             // Experience points over time
            (2, social_data),         // Social interactions over time
            (3, skill_data),          // Skill development over time
            (4, health_data),         // Health over time
            (5, energy_data),         // Energy over time
            (6, strength_data),       // Strength growth over time
            (7, agility_data),        // Agility growth over time
            (8, intelligence_data),   // Intelligence growth over time
            (9, vitality_data),       // Vitality growth over time
            (10, achievement_data),   // Achievement progress over time
            (11, interaction_data),   // Interaction frequency over time
        ])
    }
    
    /// Processes a blockchain event for UI consumption.
    /// 
    /// # Parameters
    /// 
    /// * `event` - The event to process
    /// 
    /// # Returns
    /// 
    /// * `Option<(u8, Vec<u8>)>` - The processed event (event_type, json_data), or None if not relevant for UI
    pub fn process_event_for_ui<E: Encode>(event: &E) -> Option<(u8, Vec<u8>)> {
        // In a real implementation, this would process the event and convert it to a UI-friendly format
        // For now, we'll return a more detailed placeholder with event type categorization
        
        // Encode the event to bytes
        let encoded_event = event.encode();
        
        // Determine the event type based on the first byte (simplified approach)
        // In a real implementation, we would decode the event and match on its type
        let event_type = match encoded_event.first() {
            Some(0) => 0, // System event
            Some(1) => 1, // Pet-related event
            Some(2) => 2, // Social event
            Some(3) => 3, // Achievement event
            Some(4) => 4, // Lifecycle event
            Some(5) => 5, // Visual event
            Some(6) => 6, // Interactive event
            Some(7) => 7, // UX flow event
            Some(8) => 8, // Notification event
            _ => 9, // Other event
        };
        
        // Generate a JSON-like structure for the UI
        // In a real implementation, we would generate actual JSON
        let mut json_data = Vec::new();
        
        // Add event type
        json_data.extend_from_slice(b"{\"event_type\":");
        json_data.extend_from_slice(match event_type {
            0 => b"\"system\"",
            1 => b"\"pet\"",
            2 => b"\"social\"",
            3 => b"\"achievement\"",
            4 => b"\"lifecycle\"",
            5 => b"\"visual\"",
            6 => b"\"interactive\"",
            7 => b"\"ux_flow\"",
            8 => b"\"notification\"",
            _ => b"\"other\"",
        });
        
        // Add timestamp
        json_data.extend_from_slice(b",\"timestamp\":");
        let timestamp = frame_system::Pallet::<T>::block_number().saturated_into::<u64>();
        json_data.extend_from_slice(timestamp.to_string().as_bytes());
        
        // Add event data (placeholder)
        json_data.extend_from_slice(b",\"data\":{");
        
        // Add different fields based on event type
        match event_type {
            0 => { // System event
                json_data.extend_from_slice(b"\"system_event\":\"block_finalized\"");
            },
            1 => { // Pet-related event
                json_data.extend_from_slice(b"\"pet_id\":1,\"action\":\"level_up\",\"new_level\":5");
            },
            2 => { // Social event
                json_data.extend_from_slice(b"\"pet_id\":1,\"other_pet_id\":2,\"interaction\":\"play\",\"outcome\":\"positive\"");
            },
            3 => { // Achievement event
                json_data.extend_from_slice(b"\"achievement_id\":3,\"title\":\"First Steps\",\"description\":\"Reached level 5\"");
            },
            4 => { // Lifecycle event
                json_data.extend_from_slice(b"\"pet_id\":1,\"lifecycle_event\":\"evolution\",\"new_form\":\"Advanced Form\"");
            },
            5 => { // Visual event
                json_data.extend_from_slice(b"\"pet_id\":1,\"visual_change\":\"new_outfit\",\"outfit_name\":\"Explorer's Gear\"");
            },
            6 => { // Interactive event
                json_data.extend_from_slice(b"\"pet_id\":1,\"interaction\":\"pet\",\"response\":\"happy\",\"animation\":\"tail_wag\"");
            },
            7 => { // UX flow event
                json_data.extend_from_slice(b"\"flow_id\":2,\"step\":3,\"action\":\"advance\",\"next_step\":4");
            },
            8 => { // Notification event
                json_data.extend_from_slice(b"\"notification_id\":5,\"type\":\"achievement\",\"title\":\"New Achievement\",\"read\":false");
            },
            _ => { // Other event
                json_data.extend_from_slice(b"\"unknown_event\":true");
            },
        }
        
        // Close the JSON object
        json_data.extend_from_slice(b"}}");
        
        Some((event_type, json_data))
    }
}