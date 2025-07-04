//! # Visual Representation System
//!
//! This module provides a system for managing the visual representation of pets,
//! including customization options, animations, and visual effects that reflect
//! the pet's state, personality, and environment.

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

/// Represents a visual attribute for a pet.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct VisualAttribute {
    /// The attribute type
    pub attribute_type: u8,
    
    /// The attribute value
    pub value: BoundedVec<u8, ConstU32<64>>,
}

/// Represents a visual theme for a pet.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct VisualTheme {
    /// The theme ID
    pub theme_id: u8,
    
    /// The theme name
    pub name: BoundedVec<u8, ConstU32<32>>,
    
    /// The theme description
    pub description: BoundedVec<u8, ConstU32<128>>,
    
    /// The base color palette
    pub color_palette: BoundedVec<u8, ConstU32<64>>,
    
    /// The animation style
    pub animation_style: u8,
    
    /// The visual effects
    pub visual_effects: BoundedVec<u8, ConstU32<16>>,
}

/// Represents an animation for a pet.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Animation {
    /// The animation ID
    pub animation_id: u8,
    
    /// The animation name
    pub name: BoundedVec<u8, ConstU32<32>>,
    
    /// The animation description
    pub description: BoundedVec<u8, ConstU32<128>>,
    
    /// The animation parameters
    pub parameters: BoundedVec<u8, ConstU32<256>>,
    
    /// The animation triggers
    pub triggers: BoundedVec<u8, ConstU32<16>>,
}

/// Represents a single frame in an animation sequence.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AnimationFrame {
    /// The frame ID
    pub frame_id: u8,
    
    /// The pose for this frame
    pub pose: BoundedVec<u8, ConstU32<32>>,
    
    /// The facial expression for this frame
    pub expression: BoundedVec<u8, ConstU32<32>>,
    
    /// The position (x, y, z) relative to the default position
    pub position: (i16, i16, i16),
    
    /// The scale (x, y, z) as percentage of default size (100 = 100%)
    pub scale: (u16, u16, u16),
    
    /// The rotation (x, y, z) in degrees
    pub rotation: (i16, i16, i16),
    
    /// The color shift (r, g, b, a) to apply
    pub color_shift: (u8, u8, u8, u8),
    
    /// The visual effects to apply during this frame
    pub effects: BoundedVec<u8, ConstU32<16>>,
    
    /// The duration of this frame in milliseconds
    pub timing: u32,
}

/// Represents a transition between animation frames.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AnimationTransition {
    /// The source frame ID
    pub from_frame_id: u8,
    
    /// The destination frame ID
    pub to_frame_id: u8,
    
    /// The transition type (0 = smooth, 1 = quick, 2 = special)
    pub transition_type: u8,
    
    /// The easing function (0 = linear, 1 = ease-in, 2 = ease-out, 3 = ease-in-out)
    pub easing_function: u8,
    
    /// The duration of the transition in milliseconds
    pub duration: u32,
}

/// Represents a particle effect in an animation.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ParticleEffect {
    /// The effect ID
    pub effect_id: u8,
    
    /// The effect type (0 = emitter, 1 = burst, 2 = trail)
    pub effect_type: u8,
    
    /// The effect parameters
    pub parameters: BoundedVec<u8, ConstU32<256>>,
    
    /// The position (x, y, z) relative to the pet
    pub position: (i16, i16, i16),
    
    /// The rotation (x, y, z) in degrees
    pub rotation: (i16, i16, i16),
    
    /// The scale (x, y, z) as percentage of default size (100 = 100%)
    pub scale: (u16, u16, u16),
    
    /// The timing (start_ms, end_ms) when this effect is active
    pub timing: (u32, u32),
}

/// Represents a sound effect in an animation.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SoundEffect {
    /// The effect ID
    pub effect_id: u8,
    
    /// The sound type (0 = background, 1 = one-shot, 2 = looping)
    pub sound_type: u8,
    
    /// The sound parameters
    pub parameters: BoundedVec<u8, ConstU32<256>>,
    
    /// The timing (start_ms, end_ms) when this sound is active
    pub timing: (u32, u32),
}

/// Represents a camera effect in an animation.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CameraEffect {
    /// The effect ID
    pub effect_id: u8,
    
    /// The camera effect type (0 = movement, 1 = zoom, 2 = filter)
    pub camera_type: u8,
    
    /// The camera parameters
    pub parameters: BoundedVec<u8, ConstU32<256>>,
    
    /// The timing (start_ms, end_ms) when this camera effect is active
    pub timing: (u32, u32),
}

/// Represents a complete animation sequence.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AnimationSequence {
    /// The sequence ID
    pub id: u8,
    
    /// The sequence name
    pub name: BoundedVec<u8, ConstU32<32>>,
    
    /// The sequence description
    pub description: BoundedVec<u8, ConstU32<128>>,
    
    /// The animation frames
    pub frames: BoundedVec<AnimationFrame, ConstU32<32>>,
    
    /// The transitions between frames
    pub transitions: BoundedVec<AnimationTransition, ConstU32<32>>,
    
    /// The total duration of the sequence in milliseconds
    pub duration: u32,
    
    /// The loop behavior (0 = no loop, 1 = loop)
    pub loop_behavior: u8,
    
    /// The particle effects
    pub particle_effects: BoundedVec<ParticleEffect, ConstU32<16>>,
    
    /// The sound effects
    pub sound_effects: BoundedVec<SoundEffect, ConstU32<16>>,
    
    /// The camera effects
    pub camera_effects: BoundedVec<CameraEffect, ConstU32<16>>,
}

/// Visual attribute types.
pub enum AttributeType {
    BaseColor = 0,
    SecondaryColor = 1,
    AccentColor = 2,
    EyeColor = 3,
    Pattern = 4,
    Accessory = 5,
    Background = 6,
    SpecialEffect = 7,
    Aura = 8,
    Particles = 9,
    Outfit = 10,
    Hat = 11,
    Glasses = 12,
    Wings = 13,
    Tail = 14,
    Markings = 15,
    HoloProjection = 16,
    WeatherEffect = 17,
    TimeEffect = 18,
    MoodAura = 19,
}

/// Animation styles.
pub enum AnimationStyle {
    Default = 0,
    Playful = 1,
    Elegant = 2,
    Energetic = 3,
    Relaxed = 4,
    Mysterious = 5,
    Robotic = 6,
    Fluid = 7,
    Bouncy = 8,
    Glitchy = 9,
    Ethereal = 10,
    Cosmic = 11,
    Pixelated = 12,
    Holographic = 13,
    Neon = 14,
    Retro = 15,
    Futuristic = 16,
    Magical = 17,
    Elemental = 18,
    Seasonal = 19,
}

/// Animation triggers.
pub enum AnimationTrigger {
    Idle = 0,
    Happy = 1,
    Sad = 2,
    Excited = 3,
    Tired = 4,
    Hungry = 5,
    Playing = 6,
    Sleeping = 7,
    Interacting = 8,
    EnvironmentChange = 9,
    LevelUp = 10,
    AchievementUnlocked = 11,
    NewSkill = 12,
    Surprised = 13,
    Scared = 14,
    Angry = 15,
    Loving = 16,
    Curious = 17,
    Mischievous = 18,
    Celebrating = 19,
    Dancing = 20,
    Meditating = 21,
    Transforming = 22,
    Evolving = 23,
    SeasonalEvent = 24,
    SpecialAbility = 25,
    TimeOfDay = 26,
    Weather = 27,
    MoodSwing = 28,
    ElementalReaction = 29,
}

/// A system for managing pet visual representations.
pub struct VisualSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

/// Error codes for the visual system.
pub enum VisualSystemError {
    /// Too many frames in an animation sequence
    TooManyFrames,
    /// Too many transitions in an animation sequence
    TooManyTransitions,
    /// Too many effects in an animation sequence
    TooManyEffects,
    /// Invalid animation context
    InvalidAnimationContext,
}

impl<T: Config> VisualSystem<T> {
    /// Gets the visual attributes for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<VisualAttribute>, DispatchError>` - The visual attributes, or an error
    pub fn get_visual_attributes(pet_id: PetId) -> Result<Vec<VisualAttribute>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the visual attributes from storage
        let attributes = crate::PetVisualAttributes::<T>::get(pet_id);
        
        // If no attributes are set, generate default ones based on the pet's DNA
        if attributes.is_empty() {
            let default_attributes = Self::generate_default_attributes(&pet)?;
            Ok(default_attributes)
        } else {
            Ok(attributes.to_vec())
        }
    }
    
    /// Sets a visual attribute for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `attribute_type` - The attribute type
    /// * `value` - The attribute value
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn set_visual_attribute(
        pet_id: PetId,
        attribute_type: u8,
        value: Vec<u8>,
    ) -> DispatchResult {
        // Ensure the pet exists
        ensure!(crate::PetNfts::<T>::contains_key(pet_id), Error::<T>::PetNotFound);
        
        // Ensure the attribute type is valid
        ensure!(attribute_type <= AttributeType::SpecialEffect as u8, Error::<T>::InvalidAttributeType);
        
        // Ensure the value is valid
        let bounded_value: BoundedVec<u8, ConstU32<64>> = value.try_into()
            .map_err(|_| Error::<T>::ValueTooLong)?;
        
        // Update the attribute
        crate::PetVisualAttributes::<T>::try_mutate(pet_id, |attributes| -> DispatchResult {
            // Find the attribute if it exists
            let attribute_index = attributes.iter().position(|attr| attr.attribute_type == attribute_type);
            
            if let Some(index) = attribute_index {
                // Update the existing attribute
                attributes[index].value = bounded_value;
            } else {
                // Add a new attribute
                attributes.try_push(VisualAttribute {
                    attribute_type,
                    value: bounded_value,
                }).map_err(|_| Error::<T>::TooManyAttributes)?;
            }
            
            Ok(())
        })
    }
    
    /// Gets the current animation for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `trigger` - The animation trigger
    /// 
    /// # Returns
    /// 
    /// * `Result<Animation, DispatchError>` - The animation, or an error
    pub fn get_animation(
        pet_id: PetId,
        trigger: u8,
    ) -> Result<Animation, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the pet's animation style
        let animation_style = Self::get_animation_style(&pet)?;
        
        // Get the animation based on the style and trigger
        Self::get_animation_by_style_and_trigger(animation_style, trigger)
    }
    
    /// Gets the animation style for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// 
    /// # Returns
    /// 
    /// * `Result<u8, DispatchError>` - The animation style, or an error
    fn get_animation_style(pet: &PetNft<T>) -> Result<u8, DispatchError> {
        // In a real implementation, this would determine the animation style based on the pet's traits
        // For now, we'll use a simple algorithm based on elemental affinity
        
        match pet.primary_elemental_affinity as u8 {
            0 => Ok(AnimationStyle::Default as u8),
            1 => Ok(AnimationStyle::Energetic as u8),
            2 => Ok(AnimationStyle::Fluid as u8),
            3 => Ok(AnimationStyle::Relaxed as u8),
            4 => Ok(AnimationStyle::Playful as u8),
            5 => Ok(AnimationStyle::Robotic as u8),
            6 => Ok(AnimationStyle::Elegant as u8),
            7 => Ok(AnimationStyle::Mysterious as u8),
            _ => Ok(AnimationStyle::Default as u8),
        }
    }
    
    /// Gets an animation by style and trigger.
    /// 
    /// # Parameters
    /// 
    /// * `style` - The animation style
    /// * `trigger` - The animation trigger
    /// 
    /// # Returns
    /// 
    /// * `Result<Animation, DispatchError>` - The animation, or an error
    
    /// Gets advanced animation sequences for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `context` - The animation context (e.g., "battle", "home", "social")
    /// 
    /// # Returns
    /// 
    /// * `Result<AnimationSequence, DispatchError>` - The animation sequence, or an error
    pub fn get_advanced_animation_sequence(
        pet_id: PetId,
        context: Vec<u8>,
    ) -> Result<AnimationSequence, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Create a new animation sequence
        let mut sequence = AnimationSequence {
            id: 0, // Will be set based on context
            name: BoundedVec::default(),
            description: BoundedVec::default(),
            frames: BoundedVec::default(),
            transitions: BoundedVec::default(),
            duration: 0,
            loop_behavior: 0,
            particle_effects: BoundedVec::default(),
            sound_effects: BoundedVec::default(),
            camera_effects: BoundedVec::default(),
        };
        
        // Set sequence ID based on context
        if context == b"battle".to_vec() {
            sequence.id = 1;
        } else if context == b"home".to_vec() {
            sequence.id = 2;
        } else if context == b"social".to_vec() {
            sequence.id = 3;
        } else if context == b"adventure".to_vec() {
            sequence.id = 4;
        } else if context == b"evolution".to_vec() {
            sequence.id = 5;
        } else {
            sequence.id = 0; // Default
        }
        
        // Set sequence name
        let name = match sequence.id {
            1 => b"Battle Animation".to_vec(),
            2 => b"Home Animation".to_vec(),
            3 => b"Social Animation".to_vec(),
            4 => b"Adventure Animation".to_vec(),
            5 => b"Evolution Animation".to_vec(),
            _ => b"Default Animation".to_vec(),
        };
        sequence.name = name.try_into().map_err(|_| Error::<T>::ValueTooLong)?;
        
        // Set sequence description
        let description = match sequence.id {
            1 => b"An intense animation sequence for battles, showing the pet's combat prowess and elemental abilities.".to_vec(),
            2 => b"A relaxed animation sequence for the home environment, showing the pet's personality and daily activities.".to_vec(),
            3 => b"A friendly animation sequence for social interactions, showing the pet's social behaviors and reactions.".to_vec(),
            4 => b"An exciting animation sequence for adventures, showing the pet exploring and discovering new things.".to_vec(),
            5 => b"A spectacular animation sequence for evolution, showing the pet transforming into a more advanced form.".to_vec(),
            _ => b"A standard animation sequence for general use.".to_vec(),
        };
        sequence.description = description.try_into().map_err(|_| Error::<T>::ValueTooLong)?;
        
        // Set sequence duration based on context and pet level
        sequence.duration = match sequence.id {
            1 => 5000 + (pet.level * 100) as u32, // Battle animations get longer with level
            2 => 3000, // Home animations are consistent
            3 => 4000, // Social animations are consistent
            4 => 6000, // Adventure animations are longer
            5 => 10000, // Evolution animations are the longest
            _ => 2000, // Default is short
        };
        
        // Set loop behavior
        sequence.loop_behavior = match sequence.id {
            1 => 0, // Battle animations don't loop
            2 => 1, // Home animations loop
            3 => 1, // Social animations loop
            4 => 0, // Adventure animations don't loop
            5 => 0, // Evolution animations don't loop
            _ => 1, // Default is to loop
        };
        
        // Generate animation frames based on context and pet traits
        let mut frames = Vec::new();
        
        match sequence.id {
            1 => { // Battle animation
                // Intro pose
                frames.push(AnimationFrame {
                    frame_id: 0,
                    pose: b"battle_ready".to_vec().try_into().unwrap_or_default(),
                    expression: b"determined".to_vec().try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![10, 20, 30].try_into().unwrap_or_default(),
                    timing: 500,
                });
                
                // Charge up
                frames.push(AnimationFrame {
                    frame_id: 1,
                    pose: b"charging".to_vec().try_into().unwrap_or_default(),
                    expression: b"focused".to_vec().try_into().unwrap_or_default(),
                    position: (0, 10, 0),
                    scale: (110, 110, 110),
                    rotation: (0, 0, 10),
                    color_shift: (10, 0, 0, 0),
                    effects: vec![11, 21, 31].try_into().unwrap_or_default(),
                    timing: 1000,
                });
                
                // Attack
                frames.push(AnimationFrame {
                    frame_id: 2,
                    pose: b"attacking".to_vec().try_into().unwrap_or_default(),
                    expression: b"fierce".to_vec().try_into().unwrap_or_default(),
                    position: (50, 0, 20),
                    scale: (120, 120, 120),
                    rotation: (0, 0, 20),
                    color_shift: (20, 0, 0, 0),
                    effects: vec![12, 22, 32].try_into().unwrap_or_default(),
                    timing: 300,
                });
                
                // Impact
                frames.push(AnimationFrame {
                    frame_id: 3,
                    pose: b"impact".to_vec().try_into().unwrap_or_default(),
                    expression: b"intense".to_vec().try_into().unwrap_or_default(),
                    position: (70, 0, 0),
                    scale: (130, 130, 130),
                    rotation: (0, 0, 0),
                    color_shift: (30, 0, 0, 0),
                    effects: vec![13, 23, 33].try_into().unwrap_or_default(),
                    timing: 200,
                });
                
                // Recovery
                frames.push(AnimationFrame {
                    frame_id: 4,
                    pose: b"recovery".to_vec().try_into().unwrap_or_default(),
                    expression: b"satisfied".to_vec().try_into().unwrap_or_default(),
                    position: (20, 0, 0),
                    scale: (110, 110, 110),
                    rotation: (0, 0, -10),
                    color_shift: (10, 0, 0, 0),
                    effects: vec![14, 24, 34].try_into().unwrap_or_default(),
                    timing: 1000,
                });
                
                // Return to ready
                frames.push(AnimationFrame {
                    frame_id: 5,
                    pose: b"battle_ready".to_vec().try_into().unwrap_or_default(),
                    expression: b"alert".to_vec().try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![10, 20, 30].try_into().unwrap_or_default(),
                    timing: 500,
                });
            },
            2 => { // Home animation
                // Idle
                frames.push(AnimationFrame {
                    frame_id: 0,
                    pose: b"idle".to_vec().try_into().unwrap_or_default(),
                    expression: b"content".to_vec().try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![40, 50].try_into().unwrap_or_default(),
                    timing: 1000,
                });
                
                // Look around
                frames.push(AnimationFrame {
                    frame_id: 1,
                    pose: b"looking".to_vec().try_into().unwrap_or_default(),
                    expression: b"curious".to_vec().try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 20, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![41, 51].try_into().unwrap_or_default(),
                    timing: 800,
                });
                
                // Stretch
                frames.push(AnimationFrame {
                    frame_id: 2,
                    pose: b"stretching".to_vec().try_into().unwrap_or_default(),
                    expression: b"relaxed".to_vec().try_into().unwrap_or_default(),
                    position: (0, 10, 0),
                    scale: (110, 110, 110),
                    rotation: (0, 0, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![42, 52].try_into().unwrap_or_default(),
                    timing: 1200,
                });
            },
            3 => { // Social animation
                // Greeting
                frames.push(AnimationFrame {
                    frame_id: 0,
                    pose: b"greeting".to_vec().try_into().unwrap_or_default(),
                    expression: b"friendly".to_vec().try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![60, 70].try_into().unwrap_or_default(),
                    timing: 800,
                });
                
                // Approach
                frames.push(AnimationFrame {
                    frame_id: 1,
                    pose: b"approaching".to_vec().try_into().unwrap_or_default(),
                    expression: b"interested".to_vec().try_into().unwrap_or_default(),
                    position: (30, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![61, 71].try_into().unwrap_or_default(),
                    timing: 1000,
                });
                
                // Interact
                frames.push(AnimationFrame {
                    frame_id: 2,
                    pose: b"interacting".to_vec().try_into().unwrap_or_default(),
                    expression: b"happy".to_vec().try_into().unwrap_or_default(),
                    position: (50, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 10),
                    color_shift: (0, 0, 10, 0),
                    effects: vec![62, 72].try_into().unwrap_or_default(),
                    timing: 1500,
                });
                
                // Play
                frames.push(AnimationFrame {
                    frame_id: 3,
                    pose: b"playing".to_vec().try_into().unwrap_or_default(),
                    expression: b"playful".to_vec().try_into().unwrap_or_default(),
                    position: (20, 10, 0),
                    scale: (110, 110, 110),
                    rotation: (0, 0, -10),
                    color_shift: (0, 0, 20, 0),
                    effects: vec![63, 73].try_into().unwrap_or_default(),
                    timing: 1200,
                });
            },
            4 => { // Adventure animation
                // Exploring
                frames.push(AnimationFrame {
                    frame_id: 0,
                    pose: b"exploring".to_vec().try_into().unwrap_or_default(),
                    expression: b"curious".to_vec().try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![80, 90].try_into().unwrap_or_default(),
                    timing: 1500,
                });
                
                // Discovery
                frames.push(AnimationFrame {
                    frame_id: 1,
                    pose: b"discovery".to_vec().try_into().unwrap_or_default(),
                    expression: b"surprised".to_vec().try_into().unwrap_or_default(),
                    position: (20, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 30, 0),
                    color_shift: (0, 10, 0, 0),
                    effects: vec![81, 91].try_into().unwrap_or_default(),
                    timing: 1000,
                });
                
                // Interaction
                frames.push(AnimationFrame {
                    frame_id: 2,
                    pose: b"interacting".to_vec().try_into().unwrap_or_default(),
                    expression: b"focused".to_vec().try_into().unwrap_or_default(),
                    position: (40, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 0),
                    color_shift: (0, 20, 0, 0),
                    effects: vec![82, 92].try_into().unwrap_or_default(),
                    timing: 2000,
                });
                
                // Reward
                frames.push(AnimationFrame {
                    frame_id: 3,
                    pose: b"celebrating".to_vec().try_into().unwrap_or_default(),
                    expression: b"happy".to_vec().try_into().unwrap_or_default(),
                    position: (40, 10, 0),
                    scale: (110, 110, 110),
                    rotation: (0, 0, 10),
                    color_shift: (0, 30, 0, 0),
                    effects: vec![83, 93].try_into().unwrap_or_default(),
                    timing: 1500,
                });
            },
            5 => { // Evolution animation
                // Pre-evolution
                frames.push(AnimationFrame {
                    frame_id: 0,
                    pose: b"standing".to_vec().try_into().unwrap_or_default(),
                    expression: b"determined".to_vec().try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![100, 110].try_into().unwrap_or_default(),
                    timing: 1000,
                });
                
                // Energy gathering
                frames.push(AnimationFrame {
                    frame_id: 1,
                    pose: b"powering_up".to_vec().try_into().unwrap_or_default(),
                    expression: b"focused".to_vec().try_into().unwrap_or_default(),
                    position: (0, 10, 0),
                    scale: (110, 110, 110),
                    rotation: (0, 0, 0),
                    color_shift: (10, 10, 10, 0),
                    effects: vec![101, 111].try_into().unwrap_or_default(),
                    timing: 2000,
                });
                
                // Transformation start
                frames.push(AnimationFrame {
                    frame_id: 2,
                    pose: b"transforming".to_vec().try_into().unwrap_or_default(),
                    expression: b"intense".to_vec().try_into().unwrap_or_default(),
                    position: (0, 20, 0),
                    scale: (120, 120, 120),
                    rotation: (0, 0, 0),
                    color_shift: (20, 20, 20, 0),
                    effects: vec![102, 112].try_into().unwrap_or_default(),
                    timing: 2000,
                });
                
                // Transformation peak
                frames.push(AnimationFrame {
                    frame_id: 3,
                    pose: b"transforming_peak".to_vec().try_into().unwrap_or_default(),
                    expression: b"transcendent".to_vec().try_into().unwrap_or_default(),
                    position: (0, 30, 0),
                    scale: (130, 130, 130),
                    rotation: (0, 360, 0),
                    color_shift: (30, 30, 30, 0),
                    effects: vec![103, 113].try_into().unwrap_or_default(),
                    timing: 3000,
                });
                
                // New form reveal
                frames.push(AnimationFrame {
                    frame_id: 4,
                    pose: b"evolved_form".to_vec().try_into().unwrap_or_default(),
                    expression: b"powerful".to_vec().try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    scale: (120, 120, 120),
                    rotation: (0, 0, 0),
                    color_shift: (10, 10, 10, 0),
                    effects: vec![104, 114].try_into().unwrap_or_default(),
                    timing: 2000,
                });
            },
            _ => { // Default animation
                // Simple idle
                frames.push(AnimationFrame {
                    frame_id: 0,
                    pose: b"idle".to_vec().try_into().unwrap_or_default(),
                    expression: b"neutral".to_vec().try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    scale: (100, 100, 100),
                    rotation: (0, 0, 0),
                    color_shift: (0, 0, 0, 0),
                    effects: vec![1, 2].try_into().unwrap_or_default(),
                    timing: 2000,
                });
            },
        }
        
        // Convert frames to BoundedVec
        sequence.frames = frames.try_into().map_err(|_| Error::<T>::TooManyAttributes)?;
        
        // Generate transitions between frames
        let mut transitions = Vec::new();
        
        for i in 0..frames.len().saturating_sub(1) {
            transitions.push(AnimationTransition {
                from_frame_id: i as u8,
                to_frame_id: (i + 1) as u8,
                transition_type: if i % 2 == 0 { 0 } else { 1 }, // Alternate between smooth and quick
                easing_function: if i % 3 == 0 { 0 } else if i % 3 == 1 { 1 } else { 2 }, // Cycle through easing functions
                duration: 200, // Standard transition duration
            });
        }
        
        // Add loop transition if needed
        if sequence.loop_behavior == 1 && !frames.is_empty() {
            transitions.push(AnimationTransition {
                from_frame_id: (frames.len() - 1) as u8,
                to_frame_id: 0,
                transition_type: 2, // Special transition for looping
                easing_function: 3, // Special easing for looping
                duration: 300, // Slightly longer for loop transition
            });
        }
        
        // Convert transitions to BoundedVec
        sequence.transitions = transitions.try_into().map_err(|_| Error::<T>::TooManyAttributes)?;
        
        // Add particle effects based on pet's elemental affinity
        let mut particle_effects = Vec::new();
        
        match pet.primary_elemental_affinity as u8 {
            1 => { // Fire
                particle_effects.push(ParticleEffect {
                    effect_id: 1,
                    effect_type: 0, // Emitter
                    parameters: b"type=fire;color=#FF5500;intensity=high;size=medium;rate=10;lifetime=2000".to_vec()
                        .try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    rotation: (0, 0, 0),
                    scale: (100, 100, 100),
                    timing: (0, sequence.duration),
                });
            },
            2 => { // Water
                particle_effects.push(ParticleEffect {
                    effect_id: 2,
                    effect_type: 0, // Emitter
                    parameters: b"type=water;color=#0088FF;intensity=medium;size=small;rate=15;lifetime=1500".to_vec()
                        .try_into().unwrap_or_default(),
                    position: (0, -10, 0),
                    rotation: (0, 0, 0),
                    scale: (100, 100, 100),
                    timing: (0, sequence.duration),
                });
            },
            3 => { // Earth
                particle_effects.push(ParticleEffect {
                    effect_id: 3,
                    effect_type: 0, // Emitter
                    parameters: b"type=earth;color=#885500;intensity=low;size=large;rate=5;lifetime=3000".to_vec()
                        .try_into().unwrap_or_default(),
                    position: (0, -20, 0),
                    rotation: (0, 0, 0),
                    scale: (100, 100, 100),
                    timing: (0, sequence.duration),
                });
            },
            4 => { // Air
                particle_effects.push(ParticleEffect {
                    effect_id: 4,
                    effect_type: 0, // Emitter
                    parameters: b"type=air;color=#FFFFFF;intensity=medium;size=small;rate=20;lifetime=1000".to_vec()
                        .try_into().unwrap_or_default(),
                    position: (0, 10, 0),
                    rotation: (0, 0, 0),
                    scale: (100, 100, 100),
                    timing: (0, sequence.duration),
                });
            },
            5 => { // Tech
                particle_effects.push(ParticleEffect {
                    effect_id: 5,
                    effect_type: 0, // Emitter
                    parameters: b"type=tech;color=#00FFFF;intensity=high;size=tiny;rate=30;lifetime=800".to_vec()
                        .try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    rotation: (0, 0, 0),
                    scale: (100, 100, 100),
                    timing: (0, sequence.duration),
                });
            },
            6 => { // Nature
                particle_effects.push(ParticleEffect {
                    effect_id: 6,
                    effect_type: 0, // Emitter
                    parameters: b"type=nature;color=#00FF00;intensity=medium;size=small;rate=15;lifetime=2000".to_vec()
                        .try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    rotation: (0, 0, 0),
                    scale: (100, 100, 100),
                    timing: (0, sequence.duration),
                });
            },
            7 => { // Mystic
                particle_effects.push(ParticleEffect {
                    effect_id: 7,
                    effect_type: 0, // Emitter
                    parameters: b"type=mystic;color=#FF00FF;intensity=high;size=medium;rate=10;lifetime=2500".to_vec()
                        .try_into().unwrap_or_default(),
                    position: (0, 10, 0),
                    rotation: (0, 0, 0),
                    scale: (100, 100, 100),
                    timing: (0, sequence.duration),
                });
            },
            _ => { // Neutral
                particle_effects.push(ParticleEffect {
                    effect_id: 0,
                    effect_type: 0, // Emitter
                    parameters: b"type=neutral;color=#CCCCCC;intensity=low;size=small;rate=5;lifetime=1000".to_vec()
                        .try_into().unwrap_or_default(),
                    position: (0, 0, 0),
                    rotation: (0, 0, 0),
                    scale: (100, 100, 100),
                    timing: (0, sequence.duration),
                });
            },
        }
        
        // Add special particle effect for evolution animation
        if sequence.id == 5 {
            particle_effects.push(ParticleEffect {
                effect_id: 10,
                effect_type: 1, // Burst
                parameters: b"type=starburst;color=#FFFFFF;intensity=very_high;size=large;count=50;spread=360".to_vec()
                    .try_into().unwrap_or_default(),
                position: (0, 0, 0),
                rotation: (0, 0, 0),
                scale: (150, 150, 150),
                timing: (3000, 3500), // During transformation peak
            });
        }
        
        // Convert particle effects to BoundedVec
        sequence.particle_effects = particle_effects.try_into().map_err(|_| Error::<T>::TooManyAttributes)?;
        
        // Add sound effects
        let mut sound_effects = Vec::new();
        
        // Add context-specific sound effects
        match sequence.id {
            1 => { // Battle
                sound_effects.push(SoundEffect {
                    effect_id: 1,
                    sound_type: 0, // Background
                    parameters: b"name=battle_theme;volume=0.8;loop=true".to_vec().try_into().unwrap_or_default(),
                    timing: (0, sequence.duration),
                });
                
                sound_effects.push(SoundEffect {
                    effect_id: 2,
                    sound_type: 1, // One-shot
                    parameters: b"name=attack_charge;volume=1.0;pitch=1.0".to_vec().try_into().unwrap_or_default(),
                    timing: (500, 1500),
                });
                
                sound_effects.push(SoundEffect {
                    effect_id: 3,
                    sound_type: 1, // One-shot
                    parameters: b"name=attack_release;volume=1.0;pitch=1.2".to_vec().try_into().unwrap_or_default(),
                    timing: (1500, 1800),
                });
                
                sound_effects.push(SoundEffect {
                    effect_id: 4,
                    sound_type: 1, // One-shot
                    parameters: b"name=impact;volume=1.0;pitch=1.0".to_vec().try_into().unwrap_or_default(),
                    timing: (1800, 2000),
                });
            },
            5 => { // Evolution
                sound_effects.push(SoundEffect {
                    effect_id: 5,
                    sound_type: 0, // Background
                    parameters: b"name=evolution_theme;volume=0.8;loop=true".to_vec().try_into().unwrap_or_default(),
                    timing: (0, sequence.duration),
                });
                
                sound_effects.push(SoundEffect {
                    effect_id: 6,
                    sound_type: 1, // One-shot
                    parameters: b"name=energy_gather;volume=1.0;pitch=1.0".to_vec().try_into().unwrap_or_default(),
                    timing: (1000, 3000),
                });
                
                sound_effects.push(SoundEffect {
                    effect_id: 7,
                    sound_type: 1, // One-shot
                    parameters: b"name=transformation;volume=1.0;pitch=1.0".to_vec().try_into().unwrap_or_default(),
                    timing: (3000, 5000),
                });
                
                sound_effects.push(SoundEffect {
                    effect_id: 8,
                    sound_type: 1, // One-shot
                    parameters: b"name=evolution_complete;volume=1.0;pitch=1.0".to_vec().try_into().unwrap_or_default(),
                    timing: (5000, 7000),
                });
            },
            _ => {
                // Default sound effect
                sound_effects.push(SoundEffect {
                    effect_id: 0,
                    sound_type: 0, // Background
                    parameters: b"name=ambient;volume=0.5;loop=true".to_vec().try_into().unwrap_or_default(),
                    timing: (0, sequence.duration),
                });
            },
        }
        
        // Convert sound effects to BoundedVec
        sequence.sound_effects = sound_effects.try_into().map_err(|_| Error::<T>::TooManyAttributes)?;
        
        // Add camera effects
        let mut camera_effects = Vec::new();
        
        // Add context-specific camera effects
        match sequence.id {
            1 => { // Battle
                camera_effects.push(CameraEffect {
                    effect_id: 1,
                    camera_type: 0, // Movement
                    parameters: b"type=shake;intensity=medium;duration=300".to_vec().try_into().unwrap_or_default(),
                    timing: (1800, 2100),
                });
                
                camera_effects.push(CameraEffect {
                    effect_id: 2,
                    camera_type: 1, // Zoom
                    parameters: b"type=zoom_in;target=pet;factor=1.5;duration=500".to_vec().try_into().unwrap_or_default(),
                    timing: (500, 1000),
                });
            },
            5 => { // Evolution
                camera_effects.push(CameraEffect {
                    effect_id: 3,
                    camera_type: 0, // Movement
                    parameters: b"type=orbit;speed=slow;radius=medium;duration=3000".to_vec().try_into().unwrap_or_default(),
                    timing: (1000, 4000),
                });
                
                camera_effects.push(CameraEffect {
                    effect_id: 4,
                    camera_type: 1, // Zoom
                    parameters: b"type=zoom_out;target=scene;factor=1.8;duration=1000".to_vec().try_into().unwrap_or_default(),
                    timing: (3000, 4000),
                });
                
                camera_effects.push(CameraEffect {
                    effect_id: 5,
                    camera_type: 2, // Filter
                    parameters: b"type=glow;color=#FFFFFF;intensity=high;duration=2000".to_vec().try_into().unwrap_or_default(),
                    timing: (3000, 5000),
                });
            },
            _ => {
                // No special camera effects for other contexts
            },
        }
        
        // Convert camera effects to BoundedVec
        sequence.camera_effects = camera_effects.try_into().map_err(|_| Error::<T>::TooManyAttributes)?;
        
        Ok(sequence)
    }
    fn get_animation_by_style_and_trigger(
        style: u8,
        trigger: u8,
    ) -> Result<Animation, DispatchError> {
        // In a real implementation, this would get the animation from storage
        // For now, we'll return a more detailed placeholder animation
        
        // Get the base animation name based on the trigger
        let name: BoundedVec<u8, ConstU32<32>> = match trigger {
            0 => b"Idle".to_vec(),
            1 => b"Happy".to_vec(),
            2 => b"Sad".to_vec(),
            3 => b"Excited".to_vec(),
            4 => b"Tired".to_vec(),
            5 => b"Hungry".to_vec(),
            6 => b"Playing".to_vec(),
            7 => b"Sleeping".to_vec(),
            8 => b"Interacting".to_vec(),
            9 => b"Environment Change".to_vec(),
            10 => b"Level Up".to_vec(),
            11 => b"Achievement Unlocked".to_vec(),
            12 => b"New Skill".to_vec(),
            13 => b"Surprised".to_vec(),
            14 => b"Scared".to_vec(),
            15 => b"Angry".to_vec(),
            16 => b"Loving".to_vec(),
            17 => b"Curious".to_vec(),
            18 => b"Mischievous".to_vec(),
            19 => b"Celebrating".to_vec(),
            20 => b"Dancing".to_vec(),
            21 => b"Meditating".to_vec(),
            22 => b"Transforming".to_vec(),
            23 => b"Evolving".to_vec(),
            24 => b"Seasonal Event".to_vec(),
            25 => b"Special Ability".to_vec(),
            26 => b"Time of Day".to_vec(),
            27 => b"Weather".to_vec(),
            28 => b"Mood Swing".to_vec(),
            29 => b"Elemental Reaction".to_vec(),
            _ => b"Unknown".to_vec(),
        }.try_into().unwrap_or_default();
        
        // Get the animation description based on the trigger
        let description: BoundedVec<u8, ConstU32<128>> = match trigger {
            0 => b"The pet is idle and waiting for interaction.".to_vec(),
            1 => b"The pet is happy and showing it!".to_vec(),
            2 => b"The pet is sad and needs attention.".to_vec(),
            3 => b"The pet is excited about something!".to_vec(),
            4 => b"The pet is tired and needs rest.".to_vec(),
            5 => b"The pet is hungry and needs food.".to_vec(),
            6 => b"The pet is playing and having fun.".to_vec(),
            7 => b"The pet is sleeping peacefully.".to_vec(),
            8 => b"The pet is interacting with something or someone.".to_vec(),
            9 => b"The pet is reacting to a change in its environment.".to_vec(),
            10 => b"The pet is leveling up and evolving!".to_vec(),
            11 => b"The pet has unlocked an achievement and is celebrating!".to_vec(),
            12 => b"The pet has learned a new skill and is showing it off!".to_vec(),
            13 => b"The pet is surprised by something unexpected!".to_vec(),
            14 => b"The pet is scared and looking for comfort.".to_vec(),
            15 => b"The pet is angry and showing its displeasure.".to_vec(),
            16 => b"The pet is showing affection and love.".to_vec(),
            17 => b"The pet is curious about something new.".to_vec(),
            18 => b"The pet is feeling mischievous and playful.".to_vec(),
            19 => b"The pet is celebrating a special occasion!".to_vec(),
            20 => b"The pet is dancing to a rhythm only it can hear.".to_vec(),
            21 => b"The pet is meditating and focusing its energy.".to_vec(),
            22 => b"The pet is transforming into a different form!".to_vec(),
            23 => b"The pet is evolving to its next stage!".to_vec(),
            24 => b"The pet is participating in a seasonal event.".to_vec(),
            25 => b"The pet is using a special ability!".to_vec(),
            26 => b"The pet is reacting to the time of day.".to_vec(),
            27 => b"The pet is reacting to the weather.".to_vec(),
            28 => b"The pet is experiencing a sudden mood change.".to_vec(),
            29 => b"The pet is reacting to an elemental stimulus.".to_vec(),
            _ => b"Unknown animation.".to_vec(),
        }.try_into().unwrap_or_default();
        
        // Generate animation parameters based on style and trigger
        let mut parameters_vec = Vec::new();
        
        // Add style-specific parameters
        match style {
            0 => parameters_vec.extend_from_slice(b"style=default;intensity=medium;speed=normal"),
            1 => parameters_vec.extend_from_slice(b"style=playful;intensity=high;speed=fast"),
            2 => parameters_vec.extend_from_slice(b"style=elegant;intensity=low;speed=slow"),
            3 => parameters_vec.extend_from_slice(b"style=energetic;intensity=very_high;speed=very_fast"),
            4 => parameters_vec.extend_from_slice(b"style=relaxed;intensity=very_low;speed=very_slow"),
            5 => parameters_vec.extend_from_slice(b"style=mysterious;intensity=medium;speed=variable"),
            6 => parameters_vec.extend_from_slice(b"style=robotic;intensity=medium;speed=precise"),
            7 => parameters_vec.extend_from_slice(b"style=fluid;intensity=medium;speed=smooth"),
            8 => parameters_vec.extend_from_slice(b"style=bouncy;intensity=high;speed=variable"),
            9 => parameters_vec.extend_from_slice(b"style=glitchy;intensity=high;speed=erratic"),
            10 => parameters_vec.extend_from_slice(b"style=ethereal;intensity=low;speed=flowing"),
            11 => parameters_vec.extend_from_slice(b"style=cosmic;intensity=high;speed=pulsing"),
            12 => parameters_vec.extend_from_slice(b"style=pixelated;intensity=medium;speed=stepped"),
            13 => parameters_vec.extend_from_slice(b"style=holographic;intensity=medium;speed=wavering"),
            14 => parameters_vec.extend_from_slice(b"style=neon;intensity=high;speed=flashing"),
            15 => parameters_vec.extend_from_slice(b"style=retro;intensity=medium;speed=choppy"),
            16 => parameters_vec.extend_from_slice(b"style=futuristic;intensity=high;speed=smooth"),
            17 => parameters_vec.extend_from_slice(b"style=magical;intensity=high;speed=sparkling"),
            18 => parameters_vec.extend_from_slice(b"style=elemental;intensity=high;speed=flowing"),
            19 => parameters_vec.extend_from_slice(b"style=seasonal;intensity=medium;speed=themed"),
            _ => parameters_vec.extend_from_slice(b"style=default;intensity=medium;speed=normal"),
        }
        
        // Add trigger-specific parameters
        parameters_vec.extend_from_slice(b";trigger=");
        match trigger {
            0 => parameters_vec.extend_from_slice(b"idle;duration=continuous;loop=true"),
            1 => parameters_vec.extend_from_slice(b"happy;duration=medium;loop=true;particles=sparkles"),
            2 => parameters_vec.extend_from_slice(b"sad;duration=medium;loop=true;particles=raindrops"),
            3 => parameters_vec.extend_from_slice(b"excited;duration=short;loop=true;particles=stars"),
            4 => parameters_vec.extend_from_slice(b"tired;duration=long;loop=true;particles=zzz"),
            5 => parameters_vec.extend_from_slice(b"hungry;duration=medium;loop=true;particles=food"),
            6 => parameters_vec.extend_from_slice(b"playing;duration=long;loop=true;particles=toys"),
            7 => parameters_vec.extend_from_slice(b"sleeping;duration=very_long;loop=true;particles=dreams"),
            8 => parameters_vec.extend_from_slice(b"interacting;duration=short;loop=false;particles=hearts"),
            9 => parameters_vec.extend_from_slice(b"environment_change;duration=medium;loop=false;particles=environment"),
            10 => parameters_vec.extend_from_slice(b"level_up;duration=short;loop=false;particles=level_up;sound=level_up;effect=glow"),
            11 => parameters_vec.extend_from_slice(b"achievement;duration=short;loop=false;particles=achievement;sound=achievement;effect=fireworks"),
            12 => parameters_vec.extend_from_slice(b"new_skill;duration=medium;loop=false;particles=skill;sound=skill;effect=sparkle"),
            13 => parameters_vec.extend_from_slice(b"surprised;duration=very_short;loop=false;particles=exclamation;sound=surprised"),
            14 => parameters_vec.extend_from_slice(b"scared;duration=short;loop=true;particles=sweat;sound=scared"),
            15 => parameters_vec.extend_from_slice(b"angry;duration=medium;loop=true;particles=fire;sound=angry"),
            16 => parameters_vec.extend_from_slice(b"loving;duration=medium;loop=true;particles=hearts;sound=loving"),
            17 => parameters_vec.extend_from_slice(b"curious;duration=medium;loop=true;particles=question;sound=curious"),
            18 => parameters_vec.extend_from_slice(b"mischievous;duration=medium;loop=true;particles=stars;sound=mischievous"),
            19 => parameters_vec.extend_from_slice(b"celebrating;duration=long;loop=true;particles=confetti;sound=celebrating;effect=party"),
            20 => parameters_vec.extend_from_slice(b"dancing;duration=long;loop=true;particles=music;sound=dancing;effect=rhythm"),
            21 => parameters_vec.extend_from_slice(b"meditating;duration=long;loop=true;particles=aura;sound=meditating;effect=calm"),
            22 => parameters_vec.extend_from_slice(b"transforming;duration=medium;loop=false;particles=transform;sound=transform;effect=morph"),
            23 => parameters_vec.extend_from_slice(b"evolving;duration=long;loop=false;particles=evolution;sound=evolve;effect=evolve"),
            24 => parameters_vec.extend_from_slice(b"seasonal;duration=medium;loop=true;particles=seasonal;sound=seasonal;effect=seasonal"),
            25 => parameters_vec.extend_from_slice(b"special_ability;duration=medium;loop=false;particles=ability;sound=ability;effect=power"),
            26 => parameters_vec.extend_from_slice(b"time_of_day;duration=medium;loop=true;particles=time;effect=time"),
            27 => parameters_vec.extend_from_slice(b"weather;duration=medium;loop=true;particles=weather;effect=weather"),
            28 => parameters_vec.extend_from_slice(b"mood_swing;duration=short;loop=false;particles=mood;sound=mood;effect=swing"),
            29 => parameters_vec.extend_from_slice(b"elemental;duration=medium;loop=false;particles=element;sound=element;effect=element"),
            _ => parameters_vec.extend_from_slice(b"unknown;duration=medium;loop=false"),
        }
        
        let parameters: BoundedVec<u8, ConstU32<256>> = parameters_vec.try_into().unwrap_or_default();
        let triggers: BoundedVec<u8, ConstU32<16>> = vec![trigger].try_into().unwrap_or_default();
        
        Ok(Animation {
            animation_id: trigger,
            name,
            description,
            parameters,
            triggers,
        })
    }
    
    /// Generates default visual attributes for a pet based on its DNA.
    /// 
    /// # Parameters
    /// 
    /// * `pet` - The pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<VisualAttribute>, DispatchError>` - The default attributes, or an error
    fn generate_default_attributes(pet: &PetNft<T>) -> Result<Vec<VisualAttribute>, DispatchError> {
        // In a real implementation, this would generate attributes based on the pet's DNA
        // For now, we'll return more detailed placeholder attributes
        
        let mut attributes = Vec::new();
        
        // Base color based on elemental affinity
        let base_color: BoundedVec<u8, ConstU32<64>> = match pet.primary_elemental_affinity as u8 {
            0 => b"neutral_gray:#A0A0A0".to_vec(),
            1 => b"fire_red:#FF5733".to_vec(),
            2 => b"water_blue:#33A1FF".to_vec(),
            3 => b"earth_brown:#8B4513".to_vec(),
            4 => b"air_white:#F0F0F0".to_vec(),
            5 => b"tech_silver:#C0C0C0".to_vec(),
            6 => b"nature_green:#33A133".to_vec(),
            7 => b"mystic_purple:#9933FF".to_vec(),
            _ => b"neutral_gray:#A0A0A0".to_vec(),
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::BaseColor as u8,
            value: base_color,
        });
        
        // Secondary color based on a combination of stats
        let secondary_color: BoundedVec<u8, ConstU32<64>> = if pet.base_strength > pet.base_intelligence {
            b"warm_orange:#FFA500".to_vec()
        } else {
            b"cool_teal:#008080".to_vec()
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::SecondaryColor as u8,
            value: secondary_color,
        });
        
        // Accent color based on vitality
        let accent_color: BoundedVec<u8, ConstU32<64>> = match pet.base_vitality {
            0..=50 => b"pale_yellow:#FFFF99".to_vec(),
            51..=100 => b"vibrant_pink:#FF66B2".to_vec(),
            101..=150 => b"electric_blue:#0066FF".to_vec(),
            151..=200 => b"neon_green:#33FF33".to_vec(),
            _ => b"royal_gold:#FFD700".to_vec(),
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::AccentColor as u8,
            value: accent_color,
        });
        
        // Eye color based on intelligence
        let eye_color: BoundedVec<u8, ConstU32<64>> = match pet.base_intelligence {
            0..=50 => b"simple_brown:#8B4513".to_vec(),
            51..=100 => b"bright_green:#33CC33".to_vec(),
            101..=150 => b"deep_blue:#0000CC".to_vec(),
            151..=200 => b"intense_violet:#8A2BE2".to_vec(),
            _ => b"glowing_gold:#FFD700".to_vec(),
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::EyeColor as u8,
            value: eye_color,
        });
        
        // Pattern based on agility
        let pattern: BoundedVec<u8, ConstU32<64>> = match pet.base_agility {
            0..=50 => b"solid:pattern=none".to_vec(),
            51..=100 => b"spotted:pattern=dots;density=medium".to_vec(),
            101..=150 => b"striped:pattern=stripes;direction=horizontal".to_vec(),
            151..=200 => b"swirled:pattern=swirl;intensity=high".to_vec(),
            _ => b"geometric:pattern=hexagon;complexity=high".to_vec(),
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::Pattern as u8,
            value: pattern,
        });
        
        // Accessory based on level
        let accessory: BoundedVec<u8, ConstU32<64>> = match pet.level {
            0..=5 => b"none:".to_vec(),
            6..=10 => b"simple_collar:color=blue;material=fabric".to_vec(),
            11..=20 => b"fancy_collar:color=red;material=leather;gems=small".to_vec(),
            21..=30 => b"medallion:material=bronze;design=star".to_vec(),
            31..=40 => b"amulet:material=silver;gem=sapphire;glow=soft".to_vec(),
            41..=50 => b"crown:material=gold;gems=multi;glow=bright".to_vec(),
            _ => b"legendary_artifact:material=unknown;effect=dimensional;glow=pulsing".to_vec(),
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::Accessory as u8,
            value: accessory,
        });
        
        // Background based on mood
        let background: BoundedVec<u8, ConstU32<64>> = match pet.mood_indicator {
            0..=50 => b"gloomy:color=#666666;particles=rain;intensity=low".to_vec(),
            51..=100 => b"neutral:color=#AAAAAA;particles=none".to_vec(),
            101..=150 => b"cheerful:color=#99CCFF;particles=bubbles;intensity=low".to_vec(),
            151..=200 => b"happy:color=#FFCC99;particles=stars;intensity=medium".to_vec(),
            _ => b"ecstatic:color=#FFFF99;particles=sparkles;intensity=high".to_vec(),
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::Background as u8,
            value: background,
        });
        
        // Special effect based on elemental affinity and level
        let special_effect: BoundedVec<u8, ConstU32<64>> = if pet.level >= 20 {
            match pet.primary_elemental_affinity as u8 {
                0 => b"aura:type=neutral;color=#FFFFFF;intensity=low".to_vec(),
                1 => b"flames:type=fire;color=#FF6600;intensity=medium".to_vec(),
                2 => b"bubbles:type=water;color=#66CCFF;intensity=medium".to_vec(),
                3 => b"dust:type=earth;color=#CC9966;intensity=medium".to_vec(),
                4 => b"wind:type=air;color=#FFFFFF;intensity=medium".to_vec(),
                5 => b"circuits:type=tech;color=#66FFCC;intensity=medium".to_vec(),
                6 => b"leaves:type=nature;color=#66CC66;intensity=medium".to_vec(),
                7 => b"sparkles:type=mystic;color=#CC66FF;intensity=medium".to_vec(),
                _ => b"aura:type=neutral;color=#FFFFFF;intensity=low".to_vec(),
            }
        } else {
            b"none:".to_vec()
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::SpecialEffect as u8,
            value: special_effect,
        });
        
        // Aura based on personality traits
        let aura: BoundedVec<u8, ConstU32<64>> = if !pet.personality_traits.is_empty() {
            b"personality_aura:color=#9966FF;intensity=medium;pulsing=true".to_vec()
        } else {
            b"none:".to_vec()
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::Aura as u8,
            value: aura,
        });
        
        // Particles based on mood and level
        let particles: BoundedVec<u8, ConstU32<64>> = if pet.level >= 10 && pet.mood_indicator >= 150 {
            b"happy_particles:type=hearts;color=#FF99CC;frequency=medium;size=small".to_vec()
        } else if pet.level >= 10 && pet.mood_indicator <= 50 {
            b"sad_particles:type=raindrops;color=#6699CC;frequency=low;size=small".to_vec()
        } else {
            b"none:".to_vec()
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::Particles as u8,
            value: particles,
        });
        
        // Outfit based on level
        let outfit: BoundedVec<u8, ConstU32<64>> = match pet.level {
            0..=10 => b"none:".to_vec(),
            11..=20 => b"basic_outfit:type=vest;color=#CCCCCC;material=cloth".to_vec(),
            21..=30 => b"adventurer_outfit:type=explorer;color=#CC9966;material=leather".to_vec(),
            31..=40 => b"magical_robe:type=wizard;color=#6666CC;material=silk;effect=sparkle".to_vec(),
            _ => b"legendary_armor:type=hero;color=#FFCC33;material=enchanted;effect=glow".to_vec(),
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::Outfit as u8,
            value: outfit,
        });
        
        // Mood aura that changes with mood
        let mood_aura: BoundedVec<u8, ConstU32<64>> = match pet.mood_indicator {
            0..=50 => b"sad_aura:color=#6666CC;intensity=low;effect=rain".to_vec(),
            51..=100 => b"neutral_aura:color=#CCCCCC;intensity=low;effect=none".to_vec(),
            101..=150 => b"content_aura:color=#99CC99;intensity=medium;effect=gentle_glow".to_vec(),
            151..=200 => b"happy_aura:color=#FFCC66;intensity=medium;effect=sparkle".to_vec(),
            _ => b"ecstatic_aura:color=#FF99CC;intensity=high;effect=hearts".to_vec(),
        }.try_into().unwrap_or_default();
        
        attributes.push(VisualAttribute {
            attribute_type: AttributeType::MoodAura as u8,
            value: mood_aura,
        });
        
        Ok(attributes)
    }
    
    /// Gets the visual theme for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<VisualTheme, DispatchError>` - The visual theme, or an error
    pub fn get_visual_theme(pet_id: PetId) -> Result<VisualTheme, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the theme ID from storage or generate a default one
        let theme_id = crate::PetVisualTheme::<T>::get(pet_id).unwrap_or_else(|| {
            // Default theme based on elemental affinity
            pet.primary_elemental_affinity as u8
        });
        
        // Get the theme details
        Self::get_theme_by_id(theme_id)
    }
    
    /// Gets a visual theme by ID.
    /// 
    /// # Parameters
    /// 
    /// * `theme_id` - The theme ID
    /// 
    /// # Returns
    /// 
    /// * `Result<VisualTheme, DispatchError>` - The visual theme, or an error
    fn get_theme_by_id(theme_id: u8) -> Result<VisualTheme, DispatchError> {
        // In a real implementation, this would get the theme from storage
        // For now, we'll just return a placeholder theme
        
        let (name, description, color_palette, animation_style, visual_effects) = match theme_id {
            0 => (
                b"Neutral".to_vec(),
                b"A balanced theme with neutral colors and standard animations.".to_vec(),
                b"neutral_palette".to_vec(),
                AnimationStyle::Default as u8,
                vec![0, 1],
            ),
            1 => (
                b"Fiery".to_vec(),
                b"A vibrant theme with warm colors and energetic animations.".to_vec(),
                b"fire_palette".to_vec(),
                AnimationStyle::Energetic as u8,
                vec![2, 3],
            ),
            2 => (
                b"Aquatic".to_vec(),
                b"A cool theme with blue tones and fluid animations.".to_vec(),
                b"water_palette".to_vec(),
                AnimationStyle::Fluid as u8,
                vec![4, 5],
            ),
            3 => (
                b"Earthy".to_vec(),
                b"A grounded theme with natural colors and relaxed animations.".to_vec(),
                b"earth_palette".to_vec(),
                AnimationStyle::Relaxed as u8,
                vec![6, 7],
            ),
            4 => (
                b"Airy".to_vec(),
                b"A light theme with soft colors and playful animations.".to_vec(),
                b"air_palette".to_vec(),
                AnimationStyle::Playful as u8,
                vec![8, 9],
            ),
            5 => (
                b"Tech".to_vec(),
                b"A modern theme with metallic colors and robotic animations.".to_vec(),
                b"tech_palette".to_vec(),
                AnimationStyle::Robotic as u8,
                vec![10, 11],
            ),
            6 => (
                b"Natural".to_vec(),
                b"A lush theme with green tones and elegant animations.".to_vec(),
                b"nature_palette".to_vec(),
                AnimationStyle::Elegant as u8,
                vec![12, 13],
            ),
            7 => (
                b"Mystic".to_vec(),
                b"A mysterious theme with purple hues and arcane animations.".to_vec(),
                b"mystic_palette".to_vec(),
                AnimationStyle::Mysterious as u8,
                vec![14, 15],
            ),
            _ => (
                b"Custom".to_vec(),
                b"A custom theme with unique properties.".to_vec(),
                b"custom_palette".to_vec(),
                AnimationStyle::Default as u8,
                vec![0],
            ),
        };
        
        let bounded_name: BoundedVec<u8, ConstU32<32>> = name.try_into().unwrap_or_default();
        let bounded_description: BoundedVec<u8, ConstU32<128>> = description.try_into().unwrap_or_default();
        let bounded_color_palette: BoundedVec<u8, ConstU32<64>> = color_palette.try_into().unwrap_or_default();
        let bounded_visual_effects: BoundedVec<u8, ConstU32<16>> = visual_effects.try_into().unwrap_or_default();
        
        Ok(VisualTheme {
            theme_id,
            name: bounded_name,
            description: bounded_description,
            color_palette: bounded_color_palette,
            animation_style,
            visual_effects: bounded_visual_effects,
        })
    }
    
    /// Sets the visual theme for a pet.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// * `theme_id` - The theme ID
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn set_visual_theme(
        pet_id: PetId,
        theme_id: u8,
    ) -> DispatchResult {
        // Ensure the pet exists
        ensure!(crate::PetNfts::<T>::contains_key(pet_id), Error::<T>::PetNotFound);
        
        // Ensure the theme exists
        let _ = Self::get_theme_by_id(theme_id)?;
        
        // Set the theme
        crate::PetVisualTheme::<T>::insert(pet_id, theme_id);
        
        Ok(())
    }
    
    /// Gets the visual effects for a pet based on its current state.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<u8>, DispatchError>` - The visual effects, or an error
    pub fn get_visual_effects(pet_id: PetId) -> Result<Vec<u8>, DispatchError> {
        // Get the pet from storage
        let pet = crate::PetNfts::<T>::get(pet_id).ok_or(Error::<T>::PetNotFound)?;
        
        // Get the base effects from the theme
        let theme = Self::get_visual_theme(pet_id)?;
        let mut effects = theme.visual_effects.to_vec();
        
        // Add effects based on mood with more granularity
        match pet.mood_indicator {
            0..=50 => {
                effects.push(21); // Sad cloud
                effects.push(22); // Slow movement
                effects.push(23); // Muted colors
            },
            51..=100 => {
                effects.push(24); // Neutral expression
                effects.push(25); // Normal movement
            },
            101..=150 => {
                effects.push(26); // Content expression
                effects.push(27); // Lively movement
                effects.push(28); // Soft glow
            },
            151..=200 => {
                effects.push(20); // Happy glow
                effects.push(29); // Energetic movement
                effects.push(30); // Sparkle trail
            },
            _ => {
                effects.push(20); // Happy glow
                effects.push(31); // Very energetic movement
                effects.push(32); // Sparkle explosion
                effects.push(33); // Rainbow aura
            }
        }
        
        // Add effects based on level milestones
        match pet.level {
            1..=5 => {
                effects.push(40); // Novice aura (subtle)
            },
            6..=10 => {
                effects.push(41); // Beginner aura
            },
            11..=20 => {
                effects.push(42); // Intermediate aura
            },
            21..=30 => {
                effects.push(43); // Advanced aura
            },
            31..=40 => {
                effects.push(44); // Expert aura
            },
            41..=50 => {
                effects.push(45); // Master aura
            },
            _ => {
                effects.push(46); // Legendary aura
            }
        }
        
        // Add effects based on environment adaptations
        let adaptations = crate::environment::EnvironmentalAdaptationSystem::<T>::get_environmental_adaptations(pet_id);
        for (env_type, level) in adaptations {
            if level >= 3 {
                // Environment-specific effect with intensity based on level
                effects.push(50 + env_type); // Base environment effect
                
                if level >= 5 {
                    effects.push(60 + env_type); // Enhanced environment effect
                }
                
                if level >= 8 {
                    effects.push(70 + env_type); // Master environment effect
                }
            }
        }
        
        // Add effects based on skills
        let skills = crate::training::PetTrainingSystem::<T>::get_skills(pet_id);
        for (skill_type, level) in skills {
            if level >= 3 {
                // Skill-specific effect with intensity based on level
                effects.push(80 + skill_type); // Base skill effect
                
                if level >= 5 {
                    effects.push(90 + skill_type); // Enhanced skill effect
                }
                
                if level >= 8 {
                    effects.push(100 + skill_type); // Master skill effect
                }
            }
        }
        
        // Add effects based on elemental affinity
        match pet.primary_elemental_affinity as u8 {
            0 => effects.push(110), // Neutral element effect
            1 => {
                effects.push(111); // Fire element effect
                if pet.level >= 20 {
                    effects.push(121); // Enhanced fire effect
                }
                if pet.level >= 40 {
                    effects.push(131); // Master fire effect
                }
            },
            2 => {
                effects.push(112); // Water element effect
                if pet.level >= 20 {
                    effects.push(122); // Enhanced water effect
                }
                if pet.level >= 40 {
                    effects.push(132); // Master water effect
                }
            },
            3 => {
                effects.push(113); // Earth element effect
                if pet.level >= 20 {
                    effects.push(123); // Enhanced earth effect
                }
                if pet.level >= 40 {
                    effects.push(133); // Master earth effect
                }
            },
            4 => {
                effects.push(114); // Air element effect
                if pet.level >= 20 {
                    effects.push(124); // Enhanced air effect
                }
                if pet.level >= 40 {
                    effects.push(134); // Master air effect
                }
            },
            5 => {
                effects.push(115); // Tech element effect
                if pet.level >= 20 {
                    effects.push(125); // Enhanced tech effect
                }
                if pet.level >= 40 {
                    effects.push(135); // Master tech effect
                }
            },
            6 => {
                effects.push(116); // Nature element effect
                if pet.level >= 20 {
                    effects.push(126); // Enhanced nature effect
                }
                if pet.level >= 40 {
                    effects.push(136); // Master nature effect
                }
            },
            7 => {
                effects.push(117); // Mystic element effect
                if pet.level >= 20 {
                    effects.push(127); // Enhanced mystic effect
                }
                if pet.level >= 40 {
                    effects.push(137); // Master mystic effect
                }
            },
            _ => effects.push(110), // Default to neutral
        }
        
        // Add time-of-day effects (simulated here, would be based on actual time in production)
        let current_block = frame_system::Pallet::<T>::block_number();
        let time_of_day = (current_block.saturated_into::<u64>() % 24) as u8;
        
        match time_of_day {
            0..=5 => effects.push(140), // Night effect
            6..=8 => effects.push(141), // Dawn effect
            9..=11 => effects.push(142), // Morning effect
            12..=14 => effects.push(143), // Noon effect
            15..=17 => effects.push(144), // Afternoon effect
            18..=20 => effects.push(145), // Dusk effect
            _ => effects.push(146), // Evening effect
        }
        
        // Add seasonal effects (simulated here, would be based on actual date in production)
        let season = (current_block.saturated_into::<u64>() % 4) as u8;
        
        match season {
            0 => effects.push(150), // Spring effect
            1 => effects.push(151), // Summer effect
            2 => effects.push(152), // Autumn effect
            _ => effects.push(153), // Winter effect
        }
        
        // Add special achievement effects
        let achievements = crate::achievements::AchievementSystem::<T>::get_achievements(pet_id);
        for (achievement_id, _) in achievements {
            if achievement_id % 10 == 0 { // Special milestone achievements
                effects.push(160 + (achievement_id / 10) as u8); // Achievement-specific effect
            }
        }
        
        // Add personality-based effects
        if !pet.personality_traits.is_empty() {
            // This is a simplified approach; in a real implementation, we would analyze the traits
            effects.push(170); // Base personality effect
            
            // Add more effects based on number of traits
            if pet.personality_traits.len() >= 3 {
                effects.push(171); // Complex personality effect
            }
            
            if pet.personality_traits.len() >= 5 {
                effects.push(172); // Very complex personality effect
            }
        }
        
        Ok(effects)
    }
}