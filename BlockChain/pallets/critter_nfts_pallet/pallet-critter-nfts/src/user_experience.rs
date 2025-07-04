//! # User Experience Flow System
//!
//! This module provides a system for managing user experience flows,
//! including onboarding, tutorials, notifications, and guided experiences
//! that help users understand and engage with the pet ecosystem.

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
use crate::{Config, Error, PetId, PetNft};

/// Represents a user experience flow step.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct UxFlowStep {
    /// The step ID
    pub step_id: u32,
    
    /// The flow ID this step belongs to
    pub flow_id: u16,
    
    /// The step title
    pub title: BoundedVec<u8, ConstU32<64>>,
    
    /// The step description
    pub description: BoundedVec<u8, ConstU32<256>>,
    
    /// The step type
    pub step_type: u8,
    
    /// The UI element to highlight
    pub highlight_element: BoundedVec<u8, ConstU32<64>>,
    
    /// The action required to complete the step
    pub required_action: BoundedVec<u8, ConstU32<128>>,
    
    /// The next step ID
    pub next_step_id: u32,
    
    /// Whether this step can be skipped
    pub skippable: bool,
}

/// Represents a notification for a user.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct UserNotification {
    /// The notification ID
    pub notification_id: u32,
    
    /// The notification type
    pub notification_type: u8,
    
    /// The notification title
    pub title: BoundedVec<u8, ConstU32<64>>,
    
    /// The notification message
    pub message: BoundedVec<u8, ConstU32<256>>,
    
    /// The notification icon
    pub icon: BoundedVec<u8, ConstU32<64>>,
    
    /// The notification priority
    pub priority: u8,
    
    /// The action associated with the notification
    pub action: BoundedVec<u8, ConstU32<128>>,
    
    /// The timestamp when the notification was created
    pub timestamp: u64,
    
    /// Whether the notification has been read
    pub read: bool,
}

/// Represents a user achievement.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct UserAchievement {
    /// The achievement ID
    pub achievement_id: u32,
    
    /// The achievement title
    pub title: BoundedVec<u8, ConstU32<64>>,
    
    /// The achievement description
    pub description: BoundedVec<u8, ConstU32<256>>,
    
    /// The achievement icon
    pub icon: BoundedVec<u8, ConstU32<64>>,
    
    /// The achievement points
    pub points: u32,
    
    /// The progress towards the achievement (0-100)
    pub progress: u8,
    
    /// Whether the achievement has been unlocked
    pub unlocked: bool,
    
    /// The timestamp when the achievement was unlocked
    pub unlock_timestamp: Option<u64>,
}

/// UX flow step types.
pub enum UxFlowStepType {
    Welcome = 0,
    Tutorial = 1,
    Guided = 2,
    Interactive = 3,
    Reward = 4,
    Milestone = 5,
    Challenge = 6,
    Survey = 7,
}

/// Notification types.
pub enum NotificationType {
    Info = 0,
    Warning = 1,
    Alert = 2,
    Achievement = 3,
    Event = 4,
    Social = 5,
    System = 6,
    Promotional = 7,
}

/// Notification priorities.
pub enum NotificationPriority {
    Low = 0,
    Medium = 1,
    High = 2,
    Urgent = 3,
}

/// A system for managing user experience flows.
pub struct UserExperienceSystem<T: Config> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> UserExperienceSystem<T> {
    /// Gets the current UX flow step for a user.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - The user's account ID
    /// 
    /// # Returns
    /// 
    /// * `Result<Option<UxFlowStep>, DispatchError>` - The current UX flow step, or None if no flow is active, or an error
    pub fn get_current_ux_flow_step(account_id: T::AccountId) -> Result<Option<UxFlowStep>, DispatchError> {
        // Get the user's current flow and step
        let (flow_id, step_id) = crate::UserUxFlow::<T>::get(account_id);
        
        // If no flow is active, return None
        if flow_id == 0 {
            return Ok(None);
        }
        
        // Get the step
        let step = Self::get_ux_flow_step(flow_id, step_id)?;
        
        Ok(Some(step))
    }
    
    /// Gets a UX flow step by flow ID and step ID.
    /// 
    /// # Parameters
    /// 
    /// * `flow_id` - The flow ID
    /// * `step_id` - The step ID
    /// 
    /// # Returns
    /// 
    /// * `Result<UxFlowStep, DispatchError>` - The UX flow step, or an error
    fn get_ux_flow_step(flow_id: u16, step_id: u32) -> Result<UxFlowStep, DispatchError> {
        // In a real implementation, this would get the step from storage
        // For now, we'll just return a placeholder step
        
        let title: BoundedVec<u8, ConstU32<64>> = match step_id {
            0 => b"Welcome to CritterCraft!".to_vec(),
            1 => b"Meet Your New Pet".to_vec(),
            2 => b"Feeding Your Pet".to_vec(),
            3 => b"Playing With Your Pet".to_vec(),
            4 => b"Pet Personality".to_vec(),
            5 => b"Social Interactions".to_vec(),
            6 => b"Environmental Adaptation".to_vec(),
            7 => b"Pet Training".to_vec(),
            8 => b"Pet Memories".to_vec(),
            9 => b"Achievements".to_vec(),
            10 => b"Seasonal Events".to_vec(),
            _ => b"Unknown Step".to_vec(),
        }.try_into().unwrap_or_default();
        
        let description: BoundedVec<u8, ConstU32<256>> = match step_id {
            0 => b"Welcome to CritterCraft, where you can raise and nurture your own virtual pet! This tutorial will guide you through the basics.".to_vec(),
            1 => b"This is your new pet! Each pet has unique attributes and a personality that will evolve over time based on how you interact with it.".to_vec(),
            2 => b"Your pet needs to eat regularly to stay happy and healthy. Tap the feed button to give your pet some food.".to_vec(),
            3 => b"Playing with your pet is important for its development. Tap the play button to engage in a fun activity with your pet.".to_vec(),
            4 => b"Your pet's personality will evolve based on your interactions. Check the personality tab to see your pet's current traits.".to_vec(),
            5 => b"Your pet can interact with other pets to form social bonds. Visit the social tab to see nearby pets.".to_vec(),
            6 => b"Different environments offer unique benefits and challenges for your pet. Visit the environment tab to explore new areas.".to_vec(),
            7 => b"You can train your pet to learn new skills. Visit the training tab to start teaching your pet.".to_vec(),
            8 => b"Your pet forms memories of significant events. Check the memories tab to see what your pet remembers.".to_vec(),
            9 => b"Complete achievements to earn rewards and unlock new features. Visit the achievements tab to see your progress.".to_vec(),
            10 => b"Seasonal events offer special activities and rewards. Check the events tab to see what's currently happening.".to_vec(),
            _ => b"Unknown step description.".to_vec(),
        }.try_into().unwrap_or_default();
        
        let highlight_element: BoundedVec<u8, ConstU32<64>> = match step_id {
            0 => b"welcome_screen".to_vec(),
            1 => b"pet_display".to_vec(),
            2 => b"feed_button".to_vec(),
            3 => b"play_button".to_vec(),
            4 => b"personality_tab".to_vec(),
            5 => b"social_tab".to_vec(),
            6 => b"environment_tab".to_vec(),
            7 => b"training_tab".to_vec(),
            8 => b"memories_tab".to_vec(),
            9 => b"achievements_tab".to_vec(),
            10 => b"events_tab".to_vec(),
            _ => b"".to_vec(),
        }.try_into().unwrap_or_default();
        
        let required_action: BoundedVec<u8, ConstU32<128>> = match step_id {
            0 => b"tap_continue".to_vec(),
            1 => b"tap_pet".to_vec(),
            2 => b"tap_feed_button".to_vec(),
            3 => b"tap_play_button".to_vec(),
            4 => b"view_personality".to_vec(),
            5 => b"view_social".to_vec(),
            6 => b"view_environment".to_vec(),
            7 => b"view_training".to_vec(),
            8 => b"view_memories".to_vec(),
            9 => b"view_achievements".to_vec(),
            10 => b"view_events".to_vec(),
            _ => b"".to_vec(),
        }.try_into().unwrap_or_default();
        
        Ok(UxFlowStep {
            step_id,
            flow_id,
            title,
            description,
            step_type: if step_id == 0 { UxFlowStepType::Welcome as u8 } else { UxFlowStepType::Tutorial as u8 },
            highlight_element,
            required_action,
            next_step_id: step_id + 1,
            skippable: step_id > 3, // First few steps are mandatory
        })
    }
    
    /// Advances a user to the next UX flow step.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - The user's account ID
    /// 
    /// # Returns
    /// 
    /// * `Result<Option<UxFlowStep>, DispatchError>` - The next UX flow step, or None if the flow is complete, or an error
    pub fn advance_ux_flow(account_id: T::AccountId) -> Result<Option<UxFlowStep>, DispatchError> {
        // Get the user's current flow and step
        let (flow_id, step_id) = crate::UserUxFlow::<T>::get(account_id.clone());
        
        // If no flow is active, return None
        if flow_id == 0 {
            return Ok(None);
        }
        
        // Get the current step
        let current_step = Self::get_ux_flow_step(flow_id, step_id)?;
        
        // Get the next step
        let next_step_id = current_step.next_step_id;
        
        // If there is no next step, the flow is complete
        if next_step_id == 0 {
            // Clear the user's flow
            crate::UserUxFlow::<T>::insert(account_id, (0, 0));
            return Ok(None);
        }
        
        // Update the user's current step
        crate::UserUxFlow::<T>::insert(account_id, (flow_id, next_step_id));
        
        // Get the next step
        let next_step = Self::get_ux_flow_step(flow_id, next_step_id)?;
        
        Ok(Some(next_step))
    }
    
    /// Skips the current UX flow step for a user.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - The user's account ID
    /// 
    /// # Returns
    /// 
    /// * `Result<Option<UxFlowStep>, DispatchError>` - The next UX flow step, or None if the flow is complete, or an error
    pub fn skip_ux_flow_step(account_id: T::AccountId) -> Result<Option<UxFlowStep>, DispatchError> {
        // Get the user's current flow and step
        let (flow_id, step_id) = crate::UserUxFlow::<T>::get(account_id.clone());
        
        // If no flow is active, return None
        if flow_id == 0 {
            return Ok(None);
        }
        
        // Get the current step
        let current_step = Self::get_ux_flow_step(flow_id, step_id)?;
        
        // Check if the step can be skipped
        if !current_step.skippable {
            return Err(Error::<T>::CannotSkipStep.into());
        }
        
        // Skip to the next step
        Self::advance_ux_flow(account_id)
    }
    
    /// Starts a UX flow for a user.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - The user's account ID
    /// * `flow_id` - The flow ID to start
    /// 
    /// # Returns
    /// 
    /// * `Result<UxFlowStep, DispatchError>` - The first step of the flow, or an error
    pub fn start_ux_flow(account_id: T::AccountId, flow_id: u16) -> Result<UxFlowStep, DispatchError> {
        // Check if the user already has an active flow
        let (current_flow_id, _) = crate::UserUxFlow::<T>::get(account_id.clone());
        
        if current_flow_id != 0 {
            return Err(Error::<T>::UxFlowAlreadyActive.into());
        }
        
        // Start the flow at the first step
        let first_step_id = 0;
        
        // Update the user's current flow and step
        crate::UserUxFlow::<T>::insert(account_id, (flow_id, first_step_id));
        
        // Get the first step
        let first_step = Self::get_ux_flow_step(flow_id, first_step_id)?;
        
        Ok(first_step)
    }
    
    /// Gets the notifications for a user.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - The user's account ID
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<UserNotification>, DispatchError>` - The notifications, or an error
    pub fn get_notifications(account_id: T::AccountId) -> Result<Vec<UserNotification>, DispatchError> {
        // Get the notifications from storage
        let notifications = crate::UserNotifications::<T>::get(account_id);
        
        Ok(notifications.to_vec())
    }
    
    /// Adds a notification for a user.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - The user's account ID
    /// * `notification_type` - The notification type
    /// * `title` - The notification title
    /// * `message` - The notification message
    /// * `icon` - The notification icon
    /// * `priority` - The notification priority
    /// * `action` - The action associated with the notification
    /// 
    /// # Returns
    /// 
    /// * `Result<u32, DispatchError>` - The notification ID, or an error
    pub fn add_notification(
        account_id: T::AccountId,
        notification_type: u8,
        title: Vec<u8>,
        message: Vec<u8>,
        icon: Vec<u8>,
        priority: u8,
        action: Vec<u8>,
    ) -> Result<u32, DispatchError> {
        // Ensure the notification type is valid
        ensure!(notification_type <= NotificationType::Promotional as u8, Error::<T>::InvalidNotificationType);
        
        // Ensure the priority is valid
        ensure!(priority <= NotificationPriority::Urgent as u8, Error::<T>::InvalidNotificationPriority);
        
        // Convert the inputs to bounded vectors
        let bounded_title: BoundedVec<u8, ConstU32<64>> = title.try_into()
            .map_err(|_| Error::<T>::TitleTooLong)?;
        
        let bounded_message: BoundedVec<u8, ConstU32<256>> = message.try_into()
            .map_err(|_| Error::<T>::MessageTooLong)?;
        
        let bounded_icon: BoundedVec<u8, ConstU32<64>> = icon.try_into()
            .map_err(|_| Error::<T>::IconTooLong)?;
        
        let bounded_action: BoundedVec<u8, ConstU32<128>> = action.try_into()
            .map_err(|_| Error::<T>::ActionTooLong)?;
        
        // Generate a notification ID
        let notification_id = crate::NextNotificationId::<T>::mutate(|id| {
            let current_id = *id;
            *id = id.saturating_add(1);
            current_id
        });
        
        // Get the current timestamp
        let current_block = frame_system::Pallet::<T>::block_number();
        let timestamp = current_block.saturated_into::<u64>();
        
        // Create the notification
        let notification = UserNotification {
            notification_id,
            notification_type,
            title: bounded_title,
            message: bounded_message,
            icon: bounded_icon,
            priority,
            action: bounded_action,
            timestamp,
            read: false,
        };
        
        // Add the notification to the user's notifications
        crate::UserNotifications::<T>::try_mutate(account_id, |notifications| -> DispatchResult {
            notifications.try_push(notification.clone())
                .map_err(|_| Error::<T>::TooManyNotifications)?;
            
            Ok(())
        })?;
        
        Ok(notification_id)
    }
    
    /// Marks a notification as read.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - The user's account ID
    /// * `notification_id` - The notification ID
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if successful, Err otherwise
    pub fn mark_notification_as_read(
        account_id: T::AccountId,
        notification_id: u32,
    ) -> DispatchResult {
        // Update the notification
        crate::UserNotifications::<T>::try_mutate(account_id, |notifications| -> DispatchResult {
            // Find the notification
            let notification_index = notifications.iter().position(|n| n.notification_id == notification_id)
                .ok_or(Error::<T>::NotificationNotFound)?;
            
            // Mark it as read
            notifications[notification_index].read = true;
            
            Ok(())
        })
    }
    
    /// Gets the achievements for a user.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - The user's account ID
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<UserAchievement>, DispatchError>` - The achievements, or an error
    pub fn get_achievements(account_id: T::AccountId) -> Result<Vec<UserAchievement>, DispatchError> {
        // Get the achievements from storage
        let achievements = crate::UserAchievements::<T>::get(account_id);
        
        // If the user has no achievements yet, return the default set
        if achievements.is_empty() {
            return Ok(Self::get_default_achievements());
        }
        
        Ok(achievements.to_vec())
    }
    
    /// Gets the default set of achievements.
    /// 
    /// # Returns
    /// 
    /// * `Vec<UserAchievement>` - The default achievements
    fn get_default_achievements() -> Vec<UserAchievement> {
        // In a real implementation, this would get the default achievements from storage
        // For now, we'll just return hardcoded achievements
        
        vec![
            UserAchievement {
                achievement_id: 0,
                title: b"First Steps".to_vec().try_into().unwrap_or_default(),
                description: b"Reach level 5 with your pet.".to_vec().try_into().unwrap_or_default(),
                icon: b"first_steps_icon".to_vec().try_into().unwrap_or_default(),
                points: 10,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
            UserAchievement {
                achievement_id: 1,
                title: b"Social Butterfly".to_vec().try_into().unwrap_or_default(),
                description: b"Interact with 10 different pets.".to_vec().try_into().unwrap_or_default(),
                icon: b"social_butterfly_icon".to_vec().try_into().unwrap_or_default(),
                points: 20,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
            UserAchievement {
                achievement_id: 2,
                title: b"Explorer".to_vec().try_into().unwrap_or_default(),
                description: b"Adapt to 5 different environments.".to_vec().try_into().unwrap_or_default(),
                icon: b"explorer_icon".to_vec().try_into().unwrap_or_default(),
                points: 30,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
            UserAchievement {
                achievement_id: 3,
                title: b"Master Trainer".to_vec().try_into().unwrap_or_default(),
                description: b"Reach level 5 in 3 different skills.".to_vec().try_into().unwrap_or_default(),
                icon: b"master_trainer_icon".to_vec().try_into().unwrap_or_default(),
                points: 40,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
            UserAchievement {
                achievement_id: 4,
                title: b"Memory Keeper".to_vec().try_into().unwrap_or_default(),
                description: b"Collect 10 significant memories.".to_vec().try_into().unwrap_or_default(),
                icon: b"memory_keeper_icon".to_vec().try_into().unwrap_or_default(),
                points: 30,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
            UserAchievement {
                achievement_id: 5,
                title: b"Seasonal Participant".to_vec().try_into().unwrap_or_default(),
                description: b"Participate in 3 seasonal events.".to_vec().try_into().unwrap_or_default(),
                icon: b"seasonal_participant_icon".to_vec().try_into().unwrap_or_default(),
                points: 20,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
            UserAchievement {
                achievement_id: 6,
                title: b"Lifecycle Guardian".to_vec().try_into().unwrap_or_default(),
                description: b"Experience 3 lifecycle events with your pet.".to_vec().try_into().unwrap_or_default(),
                icon: b"lifecycle_guardian_icon".to_vec().try_into().unwrap_or_default(),
                points: 50,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
            UserAchievement {
                achievement_id: 7,
                title: b"Mood Master".to_vec().try_into().unwrap_or_default(),
                description: b"Keep your pet's mood above 200 for 7 consecutive days.".to_vec().try_into().unwrap_or_default(),
                icon: b"mood_master_icon".to_vec().try_into().unwrap_or_default(),
                points: 30,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
            UserAchievement {
                achievement_id: 8,
                title: b"Collector".to_vec().try_into().unwrap_or_default(),
                description: b"Collect 20 different items.".to_vec().try_into().unwrap_or_default(),
                icon: b"collector_icon".to_vec().try_into().unwrap_or_default(),
                points: 40,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
            UserAchievement {
                achievement_id: 9,
                title: b"Master of Elements".to_vec().try_into().unwrap_or_default(),
                description: b"Interact with pets of all elemental affinities.".to_vec().try_into().unwrap_or_default(),
                icon: b"master_of_elements_icon".to_vec().try_into().unwrap_or_default(),
                points: 50,
                progress: 0,
                unlocked: false,
                unlock_timestamp: None,
            },
        ]
    }
    
    /// Updates the progress of an achievement for a user.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - The user's account ID
    /// * `achievement_id` - The achievement ID
    /// * `progress` - The new progress value
    /// 
    /// # Returns
    /// 
    /// * `Result<bool, DispatchError>` - True if the achievement was unlocked, false otherwise, or an error
    pub fn update_achievement_progress(
        account_id: T::AccountId,
        achievement_id: u32,
        progress: u8,
    ) -> Result<bool, DispatchError> {
        // Ensure the progress is valid
        ensure!(progress <= 100, Error::<T>::InvalidProgress);
        
        // Update the achievement
        let mut unlocked = false;
        
        crate::UserAchievements::<T>::try_mutate(account_id.clone(), |achievements| -> DispatchResult {
            // If the user has no achievements yet, initialize with the default set
            if achievements.is_empty() {
                let default_achievements = Self::get_default_achievements();
                for achievement in default_achievements {
                    achievements.try_push(achievement)
                        .map_err(|_| Error::<T>::TooManyAchievements)?;
                }
            }
            
            // Find the achievement
            let achievement_index = achievements.iter().position(|a| a.achievement_id == achievement_id)
                .ok_or(Error::<T>::AchievementNotFound)?;
            
            // Update the progress
            achievements[achievement_index].progress = progress;
            
            // Check if the achievement is now unlocked
            if progress >= 100 && !achievements[achievement_index].unlocked {
                // Get the current timestamp
                let current_block = frame_system::Pallet::<T>::block_number();
                let timestamp = current_block.saturated_into::<u64>();
                
                // Unlock the achievement
                achievements[achievement_index].unlocked = true;
                achievements[achievement_index].unlock_timestamp = Some(timestamp);
                
                unlocked = true;
                
                // Add a notification
                let _ = Self::add_notification(
                    account_id.clone(),
                    NotificationType::Achievement as u8,
                    achievements[achievement_index].title.to_vec(),
                    achievements[achievement_index].description.to_vec(),
                    achievements[achievement_index].icon.to_vec(),
                    NotificationPriority::Medium as u8,
                    b"view_achievement".to_vec(),
                );
            }
            
            Ok(())
        })?;
        
        Ok(unlocked)
    }
}