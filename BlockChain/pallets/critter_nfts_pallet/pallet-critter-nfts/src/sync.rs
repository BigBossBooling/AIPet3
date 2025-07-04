//! # Synchronization Hooks and Utilities
//!
//! This module defines traits and utilities for synchronizing pet state changes
//! across the CritterCraft ecosystem. It provides mechanisms for other pallets
//! to register hooks that are called when pet state changes occur.

use frame_support::dispatch::DispatchResult;
use crate::traits::PetId;
use frame_system::Config as SystemConfig;

/// Defines the types of state changes that can trigger synchronization hooks.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StateChangeType {
    /// Basic information about the pet (name, species, etc.)
    BasicInfo = 0,
    /// Pet statistics (strength, agility, etc.)
    Stats = 1,
    /// Pet personality traits
    Traits = 2,
    /// Pet ownership
    Ownership = 3,
    /// Pet interactions (feeding, playing, etc.)
    Interactions = 4,
    /// Pet level and experience
    LevelAndXp = 5,
    /// Pet mood
    Mood = 6,
    /// Any other state change
    Other = 7,
}

/// Converts a StateChangeType to a bit flag for the sync_flags field.
pub fn state_change_to_flag(change_type: StateChangeType) -> u8 {
    1 << (change_type as u8)
}

/// A trait for pallets that want to be notified of pet state changes.
pub trait PetStateChangeHook<T: frame_system::Config> {
    /// Called when a pet's state changes.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet whose state changed
    /// * `change_type` - The type of state change that occurred
    /// * `version` - The new version of the pet's state
    /// * `timestamp` - The block number when the change occurred
    /// * `data` - Optional additional data related to the state change
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if the hook was processed successfully, Err otherwise
    fn on_pet_state_change(
        pet_id: PetId,
        change_type: StateChangeType,
        version: u32,
        timestamp: frame_system::pallet_prelude::BlockNumberFor<T>,
        data: Option<Vec<u8>>,
    ) -> DispatchResult;
    
    /// Returns the types of state changes this hook is interested in.
    /// This allows for efficient filtering of notifications.
    /// 
    /// # Returns
    /// 
    /// * `u8` - A bitfield of StateChangeType flags this hook is interested in
    fn interested_in() -> u8 {
        // By default, interested in all state changes
        0xFF
    }
    
    /// Returns the priority of this hook.
    /// Higher priority hooks are executed first.
    /// 
    /// # Returns
    /// 
    /// * `u8` - The priority of this hook (0-255)
    fn priority() -> u8 {
        // Default priority
        128
    }
}

/// Information about a registered hook
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct HookInfo<T: SystemConfig> {
    /// The account ID associated with the hook
    pub account_id: T::AccountId,
    /// The types of state changes this hook is interested in (bitfield)
    pub interests: u8,
    /// The priority of this hook (higher priority hooks are executed first)
    pub priority: u8,
    /// Whether this hook is enabled
    pub enabled: bool,
    /// The last time this hook was successfully executed
    pub last_execution: T::BlockNumber,
    /// The number of times this hook has been executed
    pub execution_count: u32,
}

/// A struct for registering and managing synchronization hooks.
pub struct SyncHookManager<T: SystemConfig> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: SystemConfig> SyncHookManager<T> {
    /// Registers a new synchronization hook with detailed information.
    /// 
    /// # Parameters
    /// 
    /// * `hook_id` - A unique identifier for the hook
    /// * `hook_info` - Detailed information about the hook
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if the hook was registered successfully, Err otherwise
    pub fn register_hook_with_info(hook_id: u32, hook_info: HookInfo<T>) -> DispatchResult {
        crate::SyncHookRegistry::<T>::insert(hook_id, hook_info);
        
        // Emit an event for transparency
        crate::Pallet::<T>::deposit_event(crate::Event::HookRegistered {
            hook_id,
            account_id: hook_info.account_id,
            interests: hook_info.interests,
            priority: hook_info.priority,
        });
        
        Ok(())
    }
    
    /// Registers a new synchronization hook with basic information.
    /// 
    /// # Parameters
    /// 
    /// * `hook_id` - A unique identifier for the hook
    /// * `account_id` - The account ID associated with the hook
    /// * `interests` - The types of state changes this hook is interested in (bitfield)
    /// * `priority` - The priority of this hook (higher priority hooks are executed first)
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if the hook was registered successfully, Err otherwise
    pub fn register_hook(
        hook_id: u32, 
        account_id: T::AccountId,
        interests: u8,
        priority: u8,
    ) -> DispatchResult {
        let current_block = frame_system::Pallet::<T>::block_number();
        let hook_info = HookInfo {
            account_id: account_id.clone(),
            interests,
            priority,
            enabled: true,
            last_execution: current_block,
            execution_count: 0,
        };
        
        Self::register_hook_with_info(hook_id, hook_info)
    }
    
    /// Unregisters a synchronization hook.
    /// 
    /// # Parameters
    /// 
    /// * `hook_id` - The unique identifier of the hook to unregister
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if the hook was unregistered successfully, Err otherwise
    pub fn unregister_hook(hook_id: u32) -> DispatchResult {
        // Get the hook info before removing it for the event
        if let Some(hook_info) = crate::SyncHookRegistry::<T>::get(hook_id) {
            crate::SyncHookRegistry::<T>::remove(hook_id);
            
            // Emit an event for transparency
            crate::Pallet::<T>::deposit_event(crate::Event::HookUnregistered {
                hook_id,
                account_id: hook_info.account_id,
            });
        }
        
        Ok(())
    }
    
    /// Enables or disables a synchronization hook.
    /// 
    /// # Parameters
    /// 
    /// * `hook_id` - The unique identifier of the hook
    /// * `enabled` - Whether the hook should be enabled
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if the hook was updated successfully, Err otherwise
    pub fn set_hook_enabled(hook_id: u32, enabled: bool) -> DispatchResult {
        crate::SyncHookRegistry::<T>::try_mutate(hook_id, |hook_info_opt| -> DispatchResult {
            let hook_info = hook_info_opt.as_mut().ok_or(crate::Error::<T>::HookNotFound)?;
            hook_info.enabled = enabled;
            
            // Emit an event for transparency
            crate::Pallet::<T>::deposit_event(if enabled {
                crate::Event::HookEnabled { hook_id }
            } else {
                crate::Event::HookDisabled { hook_id }
            });
            
            Ok(())
        })
    }
    
    /// Updates a hook's interests.
    /// 
    /// # Parameters
    /// 
    /// * `hook_id` - The unique identifier of the hook
    /// * `interests` - The new interests bitfield
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if the hook was updated successfully, Err otherwise
    pub fn update_hook_interests(hook_id: u32, interests: u8) -> DispatchResult {
        crate::SyncHookRegistry::<T>::try_mutate(hook_id, |hook_info_opt| -> DispatchResult {
            let hook_info = hook_info_opt.as_mut().ok_or(crate::Error::<T>::HookNotFound)?;
            hook_info.interests = interests;
            
            // Emit an event for transparency
            crate::Pallet::<T>::deposit_event(crate::Event::HookInterestsUpdated {
                hook_id,
                interests,
            });
            
            Ok(())
        })
    }
    
    /// Notifies all registered hooks of a pet state change.
    /// Hooks are executed in priority order (highest first).
    /// Only hooks that are interested in the specific state change type are notified.
    /// 
    /// # Parameters
    /// 
    /// * `pet_id` - The ID of the pet whose state changed
    /// * `change_type` - The type of state change that occurred
    /// * `version` - The new version of the pet's state
    /// * `timestamp` - The block number when the change occurred
    /// * `data` - Optional additional data related to the state change
    /// 
    /// # Returns
    /// 
    /// * `DispatchResult` - Ok if all hooks were notified successfully, Err otherwise
    pub fn notify_hooks(
        pet_id: PetId,
        change_type: StateChangeType,
        version: u32,
        timestamp: T::BlockNumber,
        data: Option<Vec<u8>>,
    ) -> DispatchResult {
        // Get the flag for this change type
        let change_flag = state_change_to_flag(change_type);
        
        // Get all registered hooks
        let mut hooks: Vec<(u32, HookInfo<T>)> = crate::SyncHookRegistry::<T>::iter().collect();
        
        // Sort hooks by priority (highest first)
        hooks.sort_by(|(_, a), (_, b)| b.priority.cmp(&a.priority));
        
        // Track successful and failed hooks
        let mut successful_hooks = 0u32;
        let mut failed_hooks = 0u32;
        
        // Execute each hook that is interested in this change type
        for (hook_id, mut hook_info) in hooks {
            // Skip disabled hooks
            if !hook_info.enabled {
                continue;
            }
            
            // Skip hooks that aren't interested in this change type
            if hook_info.interests & change_flag == 0 {
                continue;
            }
            
            // Execute the hook
            // In a real implementation, this would call the hook's on_pet_state_change method
            // For now, we'll just update the hook's execution stats
            hook_info.last_execution = timestamp;
            hook_info.execution_count = hook_info.execution_count.saturating_add(1);
            
            // Update the hook info in storage
            crate::SyncHookRegistry::<T>::insert(hook_id, hook_info.clone());
            
            // Track success/failure
            // In a real implementation, this would depend on the result of the hook execution
            successful_hooks = successful_hooks.saturating_add(1);
        }
        
        // Emit an event for transparency
        crate::Pallet::<T>::deposit_event(crate::Event::PetStateSynchronized {
            pet_id,
            version,
            timestamp,
            change_type: change_type as u8,
            successful_hooks,
            failed_hooks,
        });
        
        Ok(())
    }
    
    /// Gets information about a registered hook.
    /// 
    /// # Parameters
    /// 
    /// * `hook_id` - The unique identifier of the hook
    /// 
    /// # Returns
    /// 
    /// * `Option<HookInfo<T>>` - Information about the hook, or None if not found
    pub fn get_hook_info(hook_id: u32) -> Option<HookInfo<T>> {
        crate::SyncHookRegistry::<T>::get(hook_id)
    }
    
    /// Gets all registered hooks.
    /// 
    /// # Returns
    /// 
    /// * `Vec<(u32, HookInfo<T>)>` - A vector of (hook_id, hook_info) pairs
    pub fn get_all_hooks() -> Vec<(u32, HookInfo<T>)> {
        crate::SyncHookRegistry::<T>::iter().collect()
    }
}