//! # CritterCraft Shared Traits
//!
//! ## The Architect's Vision
//!
//! This crate provides the strategic interfaces that unify the CritterCraft ecosystem.
//! Guided by the Expanded KISS Principle, it establishes a clear, modular, and
//! robust contract for cross-pallet communication. Each trait defines a focused
//! capability, allowing for complex interactions while ensuring a clean separation
//! of concerns between the core game logic pallets.

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export all the domain-specific traits for easy consumption by other pallets.
pub mod nft;
pub mod breeding;
pub mod items;
pub mod quests;
pub mod battle;
pub mod daycare;
pub mod governance;
pub mod types;

/// The core configuration trait that all other traits in this crate depend on.
/// (S) - This systematizes the entire interface layer. Any pallet wishing to
/// interact with the ecosystem must implement this single, central trait.
pub trait Config {
    /// The type used to identify a unique user account.
    type AccountId;
    /// The type used to identify a unique pet NFT.
    type PetId;
    /// The type used to identify a unique item.
    type ItemId;
    /// The type used to identify a unique quest.
    type QuestId;
    /// The type used for currency balances.
    type Balance;
    /// The type used for block numbers.
    type BlockNumber;
}