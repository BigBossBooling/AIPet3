// This file consolidates all traits defined by pallet-critter-nfts that are implemented by other pallets or traits that pallet-critter-nfts implements for other pallets.

pub trait NftManager {
    fn mint_nft(&self, owner: &str, metadata: &str) -> Result<(), NftError>;
    fn transfer_nft(&self, from: &str, to: &str, token_id: u32) -> Result<(), NftError>;
}

pub trait NftManagerForItems {
    fn associate_item_with_nft(&self, token_id: u32, item_id: u32) -> Result<(), NftError>;
}

pub trait NftBreedingHandler {
    fn breed_nfts(&self, parent1_id: u32, parent2_id: u32) -> Result<u32, NftError>;
}

pub trait QuestNftRequirementChecker {
    fn check_nft_requirements(&self, quest_id: u32, token_id: u32) -> Result<bool, NftError>;
}

// Common types used in these traits
pub type SimpleGeneticInfo = Vec<u8>;
pub type DnaHashType = [u8; 32];
pub type SpeciesType = String;
pub type TraitTypeString = String;
pub type ItemCategoryTag = String;

// Associated error types
#[derive(Debug)]
pub enum NftError {
    NotFound,
    InsufficientFunds,
    InvalidOperation,
    // Other error variants can be added here
}