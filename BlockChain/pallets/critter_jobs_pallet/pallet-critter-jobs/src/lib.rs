//! # Critter Jobs Pallet
//!
//! This pallet manages jobs and economic activities for CritterCraft pets.
//! It defines job types, requirements, rewards, and durations that drive
//! the economic engine of the CritterCraft ecosystem.
//!
//! Meticulously crafted to align with The Architect's vision for
//! modularity, scalability, and robust operation of the CritterCraft digital ecosystem.

#![cfg_attr(not(feature = "std"), no_std)] // No standard library for Wasm compilation

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*, // Provides common types and macros for pallets
        traits::{Currency, Randomness}, // Currency for balances, Randomness for job outcomes
        BoundedVec, // For bounded collections, crucial for security
    };
    use frame_system::{
        pallet_prelude::*, // Provides types like BlockNumberFor, AccountId, OriginFor
        ensure_signed,     // Macro to ensure origin is a signed account
    };
    use sp_std::vec::Vec; // Standard Vec for dynamic arrays (used where not bounded)
    use scale_info::TypeInfo; // For `TypeInfo` derive macro
    use frame_support::log; // Correct way to import Substrate's logging macro
    use sp_runtime::SaturatedFrom; // For saturating arithmetic

    // Import traits from critter-nfts pallet
    use crate::traits::{
        NftManagerForItems, // For integration with pet NFTs
        PetId,             // Using PetId from critter-nfts
    };

    // --- Type Aliases ---
    pub type JobId = u32; // Unique identifier for each job instance
    pub type JobDuration = u32; // Duration in blocks

    // --- Enum Definitions ---
    // JobType: Defines the different types of jobs available
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum JobType {
        CrystalMining,      // Strength-based job
        BioluminescentGuide, // Charisma-based job
        HerbalistAssistant,  // Intelligence-based job
    }

    // JobStatus: Defines the current status of a job instance
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum JobStatus {
        Active,
        Completed,
        Abandoned,
    }

    // --- Struct Definitions ---
    // JobInstance: Defines a specific instance of a job
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct JobInstance<T: Config> {
        pub id: JobId,
        pub job_type: JobType,
        pub pet_id: PetId,
        pub owner: T::AccountId,
        pub start_block: BlockNumberFor<T>,
        pub end_block: BlockNumberFor<T>,
        pub status: JobStatus,
        pub bits_reward: BalanceOf<T>,
        pub xp_reward: u32,
    }

    // JobRequirements: Defines the requirements for a job
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub struct JobRequirements {
        pub min_strength: u8,
        pub min_agility: u8,
        pub min_intelligence: u8,
        pub min_vitality: u8,
        pub min_level: u32,
    }

    // BalanceOf<T> type alias for the pallet's currency type.
    pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // --- Pallet Configuration Trait ---
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// The currency trait for handling BITS token balances.
        type Currency: Currency<Self::AccountId>;

        /// The randomness trait for generating job outcomes.
        type JobRandomness: Randomness<Self::Hash, Self::BlockNumber>;
        
        /// Maximum number of active jobs an account can have.
        #[pallet::constant]
        type MaxActiveJobs: Get<u32>;
        
        /// Base BITS reward for completing a job.
        #[pallet::constant]
        type BaseBitsReward: Get<BalanceOf<Self>>;
        
        /// Base XP reward for completing a job.
        #[pallet::constant]
        type BaseXpReward: Get<u32>;
        
        /// Minimum job duration in blocks.
        #[pallet::constant]
        type MinJobDuration: Get<Self::BlockNumber>;
        
        /// Maximum job duration in blocks.
        #[pallet::constant]
        type MaxJobDuration: Get<Self::BlockNumber>;
        
        /// Handler for interacting with pet NFTs.
        type NftHandler: NftManagerForItems<Self::AccountId, PetId, u32, DispatchResult>;
    }

    // --- Pallet Definition ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // --- Pallet Storage Items ---
    #[pallet::storage]
    #[pallet::getter(fn next_job_id)]
    /// Stores the next available unique JobId.
    pub(super) type NextJobId<T: Config> = StorageValue<_, JobId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn job_instances)]
    /// Stores the comprehensive JobInstance data for each JobId.
    pub(super) type JobInstances<T: Config> = StorageMap<_, Blake2_128Concat, JobId, JobInstance<T>>;

    #[pallet::storage]
    #[pallet::getter(fn active_jobs_by_owner)]
    /// Stores a list of active JobIds for each AccountId.
    pub(super) type ActiveJobsByOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<JobId, T::MaxActiveJobs>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pet_active_job)]
    /// Stores the active JobId for each PetId.
    pub(super) type PetActiveJob<T: Config> = StorageMap<_, Blake2_128Concat, PetId, JobId>;

    #[pallet::storage]
    #[pallet::getter(fn job_requirements)]
    /// Stores the requirements for each JobType.
    pub(super) type JobRequirementsByType<T: Config> = StorageMap<_, Blake2_128Concat, JobType, JobRequirements, ValueQuery>;

    // --- Pallet Events ---
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new job has been started. [owner, pet_id, job_id, job_type]
        JobStarted { owner: T::AccountId, pet_id: PetId, job_id: JobId, job_type: JobType },
        
        /// A job has been completed. [owner, pet_id, job_id, bits_earned, xp_gained]
        JobCompleted { owner: T::AccountId, pet_id: PetId, job_id: JobId, bits_earned: BalanceOf<T>, xp_gained: u32 },
        
        /// A job has been abandoned. [owner, pet_id, job_id]
        JobAbandoned { owner: T::AccountId, pet_id: PetId, job_id: JobId },
        
        /// A pet has leveled up from job rewards. [pet_id, new_level]
        PetLeveledUp { pet_id: PetId, new_level: u32 },
    }

    // --- Pallet Errors ---
    #[pallet::error]
    pub enum Error<T> {
        /// The next JobId has overflowed.
        NextJobIdOverflow,
        
        /// An account cannot have more active jobs than MaxActiveJobs.
        ExceedMaxActiveJobs,
        
        /// The specified job instance does not exist.
        JobNotFound,
        
        /// The sender is not the owner of the job instance.
        NotJobOwner,
        
        /// The job is already completed or abandoned.
        JobAlreadyFinished,
        
        /// The job is still in progress.
        JobStillInProgress,
        
        /// The pet is already assigned to another job.
        PetAlreadyWorking,
        
        /// The pet does not exist or is not owned by the sender.
        PetNotOwnedBySender,
        
        /// The pet's stats are too low for the selected job.
        PetStatsInsufficient,
        
        /// The job duration is invalid.
        InvalidJobDuration,
        
        /// The job is not yet complete.
        JobNotYetComplete,
        
        /// Failed to update pet's experience or stats.
        PetUpdateFailed,
        
        /// Failed to transfer BITS rewards.
        RewardTransferFailed,
    }

    // --- Pallet Hooks ---
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // --- Pallet Genesis Configuration ---
    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub crystal_mining_requirements: JobRequirements,
        pub bioluminescent_guide_requirements: JobRequirements,
        pub herbalist_assistant_requirements: JobRequirements,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                crystal_mining_requirements: JobRequirements {
                    min_strength: 10,
                    min_agility: 5,
                    min_intelligence: 5,
                    min_vitality: 8,
                    min_level: 2,
                },
                bioluminescent_guide_requirements: JobRequirements {
                    min_strength: 5,
                    min_agility: 8,
                    min_intelligence: 10,
                    min_vitality: 5,
                    min_level: 2,
                },
                herbalist_assistant_requirements: JobRequirements {
                    min_strength: 5,
                    min_agility: 5,
                    min_intelligence: 12,
                    min_vitality: 5,
                    min_level: 2,
                },
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            JobRequirementsByType::<T>::insert(JobType::CrystalMining, self.crystal_mining_requirements.clone());
            JobRequirementsByType::<T>::insert(JobType::BioluminescentGuide, self.bioluminescent_guide_requirements.clone());
            JobRequirementsByType::<T>::insert(JobType::HerbalistAssistant, self.herbalist_assistant_requirements.clone());
        }
    }

    // --- Pallet Extrinsics ---
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Start a new job for a pet.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn start_job(
            origin: OriginFor<T>,
            pet_id: PetId,
            job_type: JobType,
            duration_blocks: T::BlockNumber,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Check if the sender owns the pet.
            ensure!(T::NftHandler::is_owner(&owner, &pet_id), Error::<T>::PetNotOwnedBySender);
            
            // 2. Check if the pet is already working.
            ensure!(!PetActiveJob::<T>::contains_key(pet_id), Error::<T>::PetAlreadyWorking);
            
            // 3. Check if the account has reached the maximum number of active jobs.
            let active_jobs = ActiveJobsByOwner::<T>::get(&owner);
            ensure!(active_jobs.len() < T::MaxActiveJobs::get() as usize, Error::<T>::ExceedMaxActiveJobs);
            
            // 4. Check if the job duration is valid.
            ensure!(
                duration_blocks >= T::MinJobDuration::get() && duration_blocks <= T::MaxJobDuration::get(),
                Error::<T>::InvalidJobDuration
            );
            
            // 5. Check if the pet meets the job requirements.
            // This would require fetching the pet's stats from the NftHandler.
            // For now, we'll assume the pet meets the requirements.
            
            // 6. Get the next job ID.
            let job_id = Self::next_job_id();
            let next_job_id = job_id.checked_add(1).ok_or(Error::<T>::NextJobIdOverflow)?;
            NextJobId::<T>::put(next_job_id);
            
            // 7. Calculate rewards based on job type and duration.
            let (bits_reward, xp_reward) = Self::calculate_job_rewards(job_type, duration_blocks);
            
            // 8. Create the job instance.
            let current_block = frame_system::Pallet::<T>::block_number();
            let end_block = current_block.saturating_add(duration_blocks);
            let job_instance = JobInstance::<T> {
                id: job_id,
                job_type,
                pet_id,
                owner: owner.clone(),
                start_block: current_block,
                end_block,
                status: JobStatus::Active,
                bits_reward,
                xp_reward,
            };
            
            // 9. Store the job instance.
            JobInstances::<T>::insert(job_id, job_instance);
            
            // 10. Update the active jobs for the owner.
            ActiveJobsByOwner::<T>::try_mutate(&owner, |jobs| -> DispatchResult {
                jobs.try_push(job_id).map_err(|_| Error::<T>::ExceedMaxActiveJobs)?;
                Ok(())
            })?;
            
            // 11. Set the pet's active job.
            PetActiveJob::<T>::insert(pet_id, job_id);
            
            // 12. Emit the event.
            Self::deposit_event(Event::JobStarted {
                owner,
                pet_id,
                job_id,
                job_type,
            });
            
            Ok(())
        }

        /// Complete a job and claim rewards.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn complete_job(
            origin: OriginFor<T>,
            job_id: JobId,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Get the job instance.
            let mut job = JobInstances::<T>::get(job_id).ok_or(Error::<T>::JobNotFound)?;
            
            // 2. Check if the sender is the owner of the job.
            ensure!(job.owner == owner, Error::<T>::NotJobOwner);
            
            // 3. Check if the job is still active.
            ensure!(job.status == JobStatus::Active, Error::<T>::JobAlreadyFinished);
            
            // 4. Check if the job is complete (current block >= end block).
            let current_block = frame_system::Pallet::<T>::block_number();
            ensure!(current_block >= job.end_block, Error::<T>::JobNotYetComplete);
            
            // 5. Update the job status.
            job.status = JobStatus::Completed;
            JobInstances::<T>::insert(job_id, job.clone());
            
            // 6. Transfer BITS rewards to the owner.
            T::Currency::deposit_creating(&owner, job.bits_reward);
            
            // 7. Update the pet's experience.
            // This would call into the NftHandler to update the pet's XP.
            // For now, we'll just emit an event.
            
            // 8. Remove the job from active jobs.
            ActiveJobsByOwner::<T>::try_mutate(&owner, |jobs| -> DispatchResult {
                if let Some(pos) = jobs.iter().position(|&id| id == job_id) {
                    jobs.swap_remove(pos);
                }
                Ok(())
            })?;
            
            // 9. Remove the pet's active job.
            PetActiveJob::<T>::remove(job.pet_id);
            
            // 10. Emit the event.
            Self::deposit_event(Event::JobCompleted {
                owner,
                pet_id: job.pet_id,
                job_id,
                bits_earned: job.bits_reward,
                xp_gained: job.xp_reward,
            });
            
            Ok(())
        }

        /// Abandon a job without claiming rewards.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn abandon_job(
            origin: OriginFor<T>,
            job_id: JobId,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            
            // 1. Get the job instance.
            let mut job = JobInstances::<T>::get(job_id).ok_or(Error::<T>::JobNotFound)?;
            
            // 2. Check if the sender is the owner of the job.
            ensure!(job.owner == owner, Error::<T>::NotJobOwner);
            
            // 3. Check if the job is still active.
            ensure!(job.status == JobStatus::Active, Error::<T>::JobAlreadyFinished);
            
            // 4. Update the job status.
            job.status = JobStatus::Abandoned;
            JobInstances::<T>::insert(job_id, job.clone());
            
            // 5. Remove the job from active jobs.
            ActiveJobsByOwner::<T>::try_mutate(&owner, |jobs| -> DispatchResult {
                if let Some(pos) = jobs.iter().position(|&id| id == job_id) {
                    jobs.swap_remove(pos);
                }
                Ok(())
            })?;
            
            // 6. Remove the pet's active job.
            PetActiveJob::<T>::remove(job.pet_id);
            
            // 7. Emit the event.
            Self::deposit_event(Event::JobAbandoned {
                owner,
                pet_id: job.pet_id,
                job_id,
            });
            
            Ok(())
        }

        /// Start a Crystal Mining job (Strength-based).
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn start_crystal_mining(
            origin: OriginFor<T>,
            pet_id: PetId,
            duration_blocks: T::BlockNumber,
        ) -> DispatchResult {
            Self::start_job(origin, pet_id, JobType::CrystalMining, duration_blocks)
        }

        /// Start a Bioluminescent Guide job (Charisma-based).
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn start_bioluminescent_guide(
            origin: OriginFor<T>,
            pet_id: PetId,
            duration_blocks: T::BlockNumber,
        ) -> DispatchResult {
            Self::start_job(origin, pet_id, JobType::BioluminescentGuide, duration_blocks)
        }

        /// Start a Herbalist Assistant job (Intelligence-based).
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn start_herbalist_assistant(
            origin: OriginFor<T>,
            pet_id: PetId,
            duration_blocks: T::BlockNumber,
        ) -> DispatchResult {
            Self::start_job(origin, pet_id, JobType::HerbalistAssistant, duration_blocks)
        }
    }

    // --- Pallet Internal Helper Functions ---
    impl<T: Config> Pallet<T> {
        /// Calculate rewards based on job type and duration.
        fn calculate_job_rewards(job_type: JobType, duration_blocks: T::BlockNumber) -> (BalanceOf<T>, u32) {
            // Base rewards
            let base_bits = T::BaseBitsReward::get();
            let base_xp = T::BaseXpReward::get();
            
            // Duration factor (1.0 to 2.0 based on duration)
            let min_duration = T::MinJobDuration::get().saturated_into::<u32>();
            let max_duration = T::MaxJobDuration::get().saturated_into::<u32>();
            let duration = duration_blocks.saturated_into::<u32>();
            
            let duration_factor = 1.0 + (duration - min_duration) as f32 / (max_duration - min_duration) as f32;
            
            // Job type multiplier
            let job_type_multiplier = match job_type {
                JobType::CrystalMining => 1.2,
                JobType::BioluminescentGuide => 1.0,
                JobType::HerbalistAssistant => 1.5,
            };
            
            // Calculate final rewards
            let bits_reward = BalanceOf::<T>::saturated_from(
                (base_bits.saturated_into::<u32>() as f32 * duration_factor * job_type_multiplier) as u32
            );
            let xp_reward = (base_xp as f32 * duration_factor * job_type_multiplier) as u32;
            
            (bits_reward, xp_reward)
        }
    }
}

// Define the traits module for external interfaces
pub mod traits {
    use super::*;
    use frame_support::dispatch::DispatchResult;

    // Re-export types from critter-nfts pallet
    pub type PetId = u32;

    // Trait for interacting with pet NFTs
    pub trait NftManagerForItems<AccountId, NftId, ItemId, Result> {
        fn is_owner(owner: &AccountId, pet_id: &NftId) -> bool;
        fn add_experience(pet_id: &NftId, xp_amount: u32) -> Result;
    }
}

// Benchmarking module (empty for now)
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use super::*;
    use frame_benchmarking::{benchmarks, whitelisted_caller, account};
    use frame_system::RawOrigin;

    benchmarks! {
        // Benchmarks would be defined here
    }
}