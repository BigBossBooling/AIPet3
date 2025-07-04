//! Benchmarking for pallet-critter-jobs
//!
//! This file defines benchmarks for each extrinsic using the `frame_benchmarking` framework.
//! These benchmarks measure the computational and storage costs of pallet operations,
//! which are then used to generate accurate dispatch weights for transaction fees
//! and to ensure the economic integrity of the CritterChain network.
//!
//! Run `cargo test --features=runtime-benchmarks` and `cargo benchmark` to generate weights.
//! Meticulously crafted to align with The Architect's vision for
//! performance optimization and resource management in the CritterCraft digital ecosystem.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use sp_std::prelude::*;
use frame_support::traits::Get;

// Helper functions and constants
const SEED: u32 = 0;

fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    account(name, SEED, 0)
}

benchmarks! {
    // Benchmark for starting a job
    start_job {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32; // Assuming pet with ID 0 exists and is owned by caller
        let duration_blocks = T::MinJobDuration::get().saturating_add(10u32.into());
        
        // Mock the NftHandler trait to return true for is_owner
        // This would need to be properly implemented in the runtime
    }: {
        Pallet::<T>::start_job(RawOrigin::Signed(caller.clone()).into(), pet_id, JobType::CrystalMining, duration_blocks)?;
    }
    verify {
        let job_id = 0u32; // First job created should have ID 0
        assert!(JobInstances::<T>::contains_key(job_id));
        let job = JobInstances::<T>::get(job_id).unwrap();
        assert_eq!(job.pet_id, pet_id);
        assert_eq!(job.job_type, JobType::CrystalMining);
        assert_eq!(job.status, JobStatus::Active);
        
        assert!(PetActiveJob::<T>::contains_key(pet_id));
        assert_eq!(PetActiveJob::<T>::get(pet_id).unwrap(), job_id);
    }

    // Benchmark for completing a job
    complete_job {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let job_id = 0u32;
        let duration_blocks = T::MinJobDuration::get();
        
        // Create a job first
        Pallet::<T>::start_job(RawOrigin::Signed(caller.clone()).into(), pet_id, JobType::CrystalMining, duration_blocks)?;
        
        // Fast forward to job completion
        let job = JobInstances::<T>::get(job_id).unwrap();
        frame_system::Pallet::<T>::set_block_number(job.end_block);
    }: {
        Pallet::<T>::complete_job(RawOrigin::Signed(caller.clone()).into(), job_id)?;
    }
    verify {
        assert!(JobInstances::<T>::contains_key(job_id));
        let job = JobInstances::<T>::get(job_id).unwrap();
        assert_eq!(job.status, JobStatus::Completed);
        
        assert!(!PetActiveJob::<T>::contains_key(pet_id));
    }

    // Benchmark for abandoning a job
    abandon_job {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let job_id = 0u32;
        let duration_blocks = T::MinJobDuration::get();
        
        // Create a job first
        Pallet::<T>::start_job(RawOrigin::Signed(caller.clone()).into(), pet_id, JobType::CrystalMining, duration_blocks)?;
    }: {
        Pallet::<T>::abandon_job(RawOrigin::Signed(caller.clone()).into(), job_id)?;
    }
    verify {
        assert!(JobInstances::<T>::contains_key(job_id));
        let job = JobInstances::<T>::get(job_id).unwrap();
        assert_eq!(job.status, JobStatus::Abandoned);
        
        assert!(!PetActiveJob::<T>::contains_key(pet_id));
    }

    // Benchmark for starting a Crystal Mining job
    start_crystal_mining {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let duration_blocks = T::MinJobDuration::get().saturating_add(10u32.into());
    }: {
        Pallet::<T>::start_crystal_mining(RawOrigin::Signed(caller.clone()).into(), pet_id, duration_blocks)?;
    }
    verify {
        let job_id = 0u32;
        assert!(JobInstances::<T>::contains_key(job_id));
        let job = JobInstances::<T>::get(job_id).unwrap();
        assert_eq!(job.job_type, JobType::CrystalMining);
    }

    // Benchmark for starting a Bioluminescent Guide job
    start_bioluminescent_guide {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let duration_blocks = T::MinJobDuration::get().saturating_add(10u32.into());
    }: {
        Pallet::<T>::start_bioluminescent_guide(RawOrigin::Signed(caller.clone()).into(), pet_id, duration_blocks)?;
    }
    verify {
        let job_id = 0u32;
        assert!(JobInstances::<T>::contains_key(job_id));
        let job = JobInstances::<T>::get(job_id).unwrap();
        assert_eq!(job.job_type, JobType::BioluminescentGuide);
    }

    // Benchmark for starting a Herbalist Assistant job
    start_herbalist_assistant {
        let caller: T::AccountId = whitelisted_caller();
        let pet_id = 0u32;
        let duration_blocks = T::MinJobDuration::get().saturating_add(10u32.into());
    }: {
        Pallet::<T>::start_herbalist_assistant(RawOrigin::Signed(caller.clone()).into(), pet_id, duration_blocks)?;
    }
    verify {
        let job_id = 0u32;
        assert!(JobInstances::<T>::contains_key(job_id));
        let job = JobInstances::<T>::get(job_id).unwrap();
        assert_eq!(job.job_type, JobType::HerbalistAssistant);
    }
}

#[cfg(test)]
mod tests {
    use super::Pallet as CritterJobs;
    frame_benchmarking::impl_benchmark_test_suite!(
        CritterJobs,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}