//! Tests for pallet-critter-jobs

use crate::{mock::*, Error, JobStatus, JobType};
use frame_support::{assert_ok, assert_noop};

#[test]
fn start_job_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let duration_blocks = 200;
        
        // Act
        assert_ok!(CritterJobs::start_job(
            RuntimeOrigin::signed(account_id),
            pet_id,
            JobType::CrystalMining,
            duration_blocks
        ));
        
        // Assert
        let job = CritterJobs::job_instances(0).unwrap();
        assert_eq!(job.pet_id, pet_id);
        assert_eq!(job.owner, account_id);
        assert_eq!(job.job_type, JobType::CrystalMining);
        assert_eq!(job.status, JobStatus::Active);
        
        let active_jobs = CritterJobs::active_jobs_by_owner(account_id);
        assert_eq!(active_jobs.len(), 1);
        assert_eq!(active_jobs[0], 0);
        
        let pet_job = CritterJobs::pet_active_job(pet_id).unwrap();
        assert_eq!(pet_job, 0);
    });
}

#[test]
fn complete_job_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let job_id = 0;
        let duration_blocks = 200;
        
        assert_ok!(CritterJobs::start_job(
            RuntimeOrigin::signed(account_id),
            pet_id,
            JobType::CrystalMining,
            duration_blocks
        ));
        
        // Fast forward to job completion
        let job = CritterJobs::job_instances(job_id).unwrap();
        System::set_block_number(job.end_block);
        
        // Act
        assert_ok!(CritterJobs::complete_job(
            RuntimeOrigin::signed(account_id),
            job_id
        ));
        
        // Assert
        let job = CritterJobs::job_instances(job_id).unwrap();
        assert_eq!(job.status, JobStatus::Completed);
        
        let active_jobs = CritterJobs::active_jobs_by_owner(account_id);
        assert_eq!(active_jobs.len(), 0);
        
        assert!(CritterJobs::pet_active_job(pet_id).is_none());
    });
}

#[test]
fn abandon_job_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let job_id = 0;
        let duration_blocks = 200;
        
        assert_ok!(CritterJobs::start_job(
            RuntimeOrigin::signed(account_id),
            pet_id,
            JobType::CrystalMining,
            duration_blocks
        ));
        
        // Act
        assert_ok!(CritterJobs::abandon_job(
            RuntimeOrigin::signed(account_id),
            job_id
        ));
        
        // Assert
        let job = CritterJobs::job_instances(job_id).unwrap();
        assert_eq!(job.status, JobStatus::Abandoned);
        
        let active_jobs = CritterJobs::active_jobs_by_owner(account_id);
        assert_eq!(active_jobs.len(), 0);
        
        assert!(CritterJobs::pet_active_job(pet_id).is_none());
    });
}

#[test]
fn start_crystal_mining_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let duration_blocks = 200;
        
        // Act
        assert_ok!(CritterJobs::start_crystal_mining(
            RuntimeOrigin::signed(account_id),
            pet_id,
            duration_blocks
        ));
        
        // Assert
        let job_id = 0;
        let job = CritterJobs::job_instances(job_id).unwrap();
        assert_eq!(job.job_type, JobType::CrystalMining);
    });
}

#[test]
fn start_bioluminescent_guide_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let duration_blocks = 200;
        
        // Act
        assert_ok!(CritterJobs::start_bioluminescent_guide(
            RuntimeOrigin::signed(account_id),
            pet_id,
            duration_blocks
        ));
        
        // Assert
        let job_id = 0;
        let job = CritterJobs::job_instances(job_id).unwrap();
        assert_eq!(job.job_type, JobType::BioluminescentGuide);
    });
}

#[test]
fn start_herbalist_assistant_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let duration_blocks = 200;
        
        // Act
        assert_ok!(CritterJobs::start_herbalist_assistant(
            RuntimeOrigin::signed(account_id),
            pet_id,
            duration_blocks
        ));
        
        // Assert
        let job_id = 0;
        let job = CritterJobs::job_instances(job_id).unwrap();
        assert_eq!(job.job_type, JobType::HerbalistAssistant);
    });
}

#[test]
fn exceed_max_active_jobs_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let duration_blocks = 200;
        
        // Create maximum number of jobs
        for i in 0..MaxActiveJobs::get() {
            assert_ok!(CritterJobs::start_job(
                RuntimeOrigin::signed(account_id),
                i, // Different pet for each job
                JobType::CrystalMining,
                duration_blocks
            ));
        }
        
        // Act & Assert
        assert_noop!(
            CritterJobs::start_job(
                RuntimeOrigin::signed(account_id),
                MaxActiveJobs::get(), // Another pet
                JobType::CrystalMining,
                duration_blocks
            ),
            Error::<Test>::ExceedMaxActiveJobs
        );
    });
}

#[test]
fn pet_already_working_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let duration_blocks = 200;
        
        assert_ok!(CritterJobs::start_job(
            RuntimeOrigin::signed(account_id),
            pet_id,
            JobType::CrystalMining,
            duration_blocks
        ));
        
        // Act & Assert
        assert_noop!(
            CritterJobs::start_job(
                RuntimeOrigin::signed(account_id),
                pet_id,
                JobType::BioluminescentGuide,
                duration_blocks
            ),
            Error::<Test>::PetAlreadyWorking
        );
    });
}

#[test]
fn not_job_owner_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let other_account_id = 2;
        let pet_id = 0;
        let job_id = 0;
        let duration_blocks = 200;
        
        assert_ok!(CritterJobs::start_job(
            RuntimeOrigin::signed(account_id),
            pet_id,
            JobType::CrystalMining,
            duration_blocks
        ));
        
        // Act & Assert
        assert_noop!(
            CritterJobs::complete_job(
                RuntimeOrigin::signed(other_account_id),
                job_id
            ),
            Error::<Test>::NotJobOwner
        );
        
        assert_noop!(
            CritterJobs::abandon_job(
                RuntimeOrigin::signed(other_account_id),
                job_id
            ),
            Error::<Test>::NotJobOwner
        );
    });
}

#[test]
fn job_already_finished_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let job_id = 0;
        let duration_blocks = 200;
        
        assert_ok!(CritterJobs::start_job(
            RuntimeOrigin::signed(account_id),
            pet_id,
            JobType::CrystalMining,
            duration_blocks
        ));
        
        // Complete the job
        let job = CritterJobs::job_instances(job_id).unwrap();
        System::set_block_number(job.end_block);
        
        assert_ok!(CritterJobs::complete_job(
            RuntimeOrigin::signed(account_id),
            job_id
        ));
        
        // Act & Assert
        assert_noop!(
            CritterJobs::complete_job(
                RuntimeOrigin::signed(account_id),
                job_id
            ),
            Error::<Test>::JobAlreadyFinished
        );
        
        assert_noop!(
            CritterJobs::abandon_job(
                RuntimeOrigin::signed(account_id),
                job_id
            ),
            Error::<Test>::JobAlreadyFinished
        );
    });
}

#[test]
fn job_not_yet_complete_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let job_id = 0;
        let duration_blocks = 200;
        
        assert_ok!(CritterJobs::start_job(
            RuntimeOrigin::signed(account_id),
            pet_id,
            JobType::CrystalMining,
            duration_blocks
        ));
        
        // Act & Assert
        assert_noop!(
            CritterJobs::complete_job(
                RuntimeOrigin::signed(account_id),
                job_id
            ),
            Error::<Test>::JobNotYetComplete
        );
    });
}

#[test]
fn invalid_job_duration_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        
        // Act & Assert - Too short
        assert_noop!(
            CritterJobs::start_job(
                RuntimeOrigin::signed(account_id),
                pet_id,
                JobType::CrystalMining,
                MinJobDuration::get() - 1
            ),
            Error::<Test>::InvalidJobDuration
        );
        
        // Act & Assert - Too long
        assert_noop!(
            CritterJobs::start_job(
                RuntimeOrigin::signed(account_id),
                pet_id,
                JobType::CrystalMining,
                MaxJobDuration::get() + 1
            ),
            Error::<Test>::InvalidJobDuration
        );
    });
}