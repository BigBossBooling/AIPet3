use crate::{mock::*, Error, ContentStatus, ContentType};
use frame_support::{assert_ok, assert_noop};

// Helper function to submit content
fn submit_test_content() -> Result<(), &'static str> {
    let name = b"Test Content".to_vec();
    let description = b"This is a test content description".to_vec();
    let uri = b"ipfs://QmTest".to_vec();
    let content_hash = [0u8; 32];
    let royalty_percentage = 10;
    
    CommunityContent::submit_content(
        RuntimeOrigin::signed(2),
        ContentType::CritterSkin,
        name,
        description,
        uri,
        content_hash,
        royalty_percentage
    )?;
    
    Ok(())
}

// Helper function to add a moderator
fn add_moderator(account: u64) -> Result<(), &'static str> {
    CommunityContent::add_moderator(RuntimeOrigin::root(), account)?;
    Ok(())
}

#[test]
fn submit_content_works() {
    new_test_ext().execute_with(|| {
        // Ensure account has enough balance for deposit
        assert!(Balances::free_balance(2) >= 100);
        
        // Submit content
        assert_ok!(submit_test_content());
        
        // Check that content was stored
        assert!(CommunityContent::content(0).is_some());
        
        // Check that deposit was reserved
        assert_eq!(Balances::reserved_balance(2), 100);
        
        // Check that content is in pending state
        let content = CommunityContent::content(0).unwrap();
        assert_eq!(content.status, ContentStatus::Pending);
        
        // Check that content is in creator's content list
        let creator_content = CommunityContent::creator_content(2);
        assert!(creator_content.contains(&0));
        
        // Check that content is in pending content list
        let pending_content = CommunityContent::pending_content();
        assert!(pending_content.contains(&0));
        
        // Check that content is in content by type list
        let content_by_type = CommunityContent::content_by_type(ContentType::CritterSkin);
        assert!(content_by_type.contains(&0));
    });
}

#[test]
fn submit_content_fails_with_insufficient_balance() {
    new_test_ext().execute_with(|| {
        // Set balance to less than deposit
        Balances::set_balance(RuntimeOrigin::root(), 4, 50, 0).unwrap();
        
        // Try to submit content
        let name = b"Test Content".to_vec();
        let description = b"This is a test content description".to_vec();
        let uri = b"ipfs://QmTest".to_vec();
        let content_hash = [0u8; 32];
        let royalty_percentage = 10;
        
        assert_noop!(
            CommunityContent::submit_content(
                RuntimeOrigin::signed(4),
                ContentType::CritterSkin,
                name,
                description,
                uri,
                content_hash,
                royalty_percentage
            ),
            Error::<Test>::InsufficientDeposit
        );
    });
}

#[test]
fn submit_content_fails_with_high_royalty() {
    new_test_ext().execute_with(|| {
        // Try to submit content with royalty higher than max
        let name = b"Test Content".to_vec();
        let description = b"This is a test content description".to_vec();
        let uri = b"ipfs://QmTest".to_vec();
        let content_hash = [0u8; 32];
        let royalty_percentage = 20; // Max is 15
        
        assert_noop!(
            CommunityContent::submit_content(
                RuntimeOrigin::signed(2),
                ContentType::CritterSkin,
                name,
                description,
                uri,
                content_hash,
                royalty_percentage
            ),
            Error::<Test>::RoyaltyPercentageTooHigh
        );
    });
}

#[test]
fn moderate_content_works() {
    new_test_ext().execute_with(|| {
        // Submit content
        assert_ok!(submit_test_content());
        
        // Add moderator
        assert_ok!(add_moderator(3));
        
        // Approve content
        assert_ok!(CommunityContent::moderate_content(
            RuntimeOrigin::signed(3),
            0,
            ContentStatus::Approved,
            None
        ));
        
        // Check that content is approved
        let content = CommunityContent::content(0).unwrap();
        assert_eq!(content.status, ContentStatus::Approved);
        
        // Check that deposit was unreserved
        assert_eq!(Balances::reserved_balance(2), 0);
        
        // Check that content is in approved content list
        let approved_content = CommunityContent::approved_content();
        assert!(approved_content.contains(&0));
        
        // Check that content is not in pending content list
        let pending_content = CommunityContent::pending_content();
        assert!(!pending_content.contains(&0));
    });
}

#[test]
fn moderate_content_fails_for_non_moderator() {
    new_test_ext().execute_with(|| {
        // Submit content
        assert_ok!(submit_test_content());
        
        // Try to approve content as non-moderator
        assert_noop!(
            CommunityContent::moderate_content(
                RuntimeOrigin::signed(1),
                0,
                ContentStatus::Approved,
                None
            ),
            Error::<Test>::NotModerator
        );
    });
}

#[test]
fn reject_content_slashes_deposit() {
    new_test_ext().execute_with(|| {
        // Submit content
        assert_ok!(submit_test_content());
        
        // Add moderator
        assert_ok!(add_moderator(3));
        
        // Reject content
        assert_ok!(CommunityContent::moderate_content(
            RuntimeOrigin::signed(3),
            0,
            ContentStatus::Rejected,
            Some(b"Inappropriate content".to_vec())
        ));
        
        // Check that content is rejected
        let content = CommunityContent::content(0).unwrap();
        assert_eq!(content.status, ContentStatus::Rejected);
        
        // Check that deposit was slashed
        assert_eq!(Balances::reserved_balance(2), 0);
        assert_eq!(Balances::free_balance(2), 900); // 1000 - 100
        
        // Check that content is not in pending content list
        let pending_content = CommunityContent::pending_content();
        assert!(!pending_content.contains(&0));
    });
}

#[test]
fn update_content_works() {
    new_test_ext().execute_with(|| {
        // Submit content
        assert_ok!(submit_test_content());
        
        // Add moderator
        assert_ok!(add_moderator(3));
        
        // Approve content
        assert_ok!(CommunityContent::moderate_content(
            RuntimeOrigin::signed(3),
            0,
            ContentStatus::Approved,
            None
        ));
        
        // Update content
        let new_name = b"Updated Content".to_vec();
        let new_description = b"This is an updated description".to_vec();
        
        assert_ok!(CommunityContent::update_content(
            RuntimeOrigin::signed(2),
            0,
            Some(new_name),
            Some(new_description),
            None,
            None
        ));
        
        // Check that content was updated
        let content = CommunityContent::content(0).unwrap();
        assert_eq!(content.name, b"Updated Content".to_vec());
        
        let description = CommunityContent::content_descriptions(0).unwrap();
        assert_eq!(description, b"This is an updated description".to_vec());
    });
}

#[test]
fn update_content_fails_for_non_creator() {
    new_test_ext().execute_with(|| {
        // Submit content
        assert_ok!(submit_test_content());
        
        // Add moderator
        assert_ok!(add_moderator(3));
        
        // Approve content
        assert_ok!(CommunityContent::moderate_content(
            RuntimeOrigin::signed(3),
            0,
            ContentStatus::Approved,
            None
        ));
        
        // Try to update content as non-creator
        let new_name = b"Updated Content".to_vec();
        
        assert_noop!(
            CommunityContent::update_content(
                RuntimeOrigin::signed(1),
                0,
                Some(new_name),
                None,
                None,
                None
            ),
            Error::<Test>::NotContentCreator
        );
    });
}

#[test]
fn update_content_fails_for_non_approved_content() {
    new_test_ext().execute_with(|| {
        // Submit content
        assert_ok!(submit_test_content());
        
        // Try to update content before approval
        let new_name = b"Updated Content".to_vec();
        
        assert_noop!(
            CommunityContent::update_content(
                RuntimeOrigin::signed(2),
                0,
                Some(new_name),
                None,
                None,
                None
            ),
            Error::<Test>::ContentNotApproved
        );
    });
}

#[test]
fn record_purchase_works() {
    new_test_ext().execute_with(|| {
        // Submit content
        assert_ok!(submit_test_content());
        
        // Add moderator
        assert_ok!(add_moderator(3));
        
        // Approve content
        assert_ok!(CommunityContent::moderate_content(
            RuntimeOrigin::signed(3),
            0,
            ContentStatus::Approved,
            None
        ));
        
        // Record purchase
        assert_ok!(CommunityContent::record_purchase(
            RuntimeOrigin::signed(1), // In production, this would be the marketplace pallet
            0,
            1,
            100
        ));
        
        // Check that purchase was recorded
        let content = CommunityContent::content(0).unwrap();
        assert_eq!(content.purchase_count, 1);
        assert_eq!(content.total_earnings, 100);
    });
}

#[test]
fn record_usage_works() {
    new_test_ext().execute_with(|| {
        // Submit content
        assert_ok!(submit_test_content());
        
        // Add moderator
        assert_ok!(add_moderator(3));
        
        // Approve content
        assert_ok!(CommunityContent::moderate_content(
            RuntimeOrigin::signed(3),
            0,
            ContentStatus::Approved,
            None
        ));
        
        // Record usage
        assert_ok!(CommunityContent::record_usage(
            RuntimeOrigin::signed(1), // In production, this would be the game logic
            0,
            1
        ));
        
        // Check that usage was recorded
        let content = CommunityContent::content(0).unwrap();
        assert_eq!(content.usage_count, 1);
    });
}

#[test]
fn pay_royalty_works() {
    new_test_ext().execute_with(|| {
        // Submit content
        assert_ok!(submit_test_content());
        
        // Add moderator
        assert_ok!(add_moderator(3));
        
        // Approve content
        assert_ok!(CommunityContent::moderate_content(
            RuntimeOrigin::signed(3),
            0,
            ContentStatus::Approved,
            None
        ));
        
        // Initial balances
        let initial_creator_balance = Balances::free_balance(2);
        let initial_treasury_balance = Balances::free_balance(999);
        
        // Transfer funds to treasury for royalty payment
        assert_ok!(Balances::transfer(
            RuntimeOrigin::signed(1),
            999,
            100
        ));
        
        // Pay royalty
        assert_ok!(CommunityContent::pay_royalty(&0, 100));
        
        // Check that royalty was paid
        // Royalty percentage is 10%, so 10 should be paid
        assert_eq!(Balances::free_balance(2), initial_creator_balance + 10);
        assert_eq!(Balances::free_balance(999), initial_treasury_balance + 100 - 10);
        
        // Check that earnings were updated
        let content = CommunityContent::content(0).unwrap();
        assert_eq!(content.total_earnings, 10);
    });
}

#[test]
fn add_remove_moderator_works() {
    new_test_ext().execute_with(|| {
        // Add moderator
        assert_ok!(add_moderator(3));
        
        // Check that account is a moderator
        assert!(CommunityContent::moderators(3));
        
        // Remove moderator
        assert_ok!(CommunityContent::remove_moderator(RuntimeOrigin::root(), 3));
        
        // Check that account is no longer a moderator
        assert!(!CommunityContent::moderators(3));
    });
}

#[test]
fn add_remove_moderator_fails_for_non_root() {
    new_test_ext().execute_with(|| {
        // Try to add moderator as non-root
        assert_noop!(
            CommunityContent::add_moderator(RuntimeOrigin::signed(1), 3),
            sp_runtime::DispatchError::BadOrigin
        );
        
        // Add moderator properly
        assert_ok!(add_moderator(3));
        
        // Try to remove moderator as non-root
        assert_noop!(
            CommunityContent::remove_moderator(RuntimeOrigin::signed(1), 3),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}