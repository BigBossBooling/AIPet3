//! Unit and integration tests for pallet-critter-nfts.

use super::*;
use crate as pallet_critter_nfts;
use frame_support::{assert_ok, assert_noop, traits::{OnFinalize, OnInitialize}};
use sp_core::H256;
use frame_system as system;
use sp_runtime::{testing::Header, traits::{BlakeTwo256, IdentityLookup}};
use sp_std::vec::Vec;

// --- Mock Runtime Setup ---

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        CritterNfts: pallet_critter_nfts,
        // Add other pallets as needed (e.g., Balances, Items)
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<u64>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// Dummy implementations for required traits
pub struct MockCurrency;
impl frame_support::traits::Currency<u64> for MockCurrency {
    type Balance = u128;
    type PositiveImbalance = ();
    type NegativeImbalance = ();
    fn total_balance(_: &u64) -> u128 { 1_000_000 }
    fn can_slash(_: &u64, _: u128) -> bool { true }
    fn total_issuance() -> u128 { 1_000_000 }
    fn minimum_balance() -> u128 { 1 }
    fn burn(_: u128) -> Self::PositiveImbalance { () }
    fn issue(_: u128) -> Self::NegativeImbalance { () }
    fn free_balance(_: &u64) -> u128 { 1_000_000 }
    fn ensure_can_withdraw(_: &u64, _: u128, _: WithdrawReasons, _: u128) -> frame_support::dispatch::DispatchResult { Ok(()) }
    fn transfer(_: &u64, _: &u64, _: u128, _: ExistenceRequirement) -> frame_support::dispatch::DispatchResult { Ok(()) }
    fn slash(_: &u64, _: u128) -> (u128, Self::NegativeImbalance) { (0, ()) }
    fn deposit_into_existing(_: &u64, _: u128) -> Result<Self::PositiveImbalance, DispatchError> { Ok(()) }
    fn deposit_creating(_: &u64, _: u128) -> Self::PositiveImbalance { () }
    fn withdraw(_: &u64, _: u128, _: WithdrawReasons, _: ExistenceRequirement) -> Result<Self::NegativeImbalance, DispatchError> { Ok(()) }
    fn make_free_balance_be(_: &u64, _: u128) -> frame_support::traits::SignedImbalance<u128, Self> { unimplemented!() }
}

pub struct MockRandomness;
impl frame_support::traits::Randomness<H256, u64> for MockRandomness {
    fn random(_subject: &[u8]) -> (H256, u64) { (H256::repeat_byte(42), 0) }
    fn random_seed() -> (H256, u64) { (H256::repeat_byte(42), 0) }
}

pub struct MockItemHandler;
impl crate::traits::BasicCareItemConsumer<u64, u32, u8, DispatchResult> for MockItemHandler {
    fn consume_item_of_category(_: &u64, _: &u32, _: u8) -> DispatchResult { Ok(()) }
}

impl pallet_critter_nfts::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = MockCurrency;
    type PetRandomness = MockRandomness;
    type MaxOwnedPets = frame_support::traits::ConstU32<5>;
    type MaxSpeciesNameLen = frame_support::traits::ConstU32<16>;
    type MaxPetNameLen = frame_support::traits::ConstU32<16>;
    type MaxTraitStringLen = frame_support::traits::ConstU32<16>;
    type MaxPetPersonalityTraits = frame_support::traits::ConstU32<4>;
    type MaxMoodValue = frame_support::traits::ConstU8<100>;
    type FeedMoodBoost = frame_support::traits::ConstU8<10>;
    type PlayMoodBoost = frame_support::traits::ConstU8<10>;
    type FeedXpGain = frame_support::traits::ConstU32<5>;
    type PlayXpGain = frame_support::traits::ConstU32<5>;
    type NeglectMoodPenalty = frame_support::traits::ConstU8<20>;
    type NeglectThresholdBlocks = frame_support::traits::ConstU64<10>;
    type DailyClaimAmount = frame_support::traits::ConstU128<100>;
    type ClaimCooldownPeriod = frame_support::traits::ConstU64<5>;
    type ItemHandler = MockItemHandler;
}

// Helper to build genesis storage for tests
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
    sp_io::TestExternalities::new(t)
}

// --- Tests ---

#[test]
fn mint_pet_nft_works() {
    new_test_ext().execute_with(|| {
        let species = vec![b'C'; 4];
        let name = vec![b'N'; 4];
        assert_ok!(CritterNfts::mint_pet_nft(Origin::signed(1), species.clone(), name.clone()));
        let pet = CritterNfts::pet_nfts(0).expect("Pet should exist");
        assert_eq!(pet.current_pet_name, name.try_into().unwrap());
        assert_eq!(pet.initial_species, species.try_into().unwrap());
        assert_eq!(CritterNfts::pet_nft_owner(0), Some(1));
    });
}

#[test]
fn transfer_pet_nft_works() {
    new_test_ext().execute_with(|| {
        let species = vec![b'C'; 4];
        let name = vec![b'N'; 4];
        assert_ok!(CritterNfts::mint_pet_nft(Origin::signed(1), species.clone(), name.clone()));
        assert_ok!(CritterNfts::transfer_pet_nft(Origin::signed(1), 2, 0));
        assert_eq!(CritterNfts::pet_nft_owner(0), Some(2));
    });
}

#[test]
fn transfer_pet_nft_fails_for_non_owner() {
    new_test_ext().execute_with(|| {
        let species = vec![b'C'; 4];
        let name = vec![b'N'; 4];
        assert_ok!(CritterNfts::mint_pet_nft(Origin::signed(1), species.clone(), name.clone()));
        assert_noop!(
            CritterNfts::transfer_pet_nft(Origin::signed(2), 3, 0),
            Error::<Test>::NotOwner
        );
    });
}

#[test]
fn update_pet_metadata_works() {
    new_test_ext().execute_with(|| {
        let species = vec![b'C'; 4];
        let name = vec![b'N'; 4];
        assert_ok!(CritterNfts::mint_pet_nft(Origin::signed(1), species.clone(), name.clone()));
        let new_name = Some(vec![b'X'; 4]);
        let new_traits = None;
        assert_ok!(CritterNfts::update_pet_metadata(Origin::signed(1), 0, new_name.clone(), new_traits));
        let pet = CritterNfts::pet_nfts(0).expect("Pet should exist");
        assert_eq!(pet.current_pet_name, new_name.unwrap().try_into().unwrap());
    });
}

#[test]
fn claim_daily_ptcn_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(CritterNfts::claim_daily_ptcn(Origin::signed(1)));
        // Should not allow another claim before cooldown
        assert_noop!(
            CritterNfts::claim_daily_ptcn(Origin::signed(1)),
            Error::<Test>::ClaimCooldownNotMet
        );
    });
}

#[test]
fn feed_pet_works() {
    new_test_ext().execute_with(|| {
        let species = vec![b'C'; 4];
        let name = vec![b'N'; 4];
        assert_ok!(CritterNfts::mint_pet_nft(Origin::signed(1), species.clone(), name.clone()));
        assert_ok!(CritterNfts::feed_pet(Origin::signed(1), 0, 1));
    });
}

#[test]
fn play_with_pet_works() {
    new_test_ext().execute_with(|| {
        let species = vec![b'C'; 4];
        let name = vec![b'N'; 4];
        assert_ok!(CritterNfts::mint_pet_nft(Origin::signed(1), species.clone(), name.clone()));
        assert_ok!(CritterNfts::play_with_pet(Origin::signed(1), 0, 2));
    });
}

#[test]
fn apply_neglect_check_works() {
    new_test_ext().execute_with(|| {
        let species = vec![b'C'; 4];
        let name = vec![b'N'; 4];
        assert_ok!(CritterNfts::mint_pet_nft(Origin::signed(1), species.clone(), name.clone()));
        // Simulate block number increase