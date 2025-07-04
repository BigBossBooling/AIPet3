//! Mock runtime for pallet-critter-minigames tests

use crate as pallet_critter_minigames;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, ConstU128, Randomness},
};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use frame_system as system;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Balances: pallet_balances,
        CritterMinigames: pallet_critter_minigames,
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type Balance = u128;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type MaxHolds = ();
}

// Mock randomness source
pub struct MockRandomness;
impl Randomness<H256, u64> for MockRandomness {
    fn random(_subject: &[u8]) -> (H256, u64) {
        (H256::default(), 0)
    }
}

// Mock NftHandler implementation
pub struct MockNftHandler;
impl pallet_critter_minigames::traits::NftManagerForItems<u64, u32, u32, frame_support::dispatch::DispatchResult> for MockNftHandler {
    fn is_owner(_owner: &u64, _pet_id: &u32) -> bool {
        true // Always return true for testing
    }
    
    fn add_experience(_pet_id: &u32, _xp_amount: u32) -> frame_support::dispatch::DispatchResult {
        Ok(()) // Always succeed for testing
    }
}

parameter_types! {
    pub const MaxActiveGames: u32 = 5;
    pub const MaxCommentLen: u32 = 100;
    pub const BaseXpReward: u32 = 10;
    pub const BaseBitsReward: u128 = 100;
    pub const DifficultyXpMultiplier: u32 = 2;
    pub const DifficultyBitsMultiplier: u32 = 2;
}

impl pallet_critter_minigames::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type GameRandomness = MockRandomness;
    type MaxActiveGames = MaxActiveGames;
    type MaxCommentLen = MaxCommentLen;
    type BaseXpReward = BaseXpReward;
    type BaseBitsReward = BaseBitsReward;
    type DifficultyXpMultiplier = DifficultyXpMultiplier;
    type DifficultyBitsMultiplier = DifficultyBitsMultiplier;
    type NftHandler = MockNftHandler;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}