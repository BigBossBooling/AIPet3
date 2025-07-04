use crate as pallet_community_content;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, ConstU128, Randomness},
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        CommunityContent: pallet_community_content,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
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
    type RuntimeHoldReason = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type MaxFreezes = ();
}

// Mock randomness source
pub struct MockRandomness;
impl Randomness<H256, u64> for MockRandomness {
    fn random(_subject: &[u8]) -> (H256, u64) {
        (H256::default(), 0)
    }
}

// Mock time provider
pub struct MockTime;
impl frame_support::traits::Time for MockTime {
    type Moment = u64;

    fn now() -> Self::Moment {
        0
    }
}

parameter_types! {
    pub const MaxNameLength: u32 = 50;
    pub const MaxDescriptionLength: u32 = 1000;
    pub const MaxUriLength: u32 = 200;
    pub const MaxReasonLength: u32 = 500;
    pub const ContentSubmissionDeposit: u128 = 100;
    pub const MaxRoyaltyPercentage: u8 = 15;
    pub const CommunityTreasuryAccount: u64 = 999;
}

impl pallet_community_content::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type TimeProvider = MockTime;
    type ContentId = u64;
    type ContentRandomness = MockRandomness;
    type MaxNameLength = MaxNameLength;
    type MaxDescriptionLength = MaxDescriptionLength;
    type MaxUriLength = MaxUriLength;
    type MaxReasonLength = MaxReasonLength;
    type ContentSubmissionDeposit = ContentSubmissionDeposit;
    type MaxRoyaltyPercentage = MaxRoyaltyPercentage;
    type CommunityTreasuryAccountId = CommunityTreasuryAccount;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
    
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 1000), // Regular user
            (2, 1000), // Content creator
            (3, 1000), // Moderator
            (999, 1000), // Treasury
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    
    t.into()
}