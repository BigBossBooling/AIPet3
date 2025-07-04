//! # CritterCraft Runtime Integration
//!
//! This module integrates all the CritterCraft pallets into a cohesive runtime.
//! It defines the trait implementations and dependencies between pallets.
//!
//! Meticulously crafted to align with The Architect's vision for
//! modularity, scalability, and robust operation of the CritterCraft digital ecosystem.

use frame_support::{
    parameter_types,
    traits::{ConstU128, ConstU32, ConstU64, ConstU8, Currency, ExistenceRequirement, Randomness},
    weights::Weight,
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

// Import the pallets
use pallet_critter_profiles as profiles;
use pallet_critter_nfts as nfts;
use pallet_critter_pet_status as pet_status;
use pallet_critter_minigames as minigames;
use pallet_critter_jobs as jobs;
use pallet_critter_daycare as daycare;

// Import the new governance and blockchain functionality pallets
use pallet_critter_governance as governance;
use pallet_critter_node_rewards as node_rewards;
use pallet_critter_treasury as treasury;
use pallet_critter_battle as battle;

// Define the runtime
pub struct Runtime;

// Implement the system configuration for the runtime
impl frame_system::Config for Runtime {
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
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

// Define the balances configuration for the runtime
impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type HoldIdentifier = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type MaxFreezes = ();
}

// Define the profiles configuration for the runtime
impl profiles::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxUsernameLen = ConstU32<32>;
    type MaxBioLen = ConstU32<256>;
    type MaxAvatarURILen = ConstU32<256>;
    type MaxFriends = ConstU32<100>;
    type MaxAchievements = ConstU32<1000>;
    type MaxBadges = ConstU32<100>;
    type MaxEquippedBadges = ConstU32<5>;
    type WeightInfo = ();
}

// Define the NFTs configuration for the runtime
impl nfts::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type PetRandomness = RandomnessCollectiveFlip;
    type MaxNameLength = ConstU32<32>;
    type MaxDescriptionLength = ConstU32<256>;
    type MaxAttributes = ConstU32<10>;
    type MaxPetsPerAccount = ConstU32<50>;
    type PetId = u32;
    type MintPrice = ConstU128<1000>;
    type EvolutionPrice = ConstU128<5000>;
    type WeightInfo = ();
}

// Define the pet status configuration for the runtime
impl pet_status::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type PetRandomness = RandomnessCollectiveFlip;
    type MaxConditionNameLen = ConstU32<32>;
    type MaxConditionDescLen = ConstU32<256>;
    type MaxPetConditions = ConstU32<10>;
    type NeedDecayInterval = ConstU64<100>;
    type NeedDecayAmount = ConstU8<1>;
    type HungerInterval = ConstU64<1000>;
    type TirednessInterval = ConstU64<1200>;
    type UnhappinessInterval = ConstU64<800>;
    type DirtinessInterval = ConstU64<1500>;
    type LonelinessInterval = ConstU64<1000>;
}

// Define the minigames configuration for the runtime
impl minigames::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type GameRandomness = RandomnessCollectiveFlip;
    type NftManager = Nfts;
    type BaseExperienceReward = ConstU32<100>;
    type BaseCurrencyReward = ConstU128<50>;
    type GameEntryFee = ConstU128<10>;
    type MaxActiveGames = ConstU32<5>;
    type WeightInfo = ();
}

// Define the jobs configuration for the runtime
impl jobs::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type JobRandomness = RandomnessCollectiveFlip;
    type NftManager = Nfts;
    type BaseExperienceReward = ConstU32<200>;
    type BaseCurrencyReward = ConstU128<100>;
    type MinJobDuration = ConstU64<100>;
    type MaxJobDuration = ConstU64<10000>;
    type MaxActiveJobs = ConstU32<3>;
    type WeightInfo = ();
}

// Define the daycare configuration for the runtime
impl daycare::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type NftManager = Nfts;
    type PetStatusManager = PetStatus;
    type MaxDaycareNameLen = ConstU32<32>;
    type MaxDaycareDescriptionLen = ConstU32<256>;
    type MaxDaycaresPerAccount = ConstU32<3>;
    type MaxListingsPerDaycare = ConstU32<20>;
    type MaxCareRecordsPerListing = ConstU32<100>;
    type PlatformFeePercent = ConstU32<5>;
    type MinListingDuration = ConstU64<100>;
    type MaxListingDuration = ConstU64<10000>;
    type CareActionCooldown = ConstU64<10>;
    type WeightInfo = ();
}

// Define the governance configuration for the runtime
impl governance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type CouncilElectionPeriod = ConstU64<50400>; // ~1 week at 12-second blocks
    type ProposalBond = ConstU128<1000>;
    type VotingBond = ConstU128<100>;
    type MinVotingPeriod = ConstU64<14400>; // ~2 days at 12-second blocks
    type MaxVotingPeriod = ConstU64<100800>; // ~2 weeks at 12-second blocks
    type CouncilSize = ConstU32<5>;
    type MaxProposalWeight = ();
    type MaxProposalSize = ConstU32<16384>; // 16KB
    type CancelOrigin = EnsureRoot<AccountId>;
    type FastTrackOrigin = EnsureRoot<AccountId>;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type WeightInfo = ();
}

// Define the node rewards configuration for the runtime
impl node_rewards::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type RewardPeriod = ConstU64<14400>; // ~2 days at 12-second blocks
    type NodeBond = ConstU128<10000>;
    type MaxMetricsPerPeriod = ConstU32<10>;
    type MaxOfflineReportsPerPeriod = ConstU32<3>;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type WeightInfo = ();
}

// Define the treasury configuration for the runtime
impl treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type TreasuryPeriod = ConstU64<14400>; // ~2 days at 12-second blocks
    type ProposalBond = ConstU128<1000>;
    type MinSpend = ConstU128<100>;
    type MaxSpend = ConstU128<1000000>;
    type TreasuryFeePercent = Perbill::from_percent(20);
    type BurnPercent = Perbill::from_percent(1);
    type ApproveOrigin = EnsureRoot<AccountId>;
    type RejectOrigin = EnsureRoot<AccountId>;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type OnSlash = ();
    type WeightInfo = ();
}

// Define the battle configuration for the runtime
impl battle::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BattleRandomness = RandomnessCollectiveFlip;
    type NftManager = Nfts;
    type PetManager = Nfts;
    type MaxActiveBattles = ConstU32<5>;
    type MaxActiveTournaments = ConstU32<3>;
    type MaxTournamentParticipants = ConstU32<32>;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type WeightInfo = ();
}

// Implement the NftManagerForItems trait for the NFTs pallet
impl minigames::NftManagerForItems<u64, u32> for Nfts {
    fn is_owner(account: &u64, pet_id: &u32) -> bool {
        Nfts::is_owner(account, pet_id)
    }

    fn add_experience(pet_id: &u32, experience: u32) -> DispatchResult {
        Nfts::add_experience(pet_id, experience)
    }

    fn get_pet_level(pet_id: &u32) -> Option<u16> {
        Nfts::get_pet_level(pet_id)
    }

    fn get_pet_attributes(pet_id: &u32) -> Option<Vec<(minigames::AttributeType, u8)>> {
        Nfts::get_pet_attributes(pet_id)
    }
}

// Implement the NftManagerForItems trait for the NFTs pallet (for jobs)
impl jobs::NftManagerForItems<u64, u32> for Nfts {
    fn is_owner(account: &u64, pet_id: &u32) -> bool {
        Nfts::is_owner(account, pet_id)
    }

    fn add_experience(pet_id: &u32, experience: u32) -> DispatchResult {
        Nfts::add_experience(pet_id, experience)
    }

    fn get_pet_level(pet_id: &u32) -> Option<u16> {
        Nfts::get_pet_level(pet_id)
    }

    fn get_pet_attributes(pet_id: &u32) -> Option<Vec<(jobs::AttributeType, u8)>> {
        Nfts::get_pet_attributes(pet_id)
    }
}

// Implement the NftManagerForDaycare trait for the NFTs pallet
impl daycare::NftManagerForDaycare<u64, u32> for Nfts {
    fn is_owner(account: &u64, pet_id: &u32) -> bool {
        Nfts::is_owner(account, pet_id)
    }

    fn update_pet_state(pet_id: &u32) -> DispatchResult {
        Nfts::update_pet_state(pet_id)
    }

    fn get_pet_owner(pet_id: &u32) -> Option<u64> {
        Nfts::get_pet_owner(pet_id)
    }
}

// Implement the PetStatusManager trait for the pet status pallet
impl daycare::PetStatusManager<u32> for PetStatus {
    fn feed_pet(pet_id: &u32) -> DispatchResult {
        PetStatus::feed_pet(pet_id)
    }

    fn rest_pet(pet_id: &u32) -> DispatchResult {
        PetStatus::rest_pet(pet_id)
    }

    fn play_with_pet(pet_id: &u32) -> DispatchResult {
        PetStatus::play_with_pet(pet_id)
    }

    fn groom_pet(pet_id: &u32) -> DispatchResult {
        PetStatus::groom_pet(pet_id)
    }

    fn socialize_pet(pet_id: &u32, target_pet_id: &u32) -> DispatchResult {
        PetStatus::socialize_pet(pet_id, target_pet_id)
    }
}

// Define the runtime
pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<u32, RuntimeCall, Signature, ()>;
pub type Signature = sp_runtime::MultiSignature;
pub type BlockNumber = u64;
pub type Balance = u128;

// Define the runtime call enum
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RuntimeCall {
    System(frame_system::Call<Runtime>),
    Balances(pallet_balances::Call<Runtime>),
    Profiles(profiles::Call<Runtime>),
    Nfts(nfts::Call<Runtime>),
    PetStatus(pet_status::Call<Runtime>),
    Minigames(minigames::Call<Runtime>),
    Jobs(jobs::Call<Runtime>),
    Daycare(daycare::Call<Runtime>),
    // Add new pallets
    Governance(governance::Call<Runtime>),
    NodeRewards(node_rewards::Call<Runtime>),
    Treasury(treasury::Call<Runtime>),
    Battle(battle::Call<Runtime>),
}

// Define the runtime event enum
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RuntimeEvent {
    System(frame_system::Event<Runtime>),
    Balances(pallet_balances::Event<Runtime>),
    Profiles(profiles::Event<Runtime>),
    Nfts(nfts::Event<Runtime>),
    PetStatus(pet_status::Event<Runtime>),
    Minigames(minigames::Event<Runtime>),
    Jobs(jobs::Event<Runtime>),
    Daycare(daycare::Event<Runtime>),
    // Add new pallets
    Governance(governance::Event<Runtime>),
    NodeRewards(node_rewards::Event<Runtime>),
    Treasury(treasury::Event<Runtime>),
    Battle(battle::Event<Runtime>),
}

// Define the runtime origin enum
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RuntimeOrigin {
    System(frame_system::Origin<Runtime>),
    Signed(u64),
    None,
}

// Define the pallets in the runtime
pub struct System;
pub struct Balances;
pub struct Profiles;
pub struct Nfts;
pub struct PetStatus;
pub struct Minigames;
pub struct Jobs;
pub struct Daycare;
pub struct Governance;
pub struct NodeRewards;
pub struct Treasury;
pub struct Battle;
pub struct RandomnessCollectiveFlip;
pub struct PalletInfo;
pub struct ConstU16<const N: u16>;

// Define the dispatch result type
pub type DispatchResult = Result<(), &'static str>;

// Define the genesis configuration for the runtime
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    pallet_balances::GenesisConfig::<Runtime> {
        balances: vec![(1, 10_000_000), (2, 10_000_000), (3, 10_000_000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}

// Define a test function to demonstrate the integration
pub fn test_integration() {
    new_test_ext().execute_with(|| {
        // Create a user profile
        let alice = 1;
        let username = b"alice".to_vec();
        let bio = b"CritterCraft enthusiast".to_vec();
        let avatar_uri = b"https://example.com/avatar.png".to_vec();
        Profiles::create_profile(RuntimeOrigin::Signed(alice), username, bio, avatar_uri).unwrap();

        // Mint a pet NFT
        let pet_name = b"Fluffy".to_vec();
        let pet_description = b"A cute fluffy pet".to_vec();
        let pet_type = nfts::PetType::Aquatic;
        Nfts::mint(RuntimeOrigin::Signed(alice), pet_name, pet_description, pet_type).unwrap();

        // Initialize the pet's status
        let pet_id = 0;
        PetStatus::initialize_pet_status(RuntimeOrigin::Signed(alice), pet_id).unwrap();

        // Feed the pet
        PetStatus::feed_pet(RuntimeOrigin::Signed(alice), pet_id).unwrap();

        // Start a mini-game
        let game_type = minigames::GameType::LogicLeaper;
        let difficulty = minigames::DifficultyLevel::Easy;
        Minigames::start_game(RuntimeOrigin::Signed(alice), pet_id, game_type, difficulty).unwrap();

        // Submit a score for the mini-game
        let game_id = 0;
        let score = 1000;
        Minigames::submit_score(RuntimeOrigin::Signed(alice), game_id, score).unwrap();

        // Start a job
        let job_type = jobs::JobType::CrystalMining;
        let duration = 500;
        Jobs::start_job(RuntimeOrigin::Signed(alice), pet_id, job_type, duration).unwrap();

        // Complete the job
        let job_id = 0;
        Jobs::complete_job(RuntimeOrigin::Signed(alice), job_id).unwrap();

        // Create a daycare
        let daycare_name = b"Alice's Daycare".to_vec();
        let daycare_description = b"A cozy place for your pets".to_vec();
        let fee_per_block = 1;
        Daycare::create_daycare(RuntimeOrigin::Signed(alice), daycare_name, daycare_description, fee_per_block).unwrap();

        // Create a listing
        let daycare_id = 0;
        let listing_duration = 1000;
        Daycare::create_listing(RuntimeOrigin::Signed(alice), daycare_id, pet_id, listing_duration).unwrap();

        // Accept the listing as a caregiver
        let bob = 2;
        let listing_id = 0;
        Daycare::accept_listing(RuntimeOrigin::Signed(bob), listing_id).unwrap();

        // Perform a care action
        let action = daycare::CareAction::Feed;
        Daycare::perform_care_action(RuntimeOrigin::Signed(bob), listing_id, action, None).unwrap();

        // Complete the listing
        Daycare::complete_listing(RuntimeOrigin::Signed(alice), listing_id).unwrap();
    });
}