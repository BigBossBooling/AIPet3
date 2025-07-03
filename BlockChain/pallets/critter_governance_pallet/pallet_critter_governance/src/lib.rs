//! # CritterCraft Governance Pallet
//!
//! A pallet that provides decentralized governance functionality for the CritterCraft ecosystem.
//! It allows token holders to propose, vote on, and implement changes to the blockchain.
//!
//! ## Overview
//!
//! The governance pallet provides the following features:
//! - Proposal creation and management
//! - Voting mechanisms (token-weighted voting)
//! - Proposal execution
//! - Delegation of voting power
//! - Council elections
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `propose` - Create a new proposal
//! * `second` - Second (endorse) an existing proposal
//! * `vote` - Vote on a proposal
//! * `close_vote` - Close a vote and tally results
//! * `execute` - Execute an approved proposal
//! * `delegate` - Delegate voting power to another account
//! * `undelegate` - Remove delegation of voting power
//! * `nominate_council` - Nominate an account for council election
//! * `vote_council` - Vote in council elections
//! * `set_governance_params` - Update governance parameters (restricted to governance)

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        pallet_prelude::*,
        traits::{Currency, Get, LockIdentifier, LockableCurrency, ReservableCurrency, WithdrawReasons},
        weights::Weight,
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::{
        traits::{AccountIdConversion, CheckedAdd, CheckedSub, Hash, Zero},
        Perbill, Permill,
    };
    use sp_std::{prelude::*, vec::Vec};

    // Define the governance lock identifier
    const GOVERNANCE_LOCK_ID: LockIdentifier = *b"govnance";

    // Define the pallet's configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency used for staking and voting
        type Currency: ReservableCurrency<Self::AccountId> + LockableCurrency<Self::AccountId>;

        /// The period (in blocks) between council elections
        #[pallet::constant]
        type CouncilElectionPeriod: Get<Self::BlockNumber>;

        /// The minimum amount of tokens required to create a proposal
        #[pallet::constant]
        type ProposalBond: Get<BalanceOf<Self>>;

        /// The minimum amount of tokens required to vote
        #[pallet::constant]
        type VotingBond: Get<BalanceOf<Self>>;

        /// The minimum voting period for proposals (in blocks)
        #[pallet::constant]
        type MinVotingPeriod: Get<Self::BlockNumber>;

        /// The maximum voting period for proposals (in blocks)
        #[pallet::constant]
        type MaxVotingPeriod: Get<Self::BlockNumber>;

        /// The number of council members to elect
        #[pallet::constant]
        type CouncilSize: Get<u32>;

        /// The maximum weight of a proposal
        #[pallet::constant]
        type MaxProposalWeight: Get<Weight>;

        /// The maximum size of a proposal in bytes
        #[pallet::constant]
        type MaxProposalSize: Get<u32>;

        /// The origin that can cancel proposals
        type CancelOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// The origin that can fast-track proposals
        type FastTrackOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// The origin that can update governance parameters
        type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    // Define the pallet's events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new proposal has been created. [proposal_index, proposer, proposal_hash]
        ProposalCreated(ProposalIndex, T::AccountId, T::Hash),
        /// A proposal has been seconded. [proposal_index, seconder]
        ProposalSeconded(ProposalIndex, T::AccountId),
        /// A vote has been cast. [voter, proposal_index, vote]
        Voted(T::AccountId, ProposalIndex, Vote),
        /// A proposal has been approved. [proposal_index]
        ProposalApproved(ProposalIndex),
        /// A proposal has been rejected. [proposal_index]
        ProposalRejected(ProposalIndex),
        /// A proposal has been executed. [proposal_index, result]
        ProposalExecuted(ProposalIndex, DispatchResult),
        /// A proposal has been cancelled. [proposal_index]
        ProposalCancelled(ProposalIndex),
        /// Voting has been delegated. [delegator, delegate]
        VotingDelegated(T::AccountId, T::AccountId),
        /// Voting delegation has been removed. [delegator]
        VotingUndelegated(T::AccountId),
        /// A council member has been nominated. [candidate, nominator]
        CouncilNominated(T::AccountId, T::AccountId),
        /// A vote has been cast in council elections. [voter, candidate, vote_amount]
        CouncilVoteCast(T::AccountId, T::AccountId, BalanceOf<T>),
        /// New council members have been elected. [council_members]
        CouncilElected(Vec<T::AccountId>),
        /// Governance parameters have been updated.
        GovernanceParamsUpdated,
    }

    // Define the pallet's errors
    #[pallet::error]
    pub enum Error<T> {
        /// The proposal does not exist
        ProposalMissing,
        /// The proposal is already completed
        ProposalAlreadyComplete,
        /// The account is not allowed to vote
        NotAllowedToVote,
        /// The account has already voted
        AlreadyVoted,
        /// The proposal is not in the voting period
        NotVotingPeriod,
        /// The proposal cannot be executed yet
        CannotExecuteYet,
        /// The proposal has expired
        ProposalExpired,
        /// The account has insufficient balance
        InsufficientBalance,
        /// The proposal is too large
        ProposalTooLarge,
        /// The proposal weight is too high
        ProposalWeightTooHigh,
        /// The account is already delegating
        AlreadyDelegating,
        /// The delegation target is invalid
        InvalidDelegationTarget,
        /// The account is not delegating
        NotDelegating,
        /// The council candidate is already nominated
        AlreadyNominated,
        /// The council election is not active
        ElectionNotActive,
        /// The council election is already active
        ElectionAlreadyActive,
        /// The account is not a council member
        NotCouncilMember,
        /// The account is already a council member
        AlreadyCouncilMember,
        /// The governance parameters are invalid
        InvalidGovernanceParams,
    }

    // Define the pallet's storage items
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ProposalIndex,
        Proposal<T::AccountId, T::BlockNumber, T::Hash, BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn proposal_count)]
    pub type ProposalCount<T: Config> = StorageValue<_, ProposalIndex, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn voting_records)]
    pub type VotingRecords<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ProposalIndex,
        Blake2_128Concat,
        T::AccountId,
        Vote,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn delegations)]
    pub type Delegations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        T::AccountId,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn council_members)]
    pub type CouncilMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn council_candidates)]
    pub type CouncilCandidates<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn council_election_end)]
    pub type CouncilElectionEnd<T: Config> = StorageValue<_, T::BlockNumber, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn governance_params)]
    pub type GovernanceParams<T: Config> = StorageValue<_, GovernanceParameters<T::BlockNumber, BalanceOf<T>>, OptionQuery>;

    // Define the pallet itself
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Define the pallet's call (dispatchable functions)
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new proposal
        #[pallet::weight(T::WeightInfo::propose())]
        pub fn propose(
            origin: OriginFor<T>,
            proposal_hash: T::Hash,
            #[pallet::compact] voting_period: T::BlockNumber,
            description: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let proposer = ensure_signed(origin)?;
            
            // Ensure the voting period is within bounds
            ensure!(
                voting_period >= T::MinVotingPeriod::get() && voting_period <= T::MaxVotingPeriod::get(),
                Error::<T>::InvalidGovernanceParams
            );
            
            // Ensure the description is not too large
            ensure!(
                description.len() <= T::MaxProposalSize::get() as usize,
                Error::<T>::ProposalTooLarge
            );
            
            // Reserve the proposal bond
            let proposal_bond = T::ProposalBond::get();
            T::Currency::reserve(&proposer, proposal_bond)?;
            
            // Create the proposal
            let proposal_index = Self::next_proposal_index()?;
            let now = <frame_system::Pallet<T>>::block_number();
            let end = now.checked_add(&voting_period).ok_or(Error::<T>::InvalidGovernanceParams)?;
            
            let proposal = Proposal {
                proposer: proposer.clone(),
                hash: proposal_hash,
                description,
                votes_for: Zero::zero(),
                votes_against: Zero::zero(),
                status: ProposalStatus::Voting,
                created_at: now,
                voting_ends_at: end,
                executed_at: None,
            };
            
            <Proposals<T>>::insert(proposal_index, proposal);
            <ProposalCount<T>>::put(proposal_index + 1);
            
            Self::deposit_event(Event::ProposalCreated(proposal_index, proposer, proposal_hash));
            
            Ok(().into())
        }
        
        /// Second (endorse) an existing proposal
        #[pallet::weight(T::WeightInfo::second())]
        pub fn second(
            origin: OriginFor<T>,
            #[pallet::compact] proposal_index: ProposalIndex,
        ) -> DispatchResultWithPostInfo {
            let seconder = ensure_signed(origin)?;
            
            // Ensure the proposal exists and is in voting period
            let proposal = Self::proposals(proposal_index).ok_or(Error::<T>::ProposalMissing)?;
            ensure!(proposal.status == ProposalStatus::Voting, Error::<T>::NotVotingPeriod);
            
            // Ensure the current block is within the voting period
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(now <= proposal.voting_ends_at, Error::<T>::NotVotingPeriod);
            
            // Ensure the seconder hasn't already voted
            ensure!(
                !<VotingRecords<T>>::contains_key(proposal_index, &seconder),
                Error::<T>::AlreadyVoted
            );
            
            // Reserve the voting bond
            let voting_bond = T::VotingBond::get();
            T::Currency::reserve(&seconder, voting_bond)?;
            
            // Record the vote as a "second" (which is a vote in favor)
            let vote = Vote::Aye;
            <VotingRecords<T>>::insert(proposal_index, &seconder, vote);
            
            // Update the proposal's vote count
            let mut updated_proposal = proposal;
            updated_proposal.votes_for = updated_proposal.votes_for.saturating_add(voting_bond);
            <Proposals<T>>::insert(proposal_index, updated_proposal);
            
            Self::deposit_event(Event::ProposalSeconded(proposal_index, seconder));
            
            Ok(().into())
        }
        
        /// Vote on a proposal
        #[pallet::weight(T::WeightInfo::vote())]
        pub fn vote(
            origin: OriginFor<T>,
            #[pallet::compact] proposal_index: ProposalIndex,
            vote: Vote,
            #[pallet::compact] vote_amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let voter = ensure_signed(origin)?;
            
            // Ensure the proposal exists and is in voting period
            let proposal = Self::proposals(proposal_index).ok_or(Error::<T>::ProposalMissing)?;
            ensure!(proposal.status == ProposalStatus::Voting, Error::<T>::NotVotingPeriod);
            
            // Ensure the current block is within the voting period
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(now <= proposal.voting_ends_at, Error::<T>::NotVotingPeriod);
            
            // Ensure the voter hasn't already voted
            ensure!(
                !<VotingRecords<T>>::contains_key(proposal_index, &voter),
                Error::<T>::AlreadyVoted
            );
            
            // Ensure the voter has enough balance
            ensure!(
                T::Currency::free_balance(&voter) >= vote_amount,
                Error::<T>::InsufficientBalance
            );
            
            // Lock the voting amount
            T::Currency::set_lock(
                GOVERNANCE_LOCK_ID,
                &voter,
                vote_amount,
                WithdrawReasons::all(),
            );
            
            // Record the vote
            <VotingRecords<T>>::insert(proposal_index, &voter, vote);
            
            // Update the proposal's vote count
            let mut updated_proposal = proposal;
            match vote {
                Vote::Aye => {
                    updated_proposal.votes_for = updated_proposal.votes_for.saturating_add(vote_amount);
                }
                Vote::Nay => {
                    updated_proposal.votes_against = updated_proposal.votes_against.saturating_add(vote_amount);
                }
            }
            <Proposals<T>>::insert(proposal_index, updated_proposal);
            
            Self::deposit_event(Event::Voted(voter, proposal_index, vote));
            
            Ok(().into())
        }
        
        /// Close a vote and tally results
        #[pallet::weight(T::WeightInfo::close_vote())]
        pub fn close_vote(
            origin: OriginFor<T>,
            #[pallet::compact] proposal_index: ProposalIndex,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            
            // Ensure the proposal exists
            let proposal = Self::proposals(proposal_index).ok_or(Error::<T>::ProposalMissing)?;
            ensure!(proposal.status == ProposalStatus::Voting, Error::<T>::ProposalAlreadyComplete);
            
            // Ensure the voting period has ended
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(now > proposal.voting_ends_at, Error::<T>::NotVotingPeriod);
            
            // Determine if the proposal passed
            let mut updated_proposal = proposal;
            let passed = updated_proposal.votes_for > updated_proposal.votes_against;
            
            if passed {
                updated_proposal.status = ProposalStatus::Approved;
                Self::deposit_event(Event::ProposalApproved(proposal_index));
            } else {
                updated_proposal.status = ProposalStatus::Rejected;
                Self::deposit_event(Event::ProposalRejected(proposal_index));
                
                // Unreserve the proposer's bond
                T::Currency::unreserve(&updated_proposal.proposer, T::ProposalBond::get());
            }
            
            <Proposals<T>>::insert(proposal_index, updated_proposal);
            
            Ok(().into())
        }
        
        /// Execute an approved proposal
        #[pallet::weight(T::WeightInfo::execute())]
        pub fn execute(
            origin: OriginFor<T>,
            #[pallet::compact] proposal_index: ProposalIndex,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            
            // Ensure the proposal exists and is approved
            let proposal = Self::proposals(proposal_index).ok_or(Error::<T>::ProposalMissing)?;
            ensure!(proposal.status == ProposalStatus::Approved, Error::<T>::CannotExecuteYet);
            
            // Execute the proposal (in a real implementation, this would dispatch the call)
            // For this example, we'll just mark it as executed
            let mut updated_proposal = proposal;
            updated_proposal.status = ProposalStatus::Executed;
            updated_proposal.executed_at = Some(<frame_system::Pallet<T>>::block_number());
            
            <Proposals<T>>::insert(proposal_index, updated_proposal);
            
            // Unreserve the proposer's bond
            T::Currency::unreserve(&proposal.proposer, T::ProposalBond::get());
            
            // Unlock all votes for this proposal
            // In a real implementation, you would iterate through all voters
            
            Self::deposit_event(Event::ProposalExecuted(proposal_index, Ok(())));
            
            Ok(().into())
        }
        
        /// Delegate voting power to another account
        #[pallet::weight(T::WeightInfo::delegate())]
        pub fn delegate(
            origin: OriginFor<T>,
            delegate: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let delegator = ensure_signed(origin)?;
            
            // Ensure the delegator is not already delegating
            ensure!(
                !<Delegations<T>>::contains_key(&delegator),
                Error::<T>::AlreadyDelegating
            );
            
            // Ensure the delegate is not the delegator
            ensure!(delegator != delegate, Error::<T>::InvalidDelegationTarget);
            
            // Ensure the delegate is not delegating to someone else
            ensure!(
                !<Delegations<T>>::contains_key(&delegate),
                Error::<T>::InvalidDelegationTarget
            );
            
            // Record the delegation
            <Delegations<T>>::insert(&delegator, &delegate);
            
            Self::deposit_event(Event::VotingDelegated(delegator, delegate));
            
            Ok(().into())
        }
        
        /// Remove delegation of voting power
        #[pallet::weight(T::WeightInfo::undelegate())]
        pub fn undelegate(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            let delegator = ensure_signed(origin)?;
            
            // Ensure the delegator is delegating
            ensure!(
                <Delegations<T>>::contains_key(&delegator),
                Error::<T>::NotDelegating
            );
            
            // Remove the delegation
            <Delegations<T>>::remove(&delegator);
            
            Self::deposit_event(Event::VotingUndelegated(delegator));
            
            Ok(().into())
        }
        
        /// Nominate an account for council election
        #[pallet::weight(T::WeightInfo::nominate_council())]
        pub fn nominate_council(
            origin: OriginFor<T>,
            candidate: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let nominator = ensure_signed(origin)?;
            
            // Ensure council elections are active
            let election_end = Self::council_election_end().ok_or(Error::<T>::ElectionNotActive)?;
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(now < election_end, Error::<T>::ElectionNotActive);
            
            // Ensure the candidate is not already nominated
            ensure!(
                !<CouncilCandidates<T>>::contains_key(&candidate),
                Error::<T>::AlreadyNominated
            );
            
            // Reserve the nomination bond
            let nomination_bond = T::ProposalBond::get(); // Reuse proposal bond for simplicity
            T::Currency::reserve(&nominator, nomination_bond)?;
            
            // Record the nomination with zero votes
            <CouncilCandidates<T>>::insert(&candidate, Zero::zero());
            
            Self::deposit_event(Event::CouncilNominated(candidate, nominator));
            
            Ok(().into())
        }
        
        /// Vote in council elections
        #[pallet::weight(T::WeightInfo::vote_council())]
        pub fn vote_council(
            origin: OriginFor<T>,
            candidate: T::AccountId,
            #[pallet::compact] vote_amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let voter = ensure_signed(origin)?;
            
            // Ensure council elections are active
            let election_end = Self::council_election_end().ok_or(Error::<T>::ElectionNotActive)?;
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(now < election_end, Error::<T>::ElectionNotActive);
            
            // Ensure the candidate is nominated
            ensure!(
                <CouncilCandidates<T>>::contains_key(&candidate),
                Error::<T>::ProposalMissing
            );
            
            // Ensure the voter has enough balance
            ensure!(
                T::Currency::free_balance(&voter) >= vote_amount,
                Error::<T>::InsufficientBalance
            );
            
            // Lock the voting amount
            T::Currency::set_lock(
                GOVERNANCE_LOCK_ID,
                &voter,
                vote_amount,
                WithdrawReasons::all(),
            );
            
            // Update the candidate's vote count
            <CouncilCandidates<T>>::mutate(&candidate, |votes| {
                *votes = votes.saturating_add(vote_amount);
            });
            
            Self::deposit_event(Event::CouncilVoteCast(voter, candidate, vote_amount));
            
            Ok(().into())
        }
        
        /// Start a new council election
        #[pallet::weight(T::WeightInfo::start_council_election())]
        pub fn start_council_election(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            
            // Ensure no election is currently active
            ensure!(
                Self::council_election_end().is_none(),
                Error::<T>::ElectionAlreadyActive
            );
            
            // Set the election end block
            let now = <frame_system::Pallet<T>>::block_number();
            let election_period = T::CouncilElectionPeriod::get();
            let end = now.checked_add(&election_period).ok_or(Error::<T>::InvalidGovernanceParams)?;
            
            <CouncilElectionEnd<T>>::put(end);
            
            // Clear previous candidates
            <CouncilCandidates<T>>::remove_all(None);
            
            Ok(().into())
        }
        
        /// End the current council election and elect new council members
        #[pallet::weight(T::WeightInfo::end_council_election())]
        pub fn end_council_election(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            
            // Ensure an election is active and has ended
            let election_end = Self::council_election_end().ok_or(Error::<T>::ElectionNotActive)?;
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(now >= election_end, Error::<T>::ElectionNotActive);
            
            // Get all candidates and their vote counts
            let mut candidates: Vec<(T::AccountId, BalanceOf<T>)> = <CouncilCandidates<T>>::iter().collect();
            
            // Sort candidates by vote count (descending)
            candidates.sort_by(|a, b| b.1.cmp(&a.1));
            
            // Select the top candidates as council members
            let council_size = T::CouncilSize::get() as usize;
            let council_members: Vec<T::AccountId> = candidates
                .into_iter()
                .take(council_size)
                .map(|(account, _)| account)
                .collect();
            
            // Update council members
            <CouncilMembers<T>>::put(council_members.clone());
            
            // Clear the election end
            <CouncilElectionEnd<T>>::kill();
            
            Self::deposit_event(Event::CouncilElected(council_members));
            
            Ok(().into())
        }
        
        /// Update governance parameters
        #[pallet::weight(T::WeightInfo::set_governance_params())]
        pub fn set_governance_params(
            origin: OriginFor<T>,
            params: GovernanceParameters<T::BlockNumber, BalanceOf<T>>,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            
            // Validate parameters
            ensure!(
                params.min_voting_period <= params.max_voting_period,
                Error::<T>::InvalidGovernanceParams
            );
            
            // Update parameters
            <GovernanceParams<T>>::put(params);
            
            Self::deposit_event(Event::GovernanceParamsUpdated);
            
            Ok(().into())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Get the next proposal index
        fn next_proposal_index() -> Result<ProposalIndex, Error<T>> {
            let index = Self::proposal_count();
            let next_index = index.checked_add(1).ok_or(Error::<T>::ProposalMissing)?;
            Ok(index)
        }
    }

    // Define the weight information trait
    pub trait WeightInfo {
        fn propose() -> Weight;
        fn second() -> Weight;
        fn vote() -> Weight;
        fn close_vote() -> Weight;
        fn execute() -> Weight;
        fn delegate() -> Weight;
        fn undelegate() -> Weight;
        fn nominate_council() -> Weight;
        fn vote_council() -> Weight;
        fn start_council_election() -> Weight;
        fn end_council_election() -> Weight;
        fn set_governance_params() -> Weight;
    }

    // Define the proposal index type
    pub type ProposalIndex = u32;

    // Define the balance type
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Define the vote enum
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum Vote {
        Aye,
        Nay,
    }

    // Define the proposal status enum
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum ProposalStatus {
        Voting,
        Approved,
        Rejected,
        Executed,
        Cancelled,
    }

    // Define the proposal struct
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Proposal<AccountId, BlockNumber, Hash, Balance> {
        pub proposer: AccountId,
        pub hash: Hash,
        pub description: Vec<u8>,
        pub votes_for: Balance,
        pub votes_against: Balance,
        pub status: ProposalStatus,
        pub created_at: BlockNumber,
        pub voting_ends_at: BlockNumber,
        pub executed_at: Option<BlockNumber>,
    }

    // Define the governance parameters struct
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct GovernanceParameters<BlockNumber, Balance> {
        pub min_voting_period: BlockNumber,
        pub max_voting_period: BlockNumber,
        pub proposal_bond: Balance,
        pub voting_bond: Balance,
        pub approval_threshold: Permill,
        pub rejection_threshold: Permill,
    }
}
