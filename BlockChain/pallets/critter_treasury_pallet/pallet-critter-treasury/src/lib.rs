//! # CritterCraft Treasury Pallet
//!
//! A pallet that manages a treasury of funds for the CritterCraft ecosystem.
//!
//! ## Overview
//!
//! The treasury pallet provides the following features:
//! - Collection of transaction fees and other income
//! - Proposal system for spending treasury funds
//! - Automatic funding of ecosystem development
//! - Burning mechanism for excess funds
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `propose_spend` - Propose a treasury spend
//! * `approve_spend` - Approve a treasury spend proposal
//! * `reject_spend` - Reject a treasury spend proposal
//! * `execute_spend` - Execute an approved treasury spend
//! * `set_treasury_params` - Update treasury parameters

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        pallet_prelude::*,
        traits::{Currency, Get, ExistenceRequirement, ReservableCurrency, OnUnbalanced},
        weights::Weight,
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::{
        traits::{AccountIdConversion, CheckedAdd, CheckedSub, Zero, Saturating},
        Perbill, Permill,
    };
    use sp_std::{prelude::*, vec::Vec};

    // Define the pallet's configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency used for treasury
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The period (in blocks) for treasury operations
        #[pallet::constant]
        type TreasuryPeriod: Get<Self::BlockNumber>;

        /// The minimum amount of tokens required to create a spend proposal
        #[pallet::constant]
        type ProposalBond: Get<BalanceOf<Self>>;

        /// The minimum amount for a spend proposal
        #[pallet::constant]
        type MinSpend: Get<BalanceOf<Self>>;

        /// The maximum amount for a spend proposal
        #[pallet::constant]
        type MaxSpend: Get<BalanceOf<Self>>;

        /// The percentage of transaction fees that go to the treasury
        #[pallet::constant]
        type TreasuryFeePercent: Get<Perbill>;

        /// The percentage of treasury funds that are burned each period
        #[pallet::constant]
        type BurnPercent: Get<Perbill>;

        /// The origin that can approve treasury spends
        type ApproveOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// The origin that can reject treasury spends
        type RejectOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// The origin that can update treasury parameters
        type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Handler for unbalanced decreases when burning funds
        type OnSlash: OnUnbalanced<NegativeImbalanceOf<Self>>;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    // Define the pallet's events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new spend proposal has been created. [proposal_index, proposer, amount, beneficiary]
        SpendProposed(ProposalIndex, T::AccountId, BalanceOf<T>, T::AccountId),
        /// A spend proposal has been approved. [proposal_index]
        SpendApproved(ProposalIndex),
        /// A spend proposal has been rejected. [proposal_index]
        SpendRejected(ProposalIndex),
        /// A spend has been executed. [proposal_index, beneficiary, amount]
        SpendExecuted(ProposalIndex, T::AccountId, BalanceOf<T>),
        /// Funds have been deposited into the treasury. [amount]
        TreasuryDeposit(BalanceOf<T>),
        /// Funds have been burned from the treasury. [amount]
        TreasuryBurn(BalanceOf<T>),
        /// Treasury parameters have been updated.
        TreasuryParamsUpdated,
    }

    // Define the pallet's errors
    #[pallet::error]
    pub enum Error<T> {
        /// The proposal does not exist
        ProposalMissing,
        /// The proposal is already completed
        ProposalAlreadyComplete,
        /// The account has insufficient balance
        InsufficientBalance,
        /// The proposal amount is too small
        SpendTooSmall,
        /// The proposal amount is too large
        SpendTooLarge,
        /// The treasury has insufficient funds
        InsufficientTreasuryFunds,
        /// Invalid treasury parameters
        InvalidTreasuryParams,
    }

    // Define the pallet's storage items
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ProposalIndex,
        SpendProposal<T::AccountId, BalanceOf<T>, T::BlockNumber>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn proposal_count)]
    pub type ProposalCount<T: Config> = StorageValue<_, ProposalIndex, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn treasury_balance)]
    pub type TreasuryBalance<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_burn_block)]
    pub type NextBurnBlock<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn treasury_params)]
    pub type TreasuryParams<T: Config> = StorageValue<_, TreasuryParameters<BalanceOf<T>>, OptionQuery>;

    // Define the pallet itself
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Define the pallet's call (dispatchable functions)
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Propose a treasury spend
        #[pallet::weight(T::WeightInfo::propose_spend())]
        pub fn propose_spend(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
            beneficiary: T::AccountId,
            description: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let proposer = ensure_signed(origin)?;
            
            // Ensure the amount is within bounds
            ensure!(
                amount >= T::MinSpend::get(),
                Error::<T>::SpendTooSmall
            );
            ensure!(
                amount <= T::MaxSpend::get(),
                Error::<T>::SpendTooLarge
            );
            
            // Reserve the proposal bond
            let proposal_bond = T::ProposalBond::get();
            T::Currency::reserve(&proposer, proposal_bond)?;
            
            // Create the proposal
            let proposal_index = Self::next_proposal_index()?;
            let now = <frame_system::Pallet<T>>::block_number();
            
            let proposal = SpendProposal {
                proposer: proposer.clone(),
                amount,
                beneficiary: beneficiary.clone(),
                description,
                status: ProposalStatus::Pending,
                created_at: now,
                approved_at: None,
                executed_at: None,
            };
            
            <Proposals<T>>::insert(proposal_index, proposal);
            <ProposalCount<T>>::put(proposal_index + 1);
            
            Self::deposit_event(Event::SpendProposed(proposal_index, proposer, amount, beneficiary));
            
            Ok(().into())
        }
        
        /// Approve a treasury spend proposal
        #[pallet::weight(T::WeightInfo::approve_spend())]
        pub fn approve_spend(
            origin: OriginFor<T>,
            #[pallet::compact] proposal_index: ProposalIndex,
        ) -> DispatchResultWithPostInfo {
            T::ApproveOrigin::ensure_origin(origin)?;
            
            // Ensure the proposal exists and is pending
            let mut proposal = Self::proposals(proposal_index).ok_or(Error::<T>::ProposalMissing)?;
            ensure!(proposal.status == ProposalStatus::Pending, Error::<T>::ProposalAlreadyComplete);
            
            // Update the proposal status
            proposal.status = ProposalStatus::Approved;
            proposal.approved_at = Some(<frame_system::Pallet<T>>::block_number());
            <Proposals<T>>::insert(proposal_index, proposal);
            
            Self::deposit_event(Event::SpendApproved(proposal_index));
            
            Ok(().into())
        }
        
        /// Reject a treasury spend proposal
        #[pallet::weight(T::WeightInfo::reject_spend())]
        pub fn reject_spend(
            origin: OriginFor<T>,
            #[pallet::compact] proposal_index: ProposalIndex,
        ) -> DispatchResultWithPostInfo {
            T::RejectOrigin::ensure_origin(origin)?;
            
            // Ensure the proposal exists and is pending
            let mut proposal = Self::proposals(proposal_index).ok_or(Error::<T>::ProposalMissing)?;
            ensure!(proposal.status == ProposalStatus::Pending, Error::<T>::ProposalAlreadyComplete);
            
            // Update the proposal status
            proposal.status = ProposalStatus::Rejected;
            <Proposals<T>>::insert(proposal_index, proposal.clone());
            
            // Unreserve the proposer's bond
            T::Currency::unreserve(&proposal.proposer, T::ProposalBond::get());
            
            Self::deposit_event(Event::SpendRejected(proposal_index));
            
            Ok(().into())
        }
        
        /// Execute an approved treasury spend
        #[pallet::weight(T::WeightInfo::execute_spend())]
        pub fn execute_spend(
            origin: OriginFor<T>,
            #[pallet::compact] proposal_index: ProposalIndex,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            
            // Ensure the proposal exists and is approved
            let mut proposal = Self::proposals(proposal_index).ok_or(Error::<T>::ProposalMissing)?;
            ensure!(proposal.status == ProposalStatus::Approved, Error::<T>::ProposalAlreadyComplete);
            
            // Ensure the treasury has enough funds
            let treasury_balance = Self::treasury_balance();
            ensure!(
                treasury_balance >= proposal.amount,
                Error::<T>::InsufficientTreasuryFunds
            );
            
            // Update the treasury balance
            let new_balance = treasury_balance.saturating_sub(proposal.amount);
            <TreasuryBalance<T>>::put(new_balance);
            
            // Transfer the funds to the beneficiary
            T::Currency::transfer(
                &Self::account_id(),
                &proposal.beneficiary,
                proposal.amount,
                ExistenceRequirement::KeepAlive,
            )?;
            
            // Update the proposal status
            proposal.status = ProposalStatus::Executed;
            proposal.executed_at = Some(<frame_system::Pallet<T>>::block_number());
            <Proposals<T>>::insert(proposal_index, proposal.clone());
            
            // Unreserve the proposer's bond
            T::Currency::unreserve(&proposal.proposer, T::ProposalBond::get());
            
            Self::deposit_event(Event::SpendExecuted(proposal_index, proposal.beneficiary, proposal.amount));
            
            Ok(().into())
        }
        
        /// Set treasury parameters
        #[pallet::weight(T::WeightInfo::set_treasury_params())]
        pub fn set_treasury_params(
            origin: OriginFor<T>,
            params: TreasuryParameters<BalanceOf<T>>,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            
            // Validate parameters
            ensure!(
                params.min_spend <= params.max_spend,
                Error::<T>::InvalidTreasuryParams
            );
            
            // Update parameters
            <TreasuryParams<T>>::put(params);
            
            Self::deposit_event(Event::TreasuryParamsUpdated);
            
            Ok(().into())
        }
        
        /// Deposit funds into the treasury
        #[pallet::weight(T::WeightInfo::deposit_treasury())]
        pub fn deposit_treasury(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let depositor = ensure_signed(origin)?;
            
            // Transfer funds to the treasury
            T::Currency::transfer(
                &depositor,
                &Self::account_id(),
                amount,
                ExistenceRequirement::KeepAlive,
            )?;
            
            // Update the treasury balance
            <TreasuryBalance<T>>::mutate(|balance| {
                *balance = balance.saturating_add(amount);
            });
            
            Self::deposit_event(Event::TreasuryDeposit(amount));
            
            Ok(().into())
        }
    }

    // Define hooks for the pallet
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Called at the beginning of a new block
        fn on_initialize(n: T::BlockNumber) -> Weight {
            // Check if it's time to burn funds
            let next_burn = Self::next_burn_block();
            if n >= next_burn {
                // Burn a percentage of the treasury funds
                let treasury_balance = Self::treasury_balance();
                let burn_amount = T::BurnPercent::get().mul_floor(treasury_balance);
                
                if !burn_amount.is_zero() {
                    // Update the treasury balance
                    let new_balance = treasury_balance.saturating_sub(burn_amount);
                    <TreasuryBalance<T>>::put(new_balance);
                    
                    // Create a negative imbalance and send to the handler
                    let imbalance = T::Currency::slash_reserved(&Self::account_id(), burn_amount).0;
                    T::OnSlash::on_unbalanced(imbalance);
                    
                    Self::deposit_event(Event::TreasuryBurn(burn_amount));
                }
                
                // Set the next burn block
                let next_burn = n.saturating_add(T::TreasuryPeriod::get());
                <NextBurnBlock<T>>::put(next_burn);
                
                // Return weight consumed
                T::WeightInfo::burn_funds()
            } else {
                Weight::zero()
            }
        }
    }

    // Define the genesis configuration for the pallet
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub treasury_params: TreasuryParameters<BalanceOf<T>>,
        pub initial_balance: BalanceOf<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                treasury_params: TreasuryParameters {
                    min_spend: BalanceOf::<T>::from(100u32),
                    max_spend: BalanceOf::<T>::from(1_000_000u32),
                    proposal_bond: BalanceOf::<T>::from(1000u32),
                    burn_percent: Perbill::from_percent(1),
                },
                initial_balance: BalanceOf::<T>::from(1_000_000u32),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <TreasuryParams<T>>::put(&self.treasury_params);
            <TreasuryBalance<T>>::put(self.initial_balance);
            <NextBurnBlock<T>>::put(T::TreasuryPeriod::get());
            
            // Fund the treasury account with the initial balance
            let _ = T::Currency::make_free_balance_be(
                &Pallet::<T>::account_id(),
                self.initial_balance,
            );
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
        
        /// Get the account ID for the treasury
        pub fn account_id() -> T::AccountId {
            T::AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
                .expect("infinite length input; no invalid inputs for type; qed")
        }
    }

    // Define the weight information trait
    pub trait WeightInfo {
        fn propose_spend() -> Weight;
        fn approve_spend() -> Weight;
        fn reject_spend() -> Weight;
        fn execute_spend() -> Weight;
        fn set_treasury_params() -> Weight;
        fn deposit_treasury() -> Weight;
        fn burn_funds() -> Weight;
    }

    // Define the proposal index type
    pub type ProposalIndex = u32;

    // Define the balance type
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    
    // Define the negative imbalance type
    pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

    // Define the proposal status enum
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum ProposalStatus {
        Pending,
        Approved,
        Rejected,
        Executed,
    }

    // Define the spend proposal struct
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct SpendProposal<AccountId, Balance, BlockNumber> {
        pub proposer: AccountId,
        pub amount: Balance,
        pub beneficiary: AccountId,
        pub description: Vec<u8>,
        pub status: ProposalStatus,
        pub created_at: BlockNumber,
        pub approved_at: Option<BlockNumber>,
        pub executed_at: Option<BlockNumber>,
    }

    // Define the treasury parameters struct
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct TreasuryParameters<Balance> {
        pub min_spend: Balance,
        pub max_spend: Balance,
        pub proposal_bond: Balance,
        pub burn_percent: Perbill,
    }
}