//! # CritterCraft Node Rewards Pallet
//!
//! A pallet that monitors node activities and distributes rewards to validators and nominators.
//!
//! ## Overview
//!
//! The node rewards pallet provides the following features:
//! - Tracking of node uptime and performance
//! - Distribution of rewards based on node activity
//! - Penalties for misbehavior
//! - Reporting of node metrics
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `report_offline` - Report a validator as offline
//! * `claim_rewards` - Claim accumulated rewards
//! * `set_reward_parameters` - Update reward distribution parameters
//! * `register_node` - Register a node for reward tracking
//! * `unregister_node` - Unregister a node from reward tracking
//! * `report_metrics` - Report node performance metrics

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        pallet_prelude::*,
        traits::{Currency, Get, ReservableCurrency, ExistenceRequirement},
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

        /// The currency used for rewards
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The period (in blocks) for reward distribution
        #[pallet::constant]
        type RewardPeriod: Get<Self::BlockNumber>;

        /// The minimum amount of tokens required to register a node
        #[pallet::constant]
        type NodeBond: Get<BalanceOf<Self>>;

        /// The maximum number of metrics that can be reported per period
        #[pallet::constant]
        type MaxMetricsPerPeriod: Get<u32>;

        /// The maximum number of offline reports per validator per period
        #[pallet::constant]
        type MaxOfflineReportsPerPeriod: Get<u32>;

        /// The origin that can update reward parameters
        type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    // Define the pallet's events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A node has been registered. [node_id, operator]
        NodeRegistered(T::AccountId, T::AccountId),
        /// A node has been unregistered. [node_id, operator]
        NodeUnregistered(T::AccountId, T::AccountId),
        /// A validator has been reported offline. [validator, reporter]
        ValidatorReportedOffline(T::AccountId, T::AccountId),
        /// Rewards have been claimed. [account, amount]
        RewardsClaimed(T::AccountId, BalanceOf<T>),
        /// Reward parameters have been updated.
        RewardParametersUpdated,
        /// Node metrics have been reported. [node_id, period]
        NodeMetricsReported(T::AccountId, T::BlockNumber),
        /// Rewards have been distributed for a period. [period, total_amount]
        RewardsDistributed(T::BlockNumber, BalanceOf<T>),
    }

    // Define the pallet's errors
    #[pallet::error]
    pub enum Error<T> {
        /// The node is already registered
        NodeAlreadyRegistered,
        /// The node is not registered
        NodeNotRegistered,
        /// The account has insufficient balance
        InsufficientBalance,
        /// The validator is not in the active set
        NotActiveValidator,
        /// The validator has already been reported offline in this period
        AlreadyReportedOffline,
        /// Too many offline reports in this period
        TooManyOfflineReports,
        /// No rewards available to claim
        NoRewardsToClaim,
        /// Too many metrics reported in this period
        TooManyMetricsReported,
        /// Invalid reward parameters
        InvalidRewardParameters,
        /// The account is not authorized to operate this node
        NotAuthorized,
    }

    // Define the pallet's storage items
    #[pallet::storage]
    #[pallet::getter(fn registered_nodes)]
    pub type RegisteredNodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId, // Node ID
        NodeInfo<T::AccountId, T::BlockNumber>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn node_metrics)]
    pub type NodeMetrics<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // Node ID
        Blake2_128Concat,
        T::BlockNumber, // Period
        NodeMetricsData,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn offline_reports)]
    pub type OfflineReports<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::BlockNumber, // Period
        Blake2_128Concat,
        T::AccountId, // Validator
        Vec<T::AccountId>, // Reporters
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pending_rewards)]
    pub type PendingRewards<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId, // Account
        BalanceOf<T>, // Reward amount
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn current_period)]
    pub type CurrentPeriod<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn reward_parameters)]
    pub type RewardParameters<T: Config> = StorageValue<_, RewardParams<BalanceOf<T>>, OptionQuery>;

    // Define the pallet itself
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Define the pallet's call (dispatchable functions)
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a node for reward tracking
        #[pallet::weight(T::WeightInfo::register_node())]
        pub fn register_node(
            origin: OriginFor<T>,
            node_id: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let operator = ensure_signed(origin)?;
            
            // Ensure the node is not already registered
            ensure!(
                !<RegisteredNodes<T>>::contains_key(&node_id),
                Error::<T>::NodeAlreadyRegistered
            );
            
            // Reserve the node bond
            let node_bond = T::NodeBond::get();
            T::Currency::reserve(&operator, node_bond)?;
            
            // Register the node
            let now = <frame_system::Pallet<T>>::block_number();
            let node_info = NodeInfo {
                operator: operator.clone(),
                registered_at: now,
                last_active: now,
                total_uptime: Zero::zero(),
                total_blocks_produced: 0,
                total_rewards_earned: Zero::zero(),
                status: NodeStatus::Active,
            };
            
            <RegisteredNodes<T>>::insert(&node_id, node_info);
            
            Self::deposit_event(Event::NodeRegistered(node_id, operator));
            
            Ok(().into())
        }
        
        /// Unregister a node from reward tracking
        #[pallet::weight(T::WeightInfo::unregister_node())]
        pub fn unregister_node(
            origin: OriginFor<T>,
            node_id: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let operator = ensure_signed(origin)?;
            
            // Ensure the node is registered
            let node_info = Self::registered_nodes(&node_id).ok_or(Error::<T>::NodeNotRegistered)?;
            
            // Ensure the caller is the operator
            ensure!(
                node_info.operator == operator,
                Error::<T>::NotAuthorized
            );
            
            // Unreserve the node bond
            let node_bond = T::NodeBond::get();
            T::Currency::unreserve(&operator, node_bond);
            
            // Unregister the node
            <RegisteredNodes<T>>::remove(&node_id);
            
            Self::deposit_event(Event::NodeUnregistered(node_id, operator));
            
            Ok(().into())
        }
        
        /// Report a validator as offline
        #[pallet::weight(T::WeightInfo::report_offline())]
        pub fn report_offline(
            origin: OriginFor<T>,
            validator: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let reporter = ensure_signed(origin)?;
            
            // Ensure the validator is registered
            ensure!(
                <RegisteredNodes<T>>::contains_key(&validator),
                Error::<T>::NodeNotRegistered
            );
            
            // Get the current period
            let current_period = Self::current_period();
            
            // Ensure the validator hasn't been reported too many times
            let reports = <OfflineReports<T>>::get(current_period, &validator);
            ensure!(
                reports.len() < T::MaxOfflineReportsPerPeriod::get() as usize,
                Error::<T>::TooManyOfflineReports
            );
            
            // Ensure the reporter hasn't already reported this validator
            ensure!(
                !reports.contains(&reporter),
                Error::<T>::AlreadyReportedOffline
            );
            
            // Record the offline report
            <OfflineReports<T>>::mutate(current_period, &validator, |reporters| {
                reporters.push(reporter.clone());
            });
            
            Self::deposit_event(Event::ValidatorReportedOffline(validator, reporter));
            
            Ok(().into())
        }
        
        /// Claim accumulated rewards
        #[pallet::weight(T::WeightInfo::claim_rewards())]
        pub fn claim_rewards(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            let claimer = ensure_signed(origin)?;
            
            // Get the pending rewards
            let rewards = <PendingRewards<T>>::get(&claimer);
            ensure!(!rewards.is_zero(), Error::<T>::NoRewardsToClaim);
            
            // Transfer the rewards
            T::Currency::transfer(
                &Self::account_id(),
                &claimer,
                rewards,
                ExistenceRequirement::KeepAlive,
            )?;
            
            // Clear the pending rewards
            <PendingRewards<T>>::remove(&claimer);
            
            Self::deposit_event(Event::RewardsClaimed(claimer, rewards));
            
            Ok(().into())
        }
        
        /// Report node performance metrics
        #[pallet::weight(T::WeightInfo::report_metrics())]
        pub fn report_metrics(
            origin: OriginFor<T>,
            node_id: T::AccountId,
            uptime: u32,
            blocks_produced: u32,
            response_time: u32,
        ) -> DispatchResultWithPostInfo {
            let reporter = ensure_signed(origin)?;
            
            // Ensure the node is registered
            let mut node_info = Self::registered_nodes(&node_id).ok_or(Error::<T>::NodeNotRegistered)?;
            
            // Ensure the reporter is the operator
            ensure!(
                node_info.operator == reporter,
                Error::<T>::NotAuthorized
            );
            
            // Get the current period
            let current_period = Self::current_period();
            
            // Ensure not too many metrics have been reported
            let metrics_count = <NodeMetrics<T>>::iter_prefix_values(&node_id).count();
            ensure!(
                metrics_count < T::MaxMetricsPerPeriod::get() as usize,
                Error::<T>::TooManyMetricsReported
            );
            
            // Record the metrics
            let metrics = NodeMetricsData {
                uptime,
                blocks_produced,
                response_time,
                reported_at: <frame_system::Pallet<T>>::block_number(),
            };
            
            <NodeMetrics<T>>::insert(&node_id, current_period, metrics);
            
            // Update node info
            node_info.last_active = <frame_system::Pallet<T>>::block_number();
            node_info.total_uptime = node_info.total_uptime.saturating_add(uptime.into());
            node_info.total_blocks_produced = node_info.total_blocks_produced.saturating_add(blocks_produced);
            <RegisteredNodes<T>>::insert(&node_id, node_info);
            
            Self::deposit_event(Event::NodeMetricsReported(node_id, current_period));
            
            Ok(().into())
        }
        
        /// Set reward parameters
        #[pallet::weight(T::WeightInfo::set_reward_parameters())]
        pub fn set_reward_parameters(
            origin: OriginFor<T>,
            params: RewardParams<BalanceOf<T>>,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            
            // Validate parameters
            ensure!(
                params.base_reward_rate <= params.max_reward_rate,
                Error::<T>::InvalidRewardParameters
            );
            
            // Update parameters
            <RewardParameters<T>>::put(params);
            
            Self::deposit_event(Event::RewardParametersUpdated);
            
            Ok(().into())
        }
        
        /// Distribute rewards for the current period
        #[pallet::weight(T::WeightInfo::distribute_rewards())]
        pub fn distribute_rewards(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            
            // Get the current period
            let current_period = Self::current_period();
            
            // Calculate and distribute rewards
            let total_rewards = Self::do_distribute_rewards(current_period);
            
            // Update the current period
            let next_period = current_period.saturating_add(T::RewardPeriod::get());
            <CurrentPeriod<T>>::put(next_period);
            
            Self::deposit_event(Event::RewardsDistributed(current_period, total_rewards));
            
            Ok(().into())
        }
    }

    // Define hooks for the pallet
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Called at the beginning of a new block
        fn on_initialize(n: T::BlockNumber) -> Weight {
            // Check if it's time to distribute rewards
            if (n % T::RewardPeriod::get()).is_zero() {
                // Update the current period
                let current_period = Self::current_period();
                let next_period = current_period.saturating_add(T::RewardPeriod::get());
                <CurrentPeriod<T>>::put(next_period);
                
                // Distribute rewards
                let _ = Self::do_distribute_rewards(current_period);
                
                // Return weight consumed
                T::WeightInfo::distribute_rewards()
            } else {
                Weight::zero()
            }
        }
    }

    // Define the genesis configuration for the pallet
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub reward_parameters: RewardParams<BalanceOf<T>>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                reward_parameters: RewardParams {
                    base_reward_rate: BalanceOf::<T>::from(1000u32),
                    max_reward_rate: BalanceOf::<T>::from(10000u32),
                    offline_penalty_rate: Perbill::from_percent(10),
                    uptime_weight: Perbill::from_percent(40),
                    blocks_weight: Perbill::from_percent(40),
                    response_time_weight: Perbill::from_percent(20),
                },
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <RewardParameters<T>>::put(&self.reward_parameters);
            <CurrentPeriod<T>>::put(T::BlockNumber::from(1u32));
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Calculate and distribute rewards for a period
        fn do_distribute_rewards(period: T::BlockNumber) -> BalanceOf<T> {
            let params = match <RewardParameters<T>>::get() {
                Some(p) => p,
                None => return Zero::zero(),
            };
            
            let mut total_rewards = Zero::zero();
            
            // Iterate through all registered nodes
            for (node_id, mut node_info) in <RegisteredNodes<T>>::iter() {
                // Skip inactive nodes
                if node_info.status != NodeStatus::Active {
                    continue;
                }
                
                // Get the node's metrics for this period
                let metrics = match <NodeMetrics<T>>::get(&node_id, period) {
                    Some(m) => m,
                    None => continue,
                };
                
                // Check for offline reports
                let offline_reports = <OfflineReports<T>>::get(period, &node_id);
                let offline_penalty = if !offline_reports.is_empty() {
                    params.offline_penalty_rate
                } else {
                    Perbill::zero()
                };
                
                // Calculate the node's reward
                let uptime_score = Perbill::from_rational(metrics.uptime, 100u32);
                let blocks_score = Perbill::from_rational(metrics.blocks_produced, 100u32);
                let response_score = Perbill::from_rational(100u32.saturating_sub(metrics.response_time), 100u32);
                
                let performance_score = 
                    params.uptime_weight.mul_floor(uptime_score) +
                    params.blocks_weight.mul_floor(blocks_score) +
                    params.response_time_weight.mul_floor(response_score);
                
                let base_reward = params.base_reward_rate;
                let reward = performance_score.mul_floor(base_reward);
                let final_reward = (Perbill::from_percent(100) - offline_penalty).mul_floor(reward);
                
                // Update the node's total rewards
                node_info.total_rewards_earned = node_info.total_rewards_earned.saturating_add(final_reward);
                <RegisteredNodes<T>>::insert(&node_id, node_info);
                
                // Add to the operator's pending rewards
                <PendingRewards<T>>::mutate(&node_info.operator, |r| {
                    *r = r.saturating_add(final_reward);
                });
                
                total_rewards = total_rewards.saturating_add(final_reward);
            }
            
            // Clear metrics and reports for this period
            <NodeMetrics<T>>::remove_prefix(&period, None);
            <OfflineReports<T>>::remove_prefix(&period, None);
            
            total_rewards
        }
        
        /// Get the account ID for the pallet
        pub fn account_id() -> T::AccountId {
            T::AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
                .expect("infinite length input; no invalid inputs for type; qed")
        }
    }

    // Define the weight information trait
    pub trait WeightInfo {
        fn register_node() -> Weight;
        fn unregister_node() -> Weight;
        fn report_offline() -> Weight;
        fn claim_rewards() -> Weight;
        fn report_metrics() -> Weight;
        fn set_reward_parameters() -> Weight;
        fn distribute_rewards() -> Weight;
    }

    // Define the balance type
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Define the node status enum
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum NodeStatus {
        Active,
        Inactive,
        Slashed,
    }

    // Define the node info struct
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct NodeInfo<AccountId, BlockNumber> {
        pub operator: AccountId,
        pub registered_at: BlockNumber,
        pub last_active: BlockNumber,
        pub total_uptime: u128,
        pub total_blocks_produced: u32,
        pub total_rewards_earned: u128,
        pub status: NodeStatus,
    }

    // Define the node metrics data struct
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct NodeMetricsData {
        pub uptime: u32, // 0-100 percentage
        pub blocks_produced: u32,
        pub response_time: u32, // in milliseconds
        pub reported_at: u32,
    }

    // Define the reward parameters struct
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct RewardParams<Balance> {
        pub base_reward_rate: Balance,
        pub max_reward_rate: Balance,
        pub offline_penalty_rate: Perbill,
        pub uptime_weight: Perbill,
        pub blocks_weight: Perbill,
        pub response_time_weight: Perbill,
    }
}