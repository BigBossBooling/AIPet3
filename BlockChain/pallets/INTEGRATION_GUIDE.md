# CritterCraft Marketplace and Community Content Integration Guide

This guide explains how to integrate the enhanced marketplace and community content pallets into your CritterCraft Universe runtime.

## Overview

The integration involves:

1. Adding both pallets to your runtime
2. Configuring the pallets with appropriate parameters
3. Connecting the pallets to each other and to existing systems
4. Setting up the necessary types in your runtime

## Step 1: Add Dependencies to Cargo.toml

First, add the pallets to your runtime's `Cargo.toml`:

```toml
[dependencies]
# ... other dependencies

pallet-marketplace = { path = "../pallets/pallet-marketplace", default-features = false }
pallet-community-content = { path = "../pallets/pallet-community-content", default-features = false }

[features]
default = ["std"]
std = [
    # ... other std features
    "pallet-marketplace/std",
    "pallet-community-content/std",
]
```

## Step 2: Define Types in Runtime

Add the necessary types to your runtime's `lib.rs`:

```rust
// Define the ContentId type
pub type ContentId = u64;

// Define the ListingId type
pub type ListingId = u64;

// Define the EscrowId type
pub type EscrowId = u64;

// Define the ItemId type (if not already defined)
pub type ItemId = u64;
```

## Step 3: Configure and Implement the Pallets

Add the pallets to your runtime:

```rust
impl pallet_community_content::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type TimeProvider = Timestamp;
    type ContentId = ContentId;
    type ContentRandomness = RandomnessCollectiveFlip;
    type MaxNameLength = ConstU32<50>;
    type MaxDescriptionLength = ConstU32<1000>;
    type MaxUriLength = ConstU32<200>;
    type MaxReasonLength = ConstU32<500>;
    type ContentSubmissionDeposit = ConstU128<1_000_000_000>; // 1 CRT
    type MaxRoyaltyPercentage = ConstU8<15>; // 15%
    type CommunityTreasuryAccountId = TreasuryPalletId;
}

impl pallet_marketplace::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type TimeProvider = Timestamp;
    type PetId = PetId; // From your existing NFT system
    type ItemId = ItemId;
    type ContentId = ContentId;
    type ListingId = ListingId;
    type EscrowId = EscrowId;
    type NftHandler = PetNFT; // Your existing NFT pallet
    type ItemHandler = Items; // Your existing Items pallet
    type ContentRoyaltyHandler = CommunityContent;
    type MarketplaceFixedFee = ConstU128<500_000_000>; // 0.5 CRT
    type MarketplaceFeeRate = Percent::from_percent(2); // 2%
    type FeeDestinationAccountId = TreasuryPalletId;
    type MinAuctionDuration = ConstU32<3600>; // 1 hour in seconds
    type MaxAuctionDuration = ConstU32<2_592_000>; // 30 days in seconds
    type DefaultMinBidIncrement = Percent::from_percent(5); // 5%
    type EscrowTimeout = ConstU32<86400>; // 24 hours in seconds
    type ListingDeposit = ConstU128<100_000_000>; // 0.1 CRT
}

// Add the pallets to your construct_runtime! macro
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // ... existing pallets
        
        // Add the new pallets
        CommunityContent: pallet_community_content,
        Marketplace: pallet_marketplace,
    }
);
```

## Step 4: Implement Required Traits

Ensure your existing pallets implement the required traits:

### NFT Pallet

Your NFT pallet needs to implement the `NftManager` trait:

```rust
impl pallet_marketplace::NftManager<AccountId, PetId, DispatchResult> for PetNFT {
    fn owner_of(pet_id: &PetId) -> Option<AccountId> {
        // Return the owner of the pet
        PetNFT::owner(*pet_id)
    }
    
    fn is_transferable(pet_id: &PetId) -> bool {
        // Check if the pet can be transferred
        PetNFT::can_transfer(*pet_id)
    }
    
    fn lock_nft(owner: &AccountId, pet_id: &PetId) -> DispatchResult {
        // Lock the pet
        PetNFT::lock_pet(*owner, *pet_id)
    }
    
    fn unlock_nft(owner: &AccountId, pet_id: &PetId) -> DispatchResult {
        // Unlock the pet
        PetNFT::unlock_pet(*owner, *pet_id)
    }
    
    fn transfer_nft(from: &AccountId, to: &AccountId, pet_id: &PetId) -> DispatchResult {
        // Transfer the pet
        PetNFT::transfer_from(*from, *to, *pet_id)
    }
}
```

### Items Pallet

Your Items pallet needs to implement the `ItemManager` trait:

```rust
impl pallet_marketplace::ItemManager<AccountId, ItemId, DispatchResult> for Items {
    fn owner_of(item_id: &ItemId) -> Option<AccountId> {
        // Return the owner of the item
        Items::owner(*item_id)
    }
    
    fn is_transferable(item_id: &ItemId) -> bool {
        // Check if the item can be transferred
        Items::can_transfer(*item_id)
    }
    
    fn lock_item(owner: &AccountId, item_id: &ItemId) -> DispatchResult {
        // Lock the item
        Items::lock_item(*owner, *item_id)
    }
    
    fn unlock_item(owner: &AccountId, item_id: &ItemId) -> DispatchResult {
        // Unlock the item
        Items::unlock_item(*owner, *item_id)
    }
    
    fn transfer_item(from: &AccountId, to: &AccountId, item_id: &ItemId, quantity: u32) -> DispatchResult {
        // Transfer the item
        Items::transfer_from(*from, *to, *item_id, quantity)
    }
}
```

## Step 5: Initialize Genesis Configuration

Add genesis configuration for the pallets:

```rust
GenesisConfig {
    // ... other genesis configs
    
    community_content: CommunityContentConfig {
        // Add initial moderators
        moderators: vec![
            (AccountId::from([0; 32]), true),
        ],
    },
    
    marketplace: MarketplaceConfig {
        // Any initial configuration needed
    },
}
```

## Step 6: Update Your Frontend

Update your frontend to interact with the new pallets:

1. Add the new pallet types to your API
2. Create UI components for:
   - Content submission and management
   - Content browsing and purchasing
   - Marketplace listings (fixed price and auctions)
   - Escrow transactions
   - Moderation interface (for moderators)

## Integration Examples

### Example 1: User Creates and Sells Content

```rust
// 1. User submits content
CommunityContent::submit_content(
    origin,
    ContentType::CritterSkin,
    name,
    description,
    uri,
    content_hash,
    royalty_percentage
);

// 2. Moderator approves content
CommunityContent::moderate_content(
    moderator_origin,
    content_id,
    ContentStatus::Approved,
    None
);

// 3. Creator lists content for sale
Marketplace::list_content_fixed_price(
    origin,
    content_id,
    price,
    expiry
);

// 4. Buyer purchases content
Marketplace::buy_fixed_price(
    buyer_origin,
    listing_id
);
```

### Example 2: User Lists and Auctions a Pet

```rust
// 1. User lists pet for auction
Marketplace::list_pet_auction(
    origin,
    pet_id,
    start_price,
    duration,
    min_bid_increment
);

// 2. Other users place bids
Marketplace::place_bid(
    bidder_origin,
    listing_id,
    bid_amount
);

// 3. Auction ends automatically after duration
// The highest bidder wins and the pet is transferred
```

### Example 3: Secure Trading with Escrow

```rust
// 1. Buyer creates escrow
Marketplace::create_escrow(
    buyer_origin,
    seller,
    asset,
    price
);

// 2. Seller confirms escrow
Marketplace::confirm_escrow(
    seller_origin,
    escrow_id
);

// 3. Transaction completes automatically
// Funds are transferred to seller and asset to buyer
```

## Troubleshooting

### Common Issues

1. **Type Mismatches**: Ensure your type definitions match between pallets
2. **Missing Trait Implementations**: Verify all required traits are implemented
3. **Permission Issues**: Check that users have the correct permissions for actions
4. **Balance Issues**: Ensure users have sufficient balances for deposits and purchases

### Debugging Tips

1. Use the `debug` module to log information during runtime
2. Check events emitted by the pallets for transaction status
3. Verify storage values directly to confirm state changes
4. Test each component individually before integrating

## Conclusion

By following this guide, you should have successfully integrated the enhanced marketplace and community content systems into your CritterCraft Universe. These systems provide a robust foundation for user-generated content, trading, and community engagement.

For more detailed information, refer to the individual README files for each pallet.