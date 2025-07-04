# CritterCraft Marketplace and Community Content Usage Examples

This document provides practical examples of how to use the enhanced marketplace and community content features in the CritterCraft Universe.

## Community Content Examples

### Creating New Content

```rust
// Submit a new critter skin
let name = "Galaxy Skin".as_bytes().to_vec();
let description = "A cosmic-themed skin with animated stars and nebulae.".as_bytes().to_vec();
let uri = "ipfs://QmYourContentHash".as_bytes().to_vec();
let content_hash = [/* your content hash */];
let royalty_percentage = 10; // 10%

// Submit the content
CommunityContent::submit_content(
    origin,
    ContentType::CritterSkin,
    name,
    description,
    uri,
    content_hash,
    royalty_percentage
)?;
```

### Moderating Content

```rust
// Approve content
CommunityContent::moderate_content(
    moderator_origin,
    content_id,
    ContentStatus::Approved,
    None
)?;

// Reject content with reason
let reason = "Violates content guidelines".as_bytes().to_vec();
CommunityContent::moderate_content(
    moderator_origin,
    content_id,
    ContentStatus::Rejected,
    Some(reason)
)?;

// Flag content for further review
let reason = "Needs additional review".as_bytes().to_vec();
CommunityContent::moderate_content(
    moderator_origin,
    content_id,
    ContentStatus::Flagged,
    Some(reason)
)?;
```

### Updating Content

```rust
// Update content name and description
let new_name = "Enhanced Galaxy Skin".as_bytes().to_vec();
let new_description = "An improved cosmic-themed skin with animated stars, nebulae, and black holes.".as_bytes().to_vec();

CommunityContent::update_content(
    creator_origin,
    content_id,
    Some(new_name),
    Some(new_description),
    None,
    None
)?;
```

### Managing Moderators

```rust
// Add a new moderator (root only)
CommunityContent::add_moderator(
    root_origin,
    moderator_account
)?;

// Remove a moderator (root only)
CommunityContent::remove_moderator(
    root_origin,
    moderator_account
)?;
```

## Marketplace Examples

### Fixed Price Listings

#### Listing a Pet for Sale

```rust
// List a pet for a fixed price
let pet_id = 42;
let price = 1_000_000_000; // 1 CRT
let expiry = Some(now() + 604800); // 1 week from now

Marketplace::list_pet_fixed_price(
    owner_origin,
    pet_id,
    price,
    expiry
)?;
```

#### Listing Items for Sale

```rust
// List items for a fixed price
let item_id = 123;
let quantity = 5;
let price = 500_000_000; // 0.5 CRT per item
let expiry = None; // No expiry

Marketplace::list_items_fixed_price(
    owner_origin,
    item_id,
    quantity,
    price,
    expiry
)?;
```

#### Listing Content for Sale

```rust
// List community content for a fixed price
let content_id = 7;
let price = 250_000_000; // 0.25 CRT
let expiry = Some(now() + 2592000); // 30 days from now

Marketplace::list_content_fixed_price(
    creator_origin,
    content_id,
    price,
    expiry
)?;
```

#### Buying a Fixed Price Listing

```rust
// Buy a fixed price listing
Marketplace::buy_fixed_price(
    buyer_origin,
    listing_id
)?;
```

### Auction Listings

#### Creating an Auction for a Pet

```rust
// List a pet for auction
let pet_id = 42;
let start_price = 500_000_000; // 0.5 CRT
let duration = 86400; // 1 day in seconds
let min_bid_increment = Some(Perbill::from_percent(10)); // 10% minimum bid increment

Marketplace::list_pet_auction(
    owner_origin,
    pet_id,
    start_price,
    duration,
    min_bid_increment
)?;
```

#### Placing a Bid

```rust
// Place a bid on an auction
let listing_id = 5;
let bid_amount = 600_000_000; // 0.6 CRT

Marketplace::place_bid(
    bidder_origin,
    listing_id,
    bid_amount
)?;
```

### Canceling Listings

```rust
// Cancel a listing (only if you're the seller)
Marketplace::cancel_listing(
    seller_origin,
    listing_id
)?;
```

### Escrow Transactions

#### Creating an Escrow

```rust
// Create an escrow transaction for a pet
let seller = AccountId::from([/* seller account */]);
let asset = AssetType::Pet(pet_id);
let price = 1_500_000_000; // 1.5 CRT

Marketplace::create_escrow(
    buyer_origin,
    seller,
    asset,
    price
)?;
```

#### Confirming an Escrow

```rust
// Seller confirms the escrow
Marketplace::confirm_escrow(
    seller_origin,
    escrow_id
)?;
```

#### Canceling an Escrow

```rust
// Cancel an escrow transaction
Marketplace::cancel_escrow(
    buyer_origin,
    escrow_id
)?;
```

## Advanced Usage Examples

### Complete Content Creation and Sale Flow

```rust
// 1. Creator submits content
let content_id = 10; // Returned from submit_content

// 2. Moderator approves content
CommunityContent::moderate_content(
    moderator_origin,
    content_id,
    ContentStatus::Approved,
    None
)?;

// 3. Creator lists content for sale
let listing_id = 15; // Returned from list_content_fixed_price

// 4. Buyer purchases content
Marketplace::buy_fixed_price(
    buyer_origin,
    listing_id
)?;

// 5. Royalties are automatically paid when content is used
// This happens in the background when the content is used in-game
```

### Pet Auction with Multiple Bidders

```rust
// 1. Owner lists pet for auction
let listing_id = 20; // Returned from list_pet_auction

// 2. First bidder places a bid
Marketplace::place_bid(
    bidder1_origin,
    listing_id,
    600_000_000 // 0.6 CRT
)?;

// 3. Second bidder places a higher bid
Marketplace::place_bid(
    bidder2_origin,
    listing_id,
    700_000_000 // 0.7 CRT
)?;

// 4. First bidder places an even higher bid
Marketplace::place_bid(
    bidder1_origin,
    listing_id,
    800_000_000 // 0.8 CRT
)?;

// 5. Auction ends automatically after duration
// The highest bidder (bidder1) wins and the pet is transferred
```

### Secure Trading with Escrow and Confirmation

```rust
// 1. Buyer creates escrow for a rare item
let escrow_id = 8; // Returned from create_escrow

// 2. Seller confirms the escrow
Marketplace::confirm_escrow(
    seller_origin,
    escrow_id
)?;

// 3. Transaction completes automatically
// Funds are transferred to seller and item to buyer
```

## Frontend Integration Examples

### Displaying Content in the Marketplace

```javascript
// Fetch approved content
const approvedContent = await api.query.communityContent.approvedContent();

// Get details for each content item
const contentDetails = await Promise.all(
  approvedContent.map(id => api.query.communityContent.content(id))
);

// Display content in the UI
contentDetails.forEach(content => {
  // Create UI element for each content item
  const contentElement = createContentElement({
    name: content.name,
    description: await api.query.communityContent.contentDescriptions(content.id),
    uri: await api.query.communityContent.contentUris(content.id),
    creator: content.creator,
    royaltyPercentage: content.royaltyPercentage,
    // Add buy button if content is for sale
    buyButton: createBuyButton(content.id)
  });
  
  contentContainer.appendChild(contentElement);
});
```

### Creating a Bidding Interface

```javascript
// Display auction details
function displayAuction(listingId, auctionDetails) {
  const auctionElement = document.createElement('div');
  auctionElement.className = 'auction-listing';
  
  auctionElement.innerHTML = `
    <h3>Auction #${listingId}</h3>
    <p>Current Price: ${formatBalance(auctionDetails.currentPrice)}</p>
    <p>Current Winner: ${auctionDetails.currentWinner || 'No bids yet'}</p>
    <p>Ends: ${formatTime(auctionDetails.endTime)}</p>
    <p>Minimum Bid: ${formatBalance(calculateMinimumBid(auctionDetails))}</p>
    
    <div class="bid-form">
      <input type="number" id="bid-amount-${listingId}" placeholder="Bid Amount" />
      <button onclick="placeBid(${listingId})">Place Bid</button>
    </div>
  `;
  
  return auctionElement;
}

// Place a bid
async function placeBid(listingId) {
  const bidAmount = document.getElementById(`bid-amount-${listingId}`).value;
  
  try {
    const tx = api.tx.marketplace.placeBid(listingId, bidAmount);
    await tx.signAndSend(currentAccount);
    showNotification('Bid placed successfully!');
  } catch (error) {
    showError(`Failed to place bid: ${error.message}`);
  }
}
```

### Content Submission Form

```javascript
// Content submission form
function createSubmissionForm() {
  const form = document.createElement('form');
  form.innerHTML = `
    <h2>Submit New Content</h2>
    
    <div class="form-group">
      <label for="content-type">Content Type</label>
      <select id="content-type">
        <option value="0">Critter Skin</option>
        <option value="1">Accessory</option>
        <option value="2">Item Design</option>
        <option value="3">Environment Theme</option>
        <option value="4">Quest Template</option>
        <option value="5">Other</option>
      </select>
    </div>
    
    <div class="form-group">
      <label for="content-name">Name</label>
      <input type="text" id="content-name" maxlength="50" required />
    </div>
    
    <div class="form-group">
      <label for="content-description">Description</label>
      <textarea id="content-description" maxlength="1000" required></textarea>
    </div>
    
    <div class="form-group">
      <label for="content-uri">Content URI (IPFS)</label>
      <input type="text" id="content-uri" placeholder="ipfs://..." required />
    </div>
    
    <div class="form-group">
      <label for="content-hash">Content Hash</label>
      <input type="text" id="content-hash" required />
    </div>
    
    <div class="form-group">
      <label for="royalty-percentage">Royalty Percentage (0-15%)</label>
      <input type="number" id="royalty-percentage" min="0" max="15" value="10" required />
    </div>
    
    <button type="submit">Submit Content</button>
  `;
  
  form.addEventListener('submit', submitContent);
  return form;
}

// Submit content
async function submitContent(event) {
  event.preventDefault();
  
  const contentType = document.getElementById('content-type').value;
  const name = stringToU8a(document.getElementById('content-name').value);
  const description = stringToU8a(document.getElementById('content-description').value);
  const uri = stringToU8a(document.getElementById('content-uri').value);
  const contentHash = hexToU8a(document.getElementById('content-hash').value);
  const royaltyPercentage = document.getElementById('royalty-percentage').value;
  
  try {
    const tx = api.tx.communityContent.submitContent(
      contentType,
      name,
      description,
      uri,
      contentHash,
      royaltyPercentage
    );
    
    await tx.signAndSend(currentAccount);
    showNotification('Content submitted successfully!');
  } catch (error) {
    showError(`Failed to submit content: ${error.message}`);
  }
}
```

## Conclusion

These examples demonstrate the core functionality of the marketplace and community content systems. By combining these features, you can create a rich ecosystem for trading, content creation, and community engagement in the CritterCraft Universe.

For more detailed information, refer to the API documentation for each pallet.