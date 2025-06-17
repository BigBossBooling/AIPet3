# CritterCraft UI Wallet: Advanced Feature Interaction Logic (`app.js` Conceptual Detailing)

This document outlines the conceptual JavaScript logic within `app.js` for UI interactions with advanced features: Pet Breeding, Pet Day Cares, and User Shops. It details Polkadot.js API usage for querying storage and submitting extrinsics, building upon the general `submitExtrinsic` helper defined in `UI_INTERACTIONS_PROJECT2.md`.

**Assumptions:**
*   `window.critterApi` is initialized and connected to the CritterChain node.
*   `ALICE_ADDRESS` (or a dynamically selected user account via extension) is available for signing transactions.
*   The generic `submitExtrinsic(extrinsic, account, statusElementId, successCallback)` helper function exists.
*   Pallet names in transactions (e.g., `breedingPallet`, `daycarePallet`, `userShopsPallet`) match the actual runtime configurations.
*   HTML element IDs match those defined in `blockchain_core/ui-wallet/index.html`.
*   `ItemCategory` enum is available (e.g., from `pallet_items` metadata or constants).

## 1. Pet Breeding (`#breeding-center-section`)

### a. Populate Parent Selection Dropdowns (`#parent-a-select`, `#parent-b-select`)
*   **Function:** `async function populateBreedingParentSelects(api, accountAddress)`
*   **Logic:**
    1.  Fetch owned Pet NFTs for `accountAddress`:
        *   Query `api.query.critterNftsPallet.ownerOfPet(accountAddress)` to get an array of `PetId`s.
        *   For each `petId`, query `api.query.critterNftsPallet.petNfts(petId)` to get the `PetNft` struct.
    2.  Filter for eligible pets:
        *   Must be adult (e.g., `petNft.level >= T::MaturityLevel` from `pallet-breeding::Config` or `pallet-critter-nfts::Config`).
        *   Not currently in a breeding cooldown (e.g., `currentBlock < petNft.last_bred_block + T::BreedingCooldownDuration` from `pallet-breeding::Config`). This might require `last_bred_block` on `PetNft` or a dedicated storage in `pallet-breeding`.
    3.  Populate both `#parent-a-select` and `#parent-b-select` dropdowns with `<option value="petId">PetName (ID: petId, Lvl: X, Gen: Y)</option>`.
    4.  Add event listeners to these selects to call `displaySelectedParentDetails(api, this.value, 'parent-a-details')` (or `parent-b-details`) on change.

### b. Display Selected Parent Details (`#parent-a-details`, `#parent-b-details`)
*   **Function:** `async function displaySelectedParentDetails(api, petId, detailElementId)`
*   **Logic:**
    1.  If `petId` is empty/null, clear the content of the element with `detailElementId`.
    2.  Fetch `PetNft` details for `petId` using `api.query.critterNftsPallet.petNfts(petId)`.
    3.  If the pet exists, display key attributes relevant for breeding: species, level, charter attributes (e.g., `base_strength`, `primary_elemental_affinity`), generation, current breeding cooldown status (calculated as above).

### c. Populate Fertility Item Select (`#fertility-item-select`) - Conceptual
*   **Function:** `async function populateFertilityItemsSelect(api, accountAddress)`
*   **Logic:**
    1.  Query `api.query.itemsPallet.userItemInventory.entries(accountAddress)`. This returns an array of `[StorageKey, u32 (quantity)]`.
    2.  For each entry, decode `itemId` from `StorageKey`.
    3.  Query `api.query.itemsPallet.itemDefinitions(itemId)` to get `ItemDetails`.
    4.  Filter for items where `itemDetails.category === ItemCategory.BreedingAssist` and quantity > 0.
    5.  Populate `#fertility-item-select` with `<option value="itemId">ItemName (Qty: quantity)</option>`. Add a "None" option.

### d. Initiate Breeding (`#initiateBreedingButton` listener)
*   **Extrinsic:** `window.critterApi.tx.breedingPallet.initiateBreeding(parent1PetId, parent2PetId, selectedFertilityItemIdOrNull)`
    *   `selectedFertilityItemIdOrNull` should be `null` if "None" is selected.
*   **JS Logic:**
    1.  Get selected `parent1PetId` from `#parent-a-select`, `parent2PetId` from `#parent-b-select`, and `selectedFertilityItemId` from `#fertility-item-select`.
    2.  Perform client-side validation: ensure parents are different, both are selected, etc.
    3.  Construct the extrinsic.
    4.  Call `submitExtrinsic(extrinsic, ALICE_ADDRESS, 'breeding-status', (events) => { ... });`
    5.  On success, parse events for `breedingPallet.BreedingInitiated { offspring_id, ready_at_block, breeder }`.
        *   Refresh `populateBreedingParentSelects` (to reflect new cooldowns).
        *   Call `displayBreedingOutcomes(api, ALICE_ADDRESS)`.
        *   Update UI with new offspring details and estimated ready time.

### e. Display Breeding Outcomes (`#offspring-list`)
*   **Function:** `async function displayBreedingOutcomes(api, accountAddress)`
*   **Logic:**
    1.  Query `api.query.breedingPallet.pendingOffspring.entries()`. This gives `[StorageKey, Option<OffspringDetails>]`.
    2.  Filter for entries where `OffspringDetails.breeder === accountAddress`.
    3.  For each pending offspring:
        *   Display `offspringId` (from key or struct), parent IDs, status ("Incubating" or "Ready to Claim"), `ready_at_block`.
        *   Fetch current block number: `api.query.system.number()`.
        *   Calculate and display time remaining or if ready.
        *   Show a "Claim Newborn" button, enabled if `current_block >= offspringDetails.ready_at_block`. Store `offspringId` in a data attribute.

### f. Claim Newborn (`.claim-newborn-button` listener via event delegation on `#offspring-list`)
*   **Extrinsic:** `window.critterApi.tx.breedingPallet.claimOffspring(offspringId)`
*   **JS Logic:**
    1.  Get `offspringId` from the button's data attribute.
    2.  Construct and call `submitExtrinsic`.
    3.  On success (e.g., `breedingPallet.OffspringClaimed { new_pet_id, owner }` event):
        *   Refresh `displayOwnedNfts(api, ALICE_ADDRESS)`.
        *   Refresh `populateBreedingParentSelects(api, ALICE_ADDRESS)`.
        *   Refresh `displayBreedingOutcomes(api, ALICE_ADDRESS)`.
        *   Notify user: "Congratulations on your new Pet NFT (ID: new_pet_id)!"

### g. Display Lineage (enhanced in `displayOwnedNfts`)
*   When displaying each pet in "My Pet NFTs":
    *   If `PetNft` struct contains `parent_1_id` and `parent_2_id` (or similar fields):
        *   If IDs are present, display them. Optionally, fetch names/brief details of parents.
    *   The "[View Full Lineage Tree (Future)]" button remains conceptual for a more complex UI.

## 2. Pet Day Cares (`#daycare-center-section`)

### a. Display Available Day Care Services (`#daycare-service-list`)
*   **Function:** `async function displayDaycareServices(api, accountAddress)`
*   **Logic:**
    1.  Query `api.query.daycarePallet.dayCareServices.entries()` giving `[StorageKey (operatorId), Option<ServiceDetails>]`.
    2.  For each service where `ServiceDetails` is Some:
        *   Display operator ID, specialty, fee, capacity, current occupancy.
        *   If `ServiceDetails.caregiver_pet_id` is present, fetch and display caregiver pet details from `critterNftsPallet`.
        *   Dynamically create a pet selection dropdown (`.pet-to-enroll-select`) for this service.
            *   Populate with user's (`accountAddress`) eligible pets (owned, not locked, not already in any daycare).
        *   Add an "Enroll Selected Pet" button associated with this service and the pet select.

### b. Enroll Pet (`.enroll-daycare-button` listener, specific to a service)
*   **Extrinsic:** `window.critterApi.tx.daycarePallet.enrollPetInDaycare(petToEnrollId, serviceOperatorId)`
*   **JS Logic:** Get `petToEnrollId` from the associated dropdown and `serviceOperatorId` from the service listing. Call `submitExtrinsic`. Refresh daycare lists and owned NFT list (to show status) on success.

### c. Display My Boarded Pets (`#boarded-pets-list`)
*   **Function:** `async function displayMyBoardedPets(api, accountAddress)`
*   **Logic:**
    1.  Query `api.query.daycarePallet.boardedPets.entries()`. This gives `[StorageKey (petId), Option<BoardingDetails>]`.
    2.  For each entry, fetch `PetNft` for `petId`. If `petNft.owner === accountAddress` and `BoardingDetails` is Some:
        *   Display pet ID/name, daycare operator (from `BoardingDetails.operator`), status, expected benefits (from `ServiceDetails`), enrollment time, expected retrieval time (if applicable).
        *   Add "Retrieve Pet" button.

### d. Retrieve Pet (`.retrieve-from-daycare-button` listener)
*   **Extrinsic:** `window.critterApi.tx.daycarePallet.retrievePetFromDaycare(petId)`
*   **JS Logic:** Get `petId`. Call `submitExtrinsic`. Refresh daycare lists and owned NFT list on success.

### e. Manage My Day Care Service (`#manage-my-daycare` - For Operators)
*   **Display Service Status:**
    *   Query `api.query.daycarePallet.dayCareServices(ALICE_ADDRESS)`.
    *   If service exists, display its details.
    *   List currently boarded pets: Iterate `api.query.daycarePallet.boardedPets.entries()`, filter by `BoardingDetails.operator === ALICE_ADDRESS`.
    *   Display accumulated earnings (query `api.query.daycarePallet.operatorEarnings(ALICE_ADDRESS)` - conceptual storage).
*   **Setup/Update Service (`#setupMyDaycareButton` listener):**
    *   UI form for `capacity`, `fee_per_block`, `caregiver_pet_id_option`, `specialty_option`.
    *   **Extrinsic:** `window.critterApi.tx.daycarePallet.registerDaycareService(capacity, fee, caregiverPetIdOption, specialtyOption)`
    *   Call `submitExtrinsic`.
*   **Withdraw Earnings (`#withdrawDaycareEarningsButton` listener):**
    *   **Extrinsic:** `window.critterApi.tx.daycarePallet.withdrawDaycareEarnings()`
    *   Call `submitExtrinsic`. Refresh earnings display and account balance on success.

## 3. User Shops (`#user-shops-section`)

### a. Display/Search User Shops (`#user-shop-list`)
*   **Function:** `async function displayUserShops(api, searchTerm = null)`
*   **Logic:** Query `api.query.userShopsPallet.shops.entries()` giving `[StorageKey (shopId), Option<ShopDetails>]`.
    *   If `searchTerm`, filter results based on `ShopDetails.name` or `ShopDetails.description`.
    *   Display shop name, owner, description. Add "Visit Shop" button with `shopId`.

### b. View Individual Shop (`#view-individual-shop` area - triggered by "Visit Shop")
*   **Function:** `async function viewIndividualShop(api, shopId)`
*   **Logic:**
    1.  Query `api.query.userShopsPallet.shops(shopId)` for `ShopDetails`.
    2.  Query `api.query.userShopsPallet.shopListings.entries(shopId)` (prefix scan for `(ShopId, ListingId) -> ListingDetails`).
    3.  For each `ListingDetails`:
        *   If `listingType` is NFT, fetch NFT details from `critterNftsPallet.petNfts(itemOrNftId)`.
        *   If `listingType` is Item, fetch item details from `itemsPallet.itemDefinitions(itemOrNftId)`.
    4.  Display shop info and all listings with price, quantity, and "Buy Item/NFT" buttons.

### c. Buy from Shop (`.buy-from-shop-button` listener)
*   **Extrinsic:** `window.critterApi.tx.userShopsPallet.purchaseFromShop(shopId, listingId, quantityToBuy)`
    *   `listingId` would be known from displaying the shop.
*   **JS Logic:** Call `submitExtrinsic`. On success, refresh user's balance, user's item/NFT inventory, and possibly the shop's displayed inventory (or just re-fetch).

### d. Manage My Shop (`#manage-my-shop`)
*   **Create/Edit Shop (`#saveMyShopDetailsButton` listener):**
    *   UI form for `name`, `description`.
    *   Query `api.query.userShopsPallet.shopsOwnedBy(ALICE_ADDRESS)` to see if a shop exists.
    *   If not, **Extrinsic:** `window.critterApi.tx.userShopsPallet.createShop(name, description)`.
    *   If exists, **Extrinsic:** `window.critterApi.tx.userShopsPallet.updateShopDetails(shopId, name, description)`.
*   **Add Item/NFT to Shop (`#addListingToShopButton` listener):**
    *   UI form for selecting owned item/NFT, `price`, `quantity`.
    *   **Extrinsic:** `window.critterApi.tx.userShopsPallet.addListingToShop(shopId, listingType, itemOrNftId, price, quantity)`
*   **Remove Listing from Shop (`.remove-listing-button` listener):**
    *   **Extrinsic:** `window.critterApi.tx.userShopsPallet.removeListingFromShop(shopId, listingId)`
*   **Display My Shop Inventory/Listings (`#my-shop-inventory-list`):** Query `shopListings` for own `shopId`.
*   **Withdraw Shop Earnings (`#withdrawShopEarningsButton` listener):**
    *   **Extrinsic:** `window.critterApi.tx.userShopsPallet.withdrawShopEarnings(shopId)`
*   **JS Logic Overview:** All these management actions use `submitExtrinsic` and trigger UI refreshes for the relevant parts of the "Manage My Shop" section, user's inventory/balance, etc.

This document provides a high-level guide for the JavaScript interactions. Actual implementation would involve more detailed state management, error handling, UI updates, and potentially more sophisticated data fetching/caching strategies.
```
