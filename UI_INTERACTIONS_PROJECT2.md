# CritterCraft UI Wallet: Project 2 Interaction Logic (`app.js` Conceptual Detailing)

This document outlines the conceptual JavaScript logic within `app.js` for making the UI Wallet functional with the core gameplay pallets: `pallet-critter-nfts`, `pallet-marketplace`, `pallet-battles`, and `pallet-quests`. It details how Polkadot.js API (`window.critterApi`) would be used for querying storage and submitting extrinsics, moving beyond the previous "simulated transaction" placeholders.

**Assumptions:**
*   `window.critterApi` is initialized and connected to the CritterChain node.
*   `ALICE_ADDRESS` (or a dynamically selected user account via extension) is available for signing transactions.
*   Helper functions like `formatDisplayBalance` exist.
*   HTML element IDs match those defined in `blockchain_core/ui-wallet/index.html`.

## 1. General Transaction Handling Function (Conceptual)

A generic helper function would be beneficial for submitting extrinsics and handling status updates to reduce code duplication.

```javascript
// Conceptual helper in app.js
async function submitExtrinsic(extrinsic, account, statusElementId, successCallback) {
    const statusP = document.getElementById(statusElementId);
    if (!statusP) {
        console.error("Status element not found:", statusElementId);
        return;
    }

    try {
        statusP.textContent = 'Preparing transaction...';
        statusP.style.color = 'orange';

        // In a real app, use web3FromAddress to get signer from extension for `account`
        // const { web3FromAddress } = await import('@polkadot/extension-dapp');
        // const injector = await web3FromAddress(account);

        await extrinsic.signAndSend(account, /* { signer: injector.signer }, */ ({ status, events = [], dispatchError }) => {
            if (status.isInBlock) {
                statusP.textContent = `Transaction in block: ${status.asInBlock.toHex()}`;
                statusP.style.color = 'blue';
            } else if (status.isBroadcast) {
                statusP.textContent = `Transaction broadcasted. Hash: ${status.asBroadcast.toHex()}`;
            } else if (status.isFinalized) {
                statusP.textContent = `Transaction finalized in block: ${status.asFinalized.toHex()}`;
                statusP.style.color = 'green';

                if (dispatchError) {
                    if (dispatchError.isModule) {
                        const decoded = window.critterApi.registry.findMetaError(dispatchError.asModule);
                        const { docs, name, section } = decoded;
                        statusP.textContent = `Transaction Failed: ${section}.${name}: ${docs.join(' ')}`;
                        statusP.style.color = 'red';
                        console.error(`Error: ${section}.${name} - ${docs.join(' ')}`);
                    } else {
                        statusP.textContent = `Transaction Failed: ${dispatchError.toString()}`;
                        statusP.style.color = 'red';
                    }
                } else {
                    statusP.textContent = 'Transaction Successful and Finalized!';
                    // Call success callback if provided
                    if (successCallback && typeof successCallback === 'function') {
                        successCallback(events);
                    }
                }
                // Re-enable buttons or clear inputs if needed by the caller of submitExtrinsic
            } else if (status.isReady) {
                statusP.textContent = 'Transaction is ready.';
            } else {
                statusP.textContent = `Transaction status: ${status.type}`;
            }
        });
    } catch (error) {
        statusP.textContent = `Transaction submission error: ${error.message}`;
        statusP.style.color = 'red';
        console.error('Submission error:', error);
    }
}
```

## 2. `pallet-critter-nfts` Interactions

### a. Minting a Pet NFT (Enhance `mintNftButton` listener)
*   **Extrinsic:** `window.critterApi.tx.critterNftsPallet.mintPetNft(species, name)`
*   **JS Logic:**
    ```javascript
    // const mintNftButton = document.getElementById('mintNftButton');
    // mintNftButton.addEventListener('click', async () => {
    //     // ... (get species, name, check API/account) ...
    //     const extrinsic = window.critterApi.tx.critterNftsPallet.mintPetNft(species, name);
    //     await submitExtrinsic(extrinsic, ALICE_ADDRESS, 'mint-status', (events) => {
    //         // Look for PetNftMinted event to get PetId
    //         events.forEach(({ event: { data, method, section } }) => {
    //             if (section === 'critterNftsPallet' && method === 'PetNftMinted') {
    //                 const [owner, petId] = data; // Adjust based on actual event data structure
    //                 console.log(`PetNftMinted: Owner ${owner.toString()}, PetID ${petId.toString()}`); // Use .toString() for complex types
    //                 // Update UI or provide more specific success message
    //                 document.getElementById('mint-status').textContent += ` New Pet ID: ${petId.toString()}`;
    //             }
    //         });
    //         populateBattlePetSelect(window.critterApi, ALICE_ADDRESS); // Refresh pet lists
    //         displayOwnedNfts(window.critterApi, ALICE_ADDRESS);
    //     });
    // });
    ```

### b. Displaying Owned NFTs (`displayOwnedNfts`)
*   **Storage Queries:**
    *   `window.critterApi.query.critterNftsPallet.ownerOfPet(accountAddress)` -> `Vec<PetId>`
    *   For each `petId`: `window.critterApi.query.critterNftsPallet.petNfts(petId)` -> `Option<PetNft>` struct.
*   **JS Logic:** (Already quite detailed conceptually, ensure it uses `await` and handles `Option` types from storage correctly, e.g., `petNftOpt.isSome ? petNftOpt.unwrap() : null`).

### c. Updating Pet Metadata (e.g., `set-pet-name` via `updatePetButton` listener)
*   **Extrinsic:** `window.critterApi.tx.critterNftsPallet.updatePetMetadata(petId, newName, newLevel, newXp, newMoodIndicator, newHungerTimestamp, newEnergyTimestamp, newPersonalityTraits, newCharterAttributes)`
    *   Note: The actual extrinsic might take fewer parameters, or `null` for those not being updated. The example below simplifies to `newName` and `newPersonalityTraits`.
*   **JS Logic:**
    ```javascript
    // const updatePetButton = document.getElementById('updatePetButton');
    // updatePetButton.addEventListener('click', async () => {
    //     // ... (get petId, newName, newPersonalityTraits from form) ...
    //     // For optional fields, pass null if not provided, or ensure the pallet handles Option<T> correctly.
    //     // Example: const newNameForTx = newName ? newName : null;
    //     // Example: const newTraitsForTx = newPersonalityTraits.length > 0 ? newPersonalityTraits.split(',') : null;
    //     const extrinsic = window.critterApi.tx.critterNftsPallet.updatePetMetadata(petId, newNameForTx, null, null, null, null, null, newTraitsForTx, null); // Pass null for non-updated fields
    //     await submitExtrinsic(extrinsic, ALICE_ADDRESS, 'update-pet-status', () => {
    //         displayOwnedNfts(window.critterApi, ALICE_ADDRESS); // Refresh NFT list
    //     });
    // });
    ```

### d. `feed_pet`, `play_with_pet`, `apply_neglect_check`
*   **Extrinsics:**
    *   `window.critterApi.tx.critterNftsPallet.feedPet(petId, itemId)`
    *   `window.critterApi.tx.critterNftsPallet.playWithPet(petId, itemId)`
    *   `window.critterApi.tx.critterNftsPallet.applyNeglectCheck(petId)`
*   **JS Logic:** Similar structure to `update_pet_metadata`. Get `petId` (and `itemId` if applicable). Call `submitExtrinsic`. On success, refresh relevant pet details in `displayOwnedNfts` (or a more targeted update function) and potentially item inventory if items are consumed.

### e. `claim_daily_ptcn`
*   **Extrinsic:** `window.critterApi.tx.critterNftsPallet.claimDailyPtcn()`
*   **JS Logic:** Call `submitExtrinsic`. On success, refresh `displayAccountBalance` and `displayNextClaimTime`.

## 3. `pallet-marketplace` Interactions

### a. Displaying Marketplace Listings (`displayMarketplaceListings`)
*   **Storage Query:** `window.critterApi.query.marketplacePallet.listings.entries()` -> `Vec<[StorageKey, Option<ListingDetails>]>`
*   For each listing, if `Option<ListingDetails>` is `Some`, extract `petId` and fetch Pet NFT details: `window.critterApi.query.critterNftsPallet.petNfts(petId)`
*   **JS Logic:** (Already quite detailed conceptually. Ensure to handle the `Option` for `ListingDetails`).

### b. Listing an NFT (`listNftButton` listener)
*   **Extrinsic:** `window.critterApi.tx.marketplacePallet.listNftForSale(petId, priceBigInt)`
*   **JS Logic:** Convert price (e.g., from a text input) to chain's smallest unit (BigInt). Call `submitExtrinsic`. On success, refresh `displayOwnedNfts` and `displayMarketplaceListings`.

### c. Unlisting an NFT (`unlistNftButton` listener)
*   **Extrinsic:** `window.critterApi.tx.marketplacePallet.unlistNftFromSale(petId)`
*   **JS Logic:** Call `submitExtrinsic`. On success, refresh `displayOwnedNfts` and `displayMarketplaceListings`.

### d. Buying an NFT ("Buy" button listener in `displayMarketplaceListings`)
*   **Extrinsic:** `window.critterApi.tx.marketplacePallet.buyNft(petId)`
*   **JS Logic:** Call `submitExtrinsic`. On success, refresh `displayOwnedNfts`, `displayMarketplaceListings`, and `displayAccountBalance`.

## 4. `pallet-battles` Interactions

### a. Populating Pet Select for Battle (`populateBattlePetSelect`)
*   **Storage Queries:** Similar to `displayOwnedNfts` to get ownable, non-locked pets.

### b. Registering for Battle (`registerBattleButton` listener)
*   **Extrinsic:** `window.critterApi.tx.battlesPallet.registerForBattle(petId)`
*   **JS Logic:** Call `submitExtrinsic`. On success, refresh `displayCurrentBattles`, `populateBattlePetSelect` (as pet might now be in battle), and `displayOwnedNfts` (to show status changes).

### c. Displaying Current Battles (`displayCurrentBattles`)
*   **Storage Query:** `window.critterApi.query.battlesPallet.battles.entries()` -> `Vec<[StorageKey, Option<BattleDetails>]>`
*   Fetch Pet NFT details for display for pets involved in battles.
*   **JS Logic:** (Already detailed conceptually. Ensure to handle `Option<BattleDetails>`).

### d. Reporting Battle Outcome ("Simulate Battle" button listener)
*   **Extrinsic:** `window.critterApi.tx.battlesPallet.reportBattleOutcome(battleId, winnerPetId)` (MVP: winner is explicit)
*   **JS Logic:** Call `submitExtrinsic`. On success, refresh `displayCurrentBattles`, `displayOwnedNfts` (for XP/level changes), and `displayAccountBalance` (for rewards).

## 5. `pallet-quests` Interactions

### a. Displaying Available Quests (`displayAvailableQuests`)
*   **Storage Query:** `window.critterApi.query.questsPallet.availableQuests.entries()` -> `Vec<[StorageKey, Option<Quest>]>`
*   **JS Logic:** (Already detailed conceptually. Ensure to handle `Option<Quest>`).

### b. Displaying Completed Quests (`displayCompletedQuests`)
*   **Storage Queries:**
    *   First, get all `questId`s from `window.critterApi.query.questsPallet.availableQuests.keys()`.
    *   For each `questId`: `window.critterApi.query.questsPallet.completedQuests([accountAddress, questId])` -> `Option<()>`
    *   If `Some(())`, then the quest is completed by the user. Fetch quest details from `availableQuests(questId)` for display.
*   **JS Logic:** (Already detailed conceptually).

### c. Completing a Quest ("Complete Quest" button listener)
*   **Extrinsic:** `window.critterApi.tx.questsPallet.completeQuest(questId, maybeTargetPetId)`
    *   `maybeTargetPetId` would be `null` if the quest doesn't require a specific pet or if the UI doesn't provide a selection for it. The UI should determine if a pet selection is necessary based on quest details.
*   **JS Logic:** Call `submitExtrinsic`. On success, refresh `displayAvailableQuests`, `displayCompletedQuests`, `displayAccountBalance`, and potentially `displayOwnedNfts` if a pet gained XP/levels (though quest pallet doesn't directly do this, user profile might).

## 6. Event Subscription (`subscribeToSystemEvents`)

*   **API Call:** `window.critterApi.query.system.events((events) => { ... });`
*   **JS Logic:**
    *   Iterate `events`. For each `record { event, phase }`:
    *   Check `event.section` (e.g., `critterNftsPallet`, `marketplacePallet`, `battlesPallet`, `questsPallet`, `balances`, `system`, etc.).
    *   Check `event.method` (e.g., `PetNftMinted`, `NftListed`, `BattleConcluded`, `QuestCompleted`, `Transfer`, `ExtrinsicSuccess`, `ExtrinsicFailed`).
    *   Decode `event.data.toJSON()` or iterate `event.data.toArray()` for individual field values. Convert them to human-readable format (e.g., addresses with `toString()`, numbers, balances).
        ```javascript
        // Example within the event loop
        // const { section, method, data } = event;
        // const eventDataStr = data.map(d => d.toString()).join(', ');
        // const logMessage = `Event: ${section}.${method} [${eventDataStr}]`;
        // Add to "Recent Activity" UI list.
        ```
    *   This function is key for providing real-time feedback beyond individual transaction statuses and for observing general chain activity.

This document provides a conceptual blueprint for the JavaScript logic needed in `app.js` to make the UI Wallet fully interactive with the core gameplay pallets of CritterChain. It emphasizes using the Polkadot.js API for actual blockchain interactions.
