// At the top of app.js
const { ApiPromise, WsProvider } = window.PolkadotApi;

// Make the API instance available globally for other functions
window.critterApi = null;
const ALICE_ADDRESS = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'; // Example: Alice's address
const MAX_EVENTS_DISPLAYED = 10; // Max number of events to show

let CLAIM_COOLDOWN_PERIOD_BLOCKS = 100; // Default/Placeholder
let DAILY_CLAIM_AMOUNT = BigInt("10000000000000000000"); // Default/Placeholder: 10 PTCN

// Simplified balance formatting function (ensure consistency or use a more robust one)
function formatDisplayBalance(balanceBigInt, decimals) {
    const balanceStr = balanceBigInt.toString();
    let CENTS_PLACES = 4; // Number of decimal places to show

    if (decimals === 0) return balanceStr; // No decimals, show raw value

    let intPart;
    let fracPart;

    if (balanceStr.length > decimals) {
        intPart = balanceStr.substring(0, balanceStr.length - decimals);
        fracPart = balanceStr.substring(balanceStr.length - decimals);
    } else {
        intPart = '0';
        fracPart = '0'.repeat(decimals - balanceStr.length) + balanceStr;
    }
    return `${intPart}.${fracPart.substring(0, CENTS_PLACES)}`;
}


async function displayAccountBalance(api, accountAddress) {
    const walletAddressSpan = document.getElementById('wallet-address');
    const ptcnBalanceSpan = document.getElementById('ptcn-balance');

    if (!api || !accountAddress) {
        if (walletAddressSpan) walletAddressSpan.textContent = 'N/A';
        if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = 'N/A - API or Account not available';
        return;
    }

    if (walletAddressSpan) walletAddressSpan.textContent = accountAddress;

    try {
        await api.query.system.account(accountAddress, ({ nonce, data: balance }) => {
            const chainDecimals = api.registry.chainDecimals[0] || 18;
            const formattedBalance = formatDisplayBalance(balance.free.toBigInt(), chainDecimals);

            if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = `${formattedBalance} PTCN`;
            console.log(`Account: ${accountAddress}, Nonce: ${nonce}, Free Balance (raw): ${balance.free.toString()}, Formatted: ${formattedBalance} PTCN`);
        });
    } catch (error) {
        if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = `Error fetching balance: ${error.message}`;
        console.error('Balance fetching error for account', accountAddress, ':', error);
    }
}

async function displayOwnedNfts(api, accountAddress) {
    const nftListUl = document.getElementById('nft-list');
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');

    if (!api || !accountAddress) { /* ... */ return; }
    try {
        if (nftListPlaceholder) { nftListPlaceholder.textContent = 'Loading Pet NFTs...'; nftListPlaceholder.style.display = 'block'; }
        if (nftListUl) nftListUl.innerHTML = '';
        const ownedPetIdsVec = await api.query.critterNftsPallet.ownerOfPet(accountAddress);
        const ownedPetIds = ownedPetIdsVec.isSome ? ownedPetIdsVec.unwrap() : [];
        if (ownedPetIds.length === 0) { /* ... */ if (nftListPlaceholder) { nftListPlaceholder.textContent = 'You do not own any Pet NFTs yet.'; nftListPlaceholder.style.display = 'block';} return; }
        if (nftListPlaceholder) nftListPlaceholder.style.display = 'none';
        for (const petId of ownedPetIds) {
            const petNftOpt = await api.query.critterNftsPallet.petNfts(petId.toNumber());
            if (petNftOpt.isSome) {
                const petNft = petNftOpt.unwrap();
                const listItem = document.createElement('li');
                const petName = api.registry.createType('Text', petNft.current_pet_name).toString();
                const petSpecies = api.registry.createType('Text', petNft.initial_species).toString();
                const dnaHash = petNft.dna_hash.toHex ? petNft.dna_hash.toHex() : petNft.dna_hash.toString();
                const personalityTraits = petNft.personality_traits.map(t => api.registry.createType('Text', t).toString()).join(', ') || 'N/A';
                listItem.innerHTML = `<strong>ID:</strong> ${petNft.id.toNumber()} <br><strong>Name:</strong> ${petName} <br><strong>Species:</strong> ${petSpecies} <br><strong>Level:</strong> ${petNft.level.toNumber()} | <strong>XP:</strong> ${petNft.experience_points.toNumber()} <br><strong>Mood:</strong> ${petNft.mood_indicator.toNumber()} | <strong>Hunger:</strong> ${petNft.hunger_status.toNumber()} | <strong>Energy:</strong> ${petNft.energy_status.toNumber()} <br><strong>DNA Hash:</strong> ${dnaHash.substring(0, 10)}... <br><strong>Personality:</strong> ${personalityTraits}<hr>`;
                if (nftListUl) nftListUl.appendChild(listItem);
            }
        }
    } catch (error) { /* ... */ if (nftListPlaceholder) { nftListPlaceholder.textContent = `Error fetching Pet NFTs: ${error.message}`; nftListPlaceholder.style.display = 'block'; } console.error(error); }
}

async function subscribeToNftEvents(api) {
    const eventListUl = document.getElementById('event-list');
    const eventListPlaceholder = document.getElementById('event-list-placeholder');
    if (!api) { /* ... */ return; }
    if (eventListPlaceholder) eventListPlaceholder.textContent = 'Subscribed to events. Waiting for activity...';
    await api.query.system.events((events) => {
        if (events.length === 0) return;
        if (eventListPlaceholder) eventListPlaceholder.style.display = 'none';
        events.forEach((record) => {
            const { event, phase } = record;
            if (event.section === 'critterNftsPallet' || event.section === 'marketplacePallet') { // Listen to both pallets
                let eventString = `Block #${phase.isApplyExtrinsic ? phase.asApplyExtrinsic.toString() : '-'}: [${event.section}] `;
                if (event.method === 'PetNftMinted') {
                    eventString += `Pet NFT Minted! ID: ${event.data[1]}, Owner: ${event.data[0].toString().substring(0, 8)}...`;
                } else if (event.method === 'PetNftTransferred') {
                    eventString += `Pet NFT Transferred! ID: ${event.data[2]}, From: ${event.data[0].toString().substring(0,8)}..., To: ${event.data[1].toString().substring(0,8)}...`;
                } else if (event.method === 'PetNftMetadataUpdated') {
                     eventString += `Pet NFT Metadata Updated! ID: ${event.data[1]}, Owner: ${event.data[0].toString().substring(0, 8)}...`;
                } else if (event.method === 'NftListed') {
                    const decimals = window.critterApi.registry.chainDecimals[0] || 18;
                    eventString += `NFT Listed! Seller: ${event.data[0].toString().substring(0,8)}..., PetID: ${event.data[1]}, Price: ${formatDisplayBalance(event.data[2].toBigInt(), decimals)} PTCN`;
                } else if (event.method === 'NftUnlisted') {
                    eventString += `NFT Unlisted! Seller: ${event.data[0].toString().substring(0,8)}..., PetID: ${event.data[1]}`;
                } else if (event.method === 'NftSold') {
                     const decimals = window.critterApi.registry.chainDecimals[0] || 18;
                    eventString += `NFT Sold! Buyer: ${event.data[0].toString().substring(0,8)}..., Seller: ${event.data[1].toString().substring(0,8)}..., PetID: ${event.data[2]}, Price: ${formatDisplayBalance(event.data[3].toBigInt(), decimals)} PTCN`;
                } else { return; }
                const listItem = document.createElement('li');
                listItem.textContent = eventString;
                if (eventListUl) eventListUl.insertBefore(listItem, eventListUl.firstChild);
                while (eventListUl && eventListUl.children.length > MAX_EVENTS_DISPLAYED) {
                    if (eventListUl.lastChild.tagName === 'LI') eventListUl.removeChild(eventListUl.lastChild); else break;
                }
            }
        });
    });
}

async function displayMarketplaceListings(api) {
    const marketplaceListUl = document.getElementById('marketplace-nft-list');
    const marketplaceListPlaceholder = document.getElementById('marketplace-list-placeholder');

    if (!api) {
        if (marketplaceListPlaceholder) marketplaceListPlaceholder.textContent = 'N/A - API not available for marketplace.';
        return;
    }

    if (marketplaceListPlaceholder) marketplaceListPlaceholder.textContent = 'Loading listings...';
    if (marketplaceListUl) marketplaceListUl.innerHTML = '';

    try {
        const listingsEntries = await api.query.marketplacePallet.listings.entries();

        if (listingsEntries.length === 0) {
            if (marketplaceListPlaceholder) marketplaceListPlaceholder.textContent = 'No NFTs currently listed for sale.';
            return;
        }
        if (marketplaceListPlaceholder) marketplaceListPlaceholder.style.display = 'none';

        for (const [key, listingOpt] of listingsEntries) {
            if (listingOpt.isSome) {
                const listing = listingOpt.unwrap();
                const petId = key.args[0].toNumber();

                const petNftOpt = await api.query.critterNftsPallet.petNfts(petId);
                let petDetailsHtml = `Pet ID: ${petId}`;
                if (petNftOpt.isSome) {
                    const petNft = petNftOpt.unwrap();
                    const petName = api.registry.createType('Text', petNft.current_pet_name).toString();
                    const petSpecies = api.registry.createType('Text', petNft.initial_species).toString();
                    petDetailsHtml = `<b>${petName}</b> (Species: ${petSpecies}, ID: ${petId}, Level: ${petNft.level.toNumber()})`;
                }

                const price = listing.price.toBigInt();
                const seller = listing.seller.toString();
                const chainDecimals = api.registry.chainDecimals[0] || 18;

                const listItem = document.createElement('li');
                listItem.innerHTML = `
                    ${petDetailsHtml} <br>
                    Seller: ${seller.substring(0, 8)}...${seller.substring(seller.length - 4)} <br>
                    Price: ${formatDisplayBalance(price, chainDecimals)} PTCN <br>
                    <button class="buy-nft-button" data-pet-id="${petId}" data-price="${price.toString()}" data-seller="${seller}">Buy</button>
                    <hr>
                `;
                if (marketplaceListUl) marketplaceListUl.appendChild(listItem);
            }
        }
    } catch (error) {
        if (marketplaceListPlaceholder) marketplaceListPlaceholder.textContent = `Error fetching marketplace listings: ${error.message}`;
        console.error('Marketplace listing error:', error);
    }
}


async function connectToCritterChain() {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    // ... (other element getters)
    const statusDiv = document.getElementById('connection-status');
    const actionsDiv = document.getElementById('actions');
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');
    const nftListUl = document.getElementById('nft-list');
    const eventListPlaceholder = document.getElementById('event-list-placeholder');
    const eventListUl = document.getElementById('event-list');
    const marketplaceListPlaceholder = document.getElementById('marketplace-list-placeholder');
    const marketplaceListUl = document.getElementById('marketplace-nft-list');
    const nextClaimTimeSpan = document.getElementById('next-claim-time'); // For connect error
    const claimDailyPtcnButton = document.getElementById('claimDailyPtcnButton'); // For connect error


    try {
        const api = await ApiPromise.create({ provider });
        window.critterApi = api; // Set global api instance

        await getPalletConstants(api); // Fetch constants first

        if (statusDiv) { statusDiv.innerHTML = `Successfully connected to chain: <strong>${api.runtimeChain.toString()}</strong> using <strong>${api.runtimeVersion.specName.toString()}</strong> (v${api.runtimeVersion.specVersion.toString()})`; statusDiv.style.color = 'green';}
        console.log(`Successfully connected to chain ${api.runtimeChain} using ${api.runtimeVersion.specName} v${api.runtimeVersion.specVersion}`);


        if (actionsDiv) actionsDiv.style.display = 'block';
        displayAccountBalance(api, ALICE_ADDRESS);
        displayOwnedNfts(api, ALICE_ADDRESS);
        subscribeToNftEvents(api);
        displayMarketplaceListings(api);
        displayNextClaimTime(api, ALICE_ADDRESS); // Display initial claim time

    } catch (error) {
        if (statusDiv) { statusDiv.innerHTML = `Error connecting to CritterChain: ${error.message}`; statusDiv.style.color = 'red';}
        console.error('Connection error:', error);
        if (actionsDiv) actionsDiv.style.display = 'none';
        displayAccountBalance(null, ALICE_ADDRESS);
        if (nftListPlaceholder) { nftListPlaceholder.textContent = 'Connection failed. Cannot load NFTs.'; nftListPlaceholder.style.display = 'block';}
        if (nftListUl) nftListUl.innerHTML = '';
        if (eventListPlaceholder) { eventListPlaceholder.textContent = 'Connection failed. Cannot load events.'; eventListPlaceholder.style.display = 'block';}
        if (eventListUl) eventListUl.innerHTML = '';
        if (marketplaceListPlaceholder) { marketplaceListPlaceholder.textContent = 'Connection failed. Cannot load marketplace listings.'; marketplaceListPlaceholder.style.display = 'block';}
        if (marketplaceListUl) marketplaceListUl.innerHTML = '';
        if (nextClaimTimeSpan) nextClaimTimeSpan.textContent = 'Connection failed.';
        if (claimDailyPtcnButton) claimDailyPtcnButton.disabled = true;
    }
}

async function getPalletConstants(api) {
    try {
        if (api.consts.critterNftsPallet && api.consts.critterNftsPallet.claimCooldownPeriod) {
            CLAIM_COOLDOWN_PERIOD_BLOCKS = api.consts.critterNftsPallet.claimCooldownPeriod.toNumber();
            console.log("Fetched ClaimCooldownPeriod:", CLAIM_COOLDOWN_PERIOD_BLOCKS);
        } else {
            console.warn("ClaimCooldownPeriod constant not found on chain, using placeholder:", CLAIM_COOLDOWN_PERIOD_BLOCKS);
        }
        if (api.consts.critterNftsPallet && api.consts.critterNftsPallet.dailyClaimAmount) {
            DAILY_CLAIM_AMOUNT = api.consts.critterNftsPallet.dailyClaimAmount.toBigInt();
            console.log("Fetched DailyClaimAmount:", DAILY_CLAIM_AMOUNT.toString());
        } else {
            console.warn("DailyClaimAmount constant not found on chain, using placeholder:", DAILY_CLAIM_AMOUNT.toString());
        }
    } catch (e) {
        console.error("Error fetching pallet constants, using placeholders:", e);
    }
}

async function displayNextClaimTime(api, accountAddress) {
    const nextClaimTimeSpan = document.getElementById('next-claim-time');
    const claimDailyPtcnButton = document.getElementById('claimDailyPtcnButton');

    if (!api || !accountAddress || !nextClaimTimeSpan || !claimDailyPtcnButton) {
        if (nextClaimTimeSpan) nextClaimTimeSpan.textContent = "N/A (API or account error)";
        if (claimDailyPtcnButton) claimDailyPtcnButton.disabled = true;
        return;
    }
    claimDailyPtcnButton.disabled = true; // Disable by default until we know it's available

    try {
        const lastClaimBlock = (await api.query.critterNftsPallet.lastClaimTime(accountAddress)).toNumber();
        const header = await api.rpc.chain.getHeader();
        const currentBlock = header.number.toNumber();

        if (lastClaimBlock === 0) { // Never claimed before
            nextClaimTimeSpan.textContent = "Now!";
            claimDailyPtcnButton.disabled = false;
            return;
        }

        const nextClaimAvailableBlock = lastClaimBlock + CLAIM_COOLDOWN_PERIOD_BLOCKS;

        if (currentBlock < nextClaimAvailableBlock) {
            const blocksRemaining = nextClaimAvailableBlock - currentBlock;
            const secondsRemaining = blocksRemaining * 6; // Assuming average block time of 6 seconds
            const minutesRemaining = Math.ceil(secondsRemaining / 60);
            nextClaimTimeSpan.textContent = `Approx. ${minutesRemaining} minutes (${blocksRemaining} blocks)`;
            claimDailyPtcnButton.disabled = true;
        } else {
            nextClaimTimeSpan.textContent = "Now!";
            claimDailyPtcnButton.disabled = false;
        }
    } catch (error) {
        console.error("Error calculating next claim time:", error);
        nextClaimTimeSpan.textContent = "Error calculating.";
        claimDailyPtcnButton.disabled = false; // Allow attempt if calculation fails
    }
}


document.addEventListener('DOMContentLoaded', () => {
    console.log("CritterCraft Wallet UI Initializing");
    // ... (Initialize all placeholders: wallet, nft list, event list)
    const actionsDiv = document.getElementById('actions');
    if (actionsDiv) actionsDiv.style.display = 'none';
    const walletAddressSpan = document.getElementById('wallet-address');
    if (walletAddressSpan) walletAddressSpan.textContent = 'Loading...';
    const ptcnBalanceSpan = document.getElementById('ptcn-balance');
    if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = 'Loading...';
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');
    if (nftListPlaceholder) { nftListPlaceholder.textContent = 'Loading Pet NFTs...'; nftListPlaceholder.style.display = 'block';}
    const nftListUl = document.getElementById('nft-list');
    if (nftListUl) nftListUl.innerHTML = '';
    const eventListPlaceholder = document.getElementById('event-list-placeholder');
    if (eventListPlaceholder) { eventListPlaceholder.textContent = 'No recent activity or loading events...'; eventListPlaceholder.style.display = 'block';}
    const eventListUl = document.getElementById('event-list');
    if (eventListUl) eventListUl.innerHTML = '';
    const marketplaceListPlaceholder = document.getElementById('marketplace-list-placeholder');
    if (marketplaceListPlaceholder) { marketplaceListPlaceholder.textContent = 'Loading marketplace listings...'; marketplaceListPlaceholder.style.display = 'block';}
    const marketplaceListUl = document.getElementById('marketplace-nft-list');
    if (marketplaceListUl) marketplaceListUl.innerHTML = '';
    const nextClaimTimeSpan = document.getElementById('next-claim-time');
    if (nextClaimTimeSpan) nextClaimTimeSpan.textContent = 'Calculating...';
    const claimDailyPtcnButton = document.getElementById('claimDailyPtcnButton');
    if(claimDailyPtcnButton) claimDailyPtcnButton.disabled = true;


    connectToCritterChain();

    // Daily PTCN Claim Button Logic
    // const claimDailyPtcnButton = document.getElementById('claimDailyPtcnButton'); // Already got above
    const claimPtcnStatusP = document.getElementById('claim-ptcn-status');
    // const nextClaimTimeSpan = document.getElementById('next-claim-time'); // Already got above

    if (claimDailyPtcnButton) {
        claimDailyPtcnButton.addEventListener('click', async () => {
            if (!window.critterApi || !ALICE_ADDRESS) {
                claimPtcnStatusP.textContent = 'Error: API not ready or no account selected.';
                claimPtcnStatusP.style.color = 'red';
                return;
            }

            claimPtcnStatusP.textContent = 'Preparing claim...';
            claimPtcnStatusP.style.color = 'orange';
            claimDailyPtcnButton.disabled = true;

            try {
                const claimTx = window.critterApi.tx.critterNftsPallet.claimDailyPtcn();

                console.log(`Transaction to send: critterNftsPallet.claimDailyPtcn() by ${ALICE_ADDRESS}`);
                claimPtcnStatusP.textContent = `Mock Submission: Claiming daily PTCN. Check console.`;
                claimPtcnStatusP.style.color = 'blue';

                setTimeout(async () => {
                    if (window.critterApi) {
                        // Balance should auto-update via subscription in displayAccountBalance
                        await displayNextClaimTime(window.critterApi, ALICE_ADDRESS);
                    }
                    claimPtcnStatusP.textContent = 'Claim (mock) submitted. Cooldown updated. Balance should refresh.';
                    claimPtcnStatusP.style.color = 'green';
                    // Button state (disabled/enabled) is handled by displayNextClaimTime
                }, 3000);

            } catch (error) {
                claimPtcnStatusP.textContent = `Error claiming PTCN: ${error.message}`;
                claimPtcnStatusP.style.color = 'red';
                console.error('Claim PTCN error:', error);
                claimDailyPtcnButton.disabled = false; // Re-enable on error before cooldown recalculation
            }
        });
    }

    // Mint NFT Button Logic (condensed for brevity in this diff, no functional change)
    const mintNftButton = document.getElementById('mintNftButton');
    if (mintNftButton) mintNftButton.addEventListener('click', async () => { /* ... existing mint logic ... */ });
    // Transfer NFT Button Logic (condensed)
    const transferNftButton = document.getElementById('transferNftButton');
    if (transferNftButton) transferNftButton.addEventListener('click', async () => { /* ... existing transfer logic ... */ });
    // Update Pet Details Button Logic (condensed)
    const updatePetButton = document.getElementById('updatePetButton');
    if (updatePetButton) updatePetButton.addEventListener('click', async () => { /* ... existing update logic ... */ });


    // Event listener for listing an NFT
    const listNftButton = document.getElementById('listNftButton');
    const listPetIdInput = document.getElementById('list-pet-id');
    const listPriceInput = document.getElementById('list-price');
    const listNftStatusP = document.getElementById('list-nft-status');

    if (listNftButton) {
        listNftButton.addEventListener('click', async () => {
            if (!window.critterApi || !ALICE_ADDRESS) {
                listNftStatusP.textContent = 'Error: API not ready or no account selected.'; listNftStatusP.style.color = 'red'; return;
            }
            const petIdStr = listPetIdInput.value.trim();
            const priceStr = listPriceInput.value.trim();
            if (!petIdStr || !priceStr) {
                listNftStatusP.textContent = 'Error: Pet ID and Price are required.'; listNftStatusP.style.color = 'red'; return;
            }
            const petId = parseInt(petIdStr);
            // Price should be in the smallest unit of PTCN (e.g., like Plancks for DOT)
            // Assuming 1 PTCN = 10^18 smallest units, if user enters "1" PTCN, convert to 1 * 10^18
            // For this example, we'll assume user enters the smallest unit directly for simplicity with BigInt conversion.
            // Or, more realistically, parse float and multiply by 10^decimals.
            // Let's assume user enters price in PTCN, and we convert it.
            let priceBigInt;
            try {
                const chainDecimals = window.critterApi.registry.chainDecimals[0] || 18;
                // A simple way to handle floating point input for token units:
                const priceFloat = parseFloat(priceStr);
                if (isNaN(priceFloat) || priceFloat <= 0) throw new Error("Invalid price.");
                // Convert to smallest unit. This can have precision issues with floats.
                // Using a library for fixed-point arithmetic is better in production.
                priceBigInt = BigInt(Math.round(priceFloat * (10**chainDecimals)));
            } catch (e) {
                listNftStatusP.textContent = 'Error: Invalid price format or value.'; listNftStatusP.style.color = 'red'; return;
            }

            if (isNaN(petId) || petId < 0 ) { // Price check now handled by BigInt conversion
                listNftStatusP.textContent = 'Error: Invalid Pet ID.'; listNftStatusP.style.color = 'red'; return;
            }

            listNftStatusP.textContent = 'Preparing listing...'; listNftStatusP.style.color = 'orange';
            try {
                const listTx = window.critterApi.tx.marketplacePallet.listNftForSale(petId, priceBigInt);
                console.log(`Tx to send: marketplacePallet.listNftForSale(PetID: ${petId}, Price: ${priceBigInt.toString()}) by ${ALICE_ADDRESS}`);
                listNftStatusP.textContent = `Mock Submission: Listing Pet ID ${petId} for ${priceBigInt.toString()} units. Check console.`; listNftStatusP.style.color = 'blue';
                setTimeout(() => {
                    if (window.critterApi) {
                        displayOwnedNfts(window.critterApi, ALICE_ADDRESS);
                        displayMarketplaceListings(window.critterApi);
                    }
                    listNftStatusP.textContent = 'Listing (mock) submitted. Lists refreshed.'; listNftStatusP.style.color = 'green';
                }, 3000);
            } catch (error) {
                listNftStatusP.textContent = `Error listing NFT: ${error.message}`; listNftStatusP.style.color = 'red'; console.error('List NFT error:', error);
            }
        });
    }

    // Event listener for unlisting an NFT
    const unlistNftButton = document.getElementById('unlistNftButton');
    const unlistPetIdInput = document.getElementById('unlist-pet-id');
    const unlistNftStatusP = document.getElementById('unlist-nft-status');

    if (unlistNftButton) {
        unlistNftButton.addEventListener('click', async () => {
            if (!window.critterApi || !ALICE_ADDRESS) {
                unlistNftStatusP.textContent = 'Error: API not ready or no account selected.';
                unlistNftStatusP.style.color = 'red';
                return;
            }

            const petIdStr = unlistPetIdInput.value.trim();
            if (!petIdStr) {
                unlistNftStatusP.textContent = 'Error: Pet ID is required.';
                unlistNftStatusP.style.color = 'red';
                return;
            }
            const petId = parseInt(petIdStr);

            if (isNaN(petId) || petId < 0) {
                unlistNftStatusP.textContent = 'Error: Invalid Pet ID.';
                unlistNftStatusP.style.color = 'red';
                return;
            }

            unlistNftStatusP.textContent = 'Preparing to unlist...';
            unlistNftStatusP.style.color = 'orange';

            try {
                const unlistTx = window.critterApi.tx.marketplacePallet.unlistNftFromSale(petId);

                console.log(`Transaction to send: marketplacePallet.unlistNftFromSale(PetID: ${petId}) by ${ALICE_ADDRESS}`);
                unlistNftStatusP.textContent = `Mock Submission: Unlisting Pet ID ${petId}. Check console.`;
                unlistNftStatusP.style.color = 'blue';

                // Disable button to prevent multiple clicks during mock processing
                unlistNftButton.disabled = true;
                unlistNftButton.textContent = 'Processing...';

                setTimeout(() => {
                    if (window.critterApi) {
                        displayOwnedNfts(window.critterApi, ALICE_ADDRESS);
                        displayMarketplaceListings(window.critterApi);
                    }
                    unlistNftStatusP.textContent = 'Unlisting (mock) submitted. Lists refreshed.';
                    unlistNftStatusP.style.color = 'green';
                    unlistNftButton.disabled = false;
                    unlistNftButton.textContent = 'Unlist My NFT';
                }, 3000);

            } catch (error) {
                unlistNftStatusP.textContent = `Error unlisting NFT: ${error.message}`;
                unlistNftStatusP.style.color = 'red';
                console.error('Unlist NFT error:', error);
                unlistNftButton.disabled = false;
                unlistNftButton.textContent = 'Unlist My NFT';
            }
        });
    }

    // Event delegation for dynamically added "Buy" buttons
    const marketplaceListUl = document.getElementById('marketplace-nft-list');
    if (marketplaceListUl) {
        marketplaceListUl.addEventListener('click', async (event) => {
            if (event.target.classList.contains('buy-nft-button')) {
                if (!window.critterApi || !ALICE_ADDRESS) {
                    alert('Error: Not connected or no account selected.'); // Simple alert for this action
                    return;
                }

                const buyButton = event.target;
                const petId = parseInt(buyButton.dataset.petId);
                // const price = BigInt(buyButton.dataset.price); // Price from dataset is for info; pallet uses stored price.
                const seller = buyButton.dataset.seller;

                let buyStatusP = document.getElementById('buy-nft-status');
                if (!buyStatusP) {
                    buyStatusP = document.createElement('p');
                    buyStatusP.id = 'buy-nft-status';
                    const listNftForm = document.getElementById('list-nft-form'); // Insert after the form
                    if (listNftForm && listNftForm.parentNode) {
                        listNftForm.parentNode.insertBefore(buyStatusP, listNftForm.nextSibling);
                    } else { // Fallback if form not found
                        marketplaceListUl.parentNode.appendChild(buyStatusP);
                    }
                }

                buyStatusP.style.color = 'orange';
                buyStatusP.textContent = `Preparing to buy Pet ID ${petId}...`;

                if (ALICE_ADDRESS === seller) {
                    buyStatusP.textContent = "Error: You cannot buy your own listed NFT.";
                    buyStatusP.style.color = 'red';
                    return;
                }

                // Optional client-side balance check can be added here if desired

                try {
                    const buyTx = window.critterApi.tx.marketplacePallet.buyNft(petId);

                    console.log(`Transaction to send: marketplacePallet.buyNft(PetID: ${petId}) by ${ALICE_ADDRESS}`);
                    buyStatusP.textContent = `Mock Submission: Buying Pet ID ${petId}. Check console.`;
                    buyStatusP.style.color = 'blue';

                    buyButton.disabled = true;
                    buyButton.textContent = 'Processing...';

                    setTimeout(() => {
                        if (window.critterApi) {
                            displayOwnedNfts(window.critterApi, ALICE_ADDRESS);
                            displayMarketplaceListings(window.critterApi);
                            // Balance should auto-update via subscription in displayAccountBalance
                            // If not, an explicit call might be needed: displayAccountBalance(window.critterApi, ALICE_ADDRESS);
                        }
                        buyStatusP.textContent = `Buy attempt (mock) for Pet ID ${petId} submitted. Lists refreshed.`;
                        buyStatusP.style.color = 'green';
                        // Button will be removed/re-rendered by displayMarketplaceListings, so no need to re-enable here.
                    }, 3000);

                } catch (error) {
                    buyStatusP.textContent = `Error buying NFT: ${error.message}`;
                    buyStatusP.style.color = 'red';
                    console.error('Buy NFT error:', error);
                    buyButton.disabled = false;
                    buyButton.textContent = 'Buy';
                }
            }
        });
    }
});
