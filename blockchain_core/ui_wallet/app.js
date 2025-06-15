// At the top of app.js
const { ApiPromise, WsProvider } = window.PolkadotApi;

// Make the API instance available globally for other functions
window.critterApi = null;
const ALICE_ADDRESS = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'; // Example: Alice's address
const MAX_EVENTS_DISPLAYED = 10; // Max number of events to show

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
            let formattedBalance;
            const free = balance.free.toBigInt();

            if (chainDecimals === 0) {
                formattedBalance = free.toString();
            } else {
                const base = BigInt(10) ** BigInt(chainDecimals);
                const intPart = free / base;
                const fracPart = free % base;
                if (fracPart.toString() === '0') {
                    formattedBalance = intPart.toString();
                } else {
                    const fracString = fracPart.toString().padStart(chainDecimals, '0');
                    formattedBalance = `${intPart}.${fracString.substring(0, 4)}`;
                }
            }
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

    if (!api || !accountAddress) {
        if (nftListPlaceholder) nftListPlaceholder.textContent = 'N/A - API or Account not available';
        if (nftListUl) nftListUl.innerHTML = '';
        return;
    }

    try {
        if (nftListPlaceholder) {
            nftListPlaceholder.textContent = 'Loading Pet NFTs...';
            nftListPlaceholder.style.display = 'block';
        }
        if (nftListUl) nftListUl.innerHTML = '';

        const ownedPetIdsVec = await api.query.critterNftsPallet.ownerOfPet(accountAddress);
        const ownedPetIds = ownedPetIdsVec.isSome ? ownedPetIdsVec.unwrap() : [];

        if (ownedPetIds.length === 0) {
            if (nftListPlaceholder) {
                nftListPlaceholder.textContent = 'You do not own any Pet NFTs yet.';
                nftListPlaceholder.style.display = 'block';
            }
            return;
        }

        if (nftListPlaceholder) nftListPlaceholder.style.display = 'none';

        for (const petId of ownedPetIds) {
            const petNftOpt = await api.query.critterNftsPallet.petNfts(petId.toNumber());
            if (petNftOpt.isSome) {
                const petNft = petNftOpt.unwrap();
                const listItem = document.createElement('li');
                const petName = api.registry.createType('Text', petNft.current_pet_name).toString();
                const petSpecies = api.registry.createType('Text', petNft.initial_species).toString();
                listItem.innerHTML = `
                    <strong>ID:</strong> ${petNft.id.toNumber()} <br>
                    <strong>Name:</strong> ${petName} <br>
                    <strong>Species:</strong> ${petSpecies} <br>
                    <strong>Level:</strong> ${petNft.level.toNumber()} <br>
                    <strong>Mood:</strong> ${petNft.mood_indicator.toNumber()} |
                    <strong>Hunger:</strong> ${petNft.hunger_status.toNumber()} |
                    <strong>Energy:</strong> ${petNft.energy_status.toNumber()}
                    <hr>
                `;
                if (nftListUl) nftListUl.appendChild(listItem);
            } else {
                console.warn(`Could not find details for Pet ID: ${petId}`);
            }
        }
    } catch (error) {
        if (nftListPlaceholder) {
            nftListPlaceholder.textContent = `Error fetching Pet NFTs: ${error.message}`;
            nftListPlaceholder.style.display = 'block';
        }
        console.error('Error fetching Pet NFTs:', error);
    }
}

async function subscribeToNftEvents(api) {
    const eventListUl = document.getElementById('event-list');
    const eventListPlaceholder = document.getElementById('event-list-placeholder');

    if (!api) {
        if (eventListPlaceholder) eventListPlaceholder.textContent = 'N/A - API not available to subscribe to events.';
        return;
    }

    if (eventListPlaceholder) eventListPlaceholder.textContent = 'Subscribed to NFT events. Waiting for activity...';

    await api.query.system.events((events) => {
        if (events.length === 0) return;

        if (eventListPlaceholder) eventListPlaceholder.style.display = 'none';

        events.forEach((record) => {
            const { event, phase } = record;
            // const types = event.typeDef; // Not strictly needed for current decoding

            if (event.section === 'critterNftsPallet') {
                let eventString = `Block #${phase.isApplyExtrinsic ? phase.asApplyExtrinsic.toString() : '-'}: `;

                if (event.method === 'PetNftMinted') {
                    const owner = event.data[0].toString();
                    const pet_id = event.data[1].toString();
                    eventString += `Pet NFT Minted! ID: ${pet_id}, Owner: ${owner.substring(0, 8)}...${owner.substring(owner.length - 4)}`;
                } else if (event.method === 'PetNftTransferred') {
                    const from = event.data[0].toString();
                    const to = event.data[1].toString();
                    const pet_id = event.data[2].toString();
                    eventString += `Pet NFT Transferred! ID: ${pet_id}, From: ${from.substring(0,8)}...${from.substring(from.length - 4)}, To: ${to.substring(0,8)}...${to.substring(to.length-4)}`;
                } else {
                    return;
                }

                const listItem = document.createElement('li');
                listItem.textContent = eventString;
                if (eventListUl) eventListUl.insertBefore(listItem, eventListUl.firstChild);

                while (eventListUl && eventListUl.children.length > MAX_EVENTS_DISPLAYED) {
                    if (eventListUl.lastChild.tagName === 'LI') { // Ensure not to remove placeholder if it's the only child
                        eventListUl.removeChild(eventListUl.lastChild);
                    } else {
                        break; // Should not happen if placeholder is managed correctly
                    }
                }
            }
        });
    });
}

async function connectToCritterChain() {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const statusDiv = document.getElementById('connection-status');
    const actionsDiv = document.getElementById('actions');
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');
    const nftListUl = document.getElementById('nft-list');
    const eventListPlaceholder = document.getElementById('event-list-placeholder');
    const eventListUl = document.getElementById('event-list');

    try {
        const api = await ApiPromise.create({ provider });
        const chain = await api.rpc.system.chain();
        const nodeName = await api.rpc.system.name();
        const nodeVersion = await api.rpc.system.version();

        if (statusDiv) {
            statusDiv.innerHTML = `Successfully connected to chain: <strong>${chain}</strong> using <strong>${nodeName}</strong> (v${nodeVersion})`;
            statusDiv.style.color = 'green';
        }
        console.log(`Successfully connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
        window.critterApi = api;

        if (actionsDiv) actionsDiv.style.display = 'block';
        displayAccountBalance(api, ALICE_ADDRESS);
        displayOwnedNfts(api, ALICE_ADDRESS);
        subscribeToNftEvents(api); // Subscribe to events

    } catch (error) {
        if (statusDiv) {
            statusDiv.innerHTML = `Error connecting to CritterChain: ${error.message}`;
            statusDiv.style.color = 'red';
        }
        console.error('Connection error:', error);
        if (actionsDiv) actionsDiv.style.display = 'none';
        displayAccountBalance(null, ALICE_ADDRESS);
        if (nftListPlaceholder) {
            nftListPlaceholder.textContent = 'Connection failed. Cannot load NFTs.';
            nftListPlaceholder.style.display = 'block';
        }
        if (nftListUl) nftListUl.innerHTML = '';
        if (eventListPlaceholder) {
            eventListPlaceholder.textContent = 'Connection failed. Cannot load events.';
            eventListPlaceholder.style.display = 'block';
        }
        if (eventListUl) eventListUl.innerHTML = '';
    }
}

document.addEventListener('DOMContentLoaded', () => {
    console.log("CritterCraft Wallet UI Initializing");

    const actionsDiv = document.getElementById('actions');
    if (actionsDiv) actionsDiv.style.display = 'none';

    const walletAddressSpan = document.getElementById('wallet-address');
    const ptcnBalanceSpan = document.getElementById('ptcn-balance');
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');
    const nftListUl = document.getElementById('nft-list');
    const eventListPlaceholder = document.getElementById('event-list-placeholder');
    const eventListUl = document.getElementById('event-list');


    if (walletAddressSpan) walletAddressSpan.textContent = 'Loading...';
    if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = 'Loading...';
    if (nftListPlaceholder) {
        nftListPlaceholder.textContent = 'Loading Pet NFTs...';
        nftListPlaceholder.style.display = 'block';
    }
    if (nftListUl) nftListUl.innerHTML = '';
    if (eventListPlaceholder) {
        eventListPlaceholder.textContent = 'No recent activity or loading events...';
        eventListPlaceholder.style.display = 'block';
    }
    if (eventListUl) eventListUl.innerHTML = '';


    connectToCritterChain();

    // Mint NFT Button Logic
    const mintNftButton = document.getElementById('mintNftButton');
    const petSpeciesInput = document.getElementById('pet-species');
    const petNameInput = document.getElementById('pet-name');
    const mintStatusP = document.getElementById('mint-status');

    if (mintNftButton) {
        mintNftButton.addEventListener('click', async () => {
            // ... (minting logic as before)
            if (!window.critterApi) { mintStatusP.textContent = 'Error: Not connected.'; mintStatusP.style.color = 'red'; return; }
            if (!ALICE_ADDRESS) { mintStatusP.textContent = 'Error: No account.'; mintStatusP.style.color = 'red'; return; }
            const species = petSpeciesInput.value.trim();
            const name = petNameInput.value.trim();
            if (!species || !name) { mintStatusP.textContent = 'Error: Species/Name empty.'; mintStatusP.style.color = 'red'; return; }
            mintStatusP.textContent = 'Preparing...'; mintStatusP.style.color = 'orange';
            try {
                const mintTx = window.critterApi.tx.critterNftsPallet.mintPetNft(species, name);
                mintStatusP.textContent = `Mock Submit: Mint ${name} (${species}). Check console.`; mintStatusP.style.color = 'blue';
                console.log(`Tx to send: critterNftsPallet.mintPetNft('${species}', '${name}') for ${ALICE_ADDRESS}`);
                setTimeout(() => {
                    if (window.critterApi) {
                         displayOwnedNfts(window.critterApi, ALICE_ADDRESS);
                         mintStatusP.textContent = "NFT list refreshed (simulated)."; mintStatusP.style.color = "green";
                    }
                }, 3000);
            } catch (e) { mintStatusP.textContent = `Error: ${e.message}`; mintStatusP.style.color = 'red'; console.error(e); }
        });
    }

    // Transfer NFT Button Logic
    const transferNftButton = document.getElementById('transferNftButton');
    const recipientAddressInput = document.getElementById('recipient-address');
    const petIdTransferInput = document.getElementById('pet-id-transfer');
    const transferStatusP = document.getElementById('transfer-status');

    if (transferNftButton) {
        transferNftButton.addEventListener('click', async () => {
            // ... (transfer logic as before)
            if (!window.critterApi) { transferStatusP.textContent = 'Error: Not connected.'; transferStatusP.style.color = 'red'; return; }
            if (!ALICE_ADDRESS) { transferStatusP.textContent = 'Error: No sender account.'; transferStatusP.style.color = 'red'; return; }
            const recipientAddress = recipientAddressInput.value.trim();
            const petIdString = petIdTransferInput.value.trim();
            if (!recipientAddress || !petIdString) { transferStatusP.textContent = 'Error: Recipient/PetID empty.'; transferStatusP.style.color = 'red'; return; }
            const petId = parseInt(petIdString);
            if (isNaN(petId) || petId < 0) { transferStatusP.textContent = 'Error: Invalid Pet ID.'; transferStatusP.style.color = 'red'; return; }
            transferStatusP.textContent = 'Preparing...'; transferStatusP.style.color = 'orange';
            try {
                const transferTx = window.critterApi.tx.critterNftsPallet.transferPetNft(recipientAddress, petId);
                transferStatusP.textContent = `Mock Submit: Transfer ID ${petId} to ${recipientAddress}. Check console.`; transferStatusP.style.color = 'blue';
                console.log(`Tx to send: critterNftsPallet.transferPetNft('${recipientAddress}', ${petId}) from ${ALICE_ADDRESS}`);
                setTimeout(() => {
                    if (window.critterApi) {
                        displayOwnedNfts(window.critterApi, ALICE_ADDRESS);
                        transferStatusP.textContent = "NFT list refreshed (simulated)."; transferStatusP.style.color = "green";
                    }
                }, 3000);
            } catch (e) { transferStatusP.textContent = `Error: ${e.message}`; transferStatusP.style.color = 'red'; console.error(e); }
        });
    }
});
