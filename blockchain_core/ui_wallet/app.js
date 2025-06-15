// At the top of app.js
const { ApiPromise, WsProvider } = window.PolkadotApi;

// Make the API instance available globally for other functions
window.critterApi = null;
const ALICE_ADDRESS = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'; // Example: Alice's address
const MAX_EVENTS_DISPLAYED = 10; // Max number of events to show

let CLAIM_COOLDOWN_PERIOD_BLOCKS = 100; // Default/Placeholder
let DAILY_CLAIM_AMOUNT = BigInt("10000000000000000000"); // Default/Placeholder: 10 PTCN

// Simplified balance formatting function
function formatDisplayBalance(balanceBigInt, decimals) {
    const balanceStr = balanceBigInt.toString();
    let CENTS_PLACES = 4;
    if (decimals === 0) return balanceStr;
    let intPart = balanceStr.length > decimals ? balanceStr.substring(0, balanceStr.length - decimals) : '0';
    let fracPart = balanceStr.length > decimals ? balanceStr.substring(balanceStr.length - decimals) : '0'.repeat(decimals - balanceStr.length) + balanceStr;
    return `${intPart}.${fracPart.substring(0, CENTS_PLACES)}`;
}

async function displayAccountBalance(api, accountAddress) { /* ... existing ... */
    const ptcnBalanceSpan = document.getElementById('ptcn-balance');
    if (!api || !accountAddress) { if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = 'N/A'; return; }
    try {
        await api.query.system.account(accountAddress, ({ data: balance }) => {
            const chainDecimals = api.registry.chainDecimals[0] || 18;
            if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = `${formatDisplayBalance(balance.free.toBigInt(), chainDecimals)} PTCN`;
        });
    } catch (e) { if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = `Error`; console.error(e); }
}
async function populateBattlePetSelect(api, accountAddress) { /* ... existing ... */
    const battlePetSelect = document.getElementById('battle-pet-id-select');
    if (!battlePetSelect || !api || !accountAddress) return;
    battlePetSelect.innerHTML = '<option value="">--Select your pet--</option>';
    try {
        const ownedPetIdsVec = await api.query.critterNftsPallet.ownerOfPet(accountAddress);
        const ownedPetIds = ownedPetIdsVec.isSome ? ownedPetIdsVec.unwrap() : [];
        if (ownedPetIds.length === 0) { battlePetSelect.innerHTML = '<option value="">--No pets available--</option>'; return; }
        for (const petId of ownedPetIds) {
            const petNftOpt = await api.query.critterNftsPallet.petNfts(petId.toNumber());
            if (petNftOpt.isSome) {
                const petNft = petNftOpt.unwrap();
                const petName = api.registry.createType('Text', petNft.current_pet_name).toString();
                const option = document.createElement('option');
                option.value = petNft.id.toNumber();
                option.textContent = `${petName} (ID: ${petNft.id.toNumber()}, Lvl: ${petNft.level.toNumber()})`;
                battlePetSelect.appendChild(option);
            }
        }
    } catch (e) { console.error("Error populating battle pet select:", e); battlePetSelect.innerHTML = '<option value="">--Error loading pets--</option>'; }
}
async function displayOwnedNfts(api, accountAddress) { /* ... existing ... */
    const nftListUl = document.getElementById('nft-list');
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');
    if (!api || !accountAddress) { if (nftListPlaceholder) nftListPlaceholder.textContent = 'N/A'; return; }
    try {
        if (nftListPlaceholder) { nftListPlaceholder.textContent = 'Loading...'; nftListPlaceholder.style.display = 'block'; }
        if (nftListUl) nftListUl.innerHTML = '';
        const ownedPetIdsVec = await api.query.critterNftsPallet.ownerOfPet(accountAddress);
        const ownedPetIds = ownedPetIdsVec.isSome ? ownedPetIdsVec.unwrap() : [];
        if (ownedPetIds.length === 0) { if (nftListPlaceholder) nftListPlaceholder.textContent = 'You do not own any Pet NFTs.'; return; }
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
                listItem.innerHTML = `<strong>ID:</strong> ${petNft.id.toNumber()} | <strong>Name:</strong> ${petName} | <strong>Species:</strong> ${petSpecies} <br><strong>Lvl:</strong> ${petNft.level.toNumber()} | <strong>XP:</strong> ${petNft.experience_points.toNumber()} | <strong>Mood:</strong> ${petNft.mood_indicator.toNumber()} | <strong>Hunger:</strong> ${petNft.hunger_status.toNumber()} | <strong>Energy:</strong> ${petNft.energy_status.toNumber()} <br><strong>DNA:</strong> ${dnaHash.substring(0,10)}... | <strong>Personality:</strong> ${personalityTraits}<hr>`;
                if (nftListUl) nftListUl.appendChild(listItem);
            }
        }
    } catch (e) { if (nftListPlaceholder) nftListPlaceholder.textContent = `Error`; console.error(e); }
}
async function subscribeToSystemEvents(api) { /* ... existing, now includes questsPallet ... */
    const eventListUl = document.getElementById('event-list');
    const eventListPlaceholder = document.getElementById('event-list-placeholder');
    if (!api) { if (eventListPlaceholder) eventListPlaceholder.textContent = 'N/A - API Error'; return; }
    if (eventListPlaceholder) eventListPlaceholder.textContent = 'Subscribed to events...';
    await api.query.system.events((events) => {
        if (events.length === 0) return;
        if (eventListPlaceholder) eventListPlaceholder.style.display = 'none';
        events.forEach((record) => {
            const { event, phase } = record;
            const palletName = event.section;
            const eventMethod = event.method;
            if (palletName === 'critterNftsPallet' || palletName === 'marketplacePallet' || palletName === 'battlesPallet' || palletName === 'questsPallet') {
                let eventString = `Block #${phase.isApplyExtrinsic ? phase.asApplyExtrinsic.toString() : '-'}: [${palletName}] `;
                if (eventMethod === 'PetNftMinted') eventString += `NFT Minted! ID: ${event.data[1]}, Owner: ${event.data[0].toString().substring(0,8)}...`;
                else if (eventMethod === 'PetNftTransferred') eventString += `NFT Transferred! ID: ${event.data[2]}, From: ${event.data[0].toString().substring(0,8)}..., To: ${event.data[1].toString().substring(0,8)}...`;
                else if (eventMethod === 'PetNftMetadataUpdated') eventString += `NFT Meta Updated! ID: ${event.data[1]}, Owner: ${event.data[0].toString().substring(0,8)}...`;
                else if (eventMethod === 'DailyClaimMade') eventString += `Daily Claim! Acc: ${event.data[0].toString().substring(0,8)}..., Amt: ${formatDisplayBalance(event.data[1].toBigInt(), api.registry.chainDecimals[0]||18)} PTCN`;
                else if (eventMethod === 'NftListed') eventString += `NFT Listed! Seller: ${event.data[0].toString().substring(0,8)}..., PetID: ${event.data[1]}, Price: ${formatDisplayBalance(event.data[2].toBigInt(), api.registry.chainDecimals[0]||18)} PTCN`;
                else if (eventMethod === 'NftUnlisted') eventString += `NFT Unlisted! Seller: ${event.data[0].toString().substring(0,8)}..., PetID: ${event.data[1]}`;
                else if (eventMethod === 'NftSold') eventString += `NFT Sold! Buyer: ${event.data[0].toString().substring(0,8)}..., Seller: ${event.data[1].toString().substring(0,8)}..., PetID: ${event.data[2]}, Price: ${formatDisplayBalance(event.data[3].toBigInt(), api.registry.chainDecimals[0]||18)} PTCN`;
                else if (eventMethod === 'BattleRegistered') eventString += `Battle Registered! ID: ${event.data[0]}, Player: ${event.data[1].toString().substring(0,8)}..., Pet: ${event.data[2]}`;
                else if (eventMethod === 'BattleConcluded') eventString += `Battle Concluded! ID: ${event.data[0]}, Winner: ${event.data[1].isSome ? event.data[1].unwrap().toString().substring(0,8) : 'N/A'}...`;
                else if (eventMethod === 'QuestAdded') eventString += `Quest Added! ID: ${event.data[0]}, Reward: ${formatDisplayBalance(event.data[2].toBigInt(), api.registry.chainDecimals[0]||18)} PTCN`; // Assuming desc is event.data[1]
                else if (eventMethod === 'QuestCompleted') eventString += `Quest Completed! ID: ${event.data[0]}, Acc: ${event.data[1].toString().substring(0,8)}..., Reward: ${formatDisplayBalance(event.data[2].toBigInt(), api.registry.chainDecimals[0]||18)} PTCN`;
                else return;
                const listItem = document.createElement('li'); listItem.textContent = eventString;
                if (eventListUl) eventListUl.insertBefore(listItem, eventListUl.firstChild);
                while (eventListUl && eventListUl.children.length > MAX_EVENTS_DISPLAYED) { if (eventListUl.lastChild.tagName === 'LI') eventListUl.removeChild(eventListUl.lastChild); else break; }
            }
        });
    });
}
async function displayMarketplaceListings(api) { /* ... existing ... */ }
async function getPalletConstants(api) { /* ... existing ... */ }
async function displayNextClaimTime(api, accountAddress) { /* ... existing ... */ }
async function displayCurrentBattles(api) { /* ... existing ... */ }

async function displayAvailableQuests(api) {
    const availableQuestsUl = document.getElementById('available-quests-list');
    const availableQuestsPlaceholder = document.getElementById('available-quests-placeholder');
    if (!api) { if(availableQuestsPlaceholder) availableQuestsPlaceholder.textContent = 'N/A - API Error.'; return; }
    if(availableQuestsPlaceholder) availableQuestsPlaceholder.textContent = 'Loading available quests...';
    if(availableQuestsUl) availableQuestsUl.innerHTML = '';
    try {
        const questEntries = await api.query.questsPallet.availableQuests.entries();
        if (questEntries.length === 0) { if(availableQuestsPlaceholder) availableQuestsPlaceholder.textContent = 'No quests currently available.'; return; }
        if(availableQuestsPlaceholder) availableQuestsPlaceholder.style.display = 'none';
        for (const [key, quest] of questEntries) { // Quest struct is directly the value
            const questId = key.args[0].toNumber(); // Or .toString()
            const description = api.registry.createType('Text', quest.description).toString();
            const reward = formatDisplayBalance(quest.reward_ptcn.toBigInt(), api.registry.chainDecimals[0] || 18);
            const listItem = document.createElement('li');
            listItem.innerHTML = `<strong>Quest ID: ${questId}</strong><br>Description: ${description}<br>Reward: ${reward} PTCN<br><button class="complete-quest-button" data-quest-id="${questId}">Complete Quest</button><hr>`;
            if(availableQuestsUl) availableQuestsUl.appendChild(listItem);
        }
    } catch (e) { if(availableQuestsPlaceholder) availableQuestsPlaceholder.textContent = `Error: ${e.message}`; console.error(e); }
}

async function displayCompletedQuests(api, accountAddress) {
    const completedQuestsUl = document.getElementById('completed-quests-list');
    const completedQuestsPlaceholder = document.getElementById('completed-quests-placeholder');
    if (!api || !accountAddress) { if(completedQuestsPlaceholder) completedQuestsPlaceholder.textContent = 'N/A - API/Account Error.'; return; }
    if(completedQuestsPlaceholder) completedQuestsPlaceholder.textContent = 'Loading completed quests...';
    if(completedQuestsUl) completedQuestsUl.innerHTML = '';
    try {
        const allAvailableQuestEntries = await api.query.questsPallet.availableQuests.entries();
        let foundCompleted = false;
        for (const [key, quest] of allAvailableQuestEntries) {
            const questId = key.args[0].toNumber(); // or .toString()
            const isCompleted = await api.query.questsPallet.completedQuests([accountAddress, questId]);
            if (isCompleted.isSome) {
                foundCompleted = true;
                const description = api.registry.createType('Text', quest.description).toString();
                const reward = formatDisplayBalance(quest.reward_ptcn.toBigInt(), api.registry.chainDecimals[0] || 18);
                const listItem = document.createElement('li');
                listItem.innerHTML = `<strong>Quest ID: ${questId}</strong> (Completed)<br>Description: ${description}<br>Reward: ${reward} PTCN<hr>`;
                if(completedQuestsUl) completedQuestsUl.appendChild(listItem);
            }
        }
        if (!foundCompleted) { if(completedQuestsPlaceholder) completedQuestsPlaceholder.textContent = 'You have not completed any quests yet.';
        } else { if(completedQuestsPlaceholder) completedQuestsPlaceholder.style.display = 'none'; }
    } catch (e) { if(completedQuestsPlaceholder) completedQuestsPlaceholder.textContent = `Error: ${e.message}`; console.error(e); }
}


async function connectToCritterChain() {
    // ... (existing setup and element getters)
    const statusDiv = document.getElementById('connection-status');
    const actionsDiv = document.getElementById('actions');
    const availableQuestsPlaceholder = document.getElementById('available-quests-placeholder'); // Added
    const availableQuestsUl = document.getElementById('available-quests-list'); // Added
    const completedQuestsPlaceholder = document.getElementById('completed-quests-placeholder'); // Added
    const completedQuestsUl = document.getElementById('completed-quests-list'); // Added


    try {
        const api = await ApiPromise.create({ provider: new WsProvider('ws://127.0.0.1:9944') });
        window.critterApi = api;
        await getPalletConstants(api);
        // ... (set success message)
        if (actionsDiv) actionsDiv.style.display = 'block';
        const userAddr = ALICE_ADDRESS;
        document.getElementById('wallet-address').textContent = userAddr;
        displayAccountBalance(api, userAddr);
        displayOwnedNfts(api, userAddr);
        populateBattlePetSelect(api, userAddr);
        subscribeToSystemEvents(api);
        displayMarketplaceListings(api);
        displayNextClaimTime(api, userAddr);
        displayCurrentBattles(api);
        displayAvailableQuests(api); // Call new function
        displayCompletedQuests(api, userAddr); // Call new function
    } catch (error) {
        // ... (existing error handling)
        if (availableQuestsPlaceholder) availableQuestsPlaceholder.textContent = 'Connection failed.';
        if (availableQuestsUl) availableQuestsUl.innerHTML = '';
        if (completedQuestsPlaceholder) completedQuestsPlaceholder.textContent = 'Connection failed.';
        if (completedQuestsUl) completedQuestsUl.innerHTML = '';
    }
}

document.addEventListener('DOMContentLoaded', () => {
    // ... (existing initializations)
    const availableQuestsPlaceholder = document.getElementById('available-quests-placeholder');
    if (availableQuestsPlaceholder) { availableQuestsPlaceholder.textContent = 'Loading available quests...'; availableQuestsPlaceholder.style.display = 'block';}
    const availableQuestsUl = document.getElementById('available-quests-list');
    if (availableQuestsUl) availableQuestsUl.innerHTML = '';
    const completedQuestsPlaceholder = document.getElementById('completed-quests-placeholder');
    if (completedQuestsPlaceholder) { completedQuestsPlaceholder.textContent = 'Loading completed quests...'; completedQuestsPlaceholder.style.display = 'block';}
    const completedQuestsUl = document.getElementById('completed-quests-list');
    if (completedQuestsUl) completedQuestsUl.innerHTML = '';

    connectToCritterChain();

    // ... (Existing button logics: Daily Claim, Mint, Transfer, Update, List, Unlist, Buy NFT, Register Battle, Simulate Battle) ...
    // Condensed for brevity

    // Event listener for "Complete Quest" buttons
    const availableQuestsListUl = document.getElementById('available-quests-list'); // Renamed for clarity
    const questActionStatusP = document.getElementById('quest-action-status');

    if (availableQuestsListUl) {
        availableQuestsListUl.addEventListener('click', async (event) => {
            if (event.target.classList.contains('complete-quest-button')) {
                if (!window.critterApi || !ALICE_ADDRESS) { questActionStatusP.textContent = 'API not ready / No account'; return; }
                const questId = parseInt(event.target.dataset.questId);
                questActionStatusP.textContent = `Attempting to complete Quest ID ${questId}...`; questActionStatusP.style.color = 'orange';
                event.target.disabled = true; event.target.textContent = 'Completing...';
                try {
                    const completeTx = window.critterApi.tx.questsPallet.completeQuest(questId);
                    console.log(`Tx: questsPallet.completeQuest(${questId}) by ${ALICE_ADDRESS}`);
                    questActionStatusP.textContent = `Mock Submission: Completing Quest ID ${questId}.`; questActionStatusP.style.color = 'blue';
                    setTimeout(async () => {
                        if(window.critterApi) {
                            await displayAvailableQuests(window.critterApi);
                            await displayCompletedQuests(window.critterApi, ALICE_ADDRESS);
                            // Balance should auto-update
                        }
                        questActionStatusP.textContent = `Quest ${questId} completion (mock) submitted. Lists refreshed.`; questActionStatusP.style.color = 'green';
                        // Button is gone or re-rendered by displayAvailableQuests
                    }, 3000);
                } catch (e) {
                    questActionStatusP.textContent = `Error completing quest: ${e.message}`; questActionStatusP.style.color = 'red';
                    event.target.disabled = false; event.target.textContent = 'Complete Quest';
                    console.error("Complete Quest Error:", e);
                }
            }
        });
    }
});
