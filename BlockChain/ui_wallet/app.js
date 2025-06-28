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
        // Ensure previous subscription is cancelled if any, or that this won't create duplicates
        if (window.balanceUnsubscribe) window.balanceUnsubscribe();
        window.balanceUnsubscribe = await api.query.system.account(accountAddress, ({ data: balance }) => {
            const chainDecimals = api.registry.chainDecimals[0] || 18;
            if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = `${formatDisplayBalance(balance.free.toBigInt(), chainDecimals)} PTCN`;
        });
    } catch (e) { if (ptcnBalanceSpan) ptcnBalanceSpan.textContent = `Error`; console.error(e); }
}
async function populateBattlePetSelect(api, accountAddress) { /* ... existing ... */ }
async function displayOwnedNfts(api, accountAddress) { /* ... existing ... */ }
async function subscribeToSystemEvents(api) { /* ... existing ... */ }
async function displayMarketplaceListings(api) { /* ... existing ... */ }
async function getPalletConstants(api) { /* ... existing ... */ }
async function displayNextClaimTime(api, accountAddress) { /* ... existing ... */ }
async function displayCurrentBattles(api) { /* ... existing ... */ }
async function displayAvailableQuests(api) { /* ... existing ... */ }
async function displayCompletedQuests(api, accountAddress) { /* ... existing ... */ }

// ---- Staking UI Functions ----
async function displayMyStakingInfo(api, accountAddress) {
    const myNominationsSpan = document.getElementById('my-nominations');
    const myStakedAmountSpan = document.getElementById('my-staked-amount');
    const claimableRewardsSpan = document.getElementById('claimable-rewards');
    const claimRewardsButton = document.getElementById('claimRewardsButton');
    const unbondingChunksList = document.getElementById('unbonding-chunks-list');
    const unbondingChunksPlaceholder = document.getElementById('unbonding-chunks-placeholder');
    const withdrawUnbondedButton = document.getElementById('withdrawUnbondedButton');

    if (!api || !accountAddress) {
        if(myNominationsSpan) myNominationsSpan.textContent = 'N/A';
        if(myStakedAmountSpan) myStakedAmountSpan.textContent = 'N/A';
        if(claimableRewardsSpan) claimableRewardsSpan.textContent = 'N/A';
        if(claimRewardsButton) claimRewardsButton.disabled = true;
        if(unbondingChunksPlaceholder) unbondingChunksPlaceholder.textContent = 'N/A - API/Account Error';
        if(unbondingChunksList) unbondingChunksList.innerHTML = '';
        if(withdrawUnbondedButton) withdrawUnbondedButton.disabled = true;
        return;
    }

    myNominationsSpan.textContent = 'Loading...';
    myStakedAmountSpan.textContent = 'Loading...';
    claimableRewardsSpan.textContent = "N/A (Use validator payout)"; // Simplified
    claimRewardsButton.disabled = true; // Or enable if we have a mock claim action
    unbondingChunksPlaceholder.textContent = 'Loading unbonding info...';
    if (unbondingChunksList) unbondingChunksList.innerHTML = '';
    if (withdrawUnbondedButton) withdrawUnbondedButton.disabled = true;


    try {
        const nominatorsData = await api.query.staking.nominators(accountAddress);
        if (nominatorsData.isSome) {
            const nominations = nominatorsData.unwrap();
            const targets = nominations.targets.map(t => t.toString().substring(0, 8) + '...').join(', ');
            myNominationsSpan.textContent = targets.length ? targets : 'None';
        } else {
            myNominationsSpan.textContent = 'Not nominating anyone.';
        }

        const ledgerData = await api.query.staking.ledger(accountAddress);
        if (ledgerData.isSome) {
            const ledger = ledgerData.unwrap();
            myStakedAmountSpan.textContent = `${formatDisplayBalance(ledger.active.toBigInt(), api.registry.chainDecimals[0] || 18)} PTCN`;

            if (ledger.unlocking && ledger.unlocking.length > 0) {
                if (unbondingChunksPlaceholder) unbondingChunksPlaceholder.style.display = 'none';
                if (unbondingChunksList) unbondingChunksList.innerHTML = ''; // Clear before repopulating
                let canWithdraw = false;
                const currentEraOpt = await api.query.staking.currentEra();
                const currentEra = currentEraOpt.isSome ? currentEraOpt.unwrap().toNumber() : 0;

                ledger.unlocking.forEach(chunk => {
                    const listItem = document.createElement('li');
                    listItem.textContent = `Amount: ${formatDisplayBalance(chunk.value.toBigInt(), api.registry.chainDecimals[0] || 18)} PTCN, Unlock Era: ${chunk.era.toNumber()}`;
                    if (unbondingChunksList) unbondingChunksList.appendChild(listItem);
                    if (currentEra >= chunk.era.toNumber()) {
                        canWithdraw = true;
                    }
                });
                if (withdrawUnbondedButton) withdrawUnbondedButton.disabled = !canWithdraw;
            } else {
                if (unbondingChunksPlaceholder) {
                    unbondingChunksPlaceholder.style.display = 'block';
                    unbondingChunksPlaceholder.textContent = 'No PTCN currently unbonding.';
                }
                if (unbondingChunksList) unbondingChunksList.innerHTML = '';
                if (withdrawUnbondedButton) withdrawUnbondedButton.disabled = true;
            }

        } else {
            myStakedAmountSpan.textContent = '0 PTCN (No staking ledger found)';
            if (unbondingChunksPlaceholder) {
                unbondingChunksPlaceholder.style.display = 'block';
                unbondingChunksPlaceholder.textContent = 'No PTCN currently unbonding.';
            }
            if (unbondingChunksList) unbondingChunksList.innerHTML = '';
            if (withdrawUnbondedButton) withdrawUnbondedButton.disabled = true;
        }
    } catch (e) {
        console.error("Error fetching staking info:", e);
        if(myNominationsSpan) myNominationsSpan.textContent = 'Error';
        if(myStakedAmountSpan) myStakedAmountSpan.textContent = 'Error';
        if(unbondingChunksPlaceholder) unbondingChunksPlaceholder.textContent = 'Error loading unbonding info.';
    }
}

async function displayValidators(api) { /* ... existing condensed ... */ }


async function connectToCritterChain() {
    // ... (existing setup)
    const statusDiv = document.getElementById('connection-status');
    const actionsDiv = document.getElementById('actions');
    const myNominationsSpan = document.getElementById('my-nominations');
    const myStakedAmountSpan = document.getElementById('my-staked-amount');
    const claimableRewardsSpan = document.getElementById('claimable-rewards');
    const withdrawUnbondedButton = document.getElementById('withdrawUnbondedButton');
    const unbondingChunksPlaceholder = document.getElementById('unbonding-chunks-placeholder');


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
        displayAvailableQuests(api);
        displayCompletedQuests(api, userAddr);
        displayMyStakingInfo(api, userAddr);
        displayValidators(api);
    } catch (error) {
        // ... (existing error handling for other sections)
        if (myNominationsSpan) myNominationsSpan.textContent = 'Connection failed.';
        if (myStakedAmountSpan) myStakedAmountSpan.textContent = 'Connection failed.';
        if (claimableRewardsSpan) claimableRewardsSpan.textContent = 'Connection failed.';
        if (withdrawUnbondedButton) withdrawUnbondedButton.disabled = true;
        if (unbondingChunksPlaceholder) unbondingChunksPlaceholder.textContent = 'Connection failed.';
    }
}

document.addEventListener('DOMContentLoaded', () => {
    // ... (existing initializations for other sections)
    const myNominationsSpan = document.getElementById('my-nominations');
    if (myNominationsSpan) myNominationsSpan.textContent = 'Loading...';
    const myStakedAmountSpan = document.getElementById('my-staked-amount');
    if (myStakedAmountSpan) myStakedAmountSpan.textContent = 'Loading...';
    const claimableRewardsSpan = document.getElementById('claimable-rewards');
    if (claimableRewardsSpan) claimableRewardsSpan.textContent = 'Loading...';
    const claimRewardsButton = document.getElementById('claimRewardsButton');
    if (claimRewardsButton) claimRewardsButton.disabled = true;
    const unbondingChunksPlaceholder = document.getElementById('unbonding-chunks-placeholder');
    if (unbondingChunksPlaceholder) unbondingChunksPlaceholder.textContent = 'No PTCN currently unbonding.';
    const unbondingChunksList = document.getElementById('unbonding-chunks-list');
    if (unbondingChunksList) unbondingChunksList.innerHTML = '';
    const withdrawUnbondedButton = document.getElementById('withdrawUnbondedButton');
    if (withdrawUnbondedButton) withdrawUnbondedButton.disabled = true;

    connectToCritterChain();

    // ... (Existing button logics: condensed for brevity) ...

    // Claim Rewards Button (Simulated)
    const claimStakingRewardsButton = document.getElementById('claimRewardsButton'); // Corrected ID
    if (claimStakingRewardsButton) {
        claimStakingRewardsButton.addEventListener('click', () => {
            const statusP = document.getElementById('staking-action-status');
            if(statusP) {
                statusP.textContent = "Mock: Validator payout for rewards should be triggered. No direct user claim extrinsic in basic staking.";
                statusP.style.color = 'blue';
            } else {
                alert("Mock: Validator payout for rewards should be triggered.");
            }
        });
    }

    // Unbond Button
    const unbondButton = document.getElementById('unbondButton');
    const unbondAmountInput = document.getElementById('unbond-amount');
    const unbondStatusP = document.getElementById('unbond-status');

    if (unbondButton) {
        unbondButton.addEventListener('click', async () => {
            if (!window.critterApi || !ALICE_ADDRESS) { unbondStatusP.textContent = 'API not ready/No account'; unbondStatusP.style.color = 'red'; return; }
            const amountStr = unbondAmountInput.value.trim();
            if (!amountStr) { unbondStatusP.textContent = 'Unbond amount required.'; unbondStatusP.style.color = 'red'; return; }

            let amountInSmallestUnit;
            try {
                const decimals = window.critterApi.registry.chainDecimals[0] || 18;
                const amountFloat = parseFloat(amountStr);
                if (isNaN(amountFloat) || amountFloat <= 0) throw new Error("Invalid amount");
                amountInSmallestUnit = BigInt(Math.round(amountFloat * (10**decimals)));
            } catch(e) {
                unbondStatusP.textContent = 'Invalid unbond amount.'; unbondStatusP.style.color = 'red'; return;
            }

            unbondStatusP.textContent = 'Preparing to unbond...';
            unbondStatusP.style.color = 'orange';
            unbondButton.disabled = true;
            try {
                const unbondTx = window.critterApi.tx.staking.unbond(amountInSmallestUnit.toString());
                console.log(`Tx: staking.unbond(${amountInSmallestUnit}) by ${ALICE_ADDRESS}`);
                unbondStatusP.textContent = `Mock Submission: Unbonding ${amountStr} PTCN.`;
                unbondStatusP.style.color = 'blue';
                setTimeout(async () => {
                    if(window.critterApi) await displayMyStakingInfo(window.critterApi, ALICE_ADDRESS);
                    unbondStatusP.textContent = `Unbond (mock) for ${amountStr} PTCN submitted. Staking info refreshed.`;
                    unbondStatusP.style.color = 'green';
                    unbondButton.disabled = false;
                    unbondAmountInput.value = ''; // Clear input
                }, 3000);
            } catch (e) {
                unbondStatusP.textContent = `Error unbonding: ${e.message}`; unbondStatusP.style.color = 'red';
                unbondButton.disabled = false;
                console.error("Unbond Error:", e);
            }
        });
    }

    // Withdraw Unbonded Button
    const finalWithdrawUnbondedButton = document.getElementById('withdrawUnbondedButton'); // Corrected ID
    if (finalWithdrawUnbondedButton) {
        finalWithdrawUnbondedButton.addEventListener('click', async () => {
            if (!window.critterApi || !ALICE_ADDRESS) { unbondStatusP.textContent = 'API not ready/No account'; unbondStatusP.style.color = 'red'; return; }

            unbondStatusP.textContent = 'Preparing to withdraw unbonded...';
            unbondStatusP.style.color = 'orange';
            finalWithdrawUnbondedButton.disabled = true;
            try {
                const numSlashingSpans = 0; // Simplification for MVP

                const withdrawTx = window.critterApi.tx.staking.withdrawUnbonded(numSlashingSpans);
                console.log(`Tx: staking.withdrawUnbonded(${numSlashingSpans}) by ${ALICE_ADDRESS}`);
                unbondStatusP.textContent = `Mock Submission: Withdrawing unbonded PTCN.`;
                unbondStatusP.style.color = 'blue';
                setTimeout(async () => {
                    if(window.critterApi) {
                        await displayMyStakingInfo(window.critterApi, ALICE_ADDRESS);
                        await displayAccountBalance(window.critterApi, ALICE_ADDRESS); // Refresh balance
                    }
                    unbondStatusP.textContent = `Withdraw unbonded (mock) submitted. Staking info and balance refreshed.`;
                    unbondStatusP.style.color = 'green';
                    // Button state will be updated by displayMyStakingInfo
                }, 3000);
            } catch (e) {
                unbondStatusP.textContent = `Error withdrawing: ${e.message}`; unbondStatusP.style.color = 'red';
                finalWithdrawUnbondedButton.disabled = false; // Re-enable on error, displayMyStakingInfo will manage final state
                console.error("Withdraw Error:", e);
            }
        });
    }

    // Event listener for "Nominate" buttons (condensed)
    const validatorListActionUl = document.getElementById('validator-list');
    if (validatorListActionUl) validatorListActionUl.addEventListener('click', async (event) => { if (event.target.classList.contains('nominate-button')) { /* ... */ } });
});
