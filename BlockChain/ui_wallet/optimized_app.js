/**
 * CritterCraft Wallet - Optimized Application
 * 
 * This file contains the optimized JavaScript code for the CritterCraft wallet UI.
 * It includes performance improvements, better error handling, and a more modular structure.
 */

// Use strict mode for better error catching and performance
'use strict';

// Import Polkadot API from the window object
const { ApiPromise, WsProvider } = window.PolkadotApi;

// Application state
const AppState = {
  api: null,
  currentAccount: null,
  subscriptions: {
    balance: null,
    events: null,
    nfts: null,
    marketplace: null,
    battles: null,
    quests: null,
    daycare: null
  },
  constants: {
    claimCooldownPeriodBlocks: 100, // Default/Placeholder
    dailyClaimAmount: BigInt("10000000000000000000"), // Default/Placeholder: 10 PTCN
    chainDecimals: 18
  },
  cache: {
    ownedNfts: [],
    marketplaceListings: [],
    currentBattles: [],
    availableQuests: [],
    completedQuests: [],
    lastClaimBlock: 0,
    nextClaimBlock: 0
  }
};

// ===== Utility Functions =====

/**
 * Format a balance for display.
 * 
 * @param {BigInt} balanceBigInt - The balance as a BigInt.
 * @param {number} decimals - The number of decimal places.
 * @param {number} displayDecimals - The number of decimal places to display.
 * @returns {string} The formatted balance.
 */
function formatDisplayBalance(balanceBigInt, decimals, displayDecimals = 4) {
  const balanceStr = balanceBigInt.toString();
  
  if (decimals === 0) return balanceStr;
  
  let intPart = balanceStr.length > decimals 
    ? balanceStr.substring(0, balanceStr.length - decimals) 
    : '0';
    
  let fracPart = balanceStr.length > decimals 
    ? balanceStr.substring(balanceStr.length - decimals) 
    : '0'.repeat(decimals - balanceStr.length) + balanceStr;
    
  return `${intPart}.${fracPart.substring(0, displayDecimals)}`;
}

/**
 * Show a notification to the user.
 * 
 * @param {string} message - The message to display.
 * @param {string} type - The type of notification ('success', 'error', 'info').
 * @param {number} duration - The duration in milliseconds.
 */
function showNotification(message, type = 'info', duration = 3000) {
  const notificationContainer = document.getElementById('notification-container') 
    || createNotificationContainer();
  
  const notification = document.createElement('div');
  notification.className = `notification notification-${type}`;
  notification.textContent = message;
  
  notificationContainer.appendChild(notification);
  
  // Fade in
  setTimeout(() => {
    notification.style.opacity = '1';
  }, 10);
  
  // Fade out and remove
  setTimeout(() => {
    notification.style.opacity = '0';
    setTimeout(() => {
      notification.remove();
    }, 300);
  }, duration);
}

/**
 * Create a notification container if it doesn't exist.
 * 
 * @returns {HTMLElement} The notification container.
 */
function createNotificationContainer() {
  const container = document.createElement('div');
  container.id = 'notification-container';
  container.style.position = 'fixed';
  container.style.top = '20px';
  container.style.right = '20px';
  container.style.zIndex = '1000';
  document.body.appendChild(container);
  return container;
}

/**
 * Debounce a function to prevent excessive calls.
 * 
 * @param {Function} func - The function to debounce.
 * @param {number} wait - The debounce wait time in milliseconds.
 * @returns {Function} The debounced function.
 */
function debounce(func, wait = 300) {
  let timeout;
  return function(...args) {
    clearTimeout(timeout);
    timeout = setTimeout(() => func.apply(this, args), wait);
  };
}

/**
 * Create a DOM element with attributes and children.
 * 
 * @param {string} tag - The HTML tag name.
 * @param {Object} attributes - The attributes to set.
 * @param {Array|string} children - The children to append.
 * @returns {HTMLElement} The created element.
 */
function createElement(tag, attributes = {}, children = []) {
  const element = document.createElement(tag);
  
  // Set attributes
  Object.entries(attributes).forEach(([key, value]) => {
    if (key === 'style' && typeof value === 'object') {
      Object.entries(value).forEach(([styleKey, styleValue]) => {
        element.style[styleKey] = styleValue;
      });
    } else if (key.startsWith('on') && typeof value === 'function') {
      element.addEventListener(key.substring(2).toLowerCase(), value);
    } else {
      element.setAttribute(key, value);
    }
  });
  
  // Add children
  if (typeof children === 'string') {
    element.textContent = children;
  } else if (Array.isArray(children)) {
    children.forEach(child => {
      if (typeof child === 'string') {
        element.appendChild(document.createTextNode(child));
      } else if (child instanceof Node) {
        element.appendChild(child);
      }
    });
  }
  
  return element;
}

// ===== API Connection =====

/**
 * Initialize the API connection.
 * 
 * @returns {Promise<ApiPromise>} The API instance.
 */
async function initializeApi() {
  try {
    const connectionStatus = document.getElementById('connection-status');
    connectionStatus.textContent = 'Connecting to CritterChain...';
    
    // Connect to the local node
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    
    // Store the API in the application state
    AppState.api = api;
    
    // Get chain constants
    await getPalletConstants(api);
    
    connectionStatus.textContent = 'Connected to CritterChain';
    connectionStatus.style.color = 'green';
    
    return api;
  } catch (error) {
    console.error('Failed to connect to the blockchain:', error);
    
    const connectionStatus = document.getElementById('connection-status');
    connectionStatus.textContent = 'Failed to connect to CritterChain';
    connectionStatus.style.color = 'red';
    
    showNotification('Failed to connect to the blockchain. Please try again later.', 'error');
    
    throw error;
  }
}

/**
 * Get pallet constants from the API.
 * 
 * @param {ApiPromise} api - The API instance.
 */
async function getPalletConstants(api) {
  try {
    // Get chain decimals
    AppState.constants.chainDecimals = api.registry.chainDecimals[0] || 18;
    
    // Get claim cooldown period
    if (api.consts.critterRewards && api.consts.critterRewards.claimCooldownPeriod) {
      AppState.constants.claimCooldownPeriodBlocks = api.consts.critterRewards.claimCooldownPeriod.toNumber();
    }
    
    // Get daily claim amount
    if (api.consts.critterRewards && api.consts.critterRewards.dailyRewardAmount) {
      AppState.constants.dailyClaimAmount = api.consts.critterRewards.dailyRewardAmount.toBigInt();
    }
    
    console.log('Pallet constants loaded:', AppState.constants);
  } catch (error) {
    console.error('Failed to get pallet constants:', error);
  }
}

// ===== Account Management =====

/**
 * Set the current account.
 * 
 * @param {string} accountAddress - The account address.
 */
function setCurrentAccount(accountAddress) {
  AppState.currentAccount = accountAddress;
  
  // Update UI elements
  const walletAddressSpan = document.getElementById('wallet-address');
  if (walletAddressSpan) {
    walletAddressSpan.textContent = accountAddress;
  }
  
  // Load account data
  loadAccountData(accountAddress);
}

/**
 * Load account data.
 * 
 * @param {string} accountAddress - The account address.
 */
async function loadAccountData(accountAddress) {
  if (!AppState.api || !accountAddress) return;
  
  try {
    // Subscribe to account balance
    subscribeToAccountBalance(accountAddress);
    
    // Load NFTs
    await loadOwnedNfts(accountAddress);
    
    // Load next claim time
    await loadNextClaimTime(accountAddress);
    
    // Load completed quests
    await loadCompletedQuests(accountAddress);
    
    // Populate battle pet select
    populateBattlePetSelect(accountAddress);
    
    // Populate breeding pet selects
    populateBreedingPetSelects(accountAddress);
  } catch (error) {
    console.error('Failed to load account data:', error);
    showNotification('Failed to load account data. Please try again later.', 'error');
  }
}

/**
 * Subscribe to account balance.
 * 
 * @param {string} accountAddress - The account address.
 */
async function subscribeToAccountBalance(accountAddress) {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.balance) {
      await AppState.subscriptions.balance();
      AppState.subscriptions.balance = null;
    }
    
    // Subscribe to account balance
    const ptcnBalanceSpan = document.getElementById('ptcn-balance');
    
    AppState.subscriptions.balance = await AppState.api.query.system.account(
      accountAddress,
      ({ data: balance }) => {
        if (ptcnBalanceSpan) {
          ptcnBalanceSpan.textContent = `${formatDisplayBalance(
            balance.free.toBigInt(),
            AppState.constants.chainDecimals
          )} PTCN`;
        }
      }
    );
  } catch (error) {
    console.error('Failed to subscribe to account balance:', error);
    
    const ptcnBalanceSpan = document.getElementById('ptcn-balance');
    if (ptcnBalanceSpan) {
      ptcnBalanceSpan.textContent = 'Error';
    }
  }
}

// ===== NFT Management =====

/**
 * Load owned NFTs.
 * 
 * @param {string} accountAddress - The account address.
 */
async function loadOwnedNfts(accountAddress) {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.nfts) {
      await AppState.subscriptions.nfts();
      AppState.subscriptions.nfts = null;
    }
    
    const nftList = document.getElementById('nft-list');
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');
    
    if (!nftList || !nftListPlaceholder) return;
    
    nftListPlaceholder.textContent = 'Loading Pet NFTs...';
    
    // Subscribe to owned NFTs
    AppState.subscriptions.nfts = await AppState.api.query.critterNfts.ownedTokens(
      accountAddress,
      async (tokenIds) => {
        try {
          // Clear the list
          while (nftList.firstChild) {
            nftList.removeChild(nftList.firstChild);
          }
          
          // Get NFT details
          const tokenIdsArray = tokenIds.toArray().map(id => id.toNumber());
          AppState.cache.ownedNfts = [];
          
          if (tokenIdsArray.length === 0) {
            nftListPlaceholder.textContent = 'You don\'t own any Pet NFTs yet.';
            nftList.appendChild(nftListPlaceholder);
            return;
          }
          
          // Hide the placeholder
          nftListPlaceholder.style.display = 'none';
          
          // Fetch NFT details in parallel
          const nftDetails = await Promise.all(
            tokenIdsArray.map(id => AppState.api.query.critterNfts.tokens(id))
          );
          
          // Process NFT details
          for (let i = 0; i < tokenIdsArray.length; i++) {
            const tokenId = tokenIdsArray[i];
            const nftDetail = nftDetails[i];
            
            if (nftDetail.isSome) {
              const nft = nftDetail.unwrap();
              AppState.cache.ownedNfts.push({
                id: tokenId,
                species: nft.species.toString(),
                level: nft.level.toNumber(),
                xp: nft.xp.toNumber(),
                mood: nft.mood.toNumber(),
                hunger: nft.hunger.toNumber(),
                energy: nft.energy.toNumber(),
                auraColor: nft.auraColor.toString()
              });
              
              // Create NFT list item
              const listItem = createElement('li', {
                class: 'nft-item',
                'data-nft-id': tokenId
              }, [
                createElement('div', { class: 'nft-header' }, [
                  createElement('h3', {}, `${nft.name.toString()} (ID: ${tokenId})`),
                  createElement('span', { class: 'nft-species' }, `Species: ${nft.species.toString()}`)
                ]),
                createElement('div', { class: 'nft-details' }, [
                  createElement('p', {}, `Level: ${nft.level.toNumber()}`),
                  createElement('p', {}, `XP: ${nft.xp.toNumber()}`),
                  createElement('p', {}, `Mood: ${nft.mood.toNumber()}/255`),
                  createElement('p', {}, `Hunger: ${nft.hunger.toNumber()}/255`),
                  createElement('p', {}, `Energy: ${nft.energy.toNumber()}/255`),
                  createElement('p', {}, `Aura: ${nft.auraColor.toString()}`)
                ]),
                createElement('div', { class: 'nft-actions' }, [
                  createElement('button', {
                    class: 'nft-action-button',
                    onclick: () => showUpdatePetForm(tokenId)
                  }, 'Update'),
                  createElement('button', {
                    class: 'nft-action-button',
                    onclick: () => prepareTransferNft(tokenId)
                  }, 'Transfer'),
                  createElement('button', {
                    class: 'nft-action-button',
                    onclick: () => prepareListNft(tokenId)
                  }, 'List for Sale')
                ])
              ]);
              
              nftList.appendChild(listItem);
            }
          }
        } catch (error) {
          console.error('Failed to process NFT details:', error);
          nftListPlaceholder.textContent = 'Error loading Pet NFTs.';
          nftList.appendChild(nftListPlaceholder);
        }
      }
    );
  } catch (error) {
    console.error('Failed to load owned NFTs:', error);
    
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');
    if (nftListPlaceholder) {
      nftListPlaceholder.textContent = 'Error loading Pet NFTs.';
    }
    
    showNotification('Failed to load your Pet NFTs. Please try again later.', 'error');
  }
}

/**
 * Show the update pet form for a specific pet.
 * 
 * @param {number} petId - The pet ID.
 */
function showUpdatePetForm(petId) {
  // Find the pet in the cache
  const pet = AppState.cache.ownedNfts.find(nft => nft.id === petId);
  
  if (!pet) {
    showNotification('Pet not found.', 'error');
    return;
  }
  
  // Set form values
  document.getElementById('update-pet-id').value = petId;
  document.getElementById('update-pet-name').value = '';
  document.getElementById('update-pet-level').value = pet.level;
  document.getElementById('update-pet-xp').value = pet.xp;
  document.getElementById('update-pet-mood').value = pet.mood;
  document.getElementById('update-pet-hunger').value = pet.hunger;
  document.getElementById('update-pet-energy').value = pet.energy;
  
  // Scroll to the update form
  document.getElementById('update-pet-section').scrollIntoView({ behavior: 'smooth' });
}

/**
 * Prepare to transfer an NFT.
 * 
 * @param {number} petId - The pet ID.
 */
function prepareTransferNft(petId) {
  document.getElementById('pet-id-transfer').value = petId;
  document.getElementById('transfer-nft-section').scrollIntoView({ behavior: 'smooth' });
}

/**
 * Prepare to list an NFT for sale.
 * 
 * @param {number} petId - The pet ID.
 */
function prepareListNft(petId) {
  document.getElementById('list-pet-id').value = petId;
  document.getElementById('list-nft-form').scrollIntoView({ behavior: 'smooth' });
}

// ===== Marketplace =====

/**
 * Load marketplace listings.
 */
async function loadMarketplaceListings() {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.marketplace) {
      await AppState.subscriptions.marketplace();
      AppState.subscriptions.marketplace = null;
    }
    
    const marketplaceList = document.getElementById('marketplace-nft-list');
    const marketplaceListPlaceholder = document.getElementById('marketplace-list-placeholder');
    
    if (!marketplaceList || !marketplaceListPlaceholder) return;
    
    marketplaceListPlaceholder.textContent = 'Loading marketplace listings...';
    
    // Subscribe to marketplace listings
    AppState.subscriptions.marketplace = await AppState.api.query.critterMarketplace.listings.entries(
      async (entries) => {
        try {
          // Clear the list
          while (marketplaceList.firstChild) {
            marketplaceList.removeChild(marketplaceList.firstChild);
          }
          
          // Process listings
          AppState.cache.marketplaceListings = [];
          
          if (entries.length === 0) {
            marketplaceListPlaceholder.textContent = 'No NFTs are currently listed for sale.';
            marketplaceList.appendChild(marketplaceListPlaceholder);
            return;
          }
          
          // Hide the placeholder
          marketplaceListPlaceholder.style.display = 'none';
          
          // Process listings
          for (const [key, listing] of entries) {
            const tokenId = key.args[0].toNumber();
            const listingData = listing.unwrap();
            const seller = listingData.seller.toString();
            const price = listingData.price.toBigInt();
            
            AppState.cache.marketplaceListings.push({
              tokenId,
              seller,
              price
            });
            
            // Get NFT details
            const nftDetail = await AppState.api.query.critterNfts.tokens(tokenId);
            
            if (nftDetail.isSome) {
              const nft = nftDetail.unwrap();
              
              // Create listing item
              const listItem = createElement('li', {
                class: 'marketplace-item',
                'data-nft-id': tokenId
              }, [
                createElement('div', { class: 'marketplace-item-header' }, [
                  createElement('h3', {}, `${nft.name.toString()} (ID: ${tokenId})`),
                  createElement('span', { class: 'marketplace-item-species' }, `Species: ${nft.species.toString()}`)
                ]),
                createElement('div', { class: 'marketplace-item-details' }, [
                  createElement('p', {}, `Level: ${nft.level.toNumber()}`),
                  createElement('p', {}, `Seller: ${seller.substring(0, 8)}...${seller.substring(seller.length - 8)}`),
                  createElement('p', { class: 'marketplace-item-price' }, `Price: ${formatDisplayBalance(price, AppState.constants.chainDecimals)} PTCN`)
                ]),
                createElement('div', { class: 'marketplace-item-actions' }, [
                  createElement('button', {
                    class: 'marketplace-buy-button',
                    onclick: () => buyNft(tokenId, price)
                  }, 'Buy Now')
                ])
              ]);
              
              marketplaceList.appendChild(listItem);
            }
          }
        } catch (error) {
          console.error('Failed to process marketplace listings:', error);
          marketplaceListPlaceholder.textContent = 'Error loading marketplace listings.';
          marketplaceList.appendChild(marketplaceListPlaceholder);
        }
      }
    );
  } catch (error) {
    console.error('Failed to load marketplace listings:', error);
    
    const marketplaceListPlaceholder = document.getElementById('marketplace-list-placeholder');
    if (marketplaceListPlaceholder) {
      marketplaceListPlaceholder.textContent = 'Error loading marketplace listings.';
    }
    
    showNotification('Failed to load marketplace listings. Please try again later.', 'error');
  }
}

/**
 * Buy an NFT from the marketplace.
 * 
 * @param {number} tokenId - The token ID.
 * @param {BigInt} price - The price.
 */
async function buyNft(tokenId, price) {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  try {
    showNotification(`Buying NFT #${tokenId}...`, 'info');
    
    // Create the transaction
    const tx = AppState.api.tx.critterMarketplace.buyToken(tokenId);
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          showNotification(`Successfully purchased NFT #${tokenId}!`, 'success');
        } else {
          showNotification(`Failed to purchase NFT: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to buy NFT:', error);
    showNotification(`Failed to buy NFT: ${error.message}`, 'error');
  }
}

// ===== Battle Arena =====

/**
 * Populate the battle pet select dropdown.
 * 
 * @param {string} accountAddress - The account address.
 */
function populateBattlePetSelect(accountAddress) {
  const battlePetSelect = document.getElementById('battle-pet-id-select');
  
  if (!battlePetSelect) return;
  
  // Clear the select
  while (battlePetSelect.firstChild) {
    battlePetSelect.removeChild(battlePetSelect.firstChild);
  }
  
  // Add default option
  battlePetSelect.appendChild(
    createElement('option', { value: '' }, '--Select a pet--')
  );
  
  // Add owned pets
  AppState.cache.ownedNfts.forEach(pet => {
    battlePetSelect.appendChild(
      createElement('option', { value: pet.id }, `${pet.id} - ${pet.species} (Level ${pet.level})`)
    );
  });
}

/**
 * Load current battles.
 */
async function loadCurrentBattles() {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.battles) {
      await AppState.subscriptions.battles();
      AppState.subscriptions.battles = null;
    }
    
    const battleList = document.getElementById('battle-list');
    const battleListPlaceholder = document.getElementById('battle-list-placeholder');
    
    if (!battleList || !battleListPlaceholder) return;
    
    battleListPlaceholder.textContent = 'Loading battle information...';
    
    // Subscribe to current battles
    AppState.subscriptions.battles = await AppState.api.query.critterBattle.activeBattles.entries(
      async (entries) => {
        try {
          // Clear the list
          while (battleList.firstChild) {
            battleList.removeChild(battleList.firstChild);
          }
          
          // Process battles
          AppState.cache.currentBattles = [];
          
          if (entries.length === 0) {
            battleListPlaceholder.textContent = 'No active battles at the moment.';
            battleList.appendChild(battleListPlaceholder);
            return;
          }
          
          // Hide the placeholder
          battleListPlaceholder.style.display = 'none';
          
          // Process battles
          for (const [key, battle] of entries) {
            const battleId = key.args[0].toNumber();
            const battleData = battle.unwrap();
            
            const challenger = battleData.challenger.toString();
            const opponent = battleData.opponent.toString();
            const challengerPetId = battleData.challengerPet.toNumber();
            const opponentPetId = battleData.opponentPet.toNumber();
            const status = battleData.status.toString();
            
            AppState.cache.currentBattles.push({
              id: battleId,
              challenger,
              opponent,
              challengerPetId,
              opponentPetId,
              status
            });
            
            // Create battle item
            const listItem = createElement('li', {
              class: 'battle-item',
              'data-battle-id': battleId
            }, [
              createElement('div', { class: 'battle-header' }, [
                createElement('h3', {}, `Battle #${battleId}`),
                createElement('span', { class: 'battle-status' }, `Status: ${status}`)
              ]),
              createElement('div', { class: 'battle-details' }, [
                createElement('p', {}, `Challenger: ${challenger.substring(0, 8)}...${challenger.substring(challenger.length - 8)}`),
                createElement('p', {}, `Challenger Pet: #${challengerPetId}`),
                createElement('p', {}, `Opponent: ${opponent.substring(0, 8)}...${opponent.substring(opponent.length - 8)}`),
                createElement('p', {}, `Opponent Pet: #${opponentPetId}`)
              ]),
              createElement('div', { class: 'battle-actions' }, [
                createElement('button', {
                  class: 'battle-action-button',
                  onclick: () => viewBattleDetails(battleId),
                  disabled: status !== 'Active'
                }, 'View Details'),
                createElement('button', {
                  class: 'battle-action-button',
                  onclick: () => joinBattle(battleId),
                  disabled: status !== 'Waiting' || opponent !== AppState.currentAccount
                }, 'Join Battle')
              ])
            ]);
            
            battleList.appendChild(listItem);
          }
        } catch (error) {
          console.error('Failed to process battle information:', error);
          battleListPlaceholder.textContent = 'Error loading battle information.';
          battleList.appendChild(battleListPlaceholder);
        }
      }
    );
  } catch (error) {
    console.error('Failed to load current battles:', error);
    
    const battleListPlaceholder = document.getElementById('battle-list-placeholder');
    if (battleListPlaceholder) {
      battleListPlaceholder.textContent = 'Error loading battle information.';
    }
    
    showNotification('Failed to load battle information. Please try again later.', 'error');
  }
}

/**
 * View battle details.
 * 
 * @param {number} battleId - The battle ID.
 */
function viewBattleDetails(battleId) {
  // Find the battle in the cache
  const battle = AppState.cache.currentBattles.find(b => b.id === battleId);
  
  if (!battle) {
    showNotification('Battle not found.', 'error');
    return;
  }
  
  // Show battle details in a modal
  showBattleDetailsModal(battle);
}

/**
 * Show battle details in a modal.
 * 
 * @param {Object} battle - The battle object.
 */
function showBattleDetailsModal(battle) {
  // Create modal container if it doesn't exist
  let modalContainer = document.getElementById('modal-container');
  
  if (!modalContainer) {
    modalContainer = createElement('div', {
      id: 'modal-container',
      style: {
        position: 'fixed',
        top: '0',
        left: '0',
        width: '100%',
        height: '100%',
        backgroundColor: 'rgba(0, 0, 0, 0.5)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: '1000'
      }
    });
    
    document.body.appendChild(modalContainer);
  }
  
  // Clear the modal container
  while (modalContainer.firstChild) {
    modalContainer.removeChild(modalContainer.firstChild);
  }
  
  // Create the modal content
  const modalContent = createElement('div', {
    class: 'modal-content',
    style: {
      backgroundColor: 'white',
      padding: '20px',
      borderRadius: '5px',
      maxWidth: '500px',
      width: '100%'
    }
  }, [
    createElement('h2', {}, `Battle #${battle.id}`),
    createElement('p', {}, `Status: ${battle.status}`),
    createElement('p', {}, `Challenger: ${battle.challenger.substring(0, 8)}...${battle.challenger.substring(battle.challenger.length - 8)}`),
    createElement('p', {}, `Challenger Pet: #${battle.challengerPetId}`),
    createElement('p', {}, `Opponent: ${battle.opponent.substring(0, 8)}...${battle.opponent.substring(battle.opponent.length - 8)}`),
    createElement('p', {}, `Opponent Pet: #${battle.opponentPetId}`),
    createElement('button', {
      style: {
        marginTop: '20px',
        padding: '10px',
        backgroundColor: '#f44336',
        color: 'white',
        border: 'none',
        borderRadius: '5px',
        cursor: 'pointer'
      },
      onclick: () => modalContainer.style.display = 'none'
    }, 'Close')
  ]);
  
  modalContainer.appendChild(modalContent);
  modalContainer.style.display = 'flex';
}

/**
 * Join a battle.
 * 
 * @param {number} battleId - The battle ID.
 */
async function joinBattle(battleId) {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  // Find the battle in the cache
  const battle = AppState.cache.currentBattles.find(b => b.id === battleId);
  
  if (!battle) {
    showNotification('Battle not found.', 'error');
    return;
  }
  
  // Check if the user is the opponent
  if (battle.opponent !== AppState.currentAccount) {
    showNotification('You are not the opponent in this battle.', 'error');
    return;
  }
  
  // Check if the battle is waiting
  if (battle.status !== 'Waiting') {
    showNotification('This battle is not waiting for an opponent.', 'error');
    return;
  }
  
  try {
    showNotification(`Joining battle #${battleId}...`, 'info');
    
    // Create the transaction
    const tx = AppState.api.tx.critterBattle.joinBattle(battleId, battle.opponentPetId);
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          showNotification(`Successfully joined battle #${battleId}!`, 'success');
        } else {
          showNotification(`Failed to join battle: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to join battle:', error);
    showNotification(`Failed to join battle: ${error.message}`, 'error');
  }
}

// ===== Quests =====

/**
 * Load available quests.
 */
async function loadAvailableQuests() {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.quests) {
      await AppState.subscriptions.quests();
      AppState.subscriptions.quests = null;
    }
    
    const questsList = document.getElementById('available-quests-list');
    const questsPlaceholder = document.getElementById('available-quests-placeholder');
    
    if (!questsList || !questsPlaceholder) return;
    
    questsPlaceholder.textContent = 'Loading available quests...';
    
    // Subscribe to available quests
    AppState.subscriptions.quests = await AppState.api.query.critterQuests.quests.entries(
      async (entries) => {
        try {
          // Clear the list
          while (questsList.firstChild) {
            questsList.removeChild(questsList.firstChild);
          }
          
          // Process quests
          AppState.cache.availableQuests = [];
          
          if (entries.length === 0) {
            questsPlaceholder.textContent = 'No quests available at the moment.';
            questsList.appendChild(questsPlaceholder);
            return;
          }
          
          // Hide the placeholder
          questsPlaceholder.style.display = 'none';
          
          // Process quests
          for (const [key, quest] of entries) {
            const questId = key.args[0].toNumber();
            const questData = quest.unwrap();
            
            const name = questData.name.toString();
            const description = questData.description.toString();
            const requiredLevel = questData.requiredLevel.toNumber();
            const rewardBits = questData.rewardBits.toNumber();
            const rewardAura = questData.rewardAura.toNumber();
            
            AppState.cache.availableQuests.push({
              id: questId,
              name,
              description,
              requiredLevel,
              rewardBits,
              rewardAura
            });
            
            // Create quest item
            const listItem = createElement('li', {
              class: 'quest-item',
              'data-quest-id': questId
            }, [
              createElement('div', { class: 'quest-header' }, [
                createElement('h3', {}, name),
                createElement('span', { class: 'quest-level' }, `Required Level: ${requiredLevel}`)
              ]),
              createElement('div', { class: 'quest-details' }, [
                createElement('p', {}, description),
                createElement('p', { class: 'quest-rewards' }, `Rewards: ${rewardBits} BITS, ${rewardAura} AURA`)
              ]),
              createElement('div', { class: 'quest-actions' }, [
                createElement('button', {
                  class: 'quest-action-button',
                  onclick: () => startQuest(questId)
                }, 'Start Quest')
              ])
            ]);
            
            questsList.appendChild(listItem);
          }
        } catch (error) {
          console.error('Failed to process available quests:', error);
          questsPlaceholder.textContent = 'Error loading available quests.';
          questsList.appendChild(questsPlaceholder);
        }
      }
    );
  } catch (error) {
    console.error('Failed to load available quests:', error);
    
    const questsPlaceholder = document.getElementById('available-quests-placeholder');
    if (questsPlaceholder) {
      questsPlaceholder.textContent = 'Error loading available quests.';
    }
    
    showNotification('Failed to load available quests. Please try again later.', 'error');
  }
}

/**
 * Load completed quests.
 * 
 * @param {string} accountAddress - The account address.
 */
async function loadCompletedQuests(accountAddress) {
  try {
    const completedQuestsList = document.getElementById('completed-quests-list');
    const completedQuestsPlaceholder = document.getElementById('completed-quests-placeholder');
    
    if (!completedQuestsList || !completedQuestsPlaceholder) return;
    
    completedQuestsPlaceholder.textContent = 'Loading completed quests...';
    
    // Get completed quests
    const completedQuests = await AppState.api.query.critterQuests.completedQuests(accountAddress);
    const completedQuestsArray = completedQuests.toArray().map(id => id.toNumber());
    
    // Clear the list
    while (completedQuestsList.firstChild) {
      completedQuestsList.removeChild(completedQuestsList.firstChild);
    }
    
    if (completedQuestsArray.length === 0) {
      completedQuestsPlaceholder.textContent = 'You haven\'t completed any quests yet.';
      completedQuestsList.appendChild(completedQuestsPlaceholder);
      return;
    }
    
    // Hide the placeholder
    completedQuestsPlaceholder.style.display = 'none';
    
    // Fetch quest details in parallel
    const questDetails = await Promise.all(
      completedQuestsArray.map(id => AppState.api.query.critterQuests.quests(id))
    );
    
    // Process quest details
    for (let i = 0; i < completedQuestsArray.length; i++) {
      const questId = completedQuestsArray[i];
      const questDetail = questDetails[i];
      
      if (questDetail.isSome) {
        const quest = questDetail.unwrap();
        const name = quest.name.toString();
        const description = quest.description.toString();
        
        // Create quest item
        const listItem = createElement('li', {
          class: 'completed-quest-item',
          'data-quest-id': questId
        }, [
          createElement('div', { class: 'quest-header' }, [
            createElement('h3', {}, name),
            createElement('span', { class: 'quest-status' }, 'Completed')
          ]),
          createElement('div', { class: 'quest-details' }, [
            createElement('p', {}, description)
          ])
        ]);
        
        completedQuestsList.appendChild(listItem);
      }
    }
  } catch (error) {
    console.error('Failed to load completed quests:', error);
    
    const completedQuestsPlaceholder = document.getElementById('completed-quests-placeholder');
    if (completedQuestsPlaceholder) {
      completedQuestsPlaceholder.textContent = 'Error loading completed quests.';
    }
  }
}

/**
 * Start a quest.
 * 
 * @param {number} questId - The quest ID.
 */
async function startQuest(questId) {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  // Find the quest in the cache
  const quest = AppState.cache.availableQuests.find(q => q.id === questId);
  
  if (!quest) {
    showNotification('Quest not found.', 'error');
    return;
  }
  
  try {
    showNotification(`Starting quest: ${quest.name}...`, 'info');
    
    // Create the transaction
    const tx = AppState.api.tx.critterQuests.startQuest(questId);
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          showNotification(`Successfully started quest: ${quest.name}!`, 'success');
        } else {
          showNotification(`Failed to start quest: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to start quest:', error);
    showNotification(`Failed to start quest: ${error.message}`, 'error');
  }
}

// ===== Breeding =====

/**
 * Populate the breeding pet select dropdowns.
 * 
 * @param {string} accountAddress - The account address.
 */
function populateBreedingPetSelects(accountAddress) {
  const parentASelect = document.getElementById('parent-a-select');
  const parentBSelect = document.getElementById('parent-b-select');
  
  if (!parentASelect || !parentBSelect) return;
  
  // Clear the selects
  while (parentASelect.firstChild) {
    parentASelect.removeChild(parentASelect.firstChild);
  }
  
  while (parentBSelect.firstChild) {
    parentBSelect.removeChild(parentBSelect.firstChild);
  }
  
  // Add default options
  parentASelect.appendChild(
    createElement('option', { value: '' }, '-- Select one of your pets --')
  );
  
  parentBSelect.appendChild(
    createElement('option', { value: '' }, '-- Select one of your pets --')
  );
  
  // Add owned pets
  AppState.cache.ownedNfts.forEach(pet => {
    const optionText = `${pet.id} - ${pet.species} (Level ${pet.level})`;
    
    parentASelect.appendChild(
      createElement('option', { value: pet.id }, optionText)
    );
    
    parentBSelect.appendChild(
      createElement('option', { value: pet.id }, optionText)
    );
  });
  
  // Add event listeners
  parentASelect.addEventListener('change', () => updateParentDetails('a', parentASelect.value));
  parentBSelect.addEventListener('change', () => updateParentDetails('b', parentBSelect.value));
}

/**
 * Update parent details.
 * 
 * @param {string} parent - The parent ('a' or 'b').
 * @param {string} petId - The pet ID.
 */
function updateParentDetails(parent, petId) {
  const detailsDiv = document.getElementById(`parent-${parent}-details`);
  
  if (!detailsDiv) return;
  
  // Clear the details
  while (detailsDiv.firstChild) {
    detailsDiv.removeChild(detailsDiv.firstChild);
  }
  
  if (!petId) {
    detailsDiv.appendChild(
      createElement('p', { style: { fontStyle: 'italic' } }, `Details for Parent ${parent.toUpperCase()} will appear here.`)
    );
    return;
  }
  
  // Find the pet in the cache
  const pet = AppState.cache.ownedNfts.find(p => p.id == petId);
  
  if (!pet) {
    detailsDiv.appendChild(
      createElement('p', { style: { color: 'red' } }, 'Pet not found.')
    );
    return;
  }
  
  // Add pet details
  detailsDiv.appendChild(createElement('p', {}, `Species: ${pet.species}`));
  detailsDiv.appendChild(createElement('p', {}, `Level: ${pet.level}`));
  detailsDiv.appendChild(createElement('p', {}, `Mood: ${pet.mood}/255`));
  detailsDiv.appendChild(createElement('p', {}, `Energy: ${pet.energy}/255`));
  detailsDiv.appendChild(createElement('p', {}, `Aura: ${pet.auraColor}`));
  detailsDiv.appendChild(createElement('p', {}, 'Breeding Cooldown: Ready'));
}

// ===== Daily Claim =====

/**
 * Load next claim time.
 * 
 * @param {string} accountAddress - The account address.
 */
async function loadNextClaimTime(accountAddress) {
  try {
    const nextClaimTimeSpan = document.getElementById('next-claim-time');
    
    if (!nextClaimTimeSpan) return;
    
    // Get the last claim block
    const lastClaimBlock = await AppState.api.query.critterRewards.lastClaim(accountAddress);
    AppState.cache.lastClaimBlock = lastClaimBlock.toNumber();
    
    // Get the current block
    const currentBlock = await AppState.api.query.system.number();
    const currentBlockNumber = currentBlock.toNumber();
    
    // Calculate the next claim block
    const nextClaimBlock = AppState.cache.lastClaimBlock + AppState.constants.claimCooldownPeriodBlocks;
    AppState.cache.nextClaimBlock = nextClaimBlock;
    
    // Update the UI
    if (nextClaimBlock <= currentBlockNumber) {
      nextClaimTimeSpan.textContent = 'Now';
      document.getElementById('claimDailyPtcnButton').disabled = false;
    } else {
      const blocksRemaining = nextClaimBlock - currentBlockNumber;
      const timeRemaining = blocksRemaining * 6; // Assuming 6 seconds per block
      
      nextClaimTimeSpan.textContent = formatTimeRemaining(timeRemaining);
      document.getElementById('claimDailyPtcnButton').disabled = true;
      
      // Start a timer to update the time remaining
      startClaimTimer(timeRemaining);
    }
  } catch (error) {
    console.error('Failed to load next claim time:', error);
    
    const nextClaimTimeSpan = document.getElementById('next-claim-time');
    if (nextClaimTimeSpan) {
      nextClaimTimeSpan.textContent = 'Error';
    }
  }
}

/**
 * Format time remaining.
 * 
 * @param {number} seconds - The time remaining in seconds.
 * @returns {string} The formatted time.
 */
function formatTimeRemaining(seconds) {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const remainingSeconds = seconds % 60;
  
  return `${hours}h ${minutes}m ${remainingSeconds}s`;
}

/**
 * Start a timer to update the time remaining.
 * 
 * @param {number} seconds - The time remaining in seconds.
 */
function startClaimTimer(seconds) {
  // Clear any existing timer
  if (window.claimTimer) {
    clearInterval(window.claimTimer);
  }
  
  let timeRemaining = seconds;
  const nextClaimTimeSpan = document.getElementById('next-claim-time');
  
  window.claimTimer = setInterval(() => {
    timeRemaining--;
    
    if (timeRemaining <= 0) {
      clearInterval(window.claimTimer);
      nextClaimTimeSpan.textContent = 'Now';
      document.getElementById('claimDailyPtcnButton').disabled = false;
    } else {
      nextClaimTimeSpan.textContent = formatTimeRemaining(timeRemaining);
    }
  }, 1000);
}

// ===== Event Handlers =====

/**
 * Handle the claim daily PTCN button click.
 */
async function handleClaimDailyPtcn() {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  try {
    const claimStatus = document.getElementById('claim-ptcn-status');
    claimStatus.textContent = 'Claiming daily PTCN...';
    
    // Create the transaction
    const tx = AppState.api.tx.critterRewards.claimDailyReward();
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          claimStatus.textContent = 'Successfully claimed daily PTCN!';
          showNotification('Successfully claimed daily PTCN!', 'success');
          
          // Update the next claim time
          loadNextClaimTime(AppState.currentAccount);
        } else {
          claimStatus.textContent = `Failed to claim daily PTCN: ${errorMessage}`;
          showNotification(`Failed to claim daily PTCN: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to claim daily PTCN:', error);
    
    const claimStatus = document.getElementById('claim-ptcn-status');
    if (claimStatus) {
      claimStatus.textContent = `Failed to claim daily PTCN: ${error.message}`;
    }
    
    showNotification(`Failed to claim daily PTCN: ${error.message}`, 'error');
  }
}

/**
 * Handle the mint NFT button click.
 */
async function handleMintNft() {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  try {
    const species = document.getElementById('pet-species').value;
    const name = document.getElementById('pet-name').value;
    const mintStatus = document.getElementById('mint-status');
    
    if (!species || !name) {
      mintStatus.textContent = 'Please enter a species and name.';
      return;
    }
    
    mintStatus.textContent = 'Minting pet NFT...';
    
    // Create the transaction
    const tx = AppState.api.tx.critterNfts.mintPet(species, name);
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          mintStatus.textContent = 'Successfully minted pet NFT!';
          showNotification('Successfully minted pet NFT!', 'success');
          
          // Reload owned NFTs
          loadOwnedNfts(AppState.currentAccount);
        } else {
          mintStatus.textContent = `Failed to mint pet NFT: ${errorMessage}`;
          showNotification(`Failed to mint pet NFT: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to mint pet NFT:', error);
    
    const mintStatus = document.getElementById('mint-status');
    if (mintStatus) {
      mintStatus.textContent = `Failed to mint pet NFT: ${error.message}`;
    }
    
    showNotification(`Failed to mint pet NFT: ${error.message}`, 'error');
  }
}

// ===== Initialization =====

/**
 * Initialize the application.
 */
async function initializeApp() {
  try {
    // Initialize the API
    const api = await initializeApi();
    
    // Set Alice as the current account (for testing)
    const ALICE_ADDRESS = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    setCurrentAccount(ALICE_ADDRESS);
    
    // Load marketplace listings
    await loadMarketplaceListings();
    
    // Load current battles
    await loadCurrentBattles();
    
    // Load available quests
    await loadAvailableQuests();
    
    // Subscribe to system events
    subscribeToSystemEvents();
    
    // Add event listeners
    document.getElementById('claimDailyPtcnButton').addEventListener('click', handleClaimDailyPtcn);
    document.getElementById('mintNftButton').addEventListener('click', handleMintNft);
    
    // Add more event listeners as needed
    
    console.log('Application initialized successfully');
  } catch (error) {
    console.error('Failed to initialize application:', error);
    showNotification('Failed to initialize application. Please try again later.', 'error');
  }
}

/**
 * Subscribe to system events.
 */
async function subscribeToSystemEvents() {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.events) {
      await AppState.subscriptions.events();
      AppState.subscriptions.events = null;
    }
    
    const eventList = document.getElementById('event-list');
    const eventListPlaceholder = document.getElementById('event-list-placeholder');
    
    if (!eventList || !eventListPlaceholder) return;
    
    // Subscribe to system events
    AppState.subscriptions.events = await AppState.api.query.system.events((events) => {
      // Process events
      const relevantEvents = [];
      
      events.forEach((record) => {
        const { event } = record;
        
        // Filter for relevant events
        if (
          event.section === 'critterNfts' ||
          event.section === 'critterMarketplace' ||
          event.section === 'critterBattle' ||
          event.section === 'critterQuests' ||
          event.section === 'critterRewards'
        ) {
          relevantEvents.push({
            section: event.section,
            method: event.method,
            data: event.data.toString()
          });
        }
      });
      
      // Update the event list
      if (relevantEvents.length > 0) {
        // Clear the placeholder
        eventListPlaceholder.style.display = 'none';
        
        // Add new events to the list
        relevantEvents.forEach((event) => {
          const listItem = createElement('li', {
            class: 'event-item'
          }, [
            createElement('span', { class: 'event-section' }, event.section),
            createElement('span', { class: 'event-method' }, event.method),
            createElement('span', { class: 'event-data' }, event.data)
          ]);
          
          // Add to the beginning of the list
          eventList.insertBefore(listItem, eventList.firstChild);
          
          // Limit the number of events displayed
          if (eventList.children.length > 10) {
            eventList.removeChild(eventList.lastChild);
          }
        });
      } else if (eventList.children.length === 0) {
        // Show the placeholder if there are no events
        eventListPlaceholder.style.display = 'block';
      }
    });
  } catch (error) {
    console.error('Failed to subscribe to system events:', error);
  }
}

// Initialize the application when the DOM is loaded
document.addEventListener('DOMContentLoaded', initializeApp);/**
 * CritterCraft Wallet - Optimized Application
 * 
 * This file contains the optimized JavaScript code for the CritterCraft wallet UI.
 * It includes performance improvements, better error handling, and a more modular structure.
 */

// Use strict mode for better error catching and performance
'use strict';

// Import Polkadot API from the window object
const { ApiPromise, WsProvider } = window.PolkadotApi;

// Application state
const AppState = {
  api: null,
  currentAccount: null,
  subscriptions: {
    balance: null,
    events: null,
    nfts: null,
    marketplace: null,
    battles: null,
    quests: null,
    daycare: null
  },
  constants: {
    claimCooldownPeriodBlocks: 100, // Default/Placeholder
    dailyClaimAmount: BigInt("10000000000000000000"), // Default/Placeholder: 10 PTCN
    chainDecimals: 18
  },
  cache: {
    ownedNfts: [],
    marketplaceListings: [],
    currentBattles: [],
    availableQuests: [],
    completedQuests: [],
    lastClaimBlock: 0,
    nextClaimBlock: 0
  }
};

// ===== Utility Functions =====

/**
 * Format a balance for display.
 * 
 * @param {BigInt} balanceBigInt - The balance as a BigInt.
 * @param {number} decimals - The number of decimal places.
 * @param {number} displayDecimals - The number of decimal places to display.
 * @returns {string} The formatted balance.
 */
function formatDisplayBalance(balanceBigInt, decimals, displayDecimals = 4) {
  const balanceStr = balanceBigInt.toString();
  
  if (decimals === 0) return balanceStr;
  
  let intPart = balanceStr.length > decimals 
    ? balanceStr.substring(0, balanceStr.length - decimals) 
    : '0';
    
  let fracPart = balanceStr.length > decimals 
    ? balanceStr.substring(balanceStr.length - decimals) 
    : '0'.repeat(decimals - balanceStr.length) + balanceStr;
    
  return `${intPart}.${fracPart.substring(0, displayDecimals)}`;
}

/**
 * Show a notification to the user.
 * 
 * @param {string} message - The message to display.
 * @param {string} type - The type of notification ('success', 'error', 'info').
 * @param {number} duration - The duration in milliseconds.
 */
function showNotification(message, type = 'info', duration = 3000) {
  const notificationContainer = document.getElementById('notification-container') 
    || createNotificationContainer();
  
  const notification = document.createElement('div');
  notification.className = `notification notification-${type}`;
  notification.textContent = message;
  
  notificationContainer.appendChild(notification);
  
  // Fade in
  setTimeout(() => {
    notification.style.opacity = '1';
  }, 10);
  
  // Fade out and remove
  setTimeout(() => {
    notification.style.opacity = '0';
    setTimeout(() => {
      notification.remove();
    }, 300);
  }, duration);
}

/**
 * Create a notification container if it doesn't exist.
 * 
 * @returns {HTMLElement} The notification container.
 */
function createNotificationContainer() {
  const container = document.createElement('div');
  container.id = 'notification-container';
  container.style.position = 'fixed';
  container.style.top = '20px';
  container.style.right = '20px';
  container.style.zIndex = '1000';
  document.body.appendChild(container);
  return container;
}

/**
 * Debounce a function to prevent excessive calls.
 * 
 * @param {Function} func - The function to debounce.
 * @param {number} wait - The debounce wait time in milliseconds.
 * @returns {Function} The debounced function.
 */
function debounce(func, wait = 300) {
  let timeout;
  return function(...args) {
    clearTimeout(timeout);
    timeout = setTimeout(() => func.apply(this, args), wait);
  };
}

/**
 * Create a DOM element with attributes and children.
 * 
 * @param {string} tag - The HTML tag name.
 * @param {Object} attributes - The attributes to set.
 * @param {Array|string} children - The children to append.
 * @returns {HTMLElement} The created element.
 */
function createElement(tag, attributes = {}, children = []) {
  const element = document.createElement(tag);
  
  // Set attributes
  Object.entries(attributes).forEach(([key, value]) => {
    if (key === 'style' && typeof value === 'object') {
      Object.entries(value).forEach(([styleKey, styleValue]) => {
        element.style[styleKey] = styleValue;
      });
    } else if (key.startsWith('on') && typeof value === 'function') {
      element.addEventListener(key.substring(2).toLowerCase(), value);
    } else {
      element.setAttribute(key, value);
    }
  });
  
  // Add children
  if (typeof children === 'string') {
    element.textContent = children;
  } else if (Array.isArray(children)) {
    children.forEach(child => {
      if (typeof child === 'string') {
        element.appendChild(document.createTextNode(child));
      } else if (child instanceof Node) {
        element.appendChild(child);
      }
    });
  }
  
  return element;
}

// ===== API Connection =====

/**
 * Initialize the API connection.
 * 
 * @returns {Promise<ApiPromise>} The API instance.
 */
async function initializeApi() {
  try {
    const connectionStatus = document.getElementById('connection-status');
    connectionStatus.textContent = 'Connecting to CritterChain...';
    
    // Connect to the local node
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    
    // Store the API in the application state
    AppState.api = api;
    
    // Get chain constants
    await getPalletConstants(api);
    
    connectionStatus.textContent = 'Connected to CritterChain';
    connectionStatus.style.color = 'green';
    
    return api;
  } catch (error) {
    console.error('Failed to connect to the blockchain:', error);
    
    const connectionStatus = document.getElementById('connection-status');
    connectionStatus.textContent = 'Failed to connect to CritterChain';
    connectionStatus.style.color = 'red';
    
    showNotification('Failed to connect to the blockchain. Please try again later.', 'error');
    
    throw error;
  }
}

/**
 * Get pallet constants from the API.
 * 
 * @param {ApiPromise} api - The API instance.
 */
async function getPalletConstants(api) {
  try {
    // Get chain decimals
    AppState.constants.chainDecimals = api.registry.chainDecimals[0] || 18;
    
    // Get claim cooldown period
    if (api.consts.critterRewards && api.consts.critterRewards.claimCooldownPeriod) {
      AppState.constants.claimCooldownPeriodBlocks = api.consts.critterRewards.claimCooldownPeriod.toNumber();
    }
    
    // Get daily claim amount
    if (api.consts.critterRewards && api.consts.critterRewards.dailyRewardAmount) {
      AppState.constants.dailyClaimAmount = api.consts.critterRewards.dailyRewardAmount.toBigInt();
    }
    
    console.log('Pallet constants loaded:', AppState.constants);
  } catch (error) {
    console.error('Failed to get pallet constants:', error);
  }
}

// ===== Account Management =====

/**
 * Set the current account.
 * 
 * @param {string} accountAddress - The account address.
 */
function setCurrentAccount(accountAddress) {
  AppState.currentAccount = accountAddress;
  
  // Update UI elements
  const walletAddressSpan = document.getElementById('wallet-address');
  if (walletAddressSpan) {
    walletAddressSpan.textContent = accountAddress;
  }
  
  // Load account data
  loadAccountData(accountAddress);
}

/**
 * Load account data.
 * 
 * @param {string} accountAddress - The account address.
 */
async function loadAccountData(accountAddress) {
  if (!AppState.api || !accountAddress) return;
  
  try {
    // Subscribe to account balance
    subscribeToAccountBalance(accountAddress);
    
    // Load NFTs
    await loadOwnedNfts(accountAddress);
    
    // Load next claim time
    await loadNextClaimTime(accountAddress);
    
    // Load completed quests
    await loadCompletedQuests(accountAddress);
    
    // Populate battle pet select
    populateBattlePetSelect(accountAddress);
    
    // Populate breeding pet selects
    populateBreedingPetSelects(accountAddress);
  } catch (error) {
    console.error('Failed to load account data:', error);
    showNotification('Failed to load account data. Please try again later.', 'error');
  }
}

/**
 * Subscribe to account balance.
 * 
 * @param {string} accountAddress - The account address.
 */
async function subscribeToAccountBalance(accountAddress) {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.balance) {
      await AppState.subscriptions.balance();
      AppState.subscriptions.balance = null;
    }
    
    // Subscribe to account balance
    const ptcnBalanceSpan = document.getElementById('ptcn-balance');
    
    AppState.subscriptions.balance = await AppState.api.query.system.account(
      accountAddress,
      ({ data: balance }) => {
        if (ptcnBalanceSpan) {
          ptcnBalanceSpan.textContent = `${formatDisplayBalance(
            balance.free.toBigInt(),
            AppState.constants.chainDecimals
          )} PTCN`;
        }
      }
    );
  } catch (error) {
    console.error('Failed to subscribe to account balance:', error);
    
    const ptcnBalanceSpan = document.getElementById('ptcn-balance');
    if (ptcnBalanceSpan) {
      ptcnBalanceSpan.textContent = 'Error';
    }
  }
}

// ===== NFT Management =====

/**
 * Load owned NFTs.
 * 
 * @param {string} accountAddress - The account address.
 */
async function loadOwnedNfts(accountAddress) {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.nfts) {
      await AppState.subscriptions.nfts();
      AppState.subscriptions.nfts = null;
    }
    
    const nftList = document.getElementById('nft-list');
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');
    
    if (!nftList || !nftListPlaceholder) return;
    
    nftListPlaceholder.textContent = 'Loading Pet NFTs...';
    
    // Subscribe to owned NFTs
    AppState.subscriptions.nfts = await AppState.api.query.critterNfts.ownedTokens(
      accountAddress,
      async (tokenIds) => {
        try {
          // Clear the list
          while (nftList.firstChild) {
            nftList.removeChild(nftList.firstChild);
          }
          
          // Get NFT details
          const tokenIdsArray = tokenIds.toArray().map(id => id.toNumber());
          AppState.cache.ownedNfts = [];
          
          if (tokenIdsArray.length === 0) {
            nftListPlaceholder.textContent = 'You don\'t own any Pet NFTs yet.';
            nftList.appendChild(nftListPlaceholder);
            return;
          }
          
          // Hide the placeholder
          nftListPlaceholder.style.display = 'none';
          
          // Fetch NFT details in parallel
          const nftDetails = await Promise.all(
            tokenIdsArray.map(id => AppState.api.query.critterNfts.tokens(id))
          );
          
          // Process NFT details
          for (let i = 0; i < tokenIdsArray.length; i++) {
            const tokenId = tokenIdsArray[i];
            const nftDetail = nftDetails[i];
            
            if (nftDetail.isSome) {
              const nft = nftDetail.unwrap();
              AppState.cache.ownedNfts.push({
                id: tokenId,
                species: nft.species.toString(),
                level: nft.level.toNumber(),
                xp: nft.xp.toNumber(),
                mood: nft.mood.toNumber(),
                hunger: nft.hunger.toNumber(),
                energy: nft.energy.toNumber(),
                auraColor: nft.auraColor.toString()
              });
              
              // Create NFT list item
              const listItem = createElement('li', {
                class: 'nft-item',
                'data-nft-id': tokenId
              }, [
                createElement('div', { class: 'nft-header' }, [
                  createElement('h3', {}, `${nft.name.toString()} (ID: ${tokenId})`),
                  createElement('span', { class: 'nft-species' }, `Species: ${nft.species.toString()}`)
                ]),
                createElement('div', { class: 'nft-details' }, [
                  createElement('p', {}, `Level: ${nft.level.toNumber()}`),
                  createElement('p', {}, `XP: ${nft.xp.toNumber()}`),
                  createElement('p', {}, `Mood: ${nft.mood.toNumber()}/255`),
                  createElement('p', {}, `Hunger: ${nft.hunger.toNumber()}/255`),
                  createElement('p', {}, `Energy: ${nft.energy.toNumber()}/255`),
                  createElement('p', {}, `Aura: ${nft.auraColor.toString()}`)
                ]),
                createElement('div', { class: 'nft-actions' }, [
                  createElement('button', {
                    class: 'nft-action-button',
                    onclick: () => showUpdatePetForm(tokenId)
                  }, 'Update'),
                  createElement('button', {
                    class: 'nft-action-button',
                    onclick: () => prepareTransferNft(tokenId)
                  }, 'Transfer'),
                  createElement('button', {
                    class: 'nft-action-button',
                    onclick: () => prepareListNft(tokenId)
                  }, 'List for Sale')
                ])
              ]);
              
              nftList.appendChild(listItem);
            }
          }
        } catch (error) {
          console.error('Failed to process NFT details:', error);
          nftListPlaceholder.textContent = 'Error loading Pet NFTs.';
          nftList.appendChild(nftListPlaceholder);
        }
      }
    );
  } catch (error) {
    console.error('Failed to load owned NFTs:', error);
    
    const nftListPlaceholder = document.getElementById('nft-list-placeholder');
    if (nftListPlaceholder) {
      nftListPlaceholder.textContent = 'Error loading Pet NFTs.';
    }
    
    showNotification('Failed to load your Pet NFTs. Please try again later.', 'error');
  }
}

/**
 * Show the update pet form for a specific pet.
 * 
 * @param {number} petId - The pet ID.
 */
function showUpdatePetForm(petId) {
  // Find the pet in the cache
  const pet = AppState.cache.ownedNfts.find(nft => nft.id === petId);
  
  if (!pet) {
    showNotification('Pet not found.', 'error');
    return;
  }
  
  // Set form values
  document.getElementById('update-pet-id').value = petId;
  document.getElementById('update-pet-name').value = '';
  document.getElementById('update-pet-level').value = pet.level;
  document.getElementById('update-pet-xp').value = pet.xp;
  document.getElementById('update-pet-mood').value = pet.mood;
  document.getElementById('update-pet-hunger').value = pet.hunger;
  document.getElementById('update-pet-energy').value = pet.energy;
  
  // Scroll to the update form
  document.getElementById('update-pet-section').scrollIntoView({ behavior: 'smooth' });
}

/**
 * Prepare to transfer an NFT.
 * 
 * @param {number} petId - The pet ID.
 */
function prepareTransferNft(petId) {
  document.getElementById('pet-id-transfer').value = petId;
  document.getElementById('transfer-nft-section').scrollIntoView({ behavior: 'smooth' });
}

/**
 * Prepare to list an NFT for sale.
 * 
 * @param {number} petId - The pet ID.
 */
function prepareListNft(petId) {
  document.getElementById('list-pet-id').value = petId;
  document.getElementById('list-nft-form').scrollIntoView({ behavior: 'smooth' });
}

// ===== Marketplace =====

/**
 * Load marketplace listings.
 */
async function loadMarketplaceListings() {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.marketplace) {
      await AppState.subscriptions.marketplace();
      AppState.subscriptions.marketplace = null;
    }
    
    const marketplaceList = document.getElementById('marketplace-nft-list');
    const marketplaceListPlaceholder = document.getElementById('marketplace-list-placeholder');
    
    if (!marketplaceList || !marketplaceListPlaceholder) return;
    
    marketplaceListPlaceholder.textContent = 'Loading marketplace listings...';
    
    // Subscribe to marketplace listings
    AppState.subscriptions.marketplace = await AppState.api.query.critterMarketplace.listings.entries(
      async (entries) => {
        try {
          // Clear the list
          while (marketplaceList.firstChild) {
            marketplaceList.removeChild(marketplaceList.firstChild);
          }
          
          // Process listings
          AppState.cache.marketplaceListings = [];
          
          if (entries.length === 0) {
            marketplaceListPlaceholder.textContent = 'No NFTs are currently listed for sale.';
            marketplaceList.appendChild(marketplaceListPlaceholder);
            return;
          }
          
          // Hide the placeholder
          marketplaceListPlaceholder.style.display = 'none';
          
          // Process listings
          for (const [key, listing] of entries) {
            const tokenId = key.args[0].toNumber();
            const listingData = listing.unwrap();
            const seller = listingData.seller.toString();
            const price = listingData.price.toBigInt();
            
            AppState.cache.marketplaceListings.push({
              tokenId,
              seller,
              price
            });
            
            // Get NFT details
            const nftDetail = await AppState.api.query.critterNfts.tokens(tokenId);
            
            if (nftDetail.isSome) {
              const nft = nftDetail.unwrap();
              
              // Create listing item
              const listItem = createElement('li', {
                class: 'marketplace-item',
                'data-nft-id': tokenId
              }, [
                createElement('div', { class: 'marketplace-item-header' }, [
                  createElement('h3', {}, `${nft.name.toString()} (ID: ${tokenId})`),
                  createElement('span', { class: 'marketplace-item-species' }, `Species: ${nft.species.toString()}`)
                ]),
                createElement('div', { class: 'marketplace-item-details' }, [
                  createElement('p', {}, `Level: ${nft.level.toNumber()}`),
                  createElement('p', {}, `Seller: ${seller.substring(0, 8)}...${seller.substring(seller.length - 8)}`),
                  createElement('p', { class: 'marketplace-item-price' }, `Price: ${formatDisplayBalance(price, AppState.constants.chainDecimals)} PTCN`)
                ]),
                createElement('div', { class: 'marketplace-item-actions' }, [
                  createElement('button', {
                    class: 'marketplace-buy-button',
                    onclick: () => buyNft(tokenId, price)
                  }, 'Buy Now')
                ])
              ]);
              
              marketplaceList.appendChild(listItem);
            }
          }
        } catch (error) {
          console.error('Failed to process marketplace listings:', error);
          marketplaceListPlaceholder.textContent = 'Error loading marketplace listings.';
          marketplaceList.appendChild(marketplaceListPlaceholder);
        }
      }
    );
  } catch (error) {
    console.error('Failed to load marketplace listings:', error);
    
    const marketplaceListPlaceholder = document.getElementById('marketplace-list-placeholder');
    if (marketplaceListPlaceholder) {
      marketplaceListPlaceholder.textContent = 'Error loading marketplace listings.';
    }
    
    showNotification('Failed to load marketplace listings. Please try again later.', 'error');
  }
}

/**
 * Buy an NFT from the marketplace.
 * 
 * @param {number} tokenId - The token ID.
 * @param {BigInt} price - The price.
 */
async function buyNft(tokenId, price) {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  try {
    showNotification(`Buying NFT #${tokenId}...`, 'info');
    
    // Create the transaction
    const tx = AppState.api.tx.critterMarketplace.buyToken(tokenId);
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          showNotification(`Successfully purchased NFT #${tokenId}!`, 'success');
        } else {
          showNotification(`Failed to purchase NFT: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to buy NFT:', error);
    showNotification(`Failed to buy NFT: ${error.message}`, 'error');
  }
}

// ===== Battle Arena =====

/**
 * Populate the battle pet select dropdown.
 * 
 * @param {string} accountAddress - The account address.
 */
function populateBattlePetSelect(accountAddress) {
  const battlePetSelect = document.getElementById('battle-pet-id-select');
  
  if (!battlePetSelect) return;
  
  // Clear the select
  while (battlePetSelect.firstChild) {
    battlePetSelect.removeChild(battlePetSelect.firstChild);
  }
  
  // Add default option
  battlePetSelect.appendChild(
    createElement('option', { value: '' }, '--Select a pet--')
  );
  
  // Add owned pets
  AppState.cache.ownedNfts.forEach(pet => {
    battlePetSelect.appendChild(
      createElement('option', { value: pet.id }, `${pet.id} - ${pet.species} (Level ${pet.level})`)
    );
  });
}

/**
 * Load current battles.
 */
async function loadCurrentBattles() {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.battles) {
      await AppState.subscriptions.battles();
      AppState.subscriptions.battles = null;
    }
    
    const battleList = document.getElementById('battle-list');
    const battleListPlaceholder = document.getElementById('battle-list-placeholder');
    
    if (!battleList || !battleListPlaceholder) return;
    
    battleListPlaceholder.textContent = 'Loading battle information...';
    
    // Subscribe to current battles
    AppState.subscriptions.battles = await AppState.api.query.critterBattle.activeBattles.entries(
      async (entries) => {
        try {
          // Clear the list
          while (battleList.firstChild) {
            battleList.removeChild(battleList.firstChild);
          }
          
          // Process battles
          AppState.cache.currentBattles = [];
          
          if (entries.length === 0) {
            battleListPlaceholder.textContent = 'No active battles at the moment.';
            battleList.appendChild(battleListPlaceholder);
            return;
          }
          
          // Hide the placeholder
          battleListPlaceholder.style.display = 'none';
          
          // Process battles
          for (const [key, battle] of entries) {
            const battleId = key.args[0].toNumber();
            const battleData = battle.unwrap();
            
            const challenger = battleData.challenger.toString();
            const opponent = battleData.opponent.toString();
            const challengerPetId = battleData.challengerPet.toNumber();
            const opponentPetId = battleData.opponentPet.toNumber();
            const status = battleData.status.toString();
            
            AppState.cache.currentBattles.push({
              id: battleId,
              challenger,
              opponent,
              challengerPetId,
              opponentPetId,
              status
            });
            
            // Create battle item
            const listItem = createElement('li', {
              class: 'battle-item',
              'data-battle-id': battleId
            }, [
              createElement('div', { class: 'battle-header' }, [
                createElement('h3', {}, `Battle #${battleId}`),
                createElement('span', { class: 'battle-status' }, `Status: ${status}`)
              ]),
              createElement('div', { class: 'battle-details' }, [
                createElement('p', {}, `Challenger: ${challenger.substring(0, 8)}...${challenger.substring(challenger.length - 8)}`),
                createElement('p', {}, `Challenger Pet: #${challengerPetId}`),
                createElement('p', {}, `Opponent: ${opponent.substring(0, 8)}...${opponent.substring(opponent.length - 8)}`),
                createElement('p', {}, `Opponent Pet: #${opponentPetId}`)
              ]),
              createElement('div', { class: 'battle-actions' }, [
                createElement('button', {
                  class: 'battle-action-button',
                  onclick: () => viewBattleDetails(battleId),
                  disabled: status !== 'Active'
                }, 'View Details'),
                createElement('button', {
                  class: 'battle-action-button',
                  onclick: () => joinBattle(battleId),
                  disabled: status !== 'Waiting' || opponent !== AppState.currentAccount
                }, 'Join Battle')
              ])
            ]);
            
            battleList.appendChild(listItem);
          }
        } catch (error) {
          console.error('Failed to process battle information:', error);
          battleListPlaceholder.textContent = 'Error loading battle information.';
          battleList.appendChild(battleListPlaceholder);
        }
      }
    );
  } catch (error) {
    console.error('Failed to load current battles:', error);
    
    const battleListPlaceholder = document.getElementById('battle-list-placeholder');
    if (battleListPlaceholder) {
      battleListPlaceholder.textContent = 'Error loading battle information.';
    }
    
    showNotification('Failed to load battle information. Please try again later.', 'error');
  }
}

/**
 * View battle details.
 * 
 * @param {number} battleId - The battle ID.
 */
function viewBattleDetails(battleId) {
  // Find the battle in the cache
  const battle = AppState.cache.currentBattles.find(b => b.id === battleId);
  
  if (!battle) {
    showNotification('Battle not found.', 'error');
    return;
  }
  
  // Show battle details in a modal
  showBattleDetailsModal(battle);
}

/**
 * Show battle details in a modal.
 * 
 * @param {Object} battle - The battle object.
 */
function showBattleDetailsModal(battle) {
  // Create modal container if it doesn't exist
  let modalContainer = document.getElementById('modal-container');
  
  if (!modalContainer) {
    modalContainer = createElement('div', {
      id: 'modal-container',
      style: {
        position: 'fixed',
        top: '0',
        left: '0',
        width: '100%',
        height: '100%',
        backgroundColor: 'rgba(0, 0, 0, 0.5)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: '1000'
      }
    });
    
    document.body.appendChild(modalContainer);
  }
  
  // Clear the modal container
  while (modalContainer.firstChild) {
    modalContainer.removeChild(modalContainer.firstChild);
  }
  
  // Create the modal content
  const modalContent = createElement('div', {
    class: 'modal-content',
    style: {
      backgroundColor: 'white',
      padding: '20px',
      borderRadius: '5px',
      maxWidth: '500px',
      width: '100%'
    }
  }, [
    createElement('h2', {}, `Battle #${battle.id}`),
    createElement('p', {}, `Status: ${battle.status}`),
    createElement('p', {}, `Challenger: ${battle.challenger.substring(0, 8)}...${battle.challenger.substring(battle.challenger.length - 8)}`),
    createElement('p', {}, `Challenger Pet: #${battle.challengerPetId}`),
    createElement('p', {}, `Opponent: ${battle.opponent.substring(0, 8)}...${battle.opponent.substring(battle.opponent.length - 8)}`),
    createElement('p', {}, `Opponent Pet: #${battle.opponentPetId}`),
    createElement('button', {
      style: {
        marginTop: '20px',
        padding: '10px',
        backgroundColor: '#f44336',
        color: 'white',
        border: 'none',
        borderRadius: '5px',
        cursor: 'pointer'
      },
      onclick: () => modalContainer.style.display = 'none'
    }, 'Close')
  ]);
  
  modalContainer.appendChild(modalContent);
  modalContainer.style.display = 'flex';
}

/**
 * Join a battle.
 * 
 * @param {number} battleId - The battle ID.
 */
async function joinBattle(battleId) {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  // Find the battle in the cache
  const battle = AppState.cache.currentBattles.find(b => b.id === battleId);
  
  if (!battle) {
    showNotification('Battle not found.', 'error');
    return;
  }
  
  // Check if the user is the opponent
  if (battle.opponent !== AppState.currentAccount) {
    showNotification('You are not the opponent in this battle.', 'error');
    return;
  }
  
  // Check if the battle is waiting
  if (battle.status !== 'Waiting') {
    showNotification('This battle is not waiting for an opponent.', 'error');
    return;
  }
  
  try {
    showNotification(`Joining battle #${battleId}...`, 'info');
    
    // Create the transaction
    const tx = AppState.api.tx.critterBattle.joinBattle(battleId, battle.opponentPetId);
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          showNotification(`Successfully joined battle #${battleId}!`, 'success');
        } else {
          showNotification(`Failed to join battle: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to join battle:', error);
    showNotification(`Failed to join battle: ${error.message}`, 'error');
  }
}

// ===== Quests =====

/**
 * Load available quests.
 */
async function loadAvailableQuests() {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.quests) {
      await AppState.subscriptions.quests();
      AppState.subscriptions.quests = null;
    }
    
    const questsList = document.getElementById('available-quests-list');
    const questsPlaceholder = document.getElementById('available-quests-placeholder');
    
    if (!questsList || !questsPlaceholder) return;
    
    questsPlaceholder.textContent = 'Loading available quests...';
    
    // Subscribe to available quests
    AppState.subscriptions.quests = await AppState.api.query.critterQuests.quests.entries(
      async (entries) => {
        try {
          // Clear the list
          while (questsList.firstChild) {
            questsList.removeChild(questsList.firstChild);
          }
          
          // Process quests
          AppState.cache.availableQuests = [];
          
          if (entries.length === 0) {
            questsPlaceholder.textContent = 'No quests available at the moment.';
            questsList.appendChild(questsPlaceholder);
            return;
          }
          
          // Hide the placeholder
          questsPlaceholder.style.display = 'none';
          
          // Process quests
          for (const [key, quest] of entries) {
            const questId = key.args[0].toNumber();
            const questData = quest.unwrap();
            
            const name = questData.name.toString();
            const description = questData.description.toString();
            const requiredLevel = questData.requiredLevel.toNumber();
            const rewardBits = questData.rewardBits.toNumber();
            const rewardAura = questData.rewardAura.toNumber();
            
            AppState.cache.availableQuests.push({
              id: questId,
              name,
              description,
              requiredLevel,
              rewardBits,
              rewardAura
            });
            
            // Create quest item
            const listItem = createElement('li', {
              class: 'quest-item',
              'data-quest-id': questId
            }, [
              createElement('div', { class: 'quest-header' }, [
                createElement('h3', {}, name),
                createElement('span', { class: 'quest-level' }, `Required Level: ${requiredLevel}`)
              ]),
              createElement('div', { class: 'quest-details' }, [
                createElement('p', {}, description),
                createElement('p', { class: 'quest-rewards' }, `Rewards: ${rewardBits} BITS, ${rewardAura} AURA`)
              ]),
              createElement('div', { class: 'quest-actions' }, [
                createElement('button', {
                  class: 'quest-action-button',
                  onclick: () => startQuest(questId)
                }, 'Start Quest')
              ])
            ]);
            
            questsList.appendChild(listItem);
          }
        } catch (error) {
          console.error('Failed to process available quests:', error);
          questsPlaceholder.textContent = 'Error loading available quests.';
          questsList.appendChild(questsPlaceholder);
        }
      }
    );
  } catch (error) {
    console.error('Failed to load available quests:', error);
    
    const questsPlaceholder = document.getElementById('available-quests-placeholder');
    if (questsPlaceholder) {
      questsPlaceholder.textContent = 'Error loading available quests.';
    }
    
    showNotification('Failed to load available quests. Please try again later.', 'error');
  }
}

/**
 * Load completed quests.
 * 
 * @param {string} accountAddress - The account address.
 */
async function loadCompletedQuests(accountAddress) {
  try {
    const completedQuestsList = document.getElementById('completed-quests-list');
    const completedQuestsPlaceholder = document.getElementById('completed-quests-placeholder');
    
    if (!completedQuestsList || !completedQuestsPlaceholder) return;
    
    completedQuestsPlaceholder.textContent = 'Loading completed quests...';
    
    // Get completed quests
    const completedQuests = await AppState.api.query.critterQuests.completedQuests(accountAddress);
    const completedQuestsArray = completedQuests.toArray().map(id => id.toNumber());
    
    // Clear the list
    while (completedQuestsList.firstChild) {
      completedQuestsList.removeChild(completedQuestsList.firstChild);
    }
    
    if (completedQuestsArray.length === 0) {
      completedQuestsPlaceholder.textContent = 'You haven\'t completed any quests yet.';
      completedQuestsList.appendChild(completedQuestsPlaceholder);
      return;
    }
    
    // Hide the placeholder
    completedQuestsPlaceholder.style.display = 'none';
    
    // Fetch quest details in parallel
    const questDetails = await Promise.all(
      completedQuestsArray.map(id => AppState.api.query.critterQuests.quests(id))
    );
    
    // Process quest details
    for (let i = 0; i < completedQuestsArray.length; i++) {
      const questId = completedQuestsArray[i];
      const questDetail = questDetails[i];
      
      if (questDetail.isSome) {
        const quest = questDetail.unwrap();
        const name = quest.name.toString();
        const description = quest.description.toString();
        
        // Create quest item
        const listItem = createElement('li', {
          class: 'completed-quest-item',
          'data-quest-id': questId
        }, [
          createElement('div', { class: 'quest-header' }, [
            createElement('h3', {}, name),
            createElement('span', { class: 'quest-status' }, 'Completed')
          ]),
          createElement('div', { class: 'quest-details' }, [
            createElement('p', {}, description)
          ])
        ]);
        
        completedQuestsList.appendChild(listItem);
      }
    }
  } catch (error) {
    console.error('Failed to load completed quests:', error);
    
    const completedQuestsPlaceholder = document.getElementById('completed-quests-placeholder');
    if (completedQuestsPlaceholder) {
      completedQuestsPlaceholder.textContent = 'Error loading completed quests.';
    }
  }
}

/**
 * Start a quest.
 * 
 * @param {number} questId - The quest ID.
 */
async function startQuest(questId) {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  // Find the quest in the cache
  const quest = AppState.cache.availableQuests.find(q => q.id === questId);
  
  if (!quest) {
    showNotification('Quest not found.', 'error');
    return;
  }
  
  try {
    showNotification(`Starting quest: ${quest.name}...`, 'info');
    
    // Create the transaction
    const tx = AppState.api.tx.critterQuests.startQuest(questId);
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          showNotification(`Successfully started quest: ${quest.name}!`, 'success');
        } else {
          showNotification(`Failed to start quest: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to start quest:', error);
    showNotification(`Failed to start quest: ${error.message}`, 'error');
  }
}

// ===== Breeding =====

/**
 * Populate the breeding pet select dropdowns.
 * 
 * @param {string} accountAddress - The account address.
 */
function populateBreedingPetSelects(accountAddress) {
  const parentASelect = document.getElementById('parent-a-select');
  const parentBSelect = document.getElementById('parent-b-select');
  
  if (!parentASelect || !parentBSelect) return;
  
  // Clear the selects
  while (parentASelect.firstChild) {
    parentASelect.removeChild(parentASelect.firstChild);
  }
  
  while (parentBSelect.firstChild) {
    parentBSelect.removeChild(parentBSelect.firstChild);
  }
  
  // Add default options
  parentASelect.appendChild(
    createElement('option', { value: '' }, '-- Select one of your pets --')
  );
  
  parentBSelect.appendChild(
    createElement('option', { value: '' }, '-- Select one of your pets --')
  );
  
  // Add owned pets
  AppState.cache.ownedNfts.forEach(pet => {
    const optionText = `${pet.id} - ${pet.species} (Level ${pet.level})`;
    
    parentASelect.appendChild(
      createElement('option', { value: pet.id }, optionText)
    );
    
    parentBSelect.appendChild(
      createElement('option', { value: pet.id }, optionText)
    );
  });
  
  // Add event listeners
  parentASelect.addEventListener('change', () => updateParentDetails('a', parentASelect.value));
  parentBSelect.addEventListener('change', () => updateParentDetails('b', parentBSelect.value));
}

/**
 * Update parent details.
 * 
 * @param {string} parent - The parent ('a' or 'b').
 * @param {string} petId - The pet ID.
 */
function updateParentDetails(parent, petId) {
  const detailsDiv = document.getElementById(`parent-${parent}-details`);
  
  if (!detailsDiv) return;
  
  // Clear the details
  while (detailsDiv.firstChild) {
    detailsDiv.removeChild(detailsDiv.firstChild);
  }
  
  if (!petId) {
    detailsDiv.appendChild(
      createElement('p', { style: { fontStyle: 'italic' } }, `Details for Parent ${parent.toUpperCase()} will appear here.`)
    );
    return;
  }
  
  // Find the pet in the cache
  const pet = AppState.cache.ownedNfts.find(p => p.id == petId);
  
  if (!pet) {
    detailsDiv.appendChild(
      createElement('p', { style: { color: 'red' } }, 'Pet not found.')
    );
    return;
  }
  
  // Add pet details
  detailsDiv.appendChild(createElement('p', {}, `Species: ${pet.species}`));
  detailsDiv.appendChild(createElement('p', {}, `Level: ${pet.level}`));
  detailsDiv.appendChild(createElement('p', {}, `Mood: ${pet.mood}/255`));
  detailsDiv.appendChild(createElement('p', {}, `Energy: ${pet.energy}/255`));
  detailsDiv.appendChild(createElement('p', {}, `Aura: ${pet.auraColor}`));
  detailsDiv.appendChild(createElement('p', {}, 'Breeding Cooldown: Ready'));
}

// ===== Daily Claim =====

/**
 * Load next claim time.
 * 
 * @param {string} accountAddress - The account address.
 */
async function loadNextClaimTime(accountAddress) {
  try {
    const nextClaimTimeSpan = document.getElementById('next-claim-time');
    
    if (!nextClaimTimeSpan) return;
    
    // Get the last claim block
    const lastClaimBlock = await AppState.api.query.critterRewards.lastClaim(accountAddress);
    AppState.cache.lastClaimBlock = lastClaimBlock.toNumber();
    
    // Get the current block
    const currentBlock = await AppState.api.query.system.number();
    const currentBlockNumber = currentBlock.toNumber();
    
    // Calculate the next claim block
    const nextClaimBlock = AppState.cache.lastClaimBlock + AppState.constants.claimCooldownPeriodBlocks;
    AppState.cache.nextClaimBlock = nextClaimBlock;
    
    // Update the UI
    if (nextClaimBlock <= currentBlockNumber) {
      nextClaimTimeSpan.textContent = 'Now';
      document.getElementById('claimDailyPtcnButton').disabled = false;
    } else {
      const blocksRemaining = nextClaimBlock - currentBlockNumber;
      const timeRemaining = blocksRemaining * 6; // Assuming 6 seconds per block
      
      nextClaimTimeSpan.textContent = formatTimeRemaining(timeRemaining);
      document.getElementById('claimDailyPtcnButton').disabled = true;
      
      // Start a timer to update the time remaining
      startClaimTimer(timeRemaining);
    }
  } catch (error) {
    console.error('Failed to load next claim time:', error);
    
    const nextClaimTimeSpan = document.getElementById('next-claim-time');
    if (nextClaimTimeSpan) {
      nextClaimTimeSpan.textContent = 'Error';
    }
  }
}

/**
 * Format time remaining.
 * 
 * @param {number} seconds - The time remaining in seconds.
 * @returns {string} The formatted time.
 */
function formatTimeRemaining(seconds) {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const remainingSeconds = seconds % 60;
  
  return `${hours}h ${minutes}m ${remainingSeconds}s`;
}

/**
 * Start a timer to update the time remaining.
 * 
 * @param {number} seconds - The time remaining in seconds.
 */
function startClaimTimer(seconds) {
  // Clear any existing timer
  if (window.claimTimer) {
    clearInterval(window.claimTimer);
  }
  
  let timeRemaining = seconds;
  const nextClaimTimeSpan = document.getElementById('next-claim-time');
  
  window.claimTimer = setInterval(() => {
    timeRemaining--;
    
    if (timeRemaining <= 0) {
      clearInterval(window.claimTimer);
      nextClaimTimeSpan.textContent = 'Now';
      document.getElementById('claimDailyPtcnButton').disabled = false;
    } else {
      nextClaimTimeSpan.textContent = formatTimeRemaining(timeRemaining);
    }
  }, 1000);
}

// ===== Event Handlers =====

/**
 * Handle the claim daily PTCN button click.
 */
async function handleClaimDailyPtcn() {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  try {
    const claimStatus = document.getElementById('claim-ptcn-status');
    claimStatus.textContent = 'Claiming daily PTCN...';
    
    // Create the transaction
    const tx = AppState.api.tx.critterRewards.claimDailyReward();
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          claimStatus.textContent = 'Successfully claimed daily PTCN!';
          showNotification('Successfully claimed daily PTCN!', 'success');
          
          // Update the next claim time
          loadNextClaimTime(AppState.currentAccount);
        } else {
          claimStatus.textContent = `Failed to claim daily PTCN: ${errorMessage}`;
          showNotification(`Failed to claim daily PTCN: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to claim daily PTCN:', error);
    
    const claimStatus = document.getElementById('claim-ptcn-status');
    if (claimStatus) {
      claimStatus.textContent = `Failed to claim daily PTCN: ${error.message}`;
    }
    
    showNotification(`Failed to claim daily PTCN: ${error.message}`, 'error');
  }
}

/**
 * Handle the mint NFT button click.
 */
async function handleMintNft() {
  if (!AppState.api || !AppState.currentAccount) {
    showNotification('Please connect your wallet first.', 'error');
    return;
  }
  
  try {
    const species = document.getElementById('pet-species').value;
    const name = document.getElementById('pet-name').value;
    const mintStatus = document.getElementById('mint-status');
    
    if (!species || !name) {
      mintStatus.textContent = 'Please enter a species and name.';
      return;
    }
    
    mintStatus.textContent = 'Minting pet NFT...';
    
    // Create the transaction
    const tx = AppState.api.tx.critterNfts.mintPet(species, name);
    
    // Sign and send the transaction
    const unsub = await tx.signAndSend(AppState.currentAccount, { nonce: -1 }, (result) => {
      const { status, events } = result;
      
      if (status.isInBlock) {
        console.log(`Transaction included in block: ${status.asInBlock.toString()}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized in block: ${status.asFinalized.toString()}`);
        
        // Check for success or failure events
        let success = false;
        let errorMessage = '';
        
        events.forEach(({ event }) => {
          if (AppState.api.events.system.ExtrinsicSuccess.is(event)) {
            success = true;
          } else if (AppState.api.events.system.ExtrinsicFailed.is(event)) {
            const [dispatchError] = event.data;
            errorMessage = dispatchError.toString();
          }
        });
        
        if (success) {
          mintStatus.textContent = 'Successfully minted pet NFT!';
          showNotification('Successfully minted pet NFT!', 'success');
          
          // Reload owned NFTs
          loadOwnedNfts(AppState.currentAccount);
        } else {
          mintStatus.textContent = `Failed to mint pet NFT: ${errorMessage}`;
          showNotification(`Failed to mint pet NFT: ${errorMessage}`, 'error');
        }
        
        // Unsubscribe
        unsub();
      }
    });
  } catch (error) {
    console.error('Failed to mint pet NFT:', error);
    
    const mintStatus = document.getElementById('mint-status');
    if (mintStatus) {
      mintStatus.textContent = `Failed to mint pet NFT: ${error.message}`;
    }
    
    showNotification(`Failed to mint pet NFT: ${error.message}`, 'error');
  }
}

// ===== Initialization =====

/**
 * Initialize the application.
 */
async function initializeApp() {
  try {
    // Initialize the API
    const api = await initializeApi();
    
    // Set Alice as the current account (for testing)
    const ALICE_ADDRESS = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    setCurrentAccount(ALICE_ADDRESS);
    
    // Load marketplace listings
    await loadMarketplaceListings();
    
    // Load current battles
    await loadCurrentBattles();
    
    // Load available quests
    await loadAvailableQuests();
    
    // Subscribe to system events
    subscribeToSystemEvents();
    
    // Add event listeners
    document.getElementById('claimDailyPtcnButton').addEventListener('click', handleClaimDailyPtcn);
    document.getElementById('mintNftButton').addEventListener('click', handleMintNft);
    
    // Add more event listeners as needed
    
    console.log('Application initialized successfully');
  } catch (error) {
    console.error('Failed to initialize application:', error);
    showNotification('Failed to initialize application. Please try again later.', 'error');
  }
}

/**
 * Subscribe to system events.
 */
async function subscribeToSystemEvents() {
  try {
    // Cancel previous subscription if any
    if (AppState.subscriptions.events) {
      await AppState.subscriptions.events();
      AppState.subscriptions.events = null;
    }
    
    const eventList = document.getElementById('event-list');
    const eventListPlaceholder = document.getElementById('event-list-placeholder');
    
    if (!eventList || !eventListPlaceholder) return;
    
    // Subscribe to system events
    AppState.subscriptions.events = await AppState.api.query.system.events((events) => {
      // Process events
      const relevantEvents = [];
      
      events.forEach((record) => {
        const { event } = record;
        
        // Filter for relevant events
        if (
          event.section === 'critterNfts' ||
          event.section === 'critterMarketplace' ||
          event.section === 'critterBattle' ||
          event.section === 'critterQuests' ||
          event.section === 'critterRewards'
        ) {
          relevantEvents.push({
            section: event.section,
            method: event.method,
            data: event.data.toString()
          });
        }
      });
      
      // Update the event list
      if (relevantEvents.length > 0) {
        // Clear the placeholder
        eventListPlaceholder.style.display = 'none';
        
        // Add new events to the list
        relevantEvents.forEach((event) => {
          const listItem = createElement('li', {
            class: 'event-item'
          }, [
            createElement('span', { class: 'event-section' }, event.section),
            createElement('span', { class: 'event-method' }, event.method),
            createElement('span', { class: 'event-data' }, event.data)
          ]);
          
          // Add to the beginning of the list
          eventList.insertBefore(listItem, eventList.firstChild);
          
          // Limit the number of events displayed
          if (eventList.children.length > 10) {
            eventList.removeChild(eventList.lastChild);
          }
        });
      } else if (eventList.children.length === 0) {
        // Show the placeholder if there are no events
        eventListPlaceholder.style.display = 'block';
      }
    });
  } catch (error) {
    console.error('Failed to subscribe to system events:', error);
  }
}

// Initialize the application when the DOM is loaded
document.addEventListener('DOMContentLoaded', initializeApp);