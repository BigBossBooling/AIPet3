/**
 * CritterCraft API Integration
 * 
 * This module provides a JavaScript API for interacting with the CritterCraft blockchain.
 * It abstracts the complexity of the blockchain interactions and provides a simple interface
 * for the frontend application.
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import { formatBalance } from '@polkadot/util';

/**
 * CritterCraftAPI class for interacting with the CritterCraft blockchain
 */
class CritterCraftAPI {
  constructor() {
    this.api = null;
    this.account = null;
    this.isConnected = false;
  }

  /**
   * Initialize the API connection
   * @param {string} endpoint - WebSocket endpoint for the blockchain node
   * @returns {Promise<boolean>} - True if connection is successful
   */
  async connect(endpoint = 'ws://127.0.0.1:9944') {
    try {
      const provider = new WsProvider(endpoint);
      this.api = await ApiPromise.create({ provider });
      this.isConnected = true;
      console.log('Connected to CritterCraft blockchain');
      return true;
    } catch (error) {
      console.error('Failed to connect to CritterCraft blockchain:', error);
      return false;
    }
  }

  /**
   * Connect to the wallet and get accounts
   * @returns {Promise<Array>} - List of available accounts
   */
  async connectWallet() {
    try {
      const extensions = await web3Enable('CritterCraft');
      if (extensions.length === 0) {
        throw new Error('No extension found. Please install Polkadot.js extension.');
      }

      const allAccounts = await web3Accounts();
      return allAccounts;
    } catch (error) {
      console.error('Failed to connect wallet:', error);
      throw error;
    }
  }

  /**
   * Set the active account
   * @param {Object} account - Account to set as active
   */
  setAccount(account) {
    this.account = account;
    console.log('Active account set:', account.address);
  }

  /**
   * Get the account balance
   * @param {string} address - Account address
   * @returns {Promise<string>} - Formatted balance
   */
  async getBalance(address = this.account?.address) {
    if (!address) throw new Error('No account selected');
    
    const { data: { free, reserved } } = await this.api.query.system.account(address);
    const total = free.add(reserved);
    return formatBalance(total, { withSi: true, decimals: 12 });
  }

  /**
   * Sign and send a transaction
   * @param {Object} tx - Transaction to send
   * @returns {Promise<Object>} - Transaction result
   */
  async signAndSend(tx) {
    if (!this.account) throw new Error('No account selected');
    
    const injector = await web3FromAddress(this.account.address);
    return new Promise((resolve, reject) => {
      tx.signAndSend(this.account.address, { signer: injector.signer }, ({ status, events, dispatchError }) => {
        if (status.isInBlock || status.isFinalized) {
          if (dispatchError) {
            let errorInfo;
            if (dispatchError.isModule) {
              const decoded = this.api.registry.findMetaError(dispatchError.asModule);
              errorInfo = `${decoded.section}.${decoded.name}`;
            } else {
              errorInfo = dispatchError.toString();
            }
            reject(new Error(`Transaction failed: ${errorInfo}`));
          } else {
            resolve({ status, events });
          }
        }
      }).catch(error => {
        reject(error);
      });
    });
  }

  // ===== User Profiles API =====

  /**
   * Create a user profile
   * @param {string} username - Username
   * @param {string} bio - User bio
   * @param {string} avatarUri - Avatar URI
   * @returns {Promise<Object>} - Transaction result
   */
  async createProfile(username, bio, avatarUri) {
    const tx = this.api.tx.profiles.createProfile(username, bio, avatarUri);
    return this.signAndSend(tx);
  }

  /**
   * Update a user profile
   * @param {string} username - New username (optional)
   * @param {string} bio - New bio (optional)
   * @param {string} avatarUri - New avatar URI (optional)
   * @returns {Promise<Object>} - Transaction result
   */
  async updateProfile(username = null, bio = null, avatarUri = null) {
    const tx = this.api.tx.profiles.updateProfile(username, bio, avatarUri);
    return this.signAndSend(tx);
  }

  /**
   * Set user status
   * @param {string} status - New status
   * @returns {Promise<Object>} - Transaction result
   */
  async setStatus(status) {
    const tx = this.api.tx.profiles.setStatus(status);
    return this.signAndSend(tx);
  }

  /**
   * Send a friend request
   * @param {string} targetAccount - Target account address
   * @returns {Promise<Object>} - Transaction result
   */
  async sendFriendRequest(targetAccount) {
    const tx = this.api.tx.profiles.sendFriendRequest(targetAccount);
    return this.signAndSend(tx);
  }

  /**
   * Accept a friend request
   * @param {string} requesterAccount - Requester account address
   * @returns {Promise<Object>} - Transaction result
   */
  async acceptFriendRequest(requesterAccount) {
    const tx = this.api.tx.profiles.acceptFriendRequest(requesterAccount);
    return this.signAndSend(tx);
  }

  /**
   * Get user profile
   * @param {string} address - Account address
   * @returns {Promise<Object>} - User profile
   */
  async getProfile(address = this.account?.address) {
    if (!address) throw new Error('No account selected');
    
    const profile = await this.api.query.profiles.userProfiles(address);
    return profile.toHuman();
  }

  // ===== Pet NFTs API =====

  /**
   * Mint a new pet NFT
   * @param {string} name - Pet name
   * @param {string} description - Pet description
   * @param {number} petType - Pet type
   * @returns {Promise<Object>} - Transaction result
   */
  async mintPet(name, description, petType) {
    const tx = this.api.tx.nfts.mint(name, description, petType);
    return this.signAndSend(tx);
  }

  /**
   * Transfer a pet to another account
   * @param {number} petId - Pet ID
   * @param {string} recipient - Recipient account address
   * @returns {Promise<Object>} - Transaction result
   */
  async transferPet(petId, recipient) {
    const tx = this.api.tx.nfts.transfer(petId, recipient);
    return this.signAndSend(tx);
  }

  /**
   * Evolve a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   */
  async evolvePet(petId) {
    const tx = this.api.tx.nfts.evolve(petId);
    return this.signAndSend(tx);
  }

  /**
   * Change a pet's name
   * @param {number} petId - Pet ID
   * @param {string} name - New name
   * @returns {Promise<Object>} - Transaction result
   */
  async changePetName(petId, name) {
    const tx = this.api.tx.nfts.changeName(petId, name);
    return this.signAndSend(tx);
  }

  /**
   * Get pet details
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Pet details
   */
  async getPet(petId) {
    const pet = await this.api.query.nfts.pets(petId);
    return pet.toHuman();
  }

  /**
   * Get pets owned by an account
   * @param {string} address - Account address
   * @returns {Promise<Array>} - List of pet IDs
   */
  async getPetsByOwner(address = this.account?.address) {
    if (!address) throw new Error('No account selected');
    
    const petIds = await this.api.query.nfts.petsByOwner(address);
    return petIds.toHuman();
  }

  // ===== Pet Status API =====

  /**
   * Initialize a pet's status
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   */
  async initializePetStatus(petId) {
    const tx = this.api.tx.petStatus.initializePetStatus(petId);
    return this.signAndSend(tx);
  }

  /**
   * Feed a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   */
  async feedPet(petId) {
    const tx = this.api.tx.petStatus.feedPet(petId);
    return this.signAndSend(tx);
  }

  /**
   * Rest a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   */
  async restPet(petId) {
    const tx = this.api.tx.petStatus.restPet(petId);
    return this.signAndSend(tx);
  }

  /**
   * Play with a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   */
  async playWithPet(petId) {
    const tx = this.api.tx.petStatus.playWithPet(petId);
    return this.signAndSend(tx);
  }

  /**
   * Groom a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   */
  async groomPet(petId) {
    const tx = this.api.tx.petStatus.groomPet(petId);
    return this.signAndSend(tx);
  }

  /**
   * Socialize a pet with another pet
   * @param {number} petId - Pet ID
   * @param {number} targetPetId - Target pet ID
   * @returns {Promise<Object>} - Transaction result
   */
  async socializePet(petId, targetPetId) {
    const tx = this.api.tx.petStatus.socializePet(petId, targetPetId);
    return this.signAndSend(tx);
  }

  /**
   * Get pet status
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Pet status
   */
  async getPetStatus(petId) {
    const status = await this.api.query.petStatus.petStatuses(petId);
    return status.toHuman();
  }

  /**
   * Get pet needs
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Pet needs
   */
  async getPetNeeds(petId) {
    const needs = await this.api.query.petStatus.petNeedsStorage(petId);
    return needs.toHuman();
  }

  /**
   * Get pet stats
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Pet stats
   */
  async getPetStats(petId) {
    const stats = await this.api.query.petStatus.petStatsStorage(petId);
    return stats.toHuman();
  }

  // ===== Mini-Games API =====

  /**
   * Start a mini-game
   * @param {number} petId - Pet ID
   * @param {number} gameType - Game type
   * @param {number} difficulty - Game difficulty
   * @returns {Promise<Object>} - Transaction result
   */
  async startGame(petId, gameType, difficulty) {
    const tx = this.api.tx.minigames.startGame(petId, gameType, difficulty);
    return this.signAndSend(tx);
  }

  /**
   * Submit a score for a mini-game
   * @param {number} gameId - Game ID
   * @param {number} score - Score
   * @returns {Promise<Object>} - Transaction result
   */
  async submitScore(gameId, score) {
    const tx = this.api.tx.minigames.submitScore(gameId, score);
    return this.signAndSend(tx);
  }

  /**
   * Cancel a mini-game
   * @param {number} gameId - Game ID
   * @returns {Promise<Object>} - Transaction result
   */
  async cancelGame(gameId) {
    const tx = this.api.tx.minigames.cancelGame(gameId);
    return this.signAndSend(tx);
  }

  /**
   * Get game details
   * @param {number} gameId - Game ID
   * @returns {Promise<Object>} - Game details
   */
  async getGame(gameId) {
    const game = await this.api.query.minigames.gameInstances(gameId);
    return game.toHuman();
  }

  /**
   * Get active games by player
   * @param {string} address - Account address
   * @returns {Promise<Array>} - List of game IDs
   */
  async getActiveGamesByPlayer(address = this.account?.address) {
    if (!address) throw new Error('No account selected');
    
    const gameIds = await this.api.query.minigames.activeGamesByPlayer(address);
    return gameIds.toHuman();
  }

  // ===== Jobs API =====

  /**
   * Start a job
   * @param {number} petId - Pet ID
   * @param {number} jobType - Job type
   * @param {number} duration - Job duration
   * @returns {Promise<Object>} - Transaction result
   */
  async startJob(petId, jobType, duration) {
    const tx = this.api.tx.jobs.startJob(petId, jobType, duration);
    return this.signAndSend(tx);
  }

  /**
   * Complete a job
   * @param {number} jobId - Job ID
   * @returns {Promise<Object>} - Transaction result
   */
  async completeJob(jobId) {
    const tx = this.api.tx.jobs.completeJob(jobId);
    return this.signAndSend(tx);
  }

  /**
   * Cancel a job
   * @param {number} jobId - Job ID
   * @returns {Promise<Object>} - Transaction result
   */
  async cancelJob(jobId) {
    const tx = this.api.tx.jobs.cancelJob(jobId);
    return this.signAndSend(tx);
  }

  /**
   * Get job details
   * @param {number} jobId - Job ID
   * @returns {Promise<Object>} - Job details
   */
  async getJob(jobId) {
    const job = await this.api.query.jobs.jobInstances(jobId);
    return job.toHuman();
  }

  /**
   * Get active jobs by owner
   * @param {string} address - Account address
   * @returns {Promise<Array>} - List of job IDs
   */
  async getActiveJobsByOwner(address = this.account?.address) {
    if (!address) throw new Error('No account selected');
    
    const jobIds = await this.api.query.jobs.activeJobsByOwner(address);
    return jobIds.toHuman();
  }

  // ===== Daycare API =====

  /**
   * Create a daycare
   * @param {string} name - Daycare name
   * @param {string} description - Daycare description
   * @param {number} feePerBlock - Fee per block
   * @returns {Promise<Object>} - Transaction result
   */
  async createDaycare(name, description, feePerBlock) {
    const tx = this.api.tx.daycare.createDaycare(name, description, feePerBlock);
    return this.signAndSend(tx);
  }

  /**
   * Update a daycare
   * @param {number} daycareId - Daycare ID
   * @param {string} name - New name (optional)
   * @param {string} description - New description (optional)
   * @param {number} feePerBlock - New fee per block (optional)
   * @param {number} status - New status (optional)
   * @returns {Promise<Object>} - Transaction result
   */
  async updateDaycare(daycareId, name = null, description = null, feePerBlock = null, status = null) {
    const tx = this.api.tx.daycare.updateDaycare(daycareId, name, description, feePerBlock, status);
    return this.signAndSend(tx);
  }

  /**
   * Create a listing
   * @param {number} daycareId - Daycare ID
   * @param {number} petId - Pet ID
   * @param {number} duration - Listing duration
   * @returns {Promise<Object>} - Transaction result
   */
  async createListing(daycareId, petId, duration) {
    const tx = this.api.tx.daycare.createListing(daycareId, petId, duration);
    return this.signAndSend(tx);
  }

  /**
   * Accept a listing as a caregiver
   * @param {number} listingId - Listing ID
   * @returns {Promise<Object>} - Transaction result
   */
  async acceptListing(listingId) {
    const tx = this.api.tx.daycare.acceptListing(listingId);
    return this.signAndSend(tx);
  }

  /**
   * Complete a listing
   * @param {number} listingId - Listing ID
   * @returns {Promise<Object>} - Transaction result
   */
  async completeListing(listingId) {
    const tx = this.api.tx.daycare.completeListing(listingId);
    return this.signAndSend(tx);
  }

  /**
   * Cancel a listing
   * @param {number} listingId - Listing ID
   * @returns {Promise<Object>} - Transaction result
   */
  async cancelListing(listingId) {
    const tx = this.api.tx.daycare.cancelListing(listingId);
    return this.signAndSend(tx);
  }

  /**
   * Perform a care action
   * @param {number} listingId - Listing ID
   * @param {number} action - Care action
   * @param {number} targetPetId - Target pet ID (optional)
   * @returns {Promise<Object>} - Transaction result
   */
  async performCareAction(listingId, action, targetPetId = null) {
    const tx = this.api.tx.daycare.performCareAction(listingId, action, targetPetId);
    return this.signAndSend(tx);
  }

  /**
   * Get daycare details
   * @param {number} daycareId - Daycare ID
   * @returns {Promise<Object>} - Daycare details
   */
  async getDaycare(daycareId) {
    const daycare = await this.api.query.daycare.daycares(daycareId);
    return daycare.toHuman();
  }

  /**
   * Get listing details
   * @param {number} listingId - Listing ID
   * @returns {Promise<Object>} - Listing details
   */
  async getListing(listingId) {
    const listing = await this.api.query.daycare.listings(listingId);
    return listing.toHuman();
  }

  /**
   * Get daycares by owner
   * @param {string} address - Account address
   * @returns {Promise<Array>} - List of daycare IDs
   */
  async getDaycaresByOwner(address = this.account?.address) {
    if (!address) throw new Error('No account selected');
    
    const daycareIds = await this.api.query.daycare.daycaresByOwner(address);
    return daycareIds.toHuman();
  }

  /**
   * Get listings by daycare
   * @param {number} daycareId - Daycare ID
   * @returns {Promise<Array>} - List of listing IDs
   */
  async getListingsByDaycare(daycareId) {
    const listingIds = await this.api.query.daycare.listingsByDaycare(daycareId);
    return listingIds.toHuman();
  }
}

// Export a singleton instance
const critterCraftAPI = new CritterCraftAPI();
export default critterCraftAPI;