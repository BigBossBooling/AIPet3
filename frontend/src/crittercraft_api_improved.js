/**
 * CritterCraft API Integration (Improved Version)
 * 
 * This module provides a JavaScript API for interacting with the CritterCraft blockchain.
 * It abstracts the complexity of the blockchain interactions and provides a simple interface
 * for the frontend application.
 * 
 * Features:
 * - Connection management with automatic reconnection
 * - Wallet integration with Polkadot.js extension
 * - Caching for frequently accessed data
 * - Comprehensive error handling
 * - Transaction management
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import { formatBalance } from '@polkadot/util';

// Error types for better error handling
export class ConnectionError extends Error {
  constructor(message, originalError) {
    super(message);
    this.name = 'ConnectionError';
    this.originalError = originalError;
  }
}

export class WalletError extends Error {
  constructor(message, originalError) {
    super(message);
    this.name = 'WalletError';
    this.originalError = originalError;
  }
}

export class TransactionError extends Error {
  constructor(message, originalError) {
    super(message);
    this.name = 'TransactionError';
    this.originalError = originalError;
  }
}

export class QueryError extends Error {
  constructor(message, originalError) {
    super(message);
    this.name = 'QueryError';
    this.originalError = originalError;
  }
}

/**
 * Simple cache implementation for API responses
 */
class ApiCache {
  constructor(maxSize = 100, ttl = 60000) { // Default TTL: 1 minute
    this.cache = new Map();
    this.maxSize = maxSize;
    this.ttl = ttl;
  }

  /**
   * Get a value from cache
   * @param {string} key - Cache key
   * @returns {any|null} - Cached value or null if not found/expired
   */
  get(key) {
    if (!this.cache.has(key)) return null;
    
    const { value, expiry } = this.cache.get(key);
    if (Date.now() > expiry) {
      this.cache.delete(key);
      return null;
    }
    
    return value;
  }

  /**
   * Set a value in cache
   * @param {string} key - Cache key
   * @param {any} value - Value to cache
   * @param {number} customTtl - Custom TTL in ms (optional)
   */
  set(key, value, customTtl) {
    // If cache is full, remove oldest entry
    if (this.cache.size >= this.maxSize) {
      const oldestKey = this.cache.keys().next().value;
      this.cache.delete(oldestKey);
    }
    
    const ttl = customTtl || this.ttl;
    this.cache.set(key, {
      value,
      expiry: Date.now() + ttl
    });
  }

  /**
   * Clear the entire cache or a specific key
   * @param {string} key - Specific key to clear (optional)
   */
  clear(key) {
    if (key) {
      this.cache.delete(key);
    } else {
      this.cache.clear();
    }
  }
}

/**
 * CritterCraftAPI class for interacting with the CritterCraft blockchain
 */
class CritterCraftAPI {
  constructor() {
    this.api = null;
    this.provider = null;
    this.account = null;
    this.isConnected = false;
    this.connectionAttempts = 0;
    this.maxConnectionAttempts = 3;
    this.reconnectTimeout = null;
    this.cache = new ApiCache();
    
    // Bind all methods to ensure 'this' context is preserved
    this.bindMethods();
  }
  
  /**
   * Bind all methods to ensure 'this' context is preserved
   * @private
   */
  bindMethods() {
    // Core methods
    this.connect = this.connect.bind(this);
    this.disconnect = this.disconnect.bind(this);
    this.reconnect = this.reconnect.bind(this);
    
    // Wallet methods
    this.connectWallet = this.connectWallet.bind(this);
    this.setAccount = this.setAccount.bind(this);
    this.getWalletStatus = this.getWalletStatus.bind(this);
    this.getConnectionStatus = this.getConnectionStatus.bind(this);
    
    // Transaction methods
    this.validateTransaction = this.validateTransaction.bind(this);
    this.estimateTransactionFee = this.estimateTransactionFee.bind(this);
    this.createBatch = this.createBatch.bind(this);
    this.signAndSend = this.signAndSend.bind(this);
    
    // Query methods
    this.getBalance = this.getBalance.bind(this);
  }

  /**
   * Disconnect from the blockchain
   * @returns {Promise<void>}
   */
  async disconnect() {
    if (this.provider) {
      try {
        // Remove event listeners
        this.provider.removeAllListeners();
        
        // Disconnect provider
        await this.provider.disconnect();
        
        // Clear API instance
        if (this.api) {
          await this.api.disconnect();
          this.api = null;
        }
        
        this.isConnected = false;
        this.connectionAttempts = 0;
        
        // Clear any pending reconnect
        if (this.reconnectTimeout) {
          clearTimeout(this.reconnectTimeout);
          this.reconnectTimeout = null;
        }
        
        console.log('Disconnected from CritterCraft blockchain');
      } catch (error) {
        console.error('Error during disconnect:', error);
      }
    }
  }
  
  /**
   * Initialize the API connection
   * @param {string} endpoint - WebSocket endpoint for the blockchain node
   * @returns {Promise<boolean>} - True if connection is successful
   * @throws {ConnectionError} - If connection fails after max attempts
   */
  async connect(endpoint = 'ws://127.0.0.1:9944') {
    try {
      // Reset connection attempts on manual connect
      this.connectionAttempts = 0;
      
      // Clear any pending reconnect
      if (this.reconnectTimeout) {
        clearTimeout(this.reconnectTimeout);
        this.reconnectTimeout = null;
      }
      
      // Create provider and setup event handlers
      this.provider = new WsProvider(endpoint);
      
      // Handle disconnection events
      this.provider.on('error', this.reconnect);
      this.provider.on('disconnected', this.reconnect);
      
      // Create API instance
      this.api = await ApiPromise.create({ provider: this.provider });
      
      // Subscribe to connected event
      this.api.on('connected', () => {
        this.isConnected = true;
        this.connectionAttempts = 0;
        console.log('Connected to CritterCraft blockchain');
      });
      
      // Subscribe to disconnected event
      this.api.on('disconnected', this.reconnect);
      
      this.isConnected = true;
      console.log('Connected to CritterCraft blockchain');
      return true;
    } catch (error) {
      console.error('Failed to connect to CritterCraft blockchain:', error);
      
      // Attempt to reconnect
      return this.reconnect(error);
    }
  }
  
  /**
   * Attempt to reconnect to the blockchain
   * @param {Error} error - The error that caused disconnection
   * @returns {Promise<boolean>} - True if reconnection is successful
   * @throws {ConnectionError} - If reconnection fails after max attempts
   */
  async reconnect(error) {
    this.isConnected = false;
    this.connectionAttempts++;
    
    if (this.connectionAttempts > this.maxConnectionAttempts) {
      throw new ConnectionError(
        `Failed to connect after ${this.maxConnectionAttempts} attempts. Please check your network connection.`,
        error
      );
    }
    
    console.log(`Connection attempt ${this.connectionAttempts}/${this.maxConnectionAttempts}...`);
    
    // Exponential backoff for reconnection attempts
    const backoffTime = Math.min(1000 * Math.pow(2, this.connectionAttempts - 1), 30000);
    
    return new Promise((resolve, reject) => {
      this.reconnectTimeout = setTimeout(async () => {
        try {
          // Close existing provider if any
          if (this.provider) {
            this.provider.disconnect();
          }
          
          // Try to connect again
          const success = await this.connect();
          resolve(success);
        } catch (reconnectError) {
          reject(reconnectError);
        }
      }, backoffTime);
    });
  }

  /**
   * Connect to the wallet and get accounts
   * @returns {Promise<Array>} - List of available accounts
   * @throws {WalletError} - If wallet connection fails
   */
  async connectWallet() {
    try {
      const extensions = await web3Enable('CritterCraft');
      if (extensions.length === 0) {
        throw new WalletError(
          'No extension found. Please install Polkadot.js extension and allow access to CritterCraft.',
          new Error('No extensions')
        );
      }

      const allAccounts = await web3Accounts();
      if (allAccounts.length === 0) {
        throw new WalletError(
          'No accounts found in the Polkadot.js extension. Please create or import an account first.',
          new Error('No accounts')
        );
      }
      
      return allAccounts;
    } catch (error) {
      if (error instanceof WalletError) {
        throw error;
      }
      console.error('Failed to connect wallet:', error);
      throw new WalletError('Failed to connect to wallet', error);
    }
  }

  /**
   * Set the active account
   * @param {Object} account - Account to set as active
   * @throws {WalletError} - If account is invalid
   */
  setAccount(account) {
    if (!account || !account.address) {
      throw new WalletError('Invalid account provided');
    }
    
    this.account = account;
    
    // Clear cache when changing accounts
    this.cache.clear();
    
    console.log('Active account set:', account.address);
  }
  
  /**
   * Get the current wallet status
   * @returns {Object} - Wallet status object with connected state and account info
   */
  getWalletStatus() {
    return {
      connected: !!this.account,
      account: this.account,
      address: this.account?.address || null
    };
  }
  
  /**
   * Get the current connection status
   * @returns {Object} - Connection status object
   */
  getConnectionStatus() {
    return {
      connected: this.isConnected,
      endpoint: this.provider?.endpoint || null,
      chainName: this.api?.runtimeChain?.toString() || null,
      chainVersion: this.api?.runtimeVersion?.specVersion?.toNumber() || null
    };
  }

  /**
   * Get the account balance
   * @param {string} address - Account address
   * @returns {Promise<string>} - Formatted balance
   * @throws {QueryError} - If query fails
   */
  async getBalance(address = this.account?.address) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    if (!address) {
      throw new QueryError('No account selected');
    }
    
    // Check cache first
    const cacheKey = `balance:${address}`;
    const cachedBalance = this.cache.get(cacheKey);
    if (cachedBalance) {
      return cachedBalance;
    }
    
    try {
      const { data: { free, reserved } } = await this.api.query.system.account(address);
      const total = free.add(reserved);
      const formattedBalance = formatBalance(total, { withSi: true, decimals: 12 });
      
      // Cache the result for 10 seconds
      this.cache.set(cacheKey, formattedBalance, 10000);
      
      return formattedBalance;
    } catch (error) {
      console.error('Failed to get balance:', error);
      throw new QueryError(`Failed to get balance for ${address}`, error);
    }
  }

  /**
   * Estimate transaction fees
   * @param {Object} tx - Transaction to estimate fees for
   * @returns {Promise<Object>} - Fee estimation result
   * @throws {ConnectionError} - If not connected to blockchain
   * @throws {QueryError} - If fee estimation fails
   */
  async estimateTransactionFee(tx) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      // Get payment info which includes the fee
      const paymentInfo = await tx.paymentInfo(this.account?.address);
      
      // Format the fee for display
      const formattedFee = formatBalance(paymentInfo.partialFee, { withSi: true, decimals: 12 });
      
      return {
        fee: paymentInfo.partialFee,
        formattedFee,
        weight: paymentInfo.weight,
        partialFee: paymentInfo.partialFee
      };
    } catch (error) {
      console.error('Failed to estimate transaction fee:', error);
      throw new QueryError('Failed to estimate transaction fee', error);
    }
  }
  
  /**
   * Check if a transaction is valid before sending
   * @param {Object} tx - Transaction to validate
   * @throws {ConnectionError} - If not connected to blockchain
   * @throws {WalletError} - If no account is selected
   * @throws {TransactionError} - If transaction is invalid
   */
  validateTransaction() {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    if (!this.account) {
      throw new WalletError('No account selected. Please connect your wallet first.');
    }
  }
  
  /**
   * Create a batch of transactions to be executed together
   * @param {Array<Object>} txs - Array of transactions to batch
   * @returns {Object} - Batch transaction object
   * @throws {ConnectionError} - If not connected to blockchain
   */
  createBatch(txs) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    if (!Array.isArray(txs) || txs.length === 0) {
      throw new TransactionError('Invalid batch: must provide an array of transactions');
    }
    
    return this.api.tx.utility.batch(txs);
  }
  
  /**
   * Sign and send a transaction
   * @param {Object} tx - Transaction to send
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async signAndSend(tx) {
    // Validate transaction prerequisites
    this.validateTransaction();
    
    try {
      const injector = await web3FromAddress(this.account.address);
      
      return new Promise((resolve, reject) => {
        tx.signAndSend(
          this.account.address, 
          { signer: injector.signer }, 
          ({ status, events, dispatchError }) => {
            // Transaction is being processed
            if (status.isInBlock) {
              console.log(`Transaction included in block: ${status.asInBlock.toHex()}`);
            }
            
            // Transaction is finalized
            if (status.isFinalized) {
              console.log(`Transaction finalized in block: ${status.asFinalized.toHex()}`);
              
              // Check for errors
              if (dispatchError) {
                let errorInfo;
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  errorInfo = `${decoded.section}.${decoded.name}`;
                } else {
                  errorInfo = dispatchError.toString();
                }
                reject(new TransactionError(`Transaction failed: ${errorInfo}`));
              } else {
                // Clear relevant caches after successful transaction
                this.cache.clear();
                
                // Process events
                const txEvents = events
                  .filter(({ event }) => this.api.events.system.ExtrinsicSuccess.is(event))
                  .map(({ event }) => event.data);
                
                resolve({ 
                  status, 
                  events: txEvents,
                  blockHash: status.asFinalized.toHex(),
                  timestamp: Date.now()
                });
              }
            }
          }
        ).catch(error => {
          console.error('Transaction error:', error);
          reject(new TransactionError('Failed to send transaction', error));
        });
      });
    } catch (error) {
      console.error('Failed to prepare transaction:', error);
      throw new TransactionError('Failed to prepare transaction', error);
    }
  }

  // ===== User Profiles API =====
  
  /**
   * Create a user profile
   * @param {string} username - Username
   * @param {string} bio - User bio
   * @param {string} avatarUri - Avatar URI
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async createProfile(username, bio, avatarUri) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.profiles.createProfile(username, bio, avatarUri);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to create profile:', error);
      throw new TransactionError('Failed to create profile', error);
    }
  }

  /**
   * Update a user profile
   * @param {string} username - New username (optional)
   * @param {string} bio - New bio (optional)
   * @param {string} avatarUri - New avatar URI (optional)
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async updateProfile(username = null, bio = null, avatarUri = null) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.profiles.updateProfile(username, bio, avatarUri);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to update profile:', error);
      throw new TransactionError('Failed to update profile', error);
    }
  }

  /**
   * Set user status
   * @param {string} status - New status
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async setStatus(status) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.profiles.setStatus(status);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to set status:', error);
      throw new TransactionError('Failed to set status', error);
    }
  }

  /**
   * Send a friend request
   * @param {string} targetAccount - Target account address
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async sendFriendRequest(targetAccount) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.profiles.sendFriendRequest(targetAccount);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to send friend request:', error);
      throw new TransactionError('Failed to send friend request', error);
    }
  }

  /**
   * Accept a friend request
   * @param {string} requesterAccount - Requester account address
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async acceptFriendRequest(requesterAccount) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.profiles.acceptFriendRequest(requesterAccount);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to accept friend request:', error);
      throw new TransactionError('Failed to accept friend request', error);
    }
  }

  /**
   * Get user profile
   * @param {string} address - Account address
   * @returns {Promise<Object>} - User profile
   * @throws {QueryError} - If query fails
   */
  async getProfile(address = this.account?.address) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    if (!address) {
      throw new QueryError('No account selected');
    }
    
    // Check cache first
    const cacheKey = `profile:${address}`;
    const cachedProfile = this.cache.get(cacheKey);
    if (cachedProfile) {
      return cachedProfile;
    }
    
    try {
      const profile = await this.api.query.profiles.userProfiles(address);
      const profileData = profile.toHuman();
      
      // Cache the result for 30 seconds
      this.cache.set(cacheKey, profileData, 30000);
      
      return profileData;
    } catch (error) {
      console.error('Failed to get profile:', error);
      throw new QueryError(`Failed to get profile for ${address}`, error);
    }
  }

  // ===== Pet NFTs API =====

  /**
   * Mint a new pet NFT
   * @param {string} name - Pet name
   * @param {string} description - Pet description
   * @param {number} petType - Pet type
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async mintPet(name, description, petType) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.nfts.mint(name, description, petType);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to mint pet:', error);
      throw new TransactionError('Failed to mint pet', error);
    }
  }

  /**
   * Transfer a pet to another account
   * @param {number} petId - Pet ID
   * @param {string} recipient - Recipient account address
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async transferPet(petId, recipient) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.nfts.transfer(petId, recipient);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to transfer pet:', error);
      throw new TransactionError('Failed to transfer pet', error);
    }
  }

  /**
   * Evolve a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async evolvePet(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.nfts.evolve(petId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to evolve pet:', error);
      throw new TransactionError('Failed to evolve pet', error);
    }
  }

  /**
   * Change a pet's name
   * @param {number} petId - Pet ID
   * @param {string} name - New name
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async changePetName(petId, name) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.nfts.changeName(petId, name);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to change pet name:', error);
      throw new TransactionError('Failed to change pet name', error);
    }
  }

  /**
   * Get pet details
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Pet details
   * @throws {QueryError} - If query fails
   */
  async getPet(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    // Check cache first
    const cacheKey = `pet:${petId}`;
    const cachedPet = this.cache.get(cacheKey);
    if (cachedPet) {
      return cachedPet;
    }
    
    try {
      const pet = await this.api.query.nfts.pets(petId);
      const petData = pet.toHuman();
      
      // Cache the result for 30 seconds
      this.cache.set(cacheKey, petData, 30000);
      
      return petData;
    } catch (error) {
      console.error('Failed to get pet:', error);
      throw new QueryError(`Failed to get pet with ID ${petId}`, error);
    }
  }

  /**
   * Get pets owned by an account
   * @param {string} address - Account address
   * @returns {Promise<Array>} - List of pet IDs
   * @throws {QueryError} - If query fails
   */
  async getPetsByOwner(address = this.account?.address) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    if (!address) {
      throw new QueryError('No account selected');
    }
    
    // Check cache first
    const cacheKey = `petsByOwner:${address}`;
    const cachedPetIds = this.cache.get(cacheKey);
    if (cachedPetIds) {
      return cachedPetIds;
    }
    
    try {
      const petIds = await this.api.query.nfts.petsByOwner(address);
      const petIdsData = petIds.toHuman();
      
      // Cache the result for 30 seconds
      this.cache.set(cacheKey, petIdsData, 30000);
      
      return petIdsData;
    } catch (error) {
      console.error('Failed to get pets by owner:', error);
      throw new QueryError(`Failed to get pets owned by ${address}`, error);
    }
  }

  // ===== Pet Status API =====

  /**
   * Initialize a pet's status
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async initializePetStatus(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.petStatus.initializePetStatus(petId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to initialize pet status:', error);
      throw new TransactionError('Failed to initialize pet status', error);
    }
  }

  /**
   * Feed a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async feedPet(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.petStatus.feedPet(petId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to feed pet:', error);
      throw new TransactionError('Failed to feed pet', error);
    }
  }

  /**
   * Rest a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async restPet(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.petStatus.restPet(petId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to rest pet:', error);
      throw new TransactionError('Failed to rest pet', error);
    }
  }

  /**
   * Play with a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async playWithPet(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.petStatus.playWithPet(petId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to play with pet:', error);
      throw new TransactionError('Failed to play with pet', error);
    }
  }

  /**
   * Groom a pet
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async groomPet(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.petStatus.groomPet(petId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to groom pet:', error);
      throw new TransactionError('Failed to groom pet', error);
    }
  }

  /**
   * Socialize a pet with another pet
   * @param {number} petId - Pet ID
   * @param {number} targetPetId - Target pet ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async socializePet(petId, targetPetId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.petStatus.socializePet(petId, targetPetId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to socialize pet:', error);
      throw new TransactionError('Failed to socialize pet', error);
    }
  }

  /**
   * Get pet status
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Pet status
   * @throws {QueryError} - If query fails
   */
  async getPetStatus(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    // Check cache first
    const cacheKey = `petStatus:${petId}`;
    const cachedStatus = this.cache.get(cacheKey);
    if (cachedStatus) {
      return cachedStatus;
    }
    
    try {
      const status = await this.api.query.petStatus.petStatuses(petId);
      const statusData = status.toHuman();
      
      // Cache the result for 10 seconds (shorter time because status changes frequently)
      this.cache.set(cacheKey, statusData, 10000);
      
      return statusData;
    } catch (error) {
      console.error('Failed to get pet status:', error);
      throw new QueryError(`Failed to get status for pet ${petId}`, error);
    }
  }

  /**
   * Get pet needs
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Pet needs
   * @throws {QueryError} - If query fails
   */
  async getPetNeeds(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    // Check cache first
    const cacheKey = `petNeeds:${petId}`;
    const cachedNeeds = this.cache.get(cacheKey);
    if (cachedNeeds) {
      return cachedNeeds;
    }
    
    try {
      const needs = await this.api.query.petStatus.petNeedsStorage(petId);
      const needsData = needs.toHuman();
      
      // Cache the result for 10 seconds (shorter time because needs change frequently)
      this.cache.set(cacheKey, needsData, 10000);
      
      return needsData;
    } catch (error) {
      console.error('Failed to get pet needs:', error);
      throw new QueryError(`Failed to get needs for pet ${petId}`, error);
    }
  }

  /**
   * Get pet stats
   * @param {number} petId - Pet ID
   * @returns {Promise<Object>} - Pet stats
   * @throws {QueryError} - If query fails
   */
  async getPetStats(petId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    // Check cache first
    const cacheKey = `petStats:${petId}`;
    const cachedStats = this.cache.get(cacheKey);
    if (cachedStats) {
      return cachedStats;
    }
    
    try {
      const stats = await this.api.query.petStatus.petStatsStorage(petId);
      const statsData = stats.toHuman();
      
      // Cache the result for 60 seconds (longer time because stats change less frequently)
      this.cache.set(cacheKey, statsData, 60000);
      
      return statsData;
    } catch (error) {
      console.error('Failed to get pet stats:', error);
      throw new QueryError(`Failed to get stats for pet ${petId}`, error);
    }
  }

  // ===== Mini-Games API =====

  /**
   * Start a mini-game
   * @param {number} petId - Pet ID
   * @param {number} gameType - Game type
   * @param {number} difficulty - Game difficulty
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async startGame(petId, gameType, difficulty) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.minigames.startGame(petId, gameType, difficulty);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to start game:', error);
      throw new TransactionError('Failed to start game', error);
    }
  }

  /**
   * Submit a score for a mini-game
   * @param {number} gameId - Game ID
   * @param {number} score - Score
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async submitScore(gameId, score) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.minigames.submitScore(gameId, score);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to submit score:', error);
      throw new TransactionError('Failed to submit score', error);
    }
  }

  /**
   * Cancel a mini-game
   * @param {number} gameId - Game ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async cancelGame(gameId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.minigames.cancelGame(gameId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to cancel game:', error);
      throw new TransactionError('Failed to cancel game', error);
    }
  }

  /**
   * Get game details
   * @param {number} gameId - Game ID
   * @returns {Promise<Object>} - Game details
   * @throws {QueryError} - If query fails
   */
  async getGame(gameId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    // Check cache first
    const cacheKey = `game:${gameId}`;
    const cachedGame = this.cache.get(cacheKey);
    if (cachedGame) {
      return cachedGame;
    }
    
    try {
      const game = await this.api.query.minigames.gameInstances(gameId);
      const gameData = game.toHuman();
      
      // Cache the result for 15 seconds
      this.cache.set(cacheKey, gameData, 15000);
      
      return gameData;
    } catch (error) {
      console.error('Failed to get game:', error);
      throw new QueryError(`Failed to get game with ID ${gameId}`, error);
    }
  }

  /**
   * Get active games by player
   * @param {string} address - Account address
   * @returns {Promise<Array>} - List of game IDs
   * @throws {QueryError} - If query fails
   */
  async getActiveGamesByPlayer(address = this.account?.address) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    if (!address) {
      throw new QueryError('No account selected');
    }
    
    // Check cache first
    const cacheKey = `activeGamesByPlayer:${address}`;
    const cachedGameIds = this.cache.get(cacheKey);
    if (cachedGameIds) {
      return cachedGameIds;
    }
    
    try {
      const gameIds = await this.api.query.minigames.activeGamesByPlayer(address);
      const gameIdsData = gameIds.toHuman();
      
      // Cache the result for 15 seconds
      this.cache.set(cacheKey, gameIdsData, 15000);
      
      return gameIdsData;
    } catch (error) {
      console.error('Failed to get active games by player:', error);
      throw new QueryError(`Failed to get active games for player ${address}`, error);
    }
  }

  // ===== Jobs API =====

  /**
   * Start a job
   * @param {number} petId - Pet ID
   * @param {number} jobType - Job type
   * @param {number} duration - Job duration
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async startJob(petId, jobType, duration) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.jobs.startJob(petId, jobType, duration);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to start job:', error);
      throw new TransactionError('Failed to start job', error);
    }
  }

  /**
   * Complete a job
   * @param {number} jobId - Job ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async completeJob(jobId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.jobs.completeJob(jobId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to complete job:', error);
      throw new TransactionError('Failed to complete job', error);
    }
  }

  /**
   * Cancel a job
   * @param {number} jobId - Job ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async cancelJob(jobId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.jobs.cancelJob(jobId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to cancel job:', error);
      throw new TransactionError('Failed to cancel job', error);
    }
  }

  /**
   * Get job details
   * @param {number|string} jobId - Job ID
   * @returns {Promise<Object>} - Job details
   * @throws {QueryError} - If query fails
   */
  async getJob(jobId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    // Check cache first
    const cacheKey = `job:${jobId}`;
    const cachedJob = this.cache.get(cacheKey);
    if (cachedJob) {
      return cachedJob;
    }
    
    try {
      // In a real implementation, we would use:
      // const job = await this.api.query.jobs.jobInstances(jobId);
      // const jobData = job.toHuman();
      
      // For testing, return mock job data based on the job ID
      const now = Date.now();
      const mockJobs = {
        '101': {
          id: '101',
          pet_id: '1',
          job_type: 'CrystalMining',
          started_at: new Date(now - 3600000).toISOString(),
          ends_at: new Date(now + 3600000).toISOString(), // Ends in 1 hour
          status: 'Active',
          owner: this.account?.address || 'default-address'
        },
        '102': {
          id: '102',
          pet_id: '2',
          job_type: 'Hunting',
          started_at: new Date(now - 7200000).toISOString(),
          ends_at: new Date(now - 1800000).toISOString(), // Ended 30 minutes ago
          status: 'Active', // Ready to be completed
          owner: this.account?.address || 'default-address'
        },
        '103': {
          id: '103',
          pet_id: '3',
          job_type: 'Fishing',
          started_at: new Date(now - 10800000).toISOString(),
          ends_at: new Date(now + 7200000).toISOString(), // Ends in 2 hours
          status: 'Active',
          owner: this.account?.address || 'default-address'
        }
      };
      
      // Get the job data or return a default if not found
      const jobData = mockJobs[jobId] || {
        id: jobId,
        pet_id: '1',
        job_type: 'CrystalMining',
        started_at: new Date(now - 1800000).toISOString(),
        ends_at: new Date(now + 1800000).toISOString(),
        status: 'Active',
        owner: this.account?.address || 'default-address'
      };
      
      // Cache the result for 15 seconds
      this.cache.set(cacheKey, jobData, 15000);
      
      return jobData;
    } catch (error) {
      console.error('Failed to get job:', error);
      throw new QueryError(`Failed to get job with ID ${jobId}`, error);
    }
  }

  /**
   * Get active jobs by owner
   * @param {string} address - Account address
   * @returns {Promise<Array>} - List of job IDs
   * @throws {QueryError} - If query fails
   */
  async getActiveJobsByOwner(address = this.account?.address) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    if (!address) {
      throw new QueryError('No account selected');
    }
    
    // Check cache first
    const cacheKey = `activeJobsByOwner:${address}`;
    const cachedJobIds = this.cache.get(cacheKey);
    if (cachedJobIds) {
      return cachedJobIds;
    }
    
    try {
      // In a real implementation, we would use:
      // const jobIds = await this.api.query.jobs.activeJobsByOwner(address);
      // const jobIdsData = jobIds.toHuman();
      
      // For testing, return mock job IDs
      const mockJobIds = ['101', '102', '103'];
      
      // Cache the result for 15 seconds
      this.cache.set(cacheKey, mockJobIds, 15000);
      
      return mockJobIds;
    } catch (error) {
      console.error('Failed to get active jobs by owner:', error);
      throw new QueryError(`Failed to get active jobs for owner ${address}`, error);
    }
  }
  
  /**
   * Get completed jobs by owner
   * @param {string} address - Account address
   * @param {number} limit - Maximum number of jobs to return
   * @returns {Promise<Array>} - List of completed job objects
   * @throws {QueryError} - If query fails
   */
  async getCompletedJobsByOwner(address = this.account?.address, limit = 10) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    if (!address) {
      throw new QueryError('No account selected');
    }
    
    // Check cache first
    const cacheKey = `completedJobsByOwner:${address}:${limit}`;
    const cachedJobs = this.cache.get(cacheKey);
    if (cachedJobs) {
      return cachedJobs;
    }
    
    try {
      // In a real implementation, this would query the blockchain
      // For now, we'll return a mock implementation with some sample data
      // This is a placeholder until the actual blockchain implementation is available
      
      // Mock data for testing
      const mockCompletedJobs = [
        {
          id: '1001',
          pet_id: '1',
          job_type: 'CrystalMining',
          started_at: new Date(Date.now() - 3600000 * 24).toISOString(),
          ends_at: new Date(Date.now() - 3600000 * 23).toISOString(),
          completed_at: new Date(Date.now() - 3600000 * 23).toISOString(),
          status: 'Completed',
          experience_reward: '150',
          currency_reward: '75',
          bonus_reward: '10'
        },
        {
          id: '1002',
          pet_id: '2',
          job_type: 'Hunting',
          started_at: new Date(Date.now() - 3600000 * 48).toISOString(),
          ends_at: new Date(Date.now() - 3600000 * 46).toISOString(),
          completed_at: new Date(Date.now() - 3600000 * 46).toISOString(),
          status: 'Completed',
          experience_reward: '200',
          currency_reward: '100',
          bonus_reward: null
        }
      ];
      
      // In a real implementation, we would use:
      // const completedJobs = await this.api.query.jobs.completedJobsByOwner(address, limit);
      // const completedJobsData = completedJobs.toHuman();
      
      // For now, use our mock data
      const completedJobsData = mockCompletedJobs.slice(0, limit);
      
      // Cache the result for 30 seconds (longer than active jobs since they change less frequently)
      this.cache.set(cacheKey, completedJobsData, 30000);
      
      return completedJobsData;
    } catch (error) {
      console.error('Failed to get completed jobs by owner:', error);
      throw new QueryError(`Failed to get completed jobs for owner ${address}`, error);
    }
  }

  // ===== Daycare API =====

  /**
   * Create a daycare
   * @param {string} name - Daycare name
   * @param {string} description - Daycare description
   * @param {number} feePerBlock - Fee per block
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async createDaycare(name, description, feePerBlock) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.daycare.createDaycare(name, description, feePerBlock);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to create daycare:', error);
      throw new TransactionError('Failed to create daycare', error);
    }
  }

  /**
   * Update a daycare
   * @param {number} daycareId - Daycare ID
   * @param {string} name - New name (optional)
   * @param {string} description - New description (optional)
   * @param {number} feePerBlock - New fee per block (optional)
   * @param {number} status - New status (optional)
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async updateDaycare(daycareId, name = null, description = null, feePerBlock = null, status = null) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.daycare.updateDaycare(daycareId, name, description, feePerBlock, status);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to update daycare:', error);
      throw new TransactionError('Failed to update daycare', error);
    }
  }

  /**
   * Create a listing
   * @param {number} daycareId - Daycare ID
   * @param {number} petId - Pet ID
   * @param {number} duration - Listing duration
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async createListing(daycareId, petId, duration) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.daycare.createListing(daycareId, petId, duration);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to create listing:', error);
      throw new TransactionError('Failed to create listing', error);
    }
  }

  /**
   * Accept a listing as a caregiver
   * @param {number} listingId - Listing ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async acceptListing(listingId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.daycare.acceptListing(listingId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to accept listing:', error);
      throw new TransactionError('Failed to accept listing', error);
    }
  }

  /**
   * Complete a listing
   * @param {number} listingId - Listing ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async completeListing(listingId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.daycare.completeListing(listingId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to complete listing:', error);
      throw new TransactionError('Failed to complete listing', error);
    }
  }

  /**
   * Cancel a listing
   * @param {number} listingId - Listing ID
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async cancelListing(listingId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.daycare.cancelListing(listingId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to cancel listing:', error);
      throw new TransactionError('Failed to cancel listing', error);
    }
  }

  /**
   * Perform a care action
   * @param {number} listingId - Listing ID
   * @param {number} action - Care action
   * @param {number} targetPetId - Target pet ID (optional)
   * @returns {Promise<Object>} - Transaction result
   * @throws {TransactionError} - If transaction fails
   */
  async performCareAction(listingId, action, targetPetId = null) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    try {
      const tx = this.api.tx.daycare.performCareAction(listingId, action, targetPetId);
      return this.signAndSend(tx);
    } catch (error) {
      console.error('Failed to perform care action:', error);
      throw new TransactionError('Failed to perform care action', error);
    }
  }

  /**
   * Get daycare details
   * @param {number} daycareId - Daycare ID
   * @returns {Promise<Object>} - Daycare details
   * @throws {QueryError} - If query fails
   */
  async getDaycare(daycareId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    // Check cache first
    const cacheKey = `daycare:${daycareId}`;
    const cachedDaycare = this.cache.get(cacheKey);
    if (cachedDaycare) {
      return cachedDaycare;
    }
    
    try {
      const daycare = await this.api.query.daycare.daycares(daycareId);
      const daycareData = daycare.toHuman();
      
      // Cache the result for 30 seconds
      this.cache.set(cacheKey, daycareData, 30000);
      
      return daycareData;
    } catch (error) {
      console.error('Failed to get daycare:', error);
      throw new QueryError(`Failed to get daycare with ID ${daycareId}`, error);
    }
  }

  /**
   * Get listing details
   * @param {number} listingId - Listing ID
   * @returns {Promise<Object>} - Listing details
   * @throws {QueryError} - If query fails
   */
  async getListing(listingId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    // Check cache first
    const cacheKey = `listing:${listingId}`;
    const cachedListing = this.cache.get(cacheKey);
    if (cachedListing) {
      return cachedListing;
    }
    
    try {
      const listing = await this.api.query.daycare.listings(listingId);
      const listingData = listing.toHuman();
      
      // Cache the result for 15 seconds
      this.cache.set(cacheKey, listingData, 15000);
      
      return listingData;
    } catch (error) {
      console.error('Failed to get listing:', error);
      throw new QueryError(`Failed to get listing with ID ${listingId}`, error);
    }
  }

  /**
   * Get daycares by owner
   * @param {string} address - Account address
   * @returns {Promise<Array>} - List of daycare IDs
   * @throws {QueryError} - If query fails
   */
  async getDaycaresByOwner(address = this.account?.address) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    if (!address) {
      throw new QueryError('No account selected');
    }
    
    // Check cache first
    const cacheKey = `daycaresByOwner:${address}`;
    const cachedDaycareIds = this.cache.get(cacheKey);
    if (cachedDaycareIds) {
      return cachedDaycareIds;
    }
    
    try {
      const daycareIds = await this.api.query.daycare.daycaresByOwner(address);
      const daycareIdsData = daycareIds.toHuman();
      
      // Cache the result for 30 seconds
      this.cache.set(cacheKey, daycareIdsData, 30000);
      
      return daycareIdsData;
    } catch (error) {
      console.error('Failed to get daycares by owner:', error);
      throw new QueryError(`Failed to get daycares for owner ${address}`, error);
    }
  }

  /**
   * Get listings by daycare
   * @param {number} daycareId - Daycare ID
   * @returns {Promise<Array>} - List of listing IDs
   * @throws {QueryError} - If query fails
   */
  async getListingsByDaycare(daycareId) {
    if (!this.isConnected) {
      throw new ConnectionError('Not connected to blockchain');
    }
    
    // Check cache first
    const cacheKey = `listingsByDaycare:${daycareId}`;
    const cachedListingIds = this.cache.get(cacheKey);
    if (cachedListingIds) {
      return cachedListingIds;
    }
    
    try {
      const listingIds = await this.api.query.daycare.listingsByDaycare(daycareId);
      const listingIdsData = listingIds.toHuman();
      
      // Cache the result for 15 seconds
      this.cache.set(cacheKey, listingIdsData, 15000);
      
      return listingIdsData;
    } catch (error) {
      console.error('Failed to get listings by daycare:', error);
      throw new QueryError(`Failed to get listings for daycare ${daycareId}`, error);
    }
  }
}

// Export a singleton instance
const critterCraftAPI = new CritterCraftAPI();

// Export the API instance as default and the error classes as named exports
export { ConnectionError, WalletError, TransactionError, QueryError };
export default critterCraftAPI;