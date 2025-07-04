"""
Wallet module for the Zoologist's Ledger.

This module provides functionality for managing cryptographic keys and
creating and signing transactions.
"""

import os
import hashlib
import time
import uuid
from typing import Dict, List, Optional, Tuple

from .models import Transaction, TransactionType


class Wallet:
    """
    A wallet for interacting with the Zoologist's Ledger.
    
    The wallet manages a player's cryptographic keys and provides methods
    for creating and signing transactions.
    """
    
    def __init__(self, private_key: Optional[str] = None):
        """
        Initialize a wallet with an optional private key.
        
        If no private key is provided, a new one will be generated.
        
        Args:
            private_key: Optional private key to use for this wallet.
        """
        if private_key:
            self.private_key = private_key
        else:
            # In a real implementation, this would use proper cryptographic key generation
            # For this prototype, we'll use a simple random string
            self.private_key = hashlib.sha256(os.urandom(32)).hexdigest()
        
        # Derive the public key from the private key
        # In a real implementation, this would use proper cryptographic derivation
        # For this prototype, we'll use a simple hash
        self.public_key = hashlib.sha256(self.private_key.encode()).hexdigest()
    
    @property
    def address(self) -> str:
        """Get the wallet's address (DID) for use in transactions."""
        # In a real implementation, this would be derived from the public key
        # For this prototype, we'll use a simple hash
        return "did:zoologist:" + self.public_key[:16]
    
    def create_transaction(self, tx_type: TransactionType, payload: Dict) -> Transaction:
        """
        Create a new transaction.
        
        Args:
            tx_type: The type of transaction to create.
            payload: The payload data for the transaction.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        tx = Transaction(
            tx_id=str(uuid.uuid4()),
            tx_type=tx_type,
            timestamp=int(time.time()),
            sender_did=self.address,
            payload=payload
        )
        
        # Sign the transaction
        tx.sign(self.private_key)
        
        return tx
    
    def create_pet_mint_transaction(self, species: str, aura_color: str, genetic_hash: str, metadata_uri: str) -> Transaction:
        """
        Create a transaction to mint a new pet NFT.
        
        Args:
            species: The species (archetype) of the pet.
            aura_color: The aura color of the pet.
            genetic_hash: The unique genetic hash of the pet.
            metadata_uri: The URI pointing to the pet's off-chain metadata.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        payload = {
            "species": species,
            "aura_color": aura_color,
            "genetic_hash": genetic_hash,
            "metadata_uri": metadata_uri
        }
        
        return self.create_transaction(TransactionType.PET_MINT, payload)
    
    def create_pet_evolve_transaction(self, pet_id: str, new_form_id: str) -> Transaction:
        """
        Create a transaction to evolve a pet.
        
        Args:
            pet_id: The ID of the pet to evolve.
            new_form_id: The ID of the new form.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        payload = {
            "pet_id": pet_id,
            "new_form_id": new_form_id
        }
        
        return self.create_transaction(TransactionType.PET_EVOLVE, payload)
    
    def create_gear_mint_transaction(self, item_type: str, rarity: str, metadata_uri: str, crafting_materials: List[str]) -> Transaction:
        """
        Create a transaction to mint a new gear NFT.
        
        Args:
            item_type: The type of item.
            rarity: The rarity of the item.
            metadata_uri: The URI pointing to the item's off-chain metadata.
            crafting_materials: The materials used to craft the item.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        payload = {
            "item_type": item_type,
            "rarity": rarity,
            "metadata_uri": metadata_uri,
            "crafting_materials": crafting_materials
        }
        
        return self.create_transaction(TransactionType.GEAR_MINT, payload)
    
    def create_asset_transfer_transaction(self, asset_id: str, recipient_did: str) -> Transaction:
        """
        Create a transaction to transfer an asset to another player.
        
        Args:
            asset_id: The ID of the asset to transfer.
            recipient_did: The DID of the recipient.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        payload = {
            "asset_id": asset_id,
            "recipient_did": recipient_did
        }
        
        return self.create_transaction(TransactionType.ASSET_TRANSFER, payload)
    
    def create_vote_transaction(self, proposal_id: str, direction: str, voting_power: int) -> Transaction:
        """
        Create a transaction to cast a vote on a proposal.
        
        Args:
            proposal_id: The ID of the proposal to vote on.
            direction: The direction of the vote (FOR, AGAINST, ABSTAIN).
            voting_power: The voting power to use for this vote.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        payload = {
            "proposal_id": proposal_id,
            "direction": direction,
            "voting_power": voting_power
        }
        
        return self.create_transaction(TransactionType.VOTE_CAST, payload)
    
    def create_reputation_update_transaction(self, target_did: str, change_amount: int, reason_code: str) -> Transaction:
        """
        Create a transaction to update a player's reputation.
        
        Args:
            target_did: The DID of the player whose reputation is being updated.
            change_amount: The amount to change the reputation by (positive or negative).
            reason_code: The reason code for the reputation change.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        payload = {
            "target_did": target_did,
            "change_amount": change_amount,
            "reason_code": reason_code
        }
        
        return self.create_transaction(TransactionType.REPUTATION_UPDATE, payload)
    
    def create_aura_transfer_transaction(self, recipient_did: str, amount: int) -> Transaction:
        """
        Create a transaction to transfer AURA tokens to another player.
        
        Args:
            recipient_did: The DID of the recipient.
            amount: The amount of AURA to transfer.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        payload = {
            "recipient_did": recipient_did,
            "amount": amount
        }
        
        return self.create_transaction(TransactionType.AURA_TRANSFER, payload)
    
    def create_stake_transaction(self, amount: int) -> Transaction:
        """
        Create a transaction to stake AURA tokens.
        
        Args:
            amount: The amount of AURA to stake.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        payload = {
            "amount": amount
        }
        
        return self.create_transaction(TransactionType.STAKE, payload)
    
    def create_unstake_transaction(self, amount: int) -> Transaction:
        """
        Create a transaction to unstake AURA tokens.
        
        Args:
            amount: The amount of AURA to unstake.
            
        Returns:
            A signed transaction ready to be submitted to the ledger.
        """
        payload = {
            "amount": amount
        }
        
        return self.create_transaction(TransactionType.UNSTAKE, payload)