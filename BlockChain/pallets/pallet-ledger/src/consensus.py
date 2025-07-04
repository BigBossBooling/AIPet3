"""
Consensus module for the Zoologist's Ledger.

This module implements the Proof of Reputation & Stake (PoRS) consensus
mechanism, which combines investment with merit.
"""

import random
import time
from typing import Dict, List, Optional, Tuple

from .models import Block, Transaction, ZoologistIdentity


class ProofOfReputationStake:
    """
    Implementation of the Proof of Reputation & Stake consensus mechanism.
    
    This consensus mechanism combines traditional Proof of Stake with a
    reputation multiplier, ensuring that while invested players have influence,
    they cannot easily dominate the network.
    """
    
    def __init__(self):
        """Initialize the consensus mechanism."""
        self.validators = {}  # did -> (stake_amount, reputation_score)
        self.total_stake = 0
    
    def register_validator(self, did: str, stake_amount: int, reputation_score: int) -> None:
        """
        Register a new validator or update an existing one.
        
        Args:
            did: The DID of the validator.
            stake_amount: The amount of AURA staked by the validator.
            reputation_score: The validator's reputation score.
        """
        self.validators[did] = (stake_amount, reputation_score)
        self.total_stake += stake_amount
    
    def unregister_validator(self, did: str) -> None:
        """
        Unregister a validator.
        
        Args:
            did: The DID of the validator to unregister.
        """
        if did in self.validators:
            stake_amount, _ = self.validators[did]
            self.total_stake -= stake_amount
            del self.validators[did]
    
    def calculate_validator_weight(self, did: str) -> float:
        """
        Calculate a validator's weight in the consensus.
        
        The weight is calculated as:
        (Staked Amount) * (1 + Reputation Score / 100)
        
        Args:
            did: The DID of the validator.
            
        Returns:
            The validator's weight.
        """
        if did not in self.validators:
            return 0
        
        stake_amount, reputation_score = self.validators[did]
        return stake_amount * (1 + reputation_score / 100)
    
    def select_validator(self) -> Optional[str]:
        """
        Select a validator to create the next block.
        
        The selection is weighted by each validator's stake and reputation.
        
        Returns:
            The DID of the selected validator, or None if there are no validators.
        """
        if not self.validators:
            return None
        
        # Calculate weights for all validators
        weights = {did: self.calculate_validator_weight(did) for did in self.validators}
        total_weight = sum(weights.values())
        
        # Select a validator based on their weight
        r = random.uniform(0, total_weight)
        cumulative_weight = 0
        
        for did, weight in weights.items():
            cumulative_weight += weight
            if r <= cumulative_weight:
                return did
        
        # Fallback to a random validator
        return random.choice(list(self.validators.keys()))
    
    def validate_block(self, block: Block, validator_public_key: str) -> bool:
        """
        Validate a block created by a validator.
        
        Args:
            block: The block to validate.
            validator_public_key: The public key of the validator who created the block.
            
        Returns:
            True if the block is valid, False otherwise.
        """
        # Verify the block signature
        if not block.verify(validator_public_key):
            return False
        
        # Verify that the validator is registered
        if block.validator_did not in self.validators:
            return False
        
        # In a real implementation, we would also verify:
        # - The block's previous hash matches the hash of the previous block
        # - All transactions in the block are valid
        # - The block's timestamp is reasonable
        # - The block's number is correct
        
        return True
    
    def create_block(self, validator_did: str, previous_hash: str, transactions: List[Transaction], private_key: str) -> Block:
        """
        Create a new block.
        
        Args:
            validator_did: The DID of the validator creating the block.
            previous_hash: The hash of the previous block.
            transactions: The list of transactions to include in the block.
            private_key: The private key of the validator.
            
        Returns:
            A new, signed block.
        """
        # Create a new block
        block = Block(
            block_number=0,  # This would be set correctly in a real implementation
            timestamp=int(time.time()),
            previous_hash=previous_hash,
            transactions=transactions,
            validator_did=validator_did
        )
        
        # Sign the block
        block.sign(private_key)
        
        return block