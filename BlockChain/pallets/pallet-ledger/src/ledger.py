"""
Main module for the Zoologist's Ledger blockchain.

This module provides the ZoologistLedger class, which is the main entry point
for interacting with the blockchain.
"""

import time
import uuid
from typing import Dict, List, Optional, Set, Tuple, Union

from .models import (
    Block, 
    GearNFT, 
    PetNFT, 
    Proposal, 
    ProposalStatus, 
    ProposalType, 
    Transaction, 
    TransactionType, 
    Vote, 
    VoteDirection, 
    ZoologistIdentity, 
    ZoologistLevel
)
from .consensus import ProofOfReputationStake
from .wallet import Wallet


class ZoologistLedger:
    """
    The main class for interacting with the Zoologist's Ledger blockchain.
    
    This class provides methods for submitting transactions, querying the
    blockchain state, and participating in governance.
    """
    
    def __init__(self):
        """Initialize the Zoologist's Ledger."""
        # Initialize blockchain state
        self.blocks = []
        self.pending_transactions = []
        self.consensus = ProofOfReputationStake()
        
        # Initialize data stores
        self.pets = {}  # token_id -> PetNFT
        self.gear = {}  # token_id -> GearNFT
        self.zoologists = {}  # did -> ZoologistIdentity
        self.proposals = {}  # proposal_id -> Proposal
        self.votes = {}  # vote_id -> Vote
        self.balances = {}  # did -> AURA balance
        self.stakes = {}  # did -> staked AURA amount
        
        # Create genesis block
        self._create_genesis_block()
    
    def _create_genesis_block(self) -> None:
        """Create the genesis block of the blockchain."""
        genesis_block = Block(
            block_number=0,
            timestamp=int(time.time()),
            previous_hash="0" * 64,
            transactions=[],
            validator_did="did:zoologist:genesis",
            validator_signature="genesis_signature"
        )
        
        self.blocks.append(genesis_block)
    
    def submit_transaction(self, transaction: Transaction) -> bool:
        """
        Submit a transaction to the blockchain.
        
        Args:
            transaction: The transaction to submit.
            
        Returns:
            True if the transaction was accepted, False otherwise.
        """
        # Verify the transaction signature
        if not transaction.verify(transaction.sender_did):
            return False
        
        # Process the transaction based on its type
        if transaction.tx_type == TransactionType.PET_MINT:
            return self._process_pet_mint(transaction)
        elif transaction.tx_type == TransactionType.PET_EVOLVE:
            return self._process_pet_evolve(transaction)
        elif transaction.tx_type == TransactionType.GEAR_MINT:
            return self._process_gear_mint(transaction)
        elif transaction.tx_type == TransactionType.ASSET_TRANSFER:
            return self._process_asset_transfer(transaction)
        elif transaction.tx_type == TransactionType.VOTE_CAST:
            return self._process_vote_cast(transaction)
        elif transaction.tx_type == TransactionType.REPUTATION_UPDATE:
            return self._process_reputation_update(transaction)
        elif transaction.tx_type == TransactionType.AURA_TRANSFER:
            return self._process_aura_transfer(transaction)
        elif transaction.tx_type == TransactionType.STAKE:
            return self._process_stake(transaction)
        elif transaction.tx_type == TransactionType.UNSTAKE:
            return self._process_unstake(transaction)
        else:
            return False
    
    def _process_pet_mint(self, transaction: Transaction) -> bool:
        """
        Process a PET_MINT transaction.
        
        Args:
            transaction: The transaction to process.
            
        Returns:
            True if the transaction was processed successfully, False otherwise.
        """
        # Extract transaction data
        sender_did = transaction.sender_did
        payload = transaction.payload
        
        # Ensure the sender exists
        if sender_did not in self.zoologists:
            self.zoologists[sender_did] = ZoologistIdentity(did=sender_did)
        
        # Create a new pet NFT
        token_id = str(uuid.uuid4())
        pet = PetNFT(
            token_id=token_id,
            genesis_timestamp=transaction.timestamp,
            minter_did=sender_did,
            species=payload["species"],
            aura_color=payload["aura_color"],
            genetic_hash=payload["genetic_hash"],
            metadata_uri=payload["metadata_uri"],
            current_owner_did=sender_did
        )
        
        # Add the pet to the ledger
        self.pets[token_id] = pet
        
        # Add the transaction to pending transactions
        self.pending_transactions.append(transaction)
        
        return True
    
    def _process_pet_evolve(self, transaction: Transaction) -> bool:
        """
        Process a PET_EVOLVE transaction.
        
        Args:
            transaction: The transaction to process.
            
        Returns:
            True if the transaction was processed successfully, False otherwise.
        """
        # Extract transaction data
        sender_did = transaction.sender_did
        payload = transaction.payload
        pet_id = payload["pet_id"]
        new_form_id = payload["new_form_id"]
        
        # Ensure the pet exists and is owned by the sender
        if pet_id not in self.pets or self.pets[pet_id].current_owner_did != sender_did:
            return False
        
        # Update the pet's evolution history
        self.pets[pet_id].evolve(new_form_id, transaction.timestamp)
        
        # Update the zoologist's evolved pets count
        if sender_did in self.zoologists:
            self.zoologists[sender_did].evolved_pets_count += 1
        
        # Add the transaction to pending transactions
        self.pending_transactions.append(transaction)
        
        return True
    
    def _process_gear_mint(self, transaction: Transaction) -> bool:
        """
        Process a GEAR_MINT transaction.
        
        Args:
            transaction: The transaction to process.
            
        Returns:
            True if the transaction was processed successfully, False otherwise.
        """
        # Extract transaction data
        sender_did = transaction.sender_did
        payload = transaction.payload
        
        # Ensure the sender is a Master Zoologist
        if sender_did not in self.zoologists or self.zoologists[sender_did].level != ZoologistLevel.MASTER:
            return False
        
        # Create a new gear NFT
        token_id = str(uuid.uuid4())
        gear = GearNFT(
            token_id=token_id,
            genesis_timestamp=transaction.timestamp,
            crafter_did=sender_did,
            item_type=payload["item_type"],
            rarity=payload["rarity"],
            metadata_uri=payload["metadata_uri"],
            current_owner_did=sender_did,
            crafting_materials=payload["crafting_materials"]
        )
        
        # Add the gear to the ledger
        self.gear[token_id] = gear
        
        # Add the transaction to pending transactions
        self.pending_transactions.append(transaction)
        
        return True
    
    def _process_asset_transfer(self, transaction: Transaction) -> bool:
        """
        Process an ASSET_TRANSFER transaction.
        
        Args:
            transaction: The transaction to process.
            
        Returns:
            True if the transaction was processed successfully, False otherwise.
        """
        # Extract transaction data
        sender_did = transaction.sender_did
        payload = transaction.payload
        asset_id = payload["asset_id"]
        recipient_did = payload["recipient_did"]
        
        # Ensure the recipient exists
        if recipient_did not in self.zoologists:
            self.zoologists[recipient_did] = ZoologistIdentity(did=recipient_did)
        
        # Check if the asset is a pet
        if asset_id in self.pets:
            # Ensure the pet is owned by the sender
            if self.pets[asset_id].current_owner_did != sender_did:
                return False
            
            # Transfer the pet
            self.pets[asset_id].transfer(recipient_did)
        
        # Check if the asset is gear
        elif asset_id in self.gear:
            # Ensure the gear is owned by the sender
            if self.gear[asset_id].current_owner_did != sender_did:
                return False
            
            # Transfer the gear
            self.gear[asset_id].transfer(recipient_did)
        
        else:
            # Asset not found
            return False
        
        # Add the transaction to pending transactions
        self.pending_transactions.append(transaction)
        
        return True
    
    def _process_vote_cast(self, transaction: Transaction) -> bool:
        """
        Process a VOTE_CAST transaction.
        
        Args:
            transaction: The transaction to process.
            
        Returns:
            True if the transaction was processed successfully, False otherwise.
        """
        # Extract transaction data
        sender_did = transaction.sender_did
        payload = transaction.payload
        proposal_id = payload["proposal_id"]
        direction = payload["direction"]
        voting_power = payload["voting_power"]
        
        # Ensure the proposal exists and is active
        if proposal_id not in self.proposals or not self.proposals[proposal_id].is_active:
            return False
        
        # Ensure the sender has enough voting power
        if sender_did not in self.zoologists or self.zoologists[sender_did].voting_power < voting_power:
            return False
        
        # Create a new vote
        vote_id = str(uuid.uuid4())
        vote = Vote(
            vote_id=vote_id,
            proposal_id=proposal_id,
            voter_did=sender_did,
            direction=VoteDirection[direction],
            voting_power=voting_power,
            timestamp=transaction.timestamp
        )
        
        # Add the vote to the ledger
        self.votes[vote_id] = vote
        
        # Update the proposal's vote counts
        proposal = self.proposals[proposal_id]
        if vote.direction == VoteDirection.FOR:
            proposal.votes_for += voting_power
        elif vote.direction == VoteDirection.AGAINST:
            proposal.votes_against += voting_power
        elif vote.direction == VoteDirection.ABSTAIN:
            proposal.votes_abstain += voting_power
        
        # Add the transaction to pending transactions
        self.pending_transactions.append(transaction)
        
        return True
    
    def _process_reputation_update(self, transaction: Transaction) -> bool:
        """
        Process a REPUTATION_UPDATE transaction.
        
        Args:
            transaction: The transaction to process.
            
        Returns:
            True if the transaction was processed successfully, False otherwise.
        """
        # Extract transaction data
        sender_did = transaction.sender_did
        payload = transaction.payload
        target_did = payload["target_did"]
        change_amount = payload["change_amount"]
        
        # Ensure the target exists
        if target_did not in self.zoologists:
            self.zoologists[target_did] = ZoologistIdentity(did=target_did)
        
        # Update the target's reputation
        self.zoologists[target_did].reputation_score += change_amount
        
        # If the sender is a validator, update their weight
        if sender_did in self.consensus.validators:
            stake_amount, _ = self.consensus.validators[sender_did]
            self.consensus.validators[sender_did] = (stake_amount, self.zoologists[sender_did].reputation_score)
        
        # Add the transaction to pending transactions
        self.pending_transactions.append(transaction)
        
        return True
    
    def _process_aura_transfer(self, transaction: Transaction) -> bool:
        """
        Process an AURA_TRANSFER transaction.
        
        Args:
            transaction: The transaction to process.
            
        Returns:
            True if the transaction was processed successfully, False otherwise.
        """
        # Extract transaction data
        sender_did = transaction.sender_did
        payload = transaction.payload
        recipient_did = payload["recipient_did"]
        amount = payload["amount"]
        
        # Ensure the sender has enough AURA
        if sender_did not in self.balances or self.balances[sender_did] < amount:
            return False
        
        # Ensure the recipient exists
        if recipient_did not in self.zoologists:
            self.zoologists[recipient_did] = ZoologistIdentity(did=recipient_did)
        
        # Initialize balances if needed
        if sender_did not in self.balances:
            self.balances[sender_did] = 0
        if recipient_did not in self.balances:
            self.balances[recipient_did] = 0
        
        # Transfer the AURA
        self.balances[sender_did] -= amount
        self.balances[recipient_did] += amount
        
        # Add the transaction to pending transactions
        self.pending_transactions.append(transaction)
        
        return True
    
    def _process_stake(self, transaction: Transaction) -> bool:
        """
        Process a STAKE transaction.
        
        Args:
            transaction: The transaction to process.
            
        Returns:
            True if the transaction was processed successfully, False otherwise.
        """
        # Extract transaction data
        sender_did = transaction.sender_did
        payload = transaction.payload
        amount = payload["amount"]
        
        # Ensure the sender has enough AURA
        if sender_did not in self.balances or self.balances[sender_did] < amount:
            return False
        
        # Initialize stakes if needed
        if sender_did not in self.stakes:
            self.stakes[sender_did] = 0
        
        # Stake the AURA
        self.balances[sender_did] -= amount
        self.stakes[sender_did] += amount
        
        # Register the validator or update their stake
        if sender_did in self.zoologists:
            reputation_score = self.zoologists[sender_did].reputation_score
        else:
            reputation_score = 0
            self.zoologists[sender_did] = ZoologistIdentity(did=sender_did)
        
        self.consensus.register_validator(sender_did, self.stakes[sender_did], reputation_score)
        
        # Add the transaction to pending transactions
        self.pending_transactions.append(transaction)
        
        return True
    
    def _process_unstake(self, transaction: Transaction) -> bool:
        """
        Process an UNSTAKE transaction.
        
        Args:
            transaction: The transaction to process.
            
        Returns:
            True if the transaction was processed successfully, False otherwise.
        """
        # Extract transaction data
        sender_did = transaction.sender_did
        payload = transaction.payload
        amount = payload["amount"]
        
        # Ensure the sender has enough staked AURA
        if sender_did not in self.stakes or self.stakes[sender_did] < amount:
            return False
        
        # Initialize balances if needed
        if sender_did not in self.balances:
            self.balances[sender_did] = 0
        
        # Unstake the AURA
        self.stakes[sender_did] -= amount
        self.balances[sender_did] += amount
        
        # Update the validator's stake or unregister them
        if self.stakes[sender_did] > 0:
            if sender_did in self.zoologists:
                reputation_score = self.zoologists[sender_did].reputation_score
            else:
                reputation_score = 0
            
            self.consensus.register_validator(sender_did, self.stakes[sender_did], reputation_score)
        else:
            self.consensus.unregister_validator(sender_did)
        
        # Add the transaction to pending transactions
        self.pending_transactions.append(transaction)
        
        return True
    
    def create_block(self, validator_wallet: Wallet) -> Optional[Block]:
        """
        Create a new block with pending transactions.
        
        Args:
            validator_wallet: The wallet of the validator creating the block.
            
        Returns:
            The created block, or None if the validator is not selected.
        """
        # Ensure there are pending transactions
        if not self.pending_transactions:
            return None
        
        # Select a validator
        selected_validator = self.consensus.select_validator()
        
        # Ensure the provided wallet belongs to the selected validator
        if selected_validator != validator_wallet.address:
            return None
        
        # Get the previous block's hash
        previous_hash = self.blocks[-1].hash if self.blocks else "0" * 64
        
        # Create a new block
        block = self.consensus.create_block(
            validator_did=validator_wallet.address,
            previous_hash=previous_hash,
            transactions=self.pending_transactions,
            private_key=validator_wallet.private_key
        )
        
        # Set the block number
        block.block_number = len(self.blocks)
        
        # Add the block to the blockchain
        self.blocks.append(block)
        
        # Clear pending transactions
        self.pending_transactions = []
        
        return block
    
    def get_pet(self, pet_id: str) -> Optional[PetNFT]:
        """
        Get a pet by its ID.
        
        Args:
            pet_id: The ID of the pet to get.
            
        Returns:
            The pet, or None if not found.
        """
        return self.pets.get(pet_id)
    
    def get_gear(self, gear_id: str) -> Optional[GearNFT]:
        """
        Get gear by its ID.
        
        Args:
            gear_id: The ID of the gear to get.
            
        Returns:
            The gear, or None if not found.
        """
        return self.gear.get(gear_id)
    
    def get_zoologist(self, did: str) -> Optional[ZoologistIdentity]:
        """
        Get a zoologist by their DID.
        
        Args:
            did: The DID of the zoologist to get.
            
        Returns:
            The zoologist, or None if not found.
        """
        return self.zoologists.get(did)
    
    def get_proposal(self, proposal_id: str) -> Optional[Proposal]:
        """
        Get a proposal by its ID.
        
        Args:
            proposal_id: The ID of the proposal to get.
            
        Returns:
            The proposal, or None if not found.
        """
        return self.proposals.get(proposal_id)
    
    def get_balance(self, did: str) -> int:
        """
        Get a zoologist's AURA balance.
        
        Args:
            did: The DID of the zoologist.
            
        Returns:
            The zoologist's AURA balance.
        """
        return self.balances.get(did, 0)
    
    def get_stake(self, did: str) -> int:
        """
        Get a zoologist's staked AURA amount.
        
        Args:
            did: The DID of the zoologist.
            
        Returns:
            The zoologist's staked AURA amount.
        """
        return self.stakes.get(did, 0)
    
    def get_pets_by_owner(self, owner_did: str) -> List[PetNFT]:
        """
        Get all pets owned by a zoologist.
        
        Args:
            owner_did: The DID of the zoologist.
            
        Returns:
            A list of pets owned by the zoologist.
        """
        return [pet for pet in self.pets.values() if pet.current_owner_did == owner_did]
    
    def get_gear_by_owner(self, owner_did: str) -> List[GearNFT]:
        """
        Get all gear owned by a zoologist.
        
        Args:
            owner_did: The DID of the zoologist.
            
        Returns:
            A list of gear owned by the zoologist.
        """
        return [gear for gear in self.gear.values() if gear.current_owner_did == owner_did]
    
    def create_proposal(self, proposer_wallet: Wallet, proposal_type: ProposalType, title: str, description: str, implementation_details: Dict) -> str:
        """
        Create a new governance proposal.
        
        Args:
            proposer_wallet: The wallet of the proposer.
            proposal_type: The type of proposal.
            title: The title of the proposal.
            description: The description of the proposal.
            implementation_details: The technical details of the proposal.
            
        Returns:
            The ID of the created proposal.
        """
        # Create a new proposal
        proposal_id = str(uuid.uuid4())
        proposal = Proposal(
            proposal_id=proposal_id,
            proposer_did=proposer_wallet.address,
            proposal_type=proposal_type,
            title=title,
            description=description,
            implementation_details=implementation_details,
            status=ProposalStatus.ACTIVE
        )
        
        # Add the proposal to the ledger
        self.proposals[proposal_id] = proposal
        
        return proposal_id
    
    def vote_on_proposal(self, voter_wallet: Wallet, proposal_id: str, direction: VoteDirection) -> bool:
        """
        Vote on a governance proposal.
        
        Args:
            voter_wallet: The wallet of the voter.
            proposal_id: The ID of the proposal to vote on.
            direction: The direction of the vote.
            
        Returns:
            True if the vote was cast successfully, False otherwise.
        """
        # Ensure the proposal exists and is active
        if proposal_id not in self.proposals or not self.proposals[proposal_id].is_active:
            return False
        
        # Ensure the voter exists
        voter_did = voter_wallet.address
        if voter_did not in self.zoologists:
            return False
        
        # Calculate the voter's voting power
        voting_power = self.zoologists[voter_did].voting_power
        
        # Create a vote transaction
        tx = voter_wallet.create_vote_transaction(
            proposal_id=proposal_id,
            direction=direction.name,
            voting_power=voting_power
        )
        
        # Submit the transaction
        return self.submit_transaction(tx)
    
    def finalize_proposals(self) -> List[str]:
        """
        Finalize proposals whose voting period has ended.
        
        Returns:
            A list of IDs of finalized proposals.
        """
        finalized_proposals = []
        current_time = int(time.time())
        
        for proposal_id, proposal in self.proposals.items():
            if proposal.status == ProposalStatus.ACTIVE and current_time >= proposal.voting_ends_timestamp:
                proposal.finalize()
                finalized_proposals.append(proposal_id)
        
        return finalized_proposals
    
    def implement_proposal(self, proposal_id: str) -> bool:
        """
        Implement a passed proposal.
        
        Args:
            proposal_id: The ID of the proposal to implement.
            
        Returns:
            True if the proposal was implemented successfully, False otherwise.
        """
        # Ensure the proposal exists and has passed
        if proposal_id not in self.proposals or self.proposals[proposal_id].status != ProposalStatus.PASSED:
            return False
        
        # Mark the proposal as implemented
        self.proposals[proposal_id].status = ProposalStatus.IMPLEMENTED
        
        # In a real implementation, this would actually implement the proposal
        # based on its implementation_details
        
        return True