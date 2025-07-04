"""
Models for the Zoologist's Ledger blockchain.

This module defines the data models for the blockchain, including NFTs,
transactions, blocks, and governance structures.
"""

import time
import uuid
import hashlib
import json
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Union, Any


class TransactionType(Enum):
    """Types of transactions that can be recorded on the Zoologist's Ledger."""
    PET_MINT = auto()
    PET_EVOLVE = auto()
    GEAR_MINT = auto()
    ASSET_TRANSFER = auto()
    VOTE_CAST = auto()
    REPUTATION_UPDATE = auto()
    AURA_TRANSFER = auto()
    STAKE = auto()
    UNSTAKE = auto()


class ProposalType(Enum):
    """Types of proposals that can be submitted to the Zoologist's Guild."""
    NEW_CONTENT = auto()  # New critter adaptation, species, or environment
    GAME_BALANCE = auto()  # Balance changes to battle items or abilities
    ECOSYSTEM_DEVELOPMENT = auto()  # Community projects funded by treasury


class ProposalStatus(Enum):
    """Status of a governance proposal."""
    PENDING = auto()
    ACTIVE = auto()
    PASSED = auto()
    REJECTED = auto()
    IMPLEMENTED = auto()


class VoteDirection(Enum):
    """Direction of a vote on a proposal."""
    FOR = auto()
    AGAINST = auto()
    ABSTAIN = auto()


class ZoologistLevel(Enum):
    """Levels of Zoologist expertise."""
    NOVICE = auto()
    APPRENTICE = auto()
    JOURNEYMAN = auto()
    EXPERT = auto()
    MASTER = auto()


@dataclass
class ZoologistIdentity:
    """
    Represents a player's decentralized identity on the Zoologist's Ledger.
    
    This is the on-chain representation of a player's account, including their
    reputation, level, and voting power.
    """
    did: str  # Decentralized Identifier (public key)
    level: ZoologistLevel = ZoologistLevel.NOVICE
    reputation_score: int = 0
    evolved_pets_count: int = 0
    join_timestamp: int = field(default_factory=lambda: int(time.time()))
    
    @property
    def voting_power(self) -> int:
        """Calculate the zoologist's voting power based on their achievements."""
        level_multiplier = {
            ZoologistLevel.NOVICE: 1,
            ZoologistLevel.APPRENTICE: 2,
            ZoologistLevel.JOURNEYMAN: 5,
            ZoologistLevel.EXPERT: 8,
            ZoologistLevel.MASTER: 10
        }
        
        return (level_multiplier[self.level] * 10) + (self.evolved_pets_count * 5) + self.reputation_score


@dataclass
class PetNFT:
    """
    Represents a Pet NFT on the Zoologist's Ledger.
    
    This is the on-chain representation of a pet's immutable genetic and
    spiritual blueprint, not its dynamic state.
    """
    token_id: str
    genesis_timestamp: int
    minter_did: str
    species: str
    aura_color: str
    genetic_hash: str
    metadata_uri: str  # Points to off-chain data about the pet's current state
    current_owner_did: str = field(default="")
    evolution_history: List[Dict] = field(default_factory=list)
    
    def __post_init__(self):
        if not self.current_owner_did:
            self.current_owner_did = self.minter_did
    
    def evolve(self, new_form_id: str, timestamp: int) -> None:
        """Record an evolution event in the pet's history."""
        self.evolution_history.append({
            "timestamp": timestamp,
            "new_form": new_form_id
        })
    
    def transfer(self, new_owner_did: str) -> None:
        """Transfer ownership of the pet to a new owner."""
        self.current_owner_did = new_owner_did


@dataclass
class GearNFT:
    """
    Represents a Legendary Crafted Gear NFT on the Zoologist's Ledger.
    
    Only items crafted by Master Zoologists can be minted as NFTs.
    """
    token_id: str
    genesis_timestamp: int
    crafter_did: str
    item_type: str
    rarity: str
    metadata_uri: str  # Points to off-chain data about the item's properties
    current_owner_did: str = field(default="")
    crafting_materials: List[str] = field(default_factory=list)
    
    def __post_init__(self):
        if not self.current_owner_did:
            self.current_owner_did = self.crafter_did
    
    def transfer(self, new_owner_did: str) -> None:
        """Transfer ownership of the gear to a new owner."""
        self.current_owner_did = new_owner_did


@dataclass
class Transaction:
    """
    Represents a transaction on the Zoologist's Ledger.
    
    Transactions are the atomic units of state change in the blockchain.
    """
    tx_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    tx_type: TransactionType = None
    timestamp: int = field(default_factory=lambda: int(time.time()))
    sender_did: str = ""
    payload: Dict = field(default_factory=dict)
    signature: str = ""
    
    def sign(self, private_key: str) -> None:
        """
        Sign the transaction with the sender's private key.
        
        In a real implementation, this would use proper cryptographic signing.
        For this prototype, we'll use a simple hash-based approach.
        """
        # Create a string representation of the transaction data
        tx_data = f"{self.tx_id}{self.tx_type}{self.timestamp}{self.sender_did}{json.dumps(self.payload, sort_keys=True)}"
        
        # In a real implementation, this would be:
        # self.signature = sign_message(tx_data, private_key)
        # For this prototype, we'll use a simple hash
        self.signature = hashlib.sha256((tx_data + private_key).encode()).hexdigest()
    
    def verify(self, public_key: str) -> bool:
        """
        Verify the transaction signature using the sender's public key.
        
        In a real implementation, this would use proper cryptographic verification.
        For this prototype, we'll use a simple hash-based approach.
        """
        # Create a string representation of the transaction data
        tx_data = f"{self.tx_id}{self.tx_type}{self.timestamp}{self.sender_did}{json.dumps(self.payload, sort_keys=True)}"
        
        # In a real implementation, this would be:
        # return verify_signature(tx_data, self.signature, public_key)
        # For this prototype, we'll use a simple hash
        expected_signature = hashlib.sha256((tx_data + public_key).encode()).hexdigest()
        return self.signature == expected_signature


@dataclass
class Block:
    """
    Represents a block in the Zoologist's Ledger blockchain.
    
    Blocks contain a list of transactions and are linked together by their hashes.
    """
    block_number: int
    timestamp: int = field(default_factory=lambda: int(time.time()))
    previous_hash: str = ""
    transactions: List[Transaction] = field(default_factory=list)
    validator_did: str = ""
    validator_signature: str = ""
    nonce: int = 0
    
    @property
    def merkle_root(self) -> str:
        """Calculate the Merkle root of the transactions in this block."""
        if not self.transactions:
            return hashlib.sha256("empty".encode()).hexdigest()
        
        # In a real implementation, this would build a proper Merkle tree
        # For this prototype, we'll just hash all transaction IDs together
        tx_ids = [tx.tx_id for tx in self.transactions]
        return hashlib.sha256("".join(tx_ids).encode()).hexdigest()
    
    @property
    def hash(self) -> str:
        """Calculate the hash of this block."""
        block_data = f"{self.block_number}{self.timestamp}{self.previous_hash}{self.merkle_root}{self.validator_did}{self.nonce}"
        return hashlib.sha256(block_data.encode()).hexdigest()
    
    def sign(self, private_key: str) -> None:
        """
        Sign the block with the validator's private key.
        
        In a real implementation, this would use proper cryptographic signing.
        For this prototype, we'll use a simple hash-based approach.
        """
        # Create a string representation of the block data
        block_data = f"{self.block_number}{self.timestamp}{self.previous_hash}{self.merkle_root}{self.validator_did}{self.nonce}"
        
        # In a real implementation, this would be:
        # self.validator_signature = sign_message(block_data, private_key)
        # For this prototype, we'll use a simple hash
        self.validator_signature = hashlib.sha256((block_data + private_key).encode()).hexdigest()
    
    def verify(self, public_key: str) -> bool:
        """
        Verify the block signature using the validator's public key.
        
        In a real implementation, this would use proper cryptographic verification.
        For this prototype, we'll use a simple hash-based approach.
        """
        # Create a string representation of the block data
        block_data = f"{self.block_number}{self.timestamp}{self.previous_hash}{self.merkle_root}{self.validator_did}{self.nonce}"
        
        # In a real implementation, this would be:
        # return verify_signature(block_data, self.validator_signature, public_key)
        # For this prototype, we'll use a simple hash
        expected_signature = hashlib.sha256((block_data + public_key).encode()).hexdigest()
        return self.validator_signature == expected_signature


@dataclass
class Proposal:
    """
    Represents a governance proposal in the Zoologist's Guild.
    
    Proposals are submitted by players and voted on by the community.
    """
    proposal_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    proposer_did: str = ""
    proposal_type: ProposalType = None
    title: str = ""
    description: str = ""
    implementation_details: Dict = field(default_factory=dict)
    creation_timestamp: int = field(default_factory=lambda: int(time.time()))
    voting_ends_timestamp: int = field(default_factory=lambda: int(time.time()) + 7 * 24 * 60 * 60)  # 1 week voting period
    status: ProposalStatus = ProposalStatus.PENDING
    votes_for: int = 0
    votes_against: int = 0
    votes_abstain: int = 0
    
    @property
    def is_active(self) -> bool:
        """Check if the proposal is currently active for voting."""
        current_time = int(time.time())
        return (
            self.status == ProposalStatus.ACTIVE and
            current_time < self.voting_ends_timestamp
        )
    
    def finalize(self) -> None:
        """Finalize the proposal after the voting period ends."""
        if self.votes_for > self.votes_against:
            self.status = ProposalStatus.PASSED
        else:
            self.status = ProposalStatus.REJECTED


@dataclass
class Vote:
    """
    Represents a vote on a governance proposal.
    
    Votes are cast by players and weighted by their voting power.
    """
    vote_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    proposal_id: str = ""
    voter_did: str = ""
    direction: VoteDirection = None
    voting_power: int = 0
    timestamp: int = field(default_factory=lambda: int(time.time()))
    signature: str = ""
    
    def sign(self, private_key: str) -> None:
        """
        Sign the vote with the voter's private key.
        
        In a real implementation, this would use proper cryptographic signing.
        For this prototype, we'll use a simple hash-based approach.
        """
        # Create a string representation of the vote data
        vote_data = f"{self.vote_id}{self.proposal_id}{self.voter_did}{self.direction}{self.voting_power}{self.timestamp}"
        
        # In a real implementation, this would be:
        # self.signature = sign_message(vote_data, private_key)
        # For this prototype, we'll use a simple hash
        self.signature = hashlib.sha256((vote_data + private_key).encode()).hexdigest()
    
    def verify(self, public_key: str) -> bool:
        """
        Verify the vote signature using the voter's public key.
        
        In a real implementation, this would use proper cryptographic verification.
        For this prototype, we'll use a simple hash-based approach.
        """
        # Create a string representation of the vote data
        vote_data = f"{self.vote_id}{self.proposal_id}{self.voter_did}{self.direction}{self.voting_power}{self.timestamp}"
        
        # In a real implementation, this would be:
        # return verify_signature(vote_data, self.signature, public_key)
        # For this prototype, we'll use a simple hash
        expected_signature = hashlib.sha256((vote_data + public_key).encode()).hexdigest()
        return self.signature == expected_signature