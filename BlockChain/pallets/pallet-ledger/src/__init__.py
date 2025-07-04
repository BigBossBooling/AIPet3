"""
Zoologist's Ledger - Blockchain Integration for Critter-Craft

This package provides the blockchain integration for the Critter-Craft game,
implementing the "Zoologist's Ledger" concept for managing provenance,
permanence, and player agency.
"""

from .ledger import ZoologistLedger
from .models import (
    PetNFT, 
    GearNFT, 
    ZoologistIdentity, 
    Transaction, 
    Block, 
    ProposalType, 
    Proposal, 
    Vote
)
from .consensus import ProofOfReputationStake
from .wallet import Wallet

__all__ = [
    'ZoologistLedger',
    'PetNFT',
    'GearNFT',
    'ZoologistIdentity',
    'Transaction',
    'Block',
    'ProofOfReputationStake',
    'ProposalType',
    'Proposal',
    'Vote',
    'Wallet',
]