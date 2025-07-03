"""
Integration module for the Critter-Craft battle system and the Zoologist's Ledger.

This module provides functions for integrating the battle system with the
blockchain, allowing significant battle events to be recorded on-chain.
"""

import sys
import os
from typing import Dict, Optional

# Add the battle system and ledger to the Python path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'pallet-battles', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'pallet-ledger', 'src'))

# Import the battle system
from battle import start_battle

# Import the blockchain system
from ledger import ZoologistLedger
from wallet import Wallet
from models import TransactionType


def record_battle_victory(
    ledger: ZoologistLedger,
    winner_wallet: Wallet,
    loser_wallet: Wallet,
    winner_pet: Dict,
    loser_pet: Dict,
    battle_result: Dict
) -> bool:
    """
    Record a significant battle victory on the Zoologist's Ledger.
    
    Args:
        ledger: The Zoologist's Ledger instance.
        winner_wallet: The wallet of the winning player.
        loser_wallet: The wallet of the losing player.
        winner_pet: The winning pet's data.
        loser_pet: The losing pet's data.
        battle_result: The result of the battle.
        
    Returns:
        True if the victory was recorded successfully, False otherwise.
    """
    # Only record significant victories (e.g., against Alpha critters)
    if not loser_pet.get("is_alpha", False):
        return False
    
    # Update the winner's reputation
    reputation_tx = winner_wallet.create_reputation_update_transaction(
        target_did=winner_wallet.address,
        change_amount=10,  # Significant reputation boost for defeating an Alpha
        reason_code="ALPHA_VICTORY"
    )
    
    # Submit the transaction
    if not ledger.submit_transaction(reputation_tx):
        return False
    
    # If the winner's pet has an NFT ID, record its evolution
    if "nft_id" in winner_pet:
        evolve_tx = winner_wallet.create_pet_evolve_transaction(
            pet_id=winner_pet["nft_id"],
            new_form_id=f"evolved_{winner_pet['species'].lower()}_1"
        )
        
        # Submit the transaction
        if not ledger.submit_transaction(evolve_tx):
            return False
    
    return True


def mint_legendary_item(
    ledger: ZoologistLedger,
    crafter_wallet: Wallet,
    item_type: str,
    rarity: str,
    crafting_materials: list
) -> Optional[str]:
    """
    Mint a legendary item as an NFT on the Zoologist's Ledger.
    
    Args:
        ledger: The Zoologist's Ledger instance.
        crafter_wallet: The wallet of the crafter.
        item_type: The type of item.
        rarity: The rarity of the item.
        crafting_materials: The materials used to craft the item.
        
    Returns:
        The ID of the minted item, or None if minting failed.
    """
    # Ensure the crafter is a Master Zoologist
    crafter_did = crafter_wallet.address
    crafter = ledger.get_zoologist(crafter_did)
    
    if not crafter or crafter.level.name != "MASTER":
        return None
    
    # Create a metadata URI for the item
    metadata_uri = f"https://api.crittercraft.com/gear/{item_type.lower().replace(' ', '_')}_{hash(crafter_did)}"
    
    # Create a transaction to mint the item
    gear_tx = crafter_wallet.create_gear_mint_transaction(
        item_type=item_type,
        rarity=rarity,
        metadata_uri=metadata_uri,
        crafting_materials=crafting_materials
    )
    
    # Submit the transaction
    if not ledger.submit_transaction(gear_tx):
        return None
    
    # Get the minted item
    crafter_gear = ledger.get_gear_by_owner(crafter_did)
    
    if not crafter_gear:
        return None
    
    # Return the ID of the most recently minted item
    return crafter_gear[-1].token_id


def battle_with_blockchain(
    ledger: ZoologistLedger,
    player_wallet: Wallet,
    opponent_wallet: Wallet,
    player_pet: Dict,
    opponent_pet: Dict,
    environment_type: str,
    player_items: list
) -> Dict:
    """
    Run a battle and record significant events on the blockchain.
    
    Args:
        ledger: The Zoologist's Ledger instance.
        player_wallet: The wallet of the player.
        opponent_wallet: The wallet of the opponent.
        player_pet: The player's pet data.
        opponent_pet: The opponent's pet data.
        environment_type: The type of environment for the battle.
        player_items: The items the player has available.
        
    Returns:
        The result of the battle.
    """
    # Start the battle
    battle_result = start_battle(player_pet, opponent_pet, environment_type, player_items)
    
    # Record significant events on the blockchain
    if battle_result["winner"] == "player":
        record_battle_victory(
            ledger=ledger,
            winner_wallet=player_wallet,
            loser_wallet=opponent_wallet,
            winner_pet=player_pet,
            loser_pet=opponent_pet,
            battle_result=battle_result
        )
        
        # Create a block to confirm the transactions
        if player_wallet.address in ledger.consensus.validators:
            ledger.create_block(player_wallet)
    
    return battle_result


if __name__ == "__main__":
    print("This module is not meant to be run directly.")
    print("Import it and use its functions to integrate the battle system with the blockchain.")