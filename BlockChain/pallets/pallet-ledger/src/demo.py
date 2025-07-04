"""
Demo script for the Zoologist's Ledger blockchain integration with the battle system.

This script demonstrates how the battle system can be integrated with the
blockchain to record significant events and achievements.
"""

import sys
import os
import time
import random
from typing import Dict, List, Optional

# Add the parent directory to the Python path to import the battle system
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-battles', 'src'))

# Import the battle system
from battle import start_battle

# Import the blockchain system
from .ledger import ZoologistLedger
from .wallet import Wallet
from .models import TransactionType, ProposalType, VoteDirection


def run_demo():
    """Run a demo of the Zoologist's Ledger integration with the battle system."""
    print("Welcome to the Zoologist's Ledger Demo!")
    print("=" * 60)
    time.sleep(1)
    
    # Initialize the blockchain
    ledger = ZoologistLedger()
    
    # Create wallets for players
    player_wallet = Wallet()
    opponent_wallet = Wallet()
    
    print(f"Player DID: {player_wallet.address}")
    print(f"Opponent DID: {opponent_wallet.address}")
    print()
    
    # Initialize player balances (in a real game, this would be earned through gameplay)
    ledger.balances[player_wallet.address] = 1000
    ledger.balances[opponent_wallet.address] = 1000
    
    # Create zoologist identities
    ledger.zoologists[player_wallet.address] = ledger.zoologists.get(
        player_wallet.address, 
        ZoologistIdentity(did=player_wallet.address)
    )
    ledger.zoologists[opponent_wallet.address] = ledger.zoologists.get(
        opponent_wallet.address, 
        ZoologistIdentity(did=opponent_wallet.address)
    )
    
    # Mint pets for both players
    print("Minting pets for players...")
    
    # Player's pet
    player_pet_tx = player_wallet.create_pet_mint_transaction(
        species="Chameleon",
        aura_color="Gold",
        genetic_hash="chameleon_gold_123",
        metadata_uri="https://api.crittercraft.com/pets/chameleon_gold_123"
    )
    ledger.submit_transaction(player_pet_tx)
    
    # Opponent's pet
    opponent_pet_tx = opponent_wallet.create_pet_mint_transaction(
        species="Anglerfish",
        aura_color="Blue",
        genetic_hash="anglerfish_blue_456",
        metadata_uri="https://api.crittercraft.com/pets/anglerfish_blue_456"
    )
    ledger.submit_transaction(opponent_pet_tx)
    
    # Create a block to confirm the transactions
    validator_wallet = player_wallet  # For demo purposes, the player is also a validator
    ledger.consensus.register_validator(
        validator_wallet.address, 
        100,  # Stake amount
        ledger.zoologists[validator_wallet.address].reputation_score
    )
    block = ledger.create_block(validator_wallet)
    
    if block:
        print(f"Block created: #{block.block_number}, hash: {block.hash[:8]}...")
    
    # Get the minted pets
    player_pets = ledger.get_pets_by_owner(player_wallet.address)
    opponent_pets = ledger.get_pets_by_owner(opponent_wallet.address)
    
    if player_pets and opponent_pets:
        player_pet_nft = player_pets[0]
        opponent_pet_nft = opponent_pets[0]
        
        print(f"Player's pet: {player_pet_nft.species} with {player_pet_nft.aura_color} aura")
        print(f"Opponent's pet: {opponent_pet_nft.species} with {opponent_pet_nft.aura_color} aura")
        print()
        
        # Prepare pets for battle (in a real game, this would be more sophisticated)
        player_pet = {
            "name": "Sparkles",
            "species": player_pet_nft.species,
            "level": 3,
            "adaptations": [
                "basic_maneuver",
                "camouflage",
                "defend",
                "echolocation"
            ],
            "is_alpha": False,
            "nft_id": player_pet_nft.token_id
        }
        
        opponent_pet = {
            "name": "Glimmer",
            "species": opponent_pet_nft.species,
            "level": 2,
            "adaptations": [
                "basic_maneuver",
                "bioluminescence",
                "defend",
                "venom_strike"
            ],
            "is_alpha": True,  # This is an Alpha critter, so victory is significant
            "nft_id": opponent_pet_nft.token_id
        }
        
        # Available items for the player
        player_items = [
            "healing_salve",
            "adrenaline_berry",
            "focus_root"
        ]
        
        print("Starting battle...")
        print()
        
        # Start a battle
        try:
            # In a real game, this would be an interactive battle
            # For the demo, we'll simulate the result
            battle_result = {
                "winner": "player",
                "turns_taken": 5,
                "rewards": {
                    "experience": 20,
                    "research_points": 10,
                    "items": ["healing_salve"],
                    "friendship": 2
                }
            }
            
            print(f"Battle result: {'Player' if battle_result['winner'] == 'player' else 'Opponent'} wins in {battle_result['turns_taken']} turns!")
            
            # Record significant battle outcomes on the blockchain
            if battle_result["winner"] == "player" and opponent_pet["is_alpha"]:
                print("\nRecording victory over Alpha critter on the Zoologist's Ledger...")
                
                # Update player's reputation
                reputation_tx = player_wallet.create_reputation_update_transaction(
                    target_did=player_wallet.address,
                    change_amount=10,  # Significant reputation boost for defeating an Alpha
                    reason_code="ALPHA_VICTORY"
                )
                ledger.submit_transaction(reputation_tx)
                
                # Record the pet's evolution (in a real game, this might be a separate action)
                evolve_tx = player_wallet.create_pet_evolve_transaction(
                    pet_id=player_pet["nft_id"],
                    new_form_id="evolved_chameleon_1"
                )
                ledger.submit_transaction(evolve_tx)
                
                # Create a block to confirm the transactions
                block = ledger.create_block(validator_wallet)
                
                if block:
                    print(f"Block created: #{block.block_number}, hash: {block.hash[:8]}...")
                
                # Check the updated reputation
                player_zoologist = ledger.get_zoologist(player_wallet.address)
                print(f"Player's new reputation score: {player_zoologist.reputation_score}")
                print(f"Player's evolved pets count: {player_zoologist.evolved_pets_count}")
                
                # Mint a legendary item as a reward
                if player_zoologist.reputation_score >= 10:
                    print("\nMinting a legendary item as a reward...")
                    
                    gear_tx = player_wallet.create_gear_mint_transaction(
                        item_type="Amplifying Crystal",
                        rarity="Rare",
                        metadata_uri="https://api.crittercraft.com/gear/amplifying_crystal_789",
                        crafting_materials=["quartz", "geode", "luminous_essence"]
                    )
                    ledger.submit_transaction(gear_tx)
                    
                    # Create a block to confirm the transaction
                    block = ledger.create_block(validator_wallet)
                    
                    if block:
                        print(f"Block created: #{block.block_number}, hash: {block.hash[:8]}...")
                    
                    # Check the minted gear
                    player_gear = ledger.get_gear_by_owner(player_wallet.address)
                    if player_gear:
                        print(f"Player received: {player_gear[0].item_type} ({player_gear[0].rarity})")
            
            # Demonstrate governance
            print("\nDemonstrating governance...")
            
            # Create a proposal to add a new adaptation
            proposal_id = ledger.create_proposal(
                proposer_wallet=player_wallet,
                proposal_type=ProposalType.NEW_CONTENT,
                title="Add Sonic Blast Adaptation",
                description="A new adaptation that allows critters to emit a powerful sonic blast, stunning opponents.",
                implementation_details={
                    "adaptation_name": "sonic_blast",
                    "display_name": "Sonic Blast",
                    "description": "Emit a powerful sonic blast that may stun the opponent for one turn.",
                    "ap_cost": 3,
                    "base_success_chance": 0.7
                }
            )
            
            print(f"Created proposal: {proposal_id}")
            
            # Vote on the proposal
            ledger.vote_on_proposal(
                voter_wallet=player_wallet,
                proposal_id=proposal_id,
                direction=VoteDirection.FOR
            )
            
            ledger.vote_on_proposal(
                voter_wallet=opponent_wallet,
                proposal_id=proposal_id,
                direction=VoteDirection.AGAINST
            )
            
            # Force-finalize the proposal for demo purposes
            proposal = ledger.get_proposal(proposal_id)
            proposal.voting_ends_timestamp = int(time.time()) - 1
            finalized = ledger.finalize_proposals()
            
            if proposal_id in finalized:
                proposal = ledger.get_proposal(proposal_id)
                print(f"Proposal finalized with status: {proposal.status.name}")
                
                if proposal.status.name == "PASSED":
                    ledger.implement_proposal(proposal_id)
                    print("Proposal implemented!")
        
        except Exception as e:
            print(f"Error during battle: {e}")
    
    print("\nThank you for trying the Zoologist's Ledger Demo!")


if __name__ == "__main__":
    run_demo()