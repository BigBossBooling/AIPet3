"""
Integration module for the Echo-Synthesis breeding system with the battle system
and the Zoologist's Ledger.

This module provides functions for integrating the breeding system with the
battle system and the blockchain.
"""

import sys
import os
import time
import random
from typing import Dict, List, Optional, Tuple

# Add the necessary directories to the Python path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'pallet-battles', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'pallet-ledger', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'pallet-breeding', 'src'))

# Import the battle system
from battle import start_battle

# Import the blockchain system
from ledger import ZoologistLedger
from wallet import Wallet
from models import TransactionType, ZoologistIdentity, ZoologistLevel

# Import the breeding system
from genetics import (
    GeneticCode, 
    CoreGenes, 
    PotentialGenes, 
    CosmeticGenes, 
    AuraType,
    Stat
)
from synthesis import (
    EchoSynthesizer, 
    SynthesisType, 
    SynthesisState
)
from catalysts import (
    StableCatalyst, 
    UnstableCatalyst
)
from lineage import (
    FamilyTree, 
    calculate_inbreeding_coefficient
)


def convert_pet_to_battle_format(pet_genetic_code: GeneticCode, pet_name: str, pet_level: int) -> Dict:
    """
    Convert a pet's genetic code to the format expected by the battle system.
    
    Args:
        pet_genetic_code: The genetic code of the pet.
        pet_name: The name of the pet.
        pet_level: The level of the pet.
        
    Returns:
        The pet in battle system format.
    """
    # Map species to adaptations
    species_adaptations = {
        "sprite_chameleon": ["basic_maneuver", "camouflage", "defend", "echolocation"],
        "sprite_anglerfish": ["basic_maneuver", "bioluminescence", "defend", "venom_strike"],
        "sprite_glow": ["basic_maneuver", "illuminate", "defend", "energy_pulse"],
        "sprite_shadow": ["basic_maneuver", "shadow_cloak", "defend", "fear_inducement"],
        "sprite_crystal": ["basic_maneuver", "crystal_armor", "defend", "refract"],
        "sprite_terra": ["basic_maneuver", "burrow", "defend", "stone_skin"],
        "sprite_ember": ["basic_maneuver", "heat_wave", "defend", "ignite"],
        "sprite_aqua": ["basic_maneuver", "water_jet", "defend", "bubble_shield"],
        # Add adaptations for hybrid species
        "sprite_luminous": ["basic_maneuver", "illuminate", "water_jet", "energy_pulse", "bubble_shield"],
        "sprite_obsidian": ["basic_maneuver", "shadow_cloak", "crystal_armor", "fear_inducement", "refract"],
        "sprite_magma": ["basic_maneuver", "heat_wave", "stone_skin", "ignite", "burrow"]
    }
    
    # Get adaptations for the pet's species
    adaptations = species_adaptations.get(
        pet_genetic_code.core.species, 
        ["basic_maneuver", "defend"]  # Default adaptations
    )
    
    # Limit adaptations to the pet's adaptation slots
    adaptations = adaptations[:pet_genetic_code.potential.adaptation_slots]
    
    # Create the pet in battle system format
    battle_pet = {
        "name": pet_name,
        "species": pet_genetic_code.core.species,
        "level": pet_level,
        "adaptations": adaptations,
        "is_alpha": False,
        "nft_id": pet_genetic_code.core.genesis_id
    }
    
    return battle_pet


def convert_blockchain_pet_to_genetic_code(pet_nft, metadata: Dict) -> GeneticCode:
    """
    Convert a pet NFT from the blockchain to a genetic code.
    
    Args:
        pet_nft: The pet NFT from the blockchain.
        metadata: The off-chain metadata for the pet.
        
    Returns:
        The pet's genetic code.
    """
    # Create core genes
    core = CoreGenes(
        species=pet_nft.species,
        aura=AuraType[pet_nft.aura_color],
        genesis_id=pet_nft.token_id,
        lineage=pet_nft.evolution_history
    )
    
    # Create potential genes
    potential = PotentialGenes(
        stat_potential=metadata.get("stat_potential", {}),
        adaptation_slots=metadata.get("adaptation_slots", 3)
    )
    
    # Create cosmetic genes
    cosmetic = CosmeticGenes(
        size=metadata.get("size", "STANDARD"),
        pattern=metadata.get("pattern", "SOLID"),
        marking_color=metadata.get("marking_color", "#FFFFFF"),
        glow_intensity=metadata.get("glow_intensity", 0.0)
    )
    
    return GeneticCode(
        core=core,
        potential=potential,
        cosmetic=cosmetic
    )


def perform_breeding(
    ledger: ZoologistLedger,
    player_wallet: Wallet,
    parent_a_id: str,
    parent_b_id: str,
    synthesis_type: SynthesisType,
    catalyst_quality: int,
    gene_splicers: List = None
) -> Optional[str]:
    """
    Perform breeding between two pets and record the result on the blockchain.
    
    Args:
        ledger: The Zoologist's Ledger instance.
        player_wallet: The wallet of the player performing the breeding.
        parent_a_id: The ID of the first parent pet.
        parent_b_id: The ID of the second parent pet.
        synthesis_type: The type of synthesis to perform.
        catalyst_quality: The quality of the catalyst to use.
        gene_splicers: List of gene splicers to use.
        
    Returns:
        The ID of the offspring pet, or None if breeding failed.
    """
    # Get the parent pets from the blockchain
    parent_a_nft = ledger.get_pet(parent_a_id)
    parent_b_nft = ledger.get_pet(parent_b_id)
    
    if not parent_a_nft or not parent_b_nft:
        print("One or both parent pets not found.")
        return None
    
    # In a real implementation, we would fetch the off-chain metadata
    # For this prototype, we'll generate random metadata
    parent_a_metadata = {
        "stat_potential": {stat.name: random.randint(50, 80) for stat in Stat},
        "adaptation_slots": random.randint(3, 5),
        "size": random.choice(["TINY", "SMALL", "STANDARD", "LARGE", "HUGE"]),
        "pattern": random.choice(["SOLID", "SPOTTED", "STRIPED", "MOTTLED", "IRIDESCENT"]),
        "marking_color": f"#{random.randint(0, 0xFFFFFF):06x}",
        "glow_intensity": random.uniform(0.0, 1.0)
    }
    
    parent_b_metadata = {
        "stat_potential": {stat.name: random.randint(50, 80) for stat in Stat},
        "adaptation_slots": random.randint(3, 5),
        "size": random.choice(["TINY", "SMALL", "STANDARD", "LARGE", "HUGE"]),
        "pattern": random.choice(["SOLID", "SPOTTED", "STRIPED", "MOTTLED", "IRIDESCENT"]),
        "marking_color": f"#{random.randint(0, 0xFFFFFF):06x}",
        "glow_intensity": random.uniform(0.0, 1.0)
    }
    
    # Convert the pets to genetic codes
    parent_a = convert_blockchain_pet_to_genetic_code(parent_a_nft, parent_a_metadata)
    parent_b = convert_blockchain_pet_to_genetic_code(parent_b_nft, parent_b_metadata)
    
    # Create a family tree
    family_tree = FamilyTree()
    family_tree.add_pet(parent_a)
    family_tree.add_pet(parent_b)
    
    # Check for inbreeding
    inbreeding_coefficient = calculate_inbreeding_coefficient(family_tree, parent_a, parent_b)
    
    if inbreeding_coefficient > 0.25:
        print(f"Warning: High inbreeding coefficient ({inbreeding_coefficient:.2f}).")
        print("This breeding has a high risk of negative mutations.")
    
    # Create the appropriate catalyst
    if synthesis_type == SynthesisType.INTRA_SPECIES:
        catalyst = StableCatalyst(quality=catalyst_quality)
    else:
        catalyst = UnstableCatalyst(quality=catalyst_quality)
    
    # Get the player's zoologist level
    zoologist = ledger.get_zoologist(player_wallet.address)
    zoologist_level = 1
    
    if zoologist:
        zoologist_level = {
            ZoologistLevel.NOVICE: 1,
            ZoologistLevel.APPRENTICE: 2,
            ZoologistLevel.JOURNEYMAN: 3,
            ZoologistLevel.EXPERT: 4,
            ZoologistLevel.MASTER: 5
        }[zoologist.level]
    
    # Create the Echo-Synthesizer
    synthesizer = EchoSynthesizer()
    
    # Set parent happiness (in a real game, this would be the actual happiness values)
    parent_a_happiness = 80
    parent_b_happiness = 90
    
    # Perform the synthesis
    result = synthesizer.synthesize(
        parent_a=parent_a,
        parent_b=parent_b,
        parent_a_happiness=parent_a_happiness,
        parent_b_happiness=parent_b_happiness,
        synthesis_type=synthesis_type,
        zoologist_level=zoologist_level,
        catalysts=[catalyst],
        gene_splicers=gene_splicers or []
    )
    
    # Check the result
    if result.state != SynthesisState.COMPLETED or not result.offspring:
        print(f"Breeding failed: {result.error_message}")
        return None
    
    # Apply gene splicer effects
    offspring = result.offspring
    if gene_splicers:
        for splicer in gene_splicers:
            offspring = splicer.apply(parent_a, parent_b, offspring)
    
    # Add offspring to the family tree
    family_tree.add_pet(offspring)
    
    # Apply inbreeding penalty if necessary
    if inbreeding_coefficient > 0.25:
        # Apply a negative mutation to a random stat
        stat = random.choice(list(Stat))
        current_potential = offspring.potential.stat_potential.get(stat, 50)
        penalty = int(inbreeding_coefficient * 20)  # Higher coefficient = higher penalty
        offspring.potential.stat_potential[stat] = max(1, current_potential - penalty)
        
        print(f"Inbreeding penalty applied: {stat.name} potential reduced by {penalty}.")
    
    # Record the breeding on the blockchain
    print("Recording breeding on the Zoologist's Ledger...")
    
    # Create a transaction to mint the offspring as a pet NFT
    pet_tx = player_wallet.create_pet_mint_transaction(
        species=offspring.core.species,
        aura_color=offspring.core.aura.name,
        genetic_hash=offspring.calculate_genetic_hash(),
        metadata_uri=f"https://api.crittercraft.com/pets/{offspring.calculate_genetic_hash()}"
    )
    
    if not ledger.submit_transaction(pet_tx):
        print("Failed to record breeding on the blockchain.")
        return None
    
    # Create a block to confirm the transaction
    if player_wallet.address in ledger.consensus.validators:
        block = ledger.create_block(player_wallet)
        
        if block:
            print(f"Block created: #{block.block_number}, hash: {block.hash[:8]}...")
    
    # Get the minted pet
    player_pets = ledger.get_pets_by_owner(player_wallet.address)
    
    if not player_pets:
        print("Failed to retrieve the offspring from the blockchain.")
        return None
    
    # Return the ID of the most recently minted pet
    return player_pets[-1].token_id


def battle_with_bred_pet(
    ledger: ZoologistLedger,
    player_wallet: Wallet,
    opponent_wallet: Wallet,
    pet_id: str,
    pet_name: str,
    pet_level: int,
    opponent_pet: Dict,
    environment_type: str,
    player_items: List[str]
) -> Dict:
    """
    Battle with a bred pet and record significant events on the blockchain.
    
    Args:
        ledger: The Zoologist's Ledger instance.
        player_wallet: The wallet of the player.
        opponent_wallet: The wallet of the opponent.
        pet_id: The ID of the pet to battle with.
        pet_name: The name of the pet.
        pet_level: The level of the pet.
        opponent_pet: The opponent's pet data.
        environment_type: The type of environment for the battle.
        player_items: The items the player has available.
        
    Returns:
        The result of the battle.
    """
    # Get the pet from the blockchain
    pet_nft = ledger.get_pet(pet_id)
    
    if not pet_nft:
        print(f"Pet with ID {pet_id} not found.")
        return {"winner": "opponent", "error": "Pet not found"}
    
    # In a real implementation, we would fetch the off-chain metadata
    # For this prototype, we'll generate random metadata
    pet_metadata = {
        "stat_potential": {stat.name: random.randint(50, 80) for stat in Stat},
        "adaptation_slots": random.randint(3, 5),
        "size": random.choice(["TINY", "SMALL", "STANDARD", "LARGE", "HUGE"]),
        "pattern": random.choice(["SOLID", "SPOTTED", "STRIPED", "MOTTLED", "IRIDESCENT"]),
        "marking_color": f"#{random.randint(0, 0xFFFFFF):06x}",
        "glow_intensity": random.uniform(0.0, 1.0)
    }
    
    # Convert the pet to a genetic code
    pet_genetic_code = convert_blockchain_pet_to_genetic_code(pet_nft, pet_metadata)
    
    # Convert the pet to battle format
    player_pet = convert_pet_to_battle_format(pet_genetic_code, pet_name, pet_level)
    
    # Start a battle
    try:
        # In a real game, this would be an interactive battle
        # For the demo, we'll simulate the result
        battle_result = {
            "winner": "player" if random.random() < 0.7 else "opponent",
            "turns_taken": random.randint(3, 8),
            "rewards": {
                "experience": random.randint(10, 30),
                "research_points": random.randint(5, 15),
                "items": random.sample(["healing_salve", "adrenaline_berry", "focus_root"], random.randint(0, 2)),
                "friendship": random.randint(1, 3)
            }
        }
        
        print(f"Battle result: {'Player' if battle_result['winner'] == 'player' else 'Opponent'} wins in {battle_result['turns_taken']} turns!")
        
        # Record significant battle outcomes on the blockchain
        if battle_result["winner"] == "player" and opponent_pet.get("is_alpha", False):
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
                pet_id=pet_id,
                new_form_id=f"evolved_{pet_genetic_code.core.species}_1"
            )
            ledger.submit_transaction(evolve_tx)
            
            # Create a block to confirm the transactions
            if player_wallet.address in ledger.consensus.validators:
                block = ledger.create_block(player_wallet)
                
                if block:
                    print(f"Block created: #{block.block_number}, hash: {block.hash[:8]}...")
        
        return battle_result
    
    except Exception as e:
        print(f"Error during battle: {e}")
        return {"winner": "opponent", "error": str(e)}


if __name__ == "__main__":
    print("This module is not meant to be run directly.")
    print("Import it and use its functions to integrate the breeding system with the battle system and blockchain.")