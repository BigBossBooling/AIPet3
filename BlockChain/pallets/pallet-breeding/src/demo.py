"""
Demo script for the Echo-Synthesis breeding system.

This script demonstrates the breeding mechanics, including standard breeding
(Intra-Species Synthesis) and cross-species breeding (Hybrid Synthesis).
"""

import sys
import os
import time
import random
from typing import Dict, List, Optional

# Add the parent directory to the Python path to import the ledger system
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-ledger', 'src'))

# Import the blockchain system
from ledger import ZoologistLedger
from wallet import Wallet
from models import TransactionType, ZoologistLevel

# Import the breeding system
from .genetics import (
    GeneticCode, 
    CoreGenes, 
    PotentialGenes, 
    CosmeticGenes, 
    AuraType,
    Stat,
    Size,
    Pattern
)
from .synthesis import (
    EchoSynthesizer, 
    SynthesisResult, 
    SynthesisType, 
    SynthesisState
)
from .catalysts import (
    StableCatalyst, 
    UnstableCatalyst, 
    DominantGeneSplice, 
    AuraStabilizer, 
    PotentialSerum, 
    AdaptationMemoryCell
)
from .lineage import (
    FamilyTree, 
    calculate_inbreeding_coefficient
)


def run_demo():
    """Run a demo of the Echo-Synthesis breeding system."""
    print("Welcome to the Echo-Synthesis Breeding System Demo!")
    print("=" * 60)
    time.sleep(1)
    
    # Initialize the blockchain
    ledger = ZoologistLedger()
    
    # Create wallets for players
    player_wallet = Wallet()
    
    print(f"Player DID: {player_wallet.address}")
    print()
    
    # Initialize player balances (in a real game, this would be earned through gameplay)
    ledger.balances[player_wallet.address] = 1000
    
    # Create zoologist identity
    ledger.zoologists[player_wallet.address] = ledger.zoologists.get(
        player_wallet.address, 
        ZoologistIdentity(did=player_wallet.address, level=ZoologistLevel.MASTER)
    )
    
    # Create the Echo-Synthesizer
    synthesizer = EchoSynthesizer()
    
    # Create a family tree
    family_tree = FamilyTree()
    
    # Generate parent pets
    print("Generating parent pets...")
    
    # Parent A: Chameleon with Gold aura
    parent_a = GeneticCode.generate_random(
        species="sprite_chameleon",
        aura=AuraType.GOLD
    )
    
    # Parent B: Chameleon with Blue aura
    parent_b = GeneticCode.generate_random(
        species="sprite_chameleon",
        aura=AuraType.BLUE
    )
    
    # Add parents to the family tree
    family_tree.add_pet(parent_a)
    family_tree.add_pet(parent_b)
    
    # Display parent information
    print(f"Parent A: {parent_a.core.species} with {parent_a.core.aura.name} aura")
    print(f"  Size: {parent_a.cosmetic.size.name}")
    print(f"  Pattern: {parent_a.cosmetic.pattern.name}")
    print(f"  Color: {parent_a.cosmetic.marking_color}")
    print(f"  Adaptation Slots: {parent_a.potential.adaptation_slots}")
    print(f"  Stat Potentials:")
    for stat, value in parent_a.potential.stat_potential.items():
        print(f"    {stat.name}: {value}")
    print()
    
    print(f"Parent B: {parent_b.core.species} with {parent_b.core.aura.name} aura")
    print(f"  Size: {parent_b.cosmetic.size.name}")
    print(f"  Pattern: {parent_b.cosmetic.pattern.name}")
    print(f"  Color: {parent_b.cosmetic.marking_color}")
    print(f"  Adaptation Slots: {parent_b.potential.adaptation_slots}")
    print(f"  Stat Potentials:")
    for stat, value in parent_b.potential.stat_potential.items():
        print(f"    {stat.name}: {value}")
    print()
    
    # Perform standard (intra-species) synthesis
    print("Performing standard (intra-species) synthesis...")
    
    # Create a stable catalyst
    catalyst = StableCatalyst(quality=3)
    print(f"Using {catalyst.name}: {catalyst.description}")
    
    # Create a gene splicer
    gene_splicer = DominantGeneSplice(gene_type="pattern", parent_index=0)
    print(f"Using {gene_splicer.name}: {gene_splicer.description}")
    
    # Set parent happiness (in a real game, this would be the actual happiness values)
    parent_a_happiness = 80
    parent_b_happiness = 90
    
    # Perform the synthesis
    result = synthesizer.synthesize(
        parent_a=parent_a,
        parent_b=parent_b,
        parent_a_happiness=parent_a_happiness,
        parent_b_happiness=parent_b_happiness,
        synthesis_type=SynthesisType.INTRA_SPECIES,
        zoologist_level=5,
        catalysts=[catalyst],
        gene_splicers=[gene_splicer]
    )
    
    # Check the result
    if result.state == SynthesisState.COMPLETED and result.offspring:
        print("Synthesis successful!")
        offspring = result.offspring
        
        # Apply gene splicer effects
        offspring = gene_splicer.apply(parent_a, parent_b, offspring)
        
        # Add offspring to the family tree
        family_tree.add_pet(offspring)
        
        # Display offspring information
        print(f"Offspring: {offspring.core.species} with {offspring.core.aura.name} aura")
        print(f"  Size: {offspring.cosmetic.size.name}")
        print(f"  Pattern: {offspring.cosmetic.pattern.name}")
        print(f"  Color: {offspring.cosmetic.marking_color}")
        print(f"  Adaptation Slots: {offspring.potential.adaptation_slots}")
        print(f"  Stat Potentials:")
        for stat, value in offspring.potential.stat_potential.items():
            print(f"    {stat.name}: {value}")
        
        # Calculate inbreeding coefficient
        inbreeding = calculate_inbreeding_coefficient(family_tree, parent_a, parent_b)
        print(f"Inbreeding Coefficient: {inbreeding:.2f}")
        
        # Record the breeding on the blockchain
        print("\nRecording breeding on the Zoologist's Ledger...")
        
        # Create a transaction to mint the offspring as a pet NFT
        pet_tx = player_wallet.create_pet_mint_transaction(
            species=offspring.core.species,
            aura_color=offspring.core.aura.name,
            genetic_hash=offspring.calculate_genetic_hash(),
            metadata_uri=f"https://api.crittercraft.com/pets/{offspring.calculate_genetic_hash()}"
        )
        ledger.submit_transaction(pet_tx)
        
        # Create a block to confirm the transaction
        ledger.consensus.register_validator(
            player_wallet.address, 
            100,  # Stake amount
            ledger.zoologists[player_wallet.address].reputation_score
        )
        block = ledger.create_block(player_wallet)
        
        if block:
            print(f"Block created: #{block.block_number}, hash: {block.hash[:8]}...")
        
        # Get the minted pet
        player_pets = ledger.get_pets_by_owner(player_wallet.address)
        
        if player_pets:
            print(f"Pet minted on the blockchain with ID: {player_pets[-1].token_id}")
    else:
        print(f"Synthesis failed: {result.error_message}")
    
    print()
    
    # Generate parents for hybrid synthesis
    print("Generating parents for hybrid synthesis...")
    
    # Parent C: Anglerfish with Purple aura
    parent_c = GeneticCode.generate_random(
        species="sprite_anglerfish",
        aura=AuraType.PURPLE
    )
    
    # Add parent to the family tree
    family_tree.add_pet(parent_c)
    
    # Display parent information
    print(f"Parent C: {parent_c.core.species} with {parent_c.core.aura.name} aura")
    print()
    
    # Perform hybrid synthesis
    print("Performing hybrid (cross-species) synthesis...")
    
    # Create an unstable catalyst
    catalyst = UnstableCatalyst(quality=4)
    print(f"Using {catalyst.name}: {catalyst.description}")
    
    # Create gene splicers
    gene_splicers = [
        AuraStabilizer(parent_index=0),
        PotentialSerum(target_stat=Stat.INTELLIGENCE)
    ]
    for splicer in gene_splicers:
        print(f"Using {splicer.name}: {splicer.description}")
    
    # Set parent happiness
    parent_a_happiness = 85
    parent_c_happiness = 75
    
    # Perform the synthesis
    result = synthesizer.synthesize(
        parent_a=parent_a,
        parent_b=parent_c,
        parent_a_happiness=parent_a_happiness,
        parent_b_happiness=parent_c_happiness,
        synthesis_type=SynthesisType.HYBRID,
        zoologist_level=5,
        catalysts=[catalyst],
        gene_splicers=gene_splicers
    )
    
    # Check the result
    if result.state == SynthesisState.COMPLETED and result.offspring:
        print("Hybrid synthesis successful!")
        hybrid = result.offspring
        
        # Apply gene splicer effects
        for splicer in gene_splicers:
            hybrid = splicer.apply(parent_a, parent_c, hybrid)
        
        # Add hybrid to the family tree
        family_tree.add_pet(hybrid)
        
        # Display hybrid information
        print(f"Hybrid Offspring: {hybrid.core.species} with {hybrid.core.aura.name} aura")
        print(f"  Size: {hybrid.cosmetic.size.name}")
        print(f"  Pattern: {hybrid.cosmetic.pattern.name}")
        print(f"  Color: {hybrid.cosmetic.marking_color}")
        print(f"  Adaptation Slots: {hybrid.potential.adaptation_slots}")
        print(f"  Stat Potentials:")
        for stat, value in hybrid.potential.stat_potential.items():
            print(f"    {stat.name}: {value}")
        
        # Record the hybrid breeding on the blockchain
        print("\nRecording hybrid breeding on the Zoologist's Ledger...")
        
        # Create a transaction to mint the hybrid as a pet NFT
        pet_tx = player_wallet.create_pet_mint_transaction(
            species=hybrid.core.species,
            aura_color=hybrid.core.aura.name,
            genetic_hash=hybrid.calculate_genetic_hash(),
            metadata_uri=f"https://api.crittercraft.com/pets/{hybrid.calculate_genetic_hash()}"
        )
        ledger.submit_transaction(pet_tx)
        
        # Create a block to confirm the transaction
        block = ledger.create_block(player_wallet)
        
        if block:
            print(f"Block created: #{block.block_number}, hash: {block.hash[:8]}...")
        
        # Get the minted pet
        player_pets = ledger.get_pets_by_owner(player_wallet.address)
        
        if player_pets:
            print(f"Hybrid pet minted on the blockchain with ID: {player_pets[-1].token_id}")
    else:
        print(f"Hybrid synthesis failed: {result.error_message}")
    
    print("\nThank you for trying the Echo-Synthesis Breeding System Demo!")


if __name__ == "__main__":
    run_demo()