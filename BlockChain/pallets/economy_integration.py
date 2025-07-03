"""
Integration module for the Dual-Layer Economy System with the battle system,
the breeding system, and the Zoologist's Ledger.

This module provides functions for integrating the economy system with the
other systems in Critter-Craft.
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
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'pallet-economy', 'src'))

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

# Import the economy system
from items import (
    ItemType,
    ItemRarity,
    Item,
    Material,
    Consumable,
    Gear,
    Blueprint,
    QuestItem,
    BridgingItem,
    BreedingCatalyst,
    GeneSplicer,
    NFTMintingKit
)
from currencies import Bits, Aura
from marketplace import (
    LocalMarketplace,
    GlobalMarketplace,
    OrderType
)
from inventory import Inventory
from crafting import (
    CraftingSystem,
    Recipe,
    CraftingResult
)


def battle_with_items(
    player_pet: Dict,
    opponent_pet: Dict,
    environment_type: str,
    player_items: List[Item],
    inventory: Inventory
) -> Dict:
    """
    Battle with items from the player's inventory.
    
    Args:
        player_pet: The player's pet data.
        opponent_pet: The opponent's pet data.
        environment_type: The type of environment for the battle.
        player_items: The items the player wants to use in the battle.
        inventory: The player's inventory.
        
    Returns:
        The result of the battle.
    """
    # Check if the player has the items in their inventory
    for item in player_items:
        if not inventory.has_item(item.id):
            print(f"Player does not have {item.name} in their inventory.")
            return {"winner": "opponent", "error": f"Missing item: {item.name}"}
    
    # Convert the items to the format expected by the battle system
    battle_items = []
    for item in player_items:
        if item.item_type == ItemType.CONSUMABLE:
            battle_items.append({
                "id": item.id,
                "name": item.name,
                "effect_type": item.effect_type if hasattr(item, "effect_type") else "healing",
                "effect_value": item.effect_value if hasattr(item, "effect_value") else 10,
                "duration": item.duration if hasattr(item, "duration") else 0
            })
    
    # Start the battle
    try:
        # In a real game, this would be an interactive battle
        # For the demo, we'll simulate the result
        battle_result = {
            "winner": "player" if random.random() < 0.7 else "opponent",
            "turns_taken": random.randint(3, 8),
            "items_used": [item.id for item in player_items],
            "rewards": {
                "experience": random.randint(10, 30),
                "research_points": random.randint(5, 15),
                "items": random.sample(["healing_salve", "adrenaline_berry", "focus_root"], random.randint(0, 2)),
                "friendship": random.randint(1, 3)
            }
        }
        
        # Remove used items from the inventory
        for item in player_items:
            inventory.remove_item(item.id)
        
        # Add reward items to the inventory
        for item_id in battle_result["rewards"]["items"]:
            # In a real game, these would be actual items
            # For the demo, we'll create dummy items
            reward_item = Consumable(
                id=item_id,
                name=item_id.replace("_", " ").title(),
                description=f"A reward from battle.",
                item_type=ItemType.CONSUMABLE,
                rarity=ItemRarity.COMMON,
                effect_type="healing",
                effect_value=10,
                duration=0
            )
            inventory.add_item(reward_item)
        
        return battle_result
    
    except Exception as e:
        print(f"Error during battle: {e}")
        return {"winner": "opponent", "error": str(e)}


def craft_breeding_catalyst(
    player_wallet: Wallet,
    inventory: Inventory,
    crafting_system: CraftingSystem,
    is_stable: bool = True
) -> Optional[BreedingCatalyst]:
    """
    Craft a breeding catalyst.
    
    Args:
        player_wallet: The wallet of the player crafting the catalyst.
        inventory: The player's inventory.
        crafting_system: The crafting system.
        is_stable: Whether to craft a stable catalyst (True) or an unstable catalyst (False).
        
    Returns:
        The crafted catalyst, or None if crafting failed.
    """
    # Get the player's zoologist level
    zoologist_level = 1  # Default to Novice
    
    # In a real game, this would get the actual level from the ledger
    # For the demo, we'll use a random level
    zoologist_level = random.randint(1, 5)
    
    # Choose the appropriate recipe
    recipe_id = "recipe_stable_catalyst" if is_stable else "recipe_unstable_catalyst"
    
    # Check if the player knows the recipe
    if not crafting_system.knows_recipe(player_wallet.address, recipe_id):
        print(f"Player does not know the {recipe_id} recipe.")
        return None
    
    # Craft the catalyst
    result, catalyst = crafting_system.craft_item(
        player_id=player_wallet.address,
        recipe_id=recipe_id,
        inventory=inventory,
        player_level=zoologist_level
    )
    
    if result != CraftingResult.SUCCESS or not catalyst:
        print(f"Failed to craft catalyst: {result.name}")
        return None
    
    return catalyst


def breed_with_catalyst(
    ledger: ZoologistLedger,
    player_wallet: Wallet,
    inventory: Inventory,
    parent_a_id: str,
    parent_b_id: str,
    catalyst: BreedingCatalyst
) -> Optional[str]:
    """
    Breed two pets using a catalyst from the player's inventory.
    
    Args:
        ledger: The Zoologist's Ledger instance.
        player_wallet: The wallet of the player performing the breeding.
        inventory: The player's inventory.
        parent_a_id: The ID of the first parent pet.
        parent_b_id: The ID of the second parent pet.
        catalyst: The breeding catalyst to use.
        
    Returns:
        The ID of the offspring pet, or None if breeding failed.
    """
    # Check if the player has the catalyst in their inventory
    if not inventory.has_item(catalyst.id):
        print(f"Player does not have {catalyst.name} in their inventory.")
        return None
    
    # Determine the synthesis type based on the catalyst
    synthesis_type = SynthesisType.INTRA_SPECIES if catalyst.is_stable else SynthesisType.HYBRID
    
    # Remove the catalyst from the inventory
    inventory.remove_item(catalyst.id)
    
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
    parent_a = GeneticCode(
        core=CoreGenes(
            species=parent_a_nft.species,
            aura=AuraType[parent_a_nft.aura_color],
            genesis_id=parent_a_nft.token_id,
            lineage=parent_a_nft.evolution_history
        ),
        potential=PotentialGenes(
            stat_potential={Stat[stat]: value for stat, value in parent_a_metadata["stat_potential"].items()},
            adaptation_slots=parent_a_metadata["adaptation_slots"]
        ),
        cosmetic=CosmeticGenes(
            size=parent_a_metadata["size"],
            pattern=parent_a_metadata["pattern"],
            marking_color=parent_a_metadata["marking_color"],
            glow_intensity=parent_a_metadata["glow_intensity"]
        )
    )
    
    parent_b = GeneticCode(
        core=CoreGenes(
            species=parent_b_nft.species,
            aura=AuraType[parent_b_nft.aura_color],
            genesis_id=parent_b_nft.token_id,
            lineage=parent_b_nft.evolution_history
        ),
        potential=PotentialGenes(
            stat_potential={Stat[stat]: value for stat, value in parent_b_metadata["stat_potential"].items()},
            adaptation_slots=parent_b_metadata["adaptation_slots"]
        ),
        cosmetic=CosmeticGenes(
            size=parent_b_metadata["size"],
            pattern=parent_b_metadata["pattern"],
            marking_color=parent_b_metadata["marking_color"],
            glow_intensity=parent_b_metadata["glow_intensity"]
        )
    )
    
    # Create a family tree
    family_tree = FamilyTree()
    family_tree.add_pet(parent_a)
    family_tree.add_pet(parent_b)
    
    # Check for inbreeding
    inbreeding_coefficient = calculate_inbreeding_coefficient(family_tree, parent_a, parent_b)
    
    if inbreeding_coefficient > 0.25:
        print(f"Warning: High inbreeding coefficient ({inbreeding_coefficient:.2f}).")
        print("This breeding has a high risk of negative mutations.")
    
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
        catalysts=[catalyst]
    )
    
    # Check the result
    if result.state != SynthesisState.COMPLETED or not result.offspring:
        print(f"Breeding failed: {result.error_message}")
        return None
    
    # Add offspring to the family tree
    family_tree.add_pet(result.offspring)
    
    # Apply inbreeding penalty if necessary
    if inbreeding_coefficient > 0.25:
        # Apply a negative mutation to a random stat
        stat = random.choice(list(Stat))
        current_potential = result.offspring.potential.stat_potential.get(stat, 50)
        penalty = int(inbreeding_coefficient * 20)  # Higher coefficient = higher penalty
        result.offspring.potential.stat_potential[stat] = max(1, current_potential - penalty)
        
        print(f"Inbreeding penalty applied: {stat.name} potential reduced by {penalty}.")
    
    # Record the breeding on the blockchain
    print("Recording breeding on the Zoologist's Ledger...")
    
    # Create a transaction to mint the offspring as a pet NFT
    pet_tx = player_wallet.create_pet_mint_transaction(
        species=result.offspring.core.species,
        aura_color=result.offspring.core.aura.name,
        genetic_hash=result.offspring.calculate_genetic_hash(),
        metadata_uri=f"https://api.crittercraft.com/pets/{result.offspring.calculate_genetic_hash()}"
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


def mint_legendary_gear(
    ledger: ZoologistLedger,
    player_wallet: Wallet,
    inventory: Inventory,
    gear: Gear,
    minting_kit: NFTMintingKit
) -> Optional[Gear]:
    """
    Mint a piece of gear as a legendary NFT.
    
    Args:
        ledger: The Zoologist's Ledger instance.
        player_wallet: The wallet of the player minting the gear.
        inventory: The player's inventory.
        gear: The gear to mint.
        minting_kit: The NFT minting kit to use.
        
    Returns:
        The minted legendary gear, or None if minting failed.
    """
    # Check if the player has the gear and minting kit in their inventory
    if not inventory.has_item(gear.id):
        print(f"Player does not have {gear.name} in their inventory.")
        return None
    
    if not inventory.has_item(minting_kit.id):
        print(f"Player does not have {minting_kit.name} in their inventory.")
        return None
    
    # Check if the gear is already legendary
    if gear.is_legendary:
        print(f"{gear.name} is already a legendary item.")
        return None
    
    # Check if the minting kit is compatible with the gear
    if minting_kit.gear_type and gear.id.split("_")[0] != minting_kit.gear_type:
        print(f"{minting_kit.name} is not compatible with {gear.name}.")
        return None
    
    # Check if the player is a Master Zoologist
    zoologist = ledger.get_zoologist(player_wallet.address)
    
    if not zoologist or zoologist.level != ZoologistLevel.MASTER:
        print("Only Master Zoologists can mint legendary gear.")
        return None
    
    # Remove the minting kit from the inventory
    inventory.remove_item(minting_kit.id)
    
    # Create a transaction to mint the gear as an NFT
    gear_tx = player_wallet.create_gear_mint_transaction(
        item_type=gear.id.split("_")[0],
        rarity=gear.rarity.name,
        metadata_uri=f"https://api.crittercraft.com/gear/{gear.id}"
    )
    
    if not ledger.submit_transaction(gear_tx):
        print("Failed to mint legendary gear on the blockchain.")
        return None
    
    # Create a block to confirm the transaction
    if player_wallet.address in ledger.consensus.validators:
        block = ledger.create_block(player_wallet)
        
        if block:
            print(f"Block created: #{block.block_number}, hash: {block.hash[:8]}...")
    
    # Get the minted gear
    player_gear = ledger.get_gear_by_owner(player_wallet.address)
    
    if not player_gear:
        print("Failed to retrieve the legendary gear from the blockchain.")
        return None
    
    # Update the gear to be legendary
    gear.is_legendary = True
    gear.nft_id = player_gear[-1].token_id
    
    # Remove the old gear from the inventory
    inventory.remove_item(gear.id)
    
    # Add the legendary gear to the inventory
    inventory.add_item(gear)
    
    return gear


def sell_on_marketplace(
    player_wallet: Wallet,
    inventory: Inventory,
    item: Item,
    quantity: int,
    price: int,
    marketplace: Union[LocalMarketplace, GlobalMarketplace]
) -> Optional[str]:
    """
    Sell an item on the marketplace.
    
    Args:
        player_wallet: The wallet of the player selling the item.
        inventory: The player's inventory.
        item: The item to sell.
        quantity: The quantity to sell.
        price: The price per unit.
        marketplace: The marketplace to sell on.
        
    Returns:
        The ID of the listing, or None if the listing could not be created.
    """
    # Check if the player has the item in their inventory
    if not inventory.has_item(item.id, quantity):
        print(f"Player does not have {quantity} {item.name} in their inventory.")
        return None
    
    # Check if the item is tradable
    if not item.is_tradable:
        print(f"{item.name} is not tradable.")
        return None
    
    # Check if the item is appropriate for the marketplace
    if isinstance(marketplace, GlobalMarketplace) and (not hasattr(item, "is_legendary") or not item.is_legendary):
        print(f"{item.name} is not a legendary item and cannot be sold on the Global Marketplace.")
        return None
    
    if isinstance(marketplace, LocalMarketplace) and hasattr(item, "is_legendary") and item.is_legendary:
        print(f"{item.name} is a legendary item and cannot be sold on the Local Marketplace.")
        return None
    
    # Create the listing
    listing = marketplace.create_listing(
        player_id=player_wallet.address,
        item=item,
        quantity=quantity,
        price=price
    )
    
    if not listing:
        print("Failed to create listing.")
        return None
    
    # Remove the item from the inventory
    inventory.remove_item(item.id, quantity)
    
    return listing.id


def buy_from_marketplace(
    player_wallet: Wallet,
    inventory: Inventory,
    listing_id: str,
    marketplace: Union[LocalMarketplace, GlobalMarketplace]
) -> bool:
    """
    Buy an item from the marketplace.
    
    Args:
        player_wallet: The wallet of the player buying the item.
        inventory: The player's inventory.
        listing_id: The ID of the listing to buy.
        marketplace: The marketplace to buy from.
        
    Returns:
        True if the purchase was successful, False otherwise.
    """
    # Get the listing
    listing = marketplace.listings.get(listing_id)
    
    if not listing or listing.is_sold:
        print("Listing not found or already sold.")
        return False
    
    # Check if the player has enough currency
    # In a real game, this would check the player's balance
    # For the demo, we'll assume the player has enough
    
    # Buy the listing
    transaction = marketplace.buy_listing(
        listing_id=listing_id,
        player_id=player_wallet.address
    )
    
    if not transaction:
        print("Failed to buy listing.")
        return False
    
    # Add the item to the inventory
    inventory.add_item(listing.item, listing.quantity)
    
    return True


if __name__ == "__main__":
    print("This module is not meant to be run directly.")
    print("Import it and use its functions to integrate the economy system with the other systems.")