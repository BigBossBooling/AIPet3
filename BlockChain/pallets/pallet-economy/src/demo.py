"""
Demo script for the Dual-Layer Economy System.

This script demonstrates the economy system, including crafting, trading, and
the dual-layer marketplace.
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

# Import the economy system
from .items import (
    ItemType,
    ItemRarity,
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
from .currencies import Bits, Aura
from .marketplace import (
    LocalMarketplace,
    GlobalMarketplace,
    OrderType
)
from .inventory import Inventory
from .crafting import (
    CraftingSystem,
    Recipe,
    CraftingResult
)


def run_demo():
    """Run a demo of the Dual-Layer Economy System."""
    print("Welcome to the Dual-Layer Economy System Demo!")
    print("=" * 60)
    time.sleep(1)
    
    # Initialize the blockchain
    ledger = ZoologistLedger()
    
    # Create wallets for players
    adventurer_wallet = Wallet()
    crafter_wallet = Wallet()
    breeder_wallet = Wallet()
    
    print(f"Adventurer DID: {adventurer_wallet.address}")
    print(f"Crafter DID: {crafter_wallet.address}")
    print(f"Breeder DID: {breeder_wallet.address}")
    print()
    
    # Initialize player balances (in a real game, this would be earned through gameplay)
    ledger.balances[adventurer_wallet.address] = 1000
    ledger.balances[crafter_wallet.address] = 1000
    ledger.balances[breeder_wallet.address] = 1000
    
    # Create zoologist identities
    ledger.zoologists[adventurer_wallet.address] = ledger.zoologists.get(
        adventurer_wallet.address, 
        ZoologistIdentity(did=adventurer_wallet.address, level=ZoologistLevel.JOURNEYMAN)
    )
    ledger.zoologists[crafter_wallet.address] = ledger.zoologists.get(
        crafter_wallet.address, 
        ZoologistIdentity(did=crafter_wallet.address, level=ZoologistLevel.EXPERT)
    )
    ledger.zoologists[breeder_wallet.address] = ledger.zoologists.get(
        breeder_wallet.address, 
        ZoologistIdentity(did=breeder_wallet.address, level=ZoologistLevel.MASTER)
    )
    
    # Create inventories for players
    adventurer_inventory = Inventory(player_id=adventurer_wallet.address)
    crafter_inventory = Inventory(player_id=crafter_wallet.address)
    breeder_inventory = Inventory(player_id=breeder_wallet.address)
    
    # Create the crafting system
    crafting_system = CraftingSystem()
    
    # Create recipes
    healing_salve_recipe = Recipe(
        id="recipe_healing_salve",
        name="Healing Salve",
        description="A simple healing salve that restores health.",
        result_item_type=ItemType.CONSUMABLE,
        result_item_name="Healing Salve",
        result_item_description="Restores 20 health points.",
        result_item_rarity=ItemRarity.COMMON,
        materials={
            "sunpetal": 2,
            "river_stone": 1
        },
        required_level=1,
        success_chance=0.9
    )
    
    adrenaline_berry_recipe = Recipe(
        id="recipe_adrenaline_berry",
        name="Adrenaline Berry",
        description="A stimulating berry that increases action points.",
        result_item_type=ItemType.CONSUMABLE,
        result_item_name="Adrenaline Berry",
        result_item_description="Grants 2 additional action points for 3 turns.",
        result_item_rarity=ItemRarity.UNCOMMON,
        materials={
            "sunpetal": 1,
            "glow_dust": 2
        },
        required_level=2,
        success_chance=0.8
    )
    
    bark_armor_recipe = Recipe(
        id="recipe_bark_armor",
        name="Bark Armor",
        description="Simple armor made from toughened bark.",
        result_item_type=ItemType.GEAR,
        result_item_name="Bark Armor",
        result_item_description="Provides protection against physical attacks.",
        result_item_rarity=ItemRarity.COMMON,
        materials={
            "toughened_bark": 3,
            "river_stone": 1
        },
        required_level=1,
        success_chance=0.9
    )
    
    stable_catalyst_recipe = Recipe(
        id="recipe_stable_catalyst",
        name="Stable Catalyst",
        description="A catalyst used for standard breeding.",
        result_item_type=ItemType.BRIDGING_ITEM,
        result_item_name="Stable Catalyst",
        result_item_description="Used to initiate standard (intra-species) breeding.",
        result_item_rarity=ItemRarity.UNCOMMON,
        materials={
            "glow_dust": 3,
            "toughened_bark": 2,
            "sunpetal": 2
        },
        required_level=3,
        success_chance=0.7
    )
    
    unstable_catalyst_recipe = Recipe(
        id="recipe_unstable_catalyst",
        name="Unstable Catalyst",
        description="A catalyst used for hybrid breeding.",
        result_item_type=ItemType.BRIDGING_ITEM,
        result_item_name="Unstable Catalyst",
        result_item_description="Used to initiate hybrid (cross-species) breeding.",
        result_item_rarity=ItemRarity.RARE,
        materials={
            "glow_dust": 5,
            "toughened_bark": 3,
            "sunpetal": 3,
            "crystal_shard": 1
        },
        required_level=5,
        success_chance=0.5
    )
    
    # Add recipes to the crafting system
    crafting_system.add_recipe(healing_salve_recipe)
    crafting_system.add_recipe(adrenaline_berry_recipe)
    crafting_system.add_recipe(bark_armor_recipe)
    crafting_system.add_recipe(stable_catalyst_recipe)
    crafting_system.add_recipe(unstable_catalyst_recipe)
    
    # Learn recipes
    crafting_system.learn_recipe(crafter_wallet.address, "recipe_healing_salve")
    crafting_system.learn_recipe(crafter_wallet.address, "recipe_adrenaline_berry")
    crafting_system.learn_recipe(crafter_wallet.address, "recipe_bark_armor")
    crafting_system.learn_recipe(crafter_wallet.address, "recipe_stable_catalyst")
    crafting_system.learn_recipe(crafter_wallet.address, "recipe_unstable_catalyst")
    
    # Create materials
    sunpetal = Material(
        id="sunpetal",
        name="Sunpetal",
        description="A bright yellow flower that grows in sunny areas.",
        item_type=ItemType.MATERIAL,
        rarity=ItemRarity.COMMON,
        source="Gathering from sunny areas"
    )
    
    river_stone = Material(
        id="river_stone",
        name="River Stone",
        description="A smooth stone found in rivers and streams.",
        item_type=ItemType.MATERIAL,
        rarity=ItemRarity.COMMON,
        source="Gathering from rivers and streams"
    )
    
    glow_dust = Material(
        id="glow_dust",
        name="Glow Dust",
        description="A luminescent powder dropped by Glow Sprites.",
        item_type=ItemType.MATERIAL,
        rarity=ItemRarity.UNCOMMON,
        source="Dropped by Glow Sprites"
    )
    
    toughened_bark = Material(
        id="toughened_bark",
        name="Toughened Bark",
        description="Bark from ancient trees that has hardened over time.",
        item_type=ItemType.MATERIAL,
        rarity=ItemRarity.UNCOMMON,
        source="Gathering from ancient trees"
    )
    
    crystal_shard = Material(
        id="crystal_shard",
        name="Crystal Shard",
        description="A fragment of a rare crystal found in deep caves.",
        item_type=ItemType.MATERIAL,
        rarity=ItemRarity.RARE,
        source="Mining in deep caves"
    )
    
    # Add materials to the adventurer's inventory (simulating gathering)
    print("Adventurer gathers materials...")
    adventurer_inventory.add_item(sunpetal, 10)
    adventurer_inventory.add_item(river_stone, 8)
    adventurer_inventory.add_item(glow_dust, 6)
    adventurer_inventory.add_item(toughened_bark, 5)
    adventurer_inventory.add_item(crystal_shard, 2)
    
    print("Adventurer's inventory:")
    for slot_id, slot in adventurer_inventory.slots.items():
        print(f"  {slot.item.name} x{slot.quantity}")
    print()
    
    # Create the local marketplace
    local_marketplace = LocalMarketplace()
    
    # Adventurer lists materials for sale
    print("Adventurer lists materials for sale in the Local Marketplace...")
    
    sunpetal_listing = local_marketplace.create_listing(
        player_id=adventurer_wallet.address,
        item=sunpetal,
        quantity=5,
        price=10  # 10 BITS per Sunpetal
    )
    
    river_stone_listing = local_marketplace.create_listing(
        player_id=adventurer_wallet.address,
        item=river_stone,
        quantity=4,
        price=8  # 8 BITS per River Stone
    )
    
    glow_dust_listing = local_marketplace.create_listing(
        player_id=adventurer_wallet.address,
        item=glow_dust,
        quantity=3,
        price=20  # 20 BITS per Glow Dust
    )
    
    toughened_bark_listing = local_marketplace.create_listing(
        player_id=adventurer_wallet.address,
        item=toughened_bark,
        quantity=2,
        price=15  # 15 BITS per Toughened Bark
    )
    
    crystal_shard_listing = local_marketplace.create_listing(
        player_id=adventurer_wallet.address,
        item=crystal_shard,
        quantity=1,
        price=100  # 100 BITS per Crystal Shard
    )
    
    print("Local Marketplace listings:")
    for listing_id, listing in local_marketplace.listings.items():
        print(f"  {listing.item.name} x{listing.quantity} - {listing.price} BITS each")
    print()
    
    # Crafter buys materials
    print("Crafter buys materials from the Local Marketplace...")
    
    # Buy Sunpetals
    sunpetal_transaction = local_marketplace.buy_listing(
        listing_id=sunpetal_listing.id,
        player_id=crafter_wallet.address
    )
    
    # Buy River Stones
    river_stone_transaction = local_marketplace.buy_listing(
        listing_id=river_stone_listing.id,
        player_id=crafter_wallet.address
    )
    
    # Buy Glow Dust
    glow_dust_transaction = local_marketplace.buy_listing(
        listing_id=glow_dust_listing.id,
        player_id=crafter_wallet.address
    )
    
    # Buy Toughened Bark
    toughened_bark_transaction = local_marketplace.buy_listing(
        listing_id=toughened_bark_listing.id,
        player_id=crafter_wallet.address
    )
    
    # Buy Crystal Shard
    crystal_shard_transaction = local_marketplace.buy_listing(
        listing_id=crystal_shard_listing.id,
        player_id=crafter_wallet.address
    )
    
    print("Transactions:")
    for transaction_id, transaction in local_marketplace.transactions.items():
        print(f"  {transaction.item_id} x{transaction.quantity} - {transaction.price_per_unit} BITS each")
    print()
    
    # Add the purchased materials to the crafter's inventory
    crafter_inventory.add_item(sunpetal, 5)
    crafter_inventory.add_item(river_stone, 4)
    crafter_inventory.add_item(glow_dust, 3)
    crafter_inventory.add_item(toughened_bark, 2)
    crafter_inventory.add_item(crystal_shard, 1)
    
    print("Crafter's inventory:")
    for slot_id, slot in crafter_inventory.slots.items():
        print(f"  {slot.item.name} x{slot.quantity}")
    print()
    
    # Crafter crafts items
    print("Crafter crafts items...")
    
    # Craft Healing Salve
    healing_salve_result, healing_salve = crafting_system.craft_item(
        player_id=crafter_wallet.address,
        recipe_id="recipe_healing_salve",
        inventory=crafter_inventory,
        player_level=4  # Expert level
    )
    
    print(f"Crafting Healing Salve: {healing_salve_result.name}")
    
    # Craft Adrenaline Berry
    adrenaline_berry_result, adrenaline_berry = crafting_system.craft_item(
        player_id=crafter_wallet.address,
        recipe_id="recipe_adrenaline_berry",
        inventory=crafter_inventory,
        player_level=4  # Expert level
    )
    
    print(f"Crafting Adrenaline Berry: {adrenaline_berry_result.name}")
    
    # Craft Bark Armor
    bark_armor_result, bark_armor = crafting_system.craft_item(
        player_id=crafter_wallet.address,
        recipe_id="recipe_bark_armor",
        inventory=crafter_inventory,
        player_level=4  # Expert level
    )
    
    print(f"Crafting Bark Armor: {bark_armor_result.name}")
    
    # Craft Stable Catalyst
    stable_catalyst_result, stable_catalyst = crafting_system.craft_item(
        player_id=crafter_wallet.address,
        recipe_id="recipe_stable_catalyst",
        inventory=crafter_inventory,
        player_level=4  # Expert level
    )
    
    print(f"Crafting Stable Catalyst: {stable_catalyst_result.name}")
    
    print("Crafter's inventory after crafting:")
    for slot_id, slot in crafter_inventory.slots.items():
        print(f"  {slot.item.name} x{slot.quantity}")
    print()
    
    # Crafter lists crafted items for sale
    print("Crafter lists crafted items for sale in the Local Marketplace...")
    
    healing_salve_listing = local_marketplace.create_listing(
        player_id=crafter_wallet.address,
        item=healing_salve,
        quantity=1,
        price=30  # 30 BITS per Healing Salve
    )
    
    adrenaline_berry_listing = local_marketplace.create_listing(
        player_id=crafter_wallet.address,
        item=adrenaline_berry,
        quantity=1,
        price=50  # 50 BITS per Adrenaline Berry
    )
    
    bark_armor_listing = local_marketplace.create_listing(
        player_id=crafter_wallet.address,
        item=bark_armor,
        quantity=1,
        price=60  # 60 BITS per Bark Armor
    )
    
    stable_catalyst_listing = local_marketplace.create_listing(
        player_id=crafter_wallet.address,
        item=stable_catalyst,
        quantity=1,
        price=100  # 100 BITS per Stable Catalyst
    )
    
    print("Local Marketplace listings:")
    for listing_id, listing in local_marketplace.listings.items():
        if not listing.is_sold:
            print(f"  {listing.item.name} x{listing.quantity} - {listing.price} BITS each")
    print()
    
    # Breeder buys items
    print("Breeder buys items from the Local Marketplace...")
    
    # Buy Healing Salve
    healing_salve_transaction = local_marketplace.buy_listing(
        listing_id=healing_salve_listing.id,
        player_id=breeder_wallet.address
    )
    
    # Buy Adrenaline Berry
    adrenaline_berry_transaction = local_marketplace.buy_listing(
        listing_id=adrenaline_berry_listing.id,
        player_id=breeder_wallet.address
    )
    
    # Buy Stable Catalyst
    stable_catalyst_transaction = local_marketplace.buy_listing(
        listing_id=stable_catalyst_listing.id,
        player_id=breeder_wallet.address
    )
    
    print("Transactions:")
    for transaction_id, transaction in local_marketplace.transactions.items():
        if transaction.buyer_id == breeder_wallet.address:
            print(f"  {transaction.item_id} x{transaction.quantity} - {transaction.price_per_unit} BITS each")
    print()
    
    # Add the purchased items to the breeder's inventory
    breeder_inventory.add_item(healing_salve, 1)
    breeder_inventory.add_item(adrenaline_berry, 1)
    breeder_inventory.add_item(stable_catalyst, 1)
    
    print("Breeder's inventory:")
    for slot_id, slot in breeder_inventory.slots.items():
        print(f"  {slot.item.name} x{slot.quantity}")
    print()
    
    # Create the global marketplace
    global_marketplace = GlobalMarketplace(ledger=ledger)
    
    # Create a legendary gear item (simulating a Master Zoologist crafting a legendary item)
    legendary_staff = Gear(
        id="legendary_staff_1",
        name="Staff of the Master Zoologist",
        description="A powerful staff crafted by a Master Zoologist.",
        item_type=ItemType.GEAR,
        rarity=ItemRarity.LEGENDARY,
        stat_boosts={"intelligence": 10, "energy": 5},
        durability=100,
        is_legendary=True,
        nft_id="nft_legendary_staff_1"
    )
    
    # Add the legendary gear to the breeder's inventory
    breeder_inventory.add_item(legendary_staff, 1)
    
    print("Breeder's inventory after receiving legendary gear:")
    for slot_id, slot in breeder_inventory.slots.items():
        print(f"  {slot.item.name} x{slot.quantity}")
    print()
    
    # Breeder lists legendary gear for sale on the Global Marketplace
    print("Breeder lists legendary gear for sale in the Global Marketplace...")
    
    legendary_staff_listing = global_marketplace.create_listing(
        player_id=breeder_wallet.address,
        item=legendary_staff,
        quantity=1,
        price=50  # 50 AURA
    )
    
    print("Global Marketplace listings:")
    for listing_id, listing in global_marketplace.listings.items():
        print(f"  {listing.item.name} x{listing.quantity} - {listing.price} AURA each")
    print()
    
    # Adventurer buys legendary gear (simulating having earned AURA through gameplay)
    print("Adventurer buys legendary gear from the Global Marketplace...")
    
    legendary_staff_transaction = global_marketplace.buy_listing(
        listing_id=legendary_staff_listing.id,
        player_id=adventurer_wallet.address
    )
    
    print("Transactions:")
    for transaction_id, transaction in global_marketplace.transactions.items():
        print(f"  {transaction.item_id} x{transaction.quantity} - {transaction.price_per_unit} AURA each")
    print()
    
    # Add the purchased legendary gear to the adventurer's inventory
    adventurer_inventory.add_item(legendary_staff, 1)
    
    print("Adventurer's inventory after buying legendary gear:")
    for slot_id, slot in adventurer_inventory.slots.items():
        if slot.item.item_type == ItemType.GEAR:
            print(f"  {slot.item.name} x{slot.quantity}")
    print()
    
    print("Thank you for trying the Dual-Layer Economy System Demo!")


if __name__ == "__main__":
    run_demo()