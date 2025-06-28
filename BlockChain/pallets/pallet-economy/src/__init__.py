"""
Dual-Layer Economy System for Critter-Craft

A strategic, dual-layer ecosystem that separates high-frequency, everyday activities
from high-value, permanent asset transactions. This creates an accessible economy
for all players while providing deep strategic layers for dedicated masters.
"""

from .items import (
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
from .currencies import (
    Currency,
    Bits,
    Aura
)
from .marketplace import (
    LocalMarketplace,
    GlobalMarketplace,
    Listing,
    Order,
    OrderType,
    Transaction
)
from .inventory import (
    Inventory,
    InventorySlot
)
from .crafting import (
    CraftingSystem,
    Recipe,
    CraftingResult
)

__all__ = [
    'ItemType',
    'ItemRarity',
    'Item',
    'Material',
    'Consumable',
    'Gear',
    'Blueprint',
    'QuestItem',
    'BridgingItem',
    'BreedingCatalyst',
    'GeneSplicer',
    'NFTMintingKit',
    'Currency',
    'Bits',
    'Aura',
    'LocalMarketplace',
    'GlobalMarketplace',
    'Listing',
    'Order',
    'OrderType',
    'Transaction',
    'Inventory',
    'InventorySlot',
    'CraftingSystem',
    'Recipe',
    'CraftingResult'
]