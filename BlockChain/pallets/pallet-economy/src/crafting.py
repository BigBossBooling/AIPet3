"""
Crafting module for the Dual-Layer Economy System.

This module implements the crafting system in the Critter-Craft economy.
"""

import random
import uuid
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, Union, Any

from .items import (
    Item, 
    ItemType, 
    ItemRarity, 
    Material, 
    Consumable, 
    Gear, 
    BridgingItem, 
    BreedingCatalyst, 
    GeneSplicer, 
    NFTMintingKit
)
from .inventory import Inventory


class CraftingResult(Enum):
    """Results of a crafting attempt."""
    SUCCESS = auto()
    FAILURE = auto()
    MISSING_MATERIALS = auto()
    MISSING_BLUEPRINT = auto()
    INSUFFICIENT_LEVEL = auto()


@dataclass
class Recipe:
    """
    A recipe for crafting an item.
    
    This defines the materials required to craft an item.
    """
    id: str
    name: str
    description: str
    result_item_type: ItemType
    result_item_name: str
    result_item_description: str
    result_item_rarity: ItemRarity
    materials: Dict[str, int]  # material_id -> quantity
    required_level: int = 1
    success_chance: float = 1.0
    
    def __post_init__(self):
        """Initialize with a UUID if not provided."""
        if not self.id:
            self.id = str(uuid.uuid4())
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "id": self.id,
            "name": self.name,
            "description": self.description,
            "result_item_type": self.result_item_type.name,
            "result_item_name": self.result_item_name,
            "result_item_description": self.result_item_description,
            "result_item_rarity": self.result_item_rarity.name,
            "materials": self.materials,
            "required_level": self.required_level,
            "success_chance": self.success_chance
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Recipe':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            result_item_type=ItemType[data["result_item_type"]],
            result_item_name=data["result_item_name"],
            result_item_description=data["result_item_description"],
            result_item_rarity=ItemRarity[data["result_item_rarity"]],
            materials=data["materials"],
            required_level=data["required_level"],
            success_chance=data["success_chance"]
        )


class CraftingSystem:
    """
    The crafting system.
    
    This handles the creation of items from recipes.
    """
    
    def __init__(self):
        """Initialize the crafting system."""
        self.recipes: Dict[str, Recipe] = {}
        self.known_recipes: Dict[str, Set[str]] = {}  # player_id -> set of recipe_ids
    
    def add_recipe(self, recipe: Recipe) -> None:
        """
        Add a recipe to the crafting system.
        
        Args:
            recipe: The recipe to add.
        """
        self.recipes[recipe.id] = recipe
    
    def learn_recipe(self, player_id: str, recipe_id: str) -> bool:
        """
        Learn a recipe.
        
        Args:
            player_id: The ID of the player learning the recipe.
            recipe_id: The ID of the recipe to learn.
            
        Returns:
            True if the recipe was learned successfully, False otherwise.
        """
        # Check if the recipe exists
        if recipe_id not in self.recipes:
            return False
        
        # Initialize the player's known recipes if necessary
        if player_id not in self.known_recipes:
            self.known_recipes[player_id] = set()
        
        # Add the recipe to the player's known recipes
        self.known_recipes[player_id].add(recipe_id)
        
        return True
    
    def knows_recipe(self, player_id: str, recipe_id: str) -> bool:
        """
        Check if a player knows a recipe.
        
        Args:
            player_id: The ID of the player to check.
            recipe_id: The ID of the recipe to check.
            
        Returns:
            True if the player knows the recipe, False otherwise.
        """
        return player_id in self.known_recipes and recipe_id in self.known_recipes[player_id]
    
    def get_known_recipes(self, player_id: str) -> List[Recipe]:
        """
        Get all recipes known by a player.
        
        Args:
            player_id: The ID of the player to get recipes for.
            
        Returns:
            A list of recipes known by the player.
        """
        if player_id not in self.known_recipes:
            return []
        
        return [self.recipes[recipe_id] for recipe_id in self.known_recipes[player_id]]
    
    def craft_item(self, player_id: str, recipe_id: str, inventory: Inventory, player_level: int) -> Tuple[CraftingResult, Optional[Item]]:
        """
        Craft an item.
        
        Args:
            player_id: The ID of the player crafting the item.
            recipe_id: The ID of the recipe to use.
            inventory: The player's inventory.
            player_level: The player's level.
            
        Returns:
            A tuple containing the result of the crafting attempt and the crafted item (if successful).
        """
        # Check if the recipe exists
        if recipe_id not in self.recipes:
            return CraftingResult.MISSING_BLUEPRINT, None
        
        recipe = self.recipes[recipe_id]
        
        # Check if the player knows the recipe
        if not self.knows_recipe(player_id, recipe_id):
            return CraftingResult.MISSING_BLUEPRINT, None
        
        # Check if the player has the required level
        if player_level < recipe.required_level:
            return CraftingResult.INSUFFICIENT_LEVEL, None
        
        # Check if the player has the required materials
        for material_id, quantity in recipe.materials.items():
            if not inventory.has_item(material_id, quantity):
                return CraftingResult.MISSING_MATERIALS, None
        
        # Remove the materials from the inventory
        for material_id, quantity in recipe.materials.items():
            inventory.remove_item(material_id, quantity)
        
        # Check if the crafting is successful
        if random.random() > recipe.success_chance:
            return CraftingResult.FAILURE, None
        
        # Create the item
        item = self._create_item_from_recipe(recipe)
        
        # Add the item to the inventory
        inventory.add_item(item)
        
        return CraftingResult.SUCCESS, item
    
    def _create_item_from_recipe(self, recipe: Recipe) -> Item:
        """
        Create an item from a recipe.
        
        Args:
            recipe: The recipe to use.
            
        Returns:
            The created item.
        """
        if recipe.result_item_type == ItemType.CONSUMABLE:
            return Consumable(
                id="",
                name=recipe.result_item_name,
                description=recipe.result_item_description,
                item_type=recipe.result_item_type,
                rarity=recipe.result_item_rarity,
                effect_type="healing",  # Default effect type
                effect_value=10,  # Default effect value
                duration=0  # Default duration
            )
        elif recipe.result_item_type == ItemType.GEAR:
            return Gear(
                id="",
                name=recipe.result_item_name,
                description=recipe.result_item_description,
                item_type=recipe.result_item_type,
                rarity=recipe.result_item_rarity,
                stat_boosts={"strength": 5},  # Default stat boosts
                durability=100,  # Default durability
                is_legendary=False  # Default is not legendary
            )
        elif recipe.result_item_type == ItemType.BRIDGING_ITEM:
            if "catalyst" in recipe.result_item_name.lower():
                return BreedingCatalyst(
                    id="",
                    name=recipe.result_item_name,
                    description=recipe.result_item_description,
                    item_type=recipe.result_item_type,
                    rarity=recipe.result_item_rarity,
                    is_stable="stable" in recipe.result_item_name.lower(),
                    quality=1  # Default quality
                )
            elif "splicer" in recipe.result_item_name.lower():
                return GeneSplicer(
                    id="",
                    name=recipe.result_item_name,
                    description=recipe.result_item_description,
                    item_type=recipe.result_item_type,
                    rarity=recipe.result_item_rarity,
                    splicer_type="dominant",  # Default splicer type
                    target_gene="size"  # Default target gene
                )
            elif "minting" in recipe.result_item_name.lower():
                return NFTMintingKit(
                    id="",
                    name=recipe.result_item_name,
                    description=recipe.result_item_description,
                    item_type=recipe.result_item_type,
                    rarity=recipe.result_item_rarity,
                    gear_type="weapon"  # Default gear type
                )
            else:
                return BridgingItem(
                    id="",
                    name=recipe.result_item_name,
                    description=recipe.result_item_description,
                    item_type=recipe.result_item_type,
                    rarity=recipe.result_item_rarity,
                    bridging_type="generic"  # Default bridging type
                )
        else:
            # Default to a material
            return Material(
                id="",
                name=recipe.result_item_name,
                description=recipe.result_item_description,
                item_type=ItemType.MATERIAL,
                rarity=recipe.result_item_rarity,
                source="crafting"  # Default source
            )
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "recipes": {recipe_id: recipe.to_dict() for recipe_id, recipe in self.recipes.items()},
            "known_recipes": {player_id: list(recipe_ids) for player_id, recipe_ids in self.known_recipes.items()}
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'CraftingSystem':
        """Create from a dictionary."""
        crafting_system = cls()
        
        # Add recipes
        for recipe_id, recipe_data in data.get("recipes", {}).items():
            crafting_system.recipes[recipe_id] = Recipe.from_dict(recipe_data)
        
        # Add known recipes
        for player_id, recipe_ids in data.get("known_recipes", {}).items():
            crafting_system.known_recipes[player_id] = set(recipe_ids)
        
        return crafting_system