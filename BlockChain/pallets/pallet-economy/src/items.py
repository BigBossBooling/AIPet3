"""
Items module for the Dual-Layer Economy System.

This module defines the various item types in the Critter-Craft economy.
"""

import uuid
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, Union, Any


class ItemType(Enum):
    """Types of items in the Critter-Craft economy."""
    MATERIAL = auto()       # Raw materials for crafting
    CONSUMABLE = auto()     # Items that are consumed on use
    GEAR = auto()           # Equipment that provides stat boosts
    BLUEPRINT = auto()      # Recipes for crafting
    QUEST_ITEM = auto()     # Items for quests (soulbound)
    BRIDGING_ITEM = auto()  # Items that enable on-chain actions


class ItemRarity(Enum):
    """Rarity levels for items."""
    COMMON = auto()
    UNCOMMON = auto()
    RARE = auto()
    EPIC = auto()
    LEGENDARY = auto()


@dataclass
class Item(ABC):
    """
    Base class for all items in the Critter-Craft economy.
    """
    id: str
    name: str
    description: str
    item_type: ItemType
    rarity: ItemRarity
    stack_size: int = 1
    is_tradable: bool = True
    is_soulbound: bool = False
    
    def __post_init__(self):
        """Initialize with a UUID if not provided."""
        if not self.id:
            self.id = str(uuid.uuid4())
    
    @abstractmethod
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Use the item.
        
        Args:
            user: The entity using the item.
            target: The target of the item use, if any.
            
        Returns:
            True if the item was used successfully, False otherwise.
        """
        pass
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "id": self.id,
            "name": self.name,
            "description": self.description,
            "item_type": self.item_type.name,
            "rarity": self.rarity.name,
            "stack_size": self.stack_size,
            "is_tradable": self.is_tradable,
            "is_soulbound": self.is_soulbound
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Item':
        """Create from a dictionary."""
        item_type = ItemType[data["item_type"]]
        
        # Create the appropriate item type
        if item_type == ItemType.MATERIAL:
            return Material.from_dict(data)
        elif item_type == ItemType.CONSUMABLE:
            return Consumable.from_dict(data)
        elif item_type == ItemType.GEAR:
            return Gear.from_dict(data)
        elif item_type == ItemType.BLUEPRINT:
            return Blueprint.from_dict(data)
        elif item_type == ItemType.QUEST_ITEM:
            return QuestItem.from_dict(data)
        elif item_type == ItemType.BRIDGING_ITEM:
            return BridgingItem.from_dict(data)
        
        raise ValueError(f"Unknown item type: {item_type}")


@dataclass
class Material(Item):
    """
    Raw materials for crafting.
    
    These are gathered from the world or dropped from pacified critters.
    """
    source: str = ""  # Where the material comes from
    
    def __post_init__(self):
        """Initialize with default values."""
        super().__post_init__()
        if not hasattr(self, 'item_type') or self.item_type is None:
            self.item_type = ItemType.MATERIAL
        self.stack_size = 99  # Materials can stack up to 99
    
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Materials cannot be used directly.
        
        Args:
            user: The entity using the material.
            target: The target of the material use, if any.
            
        Returns:
            False, as materials cannot be used directly.
        """
        return False
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data["source"] = self.source
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Material':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            item_type=ItemType[data["item_type"]],
            rarity=ItemRarity[data["rarity"]],
            stack_size=data["stack_size"],
            is_tradable=data["is_tradable"],
            is_soulbound=data["is_soulbound"],
            source=data.get("source", "")
        )


@dataclass
class Consumable(Item):
    """
    Items that are consumed on use.
    
    These are player-crafted items for battle and pet care.
    """
    effect_type: str = ""  # The type of effect (e.g., "healing", "buff")
    effect_value: int = 0  # The value of the effect
    duration: int = 0      # The duration of the effect in turns (0 for instant)
    
    def __post_init__(self):
        """Initialize with default values."""
        super().__post_init__()
        if not hasattr(self, 'item_type') or self.item_type is None:
            self.item_type = ItemType.CONSUMABLE
        self.stack_size = 10  # Consumables can stack up to 10
    
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Use the consumable.
        
        Args:
            user: The entity using the consumable.
            target: The target of the consumable use, if any.
            
        Returns:
            True if the consumable was used successfully, False otherwise.
        """
        # In a real implementation, this would apply the effect to the target
        # For this prototype, we'll just return True
        return True
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data["effect_type"] = self.effect_type
        data["effect_value"] = self.effect_value
        data["duration"] = self.duration
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Consumable':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            item_type=ItemType[data["item_type"]],
            rarity=ItemRarity[data["rarity"]],
            stack_size=data["stack_size"],
            is_tradable=data["is_tradable"],
            is_soulbound=data["is_soulbound"],
            effect_type=data.get("effect_type", ""),
            effect_value=data.get("effect_value", 0),
            duration=data.get("duration", 0)
        )


@dataclass
class Gear(Item):
    """
    Equipment that provides stat boosts.
    
    These are craftable items that provide stat boosts to pets.
    """
    stat_boosts: Dict[str, int] = field(default_factory=dict)  # Stat name -> boost value
    durability: int = 100  # Durability of the gear (100 = new)
    is_legendary: bool = False  # Whether this is a legendary NFT gear
    nft_id: Optional[str] = None  # The NFT ID if this is a legendary gear
    
    def __post_init__(self):
        """Initialize with default values."""
        super().__post_init__()
        if not hasattr(self, 'item_type') or self.item_type is None:
            self.item_type = ItemType.GEAR
        self.stack_size = 1  # Gear cannot stack
    
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Equip the gear.
        
        Args:
            user: The entity equipping the gear.
            target: The pet to equip the gear on.
            
        Returns:
            True if the gear was equipped successfully, False otherwise.
        """
        # In a real implementation, this would equip the gear on the target
        # For this prototype, we'll just return True
        return True
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data["stat_boosts"] = self.stat_boosts
        data["durability"] = self.durability
        data["is_legendary"] = self.is_legendary
        data["nft_id"] = self.nft_id
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Gear':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            item_type=ItemType[data["item_type"]],
            rarity=ItemRarity[data["rarity"]],
            stack_size=data["stack_size"],
            is_tradable=data["is_tradable"],
            is_soulbound=data["is_soulbound"],
            stat_boosts=data.get("stat_boosts", {}),
            durability=data.get("durability", 100),
            is_legendary=data.get("is_legendary", False),
            nft_id=data.get("nft_id")
        )


@dataclass
class Blueprint(Item):
    """
    Recipes for crafting.
    
    These are found through exploration, quests, or by reaching higher Zoologist Levels.
    """
    recipe_id: str = ""  # The ID of the recipe this blueprint teaches
    required_level: int = 1  # The Zoologist level required to use this blueprint
    
    def __post_init__(self):
        """Initialize with default values."""
        super().__post_init__()
        if not hasattr(self, 'item_type') or self.item_type is None:
            self.item_type = ItemType.BLUEPRINT
        self.stack_size = 1  # Blueprints cannot stack
    
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Learn the blueprint.
        
        Args:
            user: The entity learning the blueprint.
            target: Not used.
            
        Returns:
            True if the blueprint was learned successfully, False otherwise.
        """
        # In a real implementation, this would add the recipe to the user's known recipes
        # For this prototype, we'll just return True
        return True
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data["recipe_id"] = self.recipe_id
        data["required_level"] = self.required_level
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Blueprint':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            item_type=ItemType[data["item_type"]],
            rarity=ItemRarity[data["rarity"]],
            stack_size=data["stack_size"],
            is_tradable=data["is_tradable"],
            is_soulbound=data["is_soulbound"],
            recipe_id=data.get("recipe_id", ""),
            required_level=data.get("required_level", 1)
        )


@dataclass
class QuestItem(Item):
    """
    Items for quests (soulbound).
    
    These are intrinsically tied to a player's personal journey.
    """
    quest_id: str = ""  # The ID of the quest this item is for
    
    def __post_init__(self):
        """Initialize with default values."""
        super().__post_init__()
        if not hasattr(self, 'item_type') or self.item_type is None:
            self.item_type = ItemType.QUEST_ITEM
        self.stack_size = 1  # Quest items cannot stack
        self.is_tradable = False  # Quest items cannot be traded
        self.is_soulbound = True  # Quest items are soulbound
    
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Use the quest item.
        
        Args:
            user: The entity using the quest item.
            target: The target of the quest item use, if any.
            
        Returns:
            True if the quest item was used successfully, False otherwise.
        """
        # In a real implementation, this would progress the quest
        # For this prototype, we'll just return True
        return True
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data["quest_id"] = self.quest_id
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'QuestItem':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            item_type=ItemType[data["item_type"]],
            rarity=ItemRarity[data["rarity"]],
            stack_size=data["stack_size"],
            is_tradable=data["is_tradable"],
            is_soulbound=data["is_soulbound"],
            quest_id=data.get("quest_id", "")
        )


@dataclass
class BridgingItem(Item):
    """
    Items that enable on-chain actions.
    
    These are rare, off-chain consumables whose sole purpose is to enable
    high-stakes, on-chain actions.
    """
    bridging_type: str = ""  # The type of on-chain action this item enables
    
    def __post_init__(self):
        """Initialize with default values."""
        super().__post_init__()
        if not hasattr(self, 'item_type') or self.item_type is None:
            self.item_type = ItemType.BRIDGING_ITEM
        self.stack_size = 1  # Bridging items cannot stack
    
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Use the bridging item.
        
        Args:
            user: The entity using the bridging item.
            target: The target of the bridging item use, if any.
            
        Returns:
            True if the bridging item was used successfully, False otherwise.
        """
        # In a real implementation, this would enable an on-chain action
        # For this prototype, we'll just return True
        return True
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data["bridging_type"] = self.bridging_type
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'BridgingItem':
        """Create from a dictionary."""
        bridging_type = data.get("bridging_type", "")
        
        # Create the appropriate bridging item type
        if bridging_type == "breeding_catalyst":
            return BreedingCatalyst.from_dict(data)
        elif bridging_type == "gene_splicer":
            return GeneSplicer.from_dict(data)
        elif bridging_type == "nft_minting_kit":
            return NFTMintingKit.from_dict(data)
        
        # Default to generic bridging item
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            item_type=ItemType[data["item_type"]],
            rarity=ItemRarity[data["rarity"]],
            stack_size=data["stack_size"],
            is_tradable=data["is_tradable"],
            is_soulbound=data["is_soulbound"],
            bridging_type=bridging_type
        )


@dataclass
class BreedingCatalyst(BridgingItem):
    """
    Catalysts for breeding.
    
    These are used to initiate the Echo-Synthesis process.
    """
    is_stable: bool = True  # Whether this is a stable catalyst (False = unstable)
    quality: int = 1  # The quality of the catalyst (1-5)
    
    def __post_init__(self):
        """Initialize with default values."""
        if not hasattr(self, 'bridging_type') or self.bridging_type is None:
            self.bridging_type = "breeding_catalyst"
        super().__post_init__()
    
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Use the breeding catalyst.
        
        Args:
            user: The entity using the catalyst.
            target: The breeding pair to use the catalyst on.
            
        Returns:
            True if the catalyst was used successfully, False otherwise.
        """
        # In a real implementation, this would initiate the breeding process
        # For this prototype, we'll just return True
        return True
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data["is_stable"] = self.is_stable
        data["quality"] = self.quality
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'BreedingCatalyst':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            item_type=ItemType[data["item_type"]],
            rarity=ItemRarity[data["rarity"]],
            stack_size=data["stack_size"],
            is_tradable=data["is_tradable"],
            is_soulbound=data["is_soulbound"],
            bridging_type=data.get("bridging_type", "breeding_catalyst"),
            is_stable=data.get("is_stable", True),
            quality=data.get("quality", 1)
        )


@dataclass
class GeneSplicer(BridgingItem):
    """
    Gene splicers for breeding.
    
    These are used to influence the outcome of the Echo-Synthesis process.
    """
    splicer_type: str = ""  # The type of gene splicer
    target_gene: str = ""  # The gene this splicer targets
    
    def __post_init__(self):
        """Initialize with default values."""
        if not hasattr(self, 'bridging_type') or self.bridging_type is None:
            self.bridging_type = "gene_splicer"
        super().__post_init__()
    
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Use the gene splicer.
        
        Args:
            user: The entity using the gene splicer.
            target: The breeding process to use the gene splicer on.
            
        Returns:
            True if the gene splicer was used successfully, False otherwise.
        """
        # In a real implementation, this would influence the breeding outcome
        # For this prototype, we'll just return True
        return True
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data["splicer_type"] = self.splicer_type
        data["target_gene"] = self.target_gene
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'GeneSplicer':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            item_type=ItemType[data["item_type"]],
            rarity=ItemRarity[data["rarity"]],
            stack_size=data["stack_size"],
            is_tradable=data["is_tradable"],
            is_soulbound=data["is_soulbound"],
            bridging_type=data.get("bridging_type", "gene_splicer"),
            splicer_type=data.get("splicer_type", ""),
            target_gene=data.get("target_gene", "")
        )


@dataclass
class NFTMintingKit(BridgingItem):
    """
    NFT minting kits.
    
    These are used to mint a piece of Master-crafted gear onto the Zoologist's Ledger as a Legendary NFT.
    """
    gear_type: str = ""  # The type of gear this kit can mint
    
    def __post_init__(self):
        """Initialize with default values."""
        if not hasattr(self, 'bridging_type') or self.bridging_type is None:
            self.bridging_type = "nft_minting_kit"
        super().__post_init__()
    
    def use(self, user: Any, target: Any = None) -> bool:
        """
        Use the NFT minting kit.
        
        Args:
            user: The entity using the minting kit.
            target: The gear to mint as an NFT.
            
        Returns:
            True if the minting kit was used successfully, False otherwise.
        """
        # In a real implementation, this would mint the gear as an NFT
        # For this prototype, we'll just return True
        return True
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data["gear_type"] = self.gear_type
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'NFTMintingKit':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            name=data["name"],
            description=data["description"],
            item_type=ItemType[data["item_type"]],
            rarity=ItemRarity[data["rarity"]],
            stack_size=data["stack_size"],
            is_tradable=data["is_tradable"],
            is_soulbound=data["is_soulbound"],
            bridging_type=data.get("bridging_type", "nft_minting_kit"),
            gear_type=data.get("gear_type", "")
        )