"""
Activities module for Critter-Craft.

This module implements the various activities and gameplay loops in Critter-Craft,
including mini-games, two-player games, jobs, and quests.
"""

import random
import time
import uuid
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, Union, Any

# Import from other modules as needed
import sys
import os

# Add the parent directory to the Python path to import from other pallets
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-economy', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-battles', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-ledger', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-breeding', 'src'))

# Import from economy system
from currencies import Bits, Aura
from items import Item, ItemType, ItemRarity, Material, Consumable
from inventory import Inventory

# Import from battle system
from battle.battle import Battle, BattleResult
from battle.environment import Environment, EnvironmentType

# Import from ledger system
from ledger import ZoologistLedger
from wallet import Wallet
from models import TransactionType, ZoologistLevel

# Import from breeding system
from genetics import GeneticCode, CoreGenes, PotentialGenes, CosmeticGenes


class ActivityType(Enum):
    """Types of activities in Critter-Craft."""
    MINI_GAME = auto()
    TWO_PLAYER_GAME = auto()
    JOB = auto()
    QUEST = auto()
    ADVENTUROUS_QUEST = auto()


class StatType(Enum):
    """Types of stats that can be trained through activities."""
    IQ = auto()
    CHARISMA = auto()
    ENERGY = auto()
    AGILITY = auto()
    STRENGTH = auto()
    SOCIAL = auto()


@dataclass
class ActivityReward:
    """
    A reward for completing an activity.
    
    This can include currency, items, stat experience, and reputation.
    """
    bits: int = 0
    aura: int = 0
    items: List[Tuple[Item, int]] = field(default_factory=list)  # List of (item, quantity) tuples
    stat_experience: Dict[StatType, int] = field(default_factory=dict)
    reputation: int = 0
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "bits": self.bits,
            "aura": self.aura,
            "items": [(item.to_dict(), quantity) for item, quantity in self.items],
            "stat_experience": {stat.name: exp for stat, exp in self.stat_experience.items()},
            "reputation": self.reputation
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'ActivityReward':
        """Create from a dictionary."""
        items = []
        for item_data, quantity in data.get("items", []):
            items.append((Item.from_dict(item_data), quantity))
        
        stat_experience = {}
        for stat_name, exp in data.get("stat_experience", {}).items():
            stat_experience[StatType[stat_name]] = exp
        
        return cls(
            bits=data.get("bits", 0),
            aura=data.get("aura", 0),
            items=items,
            stat_experience=stat_experience,
            reputation=data.get("reputation", 0)
        )


class Activity(ABC):
    """
    Base class for all activities in Critter-Craft.
    """
    
    def __init__(self, name: str, description: str, activity_type: ActivityType, required_level: int = 1):
        """
        Initialize an activity.
        
        Args:
            name: The name of the activity.
            description: The description of the activity.
            activity_type: The type of activity.
            required_level: The minimum player level required to participate.
        """
        self.id = str(uuid.uuid4())
        self.name = name
        self.description = description
        self.activity_type = activity_type
        self.required_level = required_level
    
    @abstractmethod
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the activity.
        
        Args:
            score: The player's score in the activity.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the activity.
        """
        pass
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "id": self.id,
            "name": self.name,
            "description": self.description,
            "activity_type": self.activity_type.name,
            "required_level": self.required_level
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Activity':
        """Create from a dictionary."""
        activity_type = ActivityType[data["activity_type"]]
        
        if activity_type == ActivityType.MINI_GAME:
            if "logic_leaper" in data["name"].lower():
                return LogicLeaper.from_dict(data)
            elif "aura_weaving" in data["name"].lower():
                return AuraWeaving.from_dict(data)
            elif "habitat_dash" in data["name"].lower():
                return HabitatDash.from_dict(data)
        elif activity_type == ActivityType.TWO_PLAYER_GAME:
            if "critter_tactics" in data["name"].lower():
                return CritterTactics.from_dict(data)
            elif "cooperative_crafting" in data["name"].lower():
                return CooperativeCrafting.from_dict(data)
        elif activity_type == ActivityType.JOB:
            if "crystal_mining" in data["name"].lower():
                return CrystalMining.from_dict(data)
            elif "bioluminescent_guide" in data["name"].lower():
                return BioluminescentGuide.from_dict(data)
            elif "herbalist" in data["name"].lower():
                return HerbalistAssistant.from_dict(data)
        elif activity_type == ActivityType.QUEST:
            return Quest.from_dict(data)
        elif activity_type == ActivityType.ADVENTUROUS_QUEST:
            return AdventurousQuest.from_dict(data)
        
        raise ValueError(f"Unknown activity type: {activity_type}")


class MiniGame(Activity):
    """
    Base class for all mini-games in Critter-Craft.
    
    Mini-games are short, engaging, skill-based activities designed to directly
    train specific pet stats and provide a steady stream of the soft currency, $BITS.
    """
    
    def __init__(self, name: str, description: str, primary_stat: StatType, secondary_stat: Optional[StatType] = None, required_level: int = 1):
        """
        Initialize a mini-game.
        
        Args:
            name: The name of the mini-game.
            description: The description of the mini-game.
            primary_stat: The primary stat trained by the mini-game.
            secondary_stat: The secondary stat trained by the mini-game.
            required_level: The minimum player level required to participate.
        """
        super().__init__(name, description, ActivityType.MINI_GAME, required_level)
        self.primary_stat = primary_stat
        self.secondary_stat = secondary_stat
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the mini-game.
        
        Args:
            score: The player's score in the mini-game.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the mini-game.
        """
        # Base rewards
        bits = score * 5  # 5 BITS per point
        
        # Stat experience
        stat_experience = {}
        stat_experience[self.primary_stat] = score * 10  # 10 XP per point for primary stat
        
        if self.secondary_stat:
            stat_experience[self.secondary_stat] = score * 5  # 5 XP per point for secondary stat
        
        # Reputation
        reputation = score // 10  # 1 reputation per 10 points
        
        return ActivityReward(
            bits=bits,
            stat_experience=stat_experience,
            reputation=reputation
        )
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data.update({
            "primary_stat": self.primary_stat.name,
            "secondary_stat": self.secondary_stat.name if self.secondary_stat else None
        })
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'MiniGame':
        """Create from a dictionary."""
        primary_stat = StatType[data["primary_stat"]]
        secondary_stat = StatType[data["secondary_stat"]] if data.get("secondary_stat") else None
        
        return cls(
            name=data["name"],
            description=data["description"],
            primary_stat=primary_stat,
            secondary_stat=secondary_stat,
            required_level=data["required_level"]
        )


class LogicLeaper(MiniGame):
    """
    Logic Leaper mini-game.
    
    A tile-based puzzle game where the player guides their critter across a grid to a goal.
    Some tiles are logic gates that must be activated in the correct sequence.
    It's a game of pathfinding and strategic planning.
    """
    
    def __init__(self, required_level: int = 1):
        """Initialize the Logic Leaper mini-game."""
        super().__init__(
            name="Logic Leaper",
            description="Guide your critter across a grid to a goal by activating logic gates in the correct sequence.",
            primary_stat=StatType.IQ,
            secondary_stat=StatType.ENERGY,
            required_level=required_level
        )
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the Logic Leaper mini-game.
        
        Args:
            score: The player's score in the mini-game.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the mini-game.
        """
        # Get base rewards
        rewards = super().get_rewards(score, player_level)
        
        # Add specific rewards for Logic Leaper
        if score >= 50:
            # Add a chance to find rare reagents
            if random.random() < 0.2:  # 20% chance
                focus_root = Material(
                    id="focus_root",
                    name="Focus Root",
                    description="A rare root that enhances mental focus.",
                    item_type=ItemType.MATERIAL,
                    rarity=ItemRarity.RARE,
                    source="Logic Leaper mini-game"
                )
                rewards.items.append((focus_root, 1))
        
        return rewards
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'LogicLeaper':
        """Create from a dictionary."""
        return cls(required_level=data["required_level"])


class AuraWeaving(MiniGame):
    """
    Aura Weaving mini-game.
    
    A rhythm and pattern-matching game. The player must synchronize with their pet's
    natural aura, mimicking a sequence of colors and pulses. It's a test of timing and memory.
    """
    
    def __init__(self, required_level: int = 1):
        """Initialize the Aura Weaving mini-game."""
        super().__init__(
            name="Aura Weaving",
            description="Synchronize with your pet's natural aura by mimicking a sequence of colors and pulses.",
            primary_stat=StatType.CHARISMA,
            secondary_stat=StatType.SOCIAL,
            required_level=required_level
        )
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the Aura Weaving mini-game.
        
        Args:
            score: The player's score in the mini-game.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the mini-game.
        """
        # Get base rewards
        rewards = super().get_rewards(score, player_level)
        
        # Add specific rewards for Aura Weaving
        if score >= 50:
            # Add a chance to find Aura Dust
            if random.random() < 0.2:  # 20% chance
                aura_dust = Material(
                    id="aura_dust",
                    name="Aura Dust",
                    description="A shimmering dust that resonates with a pet's aura.",
                    item_type=ItemType.MATERIAL,
                    rarity=ItemRarity.RARE,
                    source="Aura Weaving mini-game"
                )
                rewards.items.append((aura_dust, 1))
        
        return rewards
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'AuraWeaving':
        """Create from a dictionary."""
        return cls(required_level=data["required_level"])


class HabitatDash(MiniGame):
    """
    Habitat Dash mini-game.
    
    An "endless runner" style game through a procedurally generated habitat
    (forest, cavern, etc.). The player dodges obstacles and collects resources.
    """
    
    def __init__(self, required_level: int = 1):
        """Initialize the Habitat Dash mini-game."""
        super().__init__(
            name="Habitat Dash",
            description="Run through a procedurally generated habitat, dodging obstacles and collecting resources.",
            primary_stat=StatType.ENERGY,
            secondary_stat=StatType.AGILITY,
            required_level=required_level
        )
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the Habitat Dash mini-game.
        
        Args:
            score: The player's score in the mini-game.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the mini-game.
        """
        # Get base rewards
        rewards = super().get_rewards(score, player_level)
        
        # Add specific rewards for Habitat Dash
        if score >= 50:
            # Add common environmental reagents
            sunpetal = Material(
                id="sunpetal",
                name="Sunpetal",
                description="A bright yellow flower that grows in sunny areas.",
                item_type=ItemType.MATERIAL,
                rarity=ItemRarity.COMMON,
                source="Habitat Dash mini-game"
            )
            rewards.items.append((sunpetal, random.randint(1, 3)))
            
            # Add a chance to find cave mushrooms
            if random.random() < 0.2:  # 20% chance
                cave_mushroom = Material(
                    id="cave_mushroom",
                    name="Cave Mushroom",
                    description="A luminescent mushroom that grows in dark caves.",
                    item_type=ItemType.MATERIAL,
                    rarity=ItemRarity.UNCOMMON,
                    source="Habitat Dash mini-game"
                )
                rewards.items.append((cave_mushroom, 1))
        
        return rewards
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'HabitatDash':
        """Create from a dictionary."""
        return cls(required_level=data["required_level"])


class TwoPlayerGame(Activity):
    """
    Base class for all two-player games in Critter-Craft.
    
    Two-player games are designed to foster social interaction and high-level
    strategic competition.
    """
    
    def __init__(self, name: str, description: str, primary_stat: StatType, secondary_stat: Optional[StatType] = None, required_level: int = 1):
        """
        Initialize a two-player game.
        
        Args:
            name: The name of the two-player game.
            description: The description of the two-player game.
            primary_stat: The primary stat trained by the two-player game.
            secondary_stat: The secondary stat trained by the two-player game.
            required_level: The minimum player level required to participate.
        """
        super().__init__(name, description, ActivityType.TWO_PLAYER_GAME, required_level)
        self.primary_stat = primary_stat
        self.secondary_stat = secondary_stat
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the two-player game.
        
        Args:
            score: The player's score in the two-player game.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the two-player game.
        """
        # Base rewards
        bits = score * 10  # 10 BITS per point
        
        # Stat experience
        stat_experience = {}
        stat_experience[self.primary_stat] = score * 15  # 15 XP per point for primary stat
        
        if self.secondary_stat:
            stat_experience[self.secondary_stat] = score * 10  # 10 XP per point for secondary stat
        
        # Reputation
        reputation = score // 5  # 1 reputation per 5 points
        
        return ActivityReward(
            bits=bits,
            stat_experience=stat_experience,
            reputation=reputation
        )
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data.update({
            "primary_stat": self.primary_stat.name,
            "secondary_stat": self.secondary_stat.name if self.secondary_stat else None
        })
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'TwoPlayerGame':
        """Create from a dictionary."""
        primary_stat = StatType[data["primary_stat"]]
        secondary_stat = StatType[data["secondary_stat"]] if data.get("secondary_stat") else None
        
        return cls(
            name=data["name"],
            description=data["description"],
            primary_stat=primary_stat,
            secondary_stat=secondary_stat,
            required_level=data["required_level"]
        )


class CritterTactics(TwoPlayerGame):
    """
    Critter Tactics two-player game.
    
    A turn-based tactical board game, our universe's answer to chess. Each player
    brings a team of three critters to a gridded battlefield.
    """
    
    def __init__(self, required_level: int = 5):
        """Initialize the Critter Tactics two-player game."""
        super().__init__(
            name="Critter Tactics",
            description="A turn-based tactical board game where each player brings a team of three critters to a gridded battlefield.",
            primary_stat=StatType.IQ,
            secondary_stat=StatType.SOCIAL,
            required_level=required_level
        )
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the Critter Tactics two-player game.
        
        Args:
            score: The player's score in the two-player game.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the two-player game.
        """
        # Get base rewards
        rewards = super().get_rewards(score, player_level)
        
        # Add specific rewards for Critter Tactics
        if score >= 100:
            # Add AURA for high scores
            rewards.aura = score // 100  # 1 AURA per 100 points
            
            # Add a chance for exclusive cosmetic rewards
            if random.random() < 0.1:  # 10% chance
                tactics_badge = Item(
                    id="tactics_badge",
                    name="Tactics Badge",
                    description="A badge awarded to skilled Critter Tactics players.",
                    item_type=ItemType.COSMETIC,
                    rarity=ItemRarity.RARE
                )
                rewards.items.append((tactics_badge, 1))
        
        return rewards
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'CritterTactics':
        """Create from a dictionary."""
        return cls(required_level=data["required_level"])


class CooperativeCrafting(TwoPlayerGame):
    """
    Cooperative Crafting two-player game.
    
    Two players bring their unique materials to a "Synergy Workbench." They must
    complete a synchronized mini-game to craft powerful items that neither could
    create alone.
    """
    
    def __init__(self, required_level: int = 3):
        """Initialize the Cooperative Crafting two-player game."""
        super().__init__(
            name="Cooperative Crafting",
            description="Work with another player to craft powerful items that neither could create alone.",
            primary_stat=StatType.SOCIAL,
            secondary_stat=StatType.CHARISMA,
            required_level=required_level
        )
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the Cooperative Crafting two-player game.
        
        Args:
            score: The player's score in the two-player game.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the two-player game.
        """
        # Get base rewards
        rewards = super().get_rewards(score, player_level)
        
        # Add specific rewards for Cooperative Crafting
        if score >= 50:
            # Add unique co-op gear
            synergy_charm = Item(
                id="synergy_charm",
                name="Synergy Charm",
                description="A charm that enhances the bond between a player and their pet.",
                item_type=ItemType.GEAR,
                rarity=ItemRarity.UNCOMMON
            )
            rewards.items.append((synergy_charm, 1))
            
            # Add a significant boost to the Social stat
            rewards.stat_experience[StatType.SOCIAL] += 50
        
        return rewards
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'CooperativeCrafting':
        """Create from a dictionary."""
        return cls(required_level=data["required_level"])


class Job(Activity):
    """
    Base class for all jobs in Critter-Craft.
    
    Jobs are the primary, reliable method for earning $BITS and are directly
    tied to a pet's trained stats.
    """
    
    def __init__(self, name: str, description: str, primary_stat: StatType, secondary_stat: Optional[StatType] = None, required_level: int = 1, duration: int = 3600):
        """
        Initialize a job.
        
        Args:
            name: The name of the job.
            description: The description of the job.
            primary_stat: The primary stat required for the job.
            secondary_stat: The secondary stat required for the job.
            required_level: The minimum player level required to participate.
            duration: The duration of the job in seconds.
        """
        super().__init__(name, description, ActivityType.JOB, required_level)
        self.primary_stat = primary_stat
        self.secondary_stat = secondary_stat
        self.duration = duration
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the job.
        
        Args:
            score: The player's score in the job.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the job.
        """
        # Base rewards
        bits = score * 20  # 20 BITS per point
        
        # Stat experience
        stat_experience = {}
        stat_experience[self.primary_stat] = score * 5  # 5 XP per point for primary stat
        
        if self.secondary_stat:
            stat_experience[self.secondary_stat] = score * 3  # 3 XP per point for secondary stat
        
        # Reputation
        reputation = score // 20  # 1 reputation per 20 points
        
        return ActivityReward(
            bits=bits,
            stat_experience=stat_experience,
            reputation=reputation
        )
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data.update({
            "primary_stat": self.primary_stat.name,
            "secondary_stat": self.secondary_stat.name if self.secondary_stat else None,
            "duration": self.duration
        })
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Job':
        """Create from a dictionary."""
        primary_stat = StatType[data["primary_stat"]]
        secondary_stat = StatType[data["secondary_stat"]] if data.get("secondary_stat") else None
        
        return cls(
            name=data["name"],
            description=data["description"],
            primary_stat=primary_stat,
            secondary_stat=secondary_stat,
            required_level=data["required_level"],
            duration=data["duration"]
        )


class CrystalMining(Job):
    """
    Crystal Mining job.
    
    Requires a pet with high Strength. A timing-based mini-game where stronger
    pets can break harder crystals for rarer gems.
    """
    
    def __init__(self, required_level: int = 2, duration: int = 3600):
        """Initialize the Crystal Mining job."""
        super().__init__(
            name="Crystal Mining",
            description="Mine crystals in the deep caves. Stronger pets can break harder crystals for rarer gems.",
            primary_stat=StatType.STRENGTH,
            secondary_stat=StatType.ENERGY,
            required_level=required_level,
            duration=duration
        )
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the Crystal Mining job.
        
        Args:
            score: The player's score in the job.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the job.
        """
        # Get base rewards
        rewards = super().get_rewards(score, player_level)
        
        # Add specific rewards for Crystal Mining
        if score >= 50:
            # Add crystals based on score
            crystal_shard = Material(
                id="crystal_shard",
                name="Crystal Shard",
                description="A fragment of a rare crystal found in deep caves.",
                item_type=ItemType.MATERIAL,
                rarity=ItemRarity.RARE,
                source="Crystal Mining job"
            )
            rewards.items.append((crystal_shard, score // 50))
        
        return rewards
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'CrystalMining':
        """Create from a dictionary."""
        return cls(
            required_level=data["required_level"],
            duration=data["duration"]
        )


class BioluminescentGuide(Job):
    """
    Bioluminescent Guide job.
    
    A passive job. You dispatch a high-Charisma pet (like a sprite_glow) for a
    set duration to guide travelers. Higher charisma reduces the time and increases
    the chance of a bonus "tip."
    """
    
    def __init__(self, required_level: int = 2, duration: int = 7200):
        """Initialize the Bioluminescent Guide job."""
        super().__init__(
            name="Bioluminescent Guide",
            description="Guide travelers through dark areas. Higher charisma reduces the time and increases the chance of a bonus tip.",
            primary_stat=StatType.CHARISMA,
            secondary_stat=StatType.SOCIAL,
            required_level=required_level,
            duration=duration
        )
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the Bioluminescent Guide job.
        
        Args:
            score: The player's score in the job.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the job.
        """
        # Get base rewards
        rewards = super().get_rewards(score, player_level)
        
        # Add specific rewards for Bioluminescent Guide
        if score >= 50:
            # Add a chance for a bonus "tip"
            if random.random() < score / 100:  # Higher score = higher chance
                rewards.bits += 50  # 50 BITS bonus
                
                # Add a chance for a rare item as a gift
                if random.random() < 0.1:  # 10% chance
                    traveler_token = Item(
                        id="traveler_token",
                        name="Traveler's Token",
                        description="A token of appreciation from a grateful traveler.",
                        item_type=ItemType.QUEST_ITEM,
                        rarity=ItemRarity.UNCOMMON
                    )
                    rewards.items.append((traveler_token, 1))
        
        return rewards
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'BioluminescentGuide':
        """Create from a dictionary."""
        return cls(
            required_level=data["required_level"],
            duration=data["duration"]
        )


class HerbalistAssistant(Job):
    """
    Herbalist's Assistant job.
    
    Requires a high-IQ pet to correctly identify and sort rare herbs for an
    apothecary. A puzzle/memory mini-game.
    """
    
    def __init__(self, required_level: int = 2, duration: int = 3600):
        """Initialize the Herbalist's Assistant job."""
        super().__init__(
            name="Herbalist's Assistant",
            description="Identify and sort rare herbs for an apothecary. Higher IQ allows for more accurate identification.",
            primary_stat=StatType.IQ,
            secondary_stat=None,
            required_level=required_level,
            duration=duration
        )
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the Herbalist's Assistant job.
        
        Args:
            score: The player's score in the job.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the job.
        """
        # Get base rewards
        rewards = super().get_rewards(score, player_level)
        
        # Add specific rewards for Herbalist's Assistant
        if score >= 50:
            # Add herbs based on score
            healing_herb = Material(
                id="healing_herb",
                name="Healing Herb",
                description="A herb with medicinal properties.",
                item_type=ItemType.MATERIAL,
                rarity=ItemRarity.UNCOMMON,
                source="Herbalist's Assistant job"
            )
            rewards.items.append((healing_herb, score // 25))
        
        return rewards
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'HerbalistAssistant':
        """Create from a dictionary."""
        return cls(
            required_level=data["required_level"],
            duration=data["duration"]
        )


class Quest(Activity):
    """
    Base class for all quests in Critter-Craft.
    
    Quests are the backbone of the player's journey, driving them to explore and
    interact with the world.
    """
    
    def __init__(self, name: str, description: str, objectives: List[str], required_level: int = 1):
        """
        Initialize a quest.
        
        Args:
            name: The name of the quest.
            description: The description of the quest.
            objectives: The objectives of the quest.
            required_level: The minimum player level required to participate.
        """
        super().__init__(name, description, ActivityType.QUEST, required_level)
        self.objectives = objectives
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the quest.
        
        Args:
            score: The player's score in the quest.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the quest.
        """
        # Base rewards
        bits = 100 + (player_level * 10)  # 100 BITS + 10 per player level
        
        # Stat experience
        stat_experience = {}
        for stat in StatType:
            stat_experience[stat] = 10  # 10 XP for all stats
        
        # Reputation
        reputation = 5  # 5 reputation
        
        return ActivityReward(
            bits=bits,
            stat_experience=stat_experience,
            reputation=reputation
        )
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data.update({
            "objectives": self.objectives
        })
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Quest':
        """Create from a dictionary."""
        return cls(
            name=data["name"],
            description=data["description"],
            objectives=data["objectives"],
            required_level=data["required_level"]
        )


class AdventurousQuest(Quest):
    """
    Adventurous Quest.
    
    These are long, challenging questlines that tell the deeper story of the
    Critter-Craft world and offer the greatest rewards.
    """
    
    def __init__(self, name: str, description: str, objectives: List[str], stages: List[Dict], required_level: int = 10):
        """
        Initialize an adventurous quest.
        
        Args:
            name: The name of the adventurous quest.
            description: The description of the adventurous quest.
            objectives: The objectives of the adventurous quest.
            stages: The stages of the adventurous quest.
            required_level: The minimum player level required to participate.
        """
        super().__init__(name, description, objectives, required_level)
        self.activity_type = ActivityType.ADVENTUROUS_QUEST
        self.stages = stages
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the adventurous quest.
        
        Args:
            score: The player's score in the adventurous quest.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the adventurous quest.
        """
        # Get base rewards
        rewards = super().get_rewards(score, player_level)
        
        # Add specific rewards for Adventurous Quests
        rewards.bits *= 2  # Double the BITS
        rewards.aura = 10  # 10 AURA
        
        # Add a unique Legendary Gear NFT
        legendary_gear = Item(
            id=f"legendary_gear_{self.id}",
            name=f"Legendary {self.name} Gear",
            description=f"A legendary gear item awarded for completing the {self.name} quest.",
            item_type=ItemType.GEAR,
            rarity=ItemRarity.LEGENDARY
        )
        rewards.items.append((legendary_gear, 1))
        
        # Add a permanent Achievement
        rewards.reputation += 50  # 50 additional reputation
        
        return rewards
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        data = super().to_dict()
        data.update({
            "stages": self.stages
        })
        return data
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'AdventurousQuest':
        """Create from a dictionary."""
        return cls(
            name=data["name"],
            description=data["description"],
            objectives=data["objectives"],
            stages=data["stages"],
            required_level=data["required_level"]
        )


class ActivityManager:
    """
    Manager for all activities in Critter-Craft.
    
    This class is responsible for managing all activities, including mini-games,
    two-player games, jobs, and quests.
    """
    
    def __init__(self):
        """Initialize the activity manager."""
        self.activities: Dict[str, Activity] = {}
        self.completed_activities: Dict[str, Dict[str, List[str]]] = {}  # player_id -> activity_type -> [activity_id]
    
    def add_activity(self, activity: Activity) -> None:
        """
        Add an activity to the manager.
        
        Args:
            activity: The activity to add.
        """
        self.activities[activity.id] = activity
    
    def get_activity(self, activity_id: str) -> Optional[Activity]:
        """
        Get an activity by ID.
        
        Args:
            activity_id: The ID of the activity to get.
            
        Returns:
            The activity, or None if not found.
        """
        return self.activities.get(activity_id)
    
    def get_activities_by_type(self, activity_type: ActivityType) -> List[Activity]:
        """
        Get all activities of a specific type.
        
        Args:
            activity_type: The type of activities to get.
            
        Returns:
            A list of activities of the specified type.
        """
        return [
            activity for activity in self.activities.values()
            if activity.activity_type == activity_type
        ]
    
    def get_available_activities(self, player_id: str, player_level: int) -> List[Activity]:
        """
        Get all activities available to a player.
        
        Args:
            player_id: The ID of the player.
            player_level: The level of the player.
            
        Returns:
            A list of activities available to the player.
        """
        return [
            activity for activity in self.activities.values()
            if activity.required_level <= player_level
        ]
    
    def complete_activity(self, player_id: str, activity_id: str, score: int, player_level: int) -> ActivityReward:
        """
        Complete an activity.
        
        Args:
            player_id: The ID of the player completing the activity.
            activity_id: The ID of the activity to complete.
            score: The player's score in the activity.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the activity.
        """
        activity = self.get_activity(activity_id)
        
        if not activity:
            raise ValueError(f"Activity not found: {activity_id}")
        
        # Get the rewards
        rewards = activity.get_rewards(score, player_level)
        
        # Record the completion
        if player_id not in self.completed_activities:
            self.completed_activities[player_id] = {}
        
        if activity.activity_type.name not in self.completed_activities[player_id]:
            self.completed_activities[player_id][activity.activity_type.name] = []
        
        self.completed_activities[player_id][activity.activity_type.name].append(activity_id)
        
        return rewards
    
    def has_completed_activity(self, player_id: str, activity_id: str) -> bool:
        """
        Check if a player has completed an activity.
        
        Args:
            player_id: The ID of the player.
            activity_id: The ID of the activity.
            
        Returns:
            True if the player has completed the activity, False otherwise.
        """
        if player_id not in self.completed_activities:
            return False
        
        activity = self.get_activity(activity_id)
        
        if not activity:
            return False
        
        if activity.activity_type.name not in self.completed_activities[player_id]:
            return False
        
        return activity_id in self.completed_activities[player_id][activity.activity_type.name]
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "activities": {activity_id: activity.to_dict() for activity_id, activity in self.activities.items()},
            "completed_activities": self.completed_activities
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'ActivityManager':
        """Create from a dictionary."""
        manager = cls()
        
        for activity_id, activity_data in data.get("activities", {}).items():
            activity = Activity.from_dict(activity_data)
            manager.activities[activity_id] = activity
        
        manager.completed_activities = data.get("completed_activities", {})
        
        return manager


# Create some example activities
def create_example_activities() -> ActivityManager:
    """
    Create some example activities.
    
    Returns:
        An ActivityManager with example activities.
    """
    manager = ActivityManager()
    
    # Add mini-games
    manager.add_activity(LogicLeaper())
    manager.add_activity(AuraWeaving())
    manager.add_activity(HabitatDash())
    
    # Add two-player games
    manager.add_activity(CritterTactics())
    manager.add_activity(CooperativeCrafting())
    
    # Add jobs
    manager.add_activity(CrystalMining())
    manager.add_activity(BioluminescentGuide())
    manager.add_activity(HerbalistAssistant())
    
    # Add quests
    manager.add_activity(Quest(
        name="Gathering Quest",
        description="Collect 10 Sunpetal Pollens.",
        objectives=["Collect 10 Sunpetal Pollens"],
        required_level=1
    ))
    
    manager.add_activity(Quest(
        name="Crafting Quest",
        description="Craft 3 Healing Salves for the outpost.",
        objectives=["Craft 3 Healing Salves"],
        required_level=2
    ))
    
    manager.add_activity(Quest(
        name="Pacification Quest",
        description="A territorial Alpha Glimmer-Moth is causing trouble. Pacify it in a battle.",
        objectives=["Pacify the Alpha Glimmer-Moth"],
        required_level=3
    ))
    
    # Add an adventurous quest
    manager.add_activity(AdventurousQuest(
        name="The Whispering Blight",
        description="A renowned Zoologist reports that critters in a remote jungle are falling into a strange torpor.",
        objectives=[
            "Travel to the Verdant Maw habitat",
            "Investigate the strange torpor",
            "Collect samples of the parasitic fungus",
            "Take the samples to a Master Crafter",
            "Obtain the Heart of a Magma-Slug",
            "Gather untainted Starlight Algae",
            "Obtain the Blueprint: Blight-Ward Charm",
            "Craft the Blight-Ward Charm",
            "Enter the heart of the jungle",
            "Defeat the source of the blight"
        ],
        stages=[
            {
                "name": "The Mystery",
                "description": "A renowned Zoologist reports that critters in a remote jungle are falling into a strange torpor. Players must travel to the 'Verdant Maw' habitat and use pets with high IQ to investigate, finding traces of a strange, parasitic fungus.",
                "objectives": [
                    "Travel to the Verdant Maw habitat",
                    "Investigate the strange torpor",
                    "Collect samples of the parasitic fungus"
                ]
            },
            {
                "name": "The Research",
                "description": "The samples must be taken to a Master Crafter, who determines a cure is possible, but requires three rare, powerful components: the Heart of a Magma-Slug (found only in the Geothermal Vents after a boss battle), untainted Starlight Algae (gathered via a complex Aura Weaving challenge), and a Blueprint: Blight-Ward Charm held by a rival Zoologist who will only relinquish it after being defeated in a high-stakes Critter Tactics match.",
                "objectives": [
                    "Take the samples to a Master Crafter",
                    "Obtain the Heart of a Magma-Slug",
                    "Gather untainted Starlight Algae",
                    "Obtain the Blueprint: Blight-Ward Charm",
                    "Craft the Blight-Ward Charm"
                ]
            },
            {
                "name": "The Culmination",
                "description": "With the Blight-Ward Charm crafted and equipped, players can enter the heart of the jungle, where they face the source of the blight: a colossal, ancient creature covered in the parasitic fungus. This is a multi-phase boss battle that requires using the charm at key moments to weaken the boss.",
                "objectives": [
                    "Enter the heart of the jungle",
                    "Defeat the source of the blight"
                ]
            }
        ],
        required_level=10
    ))
    
    return manager


if __name__ == "__main__":
    # Create example activities
    manager = create_example_activities()
    
    # Print all activities
    print("All Activities:")
    for activity in manager.activities.values():
        print(f"- {activity.name} ({activity.activity_type.name})")
    
    # Simulate completing an activity
    player_id = "player1"
    player_level = 10
    
    activity_id = list(manager.activities.keys())[0]
    activity = manager.get_activity(activity_id)
    
    print(f"\nCompleting activity: {activity.name}")
    rewards = manager.complete_activity(player_id, activity_id, 100, player_level)
    
    print("Rewards:")
    print(f"- BITS: {rewards.bits}")
    print(f"- AURA: {rewards.aura}")
    print(f"- Stat Experience: {', '.join([f'{stat.name}: {exp}' for stat, exp in rewards.stat_experience.items()])}")
    print(f"- Reputation: {rewards.reputation}")
    print(f"- Items: {', '.join([f'{item.name} x{quantity}' for item, quantity in rewards.items])}")
    
    # Check if the player has completed the activity
    print(f"\nHas completed {activity.name}? {manager.has_completed_activity(player_id, activity_id)}")