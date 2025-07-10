"""
Activities system for Critter-Craft.

This module implements the business logic for the various activities and gameplay loops
in Critter-Craft, including mini-games, two-player games, jobs, and quests.

The system follows the Expanded KISS Principle:
- Keep it Simple: Core activity types are clearly defined and separated
- Intelligent: Activities are data-driven and configurable
- Systematized: Activities follow a consistent pattern and interface
- Secure: Rewards are calculated based on well-defined rules
"""

import random
import time
from typing import Dict, List, Optional, Tuple, Any, Union, TypeVar
from functools import lru_cache
import sys
import os
from pathlib import Path

# Add the parent directory to the Python path to import from other pallets
# Using pathlib for more robust path handling
PARENT_DIR = Path(__file__).parent.parent.parent
ECONOMY_PATH = PARENT_DIR / "pallet-economy" / "src"
OWN_SRC_DIR = Path(__file__).parent # This is BlockChain/pallets/pallet-quests/src/
sys.path.insert(0, str(ECONOMY_PATH))
if str(OWN_SRC_DIR) not in sys.path:
    sys.path.insert(0, str(OWN_SRC_DIR))


# Import from economy system
from currencies import Bits, Aura
from items import Item, ItemType, ItemRarity, Material, Consumable
from inventory import Inventory

# Import from activities module
from activities import ActivityType, StatType # Direct import for sibling module

# Type variables for better type hinting
T = TypeVar('T')


class ActivityReward:
    """
    A reward for completing an activity.
    
    This can include currency, items, stat experience, and reputation.
    
    Attributes:
        bits: The amount of BITS to reward.
        aura: The amount of AURA to reward.
        items: A list of (item, quantity) tuples to reward.
        stat_experience: A dictionary mapping stat types to experience points.
        reputation: The amount of reputation to reward.
    """
    
    __slots__ = ('bits', 'aura', 'items', 'stat_experience', 'reputation')
    
    def __init__(self, bits: int = 0, aura: int = 0, items: Optional[List[Tuple[Item, int]]] = None, 
                 stat_experience: Optional[Dict[StatType, int]] = None, reputation: int = 0):
        """
        Initialize an activity reward.
        
        Args:
            bits: The amount of BITS to reward.
            aura: The amount of AURA to reward.
            items: A list of (item, quantity) tuples to reward.
            stat_experience: A dictionary mapping stat types to experience points.
            reputation: The amount of reputation to reward.
        """
        self.bits = bits
        self.aura = aura
        self.items = items or []
        self.stat_experience = stat_experience or {}
        self.reputation = reputation
    
    def __repr__(self) -> str:
        """Return a string representation of the reward."""
        return (f"ActivityReward(bits={self.bits}, aura={self.aura}, "
                f"items={len(self.items)}, stat_exp={len(self.stat_experience)}, "
                f"reputation={self.reputation})")
    
    def combine(self, other: 'ActivityReward') -> 'ActivityReward':
        """
        Combine this reward with another reward.
        
        Args:
            other: The other reward to combine with.
            
        Returns:
            A new ActivityReward that is the combination of this reward and the other reward.
        """
        combined_items = list(self.items)
        
        # Combine items by adding quantities for the same item
        for other_item, other_quantity in other.items:
            for i, (item, quantity) in enumerate(combined_items):
                if item.id == other_item.id:
                    combined_items[i] = (item, quantity + other_quantity)
                    break
            else:
                combined_items.append((other_item, other_quantity))
        
        # Combine stat experience
        combined_stat_exp = dict(self.stat_experience)
        for stat, exp in other.stat_experience.items():
            combined_stat_exp[stat] = combined_stat_exp.get(stat, 0) + exp
        
        return ActivityReward(
            bits=self.bits + other.bits,
            aura=self.aura + other.aura,
            items=combined_items,
            stat_experience=combined_stat_exp,
            reputation=self.reputation + other.reputation
        )


class Activity:
    """
    Base class for all activities in Critter-Craft.
    
    This abstract base class defines the interface for all activities in the game.
    Subclasses should implement the get_rewards method to provide specific reward logic.
    
    Attributes:
        id: The unique identifier for the activity.
        name: The name of the activity.
        description: The description of the activity.
        activity_type: The type of activity.
        required_level: The minimum player level required to participate.
        reward_config: Configuration for rewards.
    """
    
    __slots__ = ('id', 'name', 'description', 'activity_type', 'required_level', 'reward_config')
    
    def __init__(self, activity_id: str, name: str, description: str, activity_type: ActivityType, 
                 required_level: int = 1, reward_config: Optional[Dict[str, Any]] = None):
        """
        Initialize an activity.
        
        Args:
            activity_id: The unique identifier for the activity.
            name: The name of the activity.
            description: The description of the activity.
            activity_type: The type of activity.
            required_level: The minimum player level required to participate.
            reward_config: Configuration for rewards.
        """
        self.id = activity_id
        self.name = name
        self.description = description
        self.activity_type = activity_type
        self.required_level = required_level
        self.reward_config = reward_config or {}
    
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the activity.
        
        Args:
            score: The player's score in the activity.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the activity.
        """
        # Base implementation - should be overridden by subclasses
        return ActivityReward()
    
    def __repr__(self) -> str:
        """Return a string representation of the activity."""
        return f"{self.__class__.__name__}(id={self.id}, name={self.name}, type={self.activity_type.name})"
    
    def is_available_for_player(self, player_level: int) -> bool:
        """
        Check if the activity is available for a player of the given level.
        
        Args:
            player_level: The level of the player.
            
        Returns:
            True if the activity is available, False otherwise.
        """
        return player_level >= self.required_level


class MiniGame(Activity):
    """
    A mini-game activity.
    
    Mini-games are short, engaging, skill-based activities designed to directly
    train specific pet stats and provide a steady stream of the soft currency, $BITS.
    
    Attributes:
        stats_trained: The stats trained by the mini-game.
    """
    
    __slots__ = ('stats_trained',)
    
    def __init__(self, activity_id: str, name: str, description: str, required_level: int,
                 stats_trained: List[StatType], reward_config: Optional[Dict[str, Any]] = None):
        """
        Initialize a mini-game.
        
        Args:
            activity_id: The unique identifier for the mini-game.
            name: The name of the mini-game.
            description: The description of the mini-game.
            required_level: The minimum player level required to participate.
            stats_trained: The stats trained by the mini-game.
            reward_config: Configuration for rewards.
        """
        super().__init__(activity_id, name, description, ActivityType.MINI_GAME, required_level, reward_config)
        self.stats_trained = stats_trained
    
    @lru_cache(maxsize=32)
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the mini-game.
        
        This method is cached to improve performance for repeated calls with the same parameters.
        
        Args:
            score: The player's score in the mini-game.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the mini-game.
        """
        # Calculate BITS reward
        bits_per_point = self.reward_config.get("base_bits_per_point", 5)
        bits = score * bits_per_point
        
        # Apply level multiplier (higher level players get slightly more rewards)
        level_multiplier = 1.0 + (player_level * 0.01)  # 1% increase per level
        bits = int(bits * level_multiplier)
        
        # Calculate stat experience
        stat_experience = {}
        primary_exp_per_point = self.reward_config.get("primary_stat_exp_per_point", 10)
        secondary_exp_per_point = self.reward_config.get("secondary_stat_exp_per_point", 5)
        
        if self.stats_trained:
            # Primary stat (first in the list)
            stat_experience[self.stats_trained[0]] = int(score * primary_exp_per_point * level_multiplier)
            
            # Secondary stats (rest of the list)
            for stat in self.stats_trained[1:]:
                stat_experience[stat] = int(score * secondary_exp_per_point * level_multiplier)
        
        # Calculate reputation
        reputation_per_100_points = self.reward_config.get("reputation_per_100_points", 10)
        reputation = (score // 100) * reputation_per_100_points
        
        # Apply level multiplier to reputation
        reputation = int(reputation * level_multiplier)
        
        # Check for special item drops
        items = []
        
        # Adjust drop chance based on player level (higher level = slightly better chance)
        level_drop_bonus = min(0.1, player_level * 0.005)  # Max 10% bonus at level 20
        
        for item_drop in self.reward_config.get("special_item_drops", []):
            min_score = item_drop.get("min_score", 0)
            base_chance = item_drop.get("chance", 0)
            
            # Only process if the score meets the minimum requirement
            if score >= min_score:
                # Adjust chance based on how much the score exceeds the minimum
                score_bonus = min(0.1, (score - min_score) / 1000)  # Max 10% bonus for scores 1000+ above min
                adjusted_chance = base_chance + level_drop_bonus + score_bonus
                
                if random.random() < adjusted_chance:
                    item_id = item_drop.get("item_id")
                    # In a real implementation, we would look up the item from a database
                    # For now, we'll create a placeholder item
                    item = Material(
                        id=item_id,
                        name=item_id.replace("_", " ").title(),
                        description=f"A special item from the {self.name} mini-game.",
                        item_type=ItemType.MATERIAL,
                        rarity=ItemRarity.RARE,
                        source=f"{self.name} mini-game"
                    )
                    items.append((item, 1))
        
        return ActivityReward(
            bits=bits,
            stat_experience=stat_experience,
            reputation=reputation,
            items=items
        )


class Job(Activity):
    """
    A job activity.
    
    Jobs are the primary, reliable method for earning $BITS and are directly
    tied to a pet's trained stats.
    
    Attributes:
        primary_stat: The primary stat required for the job.
        duration_seconds: The duration of the job in seconds.
    """
    
    __slots__ = ('primary_stat', 'duration_seconds')
    
    def __init__(self, activity_id: str, name: str, description: str, required_level: int,
                 primary_stat: StatType, duration_seconds: int, reward_config: Optional[Dict[str, Any]] = None):
        """
        Initialize a job.
        
        Args:
            activity_id: The unique identifier for the job.
            name: The name of the job.
            description: The description of the job.
            required_level: The minimum player level required to participate.
            primary_stat: The primary stat required for the job.
            duration_seconds: The duration of the job in seconds.
            reward_config: Configuration for rewards.
        """
        super().__init__(activity_id, name, description, ActivityType.JOB, required_level, reward_config)
        self.primary_stat = primary_stat
        self.duration_seconds = duration_seconds
    
    @lru_cache(maxsize=32)
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the job.
        
        This method is cached to improve performance for repeated calls with the same parameters.
        
        Args:
            score: The player's score in the job (0-100 representing percentage of success).
            player_level: The player's level.
            
        Returns:
            The rewards for completing the job.
        """
        # Ensure score is within valid range
        score = max(0, min(100, score))
        
        # Apply level multiplier (higher level players get slightly more rewards)
        level_multiplier = 1.0 + (player_level * 0.01)  # 1% increase per level
        
        # Calculate BITS reward
        bits_per_hour = self.reward_config.get("base_bits_per_hour", 500)
        hours = self.duration_seconds / 3600
        base_bits = bits_per_hour * hours * (score / 100)
        bits = int(base_bits * level_multiplier)
        
        # Calculate stat experience
        stat_experience = {}
        primary_exp_per_hour = self.reward_config.get("primary_stat_exp_per_hour", 100)
        base_exp = primary_exp_per_hour * hours * (score / 100)
        stat_experience[self.primary_stat] = int(base_exp * level_multiplier)
        
        # Calculate reputation
        reputation = int(bits / 100)  # 1 reputation per 100 BITS
        
        # Chance for bonus items based on job performance
        items = []
        if score > 80 and random.random() < 0.2 + (player_level * 0.01):  # 20% base chance + 1% per level
            # In a real implementation, we would look up the item from a database
            # For now, we'll create a placeholder item based on the job
            item_id = f"{self.id}_bonus"
            item = Material(
                id=item_id,
                name=f"{self.name} Bonus Material",
                description=f"A special material found while performing the {self.name} job.",
                item_type=ItemType.MATERIAL,
                rarity=ItemRarity.UNCOMMON,
                source=f"{self.name} job"
            )
            items.append((item, 1))
        
        return ActivityReward(
            bits=bits,
            stat_experience=stat_experience,
            reputation=reputation,
            items=items
        )


class Quest(Activity):
    """
    A quest activity.
    
    Quests are the backbone of the player's journey, driving them to explore and
    interact with the world.
    
    Attributes:
        objectives: The objectives of the quest.
    """
    
    __slots__ = ('objectives',)
    
    def __init__(self, activity_id: str, name: str, description: str, required_level: int,
                 objectives: List[str], reward_config: Optional[Dict[str, Any]] = None):
        """
        Initialize a quest.
        
        Args:
            activity_id: The unique identifier for the quest.
            name: The name of the quest.
            description: The description of the quest.
            required_level: The minimum player level required to participate.
            objectives: The objectives of the quest.
            reward_config: Configuration for rewards.
        """
        super().__init__(activity_id, name, description, ActivityType.QUEST, required_level, reward_config)
        self.objectives = objectives
    
    @lru_cache(maxsize=32)
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the quest.
        
        This method is cached to improve performance for repeated calls with the same parameters.
        For quests, the score parameter is typically ignored as quests have fixed rewards.
        
        Args:
            score: The player's score in the quest (typically ignored).
            player_level: The player's level.
            
        Returns:
            The rewards for completing the quest.
        """
        # Base rewards with level scaling
        base_bits = self.reward_config.get("base_bits", 100)
        level_bonus = player_level * 10
        bits = base_bits + level_bonus
        
        # AURA rewards are more precious and scale less aggressively
        base_aura = self.reward_config.get("base_aura", 0)
        aura_level_bonus = player_level // 2  # Only 1 AURA per 2 levels
        aura = base_aura + aura_level_bonus
        
        # Reputation is fixed but with a small level bonus
        base_reputation = self.reward_config.get("reputation", 5)
        reputation_level_bonus = player_level // 5  # 1 extra reputation per 5 levels
        reputation = base_reputation + reputation_level_bonus
        
        # Calculate stat experience - small boost to all stats with level scaling
        stat_experience = {}
        base_stat_exp = 10
        for stat in StatType:
            # Higher level players get more stat experience
            stat_experience[stat] = base_stat_exp + (player_level // 2)
        
        # Check for guaranteed items
        items = []
        for item_id in self.reward_config.get("guaranteed_items", []):
            # In a real implementation, we would look up the item from a database
            # For now, we'll create a placeholder item
            rarity = ItemRarity.UNCOMMON
            
            # Special handling for legendary items
            if "legendary" in item_id:
                rarity = ItemRarity.LEGENDARY
            elif "rare" in item_id:
                rarity = ItemRarity.RARE
            elif "epic" in item_id:
                rarity = ItemRarity.EPIC
            
            item = Item(
                id=item_id,
                name=item_id.replace("_", " ").title(),
                description=f"A reward from the {self.name} quest.",
                item_type=ItemType.QUEST_ITEM,
                rarity=rarity
            )
            items.append((item, 1))
        
        # Chance for bonus items based on player level
        if random.random() < 0.05 + (player_level * 0.01):  # 5% base chance + 1% per level
            bonus_item = Consumable(
                id=f"{self.id}_bonus",
                name=f"{self.name} Bonus Item",
                description=f"A special bonus item from completing the {self.name} quest.",
                item_type=ItemType.CONSUMABLE,
                rarity=ItemRarity.UNCOMMON,
                effect="Provides a temporary boost to a random stat."
            )
            items.append((bonus_item, 1))
        
        return ActivityReward(
            bits=bits,
            aura=aura,
            stat_experience=stat_experience,
            reputation=reputation,
            items=items
        )


class AdventurousQuest(Quest):
    """
    An adventurous quest activity.
    
    These are long, challenging questlines that tell the deeper story of the
    Critter-Craft world and offer the greatest rewards.
    
    Attributes:
        stages: The stages of the adventurous quest.
    """
    
    __slots__ = ('stages',)
    
    def __init__(self, activity_id: str, name: str, description: str, required_level: int,
                 stages: List[Dict[str, Any]], reward_config: Optional[Dict[str, Any]] = None):
        """
        Initialize an adventurous quest.
        
        Args:
            activity_id: The unique identifier for the adventurous quest.
            name: The name of the adventurous quest.
            description: The description of the adventurous quest.
            required_level: The minimum player level required to participate.
            stages: The stages of the adventurous quest.
            reward_config: Configuration for rewards.
        """
        # Extract objectives from stages
        objectives = []
        for stage in stages:
            objectives.extend(stage.get("objectives", []))
        
        super().__init__(activity_id, name, description, required_level, objectives, reward_config)
        self.activity_type = ActivityType.ADVENTUROUS_QUEST
        self.stages = stages
    
    @lru_cache(maxsize=32)
    def get_rewards(self, score: int, player_level: int) -> ActivityReward:
        """
        Get the rewards for completing the adventurous quest.
        
        This method is cached to improve performance for repeated calls with the same parameters.
        
        Args:
            score: The player's score in the adventurous quest.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the adventurous quest.
        """
        # Get base rewards from Quest class
        rewards = super().get_rewards(score, player_level)
        
        # Adventurous quests have much higher rewards
        # Apply a multiplier based on player level (higher level = higher multiplier)
        base_multiplier = 2.0
        level_bonus = min(1.0, player_level * 0.05)  # Max 1.0 bonus at level 20
        reward_multiplier = base_multiplier + level_bonus
        
        rewards.bits = int(rewards.bits * reward_multiplier)
        rewards.aura = int(rewards.aura * reward_multiplier)
        rewards.reputation = int(rewards.reputation * reward_multiplier)
        
        # Enhance stat experience for adventurous quests
        for stat in rewards.stat_experience:
            rewards.stat_experience[stat] = int(rewards.stat_experience[stat] * reward_multiplier)
        
        # Add special legendary items
        for item_id in self.reward_config.get("guaranteed_items", []):
            if "legendary" in item_id:
                # In a real implementation, we would look up the item from a database
                # For now, we'll create a placeholder legendary item
                legendary_item = Item(
                    id=item_id,
                    name=item_id.replace("_", " ").title(),
                    description=f"A legendary reward from the {self.name} quest.",
                    item_type=ItemType.GEAR,
                    rarity=ItemRarity.LEGENDARY
                )
                rewards.items.append((legendary_item, 1))
        
        # Add a chance for an additional epic item
        if random.random() < 0.2 + (player_level * 0.02):  # 20% base chance + 2% per level
            epic_item = Item(
                id=f"{self.id}_epic_bonus",
                name=f"Epic {self.name} Reward",
                description=f"An epic bonus reward from completing the {self.name} quest.",
                item_type=ItemType.GEAR,
                rarity=ItemRarity.EPIC
            )
            rewards.items.append((epic_item, 1))
        
        return rewards


class ActivityManager:
    """
    Manager for all activities in Critter-Craft.
    
    This class is responsible for managing all activities, including mini-games,
    two-player games, jobs, and quests. It provides methods for adding, retrieving,
    and completing activities.
    
    Attributes:
        activities: A dictionary mapping activity IDs to Activity objects.
        completed_activities: A nested dictionary tracking completed activities by player.
        activity_cache: A cache of activities by type for faster lookups.
    """
    
    def __init__(self):
        """Initialize the activity manager."""
        self.activities: Dict[str, Activity] = {}
        self.completed_activities: Dict[str, Dict[str, List[str]]] = {}  # player_id -> activity_type -> [activity_id]
        self.activity_cache: Dict[ActivityType, List[Activity]] = {}  # Cache for activities by type
    
    def add_activity(self, activity: Activity) -> None:
        """
        Add an activity to the manager.
        
        Args:
            activity: The activity to add.
        """
        self.activities[activity.id] = activity
        
        # Clear the cache for this activity type
        if activity.activity_type in self.activity_cache:
            del self.activity_cache[activity.activity_type]
    
    def get_activity(self, activity_id: str) -> Optional[Activity]:
        """
        Get an activity by ID.
        
        Args:
            activity_id: The ID of the activity to get.
            
        Returns:
            The activity, or None if not found.
        """
        return self.activities.get(activity_id)
    
    @lru_cache(maxsize=8)  # Cache results for different activity types
    def get_activities_by_type(self, activity_type: ActivityType) -> List[Activity]:
        """
        Get all activities of a specific type.
        
        This method is cached to improve performance for repeated calls with the same activity type.
        
        Args:
            activity_type: The type of activities to get.
            
        Returns:
            A list of activities of the specified type.
        """
        # Check if we have a cached result
        if activity_type in self.activity_cache:
            return self.activity_cache[activity_type]
        
        # Build the result and cache it
        result = [
            activity for activity in self.activities.values()
            if activity.activity_type == activity_type
        ]
        self.activity_cache[activity_type] = result
        return result
    
    def get_available_activities(self, player_id: str, player_level: int) -> List[Activity]:
        """
        Get all activities available to a player.
        
        Args:
            player_id: The ID of the player.
            player_level: The level of the player.
            
        Returns:
            A list of activities available to the player.
        """
        # Use the is_available_for_player method for more flexibility
        return [
            activity for activity in self.activities.values()
            if activity.is_available_for_player(player_level)
        ]
    
    def complete_activity(self, player_id: str, activity_id: str, score: int, player_level: int) -> ActivityReward:
        """
        Complete an activity and record the completion.
        
        Args:
            player_id: The ID of the player completing the activity.
            activity_id: The ID of the activity to complete.
            score: The player's score in the activity.
            player_level: The player's level.
            
        Returns:
            The rewards for completing the activity.
            
        Raises:
            ValueError: If the activity is not found.
        """
        activity = self.get_activity(activity_id)
        
        if not activity:
            raise ValueError(f"Activity not found: {activity_id}")
        
        # Validate player level requirement
        if not activity.is_available_for_player(player_level):
            raise ValueError(f"Player level {player_level} is too low for activity {activity_id} (required: {activity.required_level})")
        
        # Get the rewards
        rewards = activity.get_rewards(score, player_level)
        
        # Record the completion
        self._record_activity_completion(player_id, activity)
        
        return rewards
    
    def _record_activity_completion(self, player_id: str, activity: Activity) -> None:
        """
        Record that a player has completed an activity.
        
        Args:
            player_id: The ID of the player.
            activity: The activity that was completed.
        """
        # Initialize player's completion record if needed
        if player_id not in self.completed_activities:
            self.completed_activities[player_id] = {}
        
        activity_type_name = activity.activity_type.name
        
        # Initialize activity type list if needed
        if activity_type_name not in self.completed_activities[player_id]:
            self.completed_activities[player_id][activity_type_name] = []
        
        # Add the activity ID to the list of completed activities
        self.completed_activities[player_id][activity_type_name].append(activity.id)
    
    def has_completed_activity(self, player_id: str, activity_id: str) -> bool:
        """
        Check if a player has completed an activity.
        
        Args:
            player_id: The ID of the player.
            activity_id: The ID of the activity.
            
        Returns:
            True if the player has completed the activity, False otherwise.
        """
        # Quick checks first
        if player_id not in self.completed_activities:
            return False
        
        activity = self.get_activity(activity_id)
        if not activity:
            return False
        
        activity_type_name = activity.activity_type.name
        
        # Check if the player has completed any activities of this type
        if activity_type_name not in self.completed_activities[player_id]:
            return False
        
        # Check if the specific activity has been completed
        return activity_id in self.completed_activities[player_id][activity_type_name]
    
    def get_completed_activities(self, player_id: str, activity_type: Optional[ActivityType] = None) -> List[Activity]:
        """
        Get all activities completed by a player.
        
        Args:
            player_id: The ID of the player.
            activity_type: Optional filter for activity type.
            
        Returns:
            A list of activities completed by the player.
        """
        if player_id not in self.completed_activities:
            return []
        
        completed_activities = []
        
        if activity_type:
            # Filter by activity type
            activity_type_name = activity_type.name
            if activity_type_name in self.completed_activities[player_id]:
                for activity_id in self.completed_activities[player_id][activity_type_name]:
                    activity = self.get_activity(activity_id)
                    if activity:
                        completed_activities.append(activity)
        else:
            # Get all completed activities
            for activity_type_name, activity_ids in self.completed_activities[player_id].items():
                for activity_id in activity_ids:
                    activity = self.get_activity(activity_id)
                    if activity:
                        completed_activities.append(activity)
        
        return completed_activities


def create_activity_from_config(config: Dict[str, Any]) -> Activity:
    """
    Create an activity from a configuration dictionary.
    
    This factory function creates the appropriate activity subclass based on the
    activity type specified in the configuration.
    
    Args:
        config: The configuration dictionary.
        
    Returns:
        The created activity.
        
    Raises:
        ValueError: If the activity type is unknown or required fields are missing.
    """
    # Extract common fields with validation
    activity_type = config.get("type")
    if not activity_type:
        raise ValueError("Activity type is required")
    
    activity_id = config.get("id")
    if not activity_id:
        raise ValueError("Activity ID is required")
    
    name = config.get("name", "Unnamed Activity")
    description = config.get("description", "")
    required_level = config.get("required_level", 1)
    reward_config = config.get("reward_config", {})
    
    # Create the appropriate activity subclass
    if activity_type == ActivityType.MINI_GAME:
        stats_trained = config.get("stats_trained", [])
        if not stats_trained:
            raise ValueError(f"Mini-game {activity_id} requires stats_trained")
        return MiniGame(activity_id, name, description, required_level, stats_trained, reward_config)
    
    elif activity_type == ActivityType.JOB:
        primary_stat = config.get("primary_stat")
        if not primary_stat:
            raise ValueError(f"Job {activity_id} requires primary_stat")
        duration_seconds = config.get("duration_seconds", 3600)
        return Job(activity_id, name, description, required_level, primary_stat, duration_seconds, reward_config)
    
    elif activity_type == ActivityType.QUEST:
        objectives = config.get("objectives", [])
        if not objectives:
            raise ValueError(f"Quest {activity_id} requires objectives")
        return Quest(activity_id, name, description, required_level, objectives, reward_config)
    
    elif activity_type == ActivityType.ADVENTUROUS_QUEST:
        stages = config.get("stages", [])
        if not stages:
            raise ValueError(f"Adventurous quest {activity_id} requires stages")
        return AdventurousQuest(activity_id, name, description, required_level, stages, reward_config)
    
    else:
        raise ValueError(f"Unknown activity type: {activity_type}")


def create_activity_manager_from_config(config_list: List[Dict[str, Any]]) -> ActivityManager:
    """
    Create an activity manager from a list of configuration dictionaries.
    
    This function processes a list of activity configurations and creates an
    ActivityManager with all the activities added.
    
    Args:
        config_list: The list of configuration dictionaries.
        
    Returns:
        The created activity manager with all activities added.
    """
    manager = ActivityManager()
    
    # Track IDs to ensure uniqueness
    activity_ids = set()
    
    for i, config in enumerate(config_list):
        try:
            # Check for duplicate IDs
            activity_id = config.get("id")
            if activity_id in activity_ids:
                raise ValueError(f"Duplicate activity ID: {activity_id}")
            
            # Create and add the activity
            activity = create_activity_from_config(config)
            manager.add_activity(activity)
            
            # Record the ID
            activity_ids.add(activity_id)
            
        except ValueError as e:
            # Log the error but continue processing other activities
            print(f"Error processing activity at index {i}: {e}")
    
    return manager