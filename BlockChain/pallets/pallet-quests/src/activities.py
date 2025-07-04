"""
Core definitions for the Activities system in Critter-Craft.

This module defines the fundamental enums and data structures that represent
the different types of activities and stats in the game.
"""

from enum import Enum, auto
from typing import Dict, List, Any


class ActivityType(Enum):
    """Types of activities available in Critter-Craft."""
    MINI_GAME = auto()
    TWO_PLAYER_GAME = auto()
    JOB = auto()
    QUEST = auto()
    ADVENTUROUS_QUEST = auto()


class StatType(Enum):
    """Pet stats that can be trained through activities."""
    IQ = auto()
    CHARISMA = auto()
    ENERGY = auto()
    AGILITY = auto()
    STRENGTH = auto()
    SOCIAL = auto()


class ActivityManager:
    """
    Legacy class for backward compatibility.
    
    This class is maintained for compatibility with existing code that might
    import it. New code should use the ActivityManager from activities_system.py.
    """
    
    def __init__(self, config_list: List[Dict[str, Any]]):
        """
        Initialize the activity manager with a list of activity configurations.
        
        Args:
            config_list: A list of dictionaries containing activity configurations.
        """
        self.config_list = config_list
    
    def get_available_activities(self, player_level: int) -> List[Dict[str, Any]]:
        """
        Get all activities available to a player of the given level.
        
        Args:
            player_level: The level of the player.
            
        Returns:
            A list of activity configurations available to the player.
        """
        return [
            activity for activity in self.config_list
            if activity.get("required_level", 1) <= player_level
        ]
    
    def get_activities_by_type(self, activity_type: ActivityType) -> List[Dict[str, Any]]:
        """
        Get all activities of a specific type.
        
        Args:
            activity_type: The type of activities to get.
            
        Returns:
            A list of activity configurations of the specified type.
        """
        return [
            activity for activity in self.config_list
            if activity.get("type") == activity_type
        ]
