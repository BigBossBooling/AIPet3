"""
Critter-Craft Battle System

A strategic turn-based battle system for Critter-Craft that focuses on
tactical gameplay using critters' unique adaptations and environmental factors.
"""

from .manager import BattleManager
from .state import BattlePet, BattleEnvironment, StatusEffect
from .abilities import Ability
from .items import Item, Consumable, Gear
from .ui import BattleUI
from .demo import run_demo

__all__ = [
    'BattleManager',
    'BattlePet',
    'BattleEnvironment',
    'StatusEffect',
    'Ability',
    'Item',
    'Consumable',
    'Gear',
    'BattleUI',
    'run_demo',
]

def start_battle(player_pet, opponent_pet, environment_type, items=None):
    """
    Convenience function to start a battle between two pets in a specific environment.
    
    Args:
        player_pet: The player's pet object
        opponent_pet: The opponent's pet object (wild or another zoologist's)
        environment_type: The type of environment for the battle
        items: Optional list of items the player has available
        
    Returns:
        The result of the battle (win, loss, or draw)
    """
    from .manager import BattleManager
    from .ui import BattleUI
    
    ui = BattleUI(use_color=True)
    battle = BattleManager(player_pet, opponent_pet, environment_type, items, ui)
    return battle.run_battle()