"""
Demo script for the Critter-Craft battle system.

This script demonstrates a simple battle between a player's pet and a wild opponent.
"""

import sys
import time

from .manager import BattleManager
from .ui import BattleUI


def run_demo():
    """Run a demo battle."""
    print("Welcome to the Critter-Craft Battle System Demo!")
    print("=" * 60)
    time.sleep(1)
    
    # Create sample pets
    player_pet = {
        "name": "Sparkles",
        "species": "Chameleon",
        "level": 3,
        "adaptations": [
            "basic_maneuver",
            "camouflage",
            "defend",
            "echolocation"
        ]
    }
    
    opponent_pet = {
        "name": "Glimmer",
        "species": "Anglerfish",
        "level": 2,
        "adaptations": [
            "basic_maneuver",
            "bioluminescence",
            "defend",
            "venom_strike"
        ]
    }
    
    # Available items for the player
    player_items = [
        "healing_salve",
        "adrenaline_berry",
        "focus_root"
    ]
    
    # Create UI
    ui = BattleUI(use_color=True, animation_speed=0.3)
    
    # Create and run battle
    battle = BattleManager(
        player_pet=player_pet,
        opponent_pet=opponent_pet,
        environment_type="cavern",
        items=player_items,
        ui=ui
    )
    
    result = battle.run_battle()
    
    # Display final result
    print("\nBattle Summary:")
    print(f"Winner: {'Player' if result['winner'] == 'player' else 'Opponent'}")
    print(f"Turns taken: {result['turns_taken']}")
    
    if result['winner'] == 'player' and 'rewards' in result:
        print("\nRewards earned:")
        rewards = result['rewards']
        print(f"  Experience: {rewards['experience']} XP")
        print(f"  Research Points: {rewards['research_points']}")
        if rewards['items']:
            print(f"  Items: {', '.join(rewards['items'])}")
        print(f"  Friendship: +{rewards['friendship']}")
    
    print("\nThank you for trying the Critter-Craft Battle System!")


if __name__ == "__main__":
    run_demo()