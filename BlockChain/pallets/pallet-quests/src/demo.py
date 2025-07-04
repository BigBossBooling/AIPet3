"""
Interactive Demo: Critter-Craft Ecosystem Sandbox

This script provides an interactive command-line interface to explore the core
systems of the Critter-Craft universe. It demonstrates the data-driven activities
engine and the Zoologist's Lodge in a dynamic, user-driven environment.

This demo is a showcase of the Expanded KISS Principle in action:
- (K) Core systems are decoupled (ActivityManager, Lodge).
- (I) The demo intelligently loads all game content from a config file.
- (S) The menu is a scalable, systematized dispatcher.
- (S) The UI provides a clear, engaging, and secure user experience.
"""
import random
import time
from typing import Dict, List, Optional, Callable

# --- Core System Imports (Assuming a proper package structure) ---
# Note: The fragile sys.path hacks have been removed.
from .config_activities import ACTIVITIES_CONFIG
from .activities import ActivityManager, Activity, ActivityType, StatType
# from .lodge import ZoologistLodge, PetState, CaregiverOffer # Assuming lodge.py exists

# --- Mock Objects for a Self-Contained Demo ---
class MockPlayer:
    """A mock player to hold state during the demo session."""
    def __init__(self, player_id: str, level: int):
        self.id = player_id
        self.level = level
        self.bits = 1000
        self.aura = 10
        self.reputation = 50
        self.inventory: Dict[str, int] = {} # item_id: quantity
        self.stats: Dict[StatType, int] = {stat: 20 for stat in StatType}

class UI:
    """A simple utility class for styling CLI output."""
    @staticmethod
    def header(title: str):
        print(f"\n{'='*8} {title.upper()} {'='*8}")

    @staticmethod
    def success(message: str):
        print(f"✅ {message}")

    @staticmethod
    def error(message: str):
        print(f"❌ {message}")

    @staticmethod
    def prompt(message: str) -> str:
        return input(f"👉 {message}").strip().lower()

class DemoManager:
    """Manages the interactive demo session and user interactions."""

    def __init__(self):
        self.player = MockPlayer("Zoologist_01", 10)
        # (I) Intelligently load all activities from the config file.
        self.activity_manager = ActivityManager(ACTIVITIES_CONFIG)
        # self.lodge = create_example_lodge() # Placeholder for lodge system
        self.is_running = True
        # (S) Systematize the menu using a dispatcher pattern.
        self.menu_options: Dict[str, Callable] = {
            "1": self._explore_activities,
            "2": self._visit_lodge,
            "3": self._view_player_status,
            "0": self._exit,
        }

    def run(self):
        """The main execution loop for the interactive demo."""
        UI.header("Welcome to the Critter-Craft Ecosystem Sandbox")
        while self.is_running:
            print("\nWhat would you like to do?")
            print("  1. Explore Activities (Mini-Games, Jobs, Quests)")
            print("  2. Visit the Zoologist's Lodge (Daycare System)")
            print("  3. View Player Status (Stats & Inventory)")
            print("  0. Exit Demo")
            choice = UI.prompt("Enter your choice: ")
            action = self.menu_options.get(choice, lambda: UI.error("Invalid choice."))
            action()

    def _exit(self):
        print("\nExiting the Critter-Craft Sandbox. Goodbye!")
        self.is_running = False

    def _explore_activities(self):
        """Handler for the activities sub-menu."""
        UI.header("Activity Center")
        available_activities = self.activity_manager.get_available_activities(self.player.level)
        
        if not available_activities:
            UI.error("No activities available for your level.")
            return

        print("Available Activities:")
        for i, activity in enumerate(available_activities):
            print(f"  {i + 1}. {activity.name} ({activity.activity_type.name})")
        print("  0. Back to Main Menu")

        choice_str = UI.prompt("Choose an activity to simulate: ")
        try:
            if choice_str == "0": return
            choice_idx = int(choice_str) - 1
            if not 0 <= choice_idx < len(available_activities): raise ValueError

            activity = available_activities[choice_idx]
            self._simulate_activity(activity)
        except (ValueError, IndexError):
            UI.error("Invalid selection.")

    def _simulate_activity(self, activity: Activity):
        """Simulates performing a single activity and granting rewards."""
        UI.header(f"Simulating: {activity.name}")
        
        # Simulate a score based on activity type
        score = random.randint(50, 150) if activity.activity_type == ActivityType.MINI_GAME else 100
        pet_stat_score = self.player.stats.get(getattr(activity, 'primary_stat', None), 20)
        
        print(f"Simulated performance score: {score}")
        
        rewards = activity.calculate_rewards(score=score, player_level=self.player.level)
        
        # Apply rewards to the player
        self.player.bits += rewards.bits
        self.player.aura += rewards.aura
        self.player.reputation += rewards.reputation
        for item_id, quantity in rewards.items.items():
            self.player.inventory[item_id] = self.player.inventory.get(item_id, 0) + quantity
        for stat, exp in rewards.stat_experience.items():
            self.player.stats[stat] = self.player.stats.get(stat, 0) + exp

        UI.success(f"Activity complete! You earned rewards.")
        print(f"  + {rewards.bits} $BITS, + {rewards.aura} $AURA, + {rewards.reputation} Reputation")
        if rewards.items:
            print(f"  Items Found: {rewards.items}")
        if rewards.stat_experience:
            print(f"  Stat Experience Gained: {rewards.stat_experience}")
        time.sleep(1)

    def _visit_lodge(self):
        """Handler for the Zoologist's Lodge sub-menu."""
        UI.header("Zoologist's Lodge")
        UI.error("Lodge system simulation is not yet implemented in this demo.")
        print("  In the full system, you could:")
        print("  - View pets currently lodged.")
        print("  - Browse and hire caregivers.")
        print("  - Check in your own pet for offline care and training.")
        time.sleep(1)

    def _view_player_status(self):
        """Displays the current status of the mock player."""
        UI.header(f"Status Report for {self.player.id}")
        print(f"  Level: {self.player.level}")
        print(f"  Currencies: {self.player.bits} $BITS | {self.player.aura} $AURA")
        print(f"  Reputation: {self.player.reputation}")
        
        print("\n  Pet Stats (Experience):")
        for stat, value in self.player.stats.items():
            print(f"    - {stat.name:<12}: {value}")
            
        print("\n  Inventory:")
        if not self.player.inventory:
            print("    - Empty")
        else:
            for item_id, quantity in self.player.inventory.items():
                print(f"    - {item_id}: x{quantity}")
        time.sleep(1)


if __name__ == "__main__":
    # The demo assumes it's being run from a context where the other modules
    # (`activities`, `config_activities`) are importable as a package.
    demo = DemoManager()
    demo.run()
