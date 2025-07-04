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

# --- Core System Imports ---
from .config_activities import ACTIVITIES_CONFIG
from .activities import ActivityType, StatType
from .activities_system import (
    ActivityManager,
    Activity,
    ActivityReward,
    create_activity_from_config,
    create_activity_manager_from_config
)
from .lodge import (
    PersonalityTrait,
    CareActivityType,
    PetState,
    CaregiverOffer,
    LodgingContract,
    CareActivityLog
)
from .lodge_system import ZoologistLodge, create_example_lodge

# --- Mock Objects for a Self-Contained Demo ---
class MockPlayer:
    """A mock player to hold state during the demo session."""
    def __init__(self, player_id: str, level: int):
        self.id = player_id
        self.level = level
        self.bits = 1000
        self.aura = 10
        self.reputation = 50
        self.inventory: Dict[str, int] = {}  # item_id: quantity
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
        self.activity_manager = create_activity_manager_from_config(ACTIVITIES_CONFIG)
        self.lodge = create_example_lodge()
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
        available_activities = self.activity_manager.get_available_activities(self.player.id, self.player.level)
        
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
        
        print(f"Simulated performance score: {score}")
        
        rewards = self.activity_manager.complete_activity(
            player_id=self.player.id,
            activity_id=activity.id,
            score=score,
            player_level=self.player.level
        )
        
        # Apply rewards to the player
        self.player.bits += rewards.bits
        self.player.aura += rewards.aura
        self.player.reputation += rewards.reputation
        
        # Update player stats
        for stat, exp in rewards.stat_experience.items():
            self.player.stats[stat] = self.player.stats.get(stat, 0) + exp
        
        # Update player inventory
        for item, quantity in rewards.items:
            self.player.inventory[item.id] = self.player.inventory.get(item.id, 0) + quantity

        UI.success(f"Activity complete! You earned rewards.")
        print(f"  + {rewards.bits} $BITS, + {rewards.aura} $AURA, + {rewards.reputation} Reputation")
        
        if rewards.items:
            print("  Items Found:")
            for item, quantity in rewards.items:
                print(f"    - {item.name} x{quantity}")
        
        if rewards.stat_experience:
            print("  Stat Experience Gained:")
            for stat, exp in rewards.stat_experience.items():
                print(f"    - {stat.name}: +{exp}")
        
        time.sleep(1)

    def _visit_lodge(self):
        """Handler for the Zoologist's Lodge sub-menu."""
        UI.header("Zoologist's Lodge")
        
        print("Lodge Options:")
        print("  1. View Pets in Lodge")
        print("  2. View Caregiver Offers")
        print("  3. Create Lodging Contract")
        print("  4. Perform Care Activity")
        print("  0. Back to Main Menu")
        
        choice = UI.prompt("Choose an option: ")
        
        if choice == "1":
            self._view_pets_in_lodge()
        elif choice == "2":
            self._view_caregiver_offers()
        elif choice == "3":
            self._create_lodging_contract()
        elif choice == "4":
            self._perform_care_activity()
        elif choice == "0":
            return
        else:
            UI.error("Invalid choice.")

    def _view_pets_in_lodge(self):
        """View all pets in the Zoologist's Lodge."""
        UI.header("Pets in Lodge")
        
        if not self.lodge.pets:
            UI.error("No pets in the lodge.")
            return
        
        print("Pets currently in the Zoologist's Lodge:")
        for i, (pet_id, pet) in enumerate(self.lodge.pets.items()):
            print(f"  {i + 1}. {pet.name} ({pet.species}), Level {pet.level}, Happiness: {pet.happiness}")
            
            # Check if the pet has an active contract
            active_contract = None
            for contract in self.lodge.lodging_contracts.values():
                if contract.pet_id == pet_id and contract.is_active:
                    active_contract = contract
                    break
            
            if active_contract:
                print(f"     Cared for by: {active_contract.caregiver_id}")
                print(f"     Contract ends: {time.ctime(active_contract.end_time)}")
            else:
                print("     Not currently under care")
        
        time.sleep(1)

    def _view_caregiver_offers(self):
        """View all caregiver offers in the Zoologist's Lodge."""
        UI.header("Caregiver Offers")
        
        if not self.lodge.caregiver_offers:
            UI.error("No caregiver offers available.")
            return
        
        print("Available Caregivers:")
        for i, offer in enumerate(self.lodge.caregiver_offers.values()):
            print(f"  {i + 1}. {offer.player_id}")
            print(f"     Fee: {offer.fee_per_day} $BITS per day")
            print(f"     Dominant Trait: {offer.dominant_trait.name}")
            print(f"     Reputation: {offer.reputation}")
            print(f"     Pets: {offer.current_pets}/{offer.max_pets}")
        
        time.sleep(1)

    def _create_lodging_contract(self):
        """Create a lodging contract between a pet owner and a caregiver."""
        UI.header("Create Lodging Contract")
        
        if not self.lodge.pets:
            UI.error("No pets in the lodge.")
            return
        
        if not self.lodge.caregiver_offers:
            UI.error("No caregiver offers available.")
            return
        
        # Select a pet
        print("Select a pet:")
        pet_list = list(self.lodge.pets.items())
        for i, (pet_id, pet) in enumerate(pet_list):
            print(f"  {i + 1}. {pet.name} ({pet.species})")
        
        pet_choice = UI.prompt("Choose a pet: ")
        try:
            pet_idx = int(pet_choice) - 1
            if not 0 <= pet_idx < len(pet_list): raise ValueError
            pet_id, pet = pet_list[pet_idx]
        except (ValueError, IndexError):
            UI.error("Invalid selection.")
            return
        
        # Check if the pet already has an active contract
        for contract in self.lodge.lodging_contracts.values():
            if contract.pet_id == pet_id and contract.is_active:
                UI.error(f"{pet.name} already has an active contract.")
                return
        
        # Select a caregiver
        print("\nSelect a caregiver:")
        caregiver_list = list(self.lodge.caregiver_offers.items())
        for i, (offer_id, offer) in enumerate(caregiver_list):
            print(f"  {i + 1}. {offer.player_id}")
            print(f"     Fee: {offer.fee_per_day} $BITS per day")
            print(f"     Dominant Trait: {offer.dominant_trait.name}")
            print(f"     Pets: {offer.current_pets}/{offer.max_pets}")
        
        caregiver_choice = UI.prompt("Choose a caregiver: ")
        try:
            caregiver_idx = int(caregiver_choice) - 1
            if not 0 <= caregiver_idx < len(caregiver_list): raise ValueError
            offer_id, offer = caregiver_list[caregiver_idx]
        except (ValueError, IndexError):
            UI.error("Invalid selection.")
            return
        
        # Check if the caregiver has reached their maximum number of pets
        if offer.current_pets >= offer.max_pets:
            UI.error(f"{offer.player_id} has reached their maximum number of pets.")
            return
        
        # Select a duration
        print("\nSelect a duration:")
        print("  1. 1 day")
        print("  2. 3 days")
        print("  3. 7 days")
        
        duration_choice = UI.prompt("Choose a duration: ")
        if duration_choice == "1":
            duration_days = 1
        elif duration_choice == "2":
            duration_days = 3
        elif duration_choice == "3":
            duration_days = 7
        else:
            UI.error("Invalid selection.")
            return
        
        # Calculate the total cost
        total_cost = offer.fee_per_day * duration_days
        
        # Check if the player has enough BITS
        if self.player.bits < total_cost:
            UI.error(f"You don't have enough BITS. Cost: {total_cost} BITS, You have: {self.player.bits} BITS.")
            return
        
        # Create the contract
        contract = self.lodge.create_lodging_contract(pet_id, offer_id, duration_days)
        
        if contract:
            # Deduct the cost from the player's BITS
            self.player.bits -= total_cost
            
            UI.success(f"Lodging contract created for {pet.name} with {offer.player_id}.")
            print(f"  Duration: {duration_days} days")
            print(f"  Total Cost: {total_cost} BITS")
            print(f"  Contract ID: {contract.id}")
        else:
            UI.error("Failed to create lodging contract.")
        
        time.sleep(1)

    def _perform_care_activity(self):
        """Perform a care activity on a pet."""
        UI.header("Perform Care Activity")
        
        # Get active contracts where the player is the caregiver
        active_contracts = [
            contract for contract in self.lodge.lodging_contracts.values()
            if contract.caregiver_id == self.player.id and contract.is_active
        ]
        
        if not active_contracts:
            UI.error("You are not currently caring for any pets.")
            return
        
        # Select a pet
        print("Select a pet to care for:")
        for i, contract in enumerate(active_contracts):
            pet = self.lodge.pets.get(contract.pet_id)
            if pet:
                print(f"  {i + 1}. {pet.name} ({pet.species})")
        
        pet_choice = UI.prompt("Choose a pet: ")
        try:
            pet_idx = int(pet_choice) - 1
            if not 0 <= pet_idx < len(active_contracts): raise ValueError
            contract = active_contracts[pet_idx]
            pet = self.lodge.pets.get(contract.pet_id)
        except (ValueError, IndexError):
            UI.error("Invalid selection.")
            return
        
        if not pet:
            UI.error("Pet not found.")
            return
        
        # Select an activity
        print("\nSelect a care activity:")
        print(f"  1. Feed {pet.name}")
        print(f"  2. Play with {pet.name}")
        print(f"  3. Groom {pet.name}")
        
        activity_choice = UI.prompt("Choose an activity: ")
        if activity_choice == "1":
            activity_type = CareActivityType.FEED
        elif activity_choice == "2":
            activity_type = CareActivityType.PLAY
        elif activity_choice == "3":
            activity_type = CareActivityType.GROOM
        else:
            UI.error("Invalid selection.")
            return
        
        # Perform the care activity
        care_activity = self.lodge.perform_care_activity(self.player.id, pet.pet_id, activity_type)
        
        if care_activity:
            UI.success(f"Care activity performed: {activity_type.name}")
            print(f"  Happiness Gain: +{care_activity.happiness_gain}")
            print(f"  Stat Gains: {care_activity.stat_gains}")
            
            # Print the pet's updated state
            print(f"\n{pet.name}'s Updated State:")
            print(f"  Happiness: {pet.happiness}")
            print(f"  Stats: {pet.stats}")
            
            # Print temporary trait boosts
            if pet.temporary_trait_boosts:
                print("  Temporary Trait Boosts:")
                for trait, (boost, expiry) in pet.temporary_trait_boosts.items():
                    print(f"    - {trait.name}: +{boost} (expires: {time.ctime(expiry)})")
        else:
            UI.error("Failed to perform care activity.")
        
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
    # are importable as a package.
    demo = DemoManager()
    demo.run()