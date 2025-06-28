"""
AdvancedMenuManager: A Refactoring Case Study in the Expanded KISS Principle.

This module demonstrates a highly robust, scalable, and maintainable command-line
interface, refactored by applying the strategic tenets of the Expanded KISS
Principle. It manages a pet's advanced activities with a focus on modularity,
systematic design, and user experience.
"""

from typing import Dict, Any, Callable, Tuple, List, Optional, TypeVar
from dataclasses import dataclass, field
from enum import Enum

# Assume a 'Pet' object type exists for type hinting purposes.
Pet = TypeVar("Pet")

# --- Service Imports (K - Know Your Core, Keep it Clear) ---
from advanced_services import (
    JOB_TYPES, BATTLE_OPPONENTS, AVAILABLE_QUESTS, EDUCATION_DEGREES, EVOLUTION_PATHS,
    perform_job, battle, can_start_quest, complete_quest, can_earn_degree,
    earn_degree, can_evolve, evolve, apply_mutation, check_achievements,
    can_take_job  # Added for dynamic availability checks
)

# --- Enum for Menu Actions ---
class MenuActionEnum(Enum):
    WORK_JOB = "Work a Job"
    BATTLE_OPPONENT = "Battle an Opponent"
    UNDERTAKE_QUEST = "Undertake a Quest"
    EARN_DEGREE = "Earn a Degree"
    EVOLVE = "Evolve"
    APPLY_MUTATION = "Apply Mutation"
    CHECK_ACHIEVEMENTS = "Check Achievements"
    DISPLAY_STATS = "Display Stats"

# --- Systematic Menu Configuration (S - Systematize for Scalability) ---
@dataclass
class MenuAction:
    """A dataclass to systematically define a menu action, decoupling the menu
    from the implementation details of each action."""
    name: MenuActionEnum
    execute_func: Callable[[Pet, str], Tuple[bool, str]]
    options_source: Optional[Dict[str, Any] | List[str]] = None
    pre_check_func: Optional[Callable[[Pet, str], bool]] = None
    availability_check: Optional[Callable[[Pet], bool]] = field(default=lambda p: True)
    custom_prompt: Optional[str] = None

class AdvancedMenuManager:
    """Manages the UI and logic for the advanced services menu, built on KISS."""

    def __init__(self, pet: Pet):
        """Initializes the manager and configures all menu actions."""
        self.pet = pet
        self._is_running = True
        
        # --- (K) Centralized Configuration of Menu Actions ---
        self.actions: List[MenuAction] = [
            MenuAction(
                name=MenuActionEnum.WORK_JOB,
                options_source=JOB_TYPES,
                execute_func=perform_job,
                pre_check_func=can_take_job
            ),
            MenuAction(
                name=MenuActionEnum.BATTLE_OPPONENT,
                options_source=BATTLE_OPPONENTS,
                execute_func=battle
            ),
            MenuAction(
                name=MenuActionEnum.UNDERTAKE_QUEST,
                options_source=AVAILABLE_QUESTS,
                execute_func=complete_quest,
                pre_check_func=can_start_quest
            ),
            MenuAction(
                name=MenuActionEnum.EARN_DEGREE,
                options_source=EDUCATION_DEGREES,
                execute_func=earn_degree,
                pre_check_func=can_earn_degree
            ),
            MenuAction(
                name=MenuActionEnum.EVOLVE,
                options_source=EVOLUTION_PATHS,
                execute_func=lambda p, path: (True, evolve(p, path)),
                pre_check_func=can_evolve
            ),
            MenuAction(
                name=MenuActionEnum.APPLY_MUTATION,
                execute_func=apply_mutation,
                custom_prompt="Enter mutation key to apply: "
            ),
            MenuAction(name=MenuActionEnum.CHECK_ACHIEVEMENTS, execute_func=self._execute_achievements),
            MenuAction(name=MenuActionEnum.DISPLAY_STATS, execute_func=self._execute_display_stats),
        ]

    # --- UI & Print Helpers (S - Stimulate Engagement) ---

    def _print_header(self, title: str) -> None:
        print(f"\n{'—'*8} {title.upper()} {'—'*8}")

    def _print_success(self, message: str) -> None:
        print(f"✅ Success: {message}")

    def _print_error(self, message: str) -> None:
        print(f"❌ Error: {message}")

    # --- Core Menu & Execution Logic ---

    def run(self) -> None:
        """The main execution loop for the menu."""
        while self._is_running:
            self._display_main_menu()
            choice_str = input("👉 Choose an option (or 0 to exit): ").strip()

            if choice_str == "0":
                self._exit()
                continue
            
            try:
                choice_idx = int(choice_str) - 1
                if not 0 <= choice_idx < len(self.actions):
                    raise ValueError
                self._execute_action(self.actions[choice_idx])
            except ValueError:
                self._print_error("Invalid choice. Please enter a valid number.")

    def _exit(self) -> None:
        """Stops the menu loop."""
        print("\nExiting advanced services. Goodbye!")
        self._is_running = False

    def _display_main_menu(self) -> None:
        """(S - Sense the Landscape) Displays the main menu, showing availability."""
        self._print_header("Advanced Menu")
        for i, action in enumerate(self.actions):
            availability = " (Not Available)" if not action.availability_check(self.pet) else ""
            print(f"  {i + 1}. {action.name.value}{availability}")
        print("  0. Exit")

    def _execute_action(self, action: MenuAction) -> None:
        """(S - Systematize) A single, powerful method to handle any menu action."""
        if not action.availability_check(self.pet):
            self._print_error(f"{action.name.value} is currently unavailable.")
            return

        if action.options_source:
            self._process_selection_action(action)
        elif action.custom_prompt:
            self._process_prompt_action(action)
        else:
            success, message = action.execute_func(self.pet, None)
            self._print_success(message) if success else self._print_error(message)

    def _process_selection_action(self, action: MenuAction) -> None:
        """Handles the sub-menu logic for actions with lists of options."""
        self._print_header(action.name.value)
        options = list(action.options_source.items()) if isinstance(action.options_source, dict) else [(path, path) for path in action.options_source]

        for i, (key, item) in enumerate(options):
            name = item.get('display_name', item) if isinstance(item, dict) else item
            availability = " (Requirements not met)" if action.pre_check_func and not action.pre_check_func(self.pet, key) else ""
            print(f"  {i + 1}. {name}{availability}")
        print("  0. Back to Main Menu")

        choice_str = input(f"👉 Choose a {action.name.value.lower()} option: ").strip()
        try:
            if choice_str == "0": return
            choice_idx = int(choice_str) - 1
            if not 0 <= choice_idx < len(options): raise ValueError
            
            key, _ = options[choice_idx]
            if action.pre_check_func and not action.pre_check_func(self.pet, key):
                self._print_error("Cannot select this option as requirements are not met.")
                return

            success, message = action.execute_func(self.pet, key)
            self._print_success(message) if success else self._print_error(message)
        except ValueError:
            self._print_error("Invalid selection.")

    def _process_prompt_action(self, action: MenuAction) -> None:
        """Handles actions that need a simple text prompt."""
        self._print_header(action.name.value)
        user_input = input(f"👉 {action.custom_prompt}").strip()
        if not user_input:
            self._print_error("Input cannot be empty.")
            return
        
        success, message = action.execute_func(self.pet, user_input)
        self._print_success(message) if success else self._print_error(message)

    # --- Specific Action Implementations ---
    
    def _execute_achievements(self, pet: Pet, key: Optional[str]) -> Tuple[bool, str]:
        """Wrapper for the check_achievements service call."""
        achievements = check_achievements(pet)
        if not achievements:
            return True, "No achievements unlocked yet."
        report = "\n".join([f"🏆 {ach}" for ach in achievements])
        return True, f"Unlocked Achievements:\n{report}"
    
    def _execute_display_stats(self, pet: Pet, key: Optional[str]) -> Tuple[bool, str]:
        """Displays the detailed stats of the pet."""
        self._print_header(f"{pet.name}'s Status Report")
        main_stats = " | ".join([f"{k.capitalize()}: {v}" for k, v in pet.stats.items()])
        print(main_stats)
        print("—" * len(main_stats))
        print(f"Level: {pet.level:<5} Age: {pet.age:<5} Exp: {pet.exp:<5} Coins: {pet.coins}")
        print(f"Evolution: {pet.evolution_stage}")
        if pet.achievements:
            print(f"Achievements: {', '.join(pet.achievements)}")
        return True, "Stats displayed."  # Return a silent success

if __name__ == "__main__":
    # --- (I) Mock Pet for Iterative Development & Demonstration ---
    class MockPet:
        def __init__(self):
            self.name = "Sparky"
            self.stats = {'strength': 15, 'agility': 20, 'intellect': 25}
            self.age = 5
            self.exp = 1250
            self.coins = 300
            self.level = 12
            self.evolution_stage = "Adolescent"
            self.achievements = ["First Battle Won"]

    print("Welcome to Critter-Craft Advanced Services!")
    menu = AdvancedMenuManager(MockPet())
    menu.run()
"""
AdvancedMenuManager: A Refactoring Case Study in the Expanded KISS Principle.

This module demonstrates a highly robust, scalable, and maintainable command-line
interface, refactored by applying the strategic tenets of the Expanded KISS
Principle. It manages a pet's advanced activities with a focus on modularity,
systematic design, and user experience.
"""
from typing import Dict, Any, Callable, Tuple, List, Optional, TypeVar
from dataclasses import dataclass, field

# Assume a 'Pet' object type exists for type hinting purposes.
Pet = TypeVar("Pet")

# --- Service Imports (K - Know Your Core, Keep it Clear) ---
# Dependencies are clearly grouped, representing the "advanced services" interface.
from advanced_services import (
    JOB_TYPES, BATTLE_OPPONENTS, AVAILABLE_QUESTS, EDUCATION_DEGREES, EVOLUTION_PATHS,
    perform_job, battle, can_start_quest, complete_quest, can_earn_degree,
    earn_degree, can_evolve, evolve, apply_mutation, check_achievements,
    can_take_job # Added for dynamic availability checks
)

# --- Systematic Menu Configuration (S - Systematize for Scalability) ---
@dataclass
class MenuAction:
    """A dataclass to systematically define a menu action, decoupling the menu
    from the implementation details of each action."""
    name: str
    # The function that performs the core action (e.g., `perform_job`)
    execute_func: Callable[[Pet, str], Tuple[bool, str]]
    # A dictionary or list of options for the user to choose from
    options_source: Optional[Dict[str, Any] | List[str]] = None
    # An optional function to check if the pet meets requirements for a choice
    pre_check_func: Optional[Callable[[Pet, str], bool]] = None
    # An optional function to check if the entire action category is available
    availability_check: Optional[Callable[[Pet], bool]] = field(default=lambda: (lambda p: True))
    # Custom prompt for actions that don't select from a list
    custom_prompt: Optional[str] = None

class AdvancedMenuManager:
    """Manages the UI and logic for the advanced services menu, built on KISS."""

    def __init__(self, pet: Pet):
        """Initializes the manager and configures all menu actions."""
        self.pet = pet
        self._is_running = True
        
        # --- (K) Centralized Configuration of Menu Actions ---
        # Each action is a self-contained, descriptive object. Adding a new
        # feature is as simple as adding a new MenuAction instance.
        self.actions: List[MenuAction] = [
            MenuAction(
                name="Work a Job",
                options_source=JOB_TYPES,
                execute_func=perform_job,
                pre_check_func=can_take_job # Used to show availability
            ),
            MenuAction(
                name="Battle an Opponent",
                options_source=BATTLE_OPPONENTS,
                execute_func=battle
            ),
            MenuAction(
                name="Undertake a Quest",
                options_source=AVAILABLE_QUESTS,
                execute_func=complete_quest,
                pre_check_func=can_start_quest
            ),
            MenuAction(
                name="Earn a Degree",
                options_source=EDUCATION_DEGREES,
                execute_func=earn_degree,
                pre_check_func=can_earn_degree
            ),
            MenuAction(
                name="Evolve",
                options_source=EVOLUTION_PATHS,
                execute_func=lambda p, path: (True, evolve(p, path)), # Adapt signature
                pre_check_func=can_evolve
            ),
            MenuAction(
                name="Apply Mutation",
                execute_func=apply_mutation,
                custom_prompt="Enter mutation key to apply: "
            ),
            MenuAction(name="Check Achievements", execute_func=self._execute_achievements),
            MenuAction(name="Display Stats", execute_func=self._execute_display_stats),
        ]

    # --- UI & Print Helpers (S - Stimulate Engagement) ---

    def _print_header(self, title: str) -> None:
        print(f"\n{'—'*8} {title.upper()} {'—'*8}")

    def _print_success(self, message: str) -> None:
        print(f"✅ Success: {message}")

    def _print_error(self, message: str) -> None:
        print(f"❌ Error: {message}")

    # --- Core Menu & Execution Logic ---

    def run(self) -> None:
        """The main execution loop for the menu."""
        self._is_running = True
        while self._is_running:
            self._display_main_menu()
            choice_str = input("👉 Choose an option (or 0 to exit): ").strip()

            if choice_str == "0":
                self._exit()
                continue
            
            try:
                choice_idx = int(choice_str) - 1
                if not 0 <= choice_idx < len(self.actions):
                    raise ValueError
                self._execute_action(self.actions[choice_idx])
            except ValueError:
                self._print_error("Invalid choice. Please enter a valid number.")

    def _exit(self) -> None:
        """Stops the menu loop."""
        print("\nExiting advanced services. Goodbye!")
        self._is_running = False

    def _display_main_menu(self) -> None:
        """(S - Sense the Landscape) Displays the main menu, showing availability."""
        self._print_header("Advanced Menu")
        for i, action in enumerate(self.actions):
            availability = ""
            if not action.availability_check(self.pet):
                availability = " (Not Available)"
            print(f"  {i + 1}. {action.name}{availability}")
        print("  0. Exit")

    def _execute_action(self, action: MenuAction) -> None:
        """(S - Systematize) A single, powerful method to handle any menu action."""
        if not action.availability_check(self.pet):
            self._print_error(f"{action.name} is currently unavailable.")
            return

        # Handle actions that require selecting from a list of options
        if action.options_source:
            self._process_selection_action(action)
        # Handle actions that require custom text input
        elif action.custom_prompt:
            self._process_prompt_action(action)
        # Handle actions with no sub-selection needed
        else:
            success, message = action.execute_func(self.pet, None)
            if success: self._print_success(message)
            else: self._print_error(message)

    def _process_selection_action(self, action: MenuAction):
        """Handles the sub-menu logic for actions with lists of options."""
        self._print_header(action.name)
        options = list(action.options_source.items()) if isinstance(action.options_source, dict) else [(path, path) for path in action.options_source]

        for i, (key, item) in enumerate(options):
            name = item.get('display_name', item) if isinstance(item, dict) else item
            availability = ""
            if action.pre_check_func and not action.pre_check_func(self.pet, key):
                availability = " (Requirements not met)"
            print(f"  {i + 1}. {name}{availability}")
        print("  0. Back to Main Menu")

        choice_str = input(f"👉 Choose a {action.name.lower()} option: ").strip()
        try:
            if choice_str == "0": return
            choice_idx = int(choice_str) - 1
            if not 0 <= choice_idx < len(options): raise ValueError
            
            key, _ = options[choice_idx]
            
            if action.pre_check_func and not action.pre_check_func(self.pet, key):
                self._print_error("Cannot select this option as requirements are not met.")
                return

            success, message = action.execute_func(self.pet, key)
            if success: self._print_success(message)
            else: self._print_error(message)
        except ValueError:
            self._print_error("Invalid selection.")

    def _process_prompt_action(self, action: MenuAction):
        """Handles actions that need a simple text prompt."""
        self._print_header(action.name)
        user_input = input(f"👉 {action.custom_prompt}").strip()
        if not user_input:
            self._print_error("Input cannot be empty.")
            return
        
        success, message = action.execute_func(self.pet, user_input)
        if success: self._print_success(message)
        else: self._print_error(message)

    # --- Specific Action Implementations ---
    # These methods adapt simple functions to the standard (pet, key) -> (bool, str) signature
    
    def _execute_achievements(self, pet: Pet, key: Optional[str]) -> Tuple[bool, str]:
        """Wrapper for the check_achievements service call."""
        achievements = check_achievements(pet)
        if not achievements:
            return True, "No achievements unlocked yet."
        report = "\n".join([f"🏆 {ach}" for ach in achievements])
        return True, f"Unlocked Achievements:\n{report}"
    
    def _execute_display_stats(self, pet: Pet, key: Optional[str]) -> Tuple[bool, str]:
        """Displays the detailed stats of the pet."""
        self._print_header(f"{pet.name}'s Status Report")
        main_stats = " | ".join([f"{k.capitalize()}: {v}" for k, v in pet.stats.items()])
        print(main_stats)
        print("—" * len(main_stats))
        print(f"Level: {pet.level:<5} Age: {pet.age:<5} Exp: {pet.exp:<5} Coins: {pet.coins}")
        print(f"Evolution: {pet.evolution_stage}")
        if pet.achievements:
            print(f"Achievements: {', '.join(pet.achievements)}")
        return True, "Stats displayed." # Return a silent success

if __name__ == "__main__":
    # --- (I) Mock Pet for Iterative Development & Demonstration ---
    class MockPet:
        def __init__(self):
            self.name = "Sparky"
            self.stats = {'strength': 15, 'agility': 20, 'intellect': 25}
            self.age = 5; self.exp = 1250; self.coins = 300; self.level = 12
            self.evolution_stage = "Adolescent"; self.achievements = ["First Battle Won"]

    print("Welcome to Critter-Craft Advanced Services!")
    menu = AdvancedMenuManager(MockPet())
    menu.run()