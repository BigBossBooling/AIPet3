"""
UI module for the battle system.

This module handles all user-facing output, including rendering health bars,
menus, action descriptions, and battle results.
"""

import time
from typing import Dict, List, Optional, Tuple

from .state import BattlePet, BattleEnvironment, StatusEffect


class BattleUI:
    """Handles all user-facing output for battles."""
    
    def __init__(self, use_color: bool = True, animation_speed: float = 0.5):
        self.use_color = use_color
        self.animation_speed = animation_speed
    
    def clear_screen(self):
        """Clear the terminal screen."""
        print("\033[H\033[J", end="")
    
    def display_battle_start(self, player_pet: BattlePet, opponent_pet: BattlePet, environment: BattleEnvironment):
        """Display the battle start screen."""
        self.clear_screen()
        
        print("=" * 60)
        print(f"BATTLE BEGINS IN THE {environment.name.upper()}!")
        print("=" * 60)
        print(f"{environment.description}")
        print()
        
        print(f"{player_pet.name} (Lvl {player_pet.level}) VS {opponent_pet.name} (Lvl {opponent_pet.level})")
        print()
        
        self.display_stamina_bars(player_pet, opponent_pet)
        
        print("\nPrepare for battle!")
        time.sleep(self.animation_speed * 2)
    
    def display_stamina_bars(self, player_pet: BattlePet, opponent_pet: BattlePet):
        """Display stamina bars for both pets."""
        player_stamina_percent = int(player_pet.current_stamina / player_pet.max_stamina * 100)
        opponent_stamina_percent = int(opponent_pet.current_stamina / opponent_pet.max_stamina * 100)
        
        # Player pet stamina bar
        print(f"{player_pet.name}'s Stamina: {player_pet.current_stamina}/{player_pet.max_stamina}")
        self._draw_progress_bar(player_stamina_percent, 40, "green")
        
        # Opponent pet stamina bar
        print(f"{opponent_pet.name}'s Stamina: {opponent_pet.current_stamina}/{opponent_pet.max_stamina}")
        self._draw_progress_bar(opponent_stamina_percent, 40, "red")
    
    def _draw_progress_bar(self, percent: int, width: int = 40, color: str = "green"):
        """Draw a progress bar with the given percentage."""
        filled_width = int(width * percent / 100)
        empty_width = width - filled_width
        
        if self.use_color:
            if color == "green":
                color_code = "\033[92m"  # Green
            elif color == "red":
                color_code = "\033[91m"  # Red
            elif color == "yellow":
                color_code = "\033[93m"  # Yellow
            else:
                color_code = "\033[0m"   # Default
            
            reset_code = "\033[0m"
            bar = f"[{color_code}{'█' * filled_width}{reset_code}{' ' * empty_width}] {percent}%"
        else:
            bar = f"[{'█' * filled_width}{' ' * empty_width}] {percent}%"
        
        print(bar)
    
    def display_status_effects(self, pet: BattlePet):
        """Display active status effects for a pet."""
        if not pet.status_effects:
            return
        
        print(f"{pet.name}'s Status Effects:")
        for status in pet.status_effects:
            effect_name = status.effect.name.capitalize()
            duration = status.duration
            source = f" ({status.source})" if status.source else ""
            
            if self.use_color:
                if status.effect in [StatusEffect.POISONED, StatusEffect.BURNED, StatusEffect.BLINDED, StatusEffect.SLOWED]:
                    color_code = "\033[91m"  # Red for negative effects
                else:
                    color_code = "\033[92m"  # Green for positive effects
                
                reset_code = "\033[0m"
                print(f"  {color_code}{effect_name}{reset_code} ({duration} turns remaining){source}")
            else:
                print(f"  {effect_name} ({duration} turns remaining){source}")
    
    def display_turn_start(self, active_pet: BattlePet, turn_number: int):
        """Display the start of a pet's turn."""
        print("\n" + "-" * 60)
        print(f"Turn {turn_number}: {active_pet.name}'s turn")
        print(f"AP: {active_pet.current_ap}")
        self.display_status_effects(active_pet)
        print("-" * 60)
    
    def display_action_menu(self, pet: BattlePet, available_abilities: List[str], available_items: List[str]) -> str:
        """
        Display the action menu and get the player's choice.
        
        Returns:
            The player's choice as a string
        """
        print("\nAvailable Actions:")
        
        # Display abilities
        print("Abilities:")
        for i, ability_name in enumerate(available_abilities, 1):
            from .abilities import get_ability
            ability = get_ability(ability_name)
            if ability:
                print(f"  {i}. {ability.name} ({ability.ap_cost} AP) - {ability.description}")
        
        # Display items
        if available_items:
            print("\nItems:")
            for i, item_name in enumerate(available_items, len(available_abilities) + 1):
                from .items import get_item
                item = get_item(item_name)
                if item and hasattr(item, 'ap_cost'):
                    print(f"  {i}. {item.name} ({item.ap_cost} AP) - {item.description}")
        
        # Get player choice
        while True:
            try:
                choice = input("\nEnter your choice (number): ")
                choice_num = int(choice)
                
                if 1 <= choice_num <= len(available_abilities):
                    return available_abilities[choice_num - 1]
                elif len(available_abilities) < choice_num <= len(available_abilities) + len(available_items):
                    return available_items[choice_num - len(available_abilities) - 1]
                else:
                    print("Invalid choice. Please try again.")
            except ValueError:
                print("Please enter a number.")
    
    def display_action_result(self, messages: List[str]):
        """Display the result of an action."""
        for message in messages:
            print(message)
            time.sleep(self.animation_speed)
    
    def display_environment_effects(self, messages: List[str]):
        """Display environment effects."""
        if not messages:
            return
        
        print("\nEnvironment Effects:")
        for message in messages:
            print(f"  {message}")
            time.sleep(self.animation_speed)
    
    def display_battle_end(self, winner: BattlePet, loser: BattlePet, turns_taken: int):
        """Display the battle end screen."""
        print("\n" + "=" * 60)
        print(f"BATTLE OVER! {winner.name} WINS IN {turns_taken} TURNS!")
        print("=" * 60)
        
        print(f"\n{winner.name} has pacified {loser.name}!")
        
        # Display battle statistics
        print("\nBattle Statistics:")
        print(f"  {winner.name} dealt {winner.damage_dealt} total damage")
        print(f"  {loser.name} dealt {loser.damage_dealt} total damage")
        
        print("\nPress Enter to continue...")
        input()
    
    def display_ai_thinking(self, pet_name: str):
        """Display a message indicating the AI is thinking."""
        print(f"\n{pet_name} is considering their next move...")
        time.sleep(self.animation_speed)
    
    def animate_text(self, text: str):
        """Animate text being typed out character by character."""
        for char in text:
            print(char, end="", flush=True)
            time.sleep(self.animation_speed / 10)
        print()
    
    def display_battle_rewards(self, rewards: Dict):
        """Display rewards earned from the battle."""
        print("\nBattle Rewards:")
        
        if "experience" in rewards:
            print(f"  Experience: {rewards['experience']} XP")
        
        if "items" in rewards and rewards["items"]:
            print("  Items:")
            for item in rewards["items"]:
                print(f"    - {item}")
        
        if "research_points" in rewards:
            print(f"  Research Points: {rewards['research_points']}")
        
        if "friendship" in rewards:
            print(f"  Friendship: +{rewards['friendship']}")
        
        print("\nPress Enter to continue...")
        input()"""
UI module for the battle system.

This module handles all user-facing output, including rendering health bars,
menus, action descriptions, and battle results.
"""

import time
from typing import Dict, List, Optional, Tuple

from .state import BattlePet, BattleEnvironment, StatusEffect


class BattleUI:
    """Handles all user-facing output for battles."""
    
    def __init__(self, use_color: bool = True, animation_speed: float = 0.5):
        self.use_color = use_color
        self.animation_speed = animation_speed
    
    def clear_screen(self):
        """Clear the terminal screen."""
        print("\033[H\033[J", end="")
    
    def display_battle_start(self, player_pet: BattlePet, opponent_pet: BattlePet, environment: BattleEnvironment):
        """Display the battle start screen."""
        self.clear_screen()
        
        print("=" * 60)
        print(f"BATTLE BEGINS IN THE {environment.name.upper()}!")
        print("=" * 60)
        print(f"{environment.description}")
        print()
        
        print(f"{player_pet.name} (Lvl {player_pet.level}) VS {opponent_pet.name} (Lvl {opponent_pet.level})")
        print()
        
        self.display_stamina_bars(player_pet, opponent_pet)
        
        print("\nPrepare for battle!")
        time.sleep(self.animation_speed * 2)
    
    def display_stamina_bars(self, player_pet: BattlePet, opponent_pet: BattlePet):
        """Display stamina bars for both pets."""
        player_stamina_percent = int(player_pet.current_stamina / player_pet.max_stamina * 100)
        opponent_stamina_percent = int(opponent_pet.current_stamina / opponent_pet.max_stamina * 100)
        
        # Player pet stamina bar
        print(f"{player_pet.name}'s Stamina: {player_pet.current_stamina}/{player_pet.max_stamina}")
        self._draw_progress_bar(player_stamina_percent, 40, "green")
        
        # Opponent pet stamina bar
        print(f"{opponent_pet.name}'s Stamina: {opponent_pet.current_stamina}/{opponent_pet.max_stamina}")
        self._draw_progress_bar(opponent_stamina_percent, 40, "red")
    
    def _draw_progress_bar(self, percent: int, width: int = 40, color: str = "green"):
        """Draw a progress bar with the given percentage."""
        filled_width = int(width * percent / 100)
        empty_width = width - filled_width
        
        if self.use_color:
            if color == "green":
                color_code = "\033[92m"  # Green
            elif color == "red":
                color_code = "\033[91m"  # Red
            elif color == "yellow":
                color_code = "\033[93m"  # Yellow
            else:
                color_code = "\033[0m"   # Default
            
            reset_code = "\033[0m"
            bar = f"[{color_code}{'█' * filled_width}{reset_code}{' ' * empty_width}] {percent}%"
        else:
            bar = f"[{'█' * filled_width}{' ' * empty_width}] {percent}%"
        
        print(bar)
    
    def display_status_effects(self, pet: BattlePet):
        """Display active status effects for a pet."""
        if not pet.status_effects:
            return
        
        print(f"{pet.name}'s Status Effects:")
        for status in pet.status_effects:
            effect_name = status.effect.name.capitalize()
            duration = status.duration
            source = f" ({status.source})" if status.source else ""
            
            if self.use_color:
                if status.effect in [StatusEffect.POISONED, StatusEffect.BURNED, StatusEffect.BLINDED, StatusEffect.SLOWED]:
                    color_code = "\033[91m"  # Red for negative effects
                else:
                    color_code = "\033[92m"  # Green for positive effects
                
                reset_code = "\033[0m"
                print(f"  {color_code}{effect_name}{reset_code} ({duration} turns remaining){source}")
            else:
                print(f"  {effect_name} ({duration} turns remaining){source}")
    
    def display_turn_start(self, active_pet: BattlePet, turn_number: int):
        """Display the start of a pet's turn."""
        print("\n" + "-" * 60)
        print(f"Turn {turn_number}: {active_pet.name}'s turn")
        print(f"AP: {active_pet.current_ap}")
        self.display_status_effects(active_pet)
        print("-" * 60)
    
    def display_action_menu(self, pet: BattlePet, available_abilities: List[str], available_items: List[str]) -> str:
        """
        Display the action menu and get the player's choice.
        
        Returns:
            The player's choice as a string
        """
        print("\nAvailable Actions:")
        
        # Display abilities
        print("Abilities:")
        for i, ability_name in enumerate(available_abilities, 1):
            from .abilities import get_ability
            ability = get_ability(ability_name)
            if ability:
                print(f"  {i}. {ability.name} ({ability.ap_cost} AP) - {ability.description}")
        
        # Display items
        if available_items:
            print("\nItems:")
            for i, item_name in enumerate(available_items, len(available_abilities) + 1):
                from .items import get_item
                item = get_item(item_name)
                if item and hasattr(item, 'ap_cost'):
                    print(f"  {i}. {item.name} ({item.ap_cost} AP) - {item.description}")
        
        # Get player choice
        while True:
            try:
                choice = input("\nEnter your choice (number): ")
                choice_num = int(choice)
                
                if 1 <= choice_num <= len(available_abilities):
                    return available_abilities[choice_num - 1]
                elif len(available_abilities) < choice_num <= len(available_abilities) + len(available_items):
                    return available_items[choice_num - len(available_abilities) - 1]
                else:
                    print("Invalid choice. Please try again.")
            except ValueError:
                print("Please enter a number.")
    
    def display_action_result(self, messages: List[str]):
        """Display the result of an action."""
        for message in messages:
            print(message)
            time.sleep(self.animation_speed)
    
    def display_environment_effects(self, messages: List[str]):
        """Display environment effects."""
        if not messages:
            return
        
        print("\nEnvironment Effects:")
        for message in messages:
            print(f"  {message}")
            time.sleep(self.animation_speed)
    
    def display_battle_end(self, winner: BattlePet, loser: BattlePet, turns_taken: int):
        """Display the battle end screen."""
        print("\n" + "=" * 60)
        print(f"BATTLE OVER! {winner.name} WINS IN {turns_taken} TURNS!")
        print("=" * 60)
        
        print(f"\n{winner.name} has pacified {loser.name}!")
        
        # Display battle statistics
        print("\nBattle Statistics:")
        print(f"  {winner.name} dealt {winner.damage_dealt} total damage")
        print(f"  {loser.name} dealt {loser.damage_dealt} total damage")
        
        print("\nPress Enter to continue...")
        input()
    
    def display_ai_thinking(self, pet_name: str):
        """Display a message indicating the AI is thinking."""
        print(f"\n{pet_name} is considering their next move...")
        time.sleep(self.animation_speed)
    
    def animate_text(self, text: str):
        """Animate text being typed out character by character."""
        for char in text:
            print(char, end="", flush=True)
            time.sleep(self.animation_speed / 10)
        print()
    
    def display_battle_rewards(self, rewards: Dict):
        """Display rewards earned from the battle."""
        print("\nBattle Rewards:")
        
        if "experience" in rewards:
            print(f"  Experience: {rewards['experience']} XP")
        
        if "items" in rewards and rewards["items"]:
            print("  Items:")
            for item in rewards["items"]:
                print(f"    - {item}")
        
        if "research_points" in rewards:
            print(f"  Research Points: {rewards['research_points']}")
        
        if "friendship" in rewards:
            print(f"  Friendship: +{rewards['friendship']}")
        
        print("\nPress Enter to continue...")
        input()"""
UI module for the battle system.

This module handles all user-facing output, including rendering health bars,
menus, action descriptions, and battle results.
"""

import time
from typing import Dict, List, Optional, Tuple

from .state import BattlePet, BattleEnvironment, StatusEffect


class BattleUI:
    """Handles all user-facing output for battles."""
    
    def __init__(self, use_color: bool = True, animation_speed: float = 0.5):
        self.use_color = use_color
        self.animation_speed = animation_speed
    
    def clear_screen(self):
        """Clear the terminal screen."""
        print("\033[H\033[J", end="")
    
    def display_battle_start(self, player_pet: BattlePet, opponent_pet: BattlePet, environment: BattleEnvironment):
        """Display the battle start screen."""
        self.clear_screen()
        
        print("=" * 60)
        print(f"BATTLE BEGINS IN THE {environment.name.upper()}!")
        print("=" * 60)
        print(f"{environment.description}")
        print()
        
        print(f"{player_pet.name} (Lvl {player_pet.level}) VS {opponent_pet.name} (Lvl {opponent_pet.level})")
        print()
        
        self.display_stamina_bars(player_pet, opponent_pet)
        
        print("\nPrepare for battle!")
        time.sleep(self.animation_speed * 2)
    
    def display_stamina_bars(self, player_pet: BattlePet, opponent_pet: BattlePet):
        """Display stamina bars for both pets."""
        player_stamina_percent = int(player_pet.current_stamina / player_pet.max_stamina * 100)
        opponent_stamina_percent = int(opponent_pet.current_stamina / opponent_pet.max_stamina * 100)
        
        # Player pet stamina bar
        print(f"{player_pet.name}'s Stamina: {player_pet.current_stamina}/{player_pet.max_stamina}")
        self._draw_progress_bar(player_stamina_percent, 40, "green")
        
        # Opponent pet stamina bar
        print(f"{opponent_pet.name}'s Stamina: {opponent_pet.current_stamina}/{opponent_pet.max_stamina}")
        self._draw_progress_bar(opponent_stamina_percent, 40, "red")
    
    def _draw_progress_bar(self, percent: int, width: int = 40, color: str = "green"):
        """Draw a progress bar with the given percentage."""
        filled_width = int(width * percent / 100)
        empty_width = width - filled_width
        
        if self.use_color:
            if color == "green":
                color_code = "\033[92m"  # Green
            elif color == "red":
                color_code = "\033[91m"  # Red
            elif color == "yellow":
                color_code = "\033[93m"  # Yellow
            else:
                color_code = "\033[0m"   # Default
            
            reset_code = "\033[0m"
            bar = f"[{color_code}{'█' * filled_width}{reset_code}{' ' * empty_width}] {percent}%"
        else:
            bar = f"[{'█' * filled_width}{' ' * empty_width}] {percent}%"
        
        print(bar)
    
    def display_status_effects(self, pet: BattlePet):
        """Display active status effects for a pet."""
        if not pet.status_effects:
            return
        
        print(f"{pet.name}'s Status Effects:")
        for status in pet.status_effects:
            effect_name = status.effect.name.capitalize()
            duration = status.duration
            source = f" ({status.source})" if status.source else ""
            
            if self.use_color:
                if status.effect in [StatusEffect.POISONED, StatusEffect.BURNED, StatusEffect.BLINDED, StatusEffect.SLOWED]:
                    color_code = "\033[91m"  # Red for negative effects
                else:
                    color_code = "\033[92m"  # Green for positive effects
                
                reset_code = "\033[0m"
                print(f"  {color_code}{effect_name}{reset_code} ({duration} turns remaining){source}")
            else:
                print(f"  {effect_name} ({duration} turns remaining){source}")
    
    def display_turn_start(self, active_pet: BattlePet, turn_number: int):
        """Display the start of a pet's turn."""
        print("\n" + "-" * 60)
        print(f"Turn {turn_number}: {active_pet.name}'s turn")
        print(f"AP: {active_pet.current_ap}")
        self.display_status_effects(active_pet)
        print("-" * 60)
    
    def display_action_menu(self, pet: BattlePet, available_abilities: List[str], available_items: List[str]) -> str:
        """
        Display the action menu and get the player's choice.
        
        Returns:
            The player's choice as a string
        """
        print("\nAvailable Actions:")
        
        # Display abilities
        print("Abilities:")
        for i, ability_name in enumerate(available_abilities, 1):
            from .abilities import get_ability
            ability = get_ability(ability_name)
            if ability:
                print(f"  {i}. {ability.name} ({ability.ap_cost} AP) - {ability.description}")
        
        # Display items
        if available_items:
            print("\nItems:")
            for i, item_name in enumerate(available_items, len(available_abilities) + 1):
                from .items import get_item
                item = get_item(item_name)
                if item and hasattr(item, 'ap_cost'):
                    print(f"  {i}. {item.name} ({item.ap_cost} AP) - {item.description}")
        
        # Get player choice
        while True:
            try:
                choice = input("\nEnter your choice (number): ")
                choice_num = int(choice)
                
                if 1 <= choice_num <= len(available_abilities):
                    return available_abilities[choice_num - 1]
                elif len(available_abilities) < choice_num <= len(available_abilities) + len(available_items):
                    return available_items[choice_num - len(available_abilities) - 1]
                else:
                    print("Invalid choice. Please try again.")
            except ValueError:
                print("Please enter a number.")
    
    def display_action_result(self, messages: List[str]):
        """Display the result of an action."""
        for message in messages:
            print(message)
            time.sleep(self.animation_speed)
    
    def display_environment_effects(self, messages: List[str]):
        """Display environment effects."""
        if not messages:
            return
        
        print("\nEnvironment Effects:")
        for message in messages:
            print(f"  {message}")
            time.sleep(self.animation_speed)
    
    def display_battle_end(self, winner: BattlePet, loser: BattlePet, turns_taken: int):
        """Display the battle end screen."""
        print("\n" + "=" * 60)
        print(f"BATTLE OVER! {winner.name} WINS IN {turns_taken} TURNS!")
        print("=" * 60)
        
        print(f"\n{winner.name} has pacified {loser.name}!")
        
        # Display battle statistics
        print("\nBattle Statistics:")
        print(f"  {winner.name} dealt {winner.damage_dealt} total damage")
        print(f"  {loser.name} dealt {loser.damage_dealt} total damage")
        
        print("\nPress Enter to continue...")
        input()
    
    def display_ai_thinking(self, pet_name: str):
        """Display a message indicating the AI is thinking."""
        print(f"\n{pet_name} is considering their next move...")
        time.sleep(self.animation_speed)
    
    def animate_text(self, text: str):
        """Animate text being typed out character by character."""
        for char in text:
            print(char, end="", flush=True)
            time.sleep(self.animation_speed / 10)
        print()
    
    def display_battle_rewards(self, rewards: Dict):
        """Display rewards earned from the battle."""
        print("\nBattle Rewards:")
        
        if "experience" in rewards:
            print(f"  Experience: {rewards['experience']} XP")
        
        if "items" in rewards and rewards["items"]:
            print("  Items:")
            for item in rewards["items"]:
                print(f"    - {item}")
        
        if "research_points" in rewards:
            print(f"  Research Points: {rewards['research_points']}")
        
        if "friendship" in rewards:
            print(f"  Friendship: +{rewards['friendship']}")
        
        print("\nPress Enter to continue...")
        input()