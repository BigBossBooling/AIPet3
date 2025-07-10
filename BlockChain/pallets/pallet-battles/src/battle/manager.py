"""
Battle manager module for the battle system.

This module contains the BattleManager class, which is responsible for
managing the state of an active battle, processing actions, and determining
the outcome.
"""

import random
from typing import Dict, List, Optional, Tuple, Union

from .abilities import get_ability, ABILITY_MAPPING
from .formulas import (
    apply_status_effect_damage,
    calculate_turn_order,
)
from .items import get_item
from .state import BattlePet, BattleEnvironment, StatusEffect
from .ui import BattleUI


class BattleManager:
    """Manages the state and flow of a battle."""
    
    def __init__(
        self,
        player_pet: Dict,
        opponent_pet: Dict,
        environment_type: str,
        items: List[str] = None,
        ui: Optional[BattleUI] = None
    ):
        """
        Initialize a new battle.
        
        Args:
            player_pet: The player's pet data
            opponent_pet: The opponent's pet data
            environment_type: The type of environment for the battle
            items: List of item names the player has available
            ui: Optional UI instance for rendering the battle
        """
        self.player_battle_pet = self._create_battle_pet(player_pet)
        self.opponent_battle_pet = self._create_battle_pet(opponent_pet)
        self.environment = self._create_environment(environment_type)
        self.player_items = items or []
        self.ui = ui or BattleUI()
        
        self.turn_number = 1
        self.active_pet = None
        self.battle_log = []
        self.battle_result = None
    
    def _create_battle_pet(self, pet_data: Dict) -> BattlePet:
        """
        Create a BattlePet instance from pet data.
        
        Args:
            pet_data: Dictionary containing pet data
            
        Returns:
            A BattlePet instance
        """
        # Extract basic info
        name = pet_data.get("name", "Unknown Pet")
        species = pet_data.get("species", "Unknown Species")
        level = pet_data.get("level", 1)
        
        # Create battle pet with default values
        battle_pet = BattlePet(
            name=name,
            species=species,
            level=level
        )
        
        # Set stats based on level and species
        battle_pet.max_stamina = 80 + (level * 5)
        battle_pet.current_stamina = battle_pet.max_stamina
        battle_pet.attack = 8 + (level * 2)
        battle_pet.defense = 8 + (level * 2)
        battle_pet.speed = 8 + (level * 2)
        
        # Add adaptations
        adaptations = pet_data.get("adaptations", [])
        battle_pet.adaptations = adaptations
        
        # Apply species-specific modifiers
        if species == "Chameleon":
            battle_pet.evasion += 5
            if "camouflage" not in battle_pet.adaptations:
                battle_pet.adaptations.append("camouflage")
        
        elif species == "Anglerfish":
            battle_pet.attack += 3
            if "bioluminescence" not in battle_pet.adaptations:
                battle_pet.adaptations.append("bioluminescence")
        
        elif species == "Peacock Spider":
            battle_pet.speed += 5
            if "colorful_display" not in battle_pet.adaptations:
                battle_pet.adaptations.append("colorful_display")
        
        # Always add basic maneuver
        if "basic_maneuver" not in battle_pet.adaptations:
            battle_pet.adaptations.append("basic_maneuver")
        
        # Always add defend
        if "defend" not in battle_pet.adaptations:
            battle_pet.adaptations.append("defend")
        
        return battle_pet
    
    def _create_environment(self, environment_type: str) -> BattleEnvironment:
        """
        Create a BattleEnvironment instance based on the environment type.
        
        Args:
            environment_type: The type of environment
            
        Returns:
            A BattleEnvironment instance
        """
        if environment_type == "forest":
            return BattleEnvironment(
                name="Sun-Dappled Forest",
                description="A lush forest with dappled sunlight filtering through the canopy.",
                effects={"evasion_bonus": 10, "camouflage_boost": 50},
                available_actions=["take_cover", "climb_tree"]
            )
        
        elif environment_type == "swamp":
            return BattleEnvironment(
                name="Murky Swamp",
                description="A dark, murky swamp with thick mud and strange sounds.",
                effects={"non_aquatic_slow_chance": 25},
                available_actions=["dive", "hide_in_mud"]
            )
        
        elif environment_type == "vents":
            return BattleEnvironment(
                name="Geothermal Vents",
                description="A hot area with steaming vents and bubbling pools.",
                effects={"fire_boost": 20, "non_fire_burn": True},
                available_actions=["vent_burst", "steam_cloud"]
            )
        
        elif environment_type == "cavern":
            return BattleEnvironment(
                name="Crystal Cavern",
                description="A dark cavern filled with glowing crystals that amplify light and sound.",
                effects={"bioluminescence_boost": 30, "echolocation_boost": 30},
                available_actions=["crystal_reflect", "sound_amplify"]
            )
        
        else:
            # Default to a neutral environment
            return BattleEnvironment(
                name="Neutral Ground",
                description="A balanced environment with no special effects.",
                effects={},
                available_actions=[]
            )
    
    def run_battle(self) -> Dict:
        """
        Run the battle from start to finish.
        
        Returns:
            A dictionary containing the battle result
        """
        # Display battle start
        self.ui.display_battle_start(
            self.player_battle_pet,
            self.opponent_battle_pet,
            self.environment
        )
        
        # Main battle loop
        while True:
            # Check if battle is over
            if self.player_battle_pet.is_pacified():
                self.battle_result = {
                    "winner": "opponent",
                    "turns_taken": self.turn_number - 1
                }
                break
            
            if self.opponent_battle_pet.is_pacified():
                self.battle_result = {
                    "winner": "player",
                    "turns_taken": self.turn_number - 1
                }
                break
            
            # Determine turn order
            pets_in_order = calculate_turn_order([self.player_battle_pet, self.opponent_battle_pet])
            
            # Process each pet's turn
            for pet in pets_in_order:
                if pet.is_pacified():
                    continue
                
                self.active_pet = pet
                
                # Reset AP for this turn
                pet.current_ap = pet.get_ap_for_turn()
                
                # Display turn start
                self.ui.display_turn_start(pet, self.turn_number)
                
                # Process the turn
                if pet == self.player_battle_pet:
                    self._process_player_turn()
                else:
                    self._process_ai_turn()
                
                # Apply status effect damage
                damage, messages = apply_status_effect_damage(pet)
                if messages:
                    self.ui.display_action_result(messages)
                
                # Update status effects
                pet.update_status_effects()
                
                # Check if battle is over after this pet's turn
                if self.player_battle_pet.is_pacified() or self.opponent_battle_pet.is_pacified():
                    break
            
            # Apply environment effects
            self._apply_environment_effects()
            
            # Increment turn number
            self.turn_number += 1
        
        # Display battle end
        winner = self.player_battle_pet if self.battle_result["winner"] == "player" else self.opponent_battle_pet
        loser = self.opponent_battle_pet if self.battle_result["winner"] == "player" else self.player_battle_pet
        
        self.ui.display_battle_end(winner, loser, self.battle_result["turns_taken"])
        
        # Calculate and display rewards
        if self.battle_result["winner"] == "player":
            rewards = self._calculate_rewards()
            self.ui.display_battle_rewards(rewards)
            self.battle_result["rewards"] = rewards
        
        return self.battle_result
    
    def _process_player_turn(self):
        """Process the player's turn."""
        # Get available abilities
        available_abilities = [
            ability_name for ability_name in self.player_battle_pet.adaptations
            if ability_name in ABILITY_MAPPING
        ]
        
        # Get available items
        available_items = self.player_items.copy()
        
        # Keep processing actions until the player is out of AP or chooses to end turn
        while self.player_battle_pet.current_ap > 0:
            # Display action menu and get player choice
            choice = self.ui.display_action_menu(
                self.player_battle_pet,
                available_abilities,
                available_items
            )
            
            # Process the chosen action
            if choice in available_abilities:
                ability = get_ability(choice)
                if ability and ability.can_use(self.player_battle_pet):
                    messages, result = ability.execute(self.player_battle_pet, self.opponent_battle_pet)
                    self.ui.display_action_result(messages)
                    
                    # Log the action
                    self.battle_log.append({
                        "turn": self.turn_number,
                        "pet": self.player_battle_pet.name,
                        "action": f"Used ability: {ability.name}",
                        "result": result
                    })
                    
                    # Check if opponent is pacified
                    if self.opponent_battle_pet.is_pacified():
                        break
            
            elif choice in available_items:
                item = get_item(choice)
                if item and hasattr(item, 'use') and item.can_use(self.player_battle_pet):
                    # Remove the item from available items
                    available_items.remove(choice)
                    self.player_items.remove(choice)
                    
                    # Use the item
                    messages, result = item.use(self.player_battle_pet, self.opponent_battle_pet)
                    self.ui.display_action_result(messages)
                    
                    # Log the action
                    self.battle_log.append({
                        "turn": self.turn_number,
                        "pet": self.player_battle_pet.name,
                        "action": f"Used item: {item.name}",
                        "result": result
                    })
            
            # Ask if the player wants to end their turn
            if self.player_battle_pet.current_ap > 0:
                end_turn = input(f"\nYou have {self.player_battle_pet.current_ap} AP left. End turn? (y/n): ")
                if end_turn.lower() in ['y', 'yes']:
                    break
    
    def _process_ai_turn(self):
        """Process the AI opponent's turn."""
        self.ui.display_ai_thinking(self.opponent_battle_pet.name)
        
        # Get available abilities
        available_abilities = [
            ability_name for ability_name in self.opponent_battle_pet.adaptations
            if ability_name in ABILITY_MAPPING
        ]
        
        # Simple AI decision making
        while self.opponent_battle_pet.current_ap > 0:
            # Choose an ability
            ability_name = self._choose_ai_ability(available_abilities)
            ability = get_ability(ability_name)
            
            if ability and ability.can_use(self.opponent_battle_pet):
                messages, result = ability.execute(self.opponent_battle_pet, self.player_battle_pet)
                self.ui.display_action_result(messages)
                
                # Log the action
                self.battle_log.append({
                    "turn": self.turn_number,
                    "pet": self.opponent_battle_pet.name,
                    "action": f"Used ability: {ability.name}",
                    "result": result
                })
                
                # Check if player is pacified
                if self.player_battle_pet.is_pacified():
                    break
            
            # 50% chance to end turn early if below half AP
            if self.opponent_battle_pet.current_ap < self.opponent_battle_pet.base_ap_per_turn / 2:
                if random.random() < 0.5:
                    break
    
    def _choose_ai_ability(self, available_abilities: List[str]) -> str:
        """
        Choose an ability for the AI to use.
        
        Args:
            available_abilities: List of available ability names
            
        Returns:
            The chosen ability name
        """
        # Get abilities with their instances
        abilities = [(name, get_ability(name)) for name in available_abilities]
        
        # Filter out abilities that can't be used
        usable_abilities = [(name, ability) for name, ability in abilities 
                           if ability and ability.can_use(self.opponent_battle_pet)]
        
        if not usable_abilities:
            return "defend"  # Default to defend if nothing else is usable
        
        # Simple decision making based on current situation
        
        # If low on stamina, prioritize defensive abilities
        if self.opponent_battle_pet.current_stamina < self.opponent_battle_pet.max_stamina * 0.3:
            for name, ability in usable_abilities:
                if name == "defend" or name == "camouflage":
                    return name
        
        # If player has status effects, consider using abilities that exploit them
        player_has_blinded = any(s.effect == StatusEffect.BLINDED for s in self.player_battle_pet.status_effects)
        if player_has_blinded:
            for name, ability in usable_abilities:
                if name == "venom_strike":  # Good to use when opponent has low accuracy
                    return name
        
        # If player is close to being pacified, prioritize damage abilities
        if self.player_battle_pet.current_stamina < self.player_battle_pet.max_stamina * 0.2:
            for name, ability in usable_abilities:
                if name == "basic_maneuver" or name == "venom_strike":
                    return name
        
        # Otherwise, choose randomly with some weighting
        weights = [
            (1 if name == "basic_maneuver" else 2 if name == "defend" else ability.ap_cost * 2)
            for name, ability in usable_abilities
        ]
        
        # Normalize weights
        total_weight = sum(weights)
        if total_weight > 0:
            weights = [w / total_weight for w in weights]
            
            # Choose based on weights
            return random.choices([name for name, _ in usable_abilities], weights=weights, k=1)[0]
        else:
            # If weights are all zero, choose randomly
            return random.choice([name for name, _ in usable_abilities])
    
    def _apply_environment_effects(self):
        """Apply environment effects to both pets."""
        # Apply to player pet
        player_messages = self.environment.apply_environment_effects(
            self.player_battle_pet,
            self.turn_number
        )
        
        # Apply to opponent pet
        opponent_messages = self.environment.apply_environment_effects(
            self.opponent_battle_pet,
            self.turn_number
        )
        
        # Display messages
        all_messages = player_messages + opponent_messages
        if all_messages:
            self.ui.display_environment_effects(all_messages)
    
    def _calculate_rewards(self) -> Dict:
        """
        Calculate rewards for winning a battle.
        
        Returns:
            A dictionary of rewards
        """
        rewards = {
            "experience": 10 * self.opponent_battle_pet.level,
            "research_points": 5 * self.opponent_battle_pet.level,
        }
        
        # Chance to get items
        if random.random() < 0.3:  # 30% chance
            possible_items = ["healing_salve", "focus_root", "thick_mud"]
            if random.random() < 0.2:  # 20% chance for rarer items
                possible_items.extend(["adrenaline_berry", "polished_river_stone"])
            
            num_items = random.randint(1, 2)
            rewards["items"] = random.sample(possible_items, min(num_items, len(possible_items)))
        else:
            rewards["items"] = []
        
        # Friendship with your pet increases after a successful battle
        rewards["friendship"] = random.randint(1, 3)
        
        return rewards