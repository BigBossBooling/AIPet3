"""
Battle service for managing battle-related functionalities in the Critter-Craft project.
"""

from src.pet.battles import BATTLE_OPPONENTS
import random

class BattleService:
    def __init__(self):
        self.current_battle = None

    def initiate_battle(self, pet, opponent_type):
        if opponent_type not in BATTLE_OPPONENTS:
            raise ValueError("Invalid opponent type.")
        
        opponent = BATTLE_OPPONENTS[opponent_type]
        self.current_battle = {
            "pet": pet,
            "opponent": opponent,
            "pet_health": pet.stats['health'],
            "opponent_health": opponent['power']
        }
        return self.current_battle

    def calculate_outcome(self):
        if not self.current_battle:
            raise ValueError("No battle initiated.")
        
        pet = self.current_battle['pet']
        opponent = self.current_battle['opponent']
        
        pet_attack = random.randint(1, pet.stats['attack'])
        opponent_attack = random.randint(1, opponent['power'])
        
        self.current_battle['pet_health'] -= opponent_attack
        self.current_battle['opponent_health'] -= pet_attack
        
        if self.current_battle['pet_health'] <= 0:
            return "Pet has been defeated."
        elif self.current_battle['opponent_health'] <= 0:
            return "Opponent has been defeated."
        else:
            return "Battle is ongoing."

    def reset_battle(self):
        self.current_battle = None
"""