"""
This module implements battle mechanics for the Critter-Craft project.

Classes:
- Battle: Handles battle initiation, opponent selection, and battle outcomes.

Functions:
- initiate_battle: Starts a battle between a pet and an opponent.
- select_opponent: Randomly selects an opponent for the battle.
- calculate_outcome: Determines the outcome of the battle based on pet and opponent stats.
"""

import random
from src.pet.advanced_constants import BATTLE_OPPONENTS

class Battle:
    def __init__(self, pet):
        self.pet = pet
        self.opponent = None
        self.outcome = None

    def initiate_battle(self):
        self.opponent = self.select_opponent()
        self.outcome = self.calculate_outcome()

    def select_opponent(self):
        opponent_key = random.choice(list(BATTLE_OPPONENTS.keys()))
        return BATTLE_OPPONENTS[opponent_key]

    def calculate_outcome(self):
        pet_power = self.pet.stats['power']  # Assuming pet has a stats dictionary
        opponent_power = self.opponent['power']
        
        if pet_power >= opponent_power:
            return "Victory"
        else:
            return "Defeat"