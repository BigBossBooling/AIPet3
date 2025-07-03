"""
DNA Service for managing DNA traits and mutations for pets in the Critter-Craft project.
"""

from src.pet.advanced_constants import DNA_TRAITS, DNA_MUTATIONS
import random

class DNAService:
    def __init__(self):
        self.traits = {}

    def generate_traits(self, pet_type):
        if pet_type not in DNA_TRAITS:
            raise ValueError(f"Unknown pet type: {pet_type}")
        
        self.traits[pet_type] = {
            trait: random.choice(options)
            for trait, options in DNA_TRAITS[pet_type].items()
        }
        return self.traits[pet_type]

    def apply_mutation(self, pet_traits):
        mutation = random.choice(DNA_MUTATIONS)
        for stat, change in mutation["stat_changes"].items():
            if stat in pet_traits:
                pet_traits[stat] += change
        return mutation["name"], pet_traits

    def get_traits(self, pet_type):
        return self.traits.get(pet_type, None)