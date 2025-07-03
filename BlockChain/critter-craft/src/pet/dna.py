"""
This file manages DNA traits and mutations for pets, including trait generation and mutation effects.
"""

import random

class DNA:
    def __init__(self, species):
        self.species = species
        self.traits = self.generate_traits()
        self.mutations = []

    def generate_traits(self):
        traits = {}
        if self.species == "Glowing Sprite":
            traits = {
                "luminosity": random.choice(["bright", "moderate", "dim"]),
                "color_pattern": random.choice(["solid", "spotted", "striped", "swirled"]),
                "energy_type": random.choice(["solar", "lunar", "stellar", "ambient"]),
                "temperament": random.choice(["calm", "energetic", "curious", "shy"])
            }
        elif self.species == "Crystal Sprite":
            traits = {
                "crystal_type": random.choice(["quartz", "amethyst", "emerald", "sapphire", "ruby"]),
                "formation": random.choice(["geometric", "prismatic", "jagged", "smooth"]),
                "resonance": random.choice(["harmonic", "discordant", "pulsing", "stable"]),
                "transparency": random.choice(["clear", "translucent", "opaque", "variable"])
            }
        elif self.species == "Shadow Sprite":
            traits = {
                "shadow_density": random.choice(["thick", "wispy", "shifting", "solid"]),
                "darkness_level": random.choice(["deep", "moderate", "light", "variable"]),
                "manifestation": random.choice(["corporeal", "ethereal", "flickering", "stable"]),
                "aura": random.choice(["absorbing", "projecting", "neutral", "pulsing"])
            }
        elif self.species == "Ember Sprite":
            traits = {
                "flame_type": random.choice(["steady", "flickering", "explosive", "smoldering"]),
                "heat_level": random.choice(["scorching", "warm", "cool-burning", "variable"]),
                "fuel_source": random.choice(["emotional", "physical", "spiritual", "ambient"]),
                "burn_pattern": random.choice(["constant", "pulsing", "erratic", "controlled"])
            }
        elif self.species == "Aqua Sprite":
            traits = {
                "water_state": random.choice(["flowing", "misting", "droplet", "wave"]),
                "clarity": random.choice(["crystal", "cloudy", "reflective", "refractive"]),
                "current": random.choice(["calm", "turbulent", "swirling", "tidal"]),
                "composition": random.choice(["pure", "mineral-rich", "saline", "adaptive"])
            }
        return traits

    def mutate(self):
        mutation = random.choice(DNA_MUTATIONS)
        self.mutations.append(mutation)
        for stat, change in mutation["stat_changes"].items():
            if stat in self.traits:
                self.traits[stat] += change

    def __str__(self):
        return f"Species: {self.species}, Traits: {self.traits}, Mutations: {self.mutations}"

DNA_MUTATIONS = [
    {
        "name": "Elemental Fusion",
        "effect": "Combines two elemental properties for unique abilities",
        "stat_changes": {"iq": 5, "energy": 5}
    },
    {
        "name": "Chromatic Shift",
        "effect": "Can change colors based on mood or environment",
        "stat_changes": {"charisma": 10}
    },
    {
        "name": "Ethereal Form",
        "effect": "Can temporarily become incorporeal",
        "stat_changes": {"energy": 10}
    },
    {
        "name": "Resonant Voice",
        "effect": "Voice carries special harmonic properties",
        "stat_changes": {"charisma": 5, "social": 5}
    },
    {
        "name": "Adaptive Metabolism",
        "effect": "Can go longer without food or rest",
        "stat_changes": {"hunger": -10, "energy": 5}
    },
    {
        "name": "Enhanced Cognition",
        "effect": "Faster learning and problem-solving abilities",
        "stat_changes": {"iq": 15}
    },
    {
        "name": "Empathic Bond",
        "effect": "Can sense and influence the emotions of others",
        "stat_changes": {"social": 10, "charisma": 5}
    },
    {
        "name": "Regenerative Tissue",
        "effect": "Recovers more quickly from exhaustion",
        "stat_changes": {"energy": 10, "happiness": 5}
    }
]
