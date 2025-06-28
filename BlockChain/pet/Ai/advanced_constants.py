"""
Constants for advanced pet features including jobs, battles, quests, education, evolution, and DNA.
"""

# --- Job System Constants ---
JOB_TYPES = {
    "teacher": {
        "display_name": "Teacher",
        "description": "Educate young pets and help them grow.",
        "requirements": {
            "min_stats": {"iq": 40, "charisma": 30},
            "min_age": 5
        },
        "base_salary": 10,
        "exp_per_work": 15,
        "skills": ["teaching", "patience", "knowledge"]
    },
    "guard": {
        "display_name": "Guard",
        "description": "Protect important locations and maintain security.",
        "requirements": {
            "min_stats": {"energy": 50},
            "min_age": 3
        },
        "base_salary": 12,
        "exp_per_work": 10,
        "skills": ["vigilance", "strength", "intimidation"]
    },
    "healer": {
        "display_name": "Healer",
        "description": "Help injured and sick pets recover.",
        "requirements": {
            "min_stats": {"iq": 35, "social": 40},
            "min_age": 4
        },
        "base_salary": 15,
        "exp_per_work": 12,
        "skills": ["diagnosis", "treatment", "bedside_manner"]
    },
    "explorer": {
        "display_name": "Explorer",
        "description": "Discover new locations and treasures.",
        "requirements": {
            "min_stats": {"energy": 45, "happiness": 40},
            "min_age": 2
        },
        "base_salary": 8,
        "exp_per_work": 20,
        "skills": ["navigation", "survival", "observation"]
    },
    "entertainer": {
        "display_name": "Entertainer",
        "description": "Bring joy and laughter to other pets.",
        "requirements": {
            "min_stats": {"charisma": 45, "happiness": 50},
            "min_age": 1
        },
        "base_salary": 9,
        "exp_per_work": 18,
        "skills": ["performance", "creativity", "audience_connection"]
    }
}

# --- Battle System Constants ---
BATTLE_OPPONENTS = {
    "training_dummy": {
        "display_name": "Training Dummy",
        "description": "A basic opponent for practice.",
        "power": 30,
        "reward": 5,
        "abilities": ["basic_strike"]
    },
    "forest_sprite": {
        "display_name": "Forest Sprite",
        "description": "A mischievous nature spirit.",
        "power": 45,
        "reward": 10,
        "abilities": ["nature_touch", "leaf_shield"]
    },
    "shadow_lurker": {
        "display_name": "Shadow Lurker",
        "description": "A mysterious creature that hides in darkness.",
        "power": 60,
        "reward": 15,
        "abilities": ["shadow_cloak", "night_strike"]
    },
    "crystal_guardian": {
        "display_name": "Crystal Guardian",
        "description": "A powerful protector made of living crystal.",
        "power": 80,
        "reward": 20,
        "abilities": ["crystal_beam", "reflective_shield"]
    },
    "elder_elemental": {
        "display_name": "Elder Elemental",
        "description": "An ancient being of pure elemental energy.",
        "power": 100,
        "reward": 30,
        "abilities": ["elemental_surge", "primal_force", "energy_drain"]
    }
}

# --- Quest System Constants ---
AVAILABLE_QUESTS = {
    "welcome_quest": {
        "name": "Welcome to CritterCraft",
        "description": "Learn the basics of pet care and interaction.",
        "requirements": {
            "min_maturity": 0
        },
        "required_progress": 5,
        "reward_points": 5,
        "faction": "crittercraft"
    },
    "forest_exploration": {
        "name": "Forest Exploration",
        "description": "Explore the mysterious forest and discover its secrets.",
        "requirements": {
            "min_maturity": 10
        },
        "required_progress": 10,
        "reward_points": 10,
        "faction": "explorers"
    },
    "crystal_collection": {
        "name": "Crystal Collection",
        "description": "Collect rare crystals from the crystal caves.",
        "requirements": {
            "min_maturity": 20
        },
        "required_progress": 15,
        "reward_points": 15,
        "faction": "collectors"
    },
    "shadow_investigation": {
        "name": "Shadow Investigation",
        "description": "Investigate the strange shadows appearing in town.",
        "requirements": {
            "min_maturity": 30
        },
        "required_progress": 20,
        "reward_points": 20,
        "faction": "guardians"
    },
    "elemental_harmony": {
        "name": "Elemental Harmony",
        "description": "Restore balance to the elemental shrines.",
        "requirements": {
            "min_maturity": 50
        },
        "required_progress": 30,
        "reward_points": 30,
        "faction": "elementalists"
    }
}

# --- Education System Constants ---
EDUCATION_SUBJECTS = [
    "mathematics",
    "language",
    "science",
    "history",
    "arts",
    "physical_education",
    "magic_theory",
    "elemental_studies",
    "creature_biology",
    "alchemy"
]

EDUCATION_DEGREES = {
    "basic_education": {
        "display_name": "Basic Education Certificate",
        "requirements": {
            "mathematics": 20,
            "language": 20,
            "science": 20
        },
        "level_increase": 1
    },
    "natural_sciences": {
        "display_name": "Natural Sciences Degree",
        "requirements": {
            "science": 50,
            "creature_biology": 40,
            "mathematics": 30
        },
        "level_increase": 2
    },
    "arcane_arts": {
        "display_name": "Arcane Arts Degree",
        "requirements": {
            "magic_theory": 50,
            "elemental_studies": 40,
            "alchemy": 30
        },
        "level_increase": 2
    },
    "humanities": {
        "display_name": "Humanities Degree",
        "requirements": {
            "language": 50,
            "history": 40,
            "arts": 30
        },
        "level_increase": 2
    },
    "master_scholar": {
        "display_name": "Master Scholar Degree",
        "requirements": {
            "mathematics": 70,
            "language": 70,
            "science": 70,
            "history": 70,
            "magic_theory": 70
        },
        "level_increase": 3
    }
}

EDUCATION_CERTIFICATIONS = {
    "elemental_mastery": {
        "display_name": "Elemental Mastery Certification",
        "requirements": {
            "elemental_studies": 60
        }
    },
    "creature_handling": {
        "display_name": "Creature Handling Certification",
        "requirements": {
            "creature_biology": 60
        }
    },
    "alchemical_brewing": {
        "display_name": "Alchemical Brewing Certification",
        "requirements": {
            "alchemy": 60
        }
    },
    "artistic_expression": {
        "display_name": "Artistic Expression Certification",
        "requirements": {
            "arts": 60
        }
    },
    "physical_training": {
        "display_name": "Physical Training Certification",
        "requirements": {
            "physical_education": 60
        }
    }
}

# --- Evolution System Constants ---
EVOLUTION_PATHS = {
    "Glowing Sprite": [
        {
            "name": "Luminous Sprite",
            "requirements": {
                "min_maturity": 30,
                "min_stats": {"iq": 40, "happiness": 50},
                "achievements": ["friend_of_light"]
            },
            "bonuses": {
                "stats": {"energy": 10, "charisma": 5}
            },
            "potential_next": ["Radiant Spirit", "Prismatic Entity"]
        },
        {
            "name": "Radiant Spirit",
            "requirements": {
                "min_maturity": 60,
                "min_stats": {"iq": 60, "happiness": 70},
                "achievements": ["light_mastery"]
            },
            "bonuses": {
                "stats": {"energy": 15, "charisma": 10, "iq": 5}
            },
            "potential_next": ["Celestial Being"]
        },
        {
            "name": "Celestial Being",
            "requirements": {
                "min_maturity": 90,
                "min_stats": {"iq": 80, "happiness": 90, "energy": 80},
                "achievements": ["cosmic_harmony"]
            },
            "bonuses": {
                "stats": {"energy": 20, "charisma": 15, "iq": 10, "happiness": 10}
            },
            "potential_next": []
        }
    ],
    "Crystal Sprite": [
        {
            "name": "Faceted Sprite",
            "requirements": {
                "min_maturity": 30,
                "min_stats": {"iq": 40, "cleanliness": 50},
                "achievements": ["crystal_collector"]
            },
            "bonuses": {
                "stats": {"iq": 10, "cleanliness": 5}
            },
            "potential_next": ["Crystalline Entity", "Geode Guardian"]
        },
        {
            "name": "Crystalline Entity",
            "requirements": {
                "min_maturity": 60,
                "min_stats": {"iq": 60, "cleanliness": 70},
                "achievements": ["crystal_harmony"]
            },
            "bonuses": {
                "stats": {"iq": 15, "cleanliness": 10, "energy": 5}
            },
            "potential_next": ["Prismatic Overlord"]
        },
        {
            "name": "Prismatic Overlord",
            "requirements": {
                "min_maturity": 90,
                "min_stats": {"iq": 80, "cleanliness": 90, "energy": 80},
                "achievements": ["crystal_mastery"]
            },
            "bonuses": {
                "stats": {"iq": 20, "cleanliness": 15, "energy": 10, "charisma": 10}
            },
            "potential_next": []
        }
    ]
}

# --- Achievement System Constants ---
ACHIEVEMENTS = {
    "first_steps": {
        "name": "First Steps",
        "description": "Begin your journey with your pet.",
        "required_progress": 1,
        "points": 5
    },
    "social_butterfly": {
        "name": "Social Butterfly",
        "description": "Interact with your pet 50 times.",
        "required_progress": 50,
        "points": 10
    },
    "gourmet_feeder": {
        "name": "Gourmet Feeder",
        "description": "Feed your pet 30 times.",
        "required_progress": 30,
        "points": 10
    },
    "playful_companion": {
        "name": "Playful Companion",
        "description": "Play with your pet 30 times.",
        "required_progress": 30,
        "points": 10
    },
    "clean_freak": {
        "name": "Clean Freak",
        "description": "Groom your pet 30 times.",
        "required_progress": 30,
        "points": 10
    },
    "friend_of_light": {
        "name": "Friend of Light",
        "description": "Reach maximum happiness with a Glowing Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "crystal_collector": {
        "name": "Crystal Collector",
        "description": "Reach maximum cleanliness with a Crystal Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "shadow_master": {
        "name": "Shadow Master",
        "description": "Reach maximum energy with a Shadow Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "ember_friend": {
        "name": "Ember Friend",
        "description": "Reach maximum charisma with an Ember Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "aqua_ally": {
        "name": "Aqua Ally",
        "description": "Reach maximum social with an Aqua Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "battle_novice": {
        "name": "Battle Novice",
        "description": "Win 5 battles.",
        "required_progress": 5,
        "points": 10
    },
    "battle_expert": {
        "name": "Battle Expert",
        "description": "Win 20 battles.",
        "required_progress": 20,
        "points": 20
    },
    "battle_master": {
        "name": "Battle Master",
        "description": "Win 50 battles.",
        "required_progress": 50,
        "points": 30
    },
    "quest_seeker": {
        "name": "Quest Seeker",
        "description": "Complete 5 quests.",
        "required_progress": 5,
        "points": 10
    },
    "quest_adventurer": {
        "name": "Quest Adventurer",
        "description": "Complete 15 quests.",
        "required_progress": 15,
        "points": 20
    },
    "quest_legend": {
        "name": "Quest Legend",
        "description": "Complete 30 quests.",
        "required_progress": 30,
        "points": 30
    },
    "educated_pet": {
        "name": "Educated Pet",
        "description": "Earn a Basic Education Certificate.",
        "required_progress": 1,
        "points": 15
    },
    "scholarly_pet": {
        "name": "Scholarly Pet",
        "description": "Earn any advanced degree.",
        "required_progress": 1,
        "points": 25
    },
    "master_scholar": {
        "name": "Master Scholar",
        "description": "Earn the Master Scholar Degree.",
        "required_progress": 1,
        "points": 40
    },
    "evolution_milestone": {
        "name": "Evolution Milestone",
        "description": "Evolve your pet for the first time.",
        "required_progress": 1,
        "points": 20
    },
    "evolution_journey": {
        "name": "Evolution Journey",
        "description": "Reach the second evolution stage.",
        "required_progress": 1,
        "points": 30
    },
    "evolution_mastery": {
        "name": "Evolution Mastery",
        "description": "Reach the final evolution stage.",
        "required_progress": 1,
        "points": 50
    },
    "light_mastery": {
        "name": "Light Mastery",
        "description": "Master the power of light with your Luminous Sprite.",
        "required_progress": 1,
        "points": 25
    },
    "cosmic_harmony": {
        "name": "Cosmic Harmony",
        "description": "Achieve perfect balance with celestial energies.",
        "required_progress": 1,
        "points": 40
    },
    "crystal_harmony": {
        "name": "Crystal Harmony",
        "description": "Attune perfectly with crystal energies.",
        "required_progress": 1,
        "points": 25
    },
    "crystal_mastery": {
        "name": "Crystal Mastery",
        "description": "Become one with the crystalline forces of the world.",
        "required_progress": 1,
        "points": 40
    }
}

# --- DNA and Genetics System Constants ---
DNA_TRAITS = {
    "Glowing Sprite": {
        "luminosity": ["bright", "moderate", "dim"],
        "color_pattern": ["solid", "spotted", "striped", "swirled"],
        "energy_type": ["solar", "lunar", "stellar", "ambient"],
        "temperament": ["calm", "energetic", "curious", "shy"]
    },
    "Crystal Sprite": {
        "crystal_type": ["quartz", "amethyst", "emerald", "sapphire", "ruby"],
        "formation": ["geometric", "prismatic", "jagged", "smooth"],
        "resonance": ["harmonic", "discordant", "pulsing", "stable"],
        "transparency": ["clear", "translucent", "opaque", "variable"]
    },
    "Shadow Sprite": {
        "shadow_density": ["thick", "wispy", "shifting", "solid"],
        "darkness_level": ["deep", "moderate", "light", "variable"],
        "manifestation": ["corporeal", "ethereal", "flickering", "stable"],
        "aura": ["absorbing", "projecting", "neutral", "pulsing"]
    },
    "Ember Sprite": {
        "flame_type": ["steady", "flickering", "explosive", "smoldering"],
        "heat_level": ["scorching", "warm", "cool-burning", "variable"],
        "fuel_source": ["emotional", "physical", "spiritual", "ambient"],
        "burn_pattern": ["constant", "pulsing", "erratic", "controlled"]
    },
    "Aqua Sprite": {
        "water_state": ["flowing", "misting", "droplet", "wave"],
        "clarity": ["crystal", "cloudy", "reflective", "refractive"],
        "current": ["calm", "turbulent", "swirling", "tidal"],
        "composition": ["pure", "mineral-rich", "saline", "adaptive"]
    }
}

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
]"""
Constants for advanced pet features including jobs, battles, quests, education, evolution, and DNA.
"""

# --- Job System Constants ---
JOB_TYPES = {
    "teacher": {
        "display_name": "Teacher",
        "description": "Educate young pets and help them grow.",
        "requirements": {
            "min_stats": {"iq": 40, "charisma": 30},
            "min_age": 5
        },
        "base_salary": 10,
        "exp_per_work": 15,
        "skills": ["teaching", "patience", "knowledge"]
    },
    "guard": {
        "display_name": "Guard",
        "description": "Protect important locations and maintain security.",
        "requirements": {
            "min_stats": {"energy": 50},
            "min_age": 3
        },
        "base_salary": 12,
        "exp_per_work": 10,
        "skills": ["vigilance", "strength", "intimidation"]
    },
    "healer": {
        "display_name": "Healer",
        "description": "Help injured and sick pets recover.",
        "requirements": {
            "min_stats": {"iq": 35, "social": 40},
            "min_age": 4
        },
        "base_salary": 15,
        "exp_per_work": 12,
        "skills": ["diagnosis", "treatment", "bedside_manner"]
    },
    "explorer": {
        "display_name": "Explorer",
        "description": "Discover new locations and treasures.",
        "requirements": {
            "min_stats": {"energy": 45, "happiness": 40},
            "min_age": 2
        },
        "base_salary": 8,
        "exp_per_work": 20,
        "skills": ["navigation", "survival", "observation"]
    },
    "entertainer": {
        "display_name": "Entertainer",
        "description": "Bring joy and laughter to other pets.",
        "requirements": {
            "min_stats": {"charisma": 45, "happiness": 50},
            "min_age": 1
        },
        "base_salary": 9,
        "exp_per_work": 18,
        "skills": ["performance", "creativity", "audience_connection"]
    }
}

# --- Battle System Constants ---
BATTLE_OPPONENTS = {
    "training_dummy": {
        "display_name": "Training Dummy",
        "description": "A basic opponent for practice.",
        "power": 30,
        "reward": 5,
        "abilities": ["basic_strike"]
    },
    "forest_sprite": {
        "display_name": "Forest Sprite",
        "description": "A mischievous nature spirit.",
        "power": 45,
        "reward": 10,
        "abilities": ["nature_touch", "leaf_shield"]
    },
    "shadow_lurker": {
        "display_name": "Shadow Lurker",
        "description": "A mysterious creature that hides in darkness.",
        "power": 60,
        "reward": 15,
        "abilities": ["shadow_cloak", "night_strike"]
    },
    "crystal_guardian": {
        "display_name": "Crystal Guardian",
        "description": "A powerful protector made of living crystal.",
        "power": 80,
        "reward": 20,
        "abilities": ["crystal_beam", "reflective_shield"]
    },
    "elder_elemental": {
        "display_name": "Elder Elemental",
        "description": "An ancient being of pure elemental energy.",
        "power": 100,
        "reward": 30,
        "abilities": ["elemental_surge", "primal_force", "energy_drain"]
    }
}

# --- Quest System Constants ---
AVAILABLE_QUESTS = {
    "welcome_quest": {
        "name": "Welcome to CritterCraft",
        "description": "Learn the basics of pet care and interaction.",
        "requirements": {
            "min_maturity": 0
        },
        "required_progress": 5,
        "reward_points": 5,
        "faction": "crittercraft"
    },
    "forest_exploration": {
        "name": "Forest Exploration",
        "description": "Explore the mysterious forest and discover its secrets.",
        "requirements": {
            "min_maturity": 10
        },
        "required_progress": 10,
        "reward_points": 10,
        "faction": "explorers"
    },
    "crystal_collection": {
        "name": "Crystal Collection",
        "description": "Collect rare crystals from the crystal caves.",
        "requirements": {
            "min_maturity": 20
        },
        "required_progress": 15,
        "reward_points": 15,
        "faction": "collectors"
    },
    "shadow_investigation": {
        "name": "Shadow Investigation",
        "description": "Investigate the strange shadows appearing in town.",
        "requirements": {
            "min_maturity": 30
        },
        "required_progress": 20,
        "reward_points": 20,
        "faction": "guardians"
    },
    "elemental_harmony": {
        "name": "Elemental Harmony",
        "description": "Restore balance to the elemental shrines.",
        "requirements": {
            "min_maturity": 50
        },
        "required_progress": 30,
        "reward_points": 30,
        "faction": "elementalists"
    }
}

# --- Education System Constants ---
EDUCATION_SUBJECTS = [
    "mathematics",
    "language",
    "science",
    "history",
    "arts",
    "physical_education",
    "magic_theory",
    "elemental_studies",
    "creature_biology",
    "alchemy"
]

EDUCATION_DEGREES = {
    "basic_education": {
        "display_name": "Basic Education Certificate",
        "requirements": {
            "mathematics": 20,
            "language": 20,
            "science": 20
        },
        "level_increase": 1
    },
    "natural_sciences": {
        "display_name": "Natural Sciences Degree",
        "requirements": {
            "science": 50,
            "creature_biology": 40,
            "mathematics": 30
        },
        "level_increase": 2
    },
    "arcane_arts": {
        "display_name": "Arcane Arts Degree",
        "requirements": {
            "magic_theory": 50,
            "elemental_studies": 40,
            "alchemy": 30
        },
        "level_increase": 2
    },
    "humanities": {
        "display_name": "Humanities Degree",
        "requirements": {
            "language": 50,
            "history": 40,
            "arts": 30
        },
        "level_increase": 2
    },
    "master_scholar": {
        "display_name": "Master Scholar Degree",
        "requirements": {
            "mathematics": 70,
            "language": 70,
            "science": 70,
            "history": 70,
            "magic_theory": 70
        },
        "level_increase": 3
    }
}

EDUCATION_CERTIFICATIONS = {
    "elemental_mastery": {
        "display_name": "Elemental Mastery Certification",
        "requirements": {
            "elemental_studies": 60
        }
    },
    "creature_handling": {
        "display_name": "Creature Handling Certification",
        "requirements": {
            "creature_biology": 60
        }
    },
    "alchemical_brewing": {
        "display_name": "Alchemical Brewing Certification",
        "requirements": {
            "alchemy": 60
        }
    },
    "artistic_expression": {
        "display_name": "Artistic Expression Certification",
        "requirements": {
            "arts": 60
        }
    },
    "physical_training": {
        "display_name": "Physical Training Certification",
        "requirements": {
            "physical_education": 60
        }
    }
}

# --- Evolution System Constants ---
EVOLUTION_PATHS = {
    "Glowing Sprite": [
        {
            "name": "Luminous Sprite",
            "requirements": {
                "min_maturity": 30,
                "min_stats": {"iq": 40, "happiness": 50},
                "achievements": ["friend_of_light"]
            },
            "bonuses": {
                "stats": {"energy": 10, "charisma": 5}
            },
            "potential_next": ["Radiant Spirit", "Prismatic Entity"]
        },
        {
            "name": "Radiant Spirit",
            "requirements": {
                "min_maturity": 60,
                "min_stats": {"iq": 60, "happiness": 70},
                "achievements": ["light_mastery"]
            },
            "bonuses": {
                "stats": {"energy": 15, "charisma": 10, "iq": 5}
            },
            "potential_next": ["Celestial Being"]
        },
        {
            "name": "Celestial Being",
            "requirements": {
                "min_maturity": 90,
                "min_stats": {"iq": 80, "happiness": 90, "energy": 80},
                "achievements": ["cosmic_harmony"]
            },
            "bonuses": {
                "stats": {"energy": 20, "charisma": 15, "iq": 10, "happiness": 10}
            },
            "potential_next": []
        }
    ],
    "Crystal Sprite": [
        {
            "name": "Faceted Sprite",
            "requirements": {
                "min_maturity": 30,
                "min_stats": {"iq": 40, "cleanliness": 50},
                "achievements": ["crystal_collector"]
            },
            "bonuses": {
                "stats": {"iq": 10, "cleanliness": 5}
            },
            "potential_next": ["Crystalline Entity", "Geode Guardian"]
        },
        {
            "name": "Crystalline Entity",
            "requirements": {
                "min_maturity": 60,
                "min_stats": {"iq": 60, "cleanliness": 70},
                "achievements": ["crystal_harmony"]
            },
            "bonuses": {
                "stats": {"iq": 15, "cleanliness": 10, "energy": 5}
            },
            "potential_next": ["Prismatic Overlord"]
        },
        {
            "name": "Prismatic Overlord",
            "requirements": {
                "min_maturity": 90,
                "min_stats": {"iq": 80, "cleanliness": 90, "energy": 80},
                "achievements": ["crystal_mastery"]
            },
            "bonuses": {
                "stats": {"iq": 20, "cleanliness": 15, "energy": 10, "charisma": 10}
            },
            "potential_next": []
        }
    ]
}

# --- Achievement System Constants ---
ACHIEVEMENTS = {
    "first_steps": {
        "name": "First Steps",
        "description": "Begin your journey with your pet.",
        "required_progress": 1,
        "points": 5
    },
    "social_butterfly": {
        "name": "Social Butterfly",
        "description": "Interact with your pet 50 times.",
        "required_progress": 50,
        "points": 10
    },
    "gourmet_feeder": {
        "name": "Gourmet Feeder",
        "description": "Feed your pet 30 times.",
        "required_progress": 30,
        "points": 10
    },
    "playful_companion": {
        "name": "Playful Companion",
        "description": "Play with your pet 30 times.",
        "required_progress": 30,
        "points": 10
    },
    "clean_freak": {
        "name": "Clean Freak",
        "description": "Groom your pet 30 times.",
        "required_progress": 30,
        "points": 10
    },
    "friend_of_light": {
        "name": "Friend of Light",
        "description": "Reach maximum happiness with a Glowing Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "crystal_collector": {
        "name": "Crystal Collector",
        "description": "Reach maximum cleanliness with a Crystal Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "shadow_master": {
        "name": "Shadow Master",
        "description": "Reach maximum energy with a Shadow Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "ember_friend": {
        "name": "Ember Friend",
        "description": "Reach maximum charisma with an Ember Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "aqua_ally": {
        "name": "Aqua Ally",
        "description": "Reach maximum social with an Aqua Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "battle_novice": {
        "name": "Battle Novice",
        "description": "Win 5 battles.",
        "required_progress": 5,
        "points": 10
    },
    "battle_expert": {
        "name": "Battle Expert",
        "description": "Win 20 battles.",
        "required_progress": 20,
        "points": 20
    },
    "battle_master": {
        "name": "Battle Master",
        "description": "Win 50 battles.",
        "required_progress": 50,
        "points": 30
    },
    "quest_seeker": {
        "name": "Quest Seeker",
        "description": "Complete 5 quests.",
        "required_progress": 5,
        "points": 10
    },
    "quest_adventurer": {
        "name": "Quest Adventurer",
        "description": "Complete 15 quests.",
        "required_progress": 15,
        "points": 20
    },
    "quest_legend": {
        "name": "Quest Legend",
        "description": "Complete 30 quests.",
        "required_progress": 30,
        "points": 30
    },
    "educated_pet": {
        "name": "Educated Pet",
        "description": "Earn a Basic Education Certificate.",
        "required_progress": 1,
        "points": 15
    },
    "scholarly_pet": {
        "name": "Scholarly Pet",
        "description": "Earn any advanced degree.",
        "required_progress": 1,
        "points": 25
    },
    "master_scholar": {
        "name": "Master Scholar",
        "description": "Earn the Master Scholar Degree.",
        "required_progress": 1,
        "points": 40
    },
    "evolution_milestone": {
        "name": "Evolution Milestone",
        "description": "Evolve your pet for the first time.",
        "required_progress": 1,
        "points": 20
    },
    "evolution_journey": {
        "name": "Evolution Journey",
        "description": "Reach the second evolution stage.",
        "required_progress": 1,
        "points": 30
    },
    "evolution_mastery": {
        "name": "Evolution Mastery",
        "description": "Reach the final evolution stage.",
        "required_progress": 1,
        "points": 50
    },
    "light_mastery": {
        "name": "Light Mastery",
        "description": "Master the power of light with your Luminous Sprite.",
        "required_progress": 1,
        "points": 25
    },
    "cosmic_harmony": {
        "name": "Cosmic Harmony",
        "description": "Achieve perfect balance with celestial energies.",
        "required_progress": 1,
        "points": 40
    },
    "crystal_harmony": {
        "name": "Crystal Harmony",
        "description": "Attune perfectly with crystal energies.",
        "required_progress": 1,
        "points": 25
    },
    "crystal_mastery": {
        "name": "Crystal Mastery",
        "description": "Become one with the crystalline forces of the world.",
        "required_progress": 1,
        "points": 40
    }
}

# --- DNA and Genetics System Constants ---
DNA_TRAITS = {
    "Glowing Sprite": {
        "luminosity": ["bright", "moderate", "dim"],
        "color_pattern": ["solid", "spotted", "striped", "swirled"],
        "energy_type": ["solar", "lunar", "stellar", "ambient"],
        "temperament": ["calm", "energetic", "curious", "shy"]
    },
    "Crystal Sprite": {
        "crystal_type": ["quartz", "amethyst", "emerald", "sapphire", "ruby"],
        "formation": ["geometric", "prismatic", "jagged", "smooth"],
        "resonance": ["harmonic", "discordant", "pulsing", "stable"],
        "transparency": ["clear", "translucent", "opaque", "variable"]
    },
    "Shadow Sprite": {
        "shadow_density": ["thick", "wispy", "shifting", "solid"],
        "darkness_level": ["deep", "moderate", "light", "variable"],
        "manifestation": ["corporeal", "ethereal", "flickering", "stable"],
        "aura": ["absorbing", "projecting", "neutral", "pulsing"]
    },
    "Ember Sprite": {
        "flame_type": ["steady", "flickering", "explosive", "smoldering"],
        "heat_level": ["scorching", "warm", "cool-burning", "variable"],
        "fuel_source": ["emotional", "physical", "spiritual", "ambient"],
        "burn_pattern": ["constant", "pulsing", "erratic", "controlled"]
    },
    "Aqua Sprite": {
        "water_state": ["flowing", "misting", "droplet", "wave"],
        "clarity": ["crystal", "cloudy", "reflective", "refractive"],
        "current": ["calm", "turbulent", "swirling", "tidal"],
        "composition": ["pure", "mineral-rich", "saline", "adaptive"]
    }
}

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
]"""
Constants for advanced pet features including jobs, battles, quests, education, evolution, and DNA.
"""

# --- Job System Constants ---
JOB_TYPES = {
    "teacher": {
        "display_name": "Teacher",
        "description": "Educate young pets and help them grow.",
        "requirements": {
            "min_stats": {"iq": 40, "charisma": 30},
            "min_age": 5
        },
        "base_salary": 10,
        "exp_per_work": 15,
        "skills": ["teaching", "patience", "knowledge"]
    },
    "guard": {
        "display_name": "Guard",
        "description": "Protect important locations and maintain security.",
        "requirements": {
            "min_stats": {"energy": 50},
            "min_age": 3
        },
        "base_salary": 12,
        "exp_per_work": 10,
        "skills": ["vigilance", "strength", "intimidation"]
    },
    "healer": {
        "display_name": "Healer",
        "description": "Help injured and sick pets recover.",
        "requirements": {
            "min_stats": {"iq": 35, "social": 40},
            "min_age": 4
        },
        "base_salary": 15,
        "exp_per_work": 12,
        "skills": ["diagnosis", "treatment", "bedside_manner"]
    },
    "explorer": {
        "display_name": "Explorer",
        "description": "Discover new locations and treasures.",
        "requirements": {
            "min_stats": {"energy": 45, "happiness": 40},
            "min_age": 2
        },
        "base_salary": 8,
        "exp_per_work": 20,
        "skills": ["navigation", "survival", "observation"]
    },
    "entertainer": {
        "display_name": "Entertainer",
        "description": "Bring joy and laughter to other pets.",
        "requirements": {
            "min_stats": {"charisma": 45, "happiness": 50},
            "min_age": 1
        },
        "base_salary": 9,
        "exp_per_work": 18,
        "skills": ["performance", "creativity", "audience_connection"]
    }
}

# --- Battle System Constants ---
BATTLE_OPPONENTS = {
    "training_dummy": {
        "display_name": "Training Dummy",
        "description": "A basic opponent for practice.",
        "power": 30,
        "reward": 5,
        "abilities": ["basic_strike"]
    },
    "forest_sprite": {
        "display_name": "Forest Sprite",
        "description": "A mischievous nature spirit.",
        "power": 45,
        "reward": 10,
        "abilities": ["nature_touch", "leaf_shield"]
    },
    "shadow_lurker": {
        "display_name": "Shadow Lurker",
        "description": "A mysterious creature that hides in darkness.",
        "power": 60,
        "reward": 15,
        "abilities": ["shadow_cloak", "night_strike"]
    },
    "crystal_guardian": {
        "display_name": "Crystal Guardian",
        "description": "A powerful protector made of living crystal.",
        "power": 80,
        "reward": 20,
        "abilities": ["crystal_beam", "reflective_shield"]
    },
    "elder_elemental": {
        "display_name": "Elder Elemental",
        "description": "An ancient being of pure elemental energy.",
        "power": 100,
        "reward": 30,
        "abilities": ["elemental_surge", "primal_force", "energy_drain"]
    }
}

# --- Quest System Constants ---
AVAILABLE_QUESTS = {
    "welcome_quest": {
        "name": "Welcome to CritterCraft",
        "description": "Learn the basics of pet care and interaction.",
        "requirements": {
            "min_maturity": 0
        },
        "required_progress": 5,
        "reward_points": 5,
        "faction": "crittercraft"
    },
    "forest_exploration": {
        "name": "Forest Exploration",
        "description": "Explore the mysterious forest and discover its secrets.",
        "requirements": {
            "min_maturity": 10
        },
        "required_progress": 10,
        "reward_points": 10,
        "faction": "explorers"
    },
    "crystal_collection": {
        "name": "Crystal Collection",
        "description": "Collect rare crystals from the crystal caves.",
        "requirements": {
            "min_maturity": 20
        },
        "required_progress": 15,
        "reward_points": 15,
        "faction": "collectors"
    },
    "shadow_investigation": {
        "name": "Shadow Investigation",
        "description": "Investigate the strange shadows appearing in town.",
        "requirements": {
            "min_maturity": 30
        },
        "required_progress": 20,
        "reward_points": 20,
        "faction": "guardians"
    },
    "elemental_harmony": {
        "name": "Elemental Harmony",
        "description": "Restore balance to the elemental shrines.",
        "requirements": {
            "min_maturity": 50
        },
        "required_progress": 30,
        "reward_points": 30,
        "faction": "elementalists"
    }
}

# --- Education System Constants ---
EDUCATION_SUBJECTS = [
    "mathematics",
    "language",
    "science",
    "history",
    "arts",
    "physical_education",
    "magic_theory",
    "elemental_studies",
    "creature_biology",
    "alchemy"
]

EDUCATION_DEGREES = {
    "basic_education": {
        "display_name": "Basic Education Certificate",
        "requirements": {
            "mathematics": 20,
            "language": 20,
            "science": 20
        },
        "level_increase": 1
    },
    "natural_sciences": {
        "display_name": "Natural Sciences Degree",
        "requirements": {
            "science": 50,
            "creature_biology": 40,
            "mathematics": 30
        },
        "level_increase": 2
    },
    "arcane_arts": {
        "display_name": "Arcane Arts Degree",
        "requirements": {
            "magic_theory": 50,
            "elemental_studies": 40,
            "alchemy": 30
        },
        "level_increase": 2
    },
    "humanities": {
        "display_name": "Humanities Degree",
        "requirements": {
            "language": 50,
            "history": 40,
            "arts": 30
        },
        "level_increase": 2
    },
    "master_scholar": {
        "display_name": "Master Scholar Degree",
        "requirements": {
            "mathematics": 70,
            "language": 70,
            "science": 70,
            "history": 70,
            "magic_theory": 70
        },
        "level_increase": 3
    }
}

EDUCATION_CERTIFICATIONS = {
    "elemental_mastery": {
        "display_name": "Elemental Mastery Certification",
        "requirements": {
            "elemental_studies": 60
        }
    },
    "creature_handling": {
        "display_name": "Creature Handling Certification",
        "requirements": {
            "creature_biology": 60
        }
    },
    "alchemical_brewing": {
        "display_name": "Alchemical Brewing Certification",
        "requirements": {
            "alchemy": 60
        }
    },
    "artistic_expression": {
        "display_name": "Artistic Expression Certification",
        "requirements": {
            "arts": 60
        }
    },
    "physical_training": {
        "display_name": "Physical Training Certification",
        "requirements": {
            "physical_education": 60
        }
    }
}

# --- Evolution System Constants ---
EVOLUTION_PATHS = {
    "Glowing Sprite": [
        {
            "name": "Luminous Sprite",
            "requirements": {
                "min_maturity": 30,
                "min_stats": {"iq": 40, "happiness": 50},
                "achievements": ["friend_of_light"]
            },
            "bonuses": {
                "stats": {"energy": 10, "charisma": 5}
            },
            "potential_next": ["Radiant Spirit", "Prismatic Entity"]
        },
        {
            "name": "Radiant Spirit",
            "requirements": {
                "min_maturity": 60,
                "min_stats": {"iq": 60, "happiness": 70},
                "achievements": ["light_mastery"]
            },
            "bonuses": {
                "stats": {"energy": 15, "charisma": 10, "iq": 5}
            },
            "potential_next": ["Celestial Being"]
        },
        {
            "name": "Celestial Being",
            "requirements": {
                "min_maturity": 90,
                "min_stats": {"iq": 80, "happiness": 90, "energy": 80},
                "achievements": ["cosmic_harmony"]
            },
            "bonuses": {
                "stats": {"energy": 20, "charisma": 15, "iq": 10, "happiness": 10}
            },
            "potential_next": []
        }
    ],
    "Crystal Sprite": [
        {
            "name": "Faceted Sprite",
            "requirements": {
                "min_maturity": 30,
                "min_stats": {"iq": 40, "cleanliness": 50},
                "achievements": ["crystal_collector"]
            },
            "bonuses": {
                "stats": {"iq": 10, "cleanliness": 5}
            },
            "potential_next": ["Crystalline Entity", "Geode Guardian"]
        },
        {
            "name": "Crystalline Entity",
            "requirements": {
                "min_maturity": 60,
                "min_stats": {"iq": 60, "cleanliness": 70},
                "achievements": ["crystal_harmony"]
            },
            "bonuses": {
                "stats": {"iq": 15, "cleanliness": 10, "energy": 5}
            },
            "potential_next": ["Prismatic Overlord"]
        },
        {
            "name": "Prismatic Overlord",
            "requirements": {
                "min_maturity": 90,
                "min_stats": {"iq": 80, "cleanliness": 90, "energy": 80},
                "achievements": ["crystal_mastery"]
            },
            "bonuses": {
                "stats": {"iq": 20, "cleanliness": 15, "energy": 10, "charisma": 10}
            },
            "potential_next": []
        }
    ]
}

# --- Achievement System Constants ---
ACHIEVEMENTS = {
    "first_steps": {
        "name": "First Steps",
        "description": "Begin your journey with your pet.",
        "required_progress": 1,
        "points": 5
    },
    "social_butterfly": {
        "name": "Social Butterfly",
        "description": "Interact with your pet 50 times.",
        "required_progress": 50,
        "points": 10
    },
    "gourmet_feeder": {
        "name": "Gourmet Feeder",
        "description": "Feed your pet 30 times.",
        "required_progress": 30,
        "points": 10
    },
    "playful_companion": {
        "name": "Playful Companion",
        "description": "Play with your pet 30 times.",
        "required_progress": 30,
        "points": 10
    },
    "clean_freak": {
        "name": "Clean Freak",
        "description": "Groom your pet 30 times.",
        "required_progress": 30,
        "points": 10
    },
    "friend_of_light": {
        "name": "Friend of Light",
        "description": "Reach maximum happiness with a Glowing Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "crystal_collector": {
        "name": "Crystal Collector",
        "description": "Reach maximum cleanliness with a Crystal Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "shadow_master": {
        "name": "Shadow Master",
        "description": "Reach maximum energy with a Shadow Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "ember_friend": {
        "name": "Ember Friend",
        "description": "Reach maximum charisma with an Ember Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "aqua_ally": {
        "name": "Aqua Ally",
        "description": "Reach maximum social with an Aqua Sprite.",
        "required_progress": 1,
        "points": 15
    },
    "battle_novice": {
        "name": "Battle Novice",
        "description": "Win 5 battles.",
        "required_progress": 5,
        "points": 10
    },
    "battle_expert": {
        "name": "Battle Expert",
        "description": "Win 20 battles.",
        "required_progress": 20,
        "points": 20
    },
    "battle_master": {
        "name": "Battle Master",
        "description": "Win 50 battles.",
        "required_progress": 50,
        "points": 30
    },
    "quest_seeker": {
        "name": "Quest Seeker",
        "description": "Complete 5 quests.",
        "required_progress": 5,
        "points": 10
    },
    "quest_adventurer": {
        "name": "Quest Adventurer",
        "description": "Complete 15 quests.",
        "required_progress": 15,
        "points": 20
    },
    "quest_legend": {
        "name": "Quest Legend",
        "description": "Complete 30 quests.",
        "required_progress": 30,
        "points": 30
    },
    "educated_pet": {
        "name": "Educated Pet",
        "description": "Earn a Basic Education Certificate.",
        "required_progress": 1,
        "points": 15
    },
    "scholarly_pet": {
        "name": "Scholarly Pet",
        "description": "Earn any advanced degree.",
        "required_progress": 1,
        "points": 25
    },
    "master_scholar": {
        "name": "Master Scholar",
        "description": "Earn the Master Scholar Degree.",
        "required_progress": 1,
        "points": 40
    },
    "evolution_milestone": {
        "name": "Evolution Milestone",
        "description": "Evolve your pet for the first time.",
        "required_progress": 1,
        "points": 20
    },
    "evolution_journey": {
        "name": "Evolution Journey",
        "description": "Reach the second evolution stage.",
        "required_progress": 1,
        "points": 30
    },
    "evolution_mastery": {
        "name": "Evolution Mastery",
        "description": "Reach the final evolution stage.",
        "required_progress": 1,
        "points": 50
    },
    "light_mastery": {
        "name": "Light Mastery",
        "description": "Master the power of light with your Luminous Sprite.",
        "required_progress": 1,
        "points": 25
    },
    "cosmic_harmony": {
        "name": "Cosmic Harmony",
        "description": "Achieve perfect balance with celestial energies.",
        "required_progress": 1,
        "points": 40
    },
    "crystal_harmony": {
        "name": "Crystal Harmony",
        "description": "Attune perfectly with crystal energies.",
        "required_progress": 1,
        "points": 25
    },
    "crystal_mastery": {
        "name": "Crystal Mastery",
        "description": "Become one with the crystalline forces of the world.",
        "required_progress": 1,
        "points": 40
    }
}

# --- DNA and Genetics System Constants ---
DNA_TRAITS = {
    "Glowing Sprite": {
        "luminosity": ["bright", "moderate", "dim"],
        "color_pattern": ["solid", "spotted", "striped", "swirled"],
        "energy_type": ["solar", "lunar", "stellar", "ambient"],
        "temperament": ["calm", "energetic", "curious", "shy"]
    },
    "Crystal Sprite": {
        "crystal_type": ["quartz", "amethyst", "emerald", "sapphire", "ruby"],
        "formation": ["geometric", "prismatic", "jagged", "smooth"],
        "resonance": ["harmonic", "discordant", "pulsing", "stable"],
        "transparency": ["clear", "translucent", "opaque", "variable"]
    },
    "Shadow Sprite": {
        "shadow_density": ["thick", "wispy", "shifting", "solid"],
        "darkness_level": ["deep", "moderate", "light", "variable"],
        "manifestation": ["corporeal", "ethereal", "flickering", "stable"],
        "aura": ["absorbing", "projecting", "neutral", "pulsing"]
    },
    "Ember Sprite": {
        "flame_type": ["steady", "flickering", "explosive", "smoldering"],
        "heat_level": ["scorching", "warm", "cool-burning", "variable"],
        "fuel_source": ["emotional", "physical", "spiritual", "ambient"],
        "burn_pattern": ["constant", "pulsing", "erratic", "controlled"]
    },
    "Aqua Sprite": {
        "water_state": ["flowing", "misting", "droplet", "wave"],
        "clarity": ["crystal", "cloudy", "reflective", "refractive"],
        "current": ["calm", "turbulent", "swirling", "tidal"],
        "composition": ["pure", "mineral-rich", "saline", "adaptive"]
    }
}

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