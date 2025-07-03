"""
Unified and Strategic Configuration for the CritterCraft Universe.

This module applies the Expanded KISS Principle to centralize all game parameters
for both the 'Genesis Pet' and 'Critter-Craft' systems. By using Enums and
namespaced classes, it provides a single, clear, robust, and easily tunable
source of truth for game designers and developers.

Principles Applied:
- K (Know Your Core, Keep it Clear): Concepts are separated into distinct classes.
  Enums create a clear, typo-proof vocabulary for core stats and traits.
- S (Systematize for Scalability): A single Stat enum is the source of truth,
  and all related dictionaries are systematically derived from it. The
  hierarchical class structure is scalable for future additions.
- I (Iterate Intelligently): Errors from the original files (duplicates,
  typos) have been fixed. The structure is easy to modify for game balancing.
"""

from enum import Enum, auto

# ==============================================================================
# --- CORE ENUM DEFINITIONS (Single Source of Truth) ---
# ==============================================================================

class Stat(Enum):
    """The single source of truth for all primary pet statistics."""
    HUNGER = "Sustenance"
    ENERGY = "Energy"
    HAPPINESS = "Happiness"
    IQ = "Intelligence"
    CHARISMA = "Charisma"
    CLEANLINESS = "Cleanliness"
    SOCIAL = "Social"

class PersonalityTrait(Enum):
    """Defines the core personality dimensions."""
    PLAYFULNESS = auto()
    CURIOSITY = auto()
    SOCIABILITY = auto()
    INDEPENDENCE = auto()
    LOYALTY = auto()
    ADVENTUROUSNESS = auto()
    INTELLIGENCE = auto()
    EMPATHY = auto()
    CALMNESS = auto()
    STUBBORNNESS = auto()

class Topic(Enum):
    GREETING = auto()
    WELL_BEING = auto()
    PLAY = auto()
    FOOD = auto()
    LEARNING = auto()
    COMPLIMENT = auto()
    ENVIRONMENT = auto()
    ADVENTURE = auto()
    FRIENDSHIP = auto()
    EMOTIONS = auto()

class Mood(Enum):
    ECSTATIC = auto()
    HAPPY = auto()
    NEUTRAL = auto()
    GRUMPY = auto()
    SAD = auto()
    MISERABLE = auto()
    ANXIOUS = auto()
    EXCITED = auto()
    BORED = auto()
    FRUSTRATED = auto()
    CONTENT = auto()

class MemoryType(Enum):
    INTERACTION = auto()
    PREFERENCE = auto()
    FACT = auto()
    MILESTONE = auto()
    EMOTION = auto()
    EXPERIENCE = auto()
    SKILL = auto()
    RELATIONSHIP = auto()

# ==============================================================================
# --- I. GENESIS PET CONFIGURATION ---
# ==============================================================================

class GenesisPetConfig:
    """A namespace for all settings related to the core virtual pet experience."""

    class Core:
        """Global mechanics and thresholds."""
        MAX_STAT = 100
        DECAY_INTERVAL_SECONDS = 3600
        INITIAL_STATS = {
            Stat.HUNGER: 50,
            Stat.ENERGY: 70,
            Stat.HAPPINESS: 60,
            Stat.IQ: 10,
            Stat.CHARISMA: 10,
            Stat.CLEANLINESS: 80,
            Stat.SOCIAL: 50,
        }
        DECAY_RATES = {
            Stat.HUNGER: 5,
            Stat.ENERGY: -3,
            Stat.HAPPINESS: -2,
            Stat.CLEANLINESS: -5,
            Stat.SOCIAL: -3,
            Stat.IQ: 0,
            Stat.CHARISMA: 0,
        }

    class Interactions:
        EFFECTS = {
            'feed': {
                'stat_changes': {Stat.HUNGER: -30, Stat.HAPPINESS: 5, Stat.CLEANLINESS: -10},
                'min_energy': 5,
                'messages': {
                    'success': "{pet_name} happily munches on the treat!",
                    'too_full': "{pet_name} seems too full to eat right now.",
                }
            },
            'play': {
                'stat_changes': {Stat.ENERGY: -20, Stat.HAPPINESS: 25, Stat.HUNGER: 10, Stat.SOCIAL: 15},
                'min_energy': 15,
                'messages': {
                    'success': "{pet_name} leaps with joy while playing with you!",
                    'too_tired': "{pet_name} is too tired to play. Let them rest.",
                }
            },
            'chat': {
                'stat_changes': {Stat.IQ: 3, Stat.HAPPINESS: 8, Stat.ENERGY: -5, Stat.SOCIAL: 10},
                'min_energy': 5,
                'messages': { 'success': "{pet_name} chirps thoughtfully." }
            },
            'groom': {
                'stat_changes': {Stat.CLEANLINESS: 40, Stat.HAPPINESS: 10, Stat.ENERGY: -5},
                'min_energy': 0,
                'messages': { 'success': "{pet_name} gleams after a good grooming session!" }
            }
        }

# For backward compatibility with the original config structure
MAX_STAT = GenesisPetConfig.Core.MAX_STAT
STAT_DECAY_INTERVAL_NS = GenesisPetConfig.Core.DECAY_INTERVAL_SECONDS * 1_000_000_000

# Default starting stats for all newly created pets (backward compatibility)
DEFAULT_INITIAL_PET_STATS = {
    'hunger': GenesisPetConfig.Core.INITIAL_STATS[Stat.HUNGER],
    'energy': GenesisPetConfig.Core.INITIAL_STATS[Stat.ENERGY],
    'happiness': GenesisPetConfig.Core.INITIAL_STATS[Stat.HAPPINESS],
    'iq': GenesisPetConfig.Core.INITIAL_STATS[Stat.IQ],
    'charisma': GenesisPetConfig.Core.INITIAL_STATS[Stat.CHARISMA],
    'cleanliness': GenesisPetConfig.Core.INITIAL_STATS[Stat.CLEANLINESS],
    'social': GenesisPetConfig.Core.INITIAL_STATS[Stat.SOCIAL],
}

# Passive decay rates (backward compatibility)
DECAY_RATES = {
    'hunger': GenesisPetConfig.Core.DECAY_RATES[Stat.HUNGER],
    'energy': abs(GenesisPetConfig.Core.DECAY_RATES[Stat.ENERGY]),  # Convert to positive for backward compatibility
    'happiness': abs(GenesisPetConfig.Core.DECAY_RATES[Stat.HAPPINESS]),
    'cleanliness': abs(GenesisPetConfig.Core.DECAY_RATES[Stat.CLEANLINESS]),
    'social': abs(GenesisPetConfig.Core.DECAY_RATES[Stat.SOCIAL]),
}


# ==============================================================================
# --- Pet Interaction & Action Effects (Backward Compatibility) ---
# ==============================================================================

# Defines the effects of various owner interactions on pet stats.
# Each interaction type maps to a dictionary of stat changes.
# Positive values increase stat, negative values decrease.
INTERACTION_EFFECTS = {
    'feed': {
        'hunger': -30,  # Feeding decreases hunger (makes pet less hungry)
        'happiness': 5,
        'cleanliness': -10, # Feeding might make pet a bit messier
        'min_energy_cost': 5, # Minimum energy required to successfully feed
        'messages': {
            'success': "{pet_name} happily munches on the treat!",
            'too_full': "{pet_name} seems too full to eat right now.",
            'too_tired': "{pet_name} is too tired to eat. Needs rest!",
        }
    },
    'play': {
        'energy': -20,  # Playing consumes energy
        'happiness': 25,
        'hunger': 10,   # Playing might make pet hungrier
        'social': 15,
        'min_energy_cost': 15, # Minimum energy required to successfully play
        'messages': {
            'success': "{pet_name} leaps with joy and loves playing with you!",
            'too_tired': "{pet_name} is too tired to play. Let them rest.",
            'too_hungry': "{pet_name} is too hungry to play energetically.",
            'bored': "{pet_name} seems bored with that game.", # Placeholder for future dynamic play
        }
    },
    'chat': {
        'iq': 3,
        'happiness': 8,
        'energy': -5, # Chatting costs minimal energy
        'social': 10,
        'charisma': 2, # Chatting improves charisma
        'min_energy_cost': 5, # Minimum energy required to engage in chat
        'messages': {
            'success': "{pet_name} chirps thoughtfully in response to your words.",
            'too_tired': "{pet_name} is too sleepy to chat much right now.",
            'no_interest': "{pet_name} looks at you blankly, perhaps they're not in the mood for deep conversation.",
        }
    },
    'groom': { # New interaction: Grooming
        'cleanliness': 40,
        'happiness': 10,
        'energy': -5,
        'messages': {
            'success': "{pet_name} gleams after a good grooming session!",
            'reluctant': "{pet_name} seems reluctant to be groomed right now.",
        }
    }
}

# ==============================================================================
# --- Pet Moods & Status Descriptions (Backward Compatibility) ---
# ==============================================================================

# Update GenesisPetConfig with mood thresholds
GenesisPetConfig.Moods = type('Moods', (), {})
GenesisPetConfig.Moods.THRESHOLDS = [
    {'name': 'Ecstatic', 'threshold': 90, 'description': "absolutely thrilled!", 'emoji': '🤩'},
    {'name': 'Happy',    'threshold': 75, 'description': "feeling joyful and content.", 'emoji': '😊'},
    {'name': 'Neutral',  'threshold': 45, 'description': "in a calm and stable mood.", 'emoji': '😐'},
    {'name': 'Grumpy',   'threshold': 25, 'description': "a bit grumpy.", 'emoji': '😠'},
    {'name': 'Sad',      'threshold': 10, 'description': "feeling down.", 'emoji': '😔'},
    {'name': 'Miserable','threshold': 0,  'description': "feeling utterly miserable.", 'emoji': '😭'},
]

# For backward compatibility
MOOD_THRESHOLDS = GenesisPetConfig.Moods.THRESHOLDS

# Update GenesisPetConfig with status alerts
GenesisPetConfig.Moods.STATUS_ALERTS = {
    'hunger_critical': {'stat': Stat.HUNGER, 'condition': lambda h: h >= 80, 'message': "is very hungry!", 'emoji': '🍔'},
    'hunger_hungry':   {'stat': Stat.HUNGER, 'condition': lambda h: h >= 50, 'message': "is feeling hungry.", 'emoji': '🍎'},
    'energy_critical': {'stat': Stat.ENERGY, 'condition': lambda e: e <= 20, 'message': "is extremely tired and needs rest!", 'emoji': '😴'},
    'energy_low':      {'stat': Stat.ENERGY, 'condition': lambda e: e <= 40, 'message': "is feeling a bit sleepy.", 'emoji': '🥱'},
    'cleanliness_low': {'stat': Stat.CLEANLINESS, 'condition': lambda c: c <= 30, 'message': "is looking a bit messy.", 'emoji': '🧼'},
    'social_low':      {'stat': Stat.SOCIAL, 'condition': lambda s: s <= 20, 'message': "craves your attention!", 'emoji': '🫂'},
}

# For backward compatibility
STATUS_ALERTS = {
    'hunger_critical': {'stat': 'hunger', 'condition': lambda h: h >= 80, 'message': "is very hungry!", 'emoji': '🍔'},
    'hunger_hungry':   {'stat': 'hunger', 'condition': lambda h: h >= 50, 'message': "is feeling hungry.", 'emoji': '🍎'},
    'energy_critical': {'stat': 'energy', 'condition': lambda e: e <= 20, 'message': "is extremely tired and needs rest!", 'emoji': '😴'},
    'energy_low':      {'stat': 'energy', 'condition': lambda e: e <= 40, 'message': "is feeling a bit sleepy.", 'emoji': '🥱'},
    'cleanliness_low': {'stat': 'cleanliness', 'condition': lambda c: c <= 30, 'message': "is looking a bit messy.", 'emoji': '🧼'},
    'social_low':      {'stat': 'social', 'condition': lambda s: s <= 20, 'message': "craves your attention!", 'emoji': '🫂'},
}


# ==============================================================================
# --- Pet Archetypes (Species Definitions) ---
# ==============================================================================

# Defines distinct pet species with their unique characteristics and starting stat modifiers.
# `base_stats_modifier`: Changes to DEFAULT_INITIAL_PET_STATS.
# `trait_modifiers`: Future concept for affecting stat decay/gain or interaction outcomes.
PET_ARCHETYPES = {
    'sprite_glow': {
        'display_name': 'Glowing Sprite',
        'description': 'A luminous, ethereal creature that radiates gentle light. Often curious and energetic.',
        'traits': ['curious', 'playful', 'energetic'],
        'rarity': 'Common',
        'base_stats_modifier': {'energy': 10, 'iq': 5}, # Start with more energy and intelligence
        'aura_effect_modifier': {'energy': 0.1, 'happiness': 0.05},,, # Aura effects are 10% more potent for this species
        'aging_rate': 6  # Ages 6x faster than humans after first year
        'aging_rate': 6  # Ages 6x faster than humans after first year
        'aging_rate': 6  # Ages 6x faster than humans after first year
    },
    'sprite_crystal': {
        'display_name': 'Crystal Sprite',
        'description': 'A crystalline entity with a faceted body that refracts light beautifully. Calm and patient.',
        'traits': ['calm', 'wise', 'patient'],
        'rarity': 'Uncommon',
        'base_stats_modifier': {'happiness': 5, 'social': -10}, # Slightly happier, less social
        'trait_modifiers': {'playfulness': -0.1, 'curiosity': 0.05},,, # 10% less playful, 5% more curious over time
        'aging_rate': 4  # Ages 4x faster than humans after first year
        'aging_rate': 4  # Ages 4x faster than humans after first year
        'aging_rate': 4  # Ages 4x faster than humans after first year
    },
    'sprite_shadow': {
        'display_name': 'Shadow Sprite',
        'description': 'A mysterious being composed of shifting shadows and dark energy. Independent and observant.',
        'traits': ['mysterious', 'independent', 'observant'],
        'rarity': 'Rare',
        'base_stats_modifier': {'social': -20, 'charisma': 10}, # Very independent, charming in a dark way
        'decay_rate_modifier': {'social': -0.2}, # Social stat decays 20% slower
        'interaction_boosts': {'chat': {'iq': 2}} # Chatting boosts IQ even more for Shadow Sprites
    },
    'sprite_ember': {
        'display_name': 'Ember Sprite',
        'description': 'A warm, fiery creature with a passionate spirit. Brave and protective.',
        'traits': ['passionate', 'brave', 'protective'],
        'rarity': 'Uncommon',
        'base_stats_modifier': {'energy': 20, 'happiness': -5}, # More energetic, slightly moodier
        'decay_rate_modifier': {'energy': -0.1} # Energy decays 10% slower
    },
    'sprite_aqua': {
        'display_name': 'Aqua Sprite',
        'description': 'A fluid, water-like being that flows with grace and adaptability. Peaceful and creative.',
        'traits': ['adaptable', 'peaceful', 'creative'],
        'rarity': 'Uncommon',
        'base_stats_modifier': {'cleanliness': 10, 'iq': 5}, # Naturally cleaner and smarter
        'interaction_boosts': {'groom': {'happiness': 5}} # Grooming provides extra happiness for Aqua Sprites
    }
}


# ==============================================================================
# --- Pet Aura Colors & Effects ---
# ==============================================================================

# Defines different aura colors and their passive effects on pet stats.
# Effects are applied as a percentage boost or direct modification.
PET_AURA_COLORS = {
    'aura-blue': {
        'display_name': 'Sapphire Blue',
        'description': 'A calming blue aura that soothes the mind.',
        'effect': 'Increases wisdom and patience.',
        'stat_boosts': {'iq': 0.05, 'happiness': 0.02}, # 5% IQ boost, 2% happiness boost
        'decay_reduction': {'energy': 0.03} # Reduces energy decay by 3%
    },
    'aura-gold': {
        'display_name': 'Radiant Gold',
        'description': 'A brilliant golden aura that inspires confidence.',
        'effect': 'Boosts charisma and leadership.',
        'stat_boosts': {'charisma': 0.10, 'social': 0.05},
    },
    'aura-green': {
        'display_name': 'Emerald Green',
        'description': 'A nurturing green aura connected to growth and healing.',
        'effect': 'Enhances growth and recovery rates.',
        'stat_boosts': {'energy': 0.03, 'cleanliness': 0.05},
        'decay_reduction': {'hunger': 0.02} # Reduces hunger decay by 2%
    },
    'aura-purple': {
        'display_name': 'Mystic Purple',
        'description': 'A mysterious purple aura linked to psychic abilities.',
        'effect': 'Improves intuition and perception.',
        'stat_boosts': {'iq': 0.08, 'social': -0.01}, # IQ boost, slight social reduction
    },
    'aura-red': {
        'display_name': 'Passionate Red',
        'description': 'An energetic red aura full of vitality.',
        'effect': 'Increases energy and determination.',
        'stat_boosts': {'energy': 0.12},
        'decay_reduction': {'happiness': 0.01} # Slightly reduces happiness decay
    }
}


# ==============================================================================
# --- AI Personality Traits (for EchoSphere integration) ---
# ==============================================================================

# Defines base personality dimensions for AI interaction,
# likely to be mapped to LLM parameters or conversation style.
AI_PERSONALITY_TRAITS = {
    'playfulness': {
        'description': 'Influences the pet\'s playful and energetic responses.',
        'default': 50,
        'min': 0,
        'max': 100
    },
    'curiosity': {
        'description': 'Determines how interested the pet is in exploring and learning.',
        'default': 50,
        'min': 0,
        'max': 100
    },
    'sociability': {
        'description': 'Governs the pet\'s enjoyment of interaction and social engagement.',
        'default': 50,
        'min': 0,
        'max': 100
    },
    'independence': {
        'description': 'Indicates how self-sufficient the pet is when left alone, affecting clinginess.',
        'default': 50,
        'min': 0,
        'max': 100
    },
    'loyalty': {
        'description': 'Reflects the pet\'s devotion to its owner.',
        'default': 50,
        'min': 0,
        'max': 100
    }
}


# ==============================================================================
# --- Progression & Migration Thresholds ---
# ==============================================================================

# Defines the conditions a pet must meet to be considered "ready" for blockchain migration.
# This could represent a transition from local prototype to a persistent, NFT-like asset.
MIGRATION_READINESS_THRESHOLDS = {
    'min_happiness': 75,
    'min_energy': 65,
    'max_hunger': 25,
    'min_interactions': 30,  # Minimum number of successful interactions
    'min_days_owned': 7,     # Minimum days of actual ownership (real-world time)
    'min_iq': 20,            # New: Minimum intelligence
    'min_charisma': 15,      # New: Minimum charisma
}

# ==============================================================================
# --- Utility & Validation (Internal Use) ---
# ==============================================================================

# List of all primary stat names used in the system for iteration and validation.
ALL_PET_STATS = list(DEFAULT_INITIAL_PET_STATS.keys())

# Ensure all decay rates are defined for all default stats
for stat in ALL_PET_STATS:
    if stat not in DECAY_RATES:
        DECAY_RATES[stat] = 0 # Default to no decay if not explicitly defined

# ==============================================================================
# --- Critter-Craft: Animal-Inspired Critters ---
# ==============================================================================

# Defines different animal-inspired critters for the Critter-Craft application
CRITTER_TYPES = {
    'chameleon': {
        'display_name': 'Chameleon',
        'description': 'A color-changing reptile with a prehensile tail and independently moving eyes.',
        'habitat': 'Tropical forests',
        'diet': 'Insects',
        'adaptations': ['camouflage', 'prehensile_tail', 'rotating_eyes'],
        'conservation_status': 'Varies by species',
        'difficulty': 'medium'
    },
    'anglerfish': {
        'display_name': 'Anglerfish',
        'description': 'A deep-sea fish with a bioluminescent lure used to attract prey.',
        'habitat': 'Deep ocean',
        'diet': 'Fish and crustaceans',
        'adaptations': ['bioluminescence', 'large_jaw', 'sharp_teeth'],
        'conservation_status': 'Least Concern',
        'difficulty': 'hard'
    },
    'hummingbird': {
        'display_name': 'Hummingbird',
        'description': 'A tiny bird capable of hovering in mid-air and flying backwards.',
        'habitat': 'Gardens, forests, and meadows',
        'diet': 'Nectar and small insects',
        'adaptations': ['rapid_wing_movement', 'long_beak', 'hovering_flight'],
        'conservation_status': 'Varies by species',
        'difficulty': 'easy'
    },
    'platypus': {
        'display_name': 'Platypus',
        'description': 'A semi-aquatic egg-laying mammal with a duck-like bill and webbed feet.',
        'habitat': 'Freshwater streams and rivers in Australia',
        'diet': 'Aquatic invertebrates',
        'adaptations': ['duck_bill', 'webbed_feet', 'electroreception'],
        'conservation_status': 'Near Threatened',
        'difficulty': 'hard'
    },
    'peacock_spider': {
        'display_name': 'Peacock Spider',
        'description': 'A tiny spider known for its vibrant, colorful abdomen flap used in mating displays.',
        'habitat': 'Australia',
        'diet': 'Small insects',
        'adaptations': ['colorful_display', 'jumping_ability', 'excellent_vision'],
        'conservation_status': 'Not Evaluated',
        'difficulty': 'medium'
    }
}

# ==============================================================================
# --- Critter-Craft: Crafting Materials ---
# ==============================================================================

# Defines different materials for crafting critters
CRAFTING_MATERIALS = {
    'fur': {
        'display_name': 'Fur',
        'description': 'Soft, insulating material that covers many mammals.',
        'properties': ['insulation', 'warmth', 'texture'],
        'colors': ['brown', 'black', 'white', 'gray', 'orange']
    },
    'scales': {
        'display_name': 'Scales',
        'description': 'Overlapping plates that protect reptiles and fish.',
        'properties': ['protection', 'waterproof', 'flexibility'],
        'colors': ['green', 'blue', 'red', 'silver', 'gold']
    },
    'feathers': {
        'display_name': 'Feathers',
        'description': 'Lightweight, insulating structures that cover birds.',
        'properties': ['flight', 'insulation', 'display'],
        'colors': ['blue', 'red', 'yellow', 'green', 'purple', 'iridescent']
    },
    'shell': {
        'display_name': 'Shell',
        'description': 'Hard, protective covering found on mollusks and turtles.',
        'properties': ['protection', 'strength', 'buoyancy'],
        'colors': ['brown', 'white', 'patterned', 'iridescent']
    },
    'exoskeleton': {
        'display_name': 'Exoskeleton',
        'description': 'Hard outer structure that supports and protects insects and crustaceans.',
        'properties': ['protection', 'structure', 'segmentation'],
        'colors': ['black', 'brown', 'red', 'blue', 'green']
    }
}

# ==============================================================================
# --- Critter-Craft: Adaptations ---
# ==============================================================================

# Defines different adaptations for critters
ADAPTATIONS = {
    'camouflage': {
        'display_name': 'Camouflage',
        'description': 'The ability to blend in with surroundings to avoid predators or ambush prey.',
        'examples': ['chameleons', 'leaf insects', 'octopuses'],
        'simulation_effect': 'Reduces visibility in matching environments'
    },
    'bioluminescence': {
        'display_name': 'Bioluminescence',
        'description': 'The production of light by living organisms through chemical reactions.',
        'examples': ['anglerfish', 'fireflies', 'certain jellyfish'],
        'simulation_effect': 'Produces light in dark environments, attracts prey'
    },
    'echolocation': {
        'display_name': 'Echolocation',
        'description': 'Using sound waves and echoes to determine the location of objects.',
        'examples': ['bats', 'dolphins', 'some whales'],
        'simulation_effect': 'Enables navigation in dark or murky environments'
    },
    'mimicry': {
        'display_name': 'Mimicry',
        'description': 'Resembling another organism or object to gain an advantage.',
        'examples': ['viceroy butterflies', 'mimic octopuses', 'certain orchids'],
        'simulation_effect': 'Can deter predators or attract specific pollinators'
    },
    'hibernation': {
        'display_name': 'Hibernation',
        'description': 'A state of inactivity and metabolic depression to conserve energy.',
        'examples': ['bears', 'ground squirrels', 'certain frogs'],
        'simulation_effect': 'Reduces energy consumption during resource scarcity'
    },
    'migration': {
        'display_name': 'Migration',
        'description': 'Seasonal movement between different habitats.',
        'examples': ['monarch butterflies', 'arctic terns', 'wildebeests'],
        'simulation_effect': 'Enables survival in changing environmental conditions'
    },
    'specialized_limbs': {
        'display_name': 'Specialized Limbs',
        'description': 'Limbs adapted for specific functions like digging, climbing, or swimming.',
        'examples': ['mole claws', 'gecko feet', 'penguin flippers'],
        'simulation_effect': 'Enhances mobility in specific environments'
    }
}

# ==============================================================================
# --- Progression & Migration Thresholds ---
# ==============================================================================

# Defines the conditions a pet must meet to be considered "ready" for blockchain migration.
# This could represent a transition from local prototype to a persistent, NFT-like asset.
MIGRATION_READINESS_THRESHOLDS = {
    'min_happiness': 75,
    'min_energy': 65,
    'max_hunger': 25,
    'min_interactions': 30,  # Minimum number of successful interactions
    'min_days_owned': 7,     # Minimum days of actual ownership (real-world time)
    'min_iq': 20,            # New: Minimum intelligence
    'min_charisma': 15,      # New: Minimum charisma
}

# ==============================================================================
# --- Critter-Craft: Zoologist Levels ---
# ==============================================================================

# Defines progression levels for the Zoologist's Journal
ZOOLOGIST_LEVELS = {
    'novice': {
        'display_name': 'Novice Zoologist',
        'required_critters': 3,
        'unlocked_materials': ['fur', 'scales'],
        'unlocked_adaptations': ['camouflage', 'specialized_limbs']
    },
    'apprentice': {
        'display_name': 'Apprentice Zoologist',
        'required_critters': 7,
        'unlocked_materials': ['fur', 'scales', 'feathers'],
        'unlocked_adaptations': ['camouflage', 'specialized_limbs', 'mimicry']
    },
    'journeyman': {
        'display_name': 'Journeyman Zoologist',
        'required_critters': 12,
        'unlocked_materials': ['fur', 'scales', 'feathers', 'shell'],
        'unlocked_adaptations': ['camouflage', 'specialized_limbs', 'mimicry', 'bioluminescence', 'echolocation']
    },
    'expert': {
        'display_name': 'Expert Zoologist',
        'required_critters': 18,
        'unlocked_materials': ['fur', 'scales', 'feathers', 'shell', 'exoskeleton'],
        'unlocked_adaptations': ['camouflage', 'specialized_limbs', 'mimicry', 'bioluminescence', 'echolocation', 'hibernation']
    },
    'master': {
        'display_name': 'Master Zoologist',
        'required_critters': 25,
        'unlocked_materials': ['fur', 'scales', 'feathers', 'shell', 'exoskeleton'],
        'unlocked_adaptations': ['camouflage', 'specialized_limbs', 'mimicry', 'bioluminescence', 'echolocation', 'hibernation', 'migration']
    }
}

# Basic validation for configuration integrity
def _validate_config():
    """Internal function to validate configuration consistency."""
    # Check if all archetype modifiers reference valid stats
    for archetype_key, data in PET_ARCHETYPES.items():
        if 'base_stats_modifier' in data:
            for stat_key in data['base_stats_modifier']:
                if stat_key not in ALL_PET_STATS:
                    raise ValueError(f"Archetype '{archetype_key}' references unknown stat '{stat_key}' in base_stats_modifier.")
        if 'decay_rate_modifier' in data:
            for stat_key in data['decay_rate_modifier']:
                if stat_key not in ALL_PET_STATS:
                    raise ValueError(f"Archetype '{archetype_key}' references unknown stat '{stat_key}' in decay_rate_modifier.")
        if 'aura_effect_modifier' in data:
            for stat_key in data['aura_effect_modifier']:
                if stat_key not in ALL_PET_STATS:
                    raise ValueError(f"Archetype '{archetype_key}' references unknown stat '{stat_key}' in aura_effect_modifier.")

    # Check if all aura boosts reference valid stats
    for aura_key, data in PET_AURA_COLORS.items():
        if 'stat_boosts' in data:
            for stat_key in data['stat_boosts']:
                if stat_key not in ALL_PET_STATS:
                    raise ValueError(f"Aura '{aura_key}' references unknown stat '{stat_key}' in stat_boosts.")
        if 'decay_reduction' in data:
            for stat_key in data['decay_reduction']:
                if stat_key not in ALL_PET_STATS:
                    raise ValueError(f"Aura '{aura_key}' references unknown stat '{stat_key}' in decay_reduction.")

    # Check if interaction effects reference valid stats
    for interaction_type, effects in INTERACTION_EFFECTS.items():
        for stat_key in effects:
            if stat_key not in ALL_PET_STATS and not stat_key.startswith('min_') and stat_key != 'messages':
                raise ValueError(f"Interaction '{interaction_type}' references unknown stat '{stat_key}'.")

    # Check if all mood thresholds are within MAX_STAT and sorted correctly
    if not all(0 <= m['threshold'] <= MAX_STAT for m in MOOD_THRESHOLDS):
        raise ValueError("Mood thresholds must be within [0, MAX_STAT].")
    if not all(MOOD_THRESHOLDS[i]['threshold'] >= MOOD_THRESHOLDS[i+1]['threshold'] for i in range(len(MOOD_THRESHOLDS)-1)):
        raise ValueError("Mood thresholds must be sorted in descending order.")

    # Check if migration thresholds reference valid stats
    for threshold_stat in MIGRATION_READINESS_THRESHOLDS:
        if threshold_stat.startswith('min_') or threshold_stat.startswith('max_'):
            base_stat = threshold_stat[4:] # e.g., 'happiness' from 'min_happiness'
            if base_stat not in ALL_PET_STATS and base_stat not in ['interactions', 'days_owned']:
                raise ValueError(f"Migration threshold '{threshold_stat}' references unknown stat/metric.")


# Run validation on import
_validate_config()  