# prometheus_protocol/core/config.py (Conceptual Path)
# Centralized configuration for CritterCraft MVP.

from typing import Dict, Any

# --- Pet Core Attributes ---
MAX_STAT: int = 100
STAT_DECAY_RATE: int = 2        # Hunger/Energy decay per game tick
HAPPINESS_DECAY_RATE: int = 3   # Happiness decay per game tick
MOOD_THRESHOLD_HAPPY: int = 70  # Happiness above this is 'Happy'
MOOD_THRESHOLD_SAD: int = 30    # Happiness below this is 'Sad'

# --- Game Loop & Persistence ---
GAME_INTERVAL_SECONDS: int = 7 # How often the pet's state ticks (e.g., hunger increases)
LOCAL_STORAGE_KEY: str = "critterCraftPetMVP_v3" # Key for local persistence (increment version for new state structures)

# --- Interaction Values ---
FEED_HUNGER_RESTORE: int = 20
PLAY_HAPPINESS_BOOST: int = 25
PLAY_ENERGY_COST: int = 10

# --- Pet Archetype & Aura Definitions (Conceptual, for future modularity) ---
# In a real app, these would come from a database or asset bundle
PET_ARCHETYPES: Dict[str, Dict[str, Any]] = {
    "sprite_glow": {"display_name": "Glow Sprite", "base_mood": "Curious", "base_stats": {"strength": 50, "agility": 60}},
    "sprite_crystal": {"display_name": "Crystal Sprite", "base_mood": "Calm", "base_stats": {"strength": 40, "agility": 70}},
    "sprite_bio": {"display_name": "Bio-Lume", "base_mood": "Playful", "base_stats": {"strength": 55, "agility": 55}},
}

PET_AURA_COLORS: Dict[str, str] = {
    "aura-blue": "#00aaff",
    "aura-green": "#00cc88",
    "aura-pink": "#ff66aa",
    "aura-gold": "#ffcc00",
}

# --- AI Integration (Conceptual Placeholders) ---
# For future integration with Gemini, etc.
AI_PERSONALITY_TRAITS: Dict[str, Any] = {
    "playfulness": {"min": 0, "max": 100, "default": 50},
    "curiosity": {"min": 0, "max": 100, "default": 50},
    # ... other conceptual traits
}

# --- Blockchain Migration (Conceptual) ---
MIGRATION_READINESS_THRESHOLDS: Dict[str, int] = {
    "min_happiness": 60,
    "min_hunger": 40, # Not too hungry
    "min_energy": 30,
    "min_interactions": 10, # Needs some history
}