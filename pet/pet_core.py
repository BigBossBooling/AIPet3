# prometheus_protocol/core/pet_core.py (Conceptual Path for full app)
import json
import time
import uuid
from typing import Dict, Any, List, Optional
from dataclasses import dataclass, field

# Import constants from config.py
from .config import (
    MAX_STAT, STAT_DECAY_RATE, HAPPINESS_DECAY_RATE,
    FEED_HUNGER_RESTORE, PLAY_HAPPINESS_BOOST, PLAY_ENERGY_COST,
    MOOD_THRESHOLD_HAPPY, MOOD_THRESHOLD_SAD,
    PET_ARCHETYPES, PET_AURA_COLORS, AI_PERSONALITY_TRAITS,
    MIGRATION_READINESS_THRESHOLDS # Added for migration logic in main
)

@dataclass
class InteractionRecord:
    """Represents a single interaction event with the pet."""
    timestamp: int     # Unix nanoseconds timestamp
    type: str          # e.g., "feed", "play", "chat"
    details: Optional[str] = None # Optional details, e.g., "fed_berry"

@dataclass
class Pet:
    """
    Represents a CritterCraft Genesis Pet.
    This is the core data model for our AI digital companion.
    """
    id: str = field(default_factory=lambda: str(uuid.uuid4())) # Unique ID for potential blockchain migration
    name: str
    species: str            # e.g., 'sprite_glow', 'sprite_crystal' - maps to PET_ARCHETYPES
    aura_color: str         # e.g., 'aura-blue', 'aura-gold' - maps to PET_AURA_COLORS
    
    # Core Vitals (0-MAX_STAT)
    hunger: int = 50
    happiness: int = 50
    energy: int = 50
    
    # Dynamic Attributes
    mood: str = "Neutral"   # Derived: "Happy", "Neutral", "Sad", "Very Grumpy", "Exhausted", "Thrilled", "A bit down"
    
    # Personality Traits (Conceptual, for AI influence and EchoSphere integration)
    personality_traits: Dict[str, int] = field(default_factory=lambda: {
        k: v["default"] for k, v in AI_PERSONALITY_TRAITS.items() # Initialize from config
    })
    
    creation_timestamp: int = field(default_factory=lambda: time.time_ns()) # Unix nanoseconds
    last_active_timestamp: int = field(default_factory=lambda: time.time_ns()) # For offline progress calculation
    
    interaction_history: List[InteractionRecord] = field(default_factory=list)

    def __post_init__(self):
        """
        Perform post-initialization validation and initial setup.
        Ensures name, species, and aura_color are valid, and sets initial mood.
        """
        self.name = self.name.strip()
        if not self.name:
            raise ValueError("Pet name cannot be empty.")
        if len(self.name) > 20: # Enforce max length
            raise ValueError("Pet name exceeds 20 characters (max 20).")
        
        # Validate species and aura color against configured archetypes/colors
        if self.species not in PET_ARCHETYPES:
            raise ValueError(f"Invalid pet species: '{self.species}'. Choose from {list(PET_ARCHETYPES.keys())}.")
        if self.aura_color not in PET_AURA_COLORS:
            raise ValueError(f"Invalid aura color: '{self.aura_color}'. Choose from {list(PET_AURA_COLORS.keys())}.")
            
        self._update_mood() # Set initial mood based on happiness

    def _cap_stat(self, stat_value: int) -> int:
        """Helper to cap stat values between 0 and MAX_STAT."""
        return max(0, min(stat_value, MAX_STAT))

    def _update_mood(self):
        """
        Update the pet's mood based on its current hunger, happiness, and energy levels.
        Conceptual: AI could add nuance here based on personality_traits.
        """
        if self.hunger > 70 and self.energy < 30:
            self.mood = "Very Grumpy"
        elif self.happiness >= MOOD_THRESHOLD_HAPPY:
            self.mood = "Happy"
        elif self.happiness <= MOOD_THRESHOLD_SAD:
            self.mood = "Sad"
        elif self.hunger > (MAX_STAT * 0.8): # Very hungry
            self.mood = "Grumpy"
        elif self.energy < (MAX_STAT * 0.2): # Very low energy
            self.mood = "Exhausted"
        else:
            self.mood = "Neutral"
        # The AI Personality Engine (EchoSphere) would further refine this in full app.

    def _add_interaction_record(self, type: str, details: Optional[str] = None):
        """Add a new interaction to the pet's history."""
        self.interaction_history.append(
            InteractionRecord(timestamp=time.time_ns(), type=type, details=details)
        )
        # Limit history size to prevent excessive storage (e.g., last 100 interactions for MVP)
        if len(self.interaction_history) > 100:
            self.interaction_history = self.interaction_history[-100:]

    def feed(self):
        """Feed the pet, restoring hunger and slightly boosting happiness."""
        self.hunger = self._cap_stat(self.hunger - FEED_HUNGER_RESTORE) # Hunger decreases
        self.happiness = self._cap_stat(self.happiness + 5) # Small happiness boost
        self._update_mood()
        self._add_interaction_record("feed", f"Restored {FEED_HUNGER_RESTORE} sustenance.")
        # Conceptual: AI Personality Engine could generate a pet reaction based on personality.

    def play(self):
        """Play with the pet, boosting happiness, costing energy, and increasing hunger."""
        if self.energy < PLAY_ENERGY_COST:
            # Conceptual: AI Personality Engine could generate a pet refusal response ("too tired").
            return False, f"{self.name} is too tired to play! Energy: {self.energy}/{MAX_STAT}."
        
        self.energy = self._cap_stat(self.energy - PLAY_ENERGY_COST)
        self.happiness = self._cap_stat(self.happiness + PLAY_HAPPINESS_BOOST)
        self.hunger = self._cap_stat(self.hunger + 10) # Playing increases hunger
        self._update_mood()
        self._add_interaction_record("play", f"Boosted {PLAY_HAPPINESS_BOOST} happiness.")
        # Conceptual: AI Personality Engine could generate a playful pet reaction based on personality.
        return True, f"{self.name} enjoyed playing! Energy: {self.energy}/{MAX_STAT}, Hunger: {self.hunger}/{MAX_STAT}."

    def tick(self, current_time_ns: int):
        """
        Simulates the passage of time, decaying stats.
        Calculates offline progress if pet was inactive.
        """
        time_diff_ns = current_time_ns - self.last_active_timestamp
        # Convert nanoseconds to game intervals. Ensure GAME_INTERVAL_SECONDS is not zero to avoid division by zero.
        intervals_passed = 0
        if GAME_INTERVAL_SECONDS > 0:
            intervals_passed = time_diff_ns // (GAME_INTERVAL_SECONDS * 1_000_000_000) # ns to game intervals

        if intervals_passed > 0:
            self.hunger = self._cap_stat(self.hunger + int(STAT_DECAY_RATE * intervals_passed))
            self.energy = self._cap_stat(self.energy - int(STAT_DECAY_RATE * intervals_passed))
            self.happiness = self._cap_stat(self.happiness - int(HAPPINESS_DECAY_RATE * intervals_passed))
            self._update_mood()
            self.last_active_timestamp = current_time_ns # Update last active time

            # Conceptual: AI Personality Engine could analyze long periods of inactivity
            # and adjust personality traits (e.g., a "sad" trait might increase).
            # It could also generate a pet comment about being lonely.
            # print(f"DEBUG: {intervals_passed} intervals passed. Stats decayed.") # For debug logging

    def status(self) -> str:
        """Return a formatted string summary of the pet's current status."""
        return (
            f"--- {self.name}'s Status ---\n"
            f"ID: {self.id}\n"
            f"Species: {PET_ARCHETYPES.get(self.species, {}).get('display_name', self.species)}\n"
            f"Aura: {self.aura_color}\n"
            f"Mood: {self.mood}\n"
            f"Sustenance: {self.hunger}/{MAX_STAT}\n"
            f"Energy: {self.energy}/{MAX_STAT}\n"
            f"Happiness: {self.happiness}/{MAX_STAT}\n"
            f"Personality: {', '.join([f'{k}: {v}' for k, v in self.personality_traits.items()]) if self.personality_traits else 'N/A'}\n"
            f"Created: {time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime(self.creation_timestamp // 1_000_000_000))}\n"
            f"Last Active: {time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime(self.last_active_timestamp // 1_000_000_000))}"
            # Conceptual: AI Personality Engine could generate a more descriptive status based on pet's personality
        )

    def to_json(self) -> str:
        """Serialize the Pet object to a JSON string for persistence."""
        # dataclasses.asdict() is more robust for serialization than self.__dict__
        from dataclasses import asdict
        data = asdict(self)
        
        # Convert InteractionRecord objects to dicts for JSON serialization
        data['interaction_history'] = [asdict(rec) for rec in self.interaction_history]
        
        return json.dumps(data, indent=4) # Use indent for readability in local storage

    @classmethod
    def from_json(cls, json_string: str) -> 'Pet':
        """Deserialize a Pet object from a JSON string."""
        data = json.loads(json_string)
        
        # Ensure UUID string conversion back if necessary (dataclass handles str normally)
        # Reconstruct InteractionRecord objects from their dict representations
        if 'interaction_history' in data and data['interaction_history'] is not None:
            data['interaction_history'] = [InteractionRecord(**rec) for rec in data['interaction_history']]
        else:
            data['interaction_history'] = [] # Ensure it's always a list

        # Handle default values for newly added fields if loading older data
        # Use .get() with defaults for robustness against older save formats
        data['id'] = data.get('id', str(uuid.uuid4()))
        data['creation_timestamp'] = data.get('creation_timestamp', time.time_ns())
        data['last_active_timestamp'] = data.get('last_active_timestamp', time.time_ns())
        data['personality_traits'] = data.get('personality_traits', {k: v["default"] for k, v in AI_PERSONALITY_TRAITS.items()})
        data['species'] = data.get('species', list(PET_ARCHETYPES.keys())[0]) # Fallback to first archetype
        data['aura_color'] = data.get('aura_color', list(PET_AURA_COLORS.keys())[0]) # Fallback to first color

        return cls(**data)