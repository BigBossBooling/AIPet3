# prometheus_protocol/core/pet_core.py (Conceptual Path)
import json
import time
import uuid
from typing import Dict, Any, List, Optional
from dataclasses import dataclass, field

from .config import ( # Import constants from config
    MAX_STAT, STAT_DECAY_RATE, HAPPINESS_DECAY_RATE,
    FEED_HUNGER_RESTORE, PLAY_HAPPINESS_BOOST, PLAY_ENERGY_COST,
    MOOD_THRESHOLD_HAPPY, MOOD_THRESHOLD_SAD,
    PET_ARCHETYPES, PET_AURA_COLORS, AI_PERSONALITY_TRAITS # For initial pet creation
)

@dataclass
class InteractionRecord:
    """Represents a single interaction event with the pet."""
    timestamp: int     # Unix nanoseconds
    type: str          # e.g., "feed", "play", "chat"
    details: Optional[str] = None # Optional details, e.g., "fed_berry"

@dataclass
class Pet:
    """
    Represents a CritterCraft Genesis Pet.
    This is the core data model for our AI digital companion.
    """
    name: str
    species: str            # e.g., 'sprite_glow', 'sprite_crystal'
    aura_color: str         # e.g., 'aura-blue', 'aura-gold'
    id: str = field(default_factory=lambda: str(uuid.uuid4())) # Unique ID for potential blockchain migration
    
    # Core Vitals (0-MAX_STAT)
    hunger: int = 50
    happiness: int = 50
    energy: int = 50
    
    # Dynamic Attributes
    mood: str = "Neutral"   # Derived: "Happy", "Neutral", "Sad"
    
    # Personality Traits (Conceptual, for AI influence)
    personality_traits: Dict[str, int] = field(default_factory=lambda: {
        k: v["default"] for k, v in AI_PERSONALITY_TRAITS.items() # Initialize from config
    })
    
    creation_timestamp: int = field(default_factory=lambda: time.time_ns()) # Unix nanoseconds
    last_active_timestamp: int = field(default_factory=lambda: time.time_ns()) # For offline progress calculation
    
    interaction_history: List[InteractionRecord] = field(default_factory=list)

    def __post_init__(self):
        """Perform post-initialization validation and initial setup."""
        self.name = self.name.strip()
        if not self.name:
            raise ValueError("Pet name cannot be empty.")
        if len(self.name) > 20: # Example limit
            raise ValueError("Pet name exceeds 20 characters.")
        
        if self.species not in PET_ARCHETYPES:
            raise ValueError(f"Invalid pet species: {self.species}")
        if self.aura_color not in PET_AURA_COLORS:
            raise ValueError(f"Invalid aura color: {self.aura_color}")
            
        self._update_mood() # Set initial mood based on happiness

    def _update_mood(self):
        """Update the pet's mood based on its happiness level."""
        if self.happiness >= MOOD_THRESHOLD_HAPPY:
            self.mood = "Happy"
        elif self.happiness <= MOOD_THRESHOLD_SAD:
            self.mood = "Sad"
        else:
            self.mood = "Neutral"
        # Conceptual: AI could add nuance here based on personality_traits

    def _add_interaction_record(self, type: str, details: Optional[str] = None):
        """Add a new interaction to the pet's history."""
        self.interaction_history.append(
            InteractionRecord(timestamp=time.time_ns(), type=type, details=details)
        )
        # Limit history size to prevent bloat (e.g., last 100 interactions)
        if len(self.interaction_history) > 100:
            self.interaction_history = self.interaction_history[-100:]

    def _cap_stat(self, stat_value: int) -> int:
        """Helper to cap stat values between 0 and MAX_STAT."""
        return max(0, min(stat_value, MAX_STAT))

    def feed(self):
        """Feed the pet, restoring hunger."""
        self.hunger = self._cap_stat(self.hunger + FEED_HUNGER_RESTORE)
        self.happiness = self._cap_stat(self.happiness + 5) # Small happiness boost
        self._update_mood()
        self._add_interaction_record("feed", f"Restored {FEED_HUNGER_RESTORE} hunger.")
        # Conceptual: AI could generate a pet reaction based on personality.

    def play(self):
        """Play with the pet, boosting happiness and costing energy."""
        if self.energy < PLAY_ENERGY_COST:
            # Conceptual: AI could generate a pet refusal response ("too tired").
            return False, "Pet is too tired to play!"
        
        self.energy = self._cap_stat(self.energy - PLAY_ENERGY_COST)
        self.happiness = self._cap_stat(self.happiness + PLAY_HAPPINESS_BOOST)
        self._update_mood()
        self._add_interaction_record("play", f"Boosted {PLAY_HAPPINESS_BOOST} happiness.")
        # Conceptual: AI could generate a playful pet reaction based on personality.
        return True, "Played with pet!"

    def tick(self, current_time_ns: int):
        """
        Simulates the passage of time, decaying stats.
        Calculates offline progress if pet was inactive.
        """
        # Calculate time difference for offline progress
        time_diff_ns = current_time_ns - self.last_active_timestamp
        # Convert to game intervals
        intervals_passed = time_diff_ns // (GAME_INTERVAL_SECONDS * 1_000_000_000) # ns to seconds

        if intervals_passed > 0:
            self.hunger = self._cap_stat(self.hunger + int(STAT_DECAY_RATE * intervals_passed))
            self.energy = self._cap_stat(self.energy - int(STAT_DECAY_RATE * intervals_passed))
            self.happiness = self._cap_stat(self.happiness - int(HAPPINESS_DECAY_RATE * intervals_passed))
            self._update_mood()
            self.last_active_timestamp = current_time_ns # Update last active time

            # Conceptual: AI could analyze long periods of inactivity
            # and adjust personality traits (e.g., a "sad" trait might increase).
            # AI could also generate a pet comment about being lonely.
            # print(f"DEBUG: {intervals_passed} intervals passed. Stats decayed.") # For debug
        
        # Ensure stats don't go below 0 or above MAX_STAT (handled by _cap_stat, but double check)
        self.hunger = self._cap_stat(self.hunger)
        self.energy = self._cap_stat(self.energy)
        self.happiness = self._cap_stat(self.happiness)

    def status(self) -> str:
        """Return a string summary of the pet's current status."""
        return (
            f"{self.name} ({self.species}, {self.aura_color} aura)\n"
            f"Mood: {self.mood}\n"
            f"Sustenance: {self.hunger}/{MAX_STAT}\n"
            f"Energy: {self.energy}/{MAX_STAT}\n"
            f"Happiness: {self.happiness}/{MAX_STAT}\n"
            f"Personality: {self.personality_traits}\n"
            f"Last Active: {time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime(self.last_active_timestamp // 1_000_000_000))}"
            # Conceptual: AI could generate a more descriptive status based on pet's personality
        )

    def to_json(self) -> str:
        """Serialize the Pet object to a JSON string for persistence."""
        # Use a dictionary representation to avoid dataclass serialization complexities if needed,
        # but direct dataclass serialization to JSON often works with dataclasses.asdict
        # For simplicity, let's use json.dumps directly on dataclass for V1
        return json.dumps(self.__dict__, default=str) # default=str handles datetime/UUID if not converted

    @classmethod
    def from_json(cls, json_string: str) -> 'Pet':
        """Deserialize a Pet object from a JSON string."""
        data = json.loads(json_string)
        # Handle UUID string conversion back to UUID object if __init__ expects it
        # For simple string IDs, direct assignment is fine.
        data['id'] = str(data['id']) # Ensure it's string for consistency
        data['creation_timestamp'] = int(data['creation_timestamp'])
        data['last_active_timestamp'] = int(data['last_active_timestamp'])
        
        # Reconstruct InteractionRecord objects
        data['interaction_history'] = [InteractionRecord(**rec) for rec in data.get('interaction_history', [])]
        
        # Ensure personality_traits is a dict even if missing (from old data)
        data['personality_traits'] = data.get('personality_traits', {})
        
        # Dynamically set species and aura_color if they weren't in config (from old data)
        if 'species' not in data: data['species'] = PET_ARCHETYPES.keys().__iter__().__next__()
        if 'aura_color' not in data: data['aura_color'] = PET_AURA_COLORS.keys().__iter__().__next__()


        return cls(**data)