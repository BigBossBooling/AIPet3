# pet/pet_core.py
"""
Core functionality for the Genesis Pet virtual pet system.

This module defines the Pet class and related functionality following the KISS principles:
- K (Know Your Core, Keep it Clear): Clear separation of data models and logic
- I (Iterate Intelligently): Structured for easy updates and maintenance
- S (Systematize for Scalability): Modular design with clear interfaces
- S (Sense the Landscape & Stimulate Engagement): Designed for user engagement
"""

from __future__ import annotations
import json
import time
import uuid
from typing import Dict, Any, List, Optional, Callable
from dataclasses import dataclass, field, asdict
from enum import Enum

# Import constants from the centralized config file
from pet.config import (
    Stat, Mood as ConfigMood, PersonalityTrait as ConfigPersonalityTrait,
    GenesisPetConfig
)

# --- Custom Exceptions for Clarity ---
class PetError(Exception):
    """Base exception for pet-related errors."""
    pass

class PetInitializationError(PetError):
    """Raised when a pet cannot be initialized due to invalid parameters."""
    pass

class InsufficientEnergyError(PetError):
    """Raised when an action cannot be performed due to low energy."""
    pass

# --- Enums for Type Safety and Readability ---
# Using the Mood enum from config.py for consistency
# Mapping to maintain backward compatibility
class Mood(Enum):
    """
    Pet mood states, aligned with the centralized config.
    This maintains backward compatibility while using the centralized definitions.
    """
    THRILLED = "Thrilled"
    HAPPY = "Happy"
    NEUTRAL = "Neutral"
    SAD = "A bit down"
    VERY_SAD = "Sad"
    GRUMPY = "Grumpy"
    VERY_GRUMPY = "Very Grumpy"
    EXHAUSTED = "Exhausted"
    
    @classmethod
    def from_config_mood(cls, config_mood: ConfigMood) -> 'Mood':
        """Convert from config.py Mood to pet_core.py Mood"""
        mapping = {
            ConfigMood.ECSTATIC: cls.THRILLED,
            ConfigMood.HAPPY: cls.HAPPY,
            ConfigMood.NEUTRAL: cls.NEUTRAL,
            ConfigMood.SAD: cls.VERY_SAD,
            ConfigMood.MISERABLE: cls.VERY_SAD,
            ConfigMood.GRUMPY: cls.GRUMPY,
            ConfigMood.FRUSTRATED: cls.VERY_GRUMPY,
            ConfigMood.BORED: cls.EXHAUSTED,
            # Default fallback
            None: cls.NEUTRAL
        }
        return mapping.get(config_mood, cls.NEUTRAL)

class InteractionType(Enum):
    """Types of interactions a user can have with their pet."""
    FEED = "feed"
    PLAY = "play"
    CHAT = "chat"
    GROOM = "groom"
    TICK_DECAY = "tick_decay"

# Use the PersonalityTrait enum from config.py directly
# This class provides a mapping for backward compatibility
class PersonalityTrait:
    """
    Maps to the centralized PersonalityTrait enum in config.py.
    This maintains backward compatibility while using the centralized definitions.
    """
    @staticmethod
    def get_traits() -> Dict[str, Dict[str, Any]]:
        """Get all personality traits with their default values."""
        return {trait.name.lower(): {
            "min": 0, 
            "max": 100, 
            "default": 50
        } for trait in ConfigPersonalityTrait}

# --- Core Data Models ---
@dataclass
class InteractionRecord:
    """Represents a single interaction event with the pet."""
    timestamp: int
    type: InteractionType
    details: Optional[str] = None

@dataclass
class Pet:
    """
    Represents a CritterCraft Genesis Pet.
    This is the core data model, focused on state. Logic is handled by PetLogicManager.
    
    Following KISS principles:
    - K: Clear separation of data and logic
    - I: Structured for easy updates
    - S: Modular design with clear interfaces
    - S: Designed for user engagement
    """
    name: str
    species: str
    aura_color: str
    id: str = field(default_factory=lambda: str(uuid.uuid4()))

    # Core Vitals - initialized from GenesisPetConfig
    hunger: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.HUNGER])
    energy: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.ENERGY])
    happiness: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.HAPPINESS])
    iq: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.IQ])
    charisma: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.CHARISMA])
    cleanliness: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.CLEANLINESS])
    social: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.SOCIAL])

    # Dynamic Attributes
    mood: Mood = Mood.NEUTRAL

    # Personality - using the centralized trait definitions
    personality_traits: Dict[str, int] = field(default_factory=lambda: {
        trait.name.lower(): 50 for trait in ConfigPersonalityTrait
    })

    # Timestamps
    creation_timestamp: int = field(default_factory=time.time_ns)
    last_active_timestamp: int = field(default_factory=time.time_ns)

    # History
    interaction_history: List[InteractionRecord] = field(default_factory=list)

    def __post_init__(self):
        """Perform post-initialization validation."""
        self.name = self.name.strip()
        if not self.name or len(self.name) > 20 or not self.name.isprintable():
            raise PetInitializationError("Pet name must be 1-20 printable characters.")
        
        # Validate species and aura color against config
        if self.species not in GenesisPetConfig.Archetypes.DEFINITIONS:
            raise PetInitializationError(f"Invalid species: {self.species}.")
            
        valid_auras = GenesisPetConfig.Auras.DEFINITIONS.keys()
        if self.aura_color not in valid_auras:
            raise PetInitializationError(f"Invalid aura color: {self.aura_color}.")
        
        # Apply species-specific stat modifiers
        species_info = GenesisPetConfig.Archetypes.DEFINITIONS.get(self.species, {})
        stat_modifiers = species_info.get('stat_modifiers', {})
        
        for stat, modifier in stat_modifiers.items():
            if hasattr(self, stat.name.lower()):
                current_value = getattr(self, stat.name.lower())
                setattr(self, stat.name.lower(), 
                        max(0, min(GenesisPetConfig.Core.MAX_STAT, current_value + modifier)))
        
        # Initial mood will be set by the manager after creation

    def to_dict(self) -> Dict[str, Any]:
        """Serialize the Pet object to a dictionary, handling Enums."""
        data = asdict(self)
        data['mood'] = self.mood.value
        data['interaction_history'] = [
            {**asdict(rec), 'type': rec.type.value} for rec in self.interaction_history
        ]
        return data

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> Pet:
        """Deserialize a Pet object from a dictionary, handling Enums and defaults."""
        # Convert enum strings back to Enum members
        data['mood'] = Mood(data.get('mood', Mood.NEUTRAL.value))
        history_data = data.get('interaction_history', [])
        data['interaction_history'] = [
            InteractionRecord(
                timestamp=rec['timestamp'],
                type=InteractionType(rec['type']),
                details=rec.get('details')
            ) for rec in history_data
        ]
        
        # Ensure core fields have defaults for backward compatibility
        data.setdefault('id', str(uuid.uuid4()))
        data.setdefault('personality_traits', {k: v["default"] for k, v in AI_PERSONALITY_TRAITS.items()})

        return cls(**data)
        
# --- Logic Engines (Separation of Concerns) ---
class MoodEngine:
    """
    Determines a pet's mood using a scalable, rule-based system.
    
    Following KISS principles:
    - K: Clear, focused responsibility (mood determination only)
    - I: Easy to update with new rules
    - S: Systematized with prioritized rules
    - S: Designed for engaging pet responses
    """
    # Define mood thresholds from config
    MOOD_THRESHOLD_HAPPY = 75  # Threshold for happy mood
    MOOD_THRESHOLD_SAD = 25    # Threshold for sad mood
    
    # Prioritized mood rules - evaluated in order
    _mood_rules: List[tuple[Callable[[Pet], bool], Mood]] = [
        # Thrilled when very happy and energetic
        (lambda p: p.happiness >= MOOD_THRESHOLD_HAPPY and p.energy > 50, Mood.THRILLED),
        # Happy when above happiness threshold
        (lambda p: p.happiness >= MOOD_THRESHOLD_HAPPY, Mood.HAPPY),
        # Exhausted when energy is very low
        (lambda p: p.energy < 20, Mood.EXHAUSTED),
        # Very grumpy when hungry and tired
        (lambda p: p.hunger > 80 and p.energy < 30, Mood.VERY_GRUMPY),
        # Grumpy when hungry
        (lambda p: p.hunger > 70, Mood.GRUMPY),
        # Very sad when happiness is very low
        (lambda p: p.happiness <= MOOD_THRESHOLD_SAD, Mood.VERY_SAD),
        # Sad when happiness is moderately low
        (lambda p: p.happiness < 40, Mood.SAD),
        # Default case - neutral mood
        (lambda p: True, Mood.NEUTRAL)
    ]

    @classmethod
    def determine_mood(cls, pet: Pet) -> Mood:
        """
        Iterates through rules to find the current mood.
        
        Args:
            pet: The Pet instance to evaluate
            
        Returns:
            The determined Mood enum value
        """
        for condition, mood in cls._mood_rules:
            if condition(pet):
                return mood
        return Mood.NEUTRAL  # Should be unreachable due to the default case
        
    @classmethod
    def get_mood_emoji(cls, mood: Mood) -> str:
        """
        Get the emoji representation of a mood.
        
        Args:
            mood: The Mood enum value
            
        Returns:
            An emoji string representing the mood
        """
        # Map moods to the emojis defined in config
        mood_to_emoji = {
            Mood.THRILLED: '🤩',
            Mood.HAPPY: '😊',
            Mood.NEUTRAL: '😐',
            Mood.SAD: '😔',
            Mood.VERY_SAD: '😭',
            Mood.GRUMPY: '😒',
            Mood.VERY_GRUMPY: '😠',
            Mood.EXHAUSTED: '😴'
        }
        return mood_to_emoji.get(mood, '😐')

class ResponseGenerator:
    """Generates chat responses based on keywords and pet state."""

    @staticmethod
    def _get_greeting(pet: Pet) -> str:
        if pet.mood in [Mood.THRILLED, Mood.HAPPY]:
            return f"{pet.name} bounces excitedly! 'Hello! I'm so happy to see you!'"
        elif pet.mood in [Mood.SAD, Mood.VERY_SAD]:
            return f"{pet.name} looks up slowly. 'Oh... hi there.'"
        return f"{pet.name} perks up. 'Hello! Nice to chat with you!'"

    @staticmethod
    def _get_status_query(pet: Pet) -> str:
        return f"{pet.name} is feeling {pet.mood.value.lower()} right now. Intelligence: {pet.iq}/{GenesisPetConfig.Core.MAX_STAT}."

    @staticmethod
    def _get_compliment(pet: Pet) -> str:
        return f"{pet.name} beams with pride! Their intelligence seems to have increased to {pet.iq}!"
        
    @staticmethod
    def _get_learning(pet: Pet) -> str:
        return f"{pet.name} listens attentively, absorbing the knowledge. IQ now: {pet.iq}/{GenesisPetConfig.Core.MAX_STAT}!"
        
    @staticmethod
    def _get_social(pet: Pet) -> str:
        return f"{pet.name} becomes more sociable and charismatic! Charisma now: {pet.charisma}/{GenesisPetConfig.Core.MAX_STAT}!"

    @staticmethod
    def _get_default(pet: Pet) -> str:
        return {
            Mood.HAPPY: f"{pet.name} chirps happily, clearly enjoying the conversation!",
            Mood.SAD: f"{pet.name} sighs softly, but seems to appreciate the attention.",
            Mood.EXHAUSTED: f"{pet.name} struggles to focus, but makes an effort to engage.",
            Mood.VERY_GRUMPY: f"{pet.name} grumbles a bit, but reluctantly participates.",
            Mood.THRILLED: f"{pet.name} can barely contain its excitement as you talk!",
        }.get(pet.mood, f"{pet.name} listens to your words, tilting their head curiously.")

    @classmethod
    def generate(cls, pet: Pet, message: str) -> str:
        """Selects a response strategy based on the message content."""
        msg_lower = message.lower()
        
        # A mapping of keywords to response-generating methods
        keyword_map = {
            ("hello", "hi", "hey"): cls._get_greeting,
            ("how are you", "feeling"): cls._get_status_query,
            ("good", "smart", "clever", "beautiful"): cls._get_compliment,
            ("learn", "study", "read", "book"): cls._get_learning,
            ("friend", "social", "talk", "chat"): cls._get_social,
        }

        for keywords, func in keyword_map.items():
            if any(word in msg_lower for word in keywords):
                return func(pet)
        
        return cls._get_default(pet)

class PetLogicManager:
    """
    Handles all logic and state changes for a Pet instance.
    
    Following KISS principles:
    - K: Clear separation of responsibilities
    - I: Methods designed for easy updates
    - S: Systematized stat management
    - S: Engaging interactions for users
    """
    
    def __init__(self, pet: Pet):
        """Initialize with a pet instance."""
        self.pet = pet
        self.max_stat = GenesisPetConfig.Core.MAX_STAT
        
        # Cache interaction effects for better performance
        self.interaction_effects = GenesisPetConfig.Interactions.EFFECTS

    def _cap_stat(self, value: int) -> int:
        """Helper to cap stat values between 0 and MAX_STAT."""
        return max(0, min(value, self.max_stat))

    def _update_mood(self):
        """Delegates mood calculation to the MoodEngine."""
        self.pet.mood = MoodEngine.determine_mood(self.pet)

    def _add_interaction(self, type: InteractionType, details: Optional[str] = None):
        """Adds a new interaction record and prunes the history."""
        self.pet.interaction_history.append(
            InteractionRecord(timestamp=time.time_ns(), type=type, details=details)
        )
        # Keep history to a reasonable size
        if len(self.pet.interaction_history) > 100:
            self.pet.interaction_history.pop(0)

    def feed(self) -> str:
        """
        Feeds the pet, restoring hunger.
        
        Returns:
            A message describing the result
        
        Raises:
            InsufficientEnergyError: If the pet is too full to eat
        """
        # Get feed interaction effects from config
        feed_effects = self.interaction_effects['feed']
        min_energy = feed_effects['min_energy']
        
        # Check if pet has enough energy
        if self.pet.energy < min_energy:
            return feed_effects['messages']['too_full'].format(pet_name=self.pet.name)
        
        # Apply stat changes
        for stat, change in feed_effects['stat_changes'].items():
            stat_name = stat.name.lower()
            if hasattr(self.pet, stat_name):
                current_value = getattr(self.pet, stat_name)
                setattr(self.pet, stat_name, self._cap_stat(current_value + change))
        
        self._update_mood()
        self._add_interaction(InteractionType.FEED, "Fed the pet")
        
        return feed_effects['messages']['success'].format(pet_name=self.pet.name)

    def play(self) -> str:
        """
        Plays with the pet, boosting happiness at the cost of energy.
        
        Returns:
            A message describing the result
            
        Raises:
            InsufficientEnergyError: If the pet is too tired to play
        """
        # Get play interaction effects from config
        play_effects = self.interaction_effects['play']
        min_energy = play_effects['min_energy']
        
        # Check if pet has enough energy
        if self.pet.energy < min_energy:
            return play_effects['messages']['too_tired'].format(pet_name=self.pet.name)
        
        # Apply stat changes
        for stat, change in play_effects['stat_changes'].items():
            stat_name = stat.name.lower()
            if hasattr(self.pet, stat_name):
                current_value = getattr(self.pet, stat_name)
                setattr(self.pet, stat_name, self._cap_stat(current_value + change))
        
        self._update_mood()
        self._add_interaction(InteractionType.PLAY, "Played with the pet")
        
        return play_effects['messages']['success'].format(pet_name=self.pet.name)

    def groom(self) -> str:
        """
        Grooms the pet, improving cleanliness and happiness.
        
        Returns:
            A message describing the result
        """
        # Get groom interaction effects from config
        groom_effects = self.interaction_effects['groom']
        
        # Apply stat changes
        for stat, change in groom_effects['stat_changes'].items():
            stat_name = stat.name.lower()
            if hasattr(self.pet, stat_name):
                current_value = getattr(self.pet, stat_name)
                setattr(self.pet, stat_name, self._cap_stat(current_value + change))
        
        self._update_mood()
        self._add_interaction(InteractionType.GROOM, "Groomed the pet")
        
        return groom_effects['messages']['success'].format(pet_name=self.pet.name)

    def _calculate_iq_boost(self, message: str) -> int:
        """
        Calculates intelligence boost from a chat message.
        
        Args:
            message: The chat message
            
        Returns:
            The calculated IQ boost
        """
        # Base IQ boost from chat interaction
        chat_effects = self.interaction_effects['chat']
        base_iq_boost = chat_effects['stat_changes'].get(Stat.IQ, 3)
        
        # Calculate boost based on message complexity
        message_length_factor = min(len(message) / 50.0, 2.0)
        words = message.lower().split()
        word_count = len(words)
        unique_words = len(set(words))
        vocabulary_factor = min(unique_words / max(word_count, 1), 1.5)
        
        return int(base_iq_boost * message_length_factor * vocabulary_factor)

    def chat(self, message: str) -> str:
        """
        Handles a chat interaction, updating stats and returning a response.
        
        Args:
            message: The chat message from the user
            
        Returns:
            The pet's response
            
        Raises:
            InsufficientEnergyError: If the pet is too tired to chat
        """
        # Get chat interaction effects from config
        chat_effects = self.interaction_effects['chat']
        min_energy = chat_effects['min_energy']
        
        # Check if pet has enough energy
        if self.pet.energy < min_energy:
            raise InsufficientEnergyError(f"{self.pet.name} is too tired to chat.")
        
        # Apply base stat changes
        for stat, change in chat_effects['stat_changes'].items():
            stat_name = stat.name.lower()
            if hasattr(self.pet, stat_name):
                current_value = getattr(self.pet, stat_name)
                
                # Special case for IQ which uses a more complex calculation
                if stat == Stat.IQ:
                    iq_boost = self._calculate_iq_boost(message)
                    setattr(self.pet, stat_name, self._cap_stat(current_value + iq_boost))
                else:
                    # For other stats, apply a factor based on message length
                    factor = min(len(message) / 50.0, 1.5)
                    adjusted_change = int(change * factor)
                    setattr(self.pet, stat_name, self._cap_stat(current_value + adjusted_change))
        
        # Grant extra boosts for certain interactions identified by keywords
        if any(word in message.lower() for word in ("good", "smart", "clever")):
            self.pet.happiness = self._cap_stat(self.pet.happiness + 3)
        if any(word in message.lower() for word in ("learn", "study", "read")):
            self.pet.iq = self._cap_stat(self.pet.iq + 2)
        if any(word in message.lower() for word in ("friend", "social", "talk", "chat")):
            self.pet.charisma = self._cap_stat(self.pet.charisma + 2)

        self._update_mood()
        
        # Record the interaction
        summary = message[:30] + '...' if len(message) > 30 else message
        self._add_interaction(InteractionType.CHAT, f"Msg: '{summary}'")
        
        # Generate and return response
        return ResponseGenerator.generate(self.pet, message)

    def tick(self, current_time_ns: Optional[int] = None):
        """
        Simulates the passage of time, decaying stats and calculating offline progress.
        
        Args:
            current_time_ns: Current time in nanoseconds, or None to use current time
        """
        if current_time_ns is None:
            current_time_ns = time.time_ns()
            
        # Calculate time difference
        time_diff_ns = current_time_ns - self.pet.last_active_timestamp
        decay_interval = GenesisPetConfig.Core.DECAY_INTERVAL_SECONDS
        
        if decay_interval <= 0:
            return  # Avoid division by zero

        # Calculate how many intervals have passed
        intervals_passed = time_diff_ns // (decay_interval * 1_000_000_000)

        if intervals_passed > 0:
            # Apply decay rates to all stats
            for stat, rate in GenesisPetConfig.Core.DECAY_RATES.items():
                stat_name = stat.name.lower()
                if hasattr(self.pet, stat_name):
                    current_value = getattr(self.pet, stat_name)
                    # Positive rate means the stat increases (like hunger)
                    # Negative rate means the stat decreases (like energy)
                    new_value = self._cap_stat(current_value + int(rate * intervals_passed))
                    setattr(self.pet, stat_name, new_value)
            
            self._update_mood()
            self.pet.last_active_timestamp = current_time_ns
            self._add_interaction(InteractionType.TICK_DECAY, f"Applied decay for {intervals_passed} intervals")

    def get_status_report(self) -> str:
        """
        Returns a formatted string summary of the pet's current status.
        
        Returns:
            A formatted status report string
        """
        # Get species display name
        species_info = GenesisPetConfig.Archetypes.DEFINITIONS.get(self.pet.species, {})
        species_display = species_info.get('display_name', self.pet.species)
        
        # Get aura display name
        aura_info = GenesisPetConfig.Auras.DEFINITIONS.get(self.pet.aura_color, {})
        aura_display = aura_info.get('display_name', self.pet.aura_color)
        
        # Format the status report
        return (
            f"--- {self.pet.name}'s Status ---\n"
            f"ID: {self.pet.id}\n"
            f"Species: {species_display}\n"
            f"Aura: {aura_display}\n"
            f"Mood: {self.pet.mood.value} {MoodEngine.get_mood_emoji(self.pet.mood)}\n"
            f"Sustenance: {self.pet.hunger}/{self.max_stat}\n"
            f"Energy: {self.pet.energy}/{self.max_stat}\n"
            f"Happiness: {self.pet.happiness}/{self.max_stat}\n"
            f"Intelligence: {self.pet.iq}/{self.max_stat}\n"
            f"Charisma: {self.pet.charisma}/{self.max_stat}\n"
            f"Cleanliness: {self.pet.cleanliness}/{self.max_stat}\n"
            f"Social: {self.pet.social}/{self.max_stat}\n"
            f"Personality: {', '.join(f'{k}: {v}' for k, v in self.pet.personality_traits.items())}\n"
        )

# --- Persistence Functions ---
class PetPersistence:
    """
    Handles saving and loading pets to/from storage.
    
    Following KISS principles:
    - K: Clear, focused responsibility (persistence only)
    - I: Easy to extend with new storage methods
    - S: Systematized error handling
    - S: Designed for data integrity
    """
    
    @staticmethod
    def save_to_file(pet: Pet, filename: str) -> bool:
        """
        Saves a pet's state to a JSON file.
        
        Args:
            pet: The Pet instance to save
            filename: Path to the file
            
        Returns:
            True if successful, False otherwise
        """
        try:
            with open(filename, 'w') as f:
                json.dump(pet.to_dict(), f, indent=4)
            return True
        except (IOError, TypeError) as e:
            print(f"Error saving pet to {filename}: {e}")
            return False

    @staticmethod
    def load_from_file(filename: str) -> Optional[Pet]:
        """
        Loads a pet's state from a JSON file.
        
        Args:
            filename: Path to the file
            
        Returns:
            A Pet instance if successful, None otherwise
        """
        try:
            with open(filename, 'r') as f:
                data = json.load(f)
            return Pet.from_dict(data)
        except (IOError, json.JSONDecodeError, KeyError) as e:
            print(f"Error loading pet from {filename}: {e}")
            return None
            
    @staticmethod
    def backup_pet(pet: Pet, backup_dir: str) -> bool:
        """
        Creates a timestamped backup of a pet.
        
        Args:
            pet: The Pet instance to backup
            backup_dir: Directory to store backups
            
        Returns:
            True if successful, False otherwise
        """
        import os
        from datetime import datetime
        
        try:
            # Create backup directory if it doesn't exist
            os.makedirs(backup_dir, exist_ok=True)
            
            # Create timestamped filename
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            filename = f"{backup_dir}/{pet.name}_{timestamp}.json"
            
            # Save the backup
            return PetPersistence.save_to_file(pet, filename)
        except Exception as e:
            print(f"Error creating backup: {e}")
            return False

# Legacy functions for backward compatibility
def save_pet_to_file(pet: Pet, filename: str):
    """Legacy function for backward compatibility."""
    return PetPersistence.save_to_file(pet, filename)

def load_pet_from_file(filename: str) -> Pet:
    """Legacy function for backward compatibility."""
    pet = PetPersistence.load_from_file(filename)
    if pet is None:
        raise FileNotFoundError(f"Could not load pet from {filename}")
    return pet