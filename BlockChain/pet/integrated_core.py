# pet/integrated_core.py
"""
Integrated core functionality for CritterCraft, combining the pet care system
with the educational critter creation system.

This module defines the IntegratedPet class which merges the functionality of
Pet and Critter classes, providing a unified experience.
"""

import json
import time
import uuid
import random
import random
import random
from typing import Dict, Any, List, Optional, Set, Callable, Tuple
from dataclasses import dataclass, field, asdict
from enum import Enum

# Import AI integration
from ai_integration import AIIntegrationManager

# Import constants from config.py
from config import (
    # Pet-related constants
    MAX_STAT, STAT_DECAY_INTERVAL_NS, DECAY_RATES,
    DEFAULT_INITIAL_PET_STATS, INTERACTION_EFFECTS,
    MOOD_THRESHOLDS, STATUS_ALERTS,
    PET_ARCHETYPES, PET_AURA_COLORS, AI_PERSONALITY_TRAITS,
    MIGRATION_READINESS_THRESHOLDS,
    
    # Critter-related constants
    CRITTER_TYPES, CRAFTING_MATERIALS, ADAPTATIONS, ZOOLOGIST_LEVELS
)

# Import advanced feature constants
from pet.advanced_constants import (
    JOB_TYPES,
    BATTLE_OPPONENTS,
    AVAILABLE_QUESTS,
    EDUCATION_SUBJECTS,
    EDUCATION_DEGREES,
    EDUCATION_CERTIFICATIONS,
    EVOLUTION_PATHS,
    ACHIEVEMENTS,
    DNA_TRAITS,
    DNA_MUTATIONS
)

# Import advanced feature constants
from pet.advanced_constants import (
    JOB_TYPES,
    BATTLE_OPPONENTS,
    AVAILABLE_QUESTS,
    EDUCATION_SUBJECTS,
    EDUCATION_DEGREES,
    EDUCATION_CERTIFICATIONS,
    EVOLUTION_PATHS,
    ACHIEVEMENTS,
    DNA_TRAITS,
    DNA_MUTATIONS
)

# Import advanced feature constants
from pet.advanced_constants import (
    JOB_TYPES,
    BATTLE_OPPONENTS,
    AVAILABLE_QUESTS,
    EDUCATION_SUBJECTS,
    EDUCATION_DEGREES,
    EDUCATION_CERTIFICATIONS,
    EVOLUTION_PATHS,
    ACHIEVEMENTS,
    DNA_TRAITS,
    DNA_MUTATIONS
)

# --- Custom Exceptions ---
class PetError(Exception):
    """Base exception for pet-related errors."""
    pass

class PetInitializationError(PetError):
    """Raised when a pet cannot be initialized due to invalid parameters."""
    pass

class InsufficientEnergyError(PetError):
    """Raised when an action cannot be performed due to low energy."""
    pass

class CraftingError(Exception):
    """Base exception for crafting-related errors."""
    pass
    TRAIN = "train"
    BATTLE = "battle"
    QUEST = "quest"
    CAREER = "career"
    MILESTONE = "milestone"

# --- Enums for Type Safety and Readability ---
class InteractionType(Enum):
    FEED = "feed"
    PLAY = "play"
    CHAT = "chat"
    GROOM = "groom"
    TICK_DECAY = "tick_decay"
    CRAFT = "craft"
    LEARN = "learn"
    TRAIN = "train"
    BATTLE = "battle"
    QUEST = "quest"
    CAREER = "career"
    MILESTONE = "milestone"

# --- Data Models for Crafting ---
@dataclass
class CraftingMaterial:
    """Represents a material used in crafting a critter."""
    type: str          # e.g., "fur", "scales", "feathers"
    color: str         # e.g., "brown", "green", "blue"
    coverage: float    # 0.0 to 1.0, representing percentage of critter covered
    position: str      # e.g., "body", "head", "limbs", "tail"

@dataclass
class Adaptation:
    """Represents an adaptation applied to a critter."""
    type: str          # e.g., "camouflage", "bioluminescence"
    strength: int      # 1-10, representing effectiveness
    position: str      # e.g., "body", "head", "limbs", "tail"

# --- Core Data Models ---
@dataclass
class InteractionRecord:
    """Represents a single interaction event with the pet."""
    timestamp: int
    type: InteractionType
    details: Optional[str] = None

@dataclass
class IntegratedPet:
    """
    Represents a CritterCraft pet that combines pet care and critter creation.
    This is the core data model for the integrated system.
    """
    # Basic pet information
    name: str
    species: str  # Maps to PET_ARCHETYPES
    aura_color: str
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    
    # Core pet stats
    hunger: int = field(default_factory=lambda: DEFAULT_INITIAL_PET_STATS['hunger'])
    energy: int = field(default_factory=lambda: DEFAULT_INITIAL_PET_STATS['energy'])
    happiness: int = field(default_factory=lambda: DEFAULT_INITIAL_PET_STATS['happiness'])
    iq: int = field(default_factory=lambda: DEFAULT_INITIAL_PET_STATS['iq'])
    charisma: int = field(default_factory=lambda: DEFAULT_INITIAL_PET_STATS['charisma'])
    cleanliness: int = field(default_factory=lambda: DEFAULT_INITIAL_PET_STATS['cleanliness'])
    social: int = field(default_factory=lambda: DEFAULT_INITIAL_PET_STATS['social'])
    
    # Critter-specific attributes
    base_animal: Optional[str] = None  # Maps to CRITTER_TYPES
    materials: List[CraftingMaterial] = field(default_factory=list)
    adaptations: List[Adaptation] = field(default_factory=list)
    facts_learned: Set[str] = field(default_factory=set)
    
    # Personality
    personality_traits: Dict[str, int] = field(default_factory=lambda: {
        k: v["default"] for k, v in AI_PERSONALITY_TRAITS.items()
    })
    
    # Timestamps
    creation_timestamp: int = field(default_factory=time.time_ns)
    last_active_timestamp: int = field(default_factory=time.time_ns)
    
    # History
    interaction_history: List[InteractionRecord] = field(default_factory=list)
    
    # Zoologist progression
    zoologist_level: str = 'novice'
    critters_created: int = 0
    unlocked_materials: Set[str] = field(default_factory=lambda: set(ZOOLOGIST_LEVELS['novice']['unlocked_materials']))
    unlocked_adaptations: Set[str] = field(default_factory=lambda: set(ZOOLOGIST_LEVELS['novice']['unlocked_adaptations']))
    
    # Age tracking
    growth_rate: float = 1.0  # Base growth rate multiplier
    maturity_level: int = 0   # 0-100 scale of maturity
    # Age tracking
    growth_rate: float = 1.0  # Base growth rate multiplier
    maturity_level: int = 0   # 0-100 scale of maturity
    
    # State tracking systems
    job_states: Dict[str, Any] = field(default_factory=lambda: {
        'current_job': None,
        'job_level': 0,
        'job_experience': 0,
        'job_history': [],
        'skills': {}
    })
    
    battle_states: Dict[str, Any] = field(default_factory=lambda: {
        'strength': 10,
        'defense': 10,
        'speed': 10,
        'special_attack': 10,
        'special_defense': 10,
        'battles_won': 0,
        'battles_lost': 0,
        'abilities': [],
        'battle_items': []
    })
    
    quest_states: Dict[str, Any] = field(default_factory=lambda: {
        'active_quests': [],
        'completed_quests': [],
        'quest_points': 0,
        'reputation': {}
    })
    
    education_states: Dict[str, Any] = field(default_factory=lambda: {
        'education_level': 0,
        'subjects_studied': {},
        'degrees': [],
        'certifications': []
    })
    
    # Achievement and evolution tracking
    achievements: Dict[str, Any] = field(default_factory=lambda: {
        'mastered': [],
        'in_progress': {},
        'achievement_points': 0
    })
    
    evolution: Dict[str, Any] = field(default_factory=lambda: {
        'evolution_stage': 0,
        'evolution_path': [],
        'potential_evolutions': [],
        'evolution_requirements': {}
    })
    
    # Genealogy and DNA
    dna: Dict[str, Any] = field(default_factory=lambda: {
        'genetic_traits': {},
        'dominant_genes': [],
        'recessive_genes': [],
        'mutations': []
    })
    
    genealogy: Dict[str, Any] = field(default_factory=lambda: {
        'parents': [],
        'siblings': [],
        'offspring': [],
        'generation': 1
    })
    
    def __post_init__(self):
        """Perform post-initialization validation."""
        self.name = self.name.strip()
        if not self.name or len(self.name) > 20 or not self.name.isprintable():
            raise PetInitializationError("Pet name must be 1-20 printable characters.")
        if self.species not in PET_ARCHETYPES:
            raise PetInitializationError(f"Invalid species: {self.species}.")
        if self.aura_color not in PET_AURA_COLORS:
            raise PetInitializationError(f"Invalid aura color: {self.aura_color}.")
        
        # Apply species-specific stat modifiers
        species_info = PET_ARCHETYPES.get(self.species, {})
        base_modifiers = species_info.get('base_stats_modifier', {})
        
        for stat, modifier in base_modifiers.items():
            if hasattr(self, stat):
                current_value = getattr(self, stat)
                setattr(self, stat, max(0, min(MAX_STAT, current_value + modifier)))
    
    def calculate_age_days(self) -> float:
        """Calculate the pet's age in days based on creation timestamp."""
        current_time = time.time_ns()
        time_diff_ns = current_time - self.creation_timestamp
        # Convert nanoseconds to days
        days = time_diff_ns / (24 * 60 * 60 * 1_000_000_000)
        return days
    
    def calculate_biological_age(self) -> float:
        """
        Calculate the pet's biological age based on IQ, growth rate, and maturity.
        This represents the pet's actual developmental age.
        """
        base_age = self.calculate_age_days()
        
        # IQ factor: Higher IQ accelerates maturity
        iq_factor = 1.0 + (self.iq / 200)  # 0.5 to 1.5 range
        
        # Growth rate factor: Species-specific growth rate
        growth_factor = self.growth_rate
        
        # Calculate biological age
        biological_age = base_age * iq_factor * growth_factor
        
        # Update maturity level based on biological age
        # Maturity caps at 100 when the pet is approximately 2 years old (730 days)
        self.maturity_level = min(100, int((biological_age / 730) * 100))
        
        return biological_age
    
    def calculate_human_age_equivalent(self) -> int:
        """
        Calculate the pet's age in human-equivalent years.
        Different species have different aging rates.
        """
        biological_age = self.calculate_biological_age()
        
        # Get species-specific aging rate
        species_info = PET_ARCHETYPES.get(self.species, {})
        aging_rate = species_info.get('aging_rate', 7)  # Default: 7x human aging
        
        # First year counts as more in pet years
        if biological_age <= 365:
            human_equivalent = (biological_age / 365) * 15  # First year = 15 human years
        else:
            # After first year, aging slows down
            first_year = 15
            remaining_days = biological_age - 365
            remaining_years = (remaining_days / 365) * aging_rate
            human_equivalent = first_year + remaining_years
        
        return int(human_equivalent
    # State tracking systems
    job_states: Dict[str, Any] = field(default_factory=lambda: {
        'current_job': None,
        'job_level': 0,
        'job_experience': 0,
        'job_history': [],
        'skills': {}
    })
    
    battle_states: Dict[str, Any] = field(default_factory=lambda: {
        'strength': 10,
        'defense': 10,
        'speed': 10,
        'special_attack': 10,
        'special_defense': 10,
        'battles_won': 0,
        'battles_lost': 0,
        'abilities': [],
        'battle_items': []
    }# Age tracking
    growth_rate: float = 1.0  # Base growth rate multiplier
    maturity_level: int = 0   # 0-100 scale of maturity
    
    # State tracking systems
    job_states: Dict[str, Any] = field(default_factory=lambda: {
        'current_job': None,
        'job_level': 0,
        'job_experience': 0,
        'job_history': [],
        'skills': {}
    })
    
    battle_states: Dict[str, Any] = field(default_factory=lambda: {
        'strength': 10,
        'defense': 10,
        'speed': 10,
        'special_attack': 10,
        'special_defense': 10,
        'battles_won': 0,
        'battles_lost': 0,
        'abilities': [],
        'battle_items': []
    })
    
    quest_states: Dict[str, Any] = field(default_factory=lambda: {
        'active_quests': [],
        'completed_quests': [],
        'quest_points': 0,
        'reputation': {}
    })
    
    education_states: Dict[str, Any] = field(default_factory=lambda: {
        'education_level': 0,
        'subjects_studied': {},
        'degrees': [],
        'certifications': []
    })
    
    # Achievement and evolution tracking
    achievements: Dict[str, Any] = field(default_factory=lambda: {
        'mastered': [],
        'in_progress': {},
        'achievement_points': 0
    })
    
    evolution: Dict[str, Any] = field(default_factory=lambda: {
        'evolution_stage': 0,
        'evolution_path': [],
        'potential_evolutions': [],
        'evolution_requirements': {}
    })
    
    # Genealogy and DNA
    dna: Dict[str, Any] = field(default_factory=lambda: {
        'genetic_traits': {},
        'dominant_genes': [],
        'recessive_genes': [],
        'mutations': []
    })
    
    genealogy: Dict[str, Any] = field(default_factory=lambda: {
        'parents': [],
        'siblings': [],
        'offspring': [],
        'generation': 1
    })
    
    def __post_init__(self):
        """Perform post-initialization validation."""
        self.name = self.name.strip()
        if not self.name or len(self.name) > 20 or not self.name.isprintable():
            raise PetInitializationError("Pet name must be 1-20 printable characters.")
        if self.species not in PET_ARCHETYPES:
            raise PetInitializationError(f"Invalid species: {self.species}.")
        if self.aura_color not in PET_AURA_COLORS:
            raise PetInitializationError(f"Invalid aura color: {self.aura_color}.")
        
        # Apply species-specific stat modifiers
        species_info = PET_ARCHETYPES.get(self.species, {})
        base_modifiers = species_info.get('base_stats_modifier', {})
        
        for stat, modifier in base_modifiers.items():
            if hasattr(self, stat):
                current_value = getattr(self, stat)
                setattr(self, stat, max(0, min(MAX_STAT, current_value + modifier)))
    
    def calculate_age_days(self) -> float:
        """Calculate the pet's age in days based on creation timestamp."""
        current_time = time.time_ns()
        time_diff_ns = current_time - self.creation_timestamp
        # Convert nanoseconds to days
        days = time_diff_ns / (24 * 60 * 60 * 1_000_000_000)
        return days
    
    def calculate_biological_age(self) -> float:
        """
        Calculate the pet's biological age based on IQ, growth rate, and maturity.
        This represents the pet's actual developmental age.
        """
        base_age = self.calculate_age_days()
        
        # IQ factor: Higher IQ accelerates maturity
        iq_factor = 1.0 + (self.iq / 200)  # 0.5 to 1.5 range
        
        # Growth rate factor: Species-specific growth rate
        growth_factor = self.growth_rate
        
        # Calculate biological age
        biological_age = base_age * iq_factor * growth_factor
        
        # Update maturity level based on biological age
        # Maturity caps at 100 when the pet is approximately 2 years old (730 days)
        self.maturity_level = min(100, int((biological_age / 730) * 100))
        
        return biological_age
    
    def calculate_human_age_equivalent(self) -> int:
        """
        Calculate the pet's age in human-equivalent years.
        Different species have different aging rates.
        """
        biological_age = self.calculate_biological_age()
        
        # Get species-specific aging rate
        species_info = PET_ARCHETYPES.get(self.species, {})
        aging_rate = species_info.get('aging_rate', 7)  # Default: 7x human aging
        
        # First year counts as more in pet years
        if biological_age <= 365:
            human_equivalent = (biological_age / 365) * 15  # First year = 15 human years
        else:
            # After first year, aging slows down
            first_year = 15
            remaining_days = biological_age - 365
            remaining_years = (remaining_days / 365) * aging_rate
            human_equivalent = first_year + remaining_years
        
        return int(human_equivalent)
    
    quest_states: Dict[str, Any] = field(default_factory=lambda: {
        'active_quests': [],
        'completed_quests': [],
        'quest_points': 0,
        'reputation': {}
    })
    
    education_states: Dict[str, Any] = field(default_factory=lambda: {
        'education_level': 0,
        'subjects_studied': {},
        'degrees': [],
        'certifications': []
    })
    
    # Achievement and evolution tracking
    achievements: Dict[str, Any] = field(default_factory=lambda: {
        'mastered': [],
        'in_progress': {},
        'achievement_points': 0
    })
    
    evolution: Dict[str, Any] = field(default_factory=lambda: {
        'evolution_stage': 0,
        'evolution_path': [],
        'potential_evolutions': [],
        'evolution_requirements': {}
    })
    
    # Genealogy and DNA
    dna: Dict[str, Any] = field(default_factory=lambda: {
        'genetic_traits': {},
        'dominant_genes': [],
        'recessive_genes': [],
        'mutations': []
    })
    
    genealogy: Dict[str, Any] = field(default_factory=lambda: {
        'parents': [],
        'siblings': [],
        'offspring': [],
        'generation': 1
    })
    
    def __post_init__(self):
        """Perform post-initialization validation."""
        self.name = self.name.strip()
        if not self.name or len(self.name) > 20 or not self.name.isprintable():
            raise PetInitializationError("Pet name must be 1-20 printable characters.")
        if self.species not in PET_ARCHETYPES:
            raise PetInitializationError(f"Invalid species: {self.species}.")
        if self.aura_color not in PET_AURA_COLORS:
            raise PetInitializationError(f"Invalid aura color: {self.aura_color}.")
        
        # Apply species-specific stat modifiers
        species_info = PET_ARCHETYPES.get(self.species, {})
        base_modifiers = species_info.get('base_stats_modifier', {})
        
        for stat, modifier in base_modifiers.items():
            if hasattr(self, stat):
                current_value = getattr(self, stat)
                setattr(self, stat, max(0, min(MAX_STAT, current_value + modifier)))
    
    def calculate_age_days(self) -> float:
        """Calculate the pet's age in days based on creation timestamp."""
        current_time = time.time_ns()
        time_diff_ns = current_time - self.creation_timestamp
        # Convert nanoseconds to days
        days = time_diff_ns / (24 * 60 * 60 * 1_000_000_000)
        return days
    
    def calculate_biological_age(self) -> float:
        """
        Calculate the pet's biological age based on IQ, growth rate, and maturity.
        This represents the pet's actual developmental age.
        """
        base_age = self.calculate_age_days()
        
        # IQ factor: Higher IQ accelerates maturity
        iq_factor = 1.0 + (self.iq / 200)  # 0.5 to 1.5 range
        
        # Growth rate factor: Species-specific growth rate
        growth_factor = self.growth_rate
        
        # Calculate biological age
        biological_age = base_age * iq_factor * growth_factor
        
        # Update maturity level based on biological age
        # Maturity caps at 100 when the pet is approximately 2 years old (730 days)
        self.maturity_level = min(100, int((biological_age / 730) * 100))
        
        return biological_age
    
    def calculate_human_age_equivalent(self) -> int:
        """
        Calculate the pet's age in human-equivalent years.
        Different species have different aging rates.
        """
        biological_age = self.calculate_biological_age()
        
        # Get species-specific aging rate
        species_info = PET_ARCHETYPES.get(self.species, {})
        aging_rate = species_info.get('aging_rate', 7)  # Default: 7x human aging
        
        # First year counts as more in pet years
        if biological_age <= 365:
            human_equivalent = (biological_age / 365) * 15  # First year = 15 human years
        else:
            # After first year, aging slows down
            first_year = 15
            remaining_days = biological_age - 365
            remaining_years = (remaining_days / 365) * aging_rate
            human_equivalent = first_year + remaining_years
        
        return int(human_equivalent)
    
    def to_dict(self) -> Dict[str, Any]:
        """Serialize the IntegratedPet object to a dictionary."""
        data = asdict(self)
        
        # Convert Enum values to strings
        data['interaction_history'] = [
            {**asdict(rec), 'type': rec.type.value} for rec in self.interaction_history
        ]
        
        # Convert sets to lists for JSON serialization
        data['facts_learned'] = list(self.facts_learned)
        data['unlocked_materials'] = list(self.unlocked_materials)
        data['unlocked_adaptations'] = list(self.unlocked_adaptations)
        
        return data
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'IntegratedPet':
        """Deserialize an IntegratedPet object from a dictionary."""
        # Convert string interaction types back to Enum
        history_data = data.get('interaction_history', [])
        data['interaction_history'] = [
            InteractionRecord(
                timestamp=rec['timestamp'],
                type=InteractionType(rec['type']),
                details=rec.get('details')
            ) for rec in history_data
        ]
        
        # Convert lists back to sets
        data['facts_learned'] = set(data.get('facts_learned', []))
        data['unlocked_materials'] = set(data.get('unlocked_materials', []))
        data['unlocked_adaptations'] = set(data.get('unlocked_adaptations', []))
        
        # Convert material and adaptation dictionaries to objects
        materials_data = data.get('materials', [])
        data['materials'] = [CraftingMaterial(**m) for m in materials_data]
        
        adaptations_data = data.get('adaptations', [])
        data['adaptations'] = [Adaptation(**a) for a in adaptations_data]
        
        return cls(**data)
    
    def to_json(self) -> str:
        """Convert the pet to a JSON string."""
        return json.dumps(self.to_dict(), indent=2)
    
    @classmethod
    def from_json(cls, json_str: str) -> 'IntegratedPet':
        """Create a pet from a JSON string."""
        data = json.loads(json_str)
        return cls.from_dict(data)


# --- Logic Manager for IntegratedPet ---
class IntegratedPetManager:
    """Handles all logic and state changes for an IntegratedPet instance."""
    
    def __init__(self, pet: IntegratedPet):
        self.pet = pet
        
        # Initialize AI integration
        pet_data = {
            'name': pet.name,
            'species': pet.species,
            'aura_color': pet.aura_color,
            'personality_traits': pet.personality_traits,
            'base_animal': pet.base_animal
        }
        self.ai_manager = AIIntegrationManager(pet_data)
    
    def _cap_stat(self, value: int) -> int:
        """Helper to cap stat values between 0 and MAX_STAT."""
        return max(0, min(value, MAX_STAT))
    
    def _add_interaction(self, type: InteractionType, details: Optional[str] = None):
        """Adds a new interaction record and prunes the history."""
        self.pet.interaction_history.append(
            InteractionRecord(timestamp=time.time_ns(), type=type, details=details)
        )
        if len(self.pet.interaction_history) > 100:
            self.pet.interaction_history.pop(0)
    
    def _get_current_mood(self) -> Dict[str, Any]:
        """Determines the pet's current mood based on happiness."""
        for mood in MOOD_THRESHOLDS:
            if self.pet.happiness >= mood['threshold']:
                return mood
        return MOOD_THRESHOLDS[-1]  # Default to the lowest mood if nothing matches
    
    def _get_status_alerts(self) -> List[Dict[str, Any]]:
        """Gets a list of active status alerts based on pet stats."""
        active_alerts = []
        for alert_key, alert in STATUS_ALERTS.items():
            stat_value = getattr(self.pet, alert['stat'])
            if alert['condition'](stat_value):
                active_alerts.append(alert)
        return active_alerts
    
    def tick(self, current_time_ns: Optional[int] = None):
        """Simulates the passage of time, decaying stats and calculating offline progress."""
        if current_time_ns is None:
            current_time_ns = time.time_ns()
            
        time_diff_ns = current_time_ns - self.pet.last_active_timestamp
        intervals_passed = time_diff_ns // STAT_DECAY_INTERVAL_NS
        
        if intervals_passed > 0:
            # Apply decay to all stats based on DECAY_RATES
            for stat, decay_rate in DECAY_RATES.items():
                if hasattr(self.pet, stat):
                    current_value = getattr(self.pet, stat)
                    
                    # For hunger, the value increases (pet gets hungrier)
                    # For other stats, the value decreases
                    if stat == 'hunger':
                        new_value = self._cap_stat(current_value + int(decay_rate * intervals_passed))
                    else:
                        new_value = self._cap_stat(current_value - int(decay_rate * intervals_passed))
                    
                    setattr(self.pet, stat, new_value)
            
            # Apply species-specific decay rate modifiers
            species_info = PET_ARCHETYPES.get(self.pet.species, {})
            decay_modifiers = species_info.get('decay_rate_modifier', {})
            
            for stat, modifier in decay_modifiers.items():
                if hasattr(self.pet, stat):
                    current_value = getattr(self.pet, stat)
                    decay_rate = DECAY_RATES.get(stat, 0)
                    
                    # Calculate the modified decay amount
                    modified_decay = decay_rate * (1 + modifier) * intervals_passed
                    
                    # Apply the modified decay
                    if stat == 'hunger':
                        new_value = self._cap_stat(current_value + int(modified_decay))
                    else:
                        new_value = self._cap_stat(current_value - int(modified_decay))
                    
                    setattr(self.pet, stat, new_value)
            
            # Apply aura effects on decay rates
            aura_info = PET_AURA_COLORS.get(self.pet.aura_color, {})
            decay_reduction = aura_info.get('decay_reduction', {})
            
            for stat, reduction in decay_reduction.items():
                if hasattr(self.pet, stat):
                    current_value = getattr(self.pet, stat)
                    decay_rate = DECAY_RATES.get(stat, 0)
                    
                    # Calculate the reduced decay amount
                    reduced_decay = decay_rate * (1 - reduction) * intervals_passed
                    
                    # Apply the reduced decay
                    if stat == 'hunger':
                        new_value = self._cap_stat(current_value + int(reduced_decay))
                    else:
                        new_value = self._cap_stat(current_value - int(reduced_decay))
                    
                    setattr(self.pet, stat, new_value)
            
            self.pet.last_active_timestamp = current_time_ns
            self._add_interaction(InteractionType.TICK_DECAY, f"Applied decay for {intervals_passed} intervals")
            
            # Update the AI system
            self.ai_manager.update()
            
            # Process the tick in the AI system
            self.ai_manager.process_interaction('tick_decay', True, {'intervals_passed': intervals_passed})
    
    def feed(self) -> str:
        """Feeds the pet, affecting hunger, happiness, and cleanliness."""
        effects = INTERACTION_EFFECTS['feed']
        
        # Check if pet is too full
        if self.pet.hunger <= 10:
            return effects['messages']['too_full'].format(pet_name=self.pet.name)
        
        # Check if pet has minimum energy required
        if self.pet.energy < effects.get('min_energy_cost', 0):
            return effects['messages']['too_tired'].format(pet_name=self.pet.name)
        
        # Apply stat changes
        for stat, change in effects.items():
            if stat in ['messages', 'min_energy_cost']:
                continue
            
            if hasattr(self.pet, stat):
                current_value = getattr(self.pet, stat)
                setattr(self.pet, stat, self._cap_stat(current_value + change))
        
        self._add_interaction(InteractionType.FEED, "Fed the pet")
        return effects['messages']['success'].format(pet_name=self.pet.name)
    
    def play(self) -> Tuple[bool, str]:
        """Plays with the pet, affecting energy, happiness, hunger, and social."""
        effects = INTERACTION_EFFECTS['play']
        
        # Check if pet is too tired
        if self.pet.energy < effects.get('min_energy_cost', 0):
            return False, effects['messages']['too_tired'].format(pet_name=self.pet.name)
        
        # Check if pet is too hungry
        if self.pet.hunger >= 80:
            return False, effects['messages']['too_hungry'].format(pet_name=self.pet.name)
        
        # Apply stat changes
        for stat, change in effects.items():
            if stat in ['messages', 'min_energy_cost']:
                continue
            
            if hasattr(self.pet, stat):
                current_value = getattr(self.pet, stat)
                setattr(self.pet, stat, self._cap_stat(current_value + change))
        
        self._add_interaction(InteractionType.PLAY, "Played with the pet")
        return True, effects['messages']['success'].format(pet_name=self.pet.name)
    
    def chat(self, message: str) -> Tuple[bool, str]:
        """Chats with the pet, affecting iq, happiness, energy, and social."""
        effects = INTERACTION_EFFECTS['chat']
        
        # Check if pet is too tired
        if self.pet.energy < effects.get('min_energy_cost', 0):
            return False, effects['messages']['too_tired'].format(pet_name=self.pet.name)
        
        # Apply stat changes
        for stat, change in effects.items():
            if stat in ['messages', 'min_energy_cost']:
                continue
            
            if hasattr(self.pet, stat):
                current_value = getattr(self.pet, stat)
                setattr(self.pet, stat, self._cap_stat(current_value + change))
        
        # Apply species-specific interaction boosts
        species_info = PET_ARCHETYPES.get(self.pet.species, {})
        interaction_boosts = species_info.get('interaction_boosts', {}).get('chat', {})
        
        for stat, boost in interaction_boosts.items():
            if hasattr(self.pet, stat):
                current_value = getattr(self.pet, stat)
                setattr(self.pet, stat, self._cap_stat(current_value + boost))
        
        # Update pet data for AI manager
        pet_data = {
            'name': self.pet.name,
            'species': self.pet.species,
            'aura_color': self.pet.aura_color,
            'mood': self._get_current_mood(),
            'base_animal': self.pet.base_animal
        }
        self.ai_manager.pet_data = pet_data
        
        # Generate response using AI integration
        response = self.ai_manager.generate_chat_response(message)
        
        # Process the interaction in the AI system
        self.ai_manager.process_interaction('chat', True, {'message': message})
        
        self._add_interaction(InteractionType.CHAT, f"Chatted: '{message[:30]}...' if len(message) > 30 else message")
        return True, response
    
    def groom(self) -> str:
        """Grooms the pet, affecting cleanliness, happiness, and energy."""
        effects = INTERACTION_EFFECTS['groom']
        
        # Apply stat changes
        for stat, change in effects.items():
            if stat in ['messages']:
                continue
            
            if hasattr(self.pet, stat):
                current_value = getattr(self.pet, stat)
                setattr(self.pet, stat, self._cap_stat(current_value + change))
        
        # Apply species-specific interaction boosts
        species_info = PET_ARCHETYPES.get(self.pet.species, {})
        interaction_boosts = species_info.get('interaction_boosts', {}).get('groom', {})
        
        for stat, boost in interaction_boosts.items():
            if hasattr(self.pet, stat):
                current_value = getattr(self.pet, stat)
                setattr(self.pet, stat, self._cap_stat(current_value + boost))
        
        self._add_interaction(InteractionType.GROOM, "Groomed the pet")
        return effects['messages']['success'].format(pet_name=self.pet.name)
    
    def _generate_chat_response(self, message: str) -> str:
        """Generates a chat response based on the message and pet personality."""
        msg_lower = message.lower()
        
        # Simple keyword-based responses
        if any(word in msg_lower for word in ["hello", "hi", "hey"]):
            mood = self._get_current_mood()
            if mood['name'] in ['Ecstatic', 'Happy']:
                return f"{self.pet.name} bounces excitedly! 'Hello! I'm so happy to see you!'"
            elif mood['name'] in ['Sad', 'Miserable']:
                return f"{self.pet.name} looks up slowly. 'Oh... hi there.'"
            return f"{self.pet.name} perks up. 'Hello! Nice to chat with you!'"
        
        elif any(word in msg_lower for word in ["how are you", "feeling"]):
            mood = self._get_current_mood()
            return f"{self.pet.name} is {mood['description']} {mood['emoji']}"
        
        elif any(word in msg_lower for word in ["good", "smart", "clever", "beautiful"]):
            self.pet.happiness = self._cap_stat(self.pet.happiness + 3)
            return f"{self.pet.name} beams with pride! Their happiness increases to {self.pet.happiness}!"
        
        elif any(word in msg_lower for word in ["learn", "study", "read", "book"]):
            self.pet.iq = self._cap_stat(self.pet.iq + 2)
            return f"{self.pet.name} listens attentively, absorbing the knowledge. IQ now: {self.pet.iq}/{MAX_STAT}!"
        
        # Default response based on mood
        mood = self._get_current_mood()
        if mood['name'] == 'Ecstatic':
            return f"{self.pet.name} can barely contain its excitement as you talk!"
        elif mood['name'] == 'Happy':
            return f"{self.pet.name} chirps happily, clearly enjoying the conversation!"
        elif mood['name'] in ['Sad', 'Miserable']:
            return f"{self.pet.name} sighs softly, but seems to appreciate the attention."
        elif mood['name'] == 'Grumpy':
            return f"{self.pet.name} grumbles a bit, but reluctantly participates."
        else:
            return f"{self.pet.name} listens to your words, tilting their head curiously."
    
    def add_material(self, material_type: str, color: str, coverage: float, position: str) -> bool:
        """Add a crafting material to the pet's critter form."""
        if not self.pet.base_animal:
            raise CraftingError("Pet must have a base animal before adding materials.")
        
        if material_type not in CRAFTING_MATERIALS:
            # Add job information if applicable
        if self.pet.job_states['current_job']:
            job_name = self.pet.job_states['current_job']
            job_info = JOB_TYPES.get(job_name, {})
            status += f"\nJob: {job_info.get('display_name', job_name)}\n"
            status += f"Job Level: {self.pet.job_states['job_level']}\n"
            status += f"Job Experience: {self.pet.job_states['job_experience']}/{100 * self.pet.job_states['job_level']}\n"
            
            # Add skills
            if self.pet.job_states['skills']:
                status += "Skills:\n"
                for skill, level in self.pet.job_states['skills'].items():
                    status += f"  {skill.capitalize()}: {level}\n"
        
        # Add battle stats
        battle_stats = self.pet.battle_states
        status += f"\nBattle Stats:\n"
        status += f"  Strength: {battle_stats['strength']}\n"
        status += f"  Defense: {battle_stats['defense']}\n"
        status += f"  Speed: {battle_stats['speed']}\n"
        status += f"  Special Attack: {battle_stats['special_attack']}\n"
        status += f"  Special Defense: {battle_stats['special_defense']}\n"
        status += f"  Battles Won: {battle_stats['battles_won']}\n"
        
        # Add abilities if any
        if battle_stats['abilities']:
            status += "Abilities:\n"
            for ability in battle_stats['abilities']:
                status += f"  {ability}\n"
        
        # Add quest information
        active_quests = self.pet.quest_states['active_quests']
        if active_quests:
            status += f"\nActive Quests ({len(active_quests)}):\n"
            for quest in active_quests[:3]:  # Show up to 3 quests
                quest_info = AVAILABLE_QUESTS.get(quest['id'], {})
                status += f"  {quest_info.get('name', quest['id'])}: {quest['progress']}/{quest_info.get('required_progress', 100)}\n"
            if len(active_quests) > 3:
                status += f"  ...and {len(active_quests) - 3} more\n"
        
        # Add education information
        education = self.pet.education_states
        if education['education_level'] > 0 or education['degrees'] or education['certifications']:
            status += f"\nEducation:\n"
            status += f"  Education Level: {education['education_level']}\n"
            
            if education['degrees']:
                status += f"  Degrees: {', '.join(education['degrees'][:3])}"
                if len(education['degrees']) > 3:
                    status += f" and {len(education['degrees']) - 3} more"
                status += "\n"
            
            if education['certifications']:
                status += f"  Certifications: {', '.join(education['certifications'][:3])}"
                if len(education['certifications']) > 3:
                    status += f" and {len(education['certifications']) - 3} more"
                status += "\n"
        
        # Add evolution information
        evolution = self.pet.evolution
        if evolution['evolution_stage'] > 0 or evolution['evolution_path']:
            status += f"\nEvolution:\n"
            status += f"  Stage: {evolution['evolution_stage']}\n"
            if evolution['evolution_path']:
                status += f"  Path: {' → '.join(evolution['evolution_path'])}\n"
        
        # Add achievement information
        achievements = self.pet.achievements
        if achievements['mastered']:
            status += f"\nAchievements: {len(achievements['mastered'])}\n"
            status += f"Achievement Points: {achievements['achievement_points']}\n"
        
        return status
    
    # --- Advanced State Management Methods ---
    def get_age_info(self) -> Dict[str, Any]:
        """Get comprehensive age information for the pet."""
        age_days = self.pet.calculate_age_days()
        biological_age = self.pet.calculate_biological_age()
        human_age = self.pet.calculate_human_age_equivalent()
        
        return {
            'age_days': round(age_days, 1),
            'biological_age_days': round(biological_age, 1),
            'human_age_equivalent': human_age,
            'maturity_level': self.pet.maturity_level,
            'growth_rate': self.pet.growth_rate
        }
    
    def manage_job(self, action: str, job_name: str = None, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's job status.
        
        Args:
            action: The action to perform (apply, quit, work, train)
            job_name: The name of the job (for apply action)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'apply':
            if not job_name:
                return False, "No job specified."
            
            # Check if pet meets job requirements
            job_requirements = JOB_TYPES.get(job_name, {}).get('requirements', {})
            
            # Check if pet meets minimum stats
            for stat, min_value in job_requirements.get('min_stats', {}).items():
                if hasattr(self.pet, stat) and getattr(self.pet, stat) < min_value:
                    return False, f"Your pet doesn't meet the {stat} requirement for this job."
            
            # Check if pet meets minimum age
            min_age = job_requirements.get('min_age', 0)
            if self.pet.calculate_human_age_equivalent() < min_age:
                return False, f"Your pet is too young for this job. Minimum age: {min_age} years."
            
            # Assign the job
            self.pet.job_states['current_job'] = job_name
            self.pet.job_states['job_level'] = 1
            self.pet.job_states['job_experience'] = 0
            
            # Initialize job-specific skills
            job_skills = JOB_TYPES.get(job_name, {}).get('skills', [])
            for skill in job_skills:
                if skill not in self.pet.job_states['skills']:
                    self.pet.job_states['skills'][skill] = 0
            
            self._add_interaction(InteractionType.CAREER, f"Started new job: {job_name}")
            return True, f"{self.pet.name} has been hired as a {job_name}!"
            
        elif action == 'quit':
            if not self.pet.job_states['current_job']:
                return False, f"{self.pet.name} doesn't currently have a job."
            
            old_job = self.pet.job_states['current_job']
            
            # Add to job history
            self.pet.job_states['job_history'].append({
                'job': old_job,
                'level': self.pet.job_states['job_level'],
                'experience': self.pet.job_states['job_experience']
            })
            
            # Reset current job
            self.pet.job_states['current_job'] = None
            self.pet.job_states['job_level'] = 0
            self.pet.job_states['job_experience'] = 0
            
            self._add_interaction(InteractionType.CAREER, f"Quit job: {old_job}")
            return True, f"{self.pet.name} has quit their job as a {old_job}."
            
        elif action == 'work':
            if not self.pet.job_states['current_job']:
                return False, f"{self.pet.name} doesn't currently have a job."
            
            # Check if pet has enough energy
            if self.pet.energy < 20:
                return False, f"{self.pet.name} is too tired to work right now."
            
            job = self.pet.job_states['current_job']
            job_info = JOB_TYPES.get(job, {})
            
            # Calculate work results
            exp_gain = job_info.get('exp_per_work', 10)
            money_gain = job_info.get('base_salary', 5) * self.pet.job_states['job_level']
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 20)
            self.pet.hunger = self._cap_stat(self.pet.hunger + 10)
            
            # Update job experience
            self.pet.job_states['job_experience'] += exp_gain
            
            # Check for level up
            exp_needed = 100 * self.pet.job_states['job_level']
            if self.pet.job_states['job_experience'] >= exp_needed:
                self.pet.job_states['job_level'] += 1
                self.pet.job_states['job_experience'] = 0
                level_up_message = f"\n{self.pet.name} has been promoted to level {self.pet.job_states['job_level']}!"
            else:
                level_up_message = ""
            
            # Improve job skills
            for skill in job_info.get('skills', []):
                if skill in self.pet.job_states['skills']:
                    self.pet.job_states['skills'][skill] += 1
            
            self._add_interaction(InteractionType.CAREER, f"Worked as a {job}")
            return True, f"{self.pet.name} worked as a {job} and earned {money_gain} coins.{level_up_message}"
            
        elif action == 'train':
            if not self.pet.job_states['current_job']:
                return False, f"{self.pet.name} doesn't currently have a job."
            
            skill = kwargs.get('skill')
            if not skill or skill not in self.pet.job_states['skills']:
                return False, "Invalid skill specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 15:
                return False, f"{self.pet.name} is too tired to train right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 15)
            self.pet.iq = self._cap_stat(self.pet.iq + 2)
            
            # Improve skill
            self.pet.job_states['skills'][skill] += 2
            
            self._add_interaction(InteractionType.LEARN, f"Trained in {skill}")
            return True, f"{self.pet.name} has improved their {skill} skill!"
        
        return False, "Invalid job action."
    
    def manage_battle(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's battle activities.
        
        Args:
            action: The action to perform (train, battle, use_ability)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'train':
            stat = kwargs.get('stat')
            valid_stats = ['strength', 'defense', 'speed', 'special_attack', 'special_defense']
            
            if not stat or stat not in valid_stats:
                return False, "Invalid battle stat specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 25:
                return False, f"{self.pet.name} is too tired to train right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 25)
            self.pet.hunger = self._cap_stat(self.pet.hunger + 15)
            
            # Improve battle stat
            self.pet.battle_states[stat] += 2
            
            self._add_interaction(InteractionType.TRAIN, f"Trained battle stat: {stat}")
            return True, f"{self.pet.name} has improved their {stat}!"
            
        elif action == 'battle':
            opponent = kwargs.get('opponent')
            if not opponent:
                return False, "No opponent specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 30:
                return False, f"{self.pet.name} is too tired to battle right now."
            
            # Calculate battle result
            pet_power = (
                self.pet.battle_states['strength'] +
                self.pet.battle_states['defense'] +
                self.pet.battle_states['speed'] +
                self.pet.battle_states['special_attack'] +
                self.pet.battle_states['special_defense']
            )
            
            opponent_info = BATTLE_OPPONENTS.get(opponent, {})
            opponent_power = opponent_info.get('power', 50)
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 30)
            self.pet.hunger = self._cap_stat(self.pet.hunger + 20)
            
            # Determine outcome
            if pet_power > opponent_power:
                # Victory
                self.pet.battle_states['battles_won'] += 1
                
                # Reward
                reward = opponent_info.get('reward', 10)
                
                # Chance to learn ability
                if random.random() < 0.2:  # 20% chance
                    possible_abilities = opponent_info.get('abilities', [])
                    if possible_abilities and not all(a in self.pet.battle_states['abilities'] for a in possible_abilities):
                        new_abilities = [a for a in possible_abilities if a not in self.pet.battle_states['abilities']]
                        if new_abilities:
                            ability = random.choice(new_abilities)
                            self.pet.battle_states['abilities'].append(ability)
                            ability_message = f"\n{self.pet.name} learned a new ability: {ability}!"
                        else:
                            ability_message = ""
                    else:
                        ability_message = ""
                else:
                    ability_message = ""
                
                self._add_interaction(InteractionType.BATTLE, f"Won battle against {opponent}")
                return True, f"{self.pet.name} defeated {opponent} and earned {reward} battle points!{ability_message}"
            else:
                # Defeat
                self.pet.battle_states['battles_lost'] += 1
                self._add_interaction(InteractionType.BATTLE, f"Lost battle against {opponent}")
                return True, f"{self.pet.name} was defeated by {opponent}. Better luck next time!"
                
        elif action == 'use_ability':
            ability = kwargs.get('ability')
            if not ability or ability not in self.pet.battle_states['abilities']:
                return False, f"{self.pet.name} doesn't know that ability."
            
            # Check if pet has enough energy
            if self.pet.energy < 15:
                return False, f"{self.pet.name} is too tired to use abilities right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 15)
            
            # Ability effects would be implemented here
            
            self._add_interaction(InteractionType.BATTLE, f"Used ability: {ability}")
            return True, f"{self.pet.name} used {ability}!"
        
        return False, "Invalid battle action."
    
    def manage_quest(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's quests.
        
        Args:
            action: The action to perform (accept, complete, abandon)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'accept':
            quest_id = kwargs.get('quest_id')
            if not quest_id or quest_id not in AVAILABLE_QUESTS:
                return False, "Invalid quest specified."
            
            # Check if already on this quest
            if any(q['id'] == quest_id for q in self.pet.quest_states['active_quests']):
                return False, f"{self.pet.name} is already on this quest."
            
            # Check if already completed this quest
            if any(q['id'] == quest_id for q in self.pet.quest_states['completed_quests']):
                return False, f"{self.pet.name} has already completed this quest."
            
            quest_info = AVAILABLE_QUESTS.get(quest_id, {})
            
            # Check requirements
            requirements = quest_info.get('requirements', {})
            
            # Check level requirement
            if self.pet.maturity_level < requirements.get('min_maturity', 0):
                return False, f"This quest requires maturity level {requirements.get('min_maturity', 0)}."
            
            # Add to active quests
            self.pet.quest_states['active_quests'].append({
                'id': quest_id,
                'progress': 0,
                'started_at': time.time_ns()
            })
            
            self._add_interaction(InteractionType.QUEST, f"Accepted quest: {quest_info.get('name', quest_id)}")
            return True, f"{self.pet.name} has accepted the quest: {quest_info.get('name', quest_id)}!"
            
        elif action == 'progress':
            quest_id = kwargs.get('quest_id')
            progress = kwargs.get('progress', 1)
            
            # Find the quest in active quests
            quest_index = None
            for i, quest in enumerate(self.pet.quest_states['active_quests']):
                if quest['id'] == quest_id:
                    quest_index = i
                    break
            
            if quest_index is None:
                return False, f"{self.pet.name} is not currently on this quest."
            
            quest = self.pet.quest_states['active_quests'][quest_index]
            quest_info = AVAILABLE_QUESTS.get(quest_id, {})
            
            # Update progress
            quest['progress'] += progress
            
            # Check if quest is complete
            if quest['progress'] >= quest_info.get('required_progress', 100):
                # Complete the quest
                completed_quest = self.pet.quest_states['active_quests'].pop(quest_index)
                completed_quest['completed_at'] = time.time_ns()
                self.pet.quest_states['completed_quests'].append(completed_quest)
                
                # Award quest points
                reward_points = quest_info.get('reward_points', 10)
                self.pet.quest_states['quest_points'] += reward_points
                
                # Award reputation
                faction = quest_info.get('faction')
                if faction:
                    if faction not in self.pet.quest_states['reputation']:
                        self.pet.quest_states['reputation'][faction] = 0
                    self.pet.quest_states['reputation'][faction] += quest_info.get('reputation_gain', 5)
                
                self._add_interaction(InteractionType.QUEST, f"Completed quest: {quest_info.get('name', quest_id)}")
                return True, f"{self.pet.name} has completed the quest: {quest_info.get('name', quest_id)}!"
            else:
                return True, f"{self.pet.name} made progress on the quest: {quest_info.get('name', quest_id)}. Progress: {quest['progress']}/{quest_info.get('required_progress', 100)}"
            
        elif action == 'abandon':
            quest_id = kwargs.get('quest_id')
            
            # Find the quest in active quests
            quest_index = None
            for i, quest in enumerate(self.pet.quest_states['active_quests']):
                if quest['id'] == quest_id:
                    quest_index = i
                    break
            
            if quest_index is None:
                return False, f"{self.pet.name} is not currently on this quest."
            
            quest = self.pet.quest_states['active_quests'].pop(quest_index)
            quest_info = AVAILABLE_QUESTS.get(quest_id, {})
            
            self._add_interaction(InteractionType.QUEST, f"Abandoned quest: {quest_info.get('name', quest_id)}")
            return True, f"{self.pet.name} has abandoned the quest: {quest_info.get('name', quest_id)}."
        
        return False, "Invalid quest action."
    
    def manage_education(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's education.
        
        Args:
            action: The action to perform (study, graduate, certify)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'study':
            subject = kwargs.get('subject')
            if not subject or subject not in EDUCATION_SUBJECTS:
                return False, "Invalid subject specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 20:
                return False, f"{self.pet.name} is too tired to study right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 20)
            self.pet.iq = self._cap_stat(self.pet.iq + 3)
            
            # Update subject knowledge
            if subject not in self.pet.education_states['subjects_studied']:
                self.pet.education_states['subjects_studied'][subject] = 0
            
            self.pet.education_states['subjects_studied'][subject] += 5
            
            self._add_interaction(InteractionType.LEARN, f"Studied {subject}")
            return True, f"{self.pet.name} studied {subject} and gained knowledge!"
            
        elif action == 'graduate':
            degree = kwargs.get('degree')
            if not degree or degree not in EDUCATION_DEGREES:
                return False, "Invalid degree specified."
            
            degree_info = EDUCATION_DEGREES.get(degree, {})
            
            # Check requirements
            for subject, min_knowledge in degree_info.get('requirements', {}).items():
                current_knowledge = self.pet.education_states['subjects_studied'].get(subject, 0)
                if current_knowledge < min_knowledge:
                    return False, f"{self.pet.name} needs more knowledge in {subject} to earn this degree."
            
            # Check if already has this degree
            if degree in self.pet.education_states['degrees']:
                return False, f"{self.pet.name} already has a {degree} degree."
            
            # Award the degree
            self.pet.education_states['degrees'].append(degree)
            
            # Increase education level
            self.pet.education_states['education_level'] += degree_info.get('level_increase', 1)
            
            self._add_interaction(InteractionType.MILESTONE, f"Graduated with a {degree} degree")
            return True, f"Congratulations! {self.pet.name} has earned a {degree} degree!"
            
        elif action == 'certify':
            certification = kwargs.get('certification')
            if not certification or certification not in EDUCATION_CERTIFICATIONS:
                return False, "Invalid certification specified."
            
            cert_info = EDUCATION_CERTIFICATIONS.get(certification, {})
            
            # Check requirements
            for subject, min_knowledge in cert_info.get('requirements', {}).items():
                current_knowledge = self.pet.education_states['subjects_studied'].get(subject, 0)
                if current_knowledge < min_knowledge:
                    return False, f"{self.pet.name} needs more knowledge in {subject} to earn this certification."
            
            # Check if already has this certification
            if certification in self.pet.education_states['certifications']:
                return False, f"{self.pet.name} already has a {certification} certification."
            
            # Award the certification
            self.pet.education_states['certifications'].append(certification)
            
            self._add_interaction(InteractionType.MILESTONE, f"Earned {certification} certification")
            return True, f"Congratulations! {self.pet.name} has earned a {certification} certification!"
        
        return False, "Invalid education action."
    
    def manage_evolution(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's evolution.
        
        Args:
            action: The action to perform (check, evolve)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'check':
            # Get current evolution stage
            current_stage = self.pet.evolution['evolution_stage']
            
            # Check if pet can evolve
            if current_stage >= len(EVOLUTION_PATHS.get(self.pet.species, [])):
                return True, f"{self.pet.name} has reached their final evolution stage."
            
            # Get next evolution
            next_evolution = EVOLUTION_PATHS.get(self.pet.species, [])[current_stage]
            
            # Check requirements
            requirements = next_evolution.get('requirements', {})
            missing = []
            
            # Check level requirement
            if self.pet.maturity_level < requirements.get('min_maturity', 0):
                missing.append(f"Maturity Level: {self.pet.maturity_level}/{requirements.get('min_maturity', 0)}")
            
            # Check stat requirements
            for stat, min_value in requirements.get('min_stats', {}).items():
                if hasattr(self.pet, stat) and getattr(self.pet, stat) < min_value:
                    missing.append(f"{stat.capitalize()}: {getattr(self.pet, stat)}/{min_value}")
            
            # Check achievement requirements
            for achievement in requirements.get('achievements', []):
                if achievement not in self.pet.achievements['mastered']:
                    missing.append(f"Achievement: {achievement}")
            
            if missing:
                return True, f"{self.pet.name} is not ready to evolve yet. Missing requirements:\n" + "\n".join(missing)
            else:
                return True, f"{self.pet.name} is ready to evolve to {next_evolution.get('name', 'next stage')}!"
            
        elif action == 'evolve':
            # Get current evolution stage
            current_stage = self.pet.evolution['evolution_stage']
            
            # Check if pet can evolve
            if current_stage >= len(EVOLUTION_PATHS.get(self.pet.species, [])):
                return False, f"{self.pet.name} has reached their final evolution stage."
            
            # Get next evolution
            next_evolution = EVOLUTION_PATHS.get(self.pet.species, [])[current_stage]
            
            # Check requirements
            requirements = next_evolution.get('requirements', {})
            
            # Check level requirement
            if self.pet.maturity_level < requirements.get('min_maturity', 0):
                return False, f"{self.pet.name} needs to reach maturity level {requirements.get('min_maturity', 0)} to evolve."
            
            # Check stat requirements
            for stat, min_value in requirements.get('min_stats', {}).items():
                if hasattr(self.pet, stat) and getattr(self.pet, stat) < min_value:
                    return False, f"{self.pet.name} needs {stat} of at least {min_value} to evolve."
            
            # Check achievement requirements
            for achievement in requirements.get('achievements', []):
                if achievement not in self.pet.achievements['mastered']:
                    return False, f"{self.pet.name} needs to master the {achievement} achievement to evolve."
            
            # Perform evolution
            self.pet.evolution['evolution_stage'] += 1
            self.pet.evolution['evolution_path'].append(next_evolution.get('name', f"Stage {self.pet.evolution['evolution_stage']}"))
            
            # Apply evolution bonuses
            bonuses = next_evolution.get('bonuses', {})
            
            for stat, bonus in bonuses.get('stats', {}).items():
                if hasattr(self.pet, stat):
                    current_value = getattr(self.pet, stat)
                    setattr(self.pet, stat, self._cap_stat(current_value + bonus))
            
            # Update potential evolutions
            self.pet.evolution['potential_evolutions'] = next_evolution.get('potential_next', [])
            
            self._add_interaction(InteractionType.MILESTONE, f"Evolved to {next_evolution.get('name', f'Stage {self.pet.evolution['evolution_stage']}')}")
            return True, f"Congratulations! {self.pet.name} has evolved to {next_evolution.get('name', f'Stage {self.pet.evolution['evolution_stage']}')}!"
        
        return False, "Invalid evolution action."
    
    def manage_achievements(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's achievements.
        
        Args:
            action: The action to perform (check, claim)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'check':
            achievement_id = kwargs.get('achievement_id')
            
            if not achievement_id or achievement_id not in ACHIEVEMENTS:
                return False, "Invalid achievement specified."
            
            achievement_info = ACHIEVEMENTS.get(achievement_id, {})
            
            # Check if already mastered
            if achievement_id in self.pet.achievements['mastered']:
                return True, f"{self.pet.name} has already mastered the {achievement_info.get('name', achievement_id)} achievement."
            
            # Check progress
            if achievement_id in self.pet.achievements['in_progress']:
                current_progress = self.pet.achievements['in_progress'][achievement_id]
                required_progress = achievement_info.get('required_progress', 1)
                
                return True, f"{self.pet.name}'s progress on {achievement_info.get('name', achievement_id)}: {current_progress}/{required_progress}"
            else:
                # Start tracking this achievement
                self.pet.achievements['in_progress'][achievement_id] = 0
                return True, f"{self.pet.name} has started working on the {achievement_info.get('name', achievement_id)} achievement."
            
        elif action == 'progress':
            achievement_id = kwargs.get('achievement_id')
            progress = kwargs.get('progress', 1)
            
            if not achievement_id or achievement_id not in ACHIEVEMENTS:
                return False, "Invalid achievement specified."
            
            achievement_info = ACHIEVEMENTS.get(achievement_id, {})
            
            # Check if already mastered
            if achievement_id in self.pet.achievements['mastered']:
                return False, f"{self.pet.name} has already mastered this achievement."
            
            # Update progress
            if achievement_id not in self.pet.achievements['in_progress']:
                self.pet.achievements['in_progress'][achievement_id] = 0
            
            self.pet.achievements['in_progress'][achievement_id] += progress
            
            # Check if achievement is complete
            current_progress = self.pet.achievements['in_progress'][achievement_id]
            required_progress = achievement_info.get('required_progress', 1)
            
            if current_progress >= required_progress:
                # Complete the achievement
                del self.pet.achievements['in_progress'][achievement_id]
                self.pet.achievements['mastered'].append(achievement_id)
                
                # Award achievement points
                points = achievement_info.get('points', 10)
                self.pet.achievements['achievement_points'] += points
                
                self._add_interaction(InteractionType.MILESTONE, f"Mastered achievement: {achievement_info.get('name', achievement_id)}")
                return True, f"Achievement unlocked! {self.pet.name} has mastered {achievement_info.get('name', achievement_id)} and earned {points} achievement points!"
            else:
                return True, f"{self.pet.name} made progress on {achievement_info.get('name', achievement_id)}. Progress: {current_progress}/{required_progress}"
        
        return False, "Invalid achievement action."
    
    def manage_dna(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's DNA and genetics.
        
        Args:
            action: The action to perform (analyze, mutate)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'analyze':
            # Analyze the pet's genetic traits
            species_traits = DNA_TRAITS.get(self.pet.species, {})
            
            # Initialize genetic traits if not already done
            if not self.pet.dna['genetic_traits']:
                for trait, options in species_traits.items():
                    # Randomly select a trait value
                    trait_value = random.choice(options)
                    self.pet.dna['genetic_traits'][trait] = trait_value
                    
                    # Determine if dominant or recessive
                    if random.random() < 0.7:  # 70% chance to be dominant
                        self.pet.dna['dominant_genes'].append(trait)
                    else:
                        self.pet.dna['recessive_genes'].append(trait)
            
            # Generate analysis report
            trait_report = []
            for trait, value in self.pet.dna['genetic_traits'].items():
                dominance = "Dominant" if trait in self.pet.dna['dominant_genes'] else "Recessive"
                trait_report.append(f"{trait}: {value} ({dominance})")
            
            mutation_report = []
            for mutation in self.pet.dna['mutations']:
                mutation_report.append(f"{mutation['name']}: {mutation['effect']}")
            
            self._add_interaction(InteractionType.LEARN, "Analyzed DNA")
            
            report = f"DNA Analysis for {self.pet.name}:\n\nGenetic Traits:\n" + "\n".join(trait_report)
            
            if mutation_report:
                report += "\n\nMutations:\n" + "\n".join(mutation_report)
            else:
                report += "\n\nNo mutations detected."
                
            return True, report
            
        elif action == 'mutate':
            # Check if pet has enough energy
            if self.pet.energy < 50:
                return False, f"{self.pet.name} doesn't have enough energy for genetic mutation."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 50)
            
            # Check for existing mutations
            if len(self.pet.dna['mutations']) >= 3:
                return False, f"{self.pet.name} already has the maximum number of mutations."
            
            # Random chance of successful mutation
            if random.random() < 0.5:  # 50% chance
                # Select a random mutation
                available_mutations = [m for m in DNA_MUTATIONS if not any(existing['name'] == m['name'] for existing in self.pet.dna['mutations'])]
                
                if not available_mutations:
                    return False, "No new mutations available."
                
                mutation = random.choice(available_mutations)
                self.pet.dna['mutations'].append(mutation)
                
                # Apply mutation effects
                for stat, change in mutation.get('stat_changes', {}).items():
                    if hasattr(self.pet, stat):
                        current_value = getattr(self.pet, stat)
                        setattr(self.pet, stat, self._cap_stat(current_value + change))
                
                self._add_interaction(InteractionType.MILESTONE, f"Developed mutation: {mutation['name']}")
                return True, f"{self.pet.name} has developed a new mutation: {mutation['name']}! Effect: {mutation['effect']}"
            else:
                return True, f"The mutation attempt was unsuccessful. {self.pet.name} seems unchanged."
        
        return False, "Invalid DNA action."e
            
        if color not in CRAFTING_MATERIALS[material_type]['colors']:
            return False
            
        if coverage < 0.0 or coverage > 1.0:
            return False
            
        material = CraftingMaterial(
            type=material_type,
            color=color,
            coverage=coverage,
            position=position
        )
        
        self.pet.materials.append(material)
        self._add_interaction(InteractionType.CRAFT, f"Added {color} {material_type} to {position}")
        return True
    
    def add_adaptation(self, adaptation_type: str, strength: int, position: str) -> bool:
        """Add an adaptation to the pet's critter form."""
        if not self.pet.base_animal:
            raise CraftingError("Pet must have a base animal before adding adaptations.")
        
        if adaptation_type not in ADAPTATIONS:
            return False
            
        if strength < 1 or strength > 10:
            return False
            
        adaptation = Adaptation(
            type=adaptation_type,
            strength=strength,
            position=position
        )
        
        self.pet.adaptations.append(adaptation)
        self._add_interaction(InteractionType.CRAFT, f"Added {adaptation_type} to {position}")
        return True
    
    def learn_fact(self, fact: str) -> bool:
        """Record that the pet has learned a fact."""
        if fact in self.pet.facts_learned:
            return False
            
        self.pet.facts_learned.add(fact)
        self._add_interaction(InteractionType.LEARN, f"Learned: {fact[:50]}..." if len(fact) > 50 else fact)
        
        # Add the fact to the AI memory system
        self.ai_manager.learn_fact(fact)
        
        # Process the interaction in the AI system
        self.ai_manager.process_interaction('learn', True, {'fact': fact})
        
        return True
    
    def set_base_animal(self, base_animal: str) -> bool:
        """Set the base animal for the pet's critter form."""
        if base_animal not in CRITTER_TYPES:
            return False
        
        self.pet.base_animal = base_animal
        self._add_interaction(InteractionType.CRAFT, f"Set base animal to {base_animal}")
        return True
    
    def get_adaptation_effectiveness(self, adaptation_type: str) -> int:
        """Calculate the overall effectiveness of a specific adaptation."""
        if not self.pet.base_animal:
            return 0
            
        matching_adaptations = [a for a in self.pet.adaptations if a.type == adaptation_type]
        if not matching_adaptations:
            return 0
            
        return sum(a.strength for a in matching_adaptations)
    
    def simulate_in_environment(self, environment: str) -> Dict[str, Any]:
        """Simulate how the pet's critter form would perform in a given environment."""
        if not self.pet.base_animal:
            raise CraftingError("Pet must have a base animal before simulation.")
            
        valid_environments = ['forest', 'ocean', 'desert', 'arctic', 'grassland']
        if environment not in valid_environments:
            raise ValueError(f"Invalid environment. Must be one of: {valid_environments}")
        
        results = {
            'environment': environment,
            'survival_score': 50,  # Base score
            'advantages': [],
            'disadvantages': []
        }
        
        # Base survival score based on animal type
        animal_info = CRITTER_TYPES.get(self.pet.base_animal, {})
        natural_habitat = animal_info.get('habitat', '').lower()
        
        # Adjust base score based on natural habitat match
        habitat_match = False
        if environment == 'forest' and any(x in natural_habitat for x in ['forest', 'jungle', 'woodland']):
            results['survival_score'] += 20
            habitat_match = True
        elif environment == 'ocean' and any(x in natural_habitat for x in ['ocean', 'sea', 'marine', 'aquatic']):
            results['survival_score'] += 20
            habitat_match = True
        elif environment == 'desert' and any(x in natural_habitat for x in ['desert', 'arid', 'dry']):
            results['survival_score'] += 20
            habitat_match = True
        elif environment == 'arctic' and any(x in natural_habitat for x in ['arctic', 'polar', 'tundra', 'cold']):
            results['survival_score'] += 20
            habitat_match = True
        elif environment == 'grassland' and any(x in natural_habitat for x in ['grassland', 'savanna', 'prairie']):
            results['survival_score'] += 20
            habitat_match = True
        
        if habitat_match:
            results['advantages'].append(f"Natural habitat match: {animal_info.get('display_name', self.pet.base_animal)} are naturally adapted to {environment}-like environments")
        else:
            results['disadvantages'].append(f"Habitat mismatch: {animal_info.get('display_name', self.pet.base_animal)} are not naturally adapted to {environment} environments")
        
        # Evaluate adaptations in this environment
        # (This would include detailed logic for each adaptation type in each environment)
        
        # Increase IQ from simulation
        self.pet.iq = self._cap_stat(self.pet.iq + 1)
        self._add_interaction(InteractionType.LEARN, f"Simulated in {environment} environment")
        
        # Add job information if applicable
        if self.pet.job_states['current_job']:
            job_name = self.pet.job_states['current_job']
            job_info = JOB_TYPES.get(job_name, {})
            status += f"\nJob: {job_info.get('display_name', job_name)}\n"
            status += f"Job Level: {self.pet.job_states['job_level']}\n"
            status += f"Job Experience: {self.pet.job_states['job_experience']}/{100 * self.pet.job_states['job_level']}\n"
            
            # Add skills
            if self.pet.job_states['skills']:
                status += "Skills:\n"
                for skill, level in self.pet.job_states['skills'].items():
                    status += f"  {skill.capitalize()}: {level}\n"
        
        # Add battle stats
        battle_stats = self.pet.battle_states
        status += f"\nBattle Stats:\n"
        status += f"  Strength: {battle_stats['strength']}\n"
        status += f"  Defense: {battle_stats['defense']}\n"
        status += f"  Speed: {battle_stats['speed']}\n"
        status += f"  Special Attack: {battle_stats['special_attack']}\n"
        status += f"  Special Defense: {battle_stats['special_defense']}\n"
        status += f"  Battles Won: {battle_stats['battles_won']}\n"
        
        # Add abilities if any
        if battle_stats['abilities']:
            status += "Abilities:\n"
            for ability in battle_stats['abilities']:
                status += f"  {ability}\n"
        
        # Add quest information
        active_quests = self.pet.quest_states['active_quests']
        if active_quests:
            status += f"\nActive Quests ({len(active_quests)}):\n"
            for quest in active_quests[:3]:  # Show up to 3 quests
                quest_info = AVAILABLE_QUESTS.get(quest['id'], {})
                status += f"  {quest_info.get('name', quest['id'])}: {quest['progress']}/{quest_info.get('required_progress', 100)}\n"
            if len(active_quests) > 3:
                status += f"  ...and {len(active_quests) - 3} more\n"
        
        # Add education information
        education = self.pet.education_states
        if education['education_level'] > 0 or education['degrees'] or education['certifications']:
            status += f"\nEducation:\n"
            status += f"  Education Level: {education['education_level']}\n"
            
            if education['degrees']:
                status += f"  Degrees: {', '.join(education['degrees'][:3])}"
                if len(education['degrees']) > 3:
                    status += f" and {len(education['degrees']) - 3} more"
                status += "\n"
            
            if education['certifications']:
                status += f"  Certifications: {', '.join(education['certifications'][:3])}"
                if len(education['certifications']) > 3:
                    status += f" and {len(education['certifications']) - 3} more"
                status += "\n"
        
        # Add evolution information
        evolution = self.pet.evolution
        if evolution['evolution_stage'] > 0 or evolution['evolution_path']:
            status += f"\nEvolution:\n"
            status += f"  Stage: {evolution['evolution_stage']}\n"
            if evolution['evolution_path']:
                status += f"  Path: {' → '.join(evolution['evolution_path'])}\n"
        
        # Add achievement information
        achievements = self.pet.achievements
        if achievements['mastered']:
            status += f"\nAchievements: {len(achievements['mastered'])}\n"
            status += f"Achievement Points: {achievements['achievement_points']}\n"
        
        return status
    
    # --- Advanced State Management Methods ---
    def get_age_info(self) -> Dict[str, Any]:
        """Get comprehensive age information for the pet."""
        age_days = self.pet.calculate_age_days()
        biological_age = self.pet.calculate_biological_age()
        human_age = self.pet.calculate_human_age_equivalent()
        
        return {
            'age_days': round(age_days, 1),
            'biological_age_days': round(biological_age, 1),
            'human_age_equivalent': human_age,
            'maturity_level': self.pet.maturity_level,
            'growth_rate': self.pet.growth_rate
        }
    
    def manage_job(self, action: str, job_name: str = None, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's job status.
        
        Args:
            action: The action to perform (apply, quit, work, train)
            job_name: The name of the job (for apply action)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'apply':
            if not job_name:
                return False, "No job specified."
            
            # Check if pet meets job requirements
            job_requirements = JOB_TYPES.get(job_name, {}).get('requirements', {})
            
            # Check if pet meets minimum stats
            for stat, min_value in job_requirements.get('min_stats', {}).items():
                if hasattr(self.pet, stat) and getattr(self.pet, stat) < min_value:
                    return False, f"Your pet doesn't meet the {stat} requirement for this job."
            
            # Check if pet meets minimum age
            min_age = job_requirements.get('min_age', 0)
            if self.pet.calculate_human_age_equivalent() < min_age:
                return False, f"Your pet is too young for this job. Minimum age: {min_age} years."
            
            # Assign the job
            self.pet.job_states['current_job'] = job_name
            self.pet.job_states['job_level'] = 1
            self.pet.job_states['job_experience'] = 0
            
            # Initialize job-specific skills
            job_skills = JOB_TYPES.get(job_name, {}).get('skills', [])
            for skill in job_skills:
                if skill not in self.pet.job_states['skills']:
                    self.pet.job_states['skills'][skill] = 0
            
            self._add_interaction(InteractionType.CAREER, f"Started new job: {job_name}")
            return True, f"{self.pet.name} has been hired as a {job_name}!"
            
        elif action == 'quit':
            if not self.pet.job_states['current_job']:
                return False, f"{self.pet.name} doesn't currently have a job."
            
            old_job = self.pet.job_states['current_job']
            
            # Add to job history
            self.pet.job_states['job_history'].append({
                'job': old_job,
                'level': self.pet.job_states['job_level'],
                'experience': self.pet.job_states['job_experience']
            })
            
            # Reset current job
            self.pet.job_states['current_job'] = None
            self.pet.job_states['job_level'] = 0
            self.pet.job_states['job_experience'] = 0
            
            self._add_interaction(InteractionType.CAREER, f"Quit job: {old_job}")
            return True, f"{self.pet.name} has quit their job as a {old_job}."
            
        elif action == 'work':
            if not self.pet.job_states['current_job']:
                return False, f"{self.pet.name} doesn't currently have a job."
            
            # Check if pet has enough energy
            if self.pet.energy < 20:
                return False, f"{self.pet.name} is too tired to work right now."
            
            job = self.pet.job_states['current_job']
            job_info = JOB_TYPES.get(job, {})
            
            # Calculate work results
            exp_gain = job_info.get('exp_per_work', 10)
            money_gain = job_info.get('base_salary', 5) * self.pet.job_states['job_level']
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 20)
            self.pet.hunger = self._cap_stat(self.pet.hunger + 10)
            
            # Update job experience
            self.pet.job_states['job_experience'] += exp_gain
            
            # Check for level up
            exp_needed = 100 * self.pet.job_states['job_level']
            if self.pet.job_states['job_experience'] >= exp_needed:
                self.pet.job_states['job_level'] += 1
                self.pet.job_states['job_experience'] = 0
                level_up_message = f"\n{self.pet.name} has been promoted to level {self.pet.job_states['job_level']}!"
            else:
                level_up_message = ""
            
            # Improve job skills
            for skill in job_info.get('skills', []):
                if skill in self.pet.job_states['skills']:
                    self.pet.job_states['skills'][skill] += 1
            
            self._add_interaction(InteractionType.CAREER, f"Worked as a {job}")
            return True, f"{self.pet.name} worked as a {job} and earned {money_gain} coins.{level_up_message}"
            
        elif action == 'train':
            if not self.pet.job_states['current_job']:
                return False, f"{self.pet.name} doesn't currently have a job."
            
            skill = kwargs.get('skill')
            if not skill or skill not in self.pet.job_states['skills']:
                return False, "Invalid skill specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 15:
                return False, f"{self.pet.name} is too tired to train right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 15)
            self.pet.iq = self._cap_stat(self.pet.iq + 2)
            
            # Improve skill
            self.pet.job_states['skills'][skill] += 2
            
            self._add_interaction(InteractionType.LEARN, f"Trained in {skill}")
            return True, f"{self.pet.name} has improved their {skill} skill!"
        
        return False, "Invalid job action."
    
    def manage_battle(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's battle activities.
        
        Args:
            action: The action to perform (train, battle, use_ability)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'train':
            stat = kwargs.get('stat')
            valid_stats = ['strength', 'defense', 'speed', 'special_attack', 'special_defense']
            
            if not stat or stat not in valid_stats:
                return False, "Invalid battle stat specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 25:
                return False, f"{self.pet.name} is too tired to train right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 25)
            self.pet.hunger = self._cap_stat(self.pet.hunger + 15)
            
            # Improve battle stat
            self.pet.battle_states[stat] += 2
            
            self._add_interaction(InteractionType.TRAIN, f"Trained battle stat: {stat}")
            return True, f"{self.pet.name} has improved their {stat}!"
            
        elif action == 'battle':
            opponent = kwargs.get('opponent')
            if not opponent:
                return False, "No opponent specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 30:
                return False, f"{self.pet.name} is too tired to battle right now."
            
            # Calculate battle result
            pet_power = (
                self.pet.battle_states['strength'] +
                self.pet.battle_states['defense'] +
                self.pet.battle_states['speed'] +
                self.pet.battle_states['special_attack'] +
                self.pet.battle_states['special_defense']
            )
            
            opponent_info = BATTLE_OPPONENTS.get(opponent, {})
            opponent_power = opponent_info.get('power', 50)
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 30)
            self.pet.hunger = self._cap_stat(self.pet.hunger + 20)
            
            # Determine outcome
            if pet_power > opponent_power:
                # Victory
                self.pet.battle_states['battles_won'] += 1
                
                # Reward
                reward = opponent_info.get('reward', 10)
                
                # Chance to learn ability
                if random.random() < 0.2:  # 20% chance
                    possible_abilities = opponent_info.get('abilities', [])
                    if possible_abilities and not all(a in self.pet.battle_states['abilities'] for a in possible_abilities):
                        new_abilities = [a for a in possible_abilities if a not in self.pet.battle_states['abilities']]
                        if new_abilities:
                            ability = random.choice(new_abilities)
                            self.pet.battle_states['abilities'].append(ability)
                            ability_message = f"\n{self.pet.name} learned a new ability: {ability}!"
                        else:
                            ability_message = ""
                    else:
                        ability_message = ""
                else:
                    ability_message = ""
                
                self._add_interaction(InteractionType.BATTLE, f"Won battle against {opponent}")
                return True, f"{self.pet.name} defeated {opponent} and earned {reward} battle points!{ability_message}"
            else:
                # Defeat
                self.pet.battle_states['battles_lost'] += 1
                self._add_interaction(InteractionType.BATTLE, f"Lost battle against {opponent}")
                return True, f"{self.pet.name} was defeated by {opponent}. Better luck next time!"
                
        elif action == 'use_ability':
            ability = kwargs.get('ability')
            if not ability or ability not in self.pet.battle_states['abilities']:
                return False, f"{self.pet.name} doesn't know that ability."
            
            # Check if pet has enough energy
            if self.pet.energy < 15:
                return False, f"{self.pet.name} is too tired to use abilities right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 15)
            
            # Ability effects would be implemented here
            
            self._add_interaction(InteractionType.BATTLE, f"Used ability: {ability}")
            return True, f"{self.pet.name} used {ability}!"
        
        return False, "Invalid battle action."
    
    def manage_quest(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's quests.
        
        Args:
            action: The action to perform (accept, complete, abandon)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'accept':
            quest_id = kwargs.get('quest_id')
            if not quest_id or quest_id not in AVAILABLE_QUESTS:
                return False, "Invalid quest specified."
            
            # Check if already on this quest
            if any(q['id'] == quest_id for q in self.pet.quest_states['active_quests']):
                return False, f"{self.pet.name} is already on this quest."
            
            # Check if already completed this quest
            if any(q['id'] == quest_id for q in self.pet.quest_states['completed_quests']):
                return False, f"{self.pet.name} has already completed this quest."
            
            quest_info = AVAILABLE_QUESTS.get(quest_id, {})
            
            # Check requirements
            requirements = quest_info.get('requirements', {})
            
            # Check level requirement
            if self.pet.maturity_level < requirements.get('min_maturity', 0):
                return False, f"This quest requires maturity level {requirements.get('min_maturity', 0)}."
            
            # Add to active quests
            self.pet.quest_states['active_quests'].append({
                'id': quest_id,
                'progress': 0,
                'started_at': time.time_ns()
            })
            
            self._add_interaction(InteractionType.QUEST, f"Accepted quest: {quest_info.get('name', quest_id)}")
            return True, f"{self.pet.name} has accepted the quest: {quest_info.get('name', quest_id)}!"
            
        elif action == 'progress':
            quest_id = kwargs.get('quest_id')
            progress = kwargs.get('progress', 1)
            
            # Find the quest in active quests
            quest_index = None
            for i, quest in enumerate(self.pet.quest_states['active_quests']):
                if quest['id'] == quest_id:
                    quest_index = i
                    break
            
            if quest_index is None:
                return False, f"{self.pet.name} is not currently on this quest."
            
            quest = self.pet.quest_states['active_quests'][quest_index]
            quest_info = AVAILABLE_QUESTS.get(quest_id, {})
            
            # Update progress
            quest['progress'] += progress
            
            # Check if quest is complete
            if quest['progress'] >= quest_info.get('required_progress', 100):
                # Complete the quest
                completed_quest = self.pet.quest_states['active_quests'].pop(quest_index)
                completed_quest['completed_at'] = time.time_ns()
                self.pet.quest_states['completed_quests'].append(completed_quest)
                
                # Award quest points
                reward_points = quest_info.get('reward_points', 10)
                self.pet.quest_states['quest_points'] += reward_points
                
                # Award reputation
                faction = quest_info.get('faction')
                if faction:
                    if faction not in self.pet.quest_states['reputation']:
                        self.pet.quest_states['reputation'][faction] = 0
                    self.pet.quest_states['reputation'][faction] += quest_info.get('reputation_gain', 5)
                
                self._add_interaction(InteractionType.QUEST, f"Completed quest: {quest_info.get('name', quest_id)}")
                return True, f"{self.pet.name} has completed the quest: {quest_info.get('name', quest_id)}!"
            else:
                return True, f"{self.pet.name} made progress on the quest: {quest_info.get('name', quest_id)}. Progress: {quest['progress']}/{quest_info.get('required_progress', 100)}"
            
        elif action == 'abandon':
            quest_id = kwargs.get('quest_id')
            
            # Find the quest in active quests
            quest_index = None
            for i, quest in enumerate(self.pet.quest_states['active_quests']):
                if quest['id'] == quest_id:
                    quest_index = i
                    break
            
            if quest_index is None:
                return False, f"{self.pet.name} is not currently on this quest."
            
            quest = self.pet.quest_states['active_quests'].pop(quest_index)
            quest_info = AVAILABLE_QUESTS.get(quest_id, {})
            
            self._add_interaction(InteractionType.QUEST, f"Abandoned quest: {quest_info.get('name', quest_id)}")
            return True, f"{self.pet.name} has abandoned the quest: {quest_info.get('name', quest_id)}."
        
        return False, "Invalid quest action."
    
    def manage_education(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's education.
        
        Args:
            action: The action to perform (study, graduate, certify)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'study':
            subject = kwargs.get('subject')
            if not subject or subject not in EDUCATION_SUBJECTS:
                return False, "Invalid subject specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 20:
                return False, f"{self.pet.name} is too tired to study right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 20)
            self.pet.iq = self._cap_stat(self.pet.iq + 3)
            
            # Update subject knowledge
            if subject not in self.pet.education_states['subjects_studied']:
                self.pet.education_states['subjects_studied'][subject] = 0
            
            self.pet.education_states['subjects_studied'][subject] += 5
            
            self._add_interaction(InteractionType.LEARN, f"Studied {subject}")
            return True, f"{self.pet.name} studied {subject} and gained knowledge!"
            
        elif action == 'graduate':
            degree = kwargs.get('degree')
            if not degree or degree not in EDUCATION_DEGREES:
                return False, "Invalid degree specified."
            
            degree_info = EDUCATION_DEGREES.get(degree, {})
            
            # Check requirements
            for subject, min_knowledge in degree_info.get('requirements', {}).items():
                current_knowledge = self.pet.education_states['subjects_studied'].get(subject, 0)
                if current_knowledge < min_knowledge:
                    return False, f"{self.pet.name} needs more knowledge in {subject} to earn this degree."
            
            # Check if already has this degree
            if degree in self.pet.education_states['degrees']:
                return False, f"{self.pet.name} already has a {degree} degree."
            
            # Award the degree
            self.pet.education_states['degrees'].append(degree)
            
            # Increase education level
            self.pet.education_states['education_level'] += degree_info.get('level_increase', 1)
            
            self._add_interaction(InteractionType.MILESTONE, f"Graduated with a {degree} degree")
            return True, f"Congratulations! {self.pet.name} has earned a {degree} degree!"
            
        elif action == 'certify':
            certification = kwargs.get('certification')
            if not certification or certification not in EDUCATION_CERTIFICATIONS:
                return False, "Invalid certification specified."
            
            cert_info = EDUCATION_CERTIFICATIONS.get(certification, {})
            
            # Check requirements
            for subject, min_knowledge in cert_info.get('requirements', {}).items():
                current_knowledge = self.pet.education_states['subjects_studied'].get(subject, 0)
                if current_knowledge < min_knowledge:
                    return False, f"{self.pet.name} needs more knowledge in {subject} to earn this certification."
            
            # Check if already has this certification
            if certification in self.pet.education_states['certifications']:
                return False, f"{self.pet.name} already has a {certification} certification."
            
            # Award the certification
            self.pet.education_states['certifications'].append(certification)
            
            self._add_interaction(InteractionType.MILESTONE, f"Earned {certification} certification")
            return True, f"Congratulations! {self.pet.name} has earned a {certification} certification!"
        
        return False, "Invalid education action."
    
    def manage_evolution(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's evolution.
        
        Args:
            action: The action to perform (check, evolve)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'check':
            # Get current evolution stage
            current_stage = self.pet.evolution['evolution_stage']
            
            # Check if pet can evolve
            if current_stage >= len(EVOLUTION_PATHS.get(self.pet.species, [])):
                return True, f"{self.pet.name} has reached their final evolution stage."
            
            # Get next evolution
            next_evolution = EVOLUTION_PATHS.get(self.pet.species, [])[current_stage]
            
            # Check requirements
            requirements = next_evolution.get('requirements', {})
            missing = []
            
            # Check level requirement
            if self.pet.maturity_level < requirements.get('min_maturity', 0):
                missing.append(f"Maturity Level: {self.pet.maturity_level}/{requirements.get('min_maturity', 0)}")
            
            # Check stat requirements
            for stat, min_value in requirements.get('min_stats', {}).items():
                if hasattr(self.pet, stat) and getattr(self.pet, stat) < min_value:
                    missing.append(f"{stat.capitalize()}: {getattr(self.pet, stat)}/{min_value}")
            
            # Check achievement requirements
            for achievement in requirements.get('achievements', []):
                if achievement not in self.pet.achievements['mastered']:
                    missing.append(f"Achievement: {achievement}")
            
            if missing:
                return True, f"{self.pet.name} is not ready to evolve yet. Missing requirements:\n" + "\n".join(missing)
            else:
                return True, f"{self.pet.name} is ready to evolve to {next_evolution.get('name', 'next stage')}!"
            
        elif action == 'evolve':
            # Get current evolution stage
            current_stage = self.pet.evolution['evolution_stage']
            
            # Check if pet can evolve
            if current_stage >= len(EVOLUTION_PATHS.get(self.pet.species, [])):
                return False, f"{self.pet.name} has reached their final evolution stage."
            
            # Get next evolution
            next_evolution = EVOLUTION_PATHS.get(self.pet.species, [])[current_stage]
            
            # Check requirements
            requirements = next_evolution.get('requirements', {})
            
            # Check level requirement
            if self.pet.maturity_level < requirements.get('min_maturity', 0):
                return False, f"{self.pet.name} needs to reach maturity level {requirements.get('min_maturity', 0)} to evolve."
            
            # Check stat requirements
            for stat, min_value in requirements.get('min_stats', {}).items():
                if hasattr(self.pet, stat) and getattr(self.pet, stat) < min_value:
                    return False, f"{self.pet.name} needs {stat} of at least {min_value} to evolve."
            
            # Check achievement requirements
            for achievement in requirements.get('achievements', []):
                if achievement not in self.pet.achievements['mastered']:
                    return False, f"{self.pet.name} needs to master the {achievement} achievement to evolve."
            
            # Perform evolution
            self.pet.evolution['evolution_stage'] += 1
            self.pet.evolution['evolution_path'].append(next_evolution.get('name', f"Stage {self.pet.evolution['evolution_stage']}"))
            
            # Apply evolution bonuses
            bonuses = next_evolution.get('bonuses', {})
            
            for stat, bonus in bonuses.get('stats', {}).items():
                if hasattr(self.pet, stat):
                    current_value = getattr(self.pet, stat)
                    setattr(self.pet, stat, self._cap_stat(current_value + bonus))
            
            # Update potential evolutions
            self.pet.evolution['potential_evolutions'] = next_evolution.get('potential_next', [])
            
            self._add_interaction(InteractionType.MILESTONE, f"Evolved to {next_evolution.get('name', f'Stage {self.pet.evolution['evolution_stage']}')}")
            return True, f"Congratulations! {self.pet.name} has evolved to {next_evolution.get('name', f'Stage {self.pet.evolution['evolution_stage']}')}!"
        
        return False, "Invalid evolution action."
    
    def manage_achievements(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's achievements.
        
        Args:
            action: The action to perform (check, claim)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'check':
            achievement_id = kwargs.get('achievement_id')
            
            if not achievement_id or achievement_id not in ACHIEVEMENTS:
                return False, "Invalid achievement specified."
            
            achievement_info = ACHIEVEMENTS.get(achievement_id, {})
            
            # Check if already mastered
            if achievement_id in self.pet.achievements['mastered']:
                return True, f"{self.pet.name} has already mastered the {achievement_info.get('name', achievement_id)} achievement."
            
            # Check progress
            if achievement_id in self.pet.achievements['in_progress']:
                current_progress = self.pet.achievements['in_progress'][achievement_id]
                required_progress = achievement_info.get('required_progress', 1)
                
                return True, f"{self.pet.name}'s progress on {achievement_info.get('name', achievement_id)}: {current_progress}/{required_progress}"
            else:
                # Start tracking this achievement
                self.pet.achievements['in_progress'][achievement_id] = 0
                return True, f"{self.pet.name} has started working on the {achievement_info.get('name', achievement_id)} achievement."
            
        elif action == 'progress':
            achievement_id = kwargs.get('achievement_id')
            progress = kwargs.get('progress', 1)
            
            if not achievement_id or achievement_id not in ACHIEVEMENTS:
                return False, "Invalid achievement specified."
            
            achievement_info = ACHIEVEMENTS.get(achievement_id, {})
            
            # Check if already mastered
            if achievement_id in self.pet.achievements['mastered']:
                return False, f"{self.pet.name} has already mastered this achievement."
            
            # Update progress
            if achievement_id not in self.pet.achievements['in_progress']:
                self.pet.achievements['in_progress'][achievement_id] = 0
            
            self.pet.achievements['in_progress'][achievement_id] += progress
            
            # Check if achievement is complete
            current_progress = self.pet.achievements['in_progress'][achievement_id]
            required_progress = achievement_info.get('required_progress', 1)
            
            if current_progress >= required_progress:
                # Complete the achievement
                del self.pet.achievements['in_progress'][achievement_id]
                self.pet.achievements['mastered'].append(achievement_id)
                
                # Award achievement points
                points = achievement_info.get('points', 10)
                self.pet.achievements['achievement_points'] += points
                
                self._add_interaction(InteractionType.MILESTONE, f"Mastered achievement: {achievement_info.get('name', achievement_id)}")
                return True, f"Achievement unlocked! {self.pet.name} has mastered {achievement_info.get('name', achievement_id)} and earned {points} achievement points!"
            else:
                return True, f"{self.pet.name} made progress on {achievement_info.get('name', achievement_id)}. Progress: {current_progress}/{required_progress}"
        
        return False, "Invalid achievement action."
    
    def manage_dna(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's DNA and genetics.
        
        Args:
            action: The action to perform (analyze, mutate)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'analyze':
            # Analyze the pet's genetic traits
            species_traits = DNA_TRAITS.get(self.pet.species, {})
            
            # Initialize genetic traits if not already done
            if not self.pet.dna['genetic_traits']:
                for trait, options in species_traits.items():
                    # Randomly select a trait value
                    trait_value = random.choice(options)
                    self.pet.dna['genetic_traits'][trait] = trait_value
                    
                    # Determine if dominant or recessive
                    if random.random() < 0.7:  # 70% chance to be dominant
                        self.pet.dna['dominant_genes'].append(trait)
                    else:
                        self.pet.dna['recessive_genes'].append(trait)
            
            # Generate analysis report
            trait_report = []
            for trait, value in self.pet.dna['genetic_traits'].items():
                dominance = "Dominant" if trait in self.pet.dna['dominant_genes'] else "Recessive"
                trait_report.append(f"{trait}: {value} ({dominance})")
            
            mutation_report = []
            for mutation in self.pet.dna['mutations']:
                mutation_report.append(f"{mutation['name']}: {mutation['effect']}")
            
            self._add_interaction(InteractionType.LEARN, "Analyzed DNA")
            
            report = f"DNA Analysis for {self.pet.name}:\n\nGenetic Traits:\n" + "\n".join(trait_report)
            
            if mutation_report:
                report += "\n\nMutations:\n" + "\n".join(mutation_report)
            else:
                report += "\n\nNo mutations detected."
                
            return True, report
            
        elif action == 'mutate':
            # Check if pet has enough energy
            if self.pet.energy < 50:
                return False, f"{self.pet.name} doesn't have enough energy for genetic mutation."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 50)
            
            # Check for existing mutations
            if len(self.pet.dna['mutations']) >= 3:
                return False, f"{self.pet.name} already has the maximum number of mutations."
            
            # Random chance of successful mutation
            if random.random() < 0.5:  # 50% chance
                # Select a random mutation
                available_mutations = [m for m in DNA_MUTATIONS if not any(existing['name'] == m['name'] for existing in self.pet.dna['mutations'])]
                
                if not available_mutations:
                    return False, "No new mutations available."
                
                mutation = random.choice(available_mutations)
                self.pet.dna['mutations'].append(mutation)
                
                # Apply mutation effects
                for stat, change in mutation.get('stat_changes', {}).items():
                    if hasattr(self.pet, stat):
                        current_value = getattr(self.pet, stat)
                        setattr(self.pet, stat, self._cap_stat(current_value + change))
                
                self._add_interaction(InteractionType.MILESTONE, f"Developed mutation: {mutation['name']}")
                return True, f"{self.pet.name} has developed a new mutation: {mutation['name']}! Effect: {mutation['effect']}"
            else:
                return True, f"The mutation attempt was unsuccessful. {self.pet.name} seems unchanged."
        
        return False, "Invalid DNA action."
    
    def check_migration_readiness(self) -> Tuple[bool, str]:
        """Check if the pet is ready for blockchain migration."""
        thresholds = MIGRATION_READINESS_THRESHOLDS
        
        # Calculate days owned
        current_time_ns = time.time_ns()
        days_owned = (current_time_ns - self.pet.creation_timestamp) / (1_000_000_000 * 60 * 60 * 24)
        
        # Count interactions
        interaction_count = len(self.pet.interaction_history)
        
        # Check all conditions
        is_ready = (
            self.pet.happiness >= thresholds['min_happiness'] and
            self.pet.energy >= thresholds['min_energy'] and
            self.pet.hunger <= thresholds['max_hunger'] and
            self.pet.iq >= thresholds.get('min_iq', 0) and
            self.pet.charisma >= thresholds.get('min_charisma', 0) and
            interaction_count >= thresholds['min_interactions'] and
            days_owned >= thresholds['min_days_owned']
        )
        
        if is_ready:
            return True, "Your pet is ready for blockchain migration!\nAll conditions met."
        else:
            missing = []
            if self.pet.happiness < thresholds['min_happiness']:
                missing.append(f"• Happiness: {self.pet.happiness}/{thresholds['min_happiness']} (min)")
            if self.pet.energy < thresholds['min_energy']:
                missing.append(f"• Energy: {self.pet.energy}/{thresholds['min_energy']} (min)")
            if self.pet.hunger > thresholds['max_hunger']:
                missing.append(f"• Hunger: {self.pet.hunger}/{thresholds['max_hunger']} (max)")
            if self.pet.iq < thresholds.get('min_iq', 0):
                missing.append(f"• IQ: {self.pet.iq}/{thresholds['min_iq']} (min)")
            if self.pet.charisma < thresholds.get('min_charisma', 0):
                missing.append(f"• Charisma: {self.pet.charisma}/{thresholds['min_charisma']} (min)")
            if interaction_count < thresholds['min_interactions']:
                missing.append(f"• Interactions: {interaction_count}/{thresholds['min_interactions']} (min)")
            if days_owned < thresholds['min_days_owned']:
                missing.append(f"• Days Owned: {days_owned:.1f}/{thresholds['min_days_owned']} (min)")
            
            return False, "Your pet is not yet ready for blockchain migration. Keep nurturing them!\nMissing conditions:\n" + "\n".join(missing)
    
    def update_zoologist_level(self) -> bool:
        """Update the zoologist level based on critters created."""
        current_level = self.pet.zoologist_level
        
        # Define the level progression
        level_order = ['novice', 'apprentice', 'journeyman', 'expert', 'master']
        current_index = level_order.index(current_level)
        
        # Check if eligible for next level
        for i in range(len(level_order) - 1, current_index, -1):
            level = level_order[i]
            requirements = ZOOLOGIST_LEVELS[level]
            
            if self.pet.critters_created >= requirements['required_critters']:
                self.pet.zoologist_level = level
                
                # Unlock new materials and adaptations
                self.pet.unlocked_materials.update(requirements['unlocked_materials'])
                self.pet.unlocked_adaptations.update(requirements['unlocked_adaptations'])
                
                return True
        
        return False
    
    def status(self) -> str:
        """Returns a formatted string with the pet's current status."""
        mood = self._get_current_mood()
        alerts = self._get_status_alerts()
        
        # Format the basic status
        status = (
            f"=== {self.pet.name}'s Status ===\n"
            f"Species: {PET_ARCHETYPES[self.pet.species]['display_name']}\n"
            f"Aura: {PET_AURA_COLORS[self.pet.aura_color]['display_name']}\n"
            f"Mood: {mood['name']} {mood['emoji']}\n"
            f"\n"
            f"Hunger: {self.pet.hunger}/{MAX_STAT}\n"
            f"Energy: {self.pet.energy}/{MAX_STAT}\n"
            f"Happiness: {self.pet.happiness}/{MAX_STAT}\n"
            f"IQ: {self.pet.iq}/{MAX_STAT}\n"
            f"Charisma: {self.pet.charisma}/{MAX_STAT}\n"
            f"Cleanliness: {self.pet.cleanliness}/{MAX_STAT}\n"
            f"Social: {self.pet.social}/{MAX_STAT}\n"
        )
        
        # Add mood description
        status += f"\n{self.pet.name} is {mood['description']}\n"
        
        # Add any active alerts
        if alerts:
            status += "\nNotice:\n"
            for alert in alerts:
                status += f"{self.pet.name} {alert['message']} {alert['emoji']}\n"
        
        # Add critter info if applicable
        if self.pet.base_animal:
            animal_info = CRITTER_TYPES.get(self.pet.base_animal, {})
            status += f"\nCritter Form: {animal_info.get('display_name', self.pet.base_animal)}\n"
            status += f"Materials: {len(self.pet.materials)}\n"
            status += f"Adaptations: {len(self.pet.adaptations)}\n"
            status += f"Facts Learned: {len(self.pet.facts_learned)}\n"
            status += f"Zoologist Level: {self.pet.zoologist_level}\n"
        
        # Add job information if applicable
        if self.pet.job_states['current_job']:
            job_name = self.pet.job_states['current_job']
            job_info = JOB_TYPES.get(job_name, {})
            status += f"\nJob: {job_info.get('display_name', job_name)}\n"
            status += f"Job Level: {self.pet.job_states['job_level']}\n"
            status += f"Job Experience: {self.pet.job_states['job_experience']}/{100 * self.pet.job_states['job_level']}\n"
            
            # Add skills
            if self.pet.job_states['skills']:
                status += "Skills:\n"
                for skill, level in self.pet.job_states['skills'].items():
                    status += f"  {skill.capitalize()}: {level}\n"
        
        # Add battle stats
        battle_stats = self.pet.battle_states
        status += f"\nBattle Stats:\n"
        status += f"  Strength: {battle_stats['strength']}\n"
        status += f"  Defense: {battle_stats['defense']}\n"
        status += f"  Speed: {battle_stats['speed']}\n"
        status += f"  Special Attack: {battle_stats['special_attack']}\n"
        status += f"  Special Defense: {battle_stats['special_defense']}\n"
        status += f"  Battles Won: {battle_stats['battles_won']}\n"
        
        # Add abilities if any
        if battle_stats['abilities']:
            status += "Abilities:\n"
            for ability in battle_stats['abilities']:
                status += f"  {ability}\n"
        
        # Add quest information
        active_quests = self.pet.quest_states['active_quests']
        if active_quests:
            status += f"\nActive Quests ({len(active_quests)}):\n"
            for quest in active_quests[:3]:  # Show up to 3 quests
                quest_info = AVAILABLE_QUESTS.get(quest['id'], {})
                status += f"  {quest_info.get('name', quest['id'])}: {quest['progress']}/{quest_info.get('required_progress', 100)}\n"
            if len(active_quests) > 3:
                status += f"  ...and {len(active_quests) - 3} more\n"
        
        # Add education information
        education = self.pet.education_states
        if education['education_level'] > 0 or education['degrees'] or education['certifications']:
            status += f"\nEducation:\n"
            status += f"  Education Level: {education['education_level']}\n"
            
            if education['degrees']:
                status += f"  Degrees: {', '.join(education['degrees'][:3])}"
                if len(education['degrees']) > 3:
                    status += f" and {len(education['degrees']) - 3} more"
                status += "\n"
            
            if education['certifications']:
                status += f"  Certifications: {', '.join(education['certifications'][:3])}"
                if len(education['certifications']) > 3:
                    status += f" and {len(education['certifications']) - 3} more"
                status += "\n"
        
        # Add evolution information
        evolution = self.pet.evolution
        if evolution['evolution_stage'] > 0 or evolution['evolution_path']:
            status += f"\nEvolution:\n"
            status += f"  Stage: {evolution['evolution_stage']}\n"
            if evolution['evolution_path']:
                status += f"  Path: {' → '.join(evolution['evolution_path'])}\n"
        
        # Add achievement information
        achievements = self.pet.achievements
        if achievements['mastered']:
            status += f"\nAchievements: {len(achievements['mastered'])}\n"
            status += f"Achievement Points: {achievements['achievement_points']}\n"
        
        return status
    
    # --- Advanced State Management Methods ---
    def get_age_info(self) -> Dict[str, Any]:
        """Get comprehensive age information for the pet."""
        age_days = self.pet.calculate_age_days()
        biological_age = self.pet.calculate_biological_age()
        human_age = self.pet.calculate_human_age_equivalent()
        
        return {
            'age_days': round(age_days, 1),
            'biological_age_days': round(biological_age, 1),
            'human_age_equivalent': human_age,
            'maturity_level': self.pet.maturity_level,
            'growth_rate': self.pet.growth_rate
        }
    
    def manage_job(self, action: str, job_name: str = None, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's job status.
        
        Args:
            action: The action to perform (apply, quit, work, train)
            job_name: The name of the job (for apply action)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'apply':
            if not job_name:
                return False, "No job specified."
            
            # Check if pet meets job requirements
            job_requirements = JOB_TYPES.get(job_name, {}).get('requirements', {})
            
            # Check if pet meets minimum stats
            for stat, min_value in job_requirements.get('min_stats', {}).items():
                if hasattr(self.pet, stat) and getattr(self.pet, stat) < min_value:
                    return False, f"Your pet doesn't meet the {stat} requirement for this job."
            
            # Check if pet meets minimum age
            min_age = job_requirements.get('min_age', 0)
            if self.pet.calculate_human_age_equivalent() < min_age:
                return False, f"Your pet is too young for this job. Minimum age: {min_age} years."
            
            # Assign the job
            self.pet.job_states['current_job'] = job_name
            self.pet.job_states['job_level'] = 1
            self.pet.job_states['job_experience'] = 0
            
            # Initialize job-specific skills
            job_skills = JOB_TYPES.get(job_name, {}).get('skills', [])
            for skill in job_skills:
                if skill not in self.pet.job_states['skills']:
                    self.pet.job_states['skills'][skill] = 0
            
            self._add_interaction(InteractionType.CAREER, f"Started new job: {job_name}")
            return True, f"{self.pet.name} has been hired as a {job_name}!"
            
        elif action == 'quit':
            if not self.pet.job_states['current_job']:
                return False, f"{self.pet.name} doesn't currently have a job."
            
            old_job = self.pet.job_states['current_job']
            
            # Add to job history
            self.pet.job_states['job_history'].append({
                'job': old_job,
                'level': self.pet.job_states['job_level'],
                'experience': self.pet.job_states['job_experience']
            })
            
            # Reset current job
            self.pet.job_states['current_job'] = None
            self.pet.job_states['job_level'] = 0
            self.pet.job_states['job_experience'] = 0
            
            self._add_interaction(InteractionType.CAREER, f"Quit job: {old_job}")
            return True, f"{self.pet.name} has quit their job as a {old_job}."
            
        elif action == 'work':
            if not self.pet.job_states['current_job']:
                return False, f"{self.pet.name} doesn't currently have a job."
            
            # Check if pet has enough energy
            if self.pet.energy < 20:
                return False, f"{self.pet.name} is too tired to work right now."
            
            job = self.pet.job_states['current_job']
            job_info = JOB_TYPES.get(job, {})
            
            # Calculate work results
            exp_gain = job_info.get('exp_per_work', 10)
            money_gain = job_info.get('base_salary', 5) * self.pet.job_states['job_level']
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 20)
            self.pet.hunger = self._cap_stat(self.pet.hunger + 10)
            
            # Update job experience
            self.pet.job_states['job_experience'] += exp_gain
            
            # Check for level up
            exp_needed = 100 * self.pet.job_states['job_level']
            if self.pet.job_states['job_experience'] >= exp_needed:
                self.pet.job_states['job_level'] += 1
                self.pet.job_states['job_experience'] = 0
                level_up_message = f"\n{self.pet.name} has been promoted to level {self.pet.job_states['job_level']}!"
            else:
                level_up_message = ""
            
            # Improve job skills
            for skill in job_info.get('skills', []):
                if skill in self.pet.job_states['skills']:
                    self.pet.job_states['skills'][skill] += 1
            
            self._add_interaction(InteractionType.CAREER, f"Worked as a {job}")
            return True, f"{self.pet.name} worked as a {job} and earned {money_gain} coins.{level_up_message}"
            
        elif action == 'train':
            if not self.pet.job_states['current_job']:
                return False, f"{self.pet.name} doesn't currently have a job."
            
            skill = kwargs.get('skill')
            if not skill or skill not in self.pet.job_states['skills']:
                return False, "Invalid skill specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 15:
                return False, f"{self.pet.name} is too tired to train right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 15)
            self.pet.iq = self._cap_stat(self.pet.iq + 2)
            
            # Improve skill
            self.pet.job_states['skills'][skill] += 2
            
            self._add_interaction(InteractionType.LEARN, f"Trained in {skill}")
            return True, f"{self.pet.name} has improved their {skill} skill!"
        
        return False, "Invalid job action."
    
    def manage_battle(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's battle activities.
        
        Args:
            action: The action to perform (train, battle, use_ability)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'train':
            stat = kwargs.get('stat')
            valid_stats = ['strength', 'defense', 'speed', 'special_attack', 'special_defense']
            
            if not stat or stat not in valid_stats:
                return False, "Invalid battle stat specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 25:
                return False, f"{self.pet.name} is too tired to train right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 25)
            self.pet.hunger = self._cap_stat(self.pet.hunger + 15)
            
            # Improve battle stat
            self.pet.battle_states[stat] += 2
            
            self._add_interaction(InteractionType.TRAIN, f"Trained battle stat: {stat}")
            return True, f"{self.pet.name} has improved their {stat}!"
            
        elif action == 'battle':
            opponent = kwargs.get('opponent')
            if not opponent:
                return False, "No opponent specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 30:
                return False, f"{self.pet.name} is too tired to battle right now."
            
            # Calculate battle result
            pet_power = (
                self.pet.battle_states['strength'] +
                self.pet.battle_states['defense'] +
                self.pet.battle_states['speed'] +
                self.pet.battle_states['special_attack'] +
                self.pet.battle_states['special_defense']
            )
            
            opponent_info = BATTLE_OPPONENTS.get(opponent, {})
            opponent_power = opponent_info.get('power', 50)
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 30)
            self.pet.hunger = self._cap_stat(self.pet.hunger + 20)
            
            # Determine outcome
            if pet_power > opponent_power:
                # Victory
                self.pet.battle_states['battles_won'] += 1
                
                # Reward
                reward = opponent_info.get('reward', 10)
                
                # Chance to learn ability
                if random.random() < 0.2:  # 20% chance
                    possible_abilities = opponent_info.get('abilities', [])
                    if possible_abilities and not all(a in self.pet.battle_states['abilities'] for a in possible_abilities):
                        new_abilities = [a for a in possible_abilities if a not in self.pet.battle_states['abilities']]
                        if new_abilities:
                            ability = random.choice(new_abilities)
                            self.pet.battle_states['abilities'].append(ability)
                            ability_message = f"\n{self.pet.name} learned a new ability: {ability}!"
                        else:
                            ability_message = ""
                    else:
                        ability_message = ""
                else:
                    ability_message = ""
                
                self._add_interaction(InteractionType.BATTLE, f"Won battle against {opponent}")
                return True, f"{self.pet.name} defeated {opponent} and earned {reward} battle points!{ability_message}"
            else:
                # Defeat
                self.pet.battle_states['battles_lost'] += 1
                self._add_interaction(InteractionType.BATTLE, f"Lost battle against {opponent}")
                return True, f"{self.pet.name} was defeated by {opponent}. Better luck next time!"
                
        elif action == 'use_ability':
            ability = kwargs.get('ability')
            if not ability or ability not in self.pet.battle_states['abilities']:
                return False, f"{self.pet.name} doesn't know that ability."
            
            # Check if pet has enough energy
            if self.pet.energy < 15:
                return False, f"{self.pet.name} is too tired to use abilities right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 15)
            
            # Ability effects would be implemented here
            
            self._add_interaction(InteractionType.BATTLE, f"Used ability: {ability}")
            return True, f"{self.pet.name} used {ability}!"
        
        return False, "Invalid battle action."
    
    def manage_quest(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's quests.
        
        Args:
            action: The action to perform (accept, complete, abandon)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'accept':
            quest_id = kwargs.get('quest_id')
            if not quest_id or quest_id not in AVAILABLE_QUESTS:
                return False, "Invalid quest specified."
            
            # Check if already on this quest
            if any(q['id'] == quest_id for q in self.pet.quest_states['active_quests']):
                return False, f"{self.pet.name} is already on this quest."
            
            # Check if already completed this quest
            if any(q['id'] == quest_id for q in self.pet.quest_states['completed_quests']):
                return False, f"{self.pet.name} has already completed this quest."
            
            quest_info = AVAILABLE_QUESTS.get(quest_id, {})
            
            # Check requirements
            requirements = quest_info.get('requirements', {})
            
            # Check level requirement
            if self.pet.maturity_level < requirements.get('min_maturity', 0):
                return False, f"This quest requires maturity level {requirements.get('min_maturity', 0)}."
            
            # Add to active quests
            self.pet.quest_states['active_quests'].append({
                'id': quest_id,
                'progress': 0,
                'started_at': time.time_ns()
            })
            
            self._add_interaction(InteractionType.QUEST, f"Accepted quest: {quest_info.get('name', quest_id)}")
            return True, f"{self.pet.name} has accepted the quest: {quest_info.get('name', quest_id)}!"
            
        elif action == 'progress':
            quest_id = kwargs.get('quest_id')
            progress = kwargs.get('progress', 1)
            
            # Find the quest in active quests
            quest_index = None
            for i, quest in enumerate(self.pet.quest_states['active_quests']):
                if quest['id'] == quest_id:
                    quest_index = i
                    break
            
            if quest_index is None:
                return False, f"{self.pet.name} is not currently on this quest."
            
            quest = self.pet.quest_states['active_quests'][quest_index]
            quest_info = AVAILABLE_QUESTS.get(quest_id, {})
            
            # Update progress
            quest['progress'] += progress
            
            # Check if quest is complete
            if quest['progress'] >= quest_info.get('required_progress', 100):
                # Complete the quest
                completed_quest = self.pet.quest_states['active_quests'].pop(quest_index)
                completed_quest['completed_at'] = time.time_ns()
                self.pet.quest_states['completed_quests'].append(completed_quest)
                
                # Award quest points
                reward_points = quest_info.get('reward_points', 10)
                self.pet.quest_states['quest_points'] += reward_points
                
                # Award reputation
                faction = quest_info.get('faction')
                if faction:
                    if faction not in self.pet.quest_states['reputation']:
                        self.pet.quest_states['reputation'][faction] = 0
                    self.pet.quest_states['reputation'][faction] += quest_info.get('reputation_gain', 5)
                
                self._add_interaction(InteractionType.QUEST, f"Completed quest: {quest_info.get('name', quest_id)}")
                return True, f"{self.pet.name} has completed the quest: {quest_info.get('name', quest_id)}!"
            else:
                return True, f"{self.pet.name} made progress on the quest: {quest_info.get('name', quest_id)}. Progress: {quest['progress']}/{quest_info.get('required_progress', 100)}"
            
        elif action == 'abandon':
            quest_id = kwargs.get('quest_id')
            
            # Find the quest in active quests
            quest_index = None
            for i, quest in enumerate(self.pet.quest_states['active_quests']):
                if quest['id'] == quest_id:
                    quest_index = i
                    break
            
            if quest_index is None:
                return False, f"{self.pet.name} is not currently on this quest."
            
            quest = self.pet.quest_states['active_quests'].pop(quest_index)
            quest_info = AVAILABLE_QUESTS.get(quest_id, {})
            
            self._add_interaction(InteractionType.QUEST, f"Abandoned quest: {quest_info.get('name', quest_id)}")
            return True, f"{self.pet.name} has abandoned the quest: {quest_info.get('name', quest_id)}."
        
        return False, "Invalid quest action."
    
    def manage_education(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's education.
        
        Args:
            action: The action to perform (study, graduate, certify)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'study':
            subject = kwargs.get('subject')
            if not subject or subject not in EDUCATION_SUBJECTS:
                return False, "Invalid subject specified."
            
            # Check if pet has enough energy
            if self.pet.energy < 20:
                return False, f"{self.pet.name} is too tired to study right now."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 20)
            self.pet.iq = self._cap_stat(self.pet.iq + 3)
            
            # Update subject knowledge
            if subject not in self.pet.education_states['subjects_studied']:
                self.pet.education_states['subjects_studied'][subject] = 0
            
            self.pet.education_states['subjects_studied'][subject] += 5
            
            self._add_interaction(InteractionType.LEARN, f"Studied {subject}")
            return True, f"{self.pet.name} studied {subject} and gained knowledge!"
            
        elif action == 'graduate':
            degree = kwargs.get('degree')
            if not degree or degree not in EDUCATION_DEGREES:
                return False, "Invalid degree specified."
            
            degree_info = EDUCATION_DEGREES.get(degree, {})
            
            # Check requirements
            for subject, min_knowledge in degree_info.get('requirements', {}).items():
                current_knowledge = self.pet.education_states['subjects_studied'].get(subject, 0)
                if current_knowledge < min_knowledge:
                    return False, f"{self.pet.name} needs more knowledge in {subject} to earn this degree."
            
            # Check if already has this degree
            if degree in self.pet.education_states['degrees']:
                return False, f"{self.pet.name} already has a {degree} degree."
            
            # Award the degree
            self.pet.education_states['degrees'].append(degree)
            
            # Increase education level
            self.pet.education_states['education_level'] += degree_info.get('level_increase', 1)
            
            self._add_interaction(InteractionType.MILESTONE, f"Graduated with a {degree} degree")
            return True, f"Congratulations! {self.pet.name} has earned a {degree} degree!"
            
        elif action == 'certify':
            certification = kwargs.get('certification')
            if not certification or certification not in EDUCATION_CERTIFICATIONS:
                return False, "Invalid certification specified."
            
            cert_info = EDUCATION_CERTIFICATIONS.get(certification, {})
            
            # Check requirements
            for subject, min_knowledge in cert_info.get('requirements', {}).items():
                current_knowledge = self.pet.education_states['subjects_studied'].get(subject, 0)
                if current_knowledge < min_knowledge:
                    return False, f"{self.pet.name} needs more knowledge in {subject} to earn this certification."
            
            # Check if already has this certification
            if certification in self.pet.education_states['certifications']:
                return False, f"{self.pet.name} already has a {certification} certification."
            
            # Award the certification
            self.pet.education_states['certifications'].append(certification)
            
            self._add_interaction(InteractionType.MILESTONE, f"Earned {certification} certification")
            return True, f"Congratulations! {self.pet.name} has earned a {certification} certification!"
        
        return False, "Invalid education action."
    
    def manage_evolution(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's evolution.
        
        Args:
            action: The action to perform (check, evolve)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'check':
            # Get current evolution stage
            current_stage = self.pet.evolution['evolution_stage']
            
            # Check if pet can evolve
            if current_stage >= len(EVOLUTION_PATHS.get(self.pet.species, [])):
                return True, f"{self.pet.name} has reached their final evolution stage."
            
            # Get next evolution
            next_evolution = EVOLUTION_PATHS.get(self.pet.species, [])[current_stage]
            
            # Check requirements
            requirements = next_evolution.get('requirements', {})
            missing = []
            
            # Check level requirement
            if self.pet.maturity_level < requirements.get('min_maturity', 0):
                missing.append(f"Maturity Level: {self.pet.maturity_level}/{requirements.get('min_maturity', 0)}")
            
            # Check stat requirements
            for stat, min_value in requirements.get('min_stats', {}).items():
                if hasattr(self.pet, stat) and getattr(self.pet, stat) < min_value:
                    missing.append(f"{stat.capitalize()}: {getattr(self.pet, stat)}/{min_value}")
            
            # Check achievement requirements
            for achievement in requirements.get('achievements', []):
                if achievement not in self.pet.achievements['mastered']:
                    missing.append(f"Achievement: {achievement}")
            
            if missing:
                return True, f"{self.pet.name} is not ready to evolve yet. Missing requirements:\n" + "\n".join(missing)
            else:
                return True, f"{self.pet.name} is ready to evolve to {next_evolution.get('name', 'next stage')}!"
            
        elif action == 'evolve':
            # Get current evolution stage
            current_stage = self.pet.evolution['evolution_stage']
            
            # Check if pet can evolve
            if current_stage >= len(EVOLUTION_PATHS.get(self.pet.species, [])):
                return False, f"{self.pet.name} has reached their final evolution stage."
            
            # Get next evolution
            next_evolution = EVOLUTION_PATHS.get(self.pet.species, [])[current_stage]
            
            # Check requirements
            requirements = next_evolution.get('requirements', {})
            
            # Check level requirement
            if self.pet.maturity_level < requirements.get('min_maturity', 0):
                return False, f"{self.pet.name} needs to reach maturity level {requirements.get('min_maturity', 0)} to evolve."
            
            # Check stat requirements
            for stat, min_value in requirements.get('min_stats', {}).items():
                if hasattr(self.pet, stat) and getattr(self.pet, stat) < min_value:
                    return False, f"{self.pet.name} needs {stat} of at least {min_value} to evolve."
            
            # Check achievement requirements
            for achievement in requirements.get('achievements', []):
                if achievement not in self.pet.achievements['mastered']:
                    return False, f"{self.pet.name} needs to master the {achievement} achievement to evolve."
            
            # Perform evolution
            self.pet.evolution['evolution_stage'] += 1
            self.pet.evolution['evolution_path'].append(next_evolution.get('name', f"Stage {self.pet.evolution['evolution_stage']}"))
            
            # Apply evolution bonuses
            bonuses = next_evolution.get('bonuses', {})
            
            for stat, bonus in bonuses.get('stats', {}).items():
                if hasattr(self.pet, stat):
                    current_value = getattr(self.pet, stat)
                    setattr(self.pet, stat, self._cap_stat(current_value + bonus))
            
            # Update potential evolutions
            self.pet.evolution['potential_evolutions'] = next_evolution.get('potential_next', [])
            
            self._add_interaction(InteractionType.MILESTONE, f"Evolved to {next_evolution.get('name', f'Stage {self.pet.evolution['evolution_stage']}')}")
            return True, f"Congratulations! {self.pet.name} has evolved to {next_evolution.get('name', f'Stage {self.pet.evolution['evolution_stage']}')}!"
        
        return False, "Invalid evolution action."
    
    def manage_achievements(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's achievements.
        
        Args:
            action: The action to perform (check, claim)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'check':
            achievement_id = kwargs.get('achievement_id')
            
            if not achievement_id or achievement_id not in ACHIEVEMENTS:
                return False, "Invalid achievement specified."
            
            achievement_info = ACHIEVEMENTS.get(achievement_id, {})
            
            # Check if already mastered
            if achievement_id in self.pet.achievements['mastered']:
                return True, f"{self.pet.name} has already mastered the {achievement_info.get('name', achievement_id)} achievement."
            
            # Check progress
            if achievement_id in self.pet.achievements['in_progress']:
                current_progress = self.pet.achievements['in_progress'][achievement_id]
                required_progress = achievement_info.get('required_progress', 1)
                
                return True, f"{self.pet.name}'s progress on {achievement_info.get('name', achievement_id)}: {current_progress}/{required_progress}"
            else:
                # Start tracking this achievement
                self.pet.achievements['in_progress'][achievement_id] = 0
                return True, f"{self.pet.name} has started working on the {achievement_info.get('name', achievement_id)} achievement."
            
        elif action == 'progress':
            achievement_id = kwargs.get('achievement_id')
            progress = kwargs.get('progress', 1)
            
            if not achievement_id or achievement_id not in ACHIEVEMENTS:
                return False, "Invalid achievement specified."
            
            achievement_info = ACHIEVEMENTS.get(achievement_id, {})
            
            # Check if already mastered
            if achievement_id in self.pet.achievements['mastered']:
                return False, f"{self.pet.name} has already mastered this achievement."
            
            # Update progress
            if achievement_id not in self.pet.achievements['in_progress']:
                self.pet.achievements['in_progress'][achievement_id] = 0
            
            self.pet.achievements['in_progress'][achievement_id] += progress
            
            # Check if achievement is complete
            current_progress = self.pet.achievements['in_progress'][achievement_id]
            required_progress = achievement_info.get('required_progress', 1)
            
            if current_progress >= required_progress:
                # Complete the achievement
                del self.pet.achievements['in_progress'][achievement_id]
                self.pet.achievements['mastered'].append(achievement_id)
                
                # Award achievement points
                points = achievement_info.get('points', 10)
                self.pet.achievements['achievement_points'] += points
                
                self._add_interaction(InteractionType.MILESTONE, f"Mastered achievement: {achievement_info.get('name', achievement_id)}")
                return True, f"Achievement unlocked! {self.pet.name} has mastered {achievement_info.get('name', achievement_id)} and earned {points} achievement points!"
            else:
                return True, f"{self.pet.name} made progress on {achievement_info.get('name', achievement_id)}. Progress: {current_progress}/{required_progress}"
        
        return False, "Invalid achievement action."
    
    def manage_dna(self, action: str, **kwargs) -> Tuple[bool, str]:
        """
        Manage the pet's DNA and genetics.
        
        Args:
            action: The action to perform (analyze, mutate)
            **kwargs: Additional parameters specific to the action
            
        Returns:
            Tuple of (success, message)
        """
        if action == 'analyze':
            # Analyze the pet's genetic traits
            species_traits = DNA_TRAITS.get(self.pet.species, {})
            
            # Initialize genetic traits if not already done
            if not self.pet.dna['genetic_traits']:
                for trait, options in species_traits.items():
                    # Randomly select a trait value
                    trait_value = random.choice(options)
                    self.pet.dna['genetic_traits'][trait] = trait_value
                    
                    # Determine if dominant or recessive
                    if random.random() < 0.7:  # 70% chance to be dominant
                        self.pet.dna['dominant_genes'].append(trait)
                    else:
                        self.pet.dna['recessive_genes'].append(trait)
            
            # Generate analysis report
            trait_report = []
            for trait, value in self.pet.dna['genetic_traits'].items():
                dominance = "Dominant" if trait in self.pet.dna['dominant_genes'] else "Recessive"
                trait_report.append(f"{trait}: {value} ({dominance})")
            
            mutation_report = []
            for mutation in self.pet.dna['mutations']:
                mutation_report.append(f"{mutation['name']}: {mutation['effect']}")
            
            self._add_interaction(InteractionType.LEARN, "Analyzed DNA")
            
            report = f"DNA Analysis for {self.pet.name}:\n\nGenetic Traits:\n" + "\n".join(trait_report)
            
            if mutation_report:
                report += "\n\nMutations:\n" + "\n".join(mutation_report)
            else:
                report += "\n\nNo mutations detected."
                
            return True, report
            
        elif action == 'mutate':
            # Check if pet has enough energy
            if self.pet.energy < 50:
                return False, f"{self.pet.name} doesn't have enough energy for genetic mutation."
            
            # Apply stat changes
            self.pet.energy = self._cap_stat(self.pet.energy - 50)
            
            # Check for existing mutations
            if len(self.pet.dna['mutations']) >= 3:
                return False, f"{self.pet.name} already has the maximum number of mutations."
            
            # Random chance of successful mutation
            if random.random() < 0.5:  # 50% chance
                # Select a random mutation
                available_mutations = [m for m in DNA_MUTATIONS if not any(existing['name'] == m['name'] for existing in self.pet.dna['mutations'])]
                
                if not available_mutations:
                    return False, "No new mutations available."
                
                mutation = random.choice(available_mutations)
                self.pet.dna['mutations'].append(mutation)
                
                # Apply mutation effects
                for stat, change in mutation.get('stat_changes', {}).items():
                    if hasattr(self.pet, stat):
                        current_value = getattr(self.pet, stat)
                        setattr(self.pet, stat, self._cap_stat(current_value + change))
                
                self._add_interaction(InteractionType.MILESTONE, f"Developed mutation: {mutation['name']}")
                return True, f"{self.pet.name} has developed a new mutation: {mutation['name']}! Effect: {mutation['effect']}"
            else:
                return True, f"The mutation attempt was unsuccessful. {self.pet.name} seems unchanged."
        
        return False, "Invalid DNA action."


# --- Persistence Functions ---
def save_integrated_pet(pet: IntegratedPet, pet_manager: IntegratedPetManager, filename: str):
    """Saves a pet's state to a JSON file."""
    # Create a combined data structure with pet data and AI data
    combined_data = {
        'pet': pet.to_dict(),
        'ai': pet_manager.ai_manager.to_dict()
    }
    
    with open(filename, 'w') as f:
        f.write(json.dumps(combined_data, indent=2))

def load_integrated_pet(filename: str) -> Tuple[IntegratedPet, Dict[str, Any]]:
    """
    Loads a pet's state from a JSON file.
    
    Returns:
        Tuple containing the pet object and the AI data dictionary
    """
    with open(filename, 'r') as f:
        combined_data = json.loads(f.read())
    
    pet_data = combined_data.get('pet', {})
    ai_data = combined_data.get('ai', {})
    
    pet = IntegratedPet.from_dict(pet_data)
    
    return pet, ai_data