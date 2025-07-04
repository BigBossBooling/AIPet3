# pet/integrated_core.py
"""
Integrated core functionality for CritterCraft, combining the pet care system
with the educational critter creation system.

This module defines the IntegratedPet class which merges the functionality of
Pet and Critter classes, providing a unified experience.

Following KISS principles:
- K (Know Your Core, Keep it Clear): Clear separation of concerns with well-defined interfaces
- I (Iterate Intelligently): Structured for easy updates and maintenance
- S (Systematize for Scalability): Modular design with clear interfaces
- S (Sense the Landscape & Stimulate Engagement): Designed for user engagement
"""

import json
import time
import uuid
from typing import Dict, Any, List, Optional, Set, Callable, Tuple
from dataclasses import dataclass, field, asdict
from enum import Enum, auto

# Import from the centralized config file
from config import (
    Stat, Mood as ConfigMood, PersonalityTrait as ConfigPersonalityTrait,
    GenesisPetConfig, CritterCraftConfig
)

# Import from pet_core.py and critter_core.py
from pet_core import (
    Pet, PetLogicManager, PetPersistence, 
    Mood as PetMood, InteractionType as PetInteractionType
)

from critter_core import (
    Critter, CritterPersistence,
    MaterialType, AdaptationType, BodyPosition, Environment,
    CraftingMaterial, Adaptation
)

# --- Custom Exceptions ---
class IntegratedPetError(Exception):
    """Base exception for integrated pet-related errors."""
    pass

class IntegratedPetInitializationError(IntegratedPetError):
    """Raised when an integrated pet cannot be initialized due to invalid parameters."""
    pass

class InsufficientResourceError(IntegratedPetError):
    """Raised when an action cannot be performed due to insufficient resources."""
    pass

# --- Enums for Type Safety and Readability ---
class IntegratedInteractionType(Enum):
    """Types of interactions for the integrated pet system."""
    # Pet-related interactions
    FEED = "feed"
    PLAY = "play"
    CHAT = "chat"
    GROOM = "groom"
    
    # Critter-related interactions
    CRAFT = "craft"
    LEARN = "learn"
    
    # Advanced interactions
    TRAIN = "train"
    BATTLE = "battle"
    QUEST = "quest"
    CAREER = "career"
    MILESTONE = "milestone"
    
    # System interactions
    TICK_DECAY = "tick_decay"
    LEVEL_UP = "level_up"
    EVOLUTION = "evolution"

# --- Core Data Models ---
@dataclass
class IntegratedInteractionRecord:
    """Represents a single interaction event with the integrated pet."""
    timestamp: int
    type: IntegratedInteractionType
    details: Optional[str] = None
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for serialization."""
        return {
            'timestamp': self.timestamp,
            'type': self.type.value,
            'details': self.details
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'IntegratedInteractionRecord':
        """Create from dictionary after deserialization."""
        return cls(
            timestamp=data['timestamp'],
            type=IntegratedInteractionType(data['type']),
            details=data.get('details')
        )

@dataclass
class IntegratedPet:
    """
    Represents a CritterCraft pet that combines pet care and critter creation.
    This is the core data model for the integrated system.
    
    Following KISS principles:
    - K: Clear separation of data and validation
    - I: Structured for easy updates
    - S: Systematized with type-safe enums
    - S: Designed for user engagement
    """
    # Basic pet information
    name: str
    species: str  # Maps to GenesisPetConfig.Archetypes.DEFINITIONS
    aura_color: str
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    
    # Core pet stats - initialized from GenesisPetConfig
    hunger: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.HUNGER])
    energy: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.ENERGY])
    happiness: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.HAPPINESS])
    iq: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.IQ])
    charisma: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.CHARISMA])
    cleanliness: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.CLEANLINESS])
    social: int = field(default_factory=lambda: GenesisPetConfig.Core.INITIAL_STATS[Stat.SOCIAL])
    
    # Critter-specific attributes
    base_animal: Optional[str] = None  # Maps to CritterCraftConfig.CRITTER_TYPES
    materials: List[CraftingMaterial] = field(default_factory=list)
    adaptations: List[Adaptation] = field(default_factory=list)
    facts_learned: Set[str] = field(default_factory=set)
    
    # Personality
    personality_traits: Dict[str, int] = field(default_factory=lambda: {
        trait.name.lower(): 50 for trait in ConfigPersonalityTrait
    })
    
    # Timestamps
    creation_timestamp: int = field(default_factory=time.time_ns)
    last_active_timestamp: int = field(default_factory=time.time_ns)
    
    # History
    interaction_history: List[IntegratedInteractionRecord] = field(default_factory=list)
    
    # Zoologist progression
    zoologist_level: str = 'novice'
    critters_created: int = 0
    
    # Age tracking
    growth_rate: float = 1.0  # Base growth rate multiplier
    maturity_level: int = 0   # 0-100 scale of maturity
    
    # State tracking systems
    job_states: Dict[str, Any] = field(default_factory=dict)
    battle_states: Dict[str, Any] = field(default_factory=dict)
    quest_states: Dict[str, Any] = field(default_factory=dict)
    education_states: Dict[str, Any] = field(default_factory=dict)
    achievements: Dict[str, Any] = field(default_factory=dict)
    evolution: Dict[str, Any] = field(default_factory=dict)
    dna: Dict[str, Any] = field(default_factory=dict)
    genealogy: Dict[str, Any] = field(default_factory=dict)
    
    def __post_init__(self):
        """Perform post-initialization validation."""
        # Validate name
        self.name = self.name.strip()
        if not self.name or len(self.name) > 20 or not self.name.isprintable():
            raise IntegratedPetInitializationError("Pet name must be 1-20 printable characters.")
            
        # Validate species
        if self.species not in GenesisPetConfig.Archetypes.DEFINITIONS:
            raise IntegratedPetInitializationError(f"Invalid species: {self.species}.")
            
        # Validate aura color
        if self.aura_color not in GenesisPetConfig.Auras.DEFINITIONS:
            raise IntegratedPetInitializationError(f"Invalid aura color: {self.aura_color}.")
            
        # Validate base animal if provided
        if self.base_animal and self.base_animal not in CritterCraftConfig.CRITTER_TYPES:
            raise IntegratedPetInitializationError(f"Invalid base animal: {self.base_animal}.")
        
        # Apply species-specific stat modifiers
        species_info = GenesisPetConfig.Archetypes.DEFINITIONS.get(self.species, {})
        stat_modifiers = species_info.get('stat_modifiers', {})
        
        for stat, modifier in stat_modifiers.items():
            stat_name = stat.name.lower()
            if hasattr(self, stat_name):
                current_value = getattr(self, stat_name)
                setattr(self, stat_name, 
                        max(0, min(GenesisPetConfig.Core.MAX_STAT, current_value + modifier)))
        
        # Initialize state tracking systems with default values if empty
        self._initialize_state_tracking_systems()
    
    def _initialize_state_tracking_systems(self):
        """Initialize state tracking systems with default values if not already set."""
        # Job states
        if not self.job_states:
            self.job_states = {
                'current_job': None,
                'job_level': 0,
                'job_experience': 0,
                'job_history': [],
                'skills': {}
            }
        
        # Battle states
        if not self.battle_states:
            self.battle_states = {
                'strength': 10,
                'defense': 10,
                'speed': 10,
                'special_attack': 10,
                'special_defense': 10,
                'battles_won': 0,
                'battles_lost': 0,
                'abilities': [],
                'battle_items': []
            }
        
        # Quest states
        if not self.quest_states:
            self.quest_states = {
                'active_quests': [],
                'completed_quests': [],
                'quest_points': 0,
                'reputation': {}
            }
        
        # Education states
        if not self.education_states:
            self.education_states = {
                'education_level': 0,
                'subjects_studied': {},
                'degrees': [],
                'certifications': []
            }
        
        # Achievement tracking
        if not self.achievements:
            self.achievements = {
                'mastered': [],
                'in_progress': {},
                'achievement_points': 0
            }
        
        # Evolution tracking
        if not self.evolution:
            self.evolution = {
                'evolution_stage': 0,
                'evolution_path': [],
                'potential_evolutions': [],
                'evolution_requirements': {}
            }
        
        # DNA tracking
        if not self.dna:
            self.dna = {
                'genetic_traits': {},
                'dominant_genes': [],
                'recessive_genes': [],
                'mutations': []
            }
        
        # Genealogy tracking
        if not self.genealogy:
            self.genealogy = {
                'parents': [],
                'siblings': [],
                'offspring': [],
                'generation': 1
            }
    
    def to_dict(self) -> Dict[str, Any]:
        """
        Serialize the IntegratedPet object to a dictionary.
        
        Returns:
            Dictionary representation of the integrated pet
        """
        # Convert materials and adaptations to dictionaries
        materials_data = [
            {
                'type': m.type.value,
                'color': m.color,
                'coverage': m.coverage,
                'position': m.position.value
            } for m in self.materials
        ]
        
        adaptations_data = [
            {
                'type': a.type.value,
                'strength': a.strength,
                'position': a.position.value
            } for a in self.adaptations
        ]
        
        # Convert interaction history to dictionaries
        interaction_history_data = [
            record.to_dict() for record in self.interaction_history
        ]
        
        # Create the main data dictionary
        data = {
            # Basic information
            'name': self.name,
            'species': self.species,
            'aura_color': self.aura_color,
            'id': self.id,
            
            # Core stats
            'hunger': self.hunger,
            'energy': self.energy,
            'happiness': self.happiness,
            'iq': self.iq,
            'charisma': self.charisma,
            'cleanliness': self.cleanliness,
            'social': self.social,
            
            # Critter attributes
            'base_animal': self.base_animal,
            'materials': materials_data,
            'adaptations': adaptations_data,
            'facts_learned': list(self.facts_learned),
            
            # Personality
            'personality_traits': self.personality_traits,
            
            # Timestamps
            'creation_timestamp': self.creation_timestamp,
            'last_active_timestamp': self.last_active_timestamp,
            
            # History
            'interaction_history': interaction_history_data,
            
            # Progression
            'zoologist_level': self.zoologist_level,
            'critters_created': self.critters_created,
            'growth_rate': self.growth_rate,
            'maturity_level': self.maturity_level,
            
            # State tracking systems
            'job_states': self.job_states,
            'battle_states': self.battle_states,
            'quest_states': self.quest_states,
            'education_states': self.education_states,
            'achievements': self.achievements,
            'evolution': self.evolution,
            'dna': self.dna,
            'genealogy': self.genealogy
        }
        
        return data
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'IntegratedPet':
        """
        Deserialize an IntegratedPet object from a dictionary.
        
        Args:
            data: Dictionary representation of an integrated pet
            
        Returns:
            An IntegratedPet instance
        """
        # Create a copy to avoid modifying the input
        data_copy = data.copy()
        
        # Handle materials
        materials_data = data_copy.pop('materials', [])
        materials = []
        for m_data in materials_data:
            try:
                material = CraftingMaterial(
                    type=MaterialType.from_string(m_data['type']),
                    color=m_data['color'],
                    coverage=m_data['coverage'],
                    position=BodyPosition.from_string(m_data['position'])
                )
                materials.append(material)
            except (ValueError, KeyError) as e:
                print(f"Error loading material: {e}")
        
        # Handle adaptations
        adaptations_data = data_copy.pop('adaptations', [])
        adaptations = []
        for a_data in adaptations_data:
            try:
                adaptation = Adaptation(
                    type=AdaptationType.from_string(a_data['type']),
                    strength=a_data['strength'],
                    position=BodyPosition.from_string(a_data['position'])
                )
                adaptations.append(adaptation)
            except (ValueError, KeyError) as e:
                print(f"Error loading adaptation: {e}")
        
        # Handle interaction history
        interaction_history_data = data_copy.pop('interaction_history', [])
        interaction_history = []
        for record_data in interaction_history_data:
            try:
                record = IntegratedInteractionRecord.from_dict(record_data)
                interaction_history.append(record)
            except (ValueError, KeyError) as e:
                print(f"Error loading interaction record: {e}")
        
        # Handle facts learned
        facts_learned = set(data_copy.pop('facts_learned', []))
        
        # Create the integrated pet instance
        pet = cls(
            name=data_copy['name'],
            species=data_copy['species'],
            aura_color=data_copy['aura_color'],
            id=data_copy.get('id', str(uuid.uuid4()))
        )
        
        # Set core stats
        for stat in ['hunger', 'energy', 'happiness', 'iq', 'charisma', 'cleanliness', 'social']:
            if stat in data_copy:
                setattr(pet, stat, data_copy[stat])
        
        # Set critter attributes
        pet.base_animal = data_copy.get('base_animal')
        pet.materials = materials
        pet.adaptations = adaptations
        pet.facts_learned = facts_learned
        
        # Set personality traits
        pet.personality_traits = data_copy.get('personality_traits', pet.personality_traits)
        
        # Set timestamps
        pet.creation_timestamp = data_copy.get('creation_timestamp', pet.creation_timestamp)
        pet.last_active_timestamp = data_copy.get('last_active_timestamp', pet.last_active_timestamp)
        
        # Set history
        pet.interaction_history = interaction_history
        
        # Set progression
        pet.zoologist_level = data_copy.get('zoologist_level', 'novice')
        pet.critters_created = data_copy.get('critters_created', 0)
        pet.growth_rate = data_copy.get('growth_rate', 1.0)
        pet.maturity_level = data_copy.get('maturity_level', 0)
        
        # Set state tracking systems
        for system in ['job_states', 'battle_states', 'quest_states', 'education_states', 
                      'achievements', 'evolution', 'dna', 'genealogy']:
            if system in data_copy:
                setattr(pet, system, data_copy[system])
        
        return pet
    
    def to_pet(self) -> Pet:
        """
        Convert this integrated pet to a Pet instance.
        
        Returns:
            A Pet instance with data from this integrated pet
        """
        pet = Pet(
            name=self.name,
            species=self.species,
            aura_color=self.aura_color,
            id=self.id
        )
        
        # Set core stats
        pet.hunger = self.hunger
        pet.energy = self.energy
        pet.happiness = self.happiness
        pet.iq = self.iq
        pet.charisma = self.charisma
        pet.cleanliness = self.cleanliness
        pet.social = self.social
        
        # Set personality traits
        pet.personality_traits = self.personality_traits.copy()
        
        # Set timestamps
        pet.creation_timestamp = self.creation_timestamp
        pet.last_active_timestamp = self.last_active_timestamp
        
        # Convert interaction history
        for record in self.interaction_history:
            # Only include pet-related interactions
            if record.type.value in [t.value for t in PetInteractionType]:
                pet_record = PetInteractionRecord(
                    timestamp=record.timestamp,
                    type=PetInteractionType(record.type.value),
                    details=record.details
                )
                pet.interaction_history.append(pet_record)
        
        return pet
    
    def to_critter(self) -> Optional[Critter]:
        """
        Convert this integrated pet to a Critter instance.
        
        Returns:
            A Critter instance with data from this integrated pet, or None if no base animal is set
        """
        if not self.base_animal:
            return None
        
        critter = Critter(
            name=self.name,
            base_animal=self.base_animal,
            creator_name=self.name,  # Use pet name as creator name
            id=self.id
        )
        
        # Set critter attributes
        critter.materials = self.materials.copy()
        critter.adaptations = self.adaptations.copy()
        critter.facts_learned = self.facts_learned.copy()
        
        # Set timestamps
        critter.creation_timestamp = self.creation_timestamp
        critter.last_modified_timestamp = self.last_active_timestamp
        
        # Set zoologist level
        critter.zoologist_level = self.zoologist_level
        
        return critter
    
    @classmethod
    def from_pet(cls, pet: Pet) -> 'IntegratedPet':
        """
        Create an integrated pet from a Pet instance.
        
        Args:
            pet: The Pet instance to convert
            
        Returns:
            An IntegratedPet instance with data from the pet
        """
        integrated_pet = cls(
            name=pet.name,
            species=pet.species,
            aura_color=pet.aura_color,
            id=pet.id
        )
        
        # Set core stats
        integrated_pet.hunger = pet.hunger
        integrated_pet.energy = pet.energy
        integrated_pet.happiness = pet.happiness
        integrated_pet.iq = pet.iq
        integrated_pet.charisma = pet.charisma
        integrated_pet.cleanliness = pet.cleanliness
        integrated_pet.social = pet.social
        
        # Set personality traits
        integrated_pet.personality_traits = pet.personality_traits.copy()
        
        # Set timestamps
        integrated_pet.creation_timestamp = pet.creation_timestamp
        integrated_pet.last_active_timestamp = pet.last_active_timestamp
        
        # Convert interaction history
        for record in pet.interaction_history:
            integrated_record = IntegratedInteractionRecord(
                timestamp=record.timestamp,
                type=IntegratedInteractionType(record.type.value),
                details=record.details
            )
            integrated_pet.interaction_history.append(integrated_record)
        
        return integrated_pet
    
    @classmethod
    def from_pet_and_critter(cls, pet: Pet, critter: Critter) -> 'IntegratedPet':
        """
        Create an integrated pet from a Pet and a Critter instance.
        
        Args:
            pet: The Pet instance to convert
            critter: The Critter instance to convert
            
        Returns:
            An IntegratedPet instance with data from both the pet and critter
        """
        # Start with the pet data
        integrated_pet = cls.from_pet(pet)
        
        # Add critter data
        integrated_pet.base_animal = critter.base_animal
        integrated_pet.materials = critter.materials.copy()
        integrated_pet.adaptations = critter.adaptations.copy()
        integrated_pet.facts_learned = critter.facts_learned.copy()
        integrated_pet.zoologist_level = critter.zoologist_level
        
        return integrated_pet
    
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
        species_info = GenesisPetConfig.Archetypes.DEFINITIONS.get(self.species, {})
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

# --- Logic Manager ---
class IntegratedPetManager:
    """
    Handles all logic and state changes for an IntegratedPet instance.
    
    Following KISS principles:
    - K: Clear separation of responsibilities
    - I: Methods designed for easy updates
    - S: Systematized state management
    - S: Designed for user engagement
    """
    
    def __init__(self, pet: IntegratedPet):
        """Initialize with an integrated pet instance."""
        self.pet = pet
        self.max_stat = GenesisPetConfig.Core.MAX_STAT
        
        # Create pet and critter managers for delegating functionality
        self._pet_manager = self._create_pet_manager()
        self._critter_manager = self._create_critter_manager()
    
    def _create_pet_manager(self) -> PetLogicManager:
        """Create a PetLogicManager for handling pet-specific functionality."""
        pet = self.pet.to_pet()
        return PetLogicManager(pet)
    
    def _create_critter_manager(self) -> Optional[Critter]:
        """Create a Critter for handling critter-specific functionality."""
        return self.pet.to_critter()
    
    def _sync_from_pet_manager(self):
        """Sync data from the pet manager back to the integrated pet."""
        pet = self._pet_manager.pet
        
        # Sync core stats
        self.pet.hunger = pet.hunger
        self.pet.energy = pet.energy
        self.pet.happiness = pet.happiness
        self.pet.iq = pet.iq
        self.pet.charisma = pet.charisma
        self.pet.cleanliness = pet.cleanliness
        self.pet.social = pet.social
        
        # Sync last active timestamp
        self.pet.last_active_timestamp = pet.last_active_timestamp
    
    def _cap_stat(self, value: int) -> int:
        """Helper to cap stat values between 0 and MAX_STAT."""
        return max(0, min(value, self.max_stat))
    
    def _add_interaction(self, type: IntegratedInteractionType, details: Optional[str] = None):
        """Adds a new interaction record and prunes the history."""
        self.pet.interaction_history.append(
            IntegratedInteractionRecord(timestamp=time.time_ns(), type=type, details=details)
        )
        # Keep history to a reasonable size
        if len(self.pet.interaction_history) > 100:
            self.pet.interaction_history.pop(0)
    
    def feed(self) -> str:
        """
        Feeds the pet, restoring hunger.
        
        Returns:
            A message describing the result
        """
        # Delegate to pet manager
        result = self._pet_manager.feed()
        
        # Sync changes back to integrated pet
        self._sync_from_pet_manager()
        
        # Record the interaction
        self._add_interaction(IntegratedInteractionType.FEED, "Fed the pet")
        
        return result
    
    def play(self) -> str:
        """
        Plays with the pet, boosting happiness at the cost of energy.
        
        Returns:
            A message describing the result
        """
        # Delegate to pet manager
        result = self._pet_manager.play()
        
        # Sync changes back to integrated pet
        self._sync_from_pet_manager()
        
        # Record the interaction
        self._add_interaction(IntegratedInteractionType.PLAY, "Played with the pet")
        
        return result
    
    def groom(self) -> str:
        """
        Grooms the pet, improving cleanliness and happiness.
        
        Returns:
            A message describing the result
        """
        # Delegate to pet manager
        result = self._pet_manager.groom()
        
        # Sync changes back to integrated pet
        self._sync_from_pet_manager()
        
        # Record the interaction
        self._add_interaction(IntegratedInteractionType.GROOM, "Groomed the pet")
        
        return result
    
    def chat(self, message: str) -> str:
        """
        Handles a chat interaction, updating stats and returning a response.
        
        Args:
            message: The chat message from the user
            
        Returns:
            The pet's response
        """
        # Delegate to pet manager
        result = self._pet_manager.chat(message)
        
        # Sync changes back to integrated pet
        self._sync_from_pet_manager()
        
        # Record the interaction
        summary = message[:30] + '...' if len(message) > 30 else message
        self._add_interaction(IntegratedInteractionType.CHAT, f"Msg: '{summary}'")
        
        return result
    
    def add_material(self, material_type: str, color: str, coverage: float, position: str) -> bool:
        """
        Add a crafting material to the critter.
        
        Args:
            material_type: Type of material (e.g., "fur", "scales")
            color: Color of the material
            coverage: Percentage of critter covered (0.0 to 1.0)
            position: Where on the critter the material is applied
            
        Returns:
            True if successful, False otherwise
        """
        # Check if critter functionality is available
        if not self.pet.base_animal:
            return False
        
        # Create and validate the material
        try:
            material = CraftingMaterial(
                type=material_type,
                color=color,
                coverage=coverage,
                position=position
            )
            
            # Check if this material is allowed for the user's zoologist level
            zoologist_level_info = CritterCraftConfig.ZOOLOGIST_LEVELS.get(self.pet.zoologist_level, {})
            unlocked_materials = zoologist_level_info.get('unlocked_materials', [])
            
            if material.type.value not in unlocked_materials:
                return False
            
            # Add the material
            self.pet.materials.append(material)
            self.pet.last_modified_timestamp = time.time_ns()
            
            # Record the interaction
            self._add_interaction(
                IntegratedInteractionType.CRAFT, 
                f"Added {color} {material_type} to {position}"
            )
            
            return True
            
        except (ValueError, Exception) as e:
            print(f"Error adding material: {e}")
            return False
    
    def add_adaptation(self, adaptation_type: str, strength: int, position: str) -> bool:
        """
        Add an adaptation to the critter.
        
        Args:
            adaptation_type: Type of adaptation (e.g., "camouflage", "bioluminescence")
            strength: Effectiveness of the adaptation (1-10)
            position: Where on the critter the adaptation is applied
            
        Returns:
            True if successful, False otherwise
        """
        # Check if critter functionality is available
        if not self.pet.base_animal:
            return False
        
        # Create and validate the adaptation
        try:
            adaptation = Adaptation(
                type=adaptation_type,
                strength=strength,
                position=position
            )
            
            # Check if this adaptation is allowed for the user's zoologist level
            zoologist_level_info = CritterCraftConfig.ZOOLOGIST_LEVELS.get(self.pet.zoologist_level, {})
            unlocked_adaptations = zoologist_level_info.get('unlocked_adaptations', [])
            
            if adaptation.type.value not in unlocked_adaptations:
                return False
            
            # Add the adaptation
            self.pet.adaptations.append(adaptation)
            self.pet.last_modified_timestamp = time.time_ns()
            
            # Record the interaction
            self._add_interaction(
                IntegratedInteractionType.CRAFT, 
                f"Added {adaptation_type} (strength {strength}) to {position}"
            )
            
            return True
            
        except (ValueError, Exception) as e:
            print(f"Error adding adaptation: {e}")
            return False
    
    def learn_fact(self, fact: str) -> bool:
        """
        Record that the user has learned a fact.
        
        Args:
            fact: The fact that was learned
            
        Returns:
            True if this is a new fact, False if already known
        """
        # Normalize the fact to avoid duplicates with different capitalization/spacing
        normalized_fact = ' '.join(fact.lower().split())
        
        if normalized_fact in self.pet.facts_learned:
            return False
            
        # Add the fact
        self.pet.facts_learned.add(normalized_fact)
        self.pet.last_modified_timestamp = time.time_ns()
        
        # Record the interaction
        self._add_interaction(
            IntegratedInteractionType.LEARN, 
            f"Learned: {fact[:50]}..." if len(fact) > 50 else f"Learned: {fact}"
        )
        
        # Check if learning this fact should trigger a zoologist level up
        self._check_for_zoologist_level_up()
        
        # Learning facts also increases IQ
        self.pet.iq = self._cap_stat(self.pet.iq + 1)
        
        return True
    
    def _check_for_zoologist_level_up(self) -> bool:
        """
        Check if the user has met the requirements for a zoologist level up.
        
        Returns:
            True if leveled up, False otherwise
        """
        # Get the next level
        current_level_index = list(CritterCraftConfig.ZOOLOGIST_LEVELS.keys()).index(self.pet.zoologist_level)
        if current_level_index >= len(CritterCraftConfig.ZOOLOGIST_LEVELS) - 1:
            return False  # Already at max level
            
        next_level = list(CritterCraftConfig.ZOOLOGIST_LEVELS.keys())[current_level_index + 1]
        next_level_info = CritterCraftConfig.ZOOLOGIST_LEVELS[next_level]
        
        # Check if requirements are met
        if len(self.pet.facts_learned) >= next_level_info.get('required_facts', 999):
            self.pet.zoologist_level = next_level
            self.pet.last_modified_timestamp = time.time_ns()
            
            # Record the interaction
            self._add_interaction(
                IntegratedInteractionType.LEVEL_UP, 
                f"Advanced to zoologist level: {next_level}"
            )
            
            return True
            
        return False
    
    def simulate_in_environment(self, environment: str) -> Dict[str, Any]:
        """
        Simulate how the critter would perform in a given environment.
        
        Args:
            environment: The environment to simulate (e.g., "forest", "ocean", "desert")
            
        Returns:
            Dictionary with simulation results
        """
        # Check if critter functionality is available
        if not self.pet.base_animal or not self._critter_manager:
            raise ValueError("Critter functionality not available. Set a base animal first.")
        
        # Create a temporary critter with current data
        critter = self.pet.to_critter()
        
        # Perform the simulation
        results = critter.simulate_in_environment(environment)
        
        # If the simulation added any facts, sync them back
        if len(critter.facts_learned) > len(self.pet.facts_learned):
            new_facts = critter.facts_learned - self.pet.facts_learned
            for fact in new_facts:
                self.learn_fact(fact)
        
        # Record the interaction
        self._add_interaction(
            IntegratedInteractionType.LEARN, 
            f"Simulated in {environment} environment"
        )
        
        return results
    
    def tick(self, current_time_ns: Optional[int] = None):
        """
        Simulates the passage of time, decaying stats and calculating offline progress.
        
        Args:
            current_time_ns: Current time in nanoseconds, or None to use current time
        """
        # Delegate to pet manager
        self._pet_manager.tick(current_time_ns)
        
        # Sync changes back to integrated pet
        self._sync_from_pet_manager()
        
        # Record the interaction
        self._add_interaction(IntegratedInteractionType.TICK_DECAY, "Applied stat decay")
    
    def get_status_report(self) -> str:
        """
        Returns a formatted string summary of the integrated pet's current status.
        
        Returns:
            A formatted status report string
        """
        # Get species display name
        species_info = GenesisPetConfig.Archetypes.DEFINITIONS.get(self.pet.species, {})
        species_display = species_info.get('display_name', self.pet.species)
        
        # Get aura display name
        aura_info = GenesisPetConfig.Auras.DEFINITIONS.get(self.pet.aura_color, {})
        aura_display = aura_info.get('display_name', self.pet.aura_color)
        
        # Calculate age
        age_days = self.pet.calculate_age_days()
        human_age = self.pet.calculate_human_age_equivalent()
        
        # Format the status report
        report = [
            f"=== {self.pet.name}'s Status Report ===",
            f"ID: {self.pet.id}",
            f"Species: {species_display}",
            f"Aura: {aura_display}",
            f"Age: {age_days:.1f} days ({human_age} in human years)",
            f"Maturity: {self.pet.maturity_level}%",
            f"",
            f"--- Core Stats ---",
            f"Hunger: {self.pet.hunger}/{self.max_stat}",
            f"Energy: {self.pet.energy}/{self.max_stat}",
            f"Happiness: {self.pet.happiness}/{self.max_stat}",
            f"Intelligence: {self.pet.iq}/{self.max_stat}",
            f"Charisma: {self.pet.charisma}/{self.max_stat}",
            f"Cleanliness: {self.pet.cleanliness}/{self.max_stat}",
            f"Social: {self.pet.social}/{self.max_stat}",
            f""
        ]
        
        # Add critter information if available
        if self.pet.base_animal:
            base_animal_info = CritterCraftConfig.CRITTER_TYPES.get(self.pet.base_animal, {})
            base_animal_display = base_animal_info.get('display_name', self.pet.base_animal)
            
            report.extend([
                f"--- Critter Information ---",
                f"Base Animal: {base_animal_display}",
                f"Zoologist Level: {self.pet.zoologist_level.capitalize()}",
                f"Facts Learned: {len(self.pet.facts_learned)}",
                f"Materials: {len(self.pet.materials)}",
                f"Adaptations: {len(self.pet.adaptations)}",
                f""
            ])
        
        # Add personality traits
        report.append(f"--- Personality Traits ---")
        for trait, value in self.pet.personality_traits.items():
            report.append(f"{trait.capitalize()}: {value}")
        
        return "\n".join(report)

# --- Persistence Functions ---
class IntegratedPetPersistence:
    """
    Handles saving and loading integrated pets to/from storage.
    
    Following KISS principles:
    - K: Clear, focused responsibility (persistence only)
    - I: Easy to extend with new storage methods
    - S: Systematized error handling
    - S: Designed for data integrity
    """
    
    @staticmethod
    def save_to_file(pet: IntegratedPet, filename: str) -> bool:
        """
        Saves an integrated pet's state to a JSON file.
        
        Args:
            pet: The IntegratedPet instance to save
            filename: Path to the file
            
        Returns:
            True if successful, False otherwise
        """
        try:
            with open(filename, 'w') as f:
                json.dump(pet.to_dict(), f, indent=4)
            return True
        except (IOError, TypeError) as e:
            print(f"Error saving integrated pet to {filename}: {e}")
            return False

    @staticmethod
    def load_from_file(filename: str) -> Optional[IntegratedPet]:
        """
        Loads an integrated pet's state from a JSON file.
        
        Args:
            filename: Path to the file
            
        Returns:
            An IntegratedPet instance if successful, None otherwise
        """
        try:
            with open(filename, 'r') as f:
                data = json.load(f)
            return IntegratedPet.from_dict(data)
        except (IOError, json.JSONDecodeError, KeyError) as e:
            print(f"Error loading integrated pet from {filename}: {e}")
            return None
            
    @staticmethod
    def backup_pet(pet: IntegratedPet, backup_dir: str) -> bool:
        """
        Creates a timestamped backup of an integrated pet.
        
        Args:
            pet: The IntegratedPet instance to backup
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
            return IntegratedPetPersistence.save_to_file(pet, filename)
        except Exception as e:
            print(f"Error creating backup: {e}")
            return False
    
    @staticmethod
    def convert_pet_to_integrated(pet_filename: str, output_filename: str) -> bool:
        """
        Converts a Pet file to an IntegratedPet file.
        
        Args:
            pet_filename: Path to the Pet file
            output_filename: Path to save the IntegratedPet file
            
        Returns:
            True if successful, False otherwise
        """
        try:
            # Load the pet
            pet = PetPersistence.load_from_file(pet_filename)
            if not pet:
                return False
            
            # Convert to integrated pet
            integrated_pet = IntegratedPet.from_pet(pet)
            
            # Save the integrated pet
            return IntegratedPetPersistence.save_to_file(integrated_pet, output_filename)
        except Exception as e:
            print(f"Error converting pet to integrated pet: {e}")
            return False
    
    @staticmethod
    def convert_pet_and_critter_to_integrated(pet_filename: str, critter_filename: str, output_filename: str) -> bool:
        """
        Converts a Pet file and a Critter file to an IntegratedPet file.
        
        Args:
            pet_filename: Path to the Pet file
            critter_filename: Path to the Critter file
            output_filename: Path to save the IntegratedPet file
            
        Returns:
            True if successful, False otherwise
        """
        try:
            # Load the pet
            pet = PetPersistence.load_from_file(pet_filename)
            if not pet:
                return False
            
            # Load the critter
            critter = CritterPersistence.load_from_file(critter_filename)
            if not critter:
                return False
            
            # Convert to integrated pet
            integrated_pet = IntegratedPet.from_pet_and_critter(pet, critter)
            
            # Save the integrated pet
            return IntegratedPetPersistence.save_to_file(integrated_pet, output_filename)
        except Exception as e:
            print(f"Error converting pet and critter to integrated pet: {e}")
            return False