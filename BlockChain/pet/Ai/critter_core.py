# pet/critter_core.py
"""
Core functionality for the Critter-Craft: Nurture Your Inner Zoologist application.

This module defines the Critter class and related functionality following the KISS principles:
- K (Know Your Core, Keep it Clear): Clear separation of data models and logic
- I (Iterate Intelligently): Structured for easy updates and maintenance
- S (Systematize for Scalability): Modular design with clear interfaces
- S (Sense the Landscape & Stimulate Engagement): Designed for user engagement
"""

import json
import time
import uuid
from typing import Dict, Any, List, Optional, Set
from dataclasses import dataclass, field
from enum import Enum, auto

# Import constants from the centralized config file
from config import (
    CritterCraftConfig
)

# --- Enums for Type Safety and Readability ---
class MaterialType(Enum):
    """Types of materials that can be used in crafting."""
    FUR = "fur"
    SCALES = "scales"
    FEATHERS = "feathers"
    SKIN = "skin"
    SHELL = "shell"
    EXOSKELETON = "exoskeleton"
    
    @classmethod
    def from_string(cls, value: str) -> 'MaterialType':
        """Convert string to enum value, case-insensitive."""
        try:
            return cls(value.lower())
        except ValueError:
            valid_values = [m.value for m in cls]
            raise ValueError(f"Invalid material type: {value}. Valid values: {valid_values}")

class AdaptationType(Enum):
    """Types of adaptations that can be applied to critters."""
    CAMOUFLAGE = "camouflage"
    BIOLUMINESCENCE = "bioluminescence"
    ECHOLOCATION = "echolocation"
    MIMICRY = "mimicry"
    HIBERNATION = "hibernation"
    VENOM = "venom"
    REGENERATION = "regeneration"
    INFRARED_VISION = "infrared_vision"
    
    @classmethod
    def from_string(cls, value: str) -> 'AdaptationType':
        """Convert string to enum value, case-insensitive."""
        try:
            return cls(value.lower())
        except ValueError:
            valid_values = [a.value for a in cls]
            raise ValueError(f"Invalid adaptation type: {value}. Valid values: {valid_values}")

class BodyPosition(Enum):
    """Positions on a critter's body where materials/adaptations can be applied."""
    BODY = "body"
    HEAD = "head"
    LIMBS = "limbs"
    TAIL = "tail"
    WINGS = "wings"
    FINS = "fins"
    EYES = "eyes"
    
    @classmethod
    def from_string(cls, value: str) -> 'BodyPosition':
        """Convert string to enum value, case-insensitive."""
        try:
            return cls(value.lower())
        except ValueError:
            valid_values = [p.value for p in cls]
            raise ValueError(f"Invalid body position: {value}. Valid values: {valid_values}")

class Environment(Enum):
    """Types of environments for simulation."""
    FOREST = "forest"
    OCEAN = "ocean"
    DESERT = "desert"
    ARCTIC = "arctic"
    GRASSLAND = "grassland"
    CAVE = "cave"
    MOUNTAIN = "mountain"
    
    @classmethod
    def from_string(cls, value: str) -> 'Environment':
        """Convert string to enum value, case-insensitive."""
        try:
            return cls(value.lower())
        except ValueError:
            valid_values = [e.value for e in cls]
            raise ValueError(f"Invalid environment: {value}. Valid values: {valid_values}")

@dataclass
class CraftingMaterial:
    """
    Represents a material used in crafting a critter.
    
    Following KISS principles:
    - K: Clear, focused data model
    - I: Easy to update with new attributes
    - S: Type-safe with enums
    """
    type: MaterialType    # Type of material (e.g., fur, scales)
    color: str            # Color of the material
    coverage: float       # 0.0 to 1.0, representing percentage of critter covered
    position: BodyPosition # Where on the critter the material is applied
    
    def __post_init__(self):
        """Validate the material attributes."""
        # Convert string to enum if needed
        if isinstance(self.type, str):
            self.type = MaterialType.from_string(self.type)
            
        if isinstance(self.position, str):
            self.position = BodyPosition.from_string(self.position)
            
        # Validate coverage
        if not 0.0 <= self.coverage <= 1.0:
            raise ValueError("Coverage must be between 0.0 and 1.0")
            
        # Validate color against available colors for this material
        material_config = CritterCraftConfig.CRAFTING_MATERIALS.get(self.type.value, {})
        valid_colors = material_config.get('colors', [])
        
        if valid_colors and self.color.lower() not in [c.lower() for c in valid_colors]:
            raise ValueError(f"Invalid color '{self.color}' for {self.type.value}. Valid colors: {valid_colors}")

@dataclass
class Adaptation:
    """
    Represents an adaptation applied to a critter.
    
    Following KISS principles:
    - K: Clear, focused data model
    - I: Easy to update with new attributes
    - S: Type-safe with enums
    """
    type: AdaptationType  # Type of adaptation (e.g., camouflage, bioluminescence)
    strength: int         # 1-10, representing effectiveness
    position: BodyPosition # Where on the critter the adaptation is applied
    
    def __post_init__(self):
        """Validate the adaptation attributes."""
        # Convert string to enum if needed
        if isinstance(self.type, str):
            self.type = AdaptationType.from_string(self.type)
            
        if isinstance(self.position, str):
            self.position = BodyPosition.from_string(self.position)
            
        # Validate strength
        if not 1 <= self.strength <= 10:
            raise ValueError("Strength must be between 1 and 10")

# --- Custom Exceptions for Clarity ---
class CritterError(Exception):
    """Base exception for critter-related errors."""
    pass

class CritterInitializationError(CritterError):
    """Raised when a critter cannot be initialized due to invalid parameters."""
    pass

class CraftingError(CritterError):
    """Raised when a crafting operation fails."""
    pass

@dataclass
class Critter:
    """
    Represents a crafted critter in the Critter-Craft application.
    This is the core data model for the crafted creatures.
    
    Following KISS principles:
    - K: Clear separation of data and validation
    - I: Structured for easy updates
    - S: Systematized with type-safe enums
    - S: Designed for educational engagement
    """
    name: str
    base_animal: str   # e.g., 'chameleon', 'anglerfish' - maps to CRITTER_TYPES
    creator_name: str  # Name of the user who created this critter
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    
    # Crafting elements
    materials: List[CraftingMaterial] = field(default_factory=list)
    adaptations: List[Adaptation] = field(default_factory=list)
    
    # Metadata
    creation_timestamp: int = field(default_factory=lambda: time.time_ns())
    last_modified_timestamp: int = field(default_factory=lambda: time.time_ns())
    
    # Educational progress
    facts_learned: Set[str] = field(default_factory=set)
    
    # Zoologist progression
    zoologist_level: str = 'novice'
    
    def __post_init__(self):
        """Perform post-initialization validation."""
        # Validate name
        self.name = self.name.strip()
        if not self.name or len(self.name) > 30 or not self.name.isprintable():
            raise CritterInitializationError("Critter name must be 1-30 printable characters.")
            
        # Validate base animal
        if self.base_animal not in CritterCraftConfig.CRITTER_TYPES:
            valid_types = list(CritterCraftConfig.CRITTER_TYPES.keys())
            raise CritterInitializationError(
                f"Invalid base animal: {self.base_animal}. Valid types: {valid_types}"
            )
            
        # Validate creator name
        self.creator_name = self.creator_name.strip()
        if not self.creator_name or len(self.creator_name) > 30:
            raise CritterInitializationError("Creator name must be 1-30 characters.")
            
    def to_dict(self) -> Dict[str, Any]:
        """
        Serialize the Critter object to a dictionary.
        
        Returns:
            Dictionary representation of the critter
        """
        data = {
            'name': self.name,
            'base_animal': self.base_animal,
            'creator_name': self.creator_name,
            'id': self.id,
            'creation_timestamp': self.creation_timestamp,
            'last_modified_timestamp': self.last_modified_timestamp,
            'facts_learned': list(self.facts_learned),
            'zoologist_level': self.zoologist_level,
            'materials': [
                {
                    'type': m.type.value,
                    'color': m.color,
                    'coverage': m.coverage,
                    'position': m.position.value
                } for m in self.materials
            ],
            'adaptations': [
                {
                    'type': a.type.value,
                    'strength': a.strength,
                    'position': a.position.value
                } for a in self.adaptations
            ]
        }
        return data
        
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'Critter':
        """
        Deserialize a Critter object from a dictionary.
        
        Args:
            data: Dictionary representation of a critter
            
        Returns:
            A Critter instance
        """
        # Create a copy to avoid modifying the input
        data_copy = data.copy()
        
        # Handle materials
        materials_data = data_copy.pop('materials', [])
        materials = [
            CraftingMaterial(
                type=MaterialType.from_string(m['type']),
                color=m['color'],
                coverage=m['coverage'],
                position=BodyPosition.from_string(m['position'])
            ) for m in materials_data
        ]
        
        # Handle adaptations
        adaptations_data = data_copy.pop('adaptations', [])
        adaptations = [
            Adaptation(
                type=AdaptationType.from_string(a['type']),
                strength=a['strength'],
                position=BodyPosition.from_string(a['position'])
            ) for a in adaptations_data
        ]
        
        # Handle facts learned
        facts_learned = set(data_copy.pop('facts_learned', []))
        
        # Create the critter
        critter = cls(
            name=data_copy['name'],
            base_animal=data_copy['base_animal'],
            creator_name=data_copy['creator_name'],
            id=data_copy.get('id', str(uuid.uuid4())),
            zoologist_level=data_copy.get('zoologist_level', 'novice')
        )
        
        # Set timestamps
        critter.creation_timestamp = data_copy.get('creation_timestamp', time.time_ns())
        critter.last_modified_timestamp = data_copy.get('last_modified_timestamp', time.time_ns())
        
        # Set collections
        critter.materials = materials
        critter.adaptations = adaptations
        critter.facts_learned = facts_learned
        
        return critter
    
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
            
        Raises:
            CraftingError: If the material parameters are invalid
        """
        try:
            # Create and validate the material
            material = CraftingMaterial(
                type=material_type,
                color=color,
                coverage=coverage,
                position=position
            )
            
            # Check if this material is allowed for the user's zoologist level
            zoologist_level_info = CritterCraftConfig.ZOOLOGIST_LEVELS.get(self.zoologist_level, {})
            unlocked_materials = zoologist_level_info.get('unlocked_materials', [])
            
            if material.type.value not in unlocked_materials:
                raise CraftingError(
                    f"Material '{material.type.value}' is not unlocked for zoologist level '{self.zoologist_level}'"
                )
            
            # Add the material
            self.materials.append(material)
            self.last_modified_timestamp = time.time_ns()
            return True
            
        except (ValueError, CraftingError) as e:
            # Log the error for debugging
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
            
        Raises:
            CraftingError: If the adaptation parameters are invalid
        """
        try:
            # Create and validate the adaptation
            adaptation = Adaptation(
                type=adaptation_type,
                strength=strength,
                position=position
            )
            
            # Check if this adaptation is allowed for the user's zoologist level
            zoologist_level_info = CritterCraftConfig.ZOOLOGIST_LEVELS.get(self.zoologist_level, {})
            unlocked_adaptations = zoologist_level_info.get('unlocked_adaptations', [])
            
            if adaptation.type.value not in unlocked_adaptations:
                raise CraftingError(
                    f"Adaptation '{adaptation.type.value}' is not unlocked for zoologist level '{self.zoologist_level}'"
                )
            
            # Add the adaptation
            self.adaptations.append(adaptation)
            self.last_modified_timestamp = time.time_ns()
            return True
            
        except (ValueError, CraftingError) as e:
            # Log the error for debugging
            print(f"Error adding adaptation: {e}")
            return False
    
    def learn_fact(self, fact: str) -> bool:
        """
        Record that the user has learned a fact about this animal.
        
        Args:
            fact: The fact that was learned
            
        Returns:
            True if this is a new fact, False if already known
        """
        # Normalize the fact to avoid duplicates with different capitalization/spacing
        normalized_fact = ' '.join(fact.lower().split())
        
        if normalized_fact in self.facts_learned:
            return False
            
        self.facts_learned.add(normalized_fact)
        self.last_modified_timestamp = time.time_ns()
        
        # Check if learning this fact should trigger a zoologist level up
        self._check_for_zoologist_level_up()
        
        return True
    
    def _check_for_zoologist_level_up(self) -> bool:
        """
        Check if the user has met the requirements for a zoologist level up.
        
        Returns:
            True if leveled up, False otherwise
        """
        # Get the next level
        current_level_index = list(CritterCraftConfig.ZOOLOGIST_LEVELS.keys()).index(self.zoologist_level)
        if current_level_index >= len(CritterCraftConfig.ZOOLOGIST_LEVELS) - 1:
            return False  # Already at max level
            
        next_level = list(CritterCraftConfig.ZOOLOGIST_LEVELS.keys())[current_level_index + 1]
        next_level_info = CritterCraftConfig.ZOOLOGIST_LEVELS[next_level]
        
        # Check if requirements are met
        if len(self.facts_learned) >= next_level_info.get('required_facts', 999):
            self.zoologist_level = next_level
            self.last_modified_timestamp = time.time_ns()
            return True
            
        return False
    
    def get_adaptation_effectiveness(self, adaptation_type: str) -> int:
        """
        Calculate the overall effectiveness of a specific adaptation.
        
        Args:
            adaptation_type: The type of adaptation to evaluate
            
        Returns:
            Effectiveness score (0 if not present)
        """
        try:
            # Convert string to enum if needed
            if isinstance(adaptation_type, str):
                adaptation_type = AdaptationType.from_string(adaptation_type)
                
            # Find matching adaptations
            matching_adaptations = [a for a in self.adaptations if a.type == adaptation_type]
            if not matching_adaptations:
                return 0
                
            # Sum up the strengths
            return sum(a.strength for a in matching_adaptations)
            
        except ValueError:
            # Invalid adaptation type
            return 0
    
    def simulate_in_environment(self, environment: str) -> Dict[str, Any]:
        """
        Simulate how the critter would perform in a given environment.
        
        Args:
            environment: The environment to simulate (e.g., "forest", "ocean", "desert")
            
        Returns:
            Dictionary with simulation results
            
        Raises:
            ValueError: If the environment is invalid
        """
        try:
            # Convert string to enum if needed
            if isinstance(environment, str):
                env = Environment.from_string(environment)
            else:
                env = environment
                
            # Initialize results
            results = {
                'environment': env.value,
                'survival_score': 50,  # Base score
                'advantages': [],
                'disadvantages': []
            }
            
            # Get animal info from config
            animal_info = CritterCraftConfig.CRITTER_TYPES.get(self.base_animal, {})
            natural_habitat = animal_info.get('habitat', '').lower()
            display_name = animal_info.get('display_name', self.base_animal)
            
            # --- Habitat Match Evaluation ---
            habitat_match = self._evaluate_habitat_match(env.value, natural_habitat)
            
            if habitat_match:
                results['survival_score'] += 20
                results['advantages'].append(
                    f"Natural habitat match: {display_name} are naturally adapted to {env.value}-like environments"
                )
            else:
                results['disadvantages'].append(
                    f"Habitat mismatch: {display_name} are not naturally adapted to {env.value} environments"
                )
            
            # --- Adaptation Evaluations ---
            self._evaluate_adaptations(results, env.value)
            
            # --- Material Evaluations ---
            self._evaluate_materials(results, env.value)
            
            # Cap survival score between 0 and 100
            results['survival_score'] = max(0, min(100, results['survival_score']))
            
            # Add educational facts based on the simulation
            self._add_educational_facts(results)
            
            return results
            
        except ValueError as e:
            # Convert to a more user-friendly error
            valid_environments = [e.value for e in Environment]
            raise ValueError(f"Invalid environment: {environment}. Must be one of: {valid_environments}")
    
    def _evaluate_habitat_match(self, environment: str, natural_habitat: str) -> bool:
        """
        Evaluate if the environment matches the critter's natural habitat.
        
        Args:
            environment: The environment to evaluate
            natural_habitat: The critter's natural habitat
            
        Returns:
            True if there's a match, False otherwise
        """
        habitat_keywords = {
            'forest': ['forest', 'jungle', 'woodland', 'tropical'],
            'ocean': ['ocean', 'sea', 'marine', 'aquatic', 'water'],
            'desert': ['desert', 'arid', 'dry', 'sand'],
            'arctic': ['arctic', 'polar', 'tundra', 'cold', 'ice'],
            'grassland': ['grassland', 'savanna', 'prairie', 'plain'],
            'cave': ['cave', 'cavern', 'underground'],
            'mountain': ['mountain', 'alpine', 'highland', 'hill']
        }
        
        # Check if any keywords for this environment appear in the natural habitat
        return any(keyword in natural_habitat for keyword in habitat_keywords.get(environment, []))
    
    def _evaluate_adaptations(self, results: Dict[str, Any], environment: str):
        """
        Evaluate how the critter's adaptations affect its survival in the environment.
        
        Args:
            results: The results dictionary to update
            environment: The environment being simulated
        """
        # --- Camouflage ---
        self._evaluate_camouflage(results, environment)
        
        # --- Bioluminescence ---
        self._evaluate_bioluminescence(results, environment)
        
        # --- Echolocation ---
        self._evaluate_echolocation(results, environment)
        
        # --- Mimicry ---
        self._evaluate_mimicry(results, environment)
        
        # --- Hibernation ---
        self._evaluate_hibernation(results, environment)
        
        # --- Venom ---
        self._evaluate_venom(results, environment)
        
        # --- Regeneration ---
        self._evaluate_regeneration(results, environment)
        
        # --- Infrared Vision ---
        self._evaluate_infrared_vision(results, environment)
    
    def _evaluate_camouflage(self, results: Dict[str, Any], environment: str):
        """Evaluate camouflage adaptation in the environment."""
        camouflage_score = self.get_adaptation_effectiveness(AdaptationType.CAMOUFLAGE)
        if camouflage_score <= 0:
            return
            
        if environment == "forest":
            results['survival_score'] += camouflage_score * 2
            results['advantages'].append("Camouflage is highly effective in forests with varied vegetation")
        elif environment == "desert":
            results['survival_score'] += camouflage_score * 0.7
            results['disadvantages'].append("Limited hiding spots in deserts make camouflage less effective")
        elif environment == "ocean":
            results['survival_score'] += camouflage_score * 1.2
            results['advantages'].append("Ocean environments offer good camouflage opportunities")
        elif environment == "arctic":
            # Check if the critter has white materials
            has_white = any(m.color.lower() == "white" for m in self.materials)
            if has_white:
                results['survival_score'] += camouflage_score * 2.5
                results['advantages'].append("White coloration provides excellent camouflage in arctic environments")
            else:
                results['survival_score'] -= camouflage_score * 0.5
                results['disadvantages'].append("Non-white coloration stands out in arctic environments")
        elif environment == "grassland":
            results['survival_score'] += camouflage_score * 1.5
            results['advantages'].append("Grasslands offer decent camouflage opportunities")
    
    def _evaluate_bioluminescence(self, results: Dict[str, Any], environment: str):
        """Evaluate bioluminescence adaptation in the environment."""
        bioluminescence_score = self.get_adaptation_effectiveness(AdaptationType.BIOLUMINESCENCE)
        if bioluminescence_score <= 0:
            return
            
        if environment == "ocean":
            results['survival_score'] += bioluminescence_score * 3
            results['advantages'].append("Bioluminescence is extremely useful in deep ocean environments for attracting prey and finding mates")
        elif environment == "forest":
            results['survival_score'] += bioluminescence_score * 0.8
            results['advantages'].append("Bioluminescence has limited use in forests, mainly at night for attracting mates")
        else:
            results['survival_score'] += bioluminescence_score * 0.5
            results['disadvantages'].append(f"Bioluminescence has limited utility in {environment} environments")
    
    def _evaluate_echolocation(self, results: Dict[str, Any], environment: str):
        """Evaluate echolocation adaptation in the environment."""
        echolocation_score = self.get_adaptation_effectiveness(AdaptationType.ECHOLOCATION)
        if echolocation_score <= 0:
            return
            
        if environment == "ocean":
            results['survival_score'] += echolocation_score * 2.5
            results['advantages'].append("Echolocation works exceptionally well underwater for navigation and hunting")
        elif environment == "forest":
            results['survival_score'] += echolocation_score * 2
            results['advantages'].append("Dense forests benefit from echolocation for navigation in low visibility")
        elif environment == "cave":
            results['survival_score'] += echolocation_score * 3
            results['advantages'].append("Echolocation is essential for cave navigation")
        else:
            results['survival_score'] += echolocation_score * 1
            results['advantages'].append(f"Echolocation provides moderate benefits in {environment} environments")
    
    def _evaluate_mimicry(self, results: Dict[str, Any], environment: str):
        """Evaluate mimicry adaptation in the environment."""
        mimicry_score = self.get_adaptation_effectiveness(AdaptationType.MIMICRY)
        if mimicry_score <= 0:
            return
            
        # Mimicry is generally useful everywhere
        results['survival_score'] += mimicry_score * 1.5
        results['advantages'].append("Mimicry provides protection from predators in most environments")
    
    def _evaluate_hibernation(self, results: Dict[str, Any], environment: str):
        """Evaluate hibernation adaptation in the environment."""
        hibernation_score = self.get_adaptation_effectiveness(AdaptationType.HIBERNATION)
        if hibernation_score <= 0:
            return
            
        if environment == "arctic":
            results['survival_score'] += hibernation_score * 3
            results['advantages'].append("Hibernation is crucial for surviving harsh arctic winters with limited food")
        elif environment == "desert":
            results['survival_score'] += hibernation_score * 2
            results['advantages'].append("Hibernation helps survive extreme desert conditions and food scarcity")
        else:
            results['survival_score'] += hibernation_score * 1
            results['advantages'].append(f"Hibernation provides moderate benefits in {environment} environments")
    
    def _evaluate_venom(self, results: Dict[str, Any], environment: str):
        """Evaluate venom adaptation in the environment."""
        venom_score = self.get_adaptation_effectiveness(AdaptationType.VENOM)
        if venom_score <= 0:
            return
            
        # Venom is useful in most environments
        results['survival_score'] += venom_score * 1.8
        results['advantages'].append("Venom provides a significant advantage for hunting and defense")
    
    def _evaluate_regeneration(self, results: Dict[str, Any], environment: str):
        """Evaluate regeneration adaptation in the environment."""
        regeneration_score = self.get_adaptation_effectiveness(AdaptationType.REGENERATION)
        if regeneration_score <= 0:
            return
            
        # Regeneration is useful in all environments
        results['survival_score'] += regeneration_score * 1.5
        results['advantages'].append("Regeneration allows recovery from injuries, increasing survival chances")
    
    def _evaluate_infrared_vision(self, results: Dict[str, Any], environment: str):
        """Evaluate infrared vision adaptation in the environment."""
        infrared_score = self.get_adaptation_effectiveness(AdaptationType.INFRARED_VISION)
        if infrared_score <= 0:
            return
            
        if environment == "forest" or environment == "grassland":
            results['survival_score'] += infrared_score * 2
            results['advantages'].append("Infrared vision is excellent for detecting warm-blooded prey in vegetation")
        elif environment == "desert":
            results['survival_score'] += infrared_score * 1.5
            results['advantages'].append("Infrared vision helps locate prey hiding from the heat")
        elif environment == "arctic":
            results['survival_score'] += infrared_score * 3
            results['advantages'].append("Infrared vision is invaluable for spotting warm-blooded animals against the cold background")
        else:
            results['survival_score'] += infrared_score * 1
            results['advantages'].append(f"Infrared vision provides some benefits in {environment} environments")
    
    def _evaluate_materials(self, results: Dict[str, Any], environment: str):
        """
        Evaluate how the critter's materials affect its survival in the environment.
        
        Args:
            results: The results dictionary to update
            environment: The environment being simulated
        """
        # Check for fur in arctic environments
        fur_materials = [m for m in self.materials if m.type == MaterialType.FUR]
        if fur_materials and environment == "arctic":
            fur_coverage = sum(m.coverage for m in fur_materials)
            if fur_coverage > 0.5:
                results['survival_score'] += 15
                results['advantages'].append("Thick fur provides excellent insulation in cold arctic environments")
        
        # Check for scales in desert environments
        scales_materials = [m for m in self.materials if m.type == MaterialType.SCALES]
        if scales_materials and environment == "desert":
            scales_coverage = sum(m.coverage for m in scales_materials)
            if scales_coverage > 0.5:
                results['survival_score'] += 12
                results['advantages'].append("Scales provide good protection from the harsh desert sun and heat")
        
        # Check for feathers in various environments
        feathers_materials = [m for m in self.materials if m.type == MaterialType.FEATHERS]
        if feathers_materials:
            feathers_coverage = sum(m.coverage for m in feathers_materials)
            if feathers_coverage > 0.3:
                if environment == "forest" or environment == "grassland":
                    results['survival_score'] += 10
                    results['advantages'].append("Feathers provide good insulation and mobility in varied terrain")
                elif environment == "arctic":
                    results['survival_score'] += 8
                    results['advantages'].append("Feathers provide some insulation against the cold")
        
        # Check for exoskeleton in various environments
        exoskeleton_materials = [m for m in self.materials if m.type == MaterialType.EXOSKELETON]
        if exoskeleton_materials:
            exoskeleton_coverage = sum(m.coverage for m in exoskeleton_materials)
            if exoskeleton_coverage > 0.4:
                results['survival_score'] += 7
                results['advantages'].append("Exoskeleton provides general protection")
    
    def _add_educational_facts(self, results: Dict[str, Any]):
        """
        Add educational facts to the simulation results.
        
        Args:
            results: The results dictionary to update
        """
        # Add a fact about the environment
        environment_facts = {
            "forest": "Forests are complex ecosystems with multiple layers of vegetation, providing diverse habitats.",
            "ocean": "Oceans cover more than 70% of Earth's surface and contain 97% of Earth's water.",
            "desert": "Deserts receive less than 10 inches of precipitation annually and experience extreme temperature variations.",
            "arctic": "The Arctic is characterized by permafrost, where the ground remains frozen throughout the year.",
            "grassland": "Grasslands are dominated by grasses rather than trees and are home to many grazing animals."
        }
        
        if results['environment'] in environment_facts:
            fact = environment_facts[results['environment']]
            results['educational_fact'] = fact
            self.learn_fact(fact)

# --- Persistence Functions ---
class CritterPersistence:
    """
    Handles saving and loading critters to/from storage.
    
    Following KISS principles:
    - K: Clear, focused responsibility (persistence only)
    - I: Easy to extend with new storage methods
    - S: Systematized error handling
    - S: Designed for data integrity
    """
    
    @staticmethod
    def save_to_file(critter: Critter, filename: str) -> bool:
        """
        Saves a critter's state to a JSON file.
        
        Args:
            critter: The Critter instance to save
            filename: Path to the file
            
        Returns:
            True if successful, False otherwise
        """
        try:
            with open(filename, 'w') as f:
                json.dump(critter.to_dict(), f, indent=4)
            return True
        except (IOError, TypeError) as e:
            print(f"Error saving critter to {filename}: {e}")
            return False

    @staticmethod
    def load_from_file(filename: str) -> Optional[Critter]:
        """
        Loads a critter's state from a JSON file.
        
        Args:
            filename: Path to the file
            
        Returns:
            A Critter instance if successful, None otherwise
        """
        try:
            with open(filename, 'r') as f:
                data = json.load(f)
            return Critter.from_dict(data)
        except (IOError, json.JSONDecodeError, KeyError) as e:
            print(f"Error loading critter from {filename}: {e}")
            return None
            
    @staticmethod
    def save_collection(critters: List[Critter], directory: str) -> bool:
        """
        Saves a collection of critters to a directory.
        
        Args:
            critters: List of Critter instances to save
            directory: Directory to save the critters in
            
        Returns:
            True if all saves were successful, False otherwise
        """
        import os
        
        try:
            # Create directory if it doesn't exist
            os.makedirs(directory, exist_ok=True)
            
            # Save each critter
            success = True
            for critter in critters:
                filename = f"{directory}/{critter.id}.json"
                if not CritterPersistence.save_to_file(critter, filename):
                    success = False
                    
            return success
        except Exception as e:
            print(f"Error saving critter collection: {e}")
            return False
            
    @staticmethod
    def load_collection(directory: str) -> List[Critter]:
        """
        Loads a collection of critters from a directory.
        
        Args:
            directory: Directory to load the critters from
            
        Returns:
            List of Critter instances
        """
        import os
        
        critters = []
        
        try:
            # Check if directory exists
            if not os.path.isdir(directory):
                return critters
                
            # Load each critter file
            for filename in os.listdir(directory):
                if filename.endswith('.json'):
                    filepath = f"{directory}/{filename}"
                    critter = CritterPersistence.load_from_file(filepath)
                    if critter:
                        critters.append(critter)
                        
            return critters
        except Exception as e:
            print(f"Error loading critter collection: {e}")
            return critters