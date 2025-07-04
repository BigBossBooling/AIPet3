"""
Genetics module for the Echo-Synthesis breeding system.

This module defines the genetic code structure for critters, including core genes,
potential genes, and cosmetic genes.
"""

import random
import uuid
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, Union

# Import Stat enum from config (simulated here)
class Stat(Enum):
    """Enum representing the different stats a pet can have."""
    STAMINA = auto()
    ENERGY = auto()
    CHARISMA = auto()
    INTELLIGENCE = auto()
    AGILITY = auto()
    STRENGTH = auto()
    PERCEPTION = auto()
    RESILIENCE = auto()


class AuraType(Enum):
    """Types of auras a pet can have."""
    RED = "Passionate"  # Boosts Strength
    BLUE = "Tranquil"   # Boosts Intelligence
    GREEN = "Nurturing" # Boosts Resilience
    YELLOW = "Curious"  # Boosts Perception
    PURPLE = "Mystical" # Boosts Energy
    ORANGE = "Vibrant"  # Boosts Agility
    PINK = "Loving"     # Boosts Charisma
    GOLD = "Confident"  # Boosts all stats slightly
    SILVER = "Balanced" # Reduces stat decay
    BLACK = "Enigmatic" # Rare, unpredictable effects
    WHITE = "Serene"    # Rare, increases happiness gain


class Size(Enum):
    """Sizes a pet can be."""
    TINY = auto()
    SMALL = auto()
    STANDARD = auto()
    LARGE = auto()
    HUGE = auto()


class Pattern(Enum):
    """Patterns a pet can have."""
    SOLID = auto()
    SPOTTED = auto()
    STRIPED = auto()
    MOTTLED = auto()
    IRIDESCENT = auto()
    GRADIENT = auto()
    CRYSTALLINE = auto()
    GLOWING = auto()


@dataclass
class Adaptation:
    """Represents an adaptation ability that a pet can learn and use."""
    id: str
    name: str
    description: str
    ap_cost: int
    species_requirements: List[str] = field(default_factory=list)
    
    def is_compatible_with(self, species: str) -> bool:
        """Check if this adaptation is compatible with the given species."""
        return not self.species_requirements or species in self.species_requirements


@dataclass
class CoreGenes:
    """
    Represents the immutable core genes of a pet.
    
    These genes are stored directly or derived from the pet's NFT on the
    Zoologist's Ledger.
    """
    species: str
    aura: AuraType
    genesis_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    lineage: List[str] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "species": self.species,
            "aura": self.aura.name,
            "genesis_id": self.genesis_id,
            "lineage": self.lineage
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'CoreGenes':
        """Create from a dictionary."""
        return cls(
            species=data["species"],
            aura=AuraType[data["aura"]],
            genesis_id=data["genesis_id"],
            lineage=data["lineage"]
        )


@dataclass
class PotentialGenes:
    """
    Represents the mutable and trainable potential genes of a pet.
    
    These genes define the pet's potential, not its current state. They represent
    the upper limits of what a pet can achieve through training.
    """
    stat_potential: Dict[Stat, int] = field(default_factory=dict)
    adaptation_slots: int = 3
    
    def __post_init__(self):
        """Initialize with default values if not provided."""
        if not self.stat_potential:
            self.stat_potential = {stat: random.randint(50, 80) for stat in Stat}
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "stat_potential": {stat.name: value for stat, value in self.stat_potential.items()},
            "adaptation_slots": self.adaptation_slots
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'PotentialGenes':
        """Create from a dictionary."""
        return cls(
            stat_potential={Stat[stat]: value for stat, value in data["stat_potential"].items()},
            adaptation_slots=data["adaptation_slots"]
        )


@dataclass
class CosmeticGenes:
    """
    Represents the heritable cosmetic genes of a pet.
    
    These genes determine the pet's appearance.
    """
    size: Size = Size.STANDARD
    pattern: Pattern = Pattern.SOLID
    marking_color: str = "#FFFFFF"  # Hex color code
    glow_intensity: float = 0.0  # 0.0 to 1.0
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "size": self.size.name,
            "pattern": self.pattern.name,
            "marking_color": self.marking_color,
            "glow_intensity": self.glow_intensity
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'CosmeticGenes':
        """Create from a dictionary."""
        return cls(
            size=Size[data["size"]],
            pattern=Pattern[data["pattern"]],
            marking_color=data["marking_color"],
            glow_intensity=data["glow_intensity"]
        )


@dataclass
class GeneticCode:
    """
    Represents the complete genetic code of a pet.
    
    This is the underlying data structure for all inheritance.
    """
    core: CoreGenes
    potential: PotentialGenes
    cosmetic: CosmeticGenes
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "core": self.core.to_dict(),
            "potential": self.potential.to_dict(),
            "cosmetic": self.cosmetic.to_dict()
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'GeneticCode':
        """Create from a dictionary."""
        return cls(
            core=CoreGenes.from_dict(data["core"]),
            potential=PotentialGenes.from_dict(data["potential"]),
            cosmetic=CosmeticGenes.from_dict(data["cosmetic"])
        )
    
    @classmethod
    def generate_random(cls, species: str, aura: Optional[AuraType] = None) -> 'GeneticCode':
        """Generate a random genetic code for the given species."""
        if aura is None:
            aura = random.choice(list(AuraType))
        
        core = CoreGenes(
            species=species,
            aura=aura
        )
        
        potential = PotentialGenes(
            stat_potential={stat: random.randint(50, 80) for stat in Stat},
            adaptation_slots=random.randint(3, 5)
        )
        
        cosmetic = CosmeticGenes(
            size=random.choice(list(Size)),
            pattern=random.choice(list(Pattern)),
            marking_color=f"#{random.randint(0, 0xFFFFFF):06x}",
            glow_intensity=random.uniform(0.0, 1.0)
        )
        
        return cls(core=core, potential=potential, cosmetic=cosmetic)
    
    def calculate_genetic_hash(self) -> str:
        """
        Calculate a unique genetic hash for this genetic code.
        
        This hash is used to identify the pet on the blockchain.
        """
        # In a real implementation, this would be a more sophisticated hash
        # For this prototype, we'll use a simple string representation
        return f"{self.core.species}_{self.core.aura.name}_{self.core.genesis_id[:8]}"