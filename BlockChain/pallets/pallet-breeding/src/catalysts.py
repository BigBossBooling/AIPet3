"""
Catalysts module for the Echo-Synthesis breeding system.

This module defines the various catalysts and gene splicers used in the
breeding process.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, Union, Any, Callable

from .genetics import (
    GeneticCode, 
    CoreGenes, 
    PotentialGenes, 
    CosmeticGenes, 
    Adaptation,
    AuraType,
    Stat,
    Size,
    Pattern
)


class CatalystRarity(Enum):
    """Rarity levels for catalysts and gene splicers."""
    COMMON = auto()
    UNCOMMON = auto()
    RARE = auto()
    EPIC = auto()
    LEGENDARY = auto()


class Catalyst(ABC):
    """
    Abstract base class for all catalysts.
    
    Catalysts are items used to initiate the Echo-Synthesis process.
    """
    
    def __init__(self, name: str, description: str, rarity: CatalystRarity):
        """
        Initialize a catalyst.
        
        Args:
            name: The name of the catalyst.
            description: The description of the catalyst.
            rarity: The rarity of the catalyst.
        """
        self.name = name
        self.description = description
        self.rarity = rarity
    
    @abstractmethod
    def get_synthesis_boost(self) -> float:
        """
        Get the synthesis success chance boost provided by this catalyst.
        
        Returns:
            The boost as a percentage (0.0 to 1.0).
        """
        pass


class StableCatalyst(Catalyst):
    """
    Stable catalyst for standard (intra-species) breeding.
    
    This catalyst provides a small boost to the success chance of standard breeding.
    """
    
    def __init__(self, quality: int = 1):
        """
        Initialize a stable catalyst.
        
        Args:
            quality: The quality of the catalyst (1-5).
        """
        self.quality = max(1, min(5, quality))
        
        rarity = {
            1: CatalystRarity.COMMON,
            2: CatalystRarity.COMMON,
            3: CatalystRarity.UNCOMMON,
            4: CatalystRarity.RARE,
            5: CatalystRarity.EPIC
        }[self.quality]
        
        super().__init__(
            name=f"Stable Catalyst (Quality {self.quality})",
            description=f"A stable catalyst used for standard breeding. Quality {self.quality}/5.",
            rarity=rarity
        )
    
    def get_synthesis_boost(self) -> float:
        """
        Get the synthesis success chance boost provided by this catalyst.
        
        Returns:
            The boost as a percentage (0.0 to 1.0).
        """
        # Quality 1: 0% boost (base)
        # Quality 5: 10% boost
        return (self.quality - 1) * 0.025


class UnstableCatalyst(Catalyst):
    """
    Unstable catalyst for hybrid (cross-species) breeding.
    
    This catalyst is required for hybrid breeding and provides a boost to the
    success chance.
    """
    
    def __init__(self, quality: int = 1):
        """
        Initialize an unstable catalyst.
        
        Args:
            quality: The quality of the catalyst (1-5).
        """
        self.quality = max(1, min(5, quality))
        
        rarity = {
            1: CatalystRarity.UNCOMMON,
            2: CatalystRarity.RARE,
            3: CatalystRarity.RARE,
            4: CatalystRarity.EPIC,
            5: CatalystRarity.LEGENDARY
        }[self.quality]
        
        super().__init__(
            name=f"Unstable Catalyst (Quality {self.quality})",
            description=f"An unstable catalyst used for hybrid breeding. Quality {self.quality}/5.",
            rarity=rarity
        )
    
    def get_synthesis_boost(self) -> float:
        """
        Get the synthesis success chance boost provided by this catalyst.
        
        Returns:
            The boost as a percentage (0.0 to 1.0).
        """
        # Quality 1: 5% boost
        # Quality 5: 25% boost
        return 0.05 + (self.quality - 1) * 0.05


class GeneSplicer(ABC):
    """
    Abstract base class for all gene splicers.
    
    Gene splicers are advanced consumables used during the synthesis process
    to influence outcomes.
    """
    
    def __init__(self, name: str, description: str, rarity: CatalystRarity):
        """
        Initialize a gene splicer.
        
        Args:
            name: The name of the gene splicer.
            description: The description of the gene splicer.
            rarity: The rarity of the gene splicer.
        """
        self.name = name
        self.description = description
        self.rarity = rarity
    
    @abstractmethod
    def apply(self, parent_a: GeneticCode, parent_b: GeneticCode, offspring: GeneticCode) -> GeneticCode:
        """
        Apply the gene splicer's effect to the offspring.
        
        Args:
            parent_a: The genetic code of the first parent.
            parent_b: The genetic code of the second parent.
            offspring: The genetic code of the offspring before modification.
            
        Returns:
            The modified genetic code of the offspring.
        """
        pass


class DominantGeneSplice(GeneSplicer):
    """
    Guarantees a specific cosmetic gene is passed down.
    """
    
    def __init__(self, gene_type: str, parent_index: int):
        """
        Initialize a dominant gene splice.
        
        Args:
            gene_type: The type of gene to make dominant ("size", "pattern", or "color").
            parent_index: The parent to take the gene from (0 for parent_a, 1 for parent_b).
        """
        self.gene_type = gene_type
        self.parent_index = parent_index
        
        super().__init__(
            name=f"Dominant {gene_type.capitalize()} Splice",
            description=f"Guarantees the {gene_type} gene is inherited from parent {parent_index + 1}.",
            rarity=CatalystRarity.UNCOMMON
        )
    
    def apply(self, parent_a: GeneticCode, parent_b: GeneticCode, offspring: GeneticCode) -> GeneticCode:
        """
        Apply the gene splicer's effect to the offspring.
        
        Args:
            parent_a: The genetic code of the first parent.
            parent_b: The genetic code of the second parent.
            offspring: The genetic code of the offspring before modification.
            
        Returns:
            The modified genetic code of the offspring.
        """
        parent = parent_a if self.parent_index == 0 else parent_b
        
        if self.gene_type == "size":
            offspring.cosmetic.size = parent.cosmetic.size
        elif self.gene_type == "pattern":
            offspring.cosmetic.pattern = parent.cosmetic.pattern
        elif self.gene_type == "color":
            offspring.cosmetic.marking_color = parent.cosmetic.marking_color
        
        return offspring


class AuraStabilizer(GeneSplicer):
    """
    Increases the chance of inheriting a specific parent's aura and reduces
    the chance of a random mutation.
    """
    
    def __init__(self, parent_index: int):
        """
        Initialize an aura stabilizer.
        
        Args:
            parent_index: The parent to take the aura from (0 for parent_a, 1 for parent_b).
        """
        self.parent_index = parent_index
        
        super().__init__(
            name=f"Aura Stabilizer",
            description=f"Increases the chance of inheriting the aura from parent {parent_index + 1}.",
            rarity=CatalystRarity.RARE
        )
    
    def apply(self, parent_a: GeneticCode, parent_b: GeneticCode, offspring: GeneticCode) -> GeneticCode:
        """
        Apply the gene splicer's effect to the offspring.
        
        Args:
            parent_a: The genetic code of the first parent.
            parent_b: The genetic code of the second parent.
            offspring: The genetic code of the offspring before modification.
            
        Returns:
            The modified genetic code of the offspring.
        """
        parent = parent_a if self.parent_index == 0 else parent_b
        
        # 90% chance to inherit the specified parent's aura
        if random.random() < 0.9:
            offspring.core.aura = parent.core.aura
        
        return offspring


class PotentialSerum(GeneSplicer):
    """
    Skews the Variance in stat potential calculation, making a positive outcome more likely.
    """
    
    def __init__(self, target_stat: Optional[Stat] = None):
        """
        Initialize a potential serum.
        
        Args:
            target_stat: The stat to target, or None to target all stats.
        """
        self.target_stat = target_stat
        
        stat_name = target_stat.name if target_stat else "All Stats"
        
        super().__init__(
            name=f"{stat_name} Potential Serum",
            description=f"Increases the potential of {stat_name.lower()} in the offspring.",
            rarity=CatalystRarity.EPIC
        )
    
    def apply(self, parent_a: GeneticCode, parent_b: GeneticCode, offspring: GeneticCode) -> GeneticCode:
        """
        Apply the gene splicer's effect to the offspring.
        
        Args:
            parent_a: The genetic code of the first parent.
            parent_b: The genetic code of the second parent.
            offspring: The genetic code of the offspring before modification.
            
        Returns:
            The modified genetic code of the offspring.
        """
        if self.target_stat:
            # Target a specific stat
            current_potential = offspring.potential.stat_potential.get(self.target_stat, 50)
            boost = random.randint(5, 15)
            offspring.potential.stat_potential[self.target_stat] = min(100, current_potential + boost)
        else:
            # Target all stats
            for stat in Stat:
                current_potential = offspring.potential.stat_potential.get(stat, 50)
                boost = random.randint(2, 8)
                offspring.potential.stat_potential[stat] = min(100, current_potential + boost)
        
        return offspring


class AdaptationMemoryCell(GeneSplicer):
    """
    Guarantees a specific equipped Adaptation is inherited by the offspring.
    """
    
    def __init__(self, adaptation_id: str, parent_index: int):
        """
        Initialize an adaptation memory cell.
        
        Args:
            adaptation_id: The ID of the adaptation to guarantee inheritance.
            parent_index: The parent to take the adaptation from (0 for parent_a, 1 for parent_b).
        """
        self.adaptation_id = adaptation_id
        self.parent_index = parent_index
        
        super().__init__(
            name=f"Adaptation Memory Cell",
            description=f"Guarantees the {adaptation_id} adaptation is inherited from parent {parent_index + 1}.",
            rarity=CatalystRarity.LEGENDARY
        )
    
    def apply(self, parent_a: GeneticCode, parent_b: GeneticCode, offspring: GeneticCode) -> GeneticCode:
        """
        Apply the gene splicer's effect to the offspring.
        
        Args:
            parent_a: The genetic code of the first parent.
            parent_b: The genetic code of the second parent.
            offspring: The genetic code of the offspring before modification.
            
        Returns:
            The modified genetic code of the offspring.
        """
        # In a real implementation, this would add the adaptation to the
        # offspring's known adaptations list. For this prototype, we'll just
        # return the offspring unchanged.
        return offspring