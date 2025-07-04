"""
Synthesis module for the Echo-Synthesis breeding system.

This module implements the breeding mechanics, including standard breeding
(Intra-Species Synthesis) and cross-species breeding (Hybrid Synthesis).
"""

import random
import time
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, Union, Any

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


class SynthesisType(Enum):
    """Types of synthesis (breeding)."""
    INTRA_SPECIES = auto()  # Standard breeding (same species)
    HYBRID = auto()         # Cross-species breeding


class SynthesisState(Enum):
    """States of the synthesis process."""
    PENDING = auto()    # Waiting for synthesis to begin
    IN_PROGRESS = auto()  # Synthesis is in progress
    COMPLETED = auto()   # Synthesis completed successfully
    FAILED = auto()      # Synthesis failed


@dataclass
class SynthesisResult:
    """Result of a synthesis (breeding) process."""
    state: SynthesisState
    offspring: Optional[GeneticCode] = None
    error_message: Optional[str] = None
    timestamp: int = field(default_factory=lambda: int(time.time()))


class EchoSynthesizer:
    """
    Implements the Echo-Synthesis breeding system.
    
    This class handles both standard breeding (Intra-Species Synthesis) and
    cross-species breeding (Hybrid Synthesis).
    """
    
    def __init__(self, species_compatibility: Dict[str, List[str]] = None):
        """
        Initialize the Echo-Synthesizer.
        
        Args:
            species_compatibility: A dictionary mapping species to lists of
                compatible species for hybrid synthesis. If None, all species
                are considered compatible.
        """
        self.species_compatibility = species_compatibility or {}
        
        # Define hybrid species combinations
        self.hybrid_results = {
            frozenset(["sprite_glow", "sprite_aqua"]): "sprite_luminous",
            frozenset(["sprite_shadow", "sprite_crystal"]): "sprite_obsidian",
            frozenset(["sprite_ember", "sprite_terra"]): "sprite_magma",
            # Add more combinations as needed
        }
    
    def are_species_compatible(self, species_a: str, species_b: str) -> bool:
        """
        Check if two species are compatible for hybrid synthesis.
        
        Args:
            species_a: The first species.
            species_b: The second species.
            
        Returns:
            True if the species are compatible, False otherwise.
        """
        if not self.species_compatibility:
            return True  # All species are compatible if no compatibility map is provided
        
        return (
            species_b in self.species_compatibility.get(species_a, []) or
            species_a in self.species_compatibility.get(species_b, [])
        )
    
    def get_hybrid_species(self, species_a: str, species_b: str) -> Optional[str]:
        """
        Get the resulting hybrid species from two parent species.
        
        Args:
            species_a: The first parent species.
            species_b: The second parent species.
            
        Returns:
            The hybrid species, or None if no hybrid exists for these parents.
        """
        species_pair = frozenset([species_a, species_b])
        return self.hybrid_results.get(species_pair)
    
    def synthesize(
        self,
        parent_a: GeneticCode,
        parent_b: GeneticCode,
        parent_a_happiness: int,
        parent_b_happiness: int,
        synthesis_type: SynthesisType,
        zoologist_level: int = 1,
        catalysts: List[Any] = None,
        gene_splicers: List[Any] = None
    ) -> SynthesisResult:
        """
        Perform Echo-Synthesis (breeding) between two parents.
        
        Args:
            parent_a: The genetic code of the first parent.
            parent_b: The genetic code of the second parent.
            parent_a_happiness: The happiness of the first parent (0-100).
            parent_b_happiness: The happiness of the second parent (0-100).
            synthesis_type: The type of synthesis to perform.
            zoologist_level: The level of the zoologist performing the synthesis.
            catalysts: List of catalyst items to use.
            gene_splicers: List of gene splicer items to use.
            
        Returns:
            The result of the synthesis.
        """
        # Validate parents
        if synthesis_type == SynthesisType.INTRA_SPECIES and parent_a.core.species != parent_b.core.species:
            return SynthesisResult(
                state=SynthesisState.FAILED,
                error_message="Intra-species synthesis requires parents of the same species."
            )
        
        if synthesis_type == SynthesisType.HYBRID and not self.are_species_compatible(parent_a.core.species, parent_b.core.species):
            return SynthesisResult(
                state=SynthesisState.FAILED,
                error_message="These species are not compatible for hybrid synthesis."
            )
        
        # Check for hybrid synthesis failure chance
        if synthesis_type == SynthesisType.HYBRID:
            # Base failure chance is 30%, reduced by zoologist level
            failure_chance = 0.3 - (zoologist_level * 0.02)
            failure_chance = max(0.05, failure_chance)  # Minimum 5% failure chance
            
            if random.random() < failure_chance:
                return SynthesisResult(
                    state=SynthesisState.FAILED,
                    error_message="Hybrid synthesis failed. The catalysts and currency were consumed."
                )
        
        # Create offspring genetic code
        if synthesis_type == SynthesisType.INTRA_SPECIES:
            offspring = self._create_intra_species_offspring(
                parent_a, parent_b, parent_a_happiness, parent_b_happiness, gene_splicers
            )
        else:  # HYBRID
            offspring = self._create_hybrid_offspring(
                parent_a, parent_b, parent_a_happiness, parent_b_happiness, gene_splicers
            )
        
        return SynthesisResult(
            state=SynthesisState.COMPLETED,
            offspring=offspring
        )
    
    def _create_intra_species_offspring(
        self,
        parent_a: GeneticCode,
        parent_b: GeneticCode,
        parent_a_happiness: int,
        parent_b_happiness: int,
        gene_splicers: List[Any] = None
    ) -> GeneticCode:
        """
        Create an offspring from two parents of the same species.
        
        Args:
            parent_a: The genetic code of the first parent.
            parent_b: The genetic code of the second parent.
            parent_a_happiness: The happiness of the first parent (0-100).
            parent_b_happiness: The happiness of the second parent (0-100).
            gene_splicers: List of gene splicer items to use.
            
        Returns:
            The genetic code of the offspring.
        """
        # Create core genes
        # Species is inherited directly
        species = parent_a.core.species
        
        # Aura has a 49.5% chance from each parent, 1% chance of mutation
        aura_roll = random.random()
        if aura_roll < 0.495:
            aura = parent_a.core.aura
        elif aura_roll < 0.99:
            aura = parent_b.core.aura
        else:
            # Mutation - choose a random aura different from both parents
            available_auras = [a for a in AuraType if a != parent_a.core.aura and a != parent_b.core.aura]
            aura = random.choice(available_auras) if available_auras else parent_a.core.aura
        
        # Create lineage
        lineage = [parent_a.core.genesis_id, parent_b.core.genesis_id]
        
        core = CoreGenes(
            species=species,
            aura=aura,
            lineage=lineage
        )
        
        # Create potential genes
        # For each stat, the offspring's potential is calculated as:
        # Offspring_Pot = ((ParentA_Pot + ParentB_Pot) / 2) + Variance
        stat_potential = {}
        for stat in Stat:
            parent_a_pot = parent_a.potential.stat_potential.get(stat, 50)
            parent_b_pot = parent_b.potential.stat_potential.get(stat, 50)
            
            # Calculate variance based on parents' happiness
            # Higher happiness = more likely positive variance
            avg_happiness = (parent_a_happiness + parent_b_happiness) / 2
            variance_range = int(avg_happiness / 10)  # 0-10 range
            variance = random.randint(-5, variance_range)
            
            # Calculate offspring potential
            offspring_pot = int(((parent_a_pot + parent_b_pot) / 2) + variance)
            offspring_pot = max(1, min(100, offspring_pot))  # Clamp to 1-100
            
            stat_potential[stat] = offspring_pot
        
        # Adaptation slots are inherited as the average of parents, rounded up
        adaptation_slots = (parent_a.potential.adaptation_slots + parent_b.potential.adaptation_slots + 1) // 2
        
        potential = PotentialGenes(
            stat_potential=stat_potential,
            adaptation_slots=adaptation_slots
        )
        
        # Create cosmetic genes
        # Size has a 50% chance from each parent
        size = parent_a.cosmetic.size if random.random() < 0.5 else parent_b.cosmetic.size
        
        # Pattern has a 50% chance from each parent
        pattern = parent_a.cosmetic.pattern if random.random() < 0.5 else parent_b.cosmetic.pattern
        
        # Marking color is a blend of both parents
        color_a = int(parent_a.cosmetic.marking_color[1:], 16)
        color_b = int(parent_b.cosmetic.marking_color[1:], 16)
        
        # Extract RGB components
        r_a, g_a, b_a = (color_a >> 16) & 0xFF, (color_a >> 8) & 0xFF, color_a & 0xFF
        r_b, g_b, b_b = (color_b >> 16) & 0xFF, (color_b >> 8) & 0xFF, color_b & 0xFF
        
        # Blend with random weight
        weight = random.random()
        r = int(r_a * weight + r_b * (1 - weight))
        g = int(g_a * weight + g_b * (1 - weight))
        b = int(b_a * weight + b_b * (1 - weight))
        
        marking_color = f"#{r:02x}{g:02x}{b:02x}"
        
        # Glow intensity is the average of both parents
        glow_intensity = (parent_a.cosmetic.glow_intensity + parent_b.cosmetic.glow_intensity) / 2
        
        cosmetic = CosmeticGenes(
            size=size,
            pattern=pattern,
            marking_color=marking_color,
            glow_intensity=glow_intensity
        )
        
        return GeneticCode(
            core=core,
            potential=potential,
            cosmetic=cosmetic
        )
    
    def _create_hybrid_offspring(
        self,
        parent_a: GeneticCode,
        parent_b: GeneticCode,
        parent_a_happiness: int,
        parent_b_happiness: int,
        gene_splicers: List[Any] = None
    ) -> GeneticCode:
        """
        Create a hybrid offspring from two parents of different species.
        
        Args:
            parent_a: The genetic code of the first parent.
            parent_b: The genetic code of the second parent.
            parent_a_happiness: The happiness of the first parent (0-100).
            parent_b_happiness: The happiness of the second parent (0-100).
            gene_splicers: List of gene splicer items to use.
            
        Returns:
            The genetic code of the hybrid offspring.
        """
        # Get the hybrid species
        hybrid_species = self.get_hybrid_species(parent_a.core.species, parent_b.core.species)
        
        if not hybrid_species:
            # Fallback to a generic hybrid name if no specific one is defined
            hybrid_species = f"hybrid_{parent_a.core.species}_{parent_b.core.species}"
        
        # Create core genes
        # Aura has a 40% chance from each parent, 20% chance of mutation
        aura_roll = random.random()
        if aura_roll < 0.4:
            aura = parent_a.core.aura
        elif aura_roll < 0.8:
            aura = parent_b.core.aura
        else:
            # Mutation - choose a random aura different from both parents
            available_auras = [a for a in AuraType if a != parent_a.core.aura and a != parent_b.core.aura]
            aura = random.choice(available_auras) if available_auras else parent_a.core.aura
        
        # Create lineage
        lineage = [parent_a.core.genesis_id, parent_b.core.genesis_id]
        
        core = CoreGenes(
            species=hybrid_species,
            aura=aura,
            lineage=lineage
        )
        
        # Create potential genes
        # For hybrids, the potential is higher than either parent, but starting stats are lower
        stat_potential = {}
        for stat in Stat:
            parent_a_pot = parent_a.potential.stat_potential.get(stat, 50)
            parent_b_pot = parent_b.potential.stat_potential.get(stat, 50)
            
            # Take the maximum of both parents and add a bonus
            max_pot = max(parent_a_pot, parent_b_pot)
            bonus = random.randint(5, 15)  # Hybrid vigor bonus
            
            offspring_pot = min(100, max_pot + bonus)  # Capped at 100
            
            stat_potential[stat] = offspring_pot
        
        # Hybrids get more adaptation slots
        adaptation_slots = max(parent_a.potential.adaptation_slots, parent_b.potential.adaptation_slots) + 1
        adaptation_slots = min(7, adaptation_slots)  # Cap at 7 slots
        
        potential = PotentialGenes(
            stat_potential=stat_potential,
            adaptation_slots=adaptation_slots
        )
        
        # Create cosmetic genes - hybrids have more unique appearances
        # Size is random but influenced by parents
        size_options = [Size.TINY, Size.SMALL, Size.STANDARD, Size.LARGE, Size.HUGE]
        parent_a_size_index = size_options.index(parent_a.cosmetic.size)
        parent_b_size_index = size_options.index(parent_b.cosmetic.size)
        
        # Size index is within ±1 of the average of parents
        avg_size_index = (parent_a_size_index + parent_b_size_index) / 2
        size_index = int(avg_size_index + random.uniform(-1, 1))
        size_index = max(0, min(len(size_options) - 1, size_index))
        size = size_options[size_index]
        
        # Hybrids often have more exotic patterns
        exotic_patterns = [Pattern.IRIDESCENT, Pattern.CRYSTALLINE, Pattern.GLOWING]
        if random.random() < 0.6:  # 60% chance of exotic pattern
            pattern = random.choice(exotic_patterns)
        else:
            pattern = parent_a.cosmetic.pattern if random.random() < 0.5 else parent_b.cosmetic.pattern
        
        # Marking color is a more dramatic blend of both parents
        color_a = int(parent_a.cosmetic.marking_color[1:], 16)
        color_b = int(parent_b.cosmetic.marking_color[1:], 16)
        
        # Extract RGB components
        r_a, g_a, b_a = (color_a >> 16) & 0xFF, (color_a >> 8) & 0xFF, color_a & 0xFF
        r_b, g_b, b_b = (color_b >> 16) & 0xFF, (color_b >> 8) & 0xFF, color_b & 0xFF
        
        # For hybrids, we can create more dramatic color combinations
        # For example, take R from parent A, G from parent B, and average B
        r = r_a
        g = g_b
        b = (b_a + b_b) // 2
        
        # Add some randomness
        r = min(255, max(0, r + random.randint(-20, 20)))
        g = min(255, max(0, g + random.randint(-20, 20)))
        b = min(255, max(0, b + random.randint(-20, 20)))
        
        marking_color = f"#{r:02x}{g:02x}{b:02x}"
        
        # Glow intensity is higher for hybrids
        glow_intensity = max(parent_a.cosmetic.glow_intensity, parent_b.cosmetic.glow_intensity)
        glow_intensity = min(1.0, glow_intensity + random.uniform(0.1, 0.3))
        
        cosmetic = CosmeticGenes(
            size=size,
            pattern=pattern,
            marking_color=marking_color,
            glow_intensity=glow_intensity
        )
        
        return GeneticCode(
            core=core,
            potential=potential,
            cosmetic=cosmetic
        )