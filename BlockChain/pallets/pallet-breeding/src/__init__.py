"""
Echo-Synthesis Breeding System for Critter-Craft

A strategic, end-game system that allows players to combine the genetic and
spiritual essence of their companions to discover new potential, create unique
hybrids, and cement a permanent legacy on the blockchain.
"""

from .genetics import (
    GeneticCode, 
    CoreGenes, 
    PotentialGenes, 
    CosmeticGenes, 
    Adaptation
)
from .synthesis import (
    EchoSynthesizer, 
    SynthesisResult, 
    SynthesisType, 
    SynthesisState
)
from .catalysts import (
    Catalyst, 
    StableCatalyst, 
    UnstableCatalyst, 
    GeneSplicer, 
    DominantGeneSplice, 
    AuraStabilizer, 
    PotentialSerum, 
    AdaptationMemoryCell
)
from .lineage import (
    FamilyTree, 
    LineageNode, 
    calculate_inbreeding_coefficient
)

__all__ = [
    'GeneticCode',
    'CoreGenes',
    'PotentialGenes',
    'CosmeticGenes',
    'Adaptation',
    'EchoSynthesizer',
    'SynthesisResult',
    'SynthesisType',
    'SynthesisState',
    'Catalyst',
    'StableCatalyst',
    'UnstableCatalyst',
    'GeneSplicer',
    'DominantGeneSplice',
    'AuraStabilizer',
    'PotentialSerum',
    'AdaptationMemoryCell',
    'FamilyTree',
    'LineageNode',
    'calculate_inbreeding_coefficient',
]