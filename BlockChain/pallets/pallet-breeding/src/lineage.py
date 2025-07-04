"""
Lineage module for the Echo-Synthesis breeding system.

This module implements the family tree and inbreeding mechanics.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Set, Tuple, Union

from .genetics import GeneticCode


@dataclass
class LineageNode:
    """
    Represents a node in a pet's family tree.
    
    Each node contains a pet's genesis ID and references to its parents.
    """
    genesis_id: str
    parent_a_id: Optional[str] = None
    parent_b_id: Optional[str] = None
    children: List[str] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "genesis_id": self.genesis_id,
            "parent_a_id": self.parent_a_id,
            "parent_b_id": self.parent_b_id,
            "children": self.children
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'LineageNode':
        """Create from a dictionary."""
        return cls(
            genesis_id=data["genesis_id"],
            parent_a_id=data["parent_a_id"],
            parent_b_id=data["parent_b_id"],
            children=data["children"]
        )


class FamilyTree:
    """
    Represents a family tree of pets.
    
    This class provides methods for building and querying a pet's lineage.
    """
    
    def __init__(self):
        """Initialize an empty family tree."""
        self.nodes: Dict[str, LineageNode] = {}
    
    def add_pet(self, pet: GeneticCode) -> None:
        """
        Add a pet to the family tree.
        
        Args:
            pet: The genetic code of the pet to add.
        """
        genesis_id = pet.core.genesis_id
        
        # If the pet already exists in the tree, do nothing
        if genesis_id in self.nodes:
            return
        
        # Get parent IDs from the pet's lineage
        parent_a_id = pet.core.lineage[0] if pet.core.lineage else None
        parent_b_id = pet.core.lineage[1] if len(pet.core.lineage) > 1 else None
        
        # Create a new node for the pet
        node = LineageNode(
            genesis_id=genesis_id,
            parent_a_id=parent_a_id,
            parent_b_id=parent_b_id
        )
        
        # Add the node to the tree
        self.nodes[genesis_id] = node
        
        # Update parent nodes
        if parent_a_id and parent_a_id in self.nodes:
            self.nodes[parent_a_id].children.append(genesis_id)
        
        if parent_b_id and parent_b_id in self.nodes:
            self.nodes[parent_b_id].children.append(genesis_id)
    
    def get_ancestors(self, genesis_id: str, generations: int = 3) -> Dict[str, LineageNode]:
        """
        Get a pet's ancestors up to a certain number of generations.
        
        Args:
            genesis_id: The genesis ID of the pet.
            generations: The number of generations to include.
            
        Returns:
            A dictionary mapping genesis IDs to LineageNodes.
        """
        if genesis_id not in self.nodes or generations <= 0:
            return {}
        
        ancestors = {}
        self._collect_ancestors(genesis_id, ancestors, generations)
        return ancestors
    
    def _collect_ancestors(self, genesis_id: str, ancestors: Dict[str, LineageNode], generations_left: int) -> None:
        """
        Recursively collect ancestors.
        
        Args:
            genesis_id: The genesis ID of the pet.
            ancestors: Dictionary to collect ancestors in.
            generations_left: Number of generations left to collect.
        """
        if genesis_id not in self.nodes or generations_left <= 0:
            return
        
        node = self.nodes[genesis_id]
        ancestors[genesis_id] = node
        
        if node.parent_a_id:
            self._collect_ancestors(node.parent_a_id, ancestors, generations_left - 1)
        
        if node.parent_b_id:
            self._collect_ancestors(node.parent_b_id, ancestors, generations_left - 1)
    
    def get_descendants(self, genesis_id: str, generations: int = 3) -> Dict[str, LineageNode]:
        """
        Get a pet's descendants up to a certain number of generations.
        
        Args:
            genesis_id: The genesis ID of the pet.
            generations: The number of generations to include.
            
        Returns:
            A dictionary mapping genesis IDs to LineageNodes.
        """
        if genesis_id not in self.nodes or generations <= 0:
            return {}
        
        descendants = {}
        self._collect_descendants(genesis_id, descendants, generations)
        return descendants
    
    def _collect_descendants(self, genesis_id: str, descendants: Dict[str, LineageNode], generations_left: int) -> None:
        """
        Recursively collect descendants.
        
        Args:
            genesis_id: The genesis ID of the pet.
            descendants: Dictionary to collect descendants in.
            generations_left: Number of generations left to collect.
        """
        if genesis_id not in self.nodes or generations_left <= 0:
            return
        
        node = self.nodes[genesis_id]
        descendants[genesis_id] = node
        
        for child_id in node.children:
            self._collect_descendants(child_id, descendants, generations_left - 1)
    
    def find_common_ancestors(self, genesis_id_a: str, genesis_id_b: str, generations: int = 3) -> List[str]:
        """
        Find common ancestors between two pets.
        
        Args:
            genesis_id_a: The genesis ID of the first pet.
            genesis_id_b: The genesis ID of the second pet.
            generations: The number of generations to search.
            
        Returns:
            A list of genesis IDs of common ancestors.
        """
        ancestors_a = self.get_ancestors(genesis_id_a, generations)
        ancestors_b = self.get_ancestors(genesis_id_b, generations)
        
        return list(set(ancestors_a.keys()) & set(ancestors_b.keys()))
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            genesis_id: node.to_dict() for genesis_id, node in self.nodes.items()
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'FamilyTree':
        """Create from a dictionary."""
        tree = cls()
        tree.nodes = {
            genesis_id: LineageNode.from_dict(node_data)
            for genesis_id, node_data in data.items()
        }
        return tree


def calculate_inbreeding_coefficient(family_tree: FamilyTree, pet_a: GeneticCode, pet_b: GeneticCode) -> float:
    """
    Calculate the inbreeding coefficient between two pets.
    
    The inbreeding coefficient is a measure of how closely related two pets are.
    A higher coefficient indicates a higher risk of inbreeding depression.
    
    Args:
        family_tree: The family tree to use for the calculation.
        pet_a: The genetic code of the first pet.
        pet_b: The genetic code of the second pet.
        
    Returns:
        The inbreeding coefficient (0.0 to 1.0).
    """
    # Find common ancestors
    common_ancestors = family_tree.find_common_ancestors(
        pet_a.core.genesis_id, pet_b.core.genesis_id, generations=3
    )
    
    if not common_ancestors:
        return 0.0
    
    # Calculate the inbreeding coefficient based on the number and proximity of common ancestors
    # This is a simplified calculation for the prototype
    # In a real implementation, this would use a more sophisticated algorithm
    
    # Get all ancestors for both pets
    ancestors_a = family_tree.get_ancestors(pet_a.core.genesis_id, generations=3)
    ancestors_b = family_tree.get_ancestors(pet_b.core.genesis_id, generations=3)
    
    # Calculate the coefficient based on the number of common ancestors
    # and their position in the family tree
    coefficient = len(common_ancestors) / max(len(ancestors_a), len(ancestors_b))
    
    # If the pets are siblings (same parents), increase the coefficient
    if (pet_a.core.lineage and pet_b.core.lineage and
            pet_a.core.lineage[0] == pet_b.core.lineage[0] and
            pet_a.core.lineage[1] == pet_b.core.lineage[1]):
        coefficient = max(0.5, coefficient)
    
    # If one pet is the parent of the other, set a high coefficient
    if (pet_a.core.genesis_id in pet_b.core.lineage or
            pet_b.core.genesis_id in pet_a.core.lineage):
        coefficient = 0.75
    
    return coefficient