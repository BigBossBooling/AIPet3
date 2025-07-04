"""
State models for the battle system.

This module contains all the data classes and enums that represent
the state of a battle, including pets, environments, and status effects.
"""

from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set


class StatusEffect(Enum):
    """Enum for all possible status effects in battle."""
    PACIFIED = auto()      # Stamina is at zero. Cannot fight.
    POISONED = auto()      # Loses a percentage of Stamina each turn.
    BURNED = auto()        # Loses Stamina each turn and has reduced attack power.
    BLINDED = auto()       # Accuracy is significantly reduced.
    CAMOUFLAGED = auto()   # Evasion is significantly increased.
    EMPOWERED = auto()     # Damage output is increased.
    SLOWED = auto()        # Gains fewer AP per turn.
    INSPIRED = auto()      # Increased chance of landing critical hits.


@dataclass
class StatusEffectInstance:
    """An instance of a status effect with duration and potency."""
    effect: StatusEffect
    duration: int  # Number of turns remaining
    potency: float = 1.0  # Multiplier for effect strength (1.0 = normal)
    source: str = ""  # Description of what caused this effect


@dataclass
class BattlePet:
    """
    In-battle representation of a pet.
    
    This is separate from the core Pet object and contains only battle-relevant state.
    """
    # Basic info
    name: str
    species: str
    level: int = 1
    
    # Battle stats
    max_stamina: int = 100
    current_stamina: int = 100
    base_ap_per_turn: int = 4
    current_ap: int = 0
    
    # Combat attributes
    attack: int = 10
    defense: int = 10
    speed: int = 10
    accuracy: int = 90  # Base percentage chance to hit
    evasion: int = 10   # Base percentage chance to dodge
    
    # Status tracking
    status_effects: List[StatusEffectInstance] = field(default_factory=list)
    adaptations: List[str] = field(default_factory=list)
    equipped_items: Dict[str, str] = field(default_factory=dict)
    
    # Battle history for this pet
    actions_taken: List[str] = field(default_factory=list)
    damage_dealt: int = 0
    damage_received: int = 0
    
    def is_pacified(self) -> bool:
        """Check if the pet is pacified (unable to battle)."""
        return self.current_stamina <= 0 or any(
            status.effect == StatusEffect.PACIFIED for status in self.status_effects
        )
    
    def add_status_effect(self, effect: StatusEffect, duration: int, potency: float = 1.0, source: str = ""):
        """Add a status effect to this pet."""
        # If the effect already exists, refresh its duration and update potency if higher
        for existing in self.status_effects:
            if existing.effect == effect:
                existing.duration = max(existing.duration, duration)
                existing.potency = max(existing.potency, potency)
                existing.source = source if potency > existing.potency else existing.source
                return
        
        # Otherwise, add a new effect
        self.status_effects.append(StatusEffectInstance(effect, duration, potency, source))
    
    def remove_status_effect(self, effect: StatusEffect):
        """Remove a status effect from this pet."""
        self.status_effects = [s for s in self.status_effects if s.effect != effect]
    
    def update_status_effects(self):
        """Update status effects at the end of a turn."""
        # Decrement duration and remove expired effects
        self.status_effects = [
            status for status in self.status_effects
            if (status.duration := status.duration - 1) > 0
        ]
    
    def get_ap_for_turn(self) -> int:
        """Calculate AP for the current turn, accounting for status effects."""
        ap = self.base_ap_per_turn
        
        # Apply status effect modifiers
        for status in self.status_effects:
            if status.effect == StatusEffect.SLOWED:
                ap -= int(1 * status.potency)  # Lose 1 AP per turn when slowed
        
        return max(1, ap)  # Always get at least 1 AP
    
    def get_effective_accuracy(self) -> float:
        """Get accuracy percentage after applying status effects."""
        accuracy = self.accuracy
        
        for status in self.status_effects:
            if status.effect == StatusEffect.BLINDED:
                accuracy *= (1 - 0.3 * status.potency)  # Reduce accuracy by 30% when blinded
        
        return max(10, accuracy)  # Minimum 10% accuracy
    
    def get_effective_evasion(self) -> float:
        """Get evasion percentage after applying status effects."""
        evasion = self.evasion
        
        for status in self.status_effects:
            if status.effect == StatusEffect.CAMOUFLAGED:
                evasion += 30 * status.potency  # +30% evasion when camouflaged
        
        return evasion
    
    def get_effective_attack(self) -> float:
        """Get attack power after applying status effects."""
        attack = self.attack
        
        for status in self.status_effects:
            if status.effect == StatusEffect.EMPOWERED:
                attack *= (1 + 0.2 * status.potency)  # +20% attack when empowered
            elif status.effect == StatusEffect.BURNED:
                attack *= (1 - 0.15 * status.potency)  # -15% attack when burned
        
        return max(1, attack)  # Minimum attack of 1


@dataclass
class BattleEnvironment:
    """Represents the environment where a battle takes place."""
    name: str
    description: str
    effects: Dict[str, float] = field(default_factory=dict)
    
    # Special features available in this environment
    available_actions: List[str] = field(default_factory=list)
    
    # Visual/flavor elements
    background_image: str = ""
    ambient_sounds: List[str] = field(default_factory=list)
    
    def apply_environment_effects(self, pet: BattlePet, turn_number: int) -> List[str]:
        """
        Apply environment effects to a pet at the end of a turn.
        
        Returns:
            A list of messages describing what happened.
        """
        messages = []
        
        # Example environment effects
        if self.name == "Murky Swamp":
            # 25% chance for non-aquatic critters to become slowed
            import random
            if "aquatic" not in pet.adaptations and random.random() < 0.25:
                pet.add_status_effect(
                    StatusEffect.SLOWED, 
                    duration=1, 
                    source="Murky Swamp's thick mud"
                )
                messages.append(f"{pet.name} is slowed by the thick swamp mud!")
        
        elif self.name == "Geothermal Vents":
            # Non-fire types take minor burn damage each turn
            if "fire" not in pet.adaptations:
                damage = max(1, int(pet.max_stamina * 0.03))  # 3% of max stamina
                pet.current_stamina = max(0, pet.current_stamina - damage)
                messages.append(f"{pet.name} takes {damage} burn damage from the hot vents!")
                
                # Add BURNED status if not already present
                if not any(s.effect == StatusEffect.BURNED for s in pet.status_effects):
                    pet.add_status_effect(
                        StatusEffect.BURNED,
                        duration=2,
                        source="Geothermal heat"
                    )
                    messages.append(f"{pet.name} is burned by the intense heat!")
        
        return messages