"""
Data models for the Zoologist's Lodge system in Critter-Craft.

This module defines the core data structures (dataclasses) that represent the
state of pets, caregivers, and contracts within the Lodge. It keeps data
representation separate from business logic.
"""
import time
import uuid
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, Tuple, List, Any

class PersonalityTrait(Enum):
    """Enumeration of all possible personality traits."""
    BRAVE = auto()
    CURIOUS = auto()
    FRIENDLY = auto()
    LOYAL = auto()
    PLAYFUL = auto()
    PROTECTIVE = auto()
    SHY = auto()
    STUBBORN = auto()

class CareActivityType(Enum):
    """Enumeration of all possible care activities."""
    FEED = auto()
    PLAY = auto()
    GROOM = auto()

@dataclass
class PetState:
    """Represents the state of a pet currently checked into the Lodge."""
    pet_id: str
    owner_id: str
    name: str
    species: str
    level: int
    stats: Dict[str, int]
    happiness: int
    personality_traits: Dict[PersonalityTrait, int]
    temporary_trait_boosts: Dict[PersonalityTrait, Tuple[int, int]] = field(default_factory=dict)

@dataclass
class CaregiverOffer:
    """Represents a player's public offer to act as a caregiver."""
    player_id: str
    fee_per_day: int
    dominant_trait: PersonalityTrait
    reputation: int
    max_pets: int
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    current_pets: int = 0

@dataclass
class LodgingContract:
    """Represents an active agreement between a pet owner and a caregiver."""
    pet_id: str
    owner_id: str
    caregiver_id: str
    daily_fee: int
    start_time: int
    end_time: int
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    is_active: bool = True

@dataclass
class CareActivityLog:
    """A log entry for a single care action performed by a caregiver."""
    pet_id: str
    caregiver_id: str
    activity_type: CareActivityType
    timestamp: int = field(default_factory=time.time_ns)
    happiness_gain: int = 0
    stat_gains: Dict[str, int] = field(default_factory=dict)
