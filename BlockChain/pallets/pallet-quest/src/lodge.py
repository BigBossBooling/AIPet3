"""
Zoologist's Lodge module for Critter-Craft.

This module implements the Zoologist's Lodge (daycare) system in Critter-Craft,
where players can leave their pets when they are offline and hire other players
as temporary Caregivers.
"""

import random
import time
import uuid
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, Union, Any

# Import from other modules as needed
import sys
import os

# Add the parent directory to the Python path to import from other pallets
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-economy', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-ledger', 'src'))

# Import from economy system
from currencies import Bits, Aura
from inventory import Inventory

# Import from ledger system
from ledger import ZoologistLedger
from wallet import Wallet
from models import TransactionType, ZoologistLevel


class PersonalityTrait(Enum):
    """Personality traits for pets."""
    BRAVE = auto()
    CURIOUS = auto()
    FRIENDLY = auto()
    LOYAL = auto()
    PLAYFUL = auto()
    PROTECTIVE = auto()
    SHY = auto()
    STUBBORN = auto()


@dataclass
class PetState:
    """
    The state of a pet in the Zoologist's Lodge.
    
    This includes the pet's current stats, happiness, and personality traits.
    """
    pet_id: str
    owner_id: str
    name: str
    species: str
    level: int
    stats: Dict[str, int]
    happiness: int
    personality_traits: Dict[PersonalityTrait, int]  # Trait -> Strength (0-100)
    temporary_trait_boosts: Dict[PersonalityTrait, Tuple[int, int]] = field(default_factory=dict)  # Trait -> (Boost, Expiry Time)
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "pet_id": self.pet_id,
            "owner_id": self.owner_id,
            "name": self.name,
            "species": self.species,
            "level": self.level,
            "stats": self.stats,
            "happiness": self.happiness,
            "personality_traits": {trait.name: strength for trait, strength in self.personality_traits.items()},
            "temporary_trait_boosts": {trait.name: (boost, expiry) for trait, (boost, expiry) in self.temporary_trait_boosts.items()}
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'PetState':
        """Create from a dictionary."""
        personality_traits = {}
        for trait_name, strength in data.get("personality_traits", {}).items():
            personality_traits[PersonalityTrait[trait_name]] = strength
        
        temporary_trait_boosts = {}
        for trait_name, (boost, expiry) in data.get("temporary_trait_boosts", {}).items():
            temporary_trait_boosts[PersonalityTrait[trait_name]] = (boost, expiry)
        
        return cls(
            pet_id=data["pet_id"],
            owner_id=data["owner_id"],
            name=data["name"],
            species=data["species"],
            level=data["level"],
            stats=data["stats"],
            happiness=data["happiness"],
            personality_traits=personality_traits,
            temporary_trait_boosts=temporary_trait_boosts
        )


@dataclass
class CaregiverOffer:
    """
    An offer from a player to be a caregiver for pets in the Zoologist's Lodge.
    
    This includes the player's ID, the fee they charge, and their dominant personality trait.
    """
    id: str
    player_id: str
    fee: int  # Fee in BITS
    dominant_trait: PersonalityTrait
    reputation: int
    max_pets: int
    current_pets: int = 0
    
    def __post_init__(self):
        """Initialize with a UUID if not provided."""
        if not self.id:
            self.id = str(uuid.uuid4())
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "id": self.id,
            "player_id": self.player_id,
            "fee": self.fee,
            "dominant_trait": self.dominant_trait.name,
            "reputation": self.reputation,
            "max_pets": self.max_pets,
            "current_pets": self.current_pets
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'CaregiverOffer':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            player_id=data["player_id"],
            fee=data["fee"],
            dominant_trait=PersonalityTrait[data["dominant_trait"]],
            reputation=data["reputation"],
            max_pets=data["max_pets"],
            current_pets=data["current_pets"]
        )


@dataclass
class LodgingContract:
    """
    A contract between a pet owner and a caregiver in the Zoologist's Lodge.
    
    This includes the pet's ID, the owner's ID, the caregiver's ID, the fee,
    and the start and end times.
    """
    id: str
    pet_id: str
    owner_id: str
    caregiver_id: str
    fee: int  # Fee in BITS
    start_time: int
    end_time: int
    is_active: bool = True
    
    def __post_init__(self):
        """Initialize with a UUID if not provided."""
        if not self.id:
            self.id = str(uuid.uuid4())
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "id": self.id,
            "pet_id": self.pet_id,
            "owner_id": self.owner_id,
            "caregiver_id": self.caregiver_id,
            "fee": self.fee,
            "start_time": self.start_time,
            "end_time": self.end_time,
            "is_active": self.is_active
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'LodgingContract':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            pet_id=data["pet_id"],
            owner_id=data["owner_id"],
            caregiver_id=data["caregiver_id"],
            fee=data["fee"],
            start_time=data["start_time"],
            end_time=data["end_time"],
            is_active=data["is_active"]
        )


@dataclass
class CareActivity:
    """
    An activity performed by a caregiver on a pet in the Zoologist's Lodge.
    
    This includes the pet's ID, the caregiver's ID, the type of activity,
    and the timestamp.
    """
    id: str
    pet_id: str
    caregiver_id: str
    activity_type: str  # "feed", "play", "groom"
    timestamp: int
    happiness_gain: int
    stat_gains: Dict[str, int]
    
    def __post_init__(self):
        """Initialize with a UUID if not provided."""
        if not self.id:
            self.id = str(uuid.uuid4())
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "id": self.id,
            "pet_id": self.pet_id,
            "caregiver_id": self.caregiver_id,
            "activity_type": self.activity_type,
            "timestamp": self.timestamp,
            "happiness_gain": self.happiness_gain,
            "stat_gains": self.stat_gains
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'CareActivity':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            pet_id=data["pet_id"],
            caregiver_id=data["caregiver_id"],
            activity_type=data["activity_type"],
            timestamp=data["timestamp"],
            happiness_gain=data["happiness_gain"],
            stat_gains=data["stat_gains"]
        )


class ZoologistLodge:
    """
    The Zoologist's Lodge (daycare) system.
    
    This is where players can leave their pets when they are offline and hire
    other players as temporary Caregivers.
    """
    
    def __init__(self):
        """Initialize the Zoologist's Lodge."""
        self.pets: Dict[str, PetState] = {}  # pet_id -> PetState
        self.caregiver_offers: Dict[str, CaregiverOffer] = {}  # offer_id -> CaregiverOffer
        self.lodging_contracts: Dict[str, LodgingContract] = {}  # contract_id -> LodgingContract
        self.care_activities: Dict[str, List[CareActivity]] = {}  # pet_id -> List[CareActivity]
    
    def add_pet(self, pet: PetState) -> None:
        """
        Add a pet to the Zoologist's Lodge.
        
        Args:
            pet: The pet to add.
        """
        self.pets[pet.pet_id] = pet
        self.care_activities[pet.pet_id] = []
    
    def remove_pet(self, pet_id: str) -> Optional[PetState]:
        """
        Remove a pet from the Zoologist's Lodge.
        
        Args:
            pet_id: The ID of the pet to remove.
            
        Returns:
            The removed pet, or None if not found.
        """
        pet = self.pets.pop(pet_id, None)
        self.care_activities.pop(pet_id, None)
        
        # Cancel any active contracts for this pet
        for contract_id, contract in list(self.lodging_contracts.items()):
            if contract.pet_id == pet_id and contract.is_active:
                contract.is_active = False
                
                # Update the caregiver's current pets count
                caregiver_offer = self._get_caregiver_offer_by_player_id(contract.caregiver_id)
                if caregiver_offer:
                    caregiver_offer.current_pets -= 1
        
        return pet
    
    def get_pet(self, pet_id: str) -> Optional[PetState]:
        """
        Get a pet from the Zoologist's Lodge.
        
        Args:
            pet_id: The ID of the pet to get.
            
        Returns:
            The pet, or None if not found.
        """
        return self.pets.get(pet_id)
    
    def add_caregiver_offer(self, offer: CaregiverOffer) -> None:
        """
        Add a caregiver offer to the Zoologist's Lodge.
        
        Args:
            offer: The caregiver offer to add.
        """
        self.caregiver_offers[offer.id] = offer
    
    def remove_caregiver_offer(self, offer_id: str) -> Optional[CaregiverOffer]:
        """
        Remove a caregiver offer from the Zoologist's Lodge.
        
        Args:
            offer_id: The ID of the caregiver offer to remove.
            
        Returns:
            The removed caregiver offer, or None if not found.
        """
        return self.caregiver_offers.pop(offer_id, None)
    
    def get_caregiver_offer(self, offer_id: str) -> Optional[CaregiverOffer]:
        """
        Get a caregiver offer from the Zoologist's Lodge.
        
        Args:
            offer_id: The ID of the caregiver offer to get.
            
        Returns:
            The caregiver offer, or None if not found.
        """
        return self.caregiver_offers.get(offer_id)
    
    def _get_caregiver_offer_by_player_id(self, player_id: str) -> Optional[CaregiverOffer]:
        """
        Get a caregiver offer by player ID.
        
        Args:
            player_id: The ID of the player.
            
        Returns:
            The caregiver offer, or None if not found.
        """
        for offer in self.caregiver_offers.values():
            if offer.player_id == player_id:
                return offer
        return None
    
    def create_lodging_contract(self, pet_id: str, caregiver_offer_id: str, duration: int) -> Optional[LodgingContract]:
        """
        Create a lodging contract between a pet owner and a caregiver.
        
        Args:
            pet_id: The ID of the pet.
            caregiver_offer_id: The ID of the caregiver offer.
            duration: The duration of the contract in seconds.
            
        Returns:
            The created lodging contract, or None if creation failed.
        """
        pet = self.get_pet(pet_id)
        if not pet:
            return None
        
        caregiver_offer = self.get_caregiver_offer(caregiver_offer_id)
        if not caregiver_offer:
            return None
        
        # Check if the caregiver has reached their maximum number of pets
        if caregiver_offer.current_pets >= caregiver_offer.max_pets:
            return None
        
        # Check if the pet already has an active contract
        for contract in self.lodging_contracts.values():
            if contract.pet_id == pet_id and contract.is_active:
                return None
        
        # Create the contract
        contract = LodgingContract(
            id="",
            pet_id=pet_id,
            owner_id=pet.owner_id,
            caregiver_id=caregiver_offer.player_id,
            fee=caregiver_offer.fee,
            start_time=int(time.time()),
            end_time=int(time.time()) + duration,
            is_active=True
        )
        
        # Add the contract to the lodge
        self.lodging_contracts[contract.id] = contract
        
        # Update the caregiver's current pets count
        caregiver_offer.current_pets += 1
        
        return contract
    
    def cancel_lodging_contract(self, contract_id: str) -> bool:
        """
        Cancel a lodging contract.
        
        Args:
            contract_id: The ID of the lodging contract to cancel.
            
        Returns:
            True if the contract was cancelled successfully, False otherwise.
        """
        contract = self.lodging_contracts.get(contract_id)
        if not contract or not contract.is_active:
            return False
        
        # Mark the contract as inactive
        contract.is_active = False
        
        # Update the caregiver's current pets count
        caregiver_offer = self._get_caregiver_offer_by_player_id(contract.caregiver_id)
        if caregiver_offer:
            caregiver_offer.current_pets -= 1
        
        return True
    
    def perform_care_activity(self, caregiver_id: str, pet_id: str, activity_type: str) -> Optional[CareActivity]:
        """
        Perform a care activity on a pet.
        
        Args:
            caregiver_id: The ID of the caregiver performing the activity.
            pet_id: The ID of the pet to perform the activity on.
            activity_type: The type of activity to perform ("feed", "play", "groom").
            
        Returns:
            The created care activity, or None if the activity could not be performed.
        """
        # Check if the pet exists
        pet = self.get_pet(pet_id)
        if not pet:
            return None
        
        # Check if the caregiver has an active contract with the pet
        has_contract = False
        for contract in self.lodging_contracts.values():
            if (contract.pet_id == pet_id and
                contract.caregiver_id == caregiver_id and
                contract.is_active and
                contract.end_time > int(time.time())):
                has_contract = True
                break
        
        if not has_contract:
            return None
        
        # Get the caregiver's dominant trait
        caregiver_offer = self._get_caregiver_offer_by_player_id(caregiver_id)
        if not caregiver_offer:
            return None
        
        dominant_trait = caregiver_offer.dominant_trait
        
        # Calculate the happiness gain and stat gains based on the activity type
        happiness_gain = 0
        stat_gains = {}
        
        if activity_type == "feed":
            happiness_gain = 5
            stat_gains = {"energy": 2, "strength": 1}
        elif activity_type == "play":
            happiness_gain = 10
            stat_gains = {"agility": 2, "social": 1}
        elif activity_type == "groom":
            happiness_gain = 7
            stat_gains = {"charisma": 2, "iq": 1}
        else:
            return None
        
        # Create the care activity
        care_activity = CareActivity(
            id="",
            pet_id=pet_id,
            caregiver_id=caregiver_id,
            activity_type=activity_type,
            timestamp=int(time.time()),
            happiness_gain=happiness_gain,
            stat_gains=stat_gains
        )
        
        # Add the care activity to the lodge
        if pet_id not in self.care_activities:
            self.care_activities[pet_id] = []
        
        self.care_activities[pet_id].append(care_activity)
        
        # Update the pet's stats and happiness
        pet.happiness = min(100, pet.happiness + happiness_gain)
        
        for stat, gain in stat_gains.items():
            if stat in pet.stats:
                pet.stats[stat] = min(100, pet.stats[stat] + gain)
        
        # Apply a temporary trait boost based on the caregiver's dominant trait
        boost_amount = 10
        boost_duration = 3600  # 1 hour
        
        pet.temporary_trait_boosts[dominant_trait] = (boost_amount, int(time.time()) + boost_duration)
        
        return care_activity
    
    def get_care_activities(self, pet_id: str) -> List[CareActivity]:
        """
        Get all care activities for a pet.
        
        Args:
            pet_id: The ID of the pet.
            
        Returns:
            A list of care activities for the pet.
        """
        return self.care_activities.get(pet_id, [])
    
    def update_pet_traits(self) -> None:
        """Update all pets' temporary trait boosts."""
        current_time = int(time.time())
        
        for pet in self.pets.values():
            # Remove expired trait boosts
            expired_traits = []
            
            for trait, (boost, expiry) in pet.temporary_trait_boosts.items():
                if expiry <= current_time:
                    expired_traits.append(trait)
            
            for trait in expired_traits:
                pet.temporary_trait_boosts.pop(trait)
    
    def get_available_caregivers(self) -> List[CaregiverOffer]:
        """
        Get all available caregivers.
        
        Returns:
            A list of available caregiver offers.
        """
        return [
            offer for offer in self.caregiver_offers.values()
            if offer.current_pets < offer.max_pets
        ]
    
    def get_active_contracts(self, player_id: str) -> List[LodgingContract]:
        """
        Get all active contracts for a player.
        
        Args:
            player_id: The ID of the player.
            
        Returns:
            A list of active lodging contracts for the player.
        """
        return [
            contract for contract in self.lodging_contracts.values()
            if (contract.owner_id == player_id or contract.caregiver_id == player_id) and
            contract.is_active and
            contract.end_time > int(time.time())
        ]
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "pets": {pet_id: pet.to_dict() for pet_id, pet in self.pets.items()},
            "caregiver_offers": {offer_id: offer.to_dict() for offer_id, offer in self.caregiver_offers.items()},
            "lodging_contracts": {contract_id: contract.to_dict() for contract_id, contract in self.lodging_contracts.items()},
            "care_activities": {pet_id: [activity.to_dict() for activity in activities] for pet_id, activities in self.care_activities.items()}
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'ZoologistLodge':
        """Create from a dictionary."""
        lodge = cls()
        
        for pet_id, pet_data in data.get("pets", {}).items():
            lodge.pets[pet_id] = PetState.from_dict(pet_data)
        
        for offer_id, offer_data in data.get("caregiver_offers", {}).items():
            lodge.caregiver_offers[offer_id] = CaregiverOffer.from_dict(offer_data)
        
        for contract_id, contract_data in data.get("lodging_contracts", {}).items():
            lodge.lodging_contracts[contract_id] = LodgingContract.from_dict(contract_data)
        
        for pet_id, activities_data in data.get("care_activities", {}).items():
            lodge.care_activities[pet_id] = [CareActivity.from_dict(activity_data) for activity_data in activities_data]
        
        return lodge


# Create an example Zoologist's Lodge
def create_example_lodge() -> ZoologistLodge:
    """
    Create an example Zoologist's Lodge.
    
    Returns:
        A ZoologistLodge with example data.
    """
    lodge = ZoologistLodge()
    
    # Create some pets
    pet1 = PetState(
        pet_id="pet1",
        owner_id="player1",
        name="Sparkle",
        species="sprite_glow",
        level=5,
        stats={
            "iq": 60,
            "charisma": 70,
            "energy": 50,
            "agility": 55,
            "strength": 40,
            "social": 65
        },
        happiness=80,
        personality_traits={
            PersonalityTrait.CURIOUS: 80,
            PersonalityTrait.FRIENDLY: 70,
            PersonalityTrait.PLAYFUL: 60,
            PersonalityTrait.SHY: 30,
            PersonalityTrait.BRAVE: 40,
            PersonalityTrait.LOYAL: 50,
            PersonalityTrait.PROTECTIVE: 45,
            PersonalityTrait.STUBBORN: 35
        }
    )
    
    pet2 = PetState(
        pet_id="pet2",
        owner_id="player2",
        name="Ember",
        species="sprite_ember",
        level=7,
        stats={
            "iq": 55,
            "charisma": 50,
            "energy": 70,
            "agility": 65,
            "strength": 60,
            "social": 45
        },
        happiness=75,
        personality_traits={
            PersonalityTrait.CURIOUS: 50,
            PersonalityTrait.FRIENDLY: 60,
            PersonalityTrait.PLAYFUL: 70,
            PersonalityTrait.SHY: 20,
            PersonalityTrait.BRAVE: 80,
            PersonalityTrait.LOYAL: 65,
            PersonalityTrait.PROTECTIVE: 55,
            PersonalityTrait.STUBBORN: 45
        }
    )
    
    # Add the pets to the lodge
    lodge.add_pet(pet1)
    lodge.add_pet(pet2)
    
    # Create some caregiver offers
    caregiver1 = CaregiverOffer(
        id="",
        player_id="player3",
        fee=50,  # 50 BITS per day
        dominant_trait=PersonalityTrait.FRIENDLY,
        reputation=80,
        max_pets=3,
        current_pets=0
    )
    
    caregiver2 = CaregiverOffer(
        id="",
        player_id="player4",
        fee=75,  # 75 BITS per day
        dominant_trait=PersonalityTrait.BRAVE,
        reputation=90,
        max_pets=2,
        current_pets=0
    )
    
    # Add the caregiver offers to the lodge
    lodge.add_caregiver_offer(caregiver1)
    lodge.add_caregiver_offer(caregiver2)
    
    return lodge


if __name__ == "__main__":
    # Create an example lodge
    lodge = create_example_lodge()
    
    # Print all pets in the lodge
    print("Pets in the Zoologist's Lodge:")
    for pet in lodge.pets.values():
        print(f"- {pet.name} ({pet.species}), Level {pet.level}, Happiness: {pet.happiness}")
    
    # Print all caregiver offers
    print("\nCaregiver Offers:")
    for offer in lodge.caregiver_offers.values():
        print(f"- {offer.player_id}: {offer.fee} BITS per day, Dominant Trait: {offer.dominant_trait.name}, Reputation: {offer.reputation}")
    
    # Create a lodging contract
    pet_id = "pet1"
    caregiver_offer_id = list(lodge.caregiver_offers.keys())[0]
    duration = 86400  # 1 day
    
    contract = lodge.create_lodging_contract(pet_id, caregiver_offer_id, duration)
    
    if contract:
        print(f"\nCreated lodging contract for {lodge.pets[pet_id].name} with caregiver {contract.caregiver_id}")
        print(f"Fee: {contract.fee} BITS, Duration: {(contract.end_time - contract.start_time) // 3600} hours")
        
        # Perform some care activities
        caregiver_id = contract.caregiver_id
        
        feed_activity = lodge.perform_care_activity(caregiver_id, pet_id, "feed")
        play_activity = lodge.perform_care_activity(caregiver_id, pet_id, "play")
        groom_activity = lodge.perform_care_activity(caregiver_id, pet_id, "groom")
        
        print("\nCare Activities:")
        for activity in lodge.get_care_activities(pet_id):
            print(f"- {activity.activity_type.capitalize()}: +{activity.happiness_gain} happiness, Stats: {activity.stat_gains}")
        
        # Print the pet's updated state
        pet = lodge.get_pet(pet_id)
        print(f"\n{pet.name}'s Updated State:")
        print(f"Happiness: {pet.happiness}")
        print(f"Stats: {pet.stats}")
        print(f"Temporary Trait Boosts: {pet.temporary_trait_boosts}")
    else:
        print("\nFailed to create lodging contract.")