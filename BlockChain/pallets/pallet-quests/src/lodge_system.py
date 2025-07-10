"""
Zoologist's Lodge system for Critter-Craft.

This module implements the business logic for the Zoologist's Lodge (daycare) system,
where players can leave their pets when they are offline and hire other players
as temporary Caregivers.
"""

import time
from typing import Dict, List, Optional, Tuple

from lodge import ( # Direct import for sibling module
    PersonalityTrait,
    CareActivityType,
    PetState,
    CaregiverOffer,
    LodgingContract,
    CareActivityLog
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
        self.care_activities: Dict[str, List[CareActivityLog]] = {}  # pet_id -> List[CareActivityLog]
    
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
    
    def create_lodging_contract(self, pet_id: str, caregiver_offer_id: str, duration_days: int) -> Optional[LodgingContract]:
        """
        Create a lodging contract between a pet owner and a caregiver.
        
        Args:
            pet_id: The ID of the pet.
            caregiver_offer_id: The ID of the caregiver offer.
            duration_days: The duration of the contract in days.
            
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
        
        # Calculate duration in seconds
        duration_seconds = duration_days * 86400  # 86400 seconds in a day
        
        # Create the contract
        contract = LodgingContract(
            pet_id=pet_id,
            owner_id=pet.owner_id,
            caregiver_id=caregiver_offer.player_id,
            daily_fee=caregiver_offer.fee_per_day,
            start_time=int(time.time()),
            end_time=int(time.time()) + duration_seconds
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
    
    def perform_care_activity(self, caregiver_id: str, pet_id: str, activity_type: CareActivityType) -> Optional[CareActivityLog]:
        """
        Perform a care activity on a pet.
        
        Args:
            caregiver_id: The ID of the caregiver performing the activity.
            pet_id: The ID of the pet to perform the activity on.
            activity_type: The type of activity to perform.
            
        Returns:
            The created care activity log, or None if the activity could not be performed.
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
        
        if activity_type == CareActivityType.FEED:
            happiness_gain = 5
            stat_gains = {"energy": 2, "strength": 1}
        elif activity_type == CareActivityType.PLAY:
            happiness_gain = 10
            stat_gains = {"agility": 2, "social": 1}
        elif activity_type == CareActivityType.GROOM:
            happiness_gain = 7
            stat_gains = {"charisma": 2, "iq": 1}
        
        # Create the care activity log
        care_activity = CareActivityLog(
            pet_id=pet_id,
            caregiver_id=caregiver_id,
            activity_type=activity_type,
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
    
    def get_care_activities(self, pet_id: str) -> List[CareActivityLog]:
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
        player_id="player3",
        fee_per_day=50,  # 50 BITS per day
        dominant_trait=PersonalityTrait.FRIENDLY,
        reputation=80,
        max_pets=3
    )
    
    caregiver2 = CaregiverOffer(
        player_id="player4",
        fee_per_day=75,  # 75 BITS per day
        dominant_trait=PersonalityTrait.BRAVE,
        reputation=90,
        max_pets=2
    )
    
    # Add the caregiver offers to the lodge
    lodge.add_caregiver_offer(caregiver1)
    lodge.add_caregiver_offer(caregiver2)
    
    return lodge