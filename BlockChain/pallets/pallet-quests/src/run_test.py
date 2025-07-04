#!/usr/bin/env python3
"""
Test script for the Activities and Zoologist's Lodge systems.

This script tests the basic functionality of the activities and lodge systems.
"""

import sys
import os
import time
import random
from typing import Dict, List, Optional

# Import from activities system
from activities import ActivityType, StatType
from activities_system import (
    ActivityManager,
    Activity,
    ActivityReward,
    create_activity_from_config,
    create_activity_manager_from_config
)
from config_activities import ACTIVITIES_CONFIG

# Import from lodge system
from lodge import (
    PersonalityTrait,
    CareActivityType,
    PetState,
    CaregiverOffer,
    LodgingContract,
    CareActivityLog
)
from lodge_system import ZoologistLodge, create_example_lodge


def test_activities():
    """Test the activities system."""
    print("Testing Activities System...")
    
    # Create an activity manager
    activity_manager = create_activity_manager_from_config(ACTIVITIES_CONFIG)
    
    # Get all activities
    all_activities = list(activity_manager.activities.values())
    print(f"Total activities: {len(all_activities)}")
    
    # Get activities by type
    mini_games = activity_manager.get_activities_by_type(ActivityType.MINI_GAME)
    print(f"Mini-games: {len(mini_games)}")
    
    two_player_games = activity_manager.get_activities_by_type(ActivityType.TWO_PLAYER_GAME)
    print(f"Two-player games: {len(two_player_games)}")
    
    jobs = activity_manager.get_activities_by_type(ActivityType.JOB)
    print(f"Jobs: {len(jobs)}")
    
    quests = activity_manager.get_activities_by_type(ActivityType.QUEST)
    print(f"Quests: {len(quests)}")
    
    adventurous_quests = activity_manager.get_activities_by_type(ActivityType.ADVENTUROUS_QUEST)
    print(f"Adventurous quests: {len(adventurous_quests)}")
    
    # Test completing an activity
    if mini_games:
        mini_game = mini_games[0]
        player_id = "test_player"
        score = 100
        player_level = 10
        
        print(f"\nCompleting mini-game: {mini_game.name}")
        rewards = activity_manager.complete_activity(player_id, mini_game.id, score, player_level)
        
        print("Rewards:")
        print(f"- BITS: {rewards.bits}")
        print(f"- AURA: {rewards.aura}")
        print(f"- Stat Experience: {', '.join([f'{stat.name}: {exp}' for stat, exp in rewards.stat_experience.items()])}")
        print(f"- Reputation: {rewards.reputation}")
        
        if rewards.items:
            print(f"- Items: {', '.join([f'{item.name} x{quantity}' for item, quantity in rewards.items])}")
    
    print("Activities System Test Complete!")


def test_lodge():
    """Test the Zoologist's Lodge system."""
    print("\nTesting Zoologist's Lodge System...")
    
    # Create a lodge
    lodge = create_example_lodge()
    
    # Get all pets in the lodge
    print("Pets in the lodge:")
    for pet_id, pet in lodge.pets.items():
        print(f"- {pet.name} ({pet.species}), Level {pet.level}, Happiness: {pet.happiness}")
    
    # Get all caregiver offers
    print("\nCaregiver offers:")
    for offer_id, offer in lodge.caregiver_offers.items():
        print(f"- {offer.player_id}: {offer.fee_per_day} BITS per day, Dominant Trait: {offer.dominant_trait.name}")
    
    # Create a lodging contract
    if lodge.pets and lodge.caregiver_offers:
        pet_id = next(iter(lodge.pets))
        caregiver_offer_id = next(iter(lodge.caregiver_offers))
        duration_days = 1
        
        print(f"\nCreating lodging contract for pet {pet_id} with caregiver offer {caregiver_offer_id}...")
        contract = lodge.create_lodging_contract(pet_id, caregiver_offer_id, duration_days)
        
        if contract:
            print(f"Contract created: {contract.id}")
            print(f"- Pet ID: {contract.pet_id}")
            print(f"- Owner ID: {contract.owner_id}")
            print(f"- Caregiver ID: {contract.caregiver_id}")
            print(f"- Daily Fee: {contract.daily_fee} BITS")
            print(f"- Start Time: {time.ctime(contract.start_time)}")
            print(f"- End Time: {time.ctime(contract.end_time)}")
            
            # Perform a care activity
            print("\nPerforming care activity...")
            care_activity = lodge.perform_care_activity(contract.caregiver_id, contract.pet_id, CareActivityType.FEED)
            
            if care_activity:
                print(f"Care activity performed: {care_activity.activity_type.name}")
                print(f"- Happiness Gain: {care_activity.happiness_gain}")
                print(f"- Stat Gains: {care_activity.stat_gains}")
                
                # Get the pet's updated state
                pet = lodge.get_pet(contract.pet_id)
                if pet:
                    print(f"\n{pet.name}'s updated state:")
                    print(f"- Happiness: {pet.happiness}")
                    print(f"- Stats: {pet.stats}")
                    print(f"- Temporary Trait Boosts: {[(trait.name, boost, time.ctime(expiry)) for trait, (boost, expiry) in pet.temporary_trait_boosts.items()]}")
        else:
            print("Failed to create lodging contract.")
    
    print("Zoologist's Lodge System Test Complete!")


if __name__ == "__main__":
    test_activities()
    test_lodge()