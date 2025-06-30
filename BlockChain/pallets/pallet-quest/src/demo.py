"""
Demo script for the Activities and Zoologist's Lodge systems.

This script demonstrates the activities and Zoologist's Lodge systems in Critter-Craft.
"""

import sys
import os
import time
import random
from typing import Dict, List, Optional

# Add the parent directory to the Python path to import from other pallets
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-economy', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-battles', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-ledger', 'src'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'pallet-breeding', 'src'))

# Import from economy system
from currencies import Bits, Aura
from inventory import Inventory

# Import from activities system
from activities import (
    ActivityManager,
    create_example_activities,
    ActivityType,
    StatType,
    LogicLeaper,
    AuraWeaving,
    HabitatDash,
    CritterTactics,
    CooperativeCrafting,
    CrystalMining,
    BioluminescentGuide,
    HerbalistAssistant,
    Quest,
    AdventurousQuest
)

# Import from lodge system
from lodge import (
    ZoologistLodge,
    create_example_lodge,
    PetState,
    CaregiverOffer,
    LodgingContract,
    CareActivity,
    PersonalityTrait
)


def run_demo():
    """Run a demo of the Activities and Zoologist's Lodge systems."""
    print("Welcome to the Activities and Zoologist's Lodge Demo!")
    print("=" * 60)
    time.sleep(1)
    
    # Create example activities
    activity_manager = create_example_activities()
    
    # Create example lodge
    lodge = create_example_lodge()
    
    # Create player data
    player_id = "player1"
    player_level = 10
    player_inventory = Inventory(player_id=player_id)
    
    print(f"Player ID: {player_id}")
    print(f"Player Level: {player_level}")
    print()
    
    # Demo Mini-Games
    print("Mini-Games Demo")
    print("-" * 30)
    
    # Get all mini-games
    mini_games = activity_manager.get_activities_by_type(ActivityType.MINI_GAME)
    
    print(f"Available Mini-Games: {len(mini_games)}")
    for i, mini_game in enumerate(mini_games):
        print(f"{i+1}. {mini_game.name}: {mini_game.description}")
    
    # Play a mini-game
    if mini_games:
        mini_game = mini_games[0]  # Logic Leaper
        print(f"\nPlaying {mini_game.name}...")
        
        # Simulate playing the mini-game
        score = random.randint(50, 100)
        print(f"Score: {score}")
        
        # Complete the mini-game and get rewards
        rewards = activity_manager.complete_activity(player_id, mini_game.id, score, player_level)
        
        print("Rewards:")
        print(f"- BITS: {rewards.bits}")
        print(f"- AURA: {rewards.aura}")
        print(f"- Stat Experience: {', '.join([f'{stat.name}: {exp}' for stat, exp in rewards.stat_experience.items()])}")
        print(f"- Reputation: {rewards.reputation}")
        
        if rewards.items:
            print(f"- Items: {', '.join([f'{item.name} x{quantity}' for item, quantity in rewards.items])}")
            
            # Add the items to the player's inventory
            for item, quantity in rewards.items:
                player_inventory.add_item(item, quantity)
    
    print()
    
    # Demo Two-Player Games
    print("Two-Player Games Demo")
    print("-" * 30)
    
    # Get all two-player games
    two_player_games = activity_manager.get_activities_by_type(ActivityType.TWO_PLAYER_GAME)
    
    print(f"Available Two-Player Games: {len(two_player_games)}")
    for i, two_player_game in enumerate(two_player_games):
        print(f"{i+1}. {two_player_game.name}: {two_player_game.description}")
    
    # Play a two-player game
    if two_player_games:
        two_player_game = two_player_games[0]  # Critter Tactics
        print(f"\nPlaying {two_player_game.name}...")
        
        # Simulate playing the two-player game
        score = random.randint(80, 150)
        print(f"Score: {score}")
        
        # Complete the two-player game and get rewards
        rewards = activity_manager.complete_activity(player_id, two_player_game.id, score, player_level)
        
        print("Rewards:")
        print(f"- BITS: {rewards.bits}")
        print(f"- AURA: {rewards.aura}")
        print(f"- Stat Experience: {', '.join([f'{stat.name}: {exp}' for stat, exp in rewards.stat_experience.items()])}")
        print(f"- Reputation: {rewards.reputation}")
        
        if rewards.items:
            print(f"- Items: {', '.join([f'{item.name} x{quantity}' for item, quantity in rewards.items])}")
            
            # Add the items to the player's inventory
            for item, quantity in rewards.items:
                player_inventory.add_item(item, quantity)
    
    print()
    
    # Demo Jobs
    print("Jobs Demo")
    print("-" * 30)
    
    # Get all jobs
    jobs = activity_manager.get_activities_by_type(ActivityType.JOB)
    
    print(f"Available Jobs: {len(jobs)}")
    for i, job in enumerate(jobs):
        print(f"{i+1}. {job.name}: {job.description}")
    
    # Perform a job
    if jobs:
        job = jobs[0]  # Crystal Mining
        print(f"\nPerforming {job.name}...")
        
        # Simulate performing the job
        score = random.randint(40, 80)
        print(f"Score: {score}")
        
        # Complete the job and get rewards
        rewards = activity_manager.complete_activity(player_id, job.id, score, player_level)
        
        print("Rewards:")
        print(f"- BITS: {rewards.bits}")
        print(f"- AURA: {rewards.aura}")
        print(f"- Stat Experience: {', '.join([f'{stat.name}: {exp}' for stat, exp in rewards.stat_experience.items()])}")
        print(f"- Reputation: {rewards.reputation}")
        
        if rewards.items:
            print(f"- Items: {', '.join([f'{item.name} x{quantity}' for item, quantity in rewards.items])}")
            
            # Add the items to the player's inventory
            for item, quantity in rewards.items:
                player_inventory.add_item(item, quantity)
    
    print()
    
    # Demo Quests
    print("Quests Demo")
    print("-" * 30)
    
    # Get all quests
    quests = activity_manager.get_activities_by_type(ActivityType.QUEST)
    
    print(f"Available Quests: {len(quests)}")
    for i, quest in enumerate(quests):
        print(f"{i+1}. {quest.name}: {quest.description}")
        print(f"   Objectives: {', '.join(quest.objectives)}")
    
    # Complete a quest
    if quests:
        quest = quests[0]  # Gathering Quest
        print(f"\nCompleting {quest.name}...")
        
        # Simulate completing the quest
        score = 100  # Fixed score for quests
        
        # Complete the quest and get rewards
        rewards = activity_manager.complete_activity(player_id, quest.id, score, player_level)
        
        print("Rewards:")
        print(f"- BITS: {rewards.bits}")
        print(f"- AURA: {rewards.aura}")
        print(f"- Stat Experience: {', '.join([f'{stat.name}: {exp}' for stat, exp in rewards.stat_experience.items()])}")
        print(f"- Reputation: {rewards.reputation}")
        
        if rewards.items:
            print(f"- Items: {', '.join([f'{item.name} x{quantity}' for item, quantity in rewards.items])}")
            
            # Add the items to the player's inventory
            for item, quantity in rewards.items:
                player_inventory.add_item(item, quantity)
    
    print()
    
    # Demo Adventurous Quests
    print("Adventurous Quests Demo")
    print("-" * 30)
    
    # Get all adventurous quests
    adventurous_quests = activity_manager.get_activities_by_type(ActivityType.ADVENTUROUS_QUEST)
    
    print(f"Available Adventurous Quests: {len(adventurous_quests)}")
    for i, adventurous_quest in enumerate(adventurous_quests):
        print(f"{i+1}. {adventurous_quest.name}: {adventurous_quest.description}")
        print(f"   Stages: {len(adventurous_quest.stages)}")
        for j, stage in enumerate(adventurous_quest.stages):
            print(f"     {j+1}. {stage['name']}: {stage['description'][:100]}...")
    
    # Complete an adventurous quest
    if adventurous_quests:
        adventurous_quest = adventurous_quests[0]  # The Whispering Blight
        print(f"\nCompleting {adventurous_quest.name}...")
        
        # Simulate completing the adventurous quest
        score = 200  # Fixed score for adventurous quests
        
        # Complete the adventurous quest and get rewards
        rewards = activity_manager.complete_activity(player_id, adventurous_quest.id, score, player_level)
        
        print("Rewards:")
        print(f"- BITS: {rewards.bits}")
        print(f"- AURA: {rewards.aura}")
        print(f"- Stat Experience: {', '.join([f'{stat.name}: {exp}' for stat, exp in rewards.stat_experience.items()])}")
        print(f"- Reputation: {rewards.reputation}")
        
        if rewards.items:
            print(f"- Items: {', '.join([f'{item.name} x{quantity}' for item, quantity in rewards.items])}")
            
            # Add the items to the player's inventory
            for item, quantity in rewards.items:
                player_inventory.add_item(item, quantity)
    
    print()
    
    # Demo Zoologist's Lodge
    print("Zoologist's Lodge Demo")
    print("-" * 30)
    
    # Get all pets in the lodge
    print("Pets in the Zoologist's Lodge:")
    for pet in lodge.pets.values():
        print(f"- {pet.name} ({pet.species}), Level {pet.level}, Happiness: {pet.happiness}")
    
    # Get all caregiver offers
    print("\nCaregiver Offers:")
    for offer in lodge.caregiver_offers.values():
        print(f"- {offer.player_id}: {offer.fee} BITS per day, Dominant Trait: {offer.dominant_trait.name}, Reputation: {offer.reputation}")
    
    # Create a lodging contract
    pet_id = "pet1"
    caregiver_offer_id = list(lodge.caregiver_offers.keys())[0]
    duration = 86400  # 1 day
    
    print(f"\nCreating a lodging contract for {lodge.pets[pet_id].name}...")
    contract = lodge.create_lodging_contract(pet_id, caregiver_offer_id, duration)
    
    if contract:
        print(f"Created lodging contract with caregiver {contract.caregiver_id}")
        print(f"Fee: {contract.fee} BITS, Duration: {(contract.end_time - contract.start_time) // 3600} hours")
        
        # Perform some care activities
        caregiver_id = contract.caregiver_id
        
        print("\nPerforming care activities...")
        feed_activity = lodge.perform_care_activity(caregiver_id, pet_id, "feed")
        play_activity = lodge.perform_care_activity(caregiver_id, pet_id, "play")
        groom_activity = lodge.perform_care_activity(caregiver_id, pet_id, "groom")
        
        print("Care Activities:")
        for activity in lodge.get_care_activities(pet_id):
            print(f"- {activity.activity_type.capitalize()}: +{activity.happiness_gain} happiness, Stats: {activity.stat_gains}")
        
        # Print the pet's updated state
        pet = lodge.get_pet(pet_id)
        print(f"\n{pet.name}'s Updated State:")
        print(f"Happiness: {pet.happiness}")
        print(f"Stats: {pet.stats}")
        print(f"Temporary Trait Boosts: {[(trait.name, boost, time.ctime(expiry)) for trait, (boost, expiry) in pet.temporary_trait_boosts.items()]}")
    else:
        print("Failed to create lodging contract.")
    
    print()
    
    # Print player's inventory
    print("Player's Inventory:")
    for slot_id, slot in player_inventory.slots.items():
        print(f"- {slot.item.name} x{slot.quantity}")
    
    print("\nThank you for trying the Activities and Zoologist's Lodge Demo!")


if __name__ == "__main__":
    run_demo()
