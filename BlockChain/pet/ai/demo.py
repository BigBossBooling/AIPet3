# pet/demo.py
"""
Demonstration script for the CritterCraft Genesis Pet system.
This script shows how to create, interact with, and save/load a pet.
"""

import os
import time
import json
from pet_core import Pet
from config import (
    PET_ARCHETYPES, PET_AURA_COLORS, MIGRATION_READINESS_THRESHOLDS,
    MAX_STAT
)

def clear_screen():
    """Clear the terminal screen."""
    os.system('cls' if os.name == 'nt' else 'clear')

def print_separator():
    """Print a separator line."""
    print("=" * 50)

def demo_create_pet():
    """Demonstrate creating a new pet."""
    print_separator()
    print("CREATING A NEW PET")
    print_separator()
    
    # Create a pet with default values
    pet = Pet(
        name="Sparkle",
        species="sprite_glow",
        aura_color="aura-blue"
    )
    
    print(f"Created a new pet named {pet.name}!")
    print(pet.status())
    return pet

def demo_interact_with_pet(pet):
    """Demonstrate interacting with a pet."""
    print_separator()
    print("INTERACTING WITH PET")
    print_separator()
    
    # Feed the pet
    print("Feeding the pet...")
    pet.feed()
    print(f"{pet.name}'s hunger is now {pet.hunger}")
    
    # Play with the pet
    print("\nPlaying with the pet...")
    success, message = pet.play()
    print(message)
    
    # Show updated status
    print("\nUpdated pet status:")
    print(pet.status())

def demo_chat_with_pet(pet):
    """Demonstrate chatting with a pet."""
    print_separator()
    print("CHATTING WITH PET")
    print_separator()
    
    # Chat with the pet using different types of messages
    print("Greeting the pet...")
    success, message = pet.chat("Hello there! How are you today?")
    print(message)
    
    print("\nComplimenting the pet...")
    success, message = pet.chat("You're such a smart and beautiful pet!")
    print(message)
    
    print("\nTeaching the pet...")
    success, message = pet.chat("Let's learn about science and mathematics today! Did you know that E=mc²?")
    print(message)
    
    # Show updated status with IQ changes
    print("\nUpdated pet status after chatting:")
    print(f"Intelligence: {pet.iq}/{MAX_STAT}")
    print(f"Happiness: {pet.happiness}/{MAX_STAT}")
    print(f"Energy: {pet.energy}/{MAX_STAT}")
    print(f"Mood: {pet.mood}")

def demo_time_passage(pet):
    """Demonstrate the passage of time."""
    print_separator()
    print("SIMULATING TIME PASSAGE")
    print_separator()
    
    print("Pet status before time passage:")
    print(f"Hunger: {pet.hunger}, Energy: {pet.energy}, Happiness: {pet.happiness}")
    
    # Simulate time passage (12 hours)
    print("\nSimulating 12 hours of time passage...")
    current_time = time.time_ns()
    # Manually set last_active_timestamp to 12 hours ago
    pet.last_active_timestamp = current_time - (12 * 3600 * 1_000_000_000)
    pet.tick(current_time)
    
    print("Pet status after time passage:")
    print(f"Hunger: {pet.hunger}, Energy: {pet.energy}, Happiness: {pet.happiness}")
    print(f"Mood: {pet.mood}")

def demo_save_load_pet(pet):
    """Demonstrate saving and loading a pet."""
    print_separator()
    print("SAVING AND LOADING PET")
    print_separator()
    
    # Save the pet to a file
    save_file = "demo_pet.json"
    print(f"Saving pet to {save_file}...")
    with open(save_file, 'w') as f:
        f.write(pet.to_json())
    
    # Load the pet from the file
    print(f"Loading pet from {save_file}...")
    with open(save_file, 'r') as f:
        loaded_pet = Pet.from_json(f.read())
    
    print("Loaded pet status:")
    print(loaded_pet.status())
    
    # Clean up the file
    os.remove(save_file)
    print(f"Removed {save_file}")

def demo_migration_readiness(pet):
    """Demonstrate checking migration readiness."""
    print_separator()
    print("CHECKING MIGRATION READINESS")
    print_separator()
    
    # Calculate days owned
    current_time_ns = time.time_ns()
    days_owned = (current_time_ns - pet.creation_timestamp) / (1_000_000_000 * 60 * 60 * 24)
    
    # Count interactions
    interaction_count = len(pet.interaction_history)
    
    # Check all conditions
    thresholds = MIGRATION_READINESS_THRESHOLDS
    ready = (
        pet.happiness >= thresholds['min_happiness'] and
        pet.energy >= thresholds['min_energy'] and
        pet.hunger <= thresholds['max_hunger'] and
        interaction_count >= thresholds['min_interactions'] and
        days_owned >= thresholds['min_days_owned']
    )
    
    print(f"Migration Readiness Check for {pet.name}:")
    print(f"Happiness: {pet.happiness}/{thresholds['min_happiness']} required")
    print(f"Energy: {pet.energy}/{thresholds['min_energy']} required")
    print(f"Hunger: {pet.hunger}/{thresholds['max_hunger']} maximum")
    print(f"Interactions: {interaction_count}/{thresholds['min_interactions']} required")
    print(f"Days owned: {days_owned:.2f}/{thresholds['min_days_owned']} required")
    print(f"\nMigration ready: {'Yes' if ready else 'No'}")

def run_demo():
    """Run the full demonstration."""
    clear_screen()
    print("CRITTERCRAFT GENESIS PET DEMONSTRATION")
    print_separator()
    
    # Create a pet
    pet = demo_create_pet()
    input("\nPress Enter to continue...")
    
    # Interact with the pet
    demo_interact_with_pet(pet)
    input("\nPress Enter to continue...")
    
    # Chat with the pet
    demo_chat_with_pet(pet)
    input("\nPress Enter to continue...")
    
    # Demonstrate time passage
    demo_time_passage(pet)
    input("\nPress Enter to continue...")
    
    # Demonstrate save/load
    demo_save_load_pet(pet)
    input("\nPress Enter to continue...")
    
    # Check migration readiness
    demo_migration_readiness(pet)
    
    print_separator()
    print("DEMONSTRATION COMPLETE")
    print_separator()

if __name__ == "__main__":
    run_demo()