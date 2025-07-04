# pet/integrated_demo.py
"""
Demonstration script for the integrated CritterCraft system.

This script showcases the core functionality of the integrated pet care and critter creation system
following the KISS principles:
- K (Know Your Core, Keep it Clear): Clear separation of concerns with well-defined interfaces
- I (Iterate Intelligently): Structured for easy updates and maintenance
- S (Systematize for Scalability): Modular design with clear interfaces
- S (Sense the Landscape & Stimulate Engagement): Designed for user engagement
"""

import os
import time
import random
from typing import Dict, Any, List, Optional

from config import (
    Stat, GenesisPetConfig, CritterCraftConfig
)

from integrated_core import (
    IntegratedPet, IntegratedPetManager, IntegratedPetPersistence,
    MaterialType, AdaptationType, BodyPosition, Environment
)

# --- Demo Configuration ---
SAVE_DIRECTORY = "pet_data"
DEFAULT_PET_FILE = f"{SAVE_DIRECTORY}/integrated_pet.json"

# --- Helper Functions ---
def clear_screen():
    """Clear the terminal screen."""
    os.system('cls' if os.name == 'nt' else 'clear')

def print_section_header(title):
    """Print a section header for the demo."""
    print("\n" + "=" * 60)
    print(f" {title} ".center(60, "="))
    print("=" * 60)

def pause():
    """Pause for user input."""
    input("\nPress Enter to continue...")

def get_menu_choice(options: List[str], prompt: str = "Choose an option: ") -> int:
    """
    Display a menu and get the user's choice.
    
    Args:
        options: List of menu options
        prompt: Prompt to display
        
    Returns:
        The index of the chosen option
    """
    while True:
        print()
        for i, option in enumerate(options, 1):
            print(f"{i}. {option}")
        print()
        
        try:
            choice = int(input(prompt))
            if 1 <= choice <= len(options):
                return choice - 1
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")

def create_new_pet() -> IntegratedPet:
    """
    Create a new integrated pet based on user input.
    
    Returns:
        A new IntegratedPet instance
    """
    clear_screen()
    print_section_header("Create a New Pet")
    
    # Get pet name
    while True:
        name = input("Enter a name for your pet (1-20 characters): ").strip()
        if 1 <= len(name) <= 20:
            break
        print("Name must be 1-20 characters.")
    
    # Choose species
    print("\nAvailable Species:")
    species_options = list(GenesisPetConfig.Archetypes.DEFINITIONS.keys())
    for i, species in enumerate(species_options, 1):
        display_name = GenesisPetConfig.Archetypes.DEFINITIONS[species]['display_name']
        print(f"{i}. {display_name}")
    
    while True:
        try:
            species_choice = int(input("\nChoose a species (number): "))
            if 1 <= species_choice <= len(species_options):
                species = species_options[species_choice - 1]
                break
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")
    
    # Choose aura color
    print("\nAvailable Aura Colors:")
    aura_options = list(GenesisPetConfig.Auras.DEFINITIONS.keys())
    for i, aura in enumerate(aura_options, 1):
        display_name = GenesisPetConfig.Auras.DEFINITIONS[aura]['display_name']
        print(f"{i}. {display_name}")
    
    while True:
        try:
            aura_choice = int(input("\nChoose an aura color (number): "))
            if 1 <= aura_choice <= len(aura_options):
                aura_color = aura_options[aura_choice - 1]
                break
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")
    
    # Create the pet
    try:
        pet = IntegratedPet(
            name=name,
            species=species,
            aura_color=aura_color
        )
        print(f"\nSuccessfully created {pet.name}!")
        return pet
    except Exception as e:
        print(f"Error creating pet: {e}")
        return create_new_pet()  # Try again

def save_pet(pet: IntegratedPet, filename: str = DEFAULT_PET_FILE) -> bool:
    """
    Save the pet to a file.
    
    Args:
        pet: The pet to save
        filename: The file to save to
        
    Returns:
        True if successful, False otherwise
    """
    # Create the save directory if it doesn't exist
    os.makedirs(os.path.dirname(filename), exist_ok=True)
    
    # Save the pet
    success = IntegratedPetPersistence.save_to_file(pet, filename)
    
    if success:
        print(f"Pet saved to {filename}")
    else:
        print(f"Failed to save pet to {filename}")
    
    return success

def load_pet(filename: str = DEFAULT_PET_FILE) -> Optional[IntegratedPet]:
    """
    Load a pet from a file.
    
    Args:
        filename: The file to load from
        
    Returns:
        The loaded pet, or None if loading failed
    """
    if not os.path.exists(filename):
        print(f"No pet file found at {filename}")
        return None
    
    pet = IntegratedPetPersistence.load_from_file(filename)
    
    if pet:
        print(f"Loaded {pet.name} from {filename}")
    else:
        print(f"Failed to load pet from {filename}")
    
    return pet

def add_base_animal(pet_manager: IntegratedPetManager) -> bool:
    """
    Add a base animal to the pet.
    
    Args:
        pet_manager: The pet manager
        
    Returns:
        True if successful, False otherwise
    """
    # Check if the pet already has a base animal
    if pet_manager.pet.base_animal:
        print(f"Your pet already has a base animal: {pet_manager.pet.base_animal}")
        change = input("Do you want to change it? (y/n): ").lower()
        if change != 'y':
            return False
    
    # Choose base animal
    print("\nAvailable Base Animals:")
    animal_options = list(CritterCraftConfig.CRITTER_TYPES.keys())
    for i, animal in enumerate(animal_options, 1):
        display_name = CritterCraftConfig.CRITTER_TYPES[animal]['display_name']
        print(f"{i}. {display_name}")
    
    while True:
        try:
            animal_choice = int(input("\nChoose a base animal (number): "))
            if 1 <= animal_choice <= len(animal_options):
                base_animal = animal_options[animal_choice - 1]
                break
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")
    
    # Set the base animal
    pet_manager.pet.base_animal = base_animal
    print(f"Base animal set to {CritterCraftConfig.CRITTER_TYPES[base_animal]['display_name']}")
    
    return True

def add_material(pet_manager: IntegratedPetManager) -> bool:
    """
    Add a material to the pet's critter form.
    
    Args:
        pet_manager: The pet manager
        
    Returns:
        True if successful, False otherwise
    """
    # Check if the pet has a base animal
    if not pet_manager.pet.base_animal:
        print("You need to set a base animal first.")
        return False
    
    # Choose material type
    print("\nAvailable Material Types:")
    material_options = [m.value for m in MaterialType]
    for i, material in enumerate(material_options, 1):
        print(f"{i}. {material.capitalize()}")
    
    while True:
        try:
            material_choice = int(input("\nChoose a material type (number): "))
            if 1 <= material_choice <= len(material_options):
                material_type = material_options[material_choice - 1]
                break
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")
    
    # Choose color
    material_config = CritterCraftConfig.CRAFTING_MATERIALS.get(material_type, {})
    color_options = material_config.get('colors', ['brown', 'white', 'black', 'gray'])
    
    print("\nAvailable Colors:")
    for i, color in enumerate(color_options, 1):
        print(f"{i}. {color.capitalize()}")
    
    while True:
        try:
            color_choice = int(input("\nChoose a color (number): "))
            if 1 <= color_choice <= len(color_options):
                color = color_options[color_choice - 1]
                break
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")
    
    # Choose coverage
    while True:
        try:
            coverage = float(input("\nEnter coverage (0.0-1.0): "))
            if 0.0 <= coverage <= 1.0:
                break
            print("Coverage must be between 0.0 and 1.0.")
        except ValueError:
            print("Please enter a number.")
    
    # Choose position
    print("\nAvailable Positions:")
    position_options = [p.value for p in BodyPosition]
    for i, position in enumerate(position_options, 1):
        print(f"{i}. {position.capitalize()}")
    
    while True:
        try:
            position_choice = int(input("\nChoose a position (number): "))
            if 1 <= position_choice <= len(position_options):
                position = position_options[position_choice - 1]
                break
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")
    
    # Add the material
    success = pet_manager.add_material(material_type, color, coverage, position)
    
    if success:
        print(f"Added {color} {material_type} to {position}")
    else:
        print(f"Failed to add material. Make sure it's unlocked for your zoologist level.")
    
    return success

def add_adaptation(pet_manager: IntegratedPetManager) -> bool:
    """
    Add an adaptation to the pet's critter form.
    
    Args:
        pet_manager: The pet manager
        
    Returns:
        True if successful, False otherwise
    """
    # Check if the pet has a base animal
    if not pet_manager.pet.base_animal:
        print("You need to set a base animal first.")
        return False
    
    # Choose adaptation type
    print("\nAvailable Adaptation Types:")
    adaptation_options = [a.value for a in AdaptationType]
    for i, adaptation in enumerate(adaptation_options, 1):
        print(f"{i}. {adaptation.capitalize()}")
    
    while True:
        try:
            adaptation_choice = int(input("\nChoose an adaptation type (number): "))
            if 1 <= adaptation_choice <= len(adaptation_options):
                adaptation_type = adaptation_options[adaptation_choice - 1]
                break
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")
    
    # Choose strength
    while True:
        try:
            strength = int(input("\nEnter strength (1-10): "))
            if 1 <= strength <= 10:
                break
            print("Strength must be between 1 and 10.")
        except ValueError:
            print("Please enter a number.")
    
    # Choose position
    print("\nAvailable Positions:")
    position_options = [p.value for p in BodyPosition]
    for i, position in enumerate(position_options, 1):
        print(f"{i}. {position.capitalize()}")
    
    while True:
        try:
            position_choice = int(input("\nChoose a position (number): "))
            if 1 <= position_choice <= len(position_options):
                position = position_options[position_choice - 1]
                break
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")
    
    # Add the adaptation
    success = pet_manager.add_adaptation(adaptation_type, strength, position)
    
    if success:
        print(f"Added {adaptation_type} (strength {strength}) to {position}")
    else:
        print(f"Failed to add adaptation. Make sure it's unlocked for your zoologist level.")
    
    return success

def simulate_environment(pet_manager: IntegratedPetManager) -> bool:
    """
    Simulate the pet in an environment.
    
    Args:
        pet_manager: The pet manager
        
    Returns:
        True if successful, False otherwise
    """
    # Check if the pet has a base animal
    if not pet_manager.pet.base_animal:
        print("You need to set a base animal first.")
        return False
    
    # Choose environment
    print("\nAvailable Environments:")
    environment_options = [e.value for e in Environment]
    for i, environment in enumerate(environment_options, 1):
        print(f"{i}. {environment.capitalize()}")
    
    while True:
        try:
            environment_choice = int(input("\nChoose an environment (number): "))
            if 1 <= environment_choice <= len(environment_options):
                environment = environment_options[environment_choice - 1]
                break
            print("Invalid choice. Please try again.")
        except ValueError:
            print("Please enter a number.")
    
    # Simulate the environment
    try:
        results = pet_manager.simulate_in_environment(environment)
        
        # Display the results
        print_section_header(f"Simulation Results: {environment.capitalize()}")
        print(f"Survival Score: {results['survival_score']}/100")
        
        print("\nAdvantages:")
        if results['advantages']:
            for advantage in results['advantages']:
                print(f"- {advantage}")
        else:
            print("- None")
        
        print("\nDisadvantages:")
        if results['disadvantages']:
            for disadvantage in results['disadvantages']:
                print(f"- {disadvantage}")
        else:
            print("- None")
        
        if 'educational_fact' in results:
            print(f"\nEducational Fact: {results['educational_fact']}")
        
        return True
    except Exception as e:
        print(f"Error simulating environment: {e}")
        return False

def demo():
    """Main function for the integrated demo."""
    # Try to load an existing pet, or create a new one
    pet = load_pet()
    if not pet:
        pet = create_new_pet()
        save_pet(pet)
    
    # Create a pet manager
    pet_manager = IntegratedPetManager(pet)
    
    # Main loop
    while True:
        clear_screen()
        print_section_header(f"CritterCraft Integrated Demo - {pet.name}")
        
        # Display pet status
        print(pet_manager.get_status_report())
        
        # Main menu
        options = [
            "Feed Pet",
            "Play with Pet",
            "Chat with Pet",
            "Groom Pet",
            "Set Base Animal",
            "Add Material",
            "Add Adaptation",
            "Simulate Environment",
            "Save Pet",
            "Exit"
        ]
        
        choice = get_menu_choice(options)
        
        if choice == 0:  # Feed Pet
            result = pet_manager.feed()
            print(result)
            pause()
        
        elif choice == 1:  # Play with Pet
            result = pet_manager.play()
            print(result)
            pause()
        
        elif choice == 2:  # Chat with Pet
            message = input("\nEnter your message: ")
            result = pet_manager.chat(message)
            print(f"\n{result}")
            pause()
        
        elif choice == 3:  # Groom Pet
            result = pet_manager.groom()
            print(result)
            pause()
        
        elif choice == 4:  # Set Base Animal
            add_base_animal(pet_manager)
            pause()
        
        elif choice == 5:  # Add Material
            add_material(pet_manager)
            pause()
        
        elif choice == 6:  # Add Adaptation
            add_adaptation(pet_manager)
            pause()
        
        elif choice == 7:  # Simulate Environment
            simulate_environment(pet_manager)
            pause()
        
        elif choice == 8:  # Save Pet
            save_pet(pet_manager.pet)
            pause()
        
        elif choice == 9:  # Exit
            save_pet(pet_manager.pet)
            print("\nThank you for playing! Goodbye!")
            break
        
        # Apply time decay
        pet_manager.tick()

if __name__ == "__main__":
    try:
        demo()
    except KeyboardInterrupt:
        print("\n\nExiting...")
    except Exception as e:
        print(f"\nAn error occurred: {e}")