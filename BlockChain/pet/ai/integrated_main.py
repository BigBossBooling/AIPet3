# pet/integrated_main.py
"""
Main application for CritterCraft, combining pet care and critter creation.
This is the command-line interface for interacting with your virtual pet and creating critters.
"""

import os
import time
import json
from integrated_core import IntegratedPet, IntegratedPetManager, save_integrated_pet, load_integrated_pet
from ai_integration import AIIntegrationManager
from config import (
    PET_ARCHETYPES,
    PET_AURA_COLORS,
    CRITTER_TYPES,
    CRAFTING_MATERIALS,
    ADAPTATIONS,
    MIGRATION_READINESS_THRESHOLDS,
    ZOOLOGIST_LEVELS
)

# --- Configuration Constants ---
SAVE_FILE_PATH = "integrated_pet_data.json"
CRITTER_SAVE_DIR = "critter_data/critters"
MAX_PET_NAME_LENGTH = 20

# --- Menu Choices ---
# Main Menu
MENU_PET_CARE = '1'
MENU_CRITTER_CRAFT = '2'
MENU_CHAT = '3'
MENU_CHECK_STATUS = '4'
MENU_CHECK_MIGRATION = '5'
MENU_SAVE_EXIT = '6'

# Pet Care Menu
CARE_FEED = '1'
CARE_PLAY = '2'
CARE_CHAT = '3'
CARE_GROOM = '4'
CARE_BACK = '5'

# Critter Craft Menu
CRAFT_CREATE = '1'
CRAFT_MODIFY = '2'
CRAFT_SIMULATE = '3'
CRAFT_VIEW_JOURNAL = '4'
CRAFT_BACK = '5'

# --- Utility Functions ---
def clear_screen():
    """Clear the terminal screen."""
    os.system('cls' if os.name == 'nt' else 'clear')

def print_header():
    """Print the application header."""
    print("=" * 60)
    print("  CritterCraft - Virtual Pet & Creature Creation")
    print("=" * 60)
    print()

def get_valid_input(prompt, validation_func, error_message):
    """Generic function to get validated input from the user."""
    while True:
        user_input = input(prompt).strip()
        if validation_func(user_input):
            return user_input
        print(error_message)

def get_choice_from_dict(prompt_text, data_dict, show_description=True, show_rarity=False):
    """Displays choices from a dictionary and gets a valid integer choice."""
    items = list(data_dict.items())
    for i, (key, value) in enumerate(items, 1):
        display_name = value.get('display_name', key)
        description = value.get('description', '')
        rarity = value.get('rarity', '')
        effect = value.get('effect', '')

        print(f"{i}. {display_name}", end="")
        if show_rarity and rarity:
            print(f" ({rarity})", end="")
        print()
        if show_description and description:
            print(f"   {description}")
        if effect:
            print(f"   Effect: {effect}")

    while True:
        try:
            choice = int(input(prompt_text))
            if 1 <= choice <= len(items):
                return items[choice - 1][0]  # Return the key
            print(f"Please enter a number between 1 and {len(items)}.")
        except ValueError:
            print("Invalid input. Please enter a number.")

def ensure_save_directories():
    """Ensure that save directories exist."""
    # Only create directory if SAVE_FILE_PATH has a directory component
    if os.path.dirname(SAVE_FILE_PATH):
        os.makedirs(os.path.dirname(SAVE_FILE_PATH), exist_ok=True)
    os.makedirs(CRITTER_SAVE_DIR, exist_ok=True)

# --- Pet Creation and Management Functions ---
def create_new_pet():
    """Create a new pet with user input."""
    clear_screen()
    print_header()
    print("Let's create your new digital companion!")
    print()

    # Get pet name
    name = get_valid_input(
        "What would you like to name your pet? ",
        lambda n: n and 1 <= len(n) <= MAX_PET_NAME_LENGTH and all(c.isprintable() for c in n),
        f"Please enter a valid name (1-{MAX_PET_NAME_LENGTH} printable characters)."
    )

    print("\nAvailable Pet Species:")
    species = get_choice_from_dict(
        "\nSelect a species: ", PET_ARCHETYPES, show_rarity=True
    )

    print("\nAvailable Aura Colors:")
    aura_color = get_choice_from_dict(
        "\nSelect an aura color: ", PET_AURA_COLORS
    )

    # Create the pet
    pet = IntegratedPet(name=name, species=species, aura_color=aura_color)
    print(f"\nCongratulations! {pet.name} the {PET_ARCHETYPES[species]['display_name']} has been created with a {PET_AURA_COLORS[aura_color]['display_name']} aura!")
    return pet

def load_pet():
    """Load a pet from the save file if it exists."""
    if not os.path.exists(SAVE_FILE_PATH):
        return None, None

    try:
        pet, ai_data = load_integrated_pet(SAVE_FILE_PATH)
        return pet, ai_data
    except json.JSONDecodeError:
        print(f"Error: Corrupted pet data in {SAVE_FILE_PATH}. Starting new pet.")
        return None, None
    except Exception as e:
        print(f"An unexpected error occurred while loading pet: {e}. Starting new pet.")
        return None, None

def save_pet(pet, pet_manager):
    """Save the pet to a file."""
    ensure_save_directories()
    try:
        save_integrated_pet(pet, pet_manager, SAVE_FILE_PATH)
        print(f"\n{pet.name} has been saved successfully!")
    except Exception as e:
        print(f"Error saving {pet.name}: {e}")

# --- Pet Care Menu Functions ---
def pet_care_menu(pet_manager):
    """Display the pet care menu and handle user input."""
    while True:
        clear_screen()
        print_header()
        
        # Update pet stats based on time passed
        pet_manager.tick(time.time_ns())
        
        # Display pet status
        print(pet_manager.status())
        print("\n" + "=" * 20 + " Pet Care Menu " + "=" * 20)
        print(f"{CARE_FEED}. Feed {pet_manager.pet.name}")
        print(f"{CARE_PLAY}. Play with {pet_manager.pet.name}")
        print(f"{CARE_CHAT}. Chat with {pet_manager.pet.name}")
        print(f"{CARE_GROOM}. Groom {pet_manager.pet.name}")
        print(f"{CARE_BACK}. Back to Main Menu")
        print("=" * 60)
        
        choice = input("\nEnter your choice: ").strip()
        
        if choice == CARE_FEED:
            feedback_message = pet_manager.feed()
            print(f"\n{feedback_message}")
        
        elif choice == CARE_PLAY:
            success, feedback_message = pet_manager.play()
            print(f"\n{feedback_message}")
        
        elif choice == CARE_CHAT:
            print(f"\n=== Chat with {pet_manager.pet.name} ===")
            print("Available commands:")
            print("  help - Show available commands")
            print("  status - Check pet's current status")
            print("  exit - End the conversation")
            
            # Start a chat session
            chatting = True
            while chatting:
                message = input("\nYou: ")
                message_lower = message.lower().strip()
                
                # Check for special commands
                if message_lower in ['exit', 'quit', 'bye']:
                    print(f"\n{pet_manager.pet.name} waves goodbye!")
                    chatting = False
                    continue
                    
                elif message_lower == 'help':
                    print("\nAvailable commands:")
                    print("  help - Show this help message")
                    print("  status - Check pet's current status")
                    print("  exit - End the conversation")
                    continue
                    
                elif message_lower == 'status':
                    print(f"\n=== {pet_manager.pet.name}'s Status ===")
                    print(f"Mood: {pet_manager._get_current_mood()}")
                    print(f"Hunger: {pet_manager.pet.hunger}/100")
                    print(f"Energy: {pet_manager.pet.energy}/100")
                    print(f"Happiness: {pet_manager.pet.happiness}/100")
                    continue
                
                # Process the message
                if message.strip():
                    success, response_message = pet_manager.chat(message)
                    print(f"\n{pet_manager.pet.name}: {response_message}")
                    
                    # Update pet status after each interaction
                    pet_manager.tick(time.time_ns())
                else:
                    print(f"\n{pet_manager.pet.name} waits for you to say something meaningful...")
        
        elif choice == CARE_GROOM:
            feedback_message = pet_manager.groom()
            print(f"\n{feedback_message}")
        
        elif choice == CARE_BACK:
            break
        
        else:
            print("\nInvalid choice. Please select from the options above.")
        
        input("\nPress Enter to continue...")

# --- Critter Craft Menu Functions ---
def critter_craft_menu(pet_manager):
    """Display the critter craft menu and handle user input."""
    while True:
        clear_screen()
        print_header()
        
        # Display critter status if available
        if pet_manager.pet.base_animal:
            animal_info = CRITTER_TYPES.get(pet_manager.pet.base_animal, {})
            print(f"Current Critter Form: {animal_info.get('display_name', pet_manager.pet.base_animal)}")
            print(f"Materials: {len(pet_manager.pet.materials)}")
            print(f"Adaptations: {len(pet_manager.pet.adaptations)}")
            print(f"Zoologist Level: {pet_manager.pet.zoologist_level}")
        else:
            print("No critter form created yet.")
        
        print("\n" + "=" * 20 + " Critter Craft Menu " + "=" * 20)
        print(f"{CRAFT_CREATE}. Create New Critter Form")
        print(f"{CRAFT_MODIFY}. Modify Current Critter Form")
        print(f"{CRAFT_SIMULATE}. Simulate in Environment")
        print(f"{CRAFT_VIEW_JOURNAL}. View Zoologist's Journal")
        print(f"{CRAFT_BACK}. Back to Main Menu")
        print("=" * 60)
        
        choice = input("\nEnter your choice: ").strip()
        
        if choice == CRAFT_CREATE:
            create_critter_form(pet_manager)
        
        elif choice == CRAFT_MODIFY:
            if not pet_manager.pet.base_animal:
                print("\nYou need to create a critter form first!")
            else:
                modify_critter_form(pet_manager)
        
        elif choice == CRAFT_SIMULATE:
            if not pet_manager.pet.base_animal:
                print("\nYou need to create a critter form first!")
            else:
                simulate_critter(pet_manager)
        
        elif choice == CRAFT_VIEW_JOURNAL:
            view_zoologist_journal(pet_manager)
        
        elif choice == CRAFT_BACK:
            break
        
        else:
            print("\nInvalid choice. Please select from the options above.")
        
        input("\nPress Enter to continue...")

def create_critter_form(pet_manager):
    """Create a new critter form for the pet."""
    clear_screen()
    print_header()
    print("Let's create a new critter form for your pet!")
    
    # Check if pet already has a critter form
    if pet_manager.pet.base_animal:
        confirm = get_valid_input(
            "\nYour pet already has a critter form. Creating a new one will replace it. Continue? (y/n): ",
            lambda a: a.lower() in ['y', 'n', 'yes', 'no'],
            "Please enter 'y' or 'n'."
        )
        if confirm.lower() in ['n', 'no']:
            return
    
    # Select base animal
    print("\nAvailable Base Animals:")
    base_animal = get_choice_from_dict(
        "\nSelect a base animal for your critter: ", 
        CRITTER_TYPES
    )
    
    # Set the base animal
    success = pet_manager.set_base_animal(base_animal)
    if not success:
        print("\nFailed to set base animal. Please try again.")
        return
    
    # Display animal facts
    display_animal_facts(pet_manager)
    
    # Add materials
    add_materials_to_critter(pet_manager)
    
    # Add adaptations
    add_adaptations_to_critter(pet_manager)
    
    # Increment critters created count and check for level up
    pet_manager.pet.critters_created += 1
    if pet_manager.update_zoologist_level():
        print(f"\nCongratulations! You've reached Zoologist Level {pet_manager.pet.zoologist_level}!")
        print("You've unlocked new materials and adaptations!")
    
    print(f"\nCongratulations! {pet_manager.pet.name}'s critter form has been created!")
    
    # Boost stats from creating a critter
    pet_manager.pet.iq = min(100, pet_manager.pet.iq + 5)
    pet_manager.pet.happiness = min(100, pet_manager.pet.happiness + 10)
    print("\nCreating a critter has increased your pet's IQ and Happiness!")

def display_animal_facts(pet_manager):
    """Display facts about the base animal and add them to the pet's knowledge."""
    animal_info = CRITTER_TYPES.get(pet_manager.pet.base_animal, {})
    
    print(f"\n=== Facts about {animal_info.get('display_name', pet_manager.pet.base_animal)} ===")
    print(f"Habitat: {animal_info.get('habitat', 'Unknown')}")
    print(f"Diet: {animal_info.get('diet', 'Unknown')}")
    print(f"Conservation Status: {animal_info.get('conservation_status', 'Unknown')}")
    
    # Add these facts to the pet's knowledge
    pet_manager.learn_fact(f"Habitat: {animal_info.get('habitat', 'Unknown')}")
    pet_manager.learn_fact(f"Diet: {animal_info.get('diet', 'Unknown')}")
    pet_manager.learn_fact(f"Conservation Status: {animal_info.get('conservation_status', 'Unknown')}")
    
    print("\nAdaptations in the wild:")
    for adaptation in animal_info.get('adaptations', []):
        if adaptation in ADAPTATIONS:
            adaptation_info = ADAPTATIONS[adaptation]
            print(f"- {adaptation_info['display_name']}: {adaptation_info['description']}")
            
            # Add this fact to the pet's knowledge
            fact = f"{adaptation_info['display_name']}: {adaptation_info['description']}"
            pet_manager.learn_fact(fact)

def add_materials_to_critter(pet_manager):
    """Add materials to the critter form."""
    print("\n=== Add Materials to Your Critter ===")
    
    # Get available materials based on zoologist level
    available_materials = {k: v for k, v in CRAFTING_MATERIALS.items() 
                          if k in pet_manager.pet.unlocked_materials}
    
    if not available_materials:
        print("You don't have any materials unlocked yet!")
        return
    
    while True:
        print("\nAvailable Materials:")
        material_type = get_choice_from_dict(
            "\nSelect a material (or enter 0 to finish): ", 
            available_materials
        )
        
        if material_type == '0':
            break
        
        # Select color
        print(f"\nAvailable Colors for {CRAFTING_MATERIALS[material_type]['display_name']}:")
        colors = CRAFTING_MATERIALS[material_type]['colors']
        for i, color in enumerate(colors, 1):
            print(f"{i}. {color.capitalize()}")
        
        color_idx = int(get_valid_input(
            "\nSelect a color: ",
            lambda n: n.isdigit() and 1 <= int(n) <= len(colors),
            f"Please enter a number between 1 and {len(colors)}."
        ))
        color = colors[color_idx - 1]
        
        # Get coverage
        coverage = float(get_valid_input(
            "\nEnter coverage (0.1 to 1.0): ",
            lambda n: n.replace('.', '', 1).isdigit() and 0.1 <= float(n) <= 1.0,
            "Please enter a number between 0.1 and 1.0."
        ))
        
        # Get position
        position = get_valid_input(
            "\nWhere to apply this material (e.g., body, head, limbs, tail): ",
            lambda p: p and len(p) <= 20,
            "Please enter a valid position (max 20 characters)."
        )
        
        # Add the material to the critter
        success = pet_manager.add_material(material_type, color, coverage, position)
        if success:
            print(f"\nAdded {color} {CRAFTING_MATERIALS[material_type]['display_name']} to {position}!")
        else:
            print("\nFailed to add material. Please try again.")
        
        # Ask if they want to add more
        add_more = get_valid_input(
            "\nAdd another material? (y/n): ",
            lambda a: a.lower() in ['y', 'n', 'yes', 'no'],
            "Please enter 'y' or 'n'."
        )
        
        if add_more.lower() in ['n', 'no']:
            break

def add_adaptations_to_critter(pet_manager):
    """Add adaptations to the critter form."""
    print("\n=== Add Adaptations to Your Critter ===")
    
    # Get available adaptations based on zoologist level
    available_adaptations = {k: v for k, v in ADAPTATIONS.items() 
                            if k in pet_manager.pet.unlocked_adaptations}
    
    if not available_adaptations:
        print("You don't have any adaptations unlocked yet!")
        return
    
    while True:
        print("\nAvailable Adaptations:")
        adaptation_type = get_choice_from_dict(
            "\nSelect an adaptation (or enter 0 to finish): ", 
            available_adaptations
        )
        
        if adaptation_type == '0':
            break
        
        # Get strength
        strength = int(get_valid_input(
            "\nEnter strength (1 to 10): ",
            lambda n: n.isdigit() and 1 <= int(n) <= 10,
            "Please enter a number between 1 and 10."
        ))
        
        # Get position
        position = get_valid_input(
            "\nWhere to apply this adaptation (e.g., body, head, limbs, tail): ",
            lambda p: p and len(p) <= 20,
            "Please enter a valid position (max 20 characters)."
        )
        
        # Add the adaptation to the critter
        success = pet_manager.add_adaptation(adaptation_type, strength, position)
        if success:
            print(f"\nAdded {ADAPTATIONS[adaptation_type]['display_name']} to {position}!")
        else:
            print("\nFailed to add adaptation. Please try again.")
        
        # Ask if they want to add more
        add_more = get_valid_input(
            "\nAdd another adaptation? (y/n): ",
            lambda a: a.lower() in ['y', 'n', 'yes', 'no'],
            "Please enter 'y' or 'n'."
        )
        
        if add_more.lower() in ['n', 'no']:
            break

def modify_critter_form(pet_manager):
    """Modify the current critter form."""
    clear_screen()
    print_header()
    print("=== Modify Critter Form ===")
    
    animal_info = CRITTER_TYPES.get(pet_manager.pet.base_animal, {})
    print(f"Current Critter Form: {animal_info.get('display_name', pet_manager.pet.base_animal)}")
    print(f"Materials: {len(pet_manager.pet.materials)}")
    print(f"Adaptations: {len(pet_manager.pet.adaptations)}")
    
    print("\nWhat would you like to modify?")
    print("1. Add Materials")
    print("2. Add Adaptations")
    print("3. Back")
    
    choice = input("\nEnter your choice: ").strip()
    
    if choice == '1':
        add_materials_to_critter(pet_manager)
    elif choice == '2':
        add_adaptations_to_critter(pet_manager)
    elif choice == '3':
        return
    else:
        print("\nInvalid choice. Please select from the options above.")

def simulate_critter(pet_manager):
    """Simulate the critter in different environments."""
    clear_screen()
    print_header()
    print("=== Adaptation Station: Environmental Simulation ===")
    
    environments = ["forest", "ocean", "desert", "arctic", "grassland"]
    
    print("\nAvailable Environments:")
    for i, env in enumerate(environments, 1):
        print(f"{i}. {env.capitalize()}")
    
    env_idx = int(get_valid_input(
        "\nSelect an environment to simulate: ",
        lambda n: n.isdigit() and 1 <= int(n) <= len(environments),
        f"Please enter a number between 1 and {len(environments)}."
    ))
    environment = environments[env_idx - 1]
    
    try:
        results = pet_manager.simulate_in_environment(environment)
        
        clear_screen()
        print_header()
        print(f"=== Simulation Results: {environment.capitalize()} Environment ===")
        print(f"\nSurvival Score: {results['survival_score']}/100")
        
        if results['advantages']:
            print("\nAdvantages:")
            for advantage in results['advantages']:
                print(f"• {advantage}")
        
        if results['disadvantages']:
            print("\nDisadvantages:")
            for disadvantage in results['disadvantages']:
                print(f"• {disadvantage}")
        
        # Educational benefit - increase IQ
        print(f"\nYour pet's IQ has increased to {pet_manager.pet.iq} from running this simulation!")
        
    except Exception as e:
        print(f"\nError during simulation: {e}")

def view_zoologist_journal(pet_manager):
    """View the zoologist's journal with facts and progress."""
    clear_screen()
    print_header()
    print("=== Zoologist's Journal ===")
    
    print(f"Zoologist Level: {pet_manager.pet.zoologist_level}")
    print(f"Critters Created: {pet_manager.pet.critters_created}")
    
    # Show progress to next level
    level_order = ['novice', 'apprentice', 'journeyman', 'expert', 'master']
    current_index = level_order.index(pet_manager.pet.zoologist_level)
    
    if current_index < len(level_order) - 1:
        next_level = level_order[current_index + 1]
        critters_needed = ZOOLOGIST_LEVELS[next_level]['required_critters']
        print(f"Progress to {ZOOLOGIST_LEVELS[next_level]['display_name']}: {pet_manager.pet.critters_created}/{critters_needed} critters")
    
    # Show unlocked materials and adaptations
    print(f"\nUnlocked Materials: {len(pet_manager.pet.unlocked_materials)}/{len(CRAFTING_MATERIALS)}")
    print(f"Unlocked Adaptations: {len(pet_manager.pet.unlocked_adaptations)}/{len(ADAPTATIONS)}")
    
    # Show learned facts
    if pet_manager.pet.facts_learned:
        print("\nFacts Learned:")
        for i, fact in enumerate(sorted(pet_manager.pet.facts_learned), 1):
            print(f"{i}. {fact}")
    else:
        print("\nNo facts learned yet. Create a critter to learn animal facts!")

# --- Migration Readiness Check ---
def check_migration_readiness(pet_manager):
    """Check if the pet is ready for blockchain migration."""
    clear_screen()
    print_header()
    print("=== Blockchain Migration Readiness Check ===")
    
    ready, message = pet_manager.check_migration_readiness()
    print(f"\n{message}")
    
    if ready:
        print("\nYour pet is ready to be migrated to the blockchain!")
        print("This would allow your pet to become a unique digital asset (NFT) on the CritterChain.")
        print("The blockchain migration feature will be available in a future update.")
    else:
        print("\nKeep nurturing your pet to prepare them for blockchain migration.")
        print("This will allow them to become a unique digital asset with true ownership.")
    
    input("\nPress Enter to continue...")

# --- Main Menu and Application Flow ---
def main_menu(pet_manager):
    """Display the main menu and handle user input."""
    while True:
        clear_screen()
        print_header()
        
        # Update pet stats based on time passed
        pet_manager.tick(time.time_ns())
        
        # Display pet status
        print(pet_manager.status())
        print("\n" + "=" * 20 + " Main Menu " + "=" * 20)
        print(f"{MENU_PET_CARE}. Pet Care")
        print(f"{MENU_CRITTER_CRAFT}. Critter Craft")
        print(f"{MENU_CHAT}. Chat with {pet_manager.pet.name}")
        print(f"{MENU_CHECK_STATUS}. Check Status")
        print(f"{MENU_CHECK_MIGRATION}. Check Migration Readiness")
        print(f"{MENU_SAVE_EXIT}. Save and Exit")
        print("=" * 60)
        
        choice = input("\nEnter your choice: ").strip()
        
        if choice == MENU_PET_CARE:
            pet_care_menu(pet_manager)
        
        elif choice == MENU_CRITTER_CRAFT:
            critter_craft_menu(pet_manager)
            
        elif choice == MENU_CHAT:
            # Direct chat from main menu
            print(f"\n=== Chat with {pet_manager.pet.name} ===")
            print("Available commands:")
            print("  help - Show available commands")
            print("  status - Check pet's current status")
            print("  exit - End the conversation")
            
            # Start a chat session
            chatting = True
            while chatting:
                message = input("\nYou: ")
                message_lower = message.lower().strip()
                
                # Check for special commands
                if message_lower in ['exit', 'quit', 'bye']:
                    print(f"\n{pet_manager.pet.name} waves goodbye!")
                    chatting = False
                    continue
                    
                elif message_lower == 'help':
                    print("\nAvailable commands:")
                    print("  help - Show this help message")
                    print("  status - Check pet's current status")
                    print("  exit - End the conversation")
                    continue
                    
                elif message_lower == 'status':
                    print(f"\n=== {pet_manager.pet.name}'s Status ===")
                    print(f"Mood: {pet_manager._get_current_mood()}")
                    print(f"Hunger: {pet_manager.pet.hunger}/100")
                    print(f"Energy: {pet_manager.pet.energy}/100")
                    print(f"Happiness: {pet_manager.pet.happiness}/100")
                    continue
                
                # Process the message
                if message.strip():
                    success, response_message = pet_manager.chat(message)
                    print(f"\n{pet_manager.pet.name}: {response_message}")
                    
                    # Update pet status after each interaction
                    pet_manager.tick(time.time_ns())
                else:
                    print(f"\n{pet_manager.pet.name} waits for you to say something meaningful...")
            
            # Wait for user to continue
            input("\nPress Enter to return to the main menu...")
            
        elif choice == MENU_CHECK_STATUS:
            # Just display the status and wait for input
            input("\nPress Enter to continue...")
        
        elif choice == MENU_CHECK_MIGRATION:
            check_migration_readiness(pet_manager)
        
        elif choice == MENU_SAVE_EXIT:
            save_pet(pet_manager.pet, pet_manager)
            print("\nThank you for playing! See you next time.")
            break
            
        else:
            print("\nInvalid choice. Please select from the options above.")

def main():
    """Main application entry point."""
    clear_screen()
    print_header()
    
    # Try to load existing pet
    pet, ai_data = load_pet()
    
    if pet:
        print(f"Welcome back! {pet.name} is excited to see you again!")
        pet_manager = IntegratedPetManager(pet)
        
        # Initialize AI manager with saved data if available
        if ai_data:
            pet_data = {
                'name': pet.name,
                'species': pet.species,
                'aura_color': pet.aura_color,
                'personality_traits': pet.personality_traits,
                'base_animal': pet.base_animal
            }
            pet_manager.ai_manager = AIIntegrationManager.from_dict(ai_data, pet_data)
        
        input("\nPress Enter to continue...")
    else:
        # Create a new pet if no save file or corrupted
        print("No pet found or save file corrupted. Let's create a new one!")
        input("\nPress Enter to continue...")
        pet = create_new_pet()
        pet_manager = IntegratedPetManager(pet)
        save_pet(pet, pet_manager)  # Save the newly created pet immediately
        input("\nPress Enter to continue...")
    
    # Enter the main game loop
    main_menu(pet_manager)

if __name__ == "__main__":
    main()