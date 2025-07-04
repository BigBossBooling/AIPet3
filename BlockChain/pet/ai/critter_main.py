# pet/critter_main.py
"""
Main application for Critter-Craft: Nurture Your Inner Zoologist.
This is the command-line interface for creating and interacting with your crafted critters.
"""

import os
import time
import json
from critter_core import Critter, ZoologistJournal, CraftingMaterial, Adaptation
from config import (
    CRITTER_TYPES,
    CRAFTING_MATERIALS,
    ADAPTATIONS,
    ZOOLOGIST_LEVELS
)

# --- Configuration Constants ---
SAVE_DIR = "critter_data"
CRITTER_SAVE_DIR = f"{SAVE_DIR}/critters"
JOURNAL_SAVE_PATH = f"{SAVE_DIR}/journal.json"
MAX_CRITTER_NAME_LENGTH = 20

# --- Menu Choices ---
MENU_CREATE = '1'
MENU_VIEW_GALLERY = '2'
MENU_ADAPTATION_STATION = '3'
MENU_ZOOLOGIST_JOURNAL = '4'
MENU_SAVE_EXIT = '5'

# --- Utility Functions ---
def clear_screen():
    """Clear the terminal screen."""
    os.system('cls' if os.name == 'nt' else 'clear')

def print_header():
    """Print the application header."""
    print("=" * 60)
    print("  Critter-Craft: Nurture Your Inner Zoologist")
    print("=" * 60)
    print()

def get_valid_input(prompt, validation_func, error_message):
    """Generic function to get validated input from the user."""
    while True:
        user_input = input(prompt).strip()
        if validation_func(user_input):
            return user_input
        print(error_message)

def get_choice_from_dict(prompt_text, data_dict, show_description=True, show_difficulty=False):
    """Displays choices from a dictionary and gets a valid integer choice."""
    items = list(data_dict.items())
    for i, (key, value) in enumerate(items, 1):
        display_name = value.get('display_name', key)
        
        print(f"{i}. {display_name}", end="")
        
        if show_difficulty and 'difficulty' in value:
            print(f" (Difficulty: {value['difficulty'].capitalize()})", end="")
            
        print()
        
        if show_description and 'description' in value:
            print(f"   {value['description']}")

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
    os.makedirs(SAVE_DIR, exist_ok=True)
    os.makedirs(CRITTER_SAVE_DIR, exist_ok=True)

# --- Critter Management Functions ---
def create_new_critter(journal):
    """Create a new critter with user input."""
    clear_screen()
    print_header()
    print("Let's create a new critter!")
    print()

    # Get critter name
    name = get_valid_input(
        "What would you like to name your critter? ",
        lambda n: n and 1 <= len(n) <= MAX_CRITTER_NAME_LENGTH and all(c.isprintable() for c in n),
        f"Please enter a valid name (1-{MAX_CRITTER_NAME_LENGTH} printable characters)."
    )

    # Get creator name
    creator_name = get_valid_input(
        "Enter your name (as the creator): ",
        lambda n: n and 1 <= len(n) <= MAX_CRITTER_NAME_LENGTH and all(c.isprintable() for c in n),
        f"Please enter a valid name (1-{MAX_CRITTER_NAME_LENGTH} printable characters)."
    )

    # Select base animal
    print("\nAvailable Base Animals:")
    base_animal = get_choice_from_dict(
        "\nSelect a base animal for your critter: ", 
        CRITTER_TYPES,
        show_difficulty=True
    )

    # Create the critter
    critter = Critter(name=name, base_animal=base_animal, creator_name=creator_name)
    
    # Display animal facts
    display_animal_facts(critter, journal)
    
    # Add materials
    add_materials_to_critter(critter, journal)
    
    # Add adaptations
    add_adaptations_to_critter(critter, journal)
    
    # Update journal
    journal.add_critter()
    
    print(f"\nCongratulations! {critter.name} has been created!")
    print(critter.get_info_card())
    
    # Save the critter
    save_critter(critter)
    save_journal(journal)
    
    return critter

def display_animal_facts(critter, journal):
    """Display facts about the base animal and add them to the journal."""
    animal_info = CRITTER_TYPES.get(critter.base_animal, {})
    
    print(f"\n=== Facts about {animal_info.get('display_name', critter.base_animal)} ===")
    print(f"Habitat: {animal_info.get('habitat', 'Unknown')}")
    print(f"Diet: {animal_info.get('diet', 'Unknown')}")
    print(f"Conservation Status: {animal_info.get('conservation_status', 'Unknown')}")
    
    # Add these facts to the journal
    journal.add_animal_fact(critter.base_animal, f"Habitat: {animal_info.get('habitat', 'Unknown')}")
    journal.add_animal_fact(critter.base_animal, f"Diet: {animal_info.get('diet', 'Unknown')}")
    journal.add_animal_fact(critter.base_animal, f"Conservation Status: {animal_info.get('conservation_status', 'Unknown')}")
    
    # Add these facts to the critter
    critter.learn_fact(f"Habitat: {animal_info.get('habitat', 'Unknown')}")
    critter.learn_fact(f"Diet: {animal_info.get('diet', 'Unknown')}")
    critter.learn_fact(f"Conservation Status: {animal_info.get('conservation_status', 'Unknown')}")
    
    print("\nAdaptations in the wild:")
    for adaptation in animal_info.get('adaptations', []):
        if adaptation in ADAPTATIONS:
            adaptation_info = ADAPTATIONS[adaptation]
            print(f"- {adaptation_info['display_name']}: {adaptation_info['description']}")
            
            # Add this fact to the journal and critter
            fact = f"{adaptation_info['display_name']}: {adaptation_info['description']}"
            journal.add_animal_fact(critter.base_animal, fact)
            critter.learn_fact(fact)

def add_materials_to_critter(critter, journal):
    """Add materials to the critter."""
    print("\n=== Add Materials to Your Critter ===")
    
    # Get available materials based on journal level
    available_materials = {k: v for k, v in CRAFTING_MATERIALS.items() 
                          if k in journal.unlocked_materials}
    
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
        success = critter.add_material(material_type, color, coverage, position)
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

def add_adaptations_to_critter(critter, journal):
    """Add adaptations to the critter."""
    print("\n=== Add Adaptations to Your Critter ===")
    
    # Get available adaptations based on journal level
    available_adaptations = {k: v for k, v in ADAPTATIONS.items() 
                            if k in journal.unlocked_adaptations}
    
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
        success = critter.add_adaptation(adaptation_type, strength, position)
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

def view_critter_gallery(journal):
    """View all created critters."""
    clear_screen()
    print_header()
    print("=== Critter Gallery ===")
    
    # Get all critter files
    critter_files = []
    try:
        critter_files = os.listdir(CRITTER_SAVE_DIR)
    except FileNotFoundError:
        ensure_save_directories()
    
    if not critter_files:
        print("\nYour gallery is empty. Create some critters first!")
        input("\nPress Enter to continue...")
        return
    
    # Load and display critters
    critters = []
    for filename in critter_files:
        if filename.endswith('.json'):
            try:
                with open(os.path.join(CRITTER_SAVE_DIR, filename), 'r') as f:
                    critter = Critter.from_json(f.read())
                    critters.append(critter)
            except Exception as e:
                print(f"Error loading {filename}: {e}")
    
    if not critters:
        print("\nNo valid critters found in your gallery.")
        input("\nPress Enter to continue...")
        return
    
    # Display critter list
    print(f"\nYou have {len(critters)} critters in your gallery:")
    for i, critter in enumerate(critters, 1):
        base_animal = CRITTER_TYPES.get(critter.base_animal, {}).get('display_name', critter.base_animal)
        print(f"{i}. {critter.name} ({base_animal})")
    
    # Let user select a critter to view in detail
    while True:
        try:
            choice = input("\nEnter a number to view details (or 0 to return): ")
            if choice == '0':
                break
                
            idx = int(choice) - 1
            if 0 <= idx < len(critters):
                clear_screen()
                print_header()
                print(critters[idx].get_info_card())
                
                # Offer to simulate in environment
                simulate = get_valid_input(
                    "\nSimulate this critter in an environment? (y/n): ",
                    lambda a: a.lower() in ['y', 'n', 'yes', 'no'],
                    "Please enter 'y' or 'n'."
                )
                
                if simulate.lower() in ['y', 'yes']:
                    simulate_critter(critters[idx])
                
                input("\nPress Enter to return to gallery...")
                clear_screen()
                print_header()
                print("=== Critter Gallery ===")
                print(f"\nYou have {len(critters)} critters in your gallery:")
                for i, critter in enumerate(critters, 1):
                    base_animal = CRITTER_TYPES.get(critter.base_animal, {}).get('display_name', critter.base_animal)
                    print(f"{i}. {critter.name} ({base_animal})")
            else:
                print(f"Please enter a number between 1 and {len(critters)}.")
        except ValueError:
            print("Invalid input. Please enter a number.")
    
    return

def simulate_critter(critter):
    """Simulate a critter in different environments."""
    clear_screen()
    print_header()
    print(f"=== Adaptation Station: Simulating {critter.name} ===")
    
    environments = ["forest", "ocean", "desert", "arctic", "grassland"]
    
    print("\nAvailable Environments:")
    for i, env in enumerate(environments, 1):
        print(f"{i}. {env.capitalize()}")
    
    while True:
        try:
            choice = input("\nSelect an environment (or 0 to return): ")
            if choice == '0':
                break
                
            idx = int(choice) - 1
            if 0 <= idx < len(environments):
                environment = environments[idx]
                results = critter.simulate_in_environment(environment)
                
                clear_screen()
                print_header()
                print(f"=== Simulating {critter.name} in {environment.capitalize()} Environment ===")
                print(f"\nSurvival Score: {results['survival_score']}/100")
                
                if results['advantages']:
                    print("\nAdvantages:")
                    for advantage in results['advantages']:
                        print(f"+ {advantage}")
                
                if results['disadvantages']:
                    print("\nDisadvantages:")
                    for disadvantage in results['disadvantages']:
                        print(f"- {disadvantage}")
                
                input("\nPress Enter to try another environment...")
                clear_screen()
                print_header()
                print(f"=== Adaptation Station: Simulating {critter.name} ===")
                print("\nAvailable Environments:")
                for i, env in enumerate(environments, 1):
                    print(f"{i}. {env.capitalize()}")
            else:
                print(f"Please enter a number between 1 and {len(environments)}.")
        except ValueError:
            print("Invalid input. Please enter a number.")
    
    return

def view_zoologist_journal(journal):
    """View the zoologist's journal."""
    clear_screen()
    print_header()
    print("=== Zoologist's Journal ===")
    
    # Display level info
    level_info = journal.get_level_info()
    print(f"\nZoologist Level: {level_info['current_level']}")
    print(f"Critters Created: {level_info['critters_created']}")
    
    if level_info['next_level']:
        print(f"Next Level: {level_info['next_level']} (requires {level_info['critters_needed_for_next_level']} critters)")
    else:
        print("You have reached the maximum level!")
    
    print(f"\nTotal Facts Learned: {level_info['total_facts_learned']}")
    
    # Display unlocked materials
    print("\nUnlocked Materials:")
    for material in level_info['unlocked_materials']:
        print(f"- {material}")
    
    # Display unlocked adaptations
    print("\nUnlocked Adaptations:")
    for adaptation in level_info['unlocked_adaptations']:
        print(f"- {adaptation}")
    
    # Display animal facts
    if journal.animal_facts:
        print("\nAnimal Facts:")
        for animal, facts in journal.animal_facts.items():
            animal_name = CRITTER_TYPES.get(animal, {}).get('display_name', animal)
            print(f"\n{animal_name}:")
            for fact in facts:
                print(f"- {fact}")
    
    input("\nPress Enter to continue...")
    return

def adaptation_station(journal):
    """Enter the adaptation station to simulate critters in different environments."""
    clear_screen()
    print_header()
    print("=== Adaptation Station ===")
    
    # Get all critter files
    critter_files = []
    try:
        critter_files = os.listdir(CRITTER_SAVE_DIR)
    except FileNotFoundError:
        ensure_save_directories()
    
    if not critter_files:
        print("\nYou don't have any critters to simulate. Create some first!")
        input("\nPress Enter to continue...")
        return
    
    # Load critters
    critters = []
    for filename in critter_files:
        if filename.endswith('.json'):
            try:
                with open(os.path.join(CRITTER_SAVE_DIR, filename), 'r') as f:
                    critter = Critter.from_json(f.read())
                    critters.append(critter)
            except Exception as e:
                print(f"Error loading {filename}: {e}")
    
    if not critters:
        print("\nNo valid critters found.")
        input("\nPress Enter to continue...")
        return
    
    # Display critter list
    print(f"\nSelect a critter to simulate:")
    for i, critter in enumerate(critters, 1):
        base_animal = CRITTER_TYPES.get(critter.base_animal, {}).get('display_name', critter.base_animal)
        print(f"{i}. {critter.name} ({base_animal})")
    
    # Let user select a critter
    while True:
        try:
            choice = input("\nEnter a number (or 0 to return): ")
            if choice == '0':
                break
                
            idx = int(choice) - 1
            if 0 <= idx < len(critters):
                simulate_critter(critters[idx])
                
                # Return to critter list
                clear_screen()
                print_header()
                print("=== Adaptation Station ===")
                print(f"\nSelect a critter to simulate:")
                for i, critter in enumerate(critters, 1):
                    base_animal = CRITTER_TYPES.get(critter.base_animal, {}).get('display_name', critter.base_animal)
                    print(f"{i}. {critter.name} ({base_animal})")
            else:
                print(f"Please enter a number between 1 and {len(critters)}.")
        except ValueError:
            print("Invalid input. Please enter a number.")
    
    return

# --- Save/Load Functions ---
def save_critter(critter):
    """Save a critter to a file."""
    ensure_save_directories()
    
    filename = f"{critter.id}.json"
    filepath = os.path.join(CRITTER_SAVE_DIR, filename)
    
    try:
        with open(filepath, 'w') as f:
            f.write(critter.to_json())
        print(f"\n{critter.name} has been saved to your gallery!")
    except Exception as e:
        print(f"Error saving {critter.name}: {e}")

def save_journal(journal):
    """Save the zoologist's journal to a file."""
    ensure_save_directories()
    
    try:
        with open(JOURNAL_SAVE_PATH, 'w') as f:
            f.write(journal.to_json())
    except Exception as e:
        print(f"Error saving journal: {e}")

def load_journal():
    """Load the zoologist's journal from a file."""
    ensure_save_directories()
    
    if not os.path.exists(JOURNAL_SAVE_PATH):
        # Create a new journal
        return ZoologistJournal(username="Zoologist")
    
    try:
        with open(JOURNAL_SAVE_PATH, 'r') as f:
            journal_json = f.read()
            if not journal_json:
                print(f"Warning: {JOURNAL_SAVE_PATH} is empty. Starting new journal.")
                return ZoologistJournal(username="Zoologist")
            return ZoologistJournal.from_json(journal_json)
    except json.JSONDecodeError:
        print(f"Error: Corrupted journal data in {JOURNAL_SAVE_PATH}. Starting new journal.")
        return ZoologistJournal(username="Zoologist")
    except Exception as e:
        print(f"An unexpected error occurred while loading journal: {e}. Starting new journal.")
        return ZoologistJournal(username="Zoologist")

# --- Main Menu Function ---
def main_menu(journal):
    """Display the main menu and handle user input."""
    while True:
        clear_screen()
        print_header()
        
        # Display zoologist level
        level_info = journal.get_level_info()
        print(f"Zoologist Level: {level_info['current_level']}")
        print(f"Critters Created: {level_info['critters_created']}")
        
        if level_info['next_level']:
            print(f"Next Level: {level_info['next_level']} (requires {level_info['critters_needed_for_next_level']} critters)")
        
        print("\n" + "=" * 25 + " Main Menu " + "=" * 25)
        print(f"{MENU_CREATE}. Create a New Critter")
        print(f"{MENU_VIEW_GALLERY}. View Critter Gallery")
        print(f"{MENU_ADAPTATION_STATION}. Adaptation Station")
        print(f"{MENU_ZOOLOGIST_JOURNAL}. Zoologist's Journal")
        print(f"{MENU_SAVE_EXIT}. Save and Exit")
        print("=" * 60)
        
        choice = input("\nEnter your choice: ").strip()
        
        if choice == MENU_CREATE:
            create_new_critter(journal)
            save_journal(journal)
            input("\nPress Enter to continue...")
        
        elif choice == MENU_VIEW_GALLERY:
            view_critter_gallery(journal)
        
        elif choice == MENU_ADAPTATION_STATION:
            adaptation_station(journal)
        
        elif choice == MENU_ZOOLOGIST_JOURNAL:
            view_zoologist_journal(journal)
        
        elif choice == MENU_SAVE_EXIT:
            save_journal(journal)
            print("\nThank you for playing Critter-Craft! See you next time.")
            break
        
        else:
            print("\nInvalid choice. Please select from the options above.")
            input("\nPress Enter to continue...")

# --- Main Application Flow ---
def main():
    """Main application entry point."""
    clear_screen()
    print_header()
    
    # Load or create zoologist's journal
    journal = load_journal()
    
    print("Welcome to Critter-Craft: Nurture Your Inner Zoologist!")
    print("\nIn this application, you can create and customize your own virtual creatures")
    print("inspired by real-world animals and their unique adaptations.")
    print("\nLearn fascinating biological facts while unleashing your creativity!")
    
    input("\nPress Enter to continue...")
    
    # Enter the main game loop
    main_menu(journal)

if __name__ == "__main__":
    main()