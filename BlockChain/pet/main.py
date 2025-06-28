# pet/main.py
"""
Main application for CritterCraft Genesis Pet.
This is the command-line interface for interacting with your virtual pet.
"""

import os
import time
import json
from pet.ai.pet_core import Pet, PetLogicManager # Import Pet class from the ai directory
from pet.config import ( # Assumes config.py defines these dictionaries/constants
    PET_ARCHETYPES,
    PET_AURA_COLORS,
    MIGRATION_READINESS_THRESHOLDS
)

# --- Configuration Constants ---
SAVE_FILE_PATH = "pet_data.json"
MAX_PET_NAME_LENGTH = 20

# --- Menu Choices ---
MENU_FEED = '1'
MENU_PLAY = '2'
MENU_CHAT = '3'
MENU_CHECK_MIGRATION = '4'
MENU_SAVE_EXIT = '5'

# --- Utility Functions ---
def clear_screen():
    """Clear the terminal screen."""
    os.system('cls' if os.name == 'nt' else 'clear')

def print_header():
    """Print the application header."""
    print("=" * 50)
    print("  CritterCraft Genesis Pet - Local Prototype")
    print("=" * 50)
    print()

def get_valid_input(prompt, validation_func, error_message):
    """Generic function to get validated input from the user."""
    while True:
        user_input = input(prompt).strip()
        if validation_func(user_input):
            return user_input
        print(error_message)

def get_choice_from_dict(prompt_text, data_dict):
    """Displays choices from a dictionary and gets a valid integer choice."""
    items = list(data_dict.items())
    for i, (key, value) in enumerate(items, 1):
        display_name = value.get('display_name', key)
        description = value.get('description', '')
        rarity = value.get('rarity', '')
        effect = value.get('effect', '')

        print(f"{i}. {display_name}", end="")
        if rarity:
            print(f" ({rarity})", end="")
        if description:
            print(f" - {description}", end="")
        print()
        if effect:
            print(f"   Effect: {effect}")

    while True:
        try:
            choice = int(input(prompt_text))
            if 1 <= choice <= len(items):
                return items[choice - 1][0] # Return the key (e.g., 'draconis', 'solar_flare')
            print(f"Please enter a number between 1 and {len(items)}.")
        except ValueError:
            print("Invalid input. Please enter a number.")

# --- Pet Management Functions ---
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
        "\nSelect a species: ", PET_ARCHETYPES
    )

    print("\nAvailable Aura Colors:")
    aura_color = get_choice_from_dict(
        "\nSelect an aura color: ", PET_AURA_COLORS
    )

    # Create the pet
    pet = Pet(name=name, species=species, aura_color=aura_color)
    pet_manager = PetLogicManager(pet)
    print(f"\nCongratulations! {pet.name} the {PET_ARCHETYPES[species]['display_name']} has been created with a {PET_AURA_COLORS[aura_color]['display_name']} aura!")
    return pet, pet_manager

def load_pet():
    """Load a pet from the save file if it exists."""
    if not os.path.exists(SAVE_FILE_PATH):
        return None, None

    try:
        with open(SAVE_FILE_PATH, 'r') as f:
            pet_json = f.read()
            if not pet_json:
                print(f"Warning: {SAVE_FILE_PATH} is empty. Starting new pet.")
                return None, None
            pet = Pet.from_dict(json.loads(pet_json))
            pet_manager = PetLogicManager(pet)
            return pet, pet_manager
    except json.JSONDecodeError:
        print(f"Error: Corrupted pet data in {SAVE_FILE_PATH}. Starting new pet.")
        return None, None
    except Exception as e:
        print(f"An unexpected error occurred while loading pet: {e}. Starting new pet.")
        return None, None

def save_pet(pet):
    """Save the pet to a file."""
    try:
        with open(SAVE_FILE_PATH, 'w') as f:
            json.dump(pet.to_dict(), f, indent=4)
        print(f"\n{pet.name} has been saved successfully!")
    except Exception as e:
        print(f"Error saving {pet.name}: {e}")

# --- Game Logic Functions ---
def check_migration_readiness(pet):
    """Check if the pet is ready for blockchain migration."""
    thresholds = MIGRATION_READINESS_THRESHOLDS

    # Calculate days owned
    current_time_ns = time.time_ns()
    days_owned = (current_time_ns - pet.creation_timestamp) / (1_000_000_000 * 60 * 60 * 24)

    # Count interactions (assuming pet.interaction_history is a list)
    interaction_count = len(pet.interaction_history)

    # Check all conditions
    is_ready = (
        pet.happiness >= thresholds['min_happiness'] and
        pet.energy >= thresholds['min_energy'] and
        pet.hunger <= thresholds['max_hunger'] and
        pet.iq >= thresholds['min_iq'] and
        pet.charisma >= thresholds['min_charisma'] and
        interaction_count >= thresholds['min_interactions'] and
        days_owned >= thresholds['min_days_owned']
    )

    if is_ready:
        return True, "Your pet is ready for blockchain migration!\nAll conditions met."
    else:
        missing = []
        if pet.happiness < thresholds['min_happiness']:
            missing.append(f"• Happiness: {pet.happiness}/{thresholds['min_happiness']} (min)")
        if pet.energy < thresholds['min_energy']:
            missing.append(f"• Energy: {pet.energy}/{thresholds['min_energy']} (min)")
        if pet.hunger > thresholds['max_hunger']:
            missing.append(f"• Hunger: {pet.hunger}/{thresholds['max_hunger']} (max)")
        if pet.iq < thresholds['min_iq']:
            missing.append(f"• Intelligence: {pet.iq}/{thresholds['min_iq']} (min)")
        if pet.charisma < thresholds['min_charisma']:
            missing.append(f"• Charisma: {pet.charisma}/{thresholds['min_charisma']} (min)")
        if interaction_count < thresholds['min_interactions']:
            missing.append(f"• Interactions: {interaction_count}/{thresholds['min_interactions']} (min)")
        if days_owned < thresholds['min_days_owned']:
            missing.append(f"• Days Owned: {days_owned:.1f}/{thresholds['min_days_owned']} (min)")

        return False, "Your pet is not yet ready for blockchain migration. Keep nurturing them!\nMissing conditions:\n" + "\n".join(missing)

def main_menu(pet, pet_manager):
    """Display the main menu and handle user input."""
    while True:
        clear_screen()
        print_header()

        # Update pet stats based on time passed since last tick
        current_time_ns = time.time_ns()
        pet_manager.tick(current_time_ns)

        # Display pet status
        print(pet_manager.get_status_report())
        print("\n" + "=" * 20 + " Main Menu " + "=" * 20)
        print(f"{MENU_FEED}. Feed {pet.name}")
        print(f"{MENU_PLAY}. Play with {pet.name}")
        print(f"{MENU_CHAT}. Chat with {pet.name}")
        print(f"{MENU_CHECK_MIGRATION}. Check migration readiness")
        print(f"{MENU_SAVE_EXIT}. Save and exit")
        print("=" * 50)

        choice = input("\nEnter your choice: ").strip()

        if choice == MENU_FEED:
            feedback_message = pet_manager.feed()
            print(f"\n{feedback_message}")

        elif choice == MENU_PLAY:
            try:
                feedback_message = pet_manager.play()
                print(f"\n{feedback_message}")
            except Exception as e:
                print(f"\n{pet.name} is too tired to play right now.")

        elif choice == MENU_CHAT:
            print(f"\nChat with {pet.name}:")
            message = input("You: ")
            if message.strip():
                try:
                    response_message = pet_manager.chat(message)
                    print(f"\n{pet.name}: {response_message}")
                except Exception as e:
                    print(f"\n{pet.name} is too tired to chat right now.")
            else:
                print(f"\n{pet.name} waits for you to say something meaningful...")

        elif choice == MENU_CHECK_MIGRATION:
            ready, message = check_migration_readiness(pet)
            print(f"\n{message}")

        elif choice == MENU_SAVE_EXIT:
            save_pet(pet)
            print("\nThank you for playing! See you next time.")
            break

        else:
            print("\nInvalid choice. Please select from the options above.")

        input("\nPress Enter to continue...")

# --- Main Application Flow ---
def main():
    """Main application entry point."""
    clear_screen()
    print_header()

    # Try to load existing pet
    pet, pet_manager = load_pet()

    if pet:
        print(f"Welcome back! {pet.name} is excited to see you again!")
        input("\nPress Enter to continue...")
    else:
        # Create a new pet if no save file or corrupted
        print("No pet found or save file corrupted. Let's create a new one!")
        input("\nPress Enter to continue...")
        pet, pet_manager = create_new_pet()
        save_pet(pet) # Save the newly created pet immediately after creation
        input("\nPress Enter to continue...")

    # Enter the main game loop
    main_menu(pet, pet_manager)

if __name__ == "__main__":
    main()