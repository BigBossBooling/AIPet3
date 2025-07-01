# prometheus_protocol/main.py (Conceptual Path)
import time
import os
import sys

# Add parent directory to path to allow import if running directly from this folder
# This setup is for local testing structure, might differ in actual app
current_dir = os.path.dirname(os.path.abspath(__file__))
if current_dir not in sys.path:
    sys.path.insert(0, current_dir)

from .pet_core import Pet, InteractionRecord # Import Pet class and InteractionRecord
from .config import LOCAL_STORAGE_KEY, GAME_INTERVAL_SECONDS, PET_ARCHETYPES, PET_AURA_COLORS # Import configs

# --- Persistence Manager (Simplified for CLI) ---
# In a real app, this would be a dedicated module or integrated with state management.
def save_pet_to_local_storage(pet: Pet):
    """Saves pet state to a local JSON file."""
    try:
        # Use a consistent filename based on pet ID for potential future multi-pet support
        filename = f"{LOCAL_STORAGE_KEY}_{pet.id}.json" 
        with open(filename, 'w') as f:
            f.write(pet.to_json())
        print(f"[{pet.name}] State saved successfully.")
    except Exception as e:
        print(f"ERROR: Failed to save pet state: {e}")

def load_pet_from_local_storage(pet_id: Optional[str] = None) -> Optional[Pet]:
    """Loads pet state from a local JSON file."""
    try:
        # For simplicity, if no ID, try to load any existing pet or the first one.
        # In a real app, user would select.
        if pet_id:
            filename = f"{LOCAL_STORAGE_KEY}_{pet_id}.json"
        else: # Try to find any existing pet file
            files = [f for f in os.listdir('.') if f.startswith(LOCAL_STORAGE_KEY) and f.endswith('.json')]
            if not files:
                return None
            filename = files[0] # Load the first one found
            
        with open(filename, 'r') as f:
            json_data = f.read()
            pet = Pet.from_json(json_data)
            print(f"[{pet.name}] State loaded successfully.")
            return pet
    except FileNotFoundError:
        print(f"No saved pet found with ID {pet_id if pet_id else 'any'}.")
        return None
    except json.JSONDecodeError as e:
        print(f"ERROR: Corrupted pet save file '{filename}': {e}")
        return None
    except Exception as e:
        print(f"ERROR: Failed to load pet state: {e}")
        return None

# --- Main Game Loop ---
def main():
    print("Welcome to CritterCraft MVP: The Genesis Pet!")
    current_pet: Optional[Pet] = None

    # Attempt to load an existing pet
    current_pet = load_pet_from_local_storage()

    if current_pet is None:
        print("\nNo existing Genesis Pet found. Let's sculpt a new one!")
        while True:
            try:
                pet_name = input("What would you like to name your pet? (max 20 chars) ")
                if not pet_name.strip():
                    print("Pet name cannot be empty. Please try again.")
                    continue
                if len(pet_name.strip()) > 20:
                    print("Pet name is too long (max 20 characters). Please try again.")
                    continue

                print("\nChoose an Archetype:")
                for key, val in PET_ARCHETYPES.items():
                    print(f"- {key} ({val['display_name']})")
                pet_species = input("Enter archetype (e.g., sprite_glow): ")
                if pet_species not in PET_ARCHETYPES:
                    print("Invalid archetype. Please choose from the list.")
                    continue

                print("\nChoose a Primary Aura Color:")
                for key, val in PET_AURA_COLORS.items():
                    print(f"- {key} ({val})")
                pet_aura_color = input("Enter aura color (e.g., aura-blue): ")
                if pet_aura_color not in PET_AURA_COLORS:
                    print("Invalid aura color. Please choose from the list.")
                    continue

                current_pet = Pet(name=pet_name.strip(), species=pet_species, aura_color=pet_aura_color)
                break
            except ValueError as e:
                print(f"Error during pet creation: {e}")
                # Loop will continue

    print(f"\nCongratulations! You've adopted {current_pet.name} the {PET_ARCHETYPES[current_pet.species]['display_name']} with a {current_pet.aura_color} aura.")
    current_pet.status()

    # Main game loop
    while True:
        print("\n--- CritterCraft Command Center ---")
        print("1. Nourish (Feed Pet)")
        print("2. Engage (Play with Pet)")
        print("3. Chronicle (Check Status)")
        print("4. Let Time Pass (Advance Pet State)")
        print("5. Forge Genesis (Create New Pet - Saves current & starts new)") # Added for easier testing
        print("6. Bridge to CritterChain (Prepare for Migration - Conceptual)") # Conceptual feature
        print("7. Exit (Save & Quit)")

        choice = input("Enter your choice (1-7): ")
        
        action_taken_that_ticks = True # Assume most actions trigger a tick unless specified
        
        if choice == '1':
            current_pet.feed()
            print(f"{current_pet.name} consumed sustenance.")
        elif choice == '2':
            success, msg = current_pet.play()
            print(msg)
            if not success: # If pet was too tired, no time passes due to this action
                action_taken_that_ticks = False
        elif choice == '3':
            print("\n--- Pet Status Report ---")
            print(current_pet.status())
            action_taken_that_ticks = False # Checking status doesn't pass time
        elif choice == '4':
            print("Time flows onward for your Genesis Pet...")
        elif choice == '5':
            # Create New Pet - This action saves the current pet and then starts fresh
            print("Saving current pet and forging a new Genesis...")
            save_pet_to_local_storage(current_pet) # Save current pet before replacing
            main() # Recursively call main to start new pet creation flow
            return # Exit this instance of main after recursive call
        elif choice == '6':
            print("\n--- Bridging to CritterChain (Conceptual) ---")
            print("Your Genesis Pet is preparing for its grand journey to the blockchain!")
            
            # Calculate conceptual migration readiness
            readiness_score = 0
            if current_pet.happiness >= MOOD_THRESHOLD_HAPPY and current_pet.hunger <= MAX_STAT - FEED_HUNGER_RESTORE:
                readiness_score += 50
            if len(current_pet.interaction_history) >= MIGRATION_READINESS_THRESHOLDS["min_interactions"]:
                 readiness_score += 50 # Example: if enough interactions
            
            # Conceptual: AI could predict readiness based on personality traits
            # ai_readiness_boost = ai_module.predict_migration_readiness(current_pet.personality_traits)
            # readiness_score = min(100, readiness_score + ai_readiness_boost)

            print(f"Migration Readiness Score: {readiness_score}%")
            if readiness_score == 100:
                print("Your pet is fully ready! Conceptual data export initiated.")
                # In real app: trigger secure data export process here
                # Conceptual: Secure export data for blockchain
                # exported_data = pet_to_blockchain_format(current_pet)
                # save_exported_data_securely(exported_data)
                print(f"Conceptual export for {current_pet.name} complete!")
                print("You will be able to import this pet into the full CritterCraft Blockchain app.")
            else:
                print("Continue nurturing your pet to reach full migration readiness!")
            action_taken_that_ticks = False # Conceptual action doesn't pass game time
        elif choice == '7':
            print(f"Goodbye! Saving {current_pet.name}'s state...")
            save_pet_to_local_storage(current_pet)
            break
        else:
            print("Invalid choice. Please enter a number between 1 and 7.")
            action_taken_that_ticks = False # Invalid choice doesn't pass time

        # Only tick the pet if an action was taken that should advance time
        if action_taken_that_ticks:
            print("A moment passes...")
            current_pet.tick(time.time_ns()) # Pass current time for accurate decay calculation
            # Display brief update after tick
            print(f"[{current_pet.name}] Mood: {current_pet.mood}, Sustenance: {current_pet.hunger}/{MAX_STAT}, Energy: {current_pet.energy}/{MAX_STAT}, Happiness: {current_pet.happiness}/{MAX_STAT}.")

if __name__ == "__main__":
    # Ensure a local storage directory exists for clean file management
    if not os.path.exists("crittercraft_saves"):
        os.makedirs("crittercraft_saves")
    os.chdir("crittercraft_saves") # Change into saves directory

    main()