# pet/critter_demo.py
"""
Demonstration script for the Critter-Craft: Nurture Your Inner Zoologist application.
This script shows how to create, customize, and simulate critters.
"""

import os
import time
from critter_core import Critter, ZoologistJournal, CraftingMaterial, Adaptation
from config import CRITTER_TYPES, CRAFTING_MATERIALS, ADAPTATIONS

def clear_screen():
    """Clear the terminal screen."""
    os.system('cls' if os.name == 'nt' else 'clear')

def print_separator():
    """Print a separator line."""
    print("=" * 60)

def print_header():
    """Print the application header."""
    print_separator()
    print("  Critter-Craft: Nurture Your Inner Zoologist - DEMO")
    print_separator()
    print()

def demo_create_critter():
    """Demonstrate creating a new critter."""
    print_header()
    print("CREATING A NEW CRITTER")
    print_separator()
    
    # Create a journal for the demo
    journal = ZoologistJournal(username="Demo User")
    
    # Create a chameleon critter
    critter = Critter(
        name="Camo",
        base_animal="chameleon",
        creator_name="Demo User"
    )
    
    print(f"Created a new critter named {critter.name}!")
    print(f"Base animal: {CRITTER_TYPES[critter.base_animal]['display_name']}")
    print(f"Description: {CRITTER_TYPES[critter.base_animal]['description']}")
    
    # Add materials
    critter.add_material("scales", "green", 0.8, "body")
    print("\nAdded green scales to the body!")
    
    critter.add_material("scales", "blue", 0.2, "head")
    print("Added blue scales to the head!")
    
    # Add adaptations
    critter.add_adaptation("camouflage", 8, "body")
    print("\nAdded camouflage adaptation to the body!")
    
    critter.add_adaptation("specialized_limbs", 6, "feet")
    print("Added specialized limbs adaptation to the feet!")
    
    # Learn some facts
    critter.learn_fact("Chameleons can move their eyes independently of each other.")
    critter.learn_fact("Chameleons change color to communicate, not just for camouflage.")
    critter.learn_fact("Chameleons have specialized feet for gripping branches.")
    
    print("\nLearned 3 facts about chameleons!")
    
    # Display the critter info card
    print("\nCritter Info Card:")
    print(critter.get_info_card())
    
    return critter, journal

def demo_simulate_critter(critter):
    """Demonstrate simulating a critter in different environments."""
    print_separator()
    print("SIMULATING CRITTER IN ENVIRONMENTS")
    print_separator()
    
    environments = ["forest", "ocean", "desert"]
    
    for environment in environments:
        print(f"\nSimulating {critter.name} in {environment.capitalize()} Environment:")
        results = critter.simulate_in_environment(environment)
        
        print(f"Survival Score: {results['survival_score']}/100")
        
        if results['advantages']:
            print("\nAdvantages:")
            for advantage in results['advantages']:
                print(f"+ {advantage}")
        
        if results['disadvantages']:
            print("\nDisadvantages:")
            for disadvantage in results['disadvantages']:
                print(f"- {disadvantage}")
        
        print_separator()

def demo_zoologist_journal(journal):
    """Demonstrate the zoologist's journal functionality."""
    print_separator()
    print("ZOOLOGIST'S JOURNAL")
    print_separator()
    
    # Add some animal facts
    journal.add_animal_fact("chameleon", "Chameleons can move their eyes independently of each other.")
    journal.add_animal_fact("chameleon", "Chameleons change color to communicate, not just for camouflage.")
    journal.add_animal_fact("chameleon", "Chameleons have specialized feet for gripping branches.")
    
    journal.add_animal_fact("anglerfish", "Anglerfish live in the deep sea, up to a mile below the surface.")
    journal.add_animal_fact("anglerfish", "Female anglerfish are much larger than males.")
    
    # Add some critters to increase level
    for i in range(5):
        journal.add_critter()
    
    # Display journal info
    level_info = journal.get_level_info()
    print(f"Zoologist Level: {level_info['current_level']}")
    print(f"Critters Created: {level_info['critters_created']}")
    
    if level_info['next_level']:
        print(f"Next Level: {level_info['next_level']} (requires {level_info['critters_needed_for_next_level']} critters)")
    
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
    print("\nAnimal Facts:")
    for animal, facts in journal.animal_facts.items():
        animal_name = CRITTER_TYPES.get(animal, {}).get('display_name', animal)
        print(f"\n{animal_name}:")
        for fact in facts:
            print(f"- {fact}")

def demo_create_multiple_critters():
    """Demonstrate creating multiple critters with different base animals."""
    print_separator()
    print("CREATING MULTIPLE CRITTERS")
    print_separator()
    
    critters = []
    
    # Create an anglerfish critter
    anglerfish = Critter(
        name="Lumina",
        base_animal="anglerfish",
        creator_name="Demo User"
    )
    
    anglerfish.add_material("scales", "black", 0.9, "body")
    anglerfish.add_adaptation("bioluminescence", 10, "lure")
    anglerfish.add_adaptation("large_jaw", 8, "mouth")
    
    critters.append(anglerfish)
    print(f"Created {anglerfish.name} the Anglerfish!")
    
    # Create a hummingbird critter
    hummingbird = Critter(
        name="Zippy",
        base_animal="hummingbird",
        creator_name="Demo User"
    )
    
    hummingbird.add_material("feathers", "iridescent", 0.9, "body")
    hummingbird.add_adaptation("rapid_wing_movement", 10, "wings")
    hummingbird.add_adaptation("long_beak", 7, "head")
    
    critters.append(hummingbird)
    print(f"Created {hummingbird.name} the Hummingbird!")
    
    # Create a platypus critter
    platypus = Critter(
        name="Oddball",
        base_animal="platypus",
        creator_name="Demo User"
    )
    
    platypus.add_material("fur", "brown", 0.8, "body")
    platypus.add_adaptation("duck_bill", 9, "head")
    platypus.add_adaptation("webbed_feet", 8, "feet")
    
    critters.append(platypus)
    print(f"Created {platypus.name} the Platypus!")
    
    # Display all critters
    print("\nCreated Critters:")
    for critter in critters:
        print(f"\n{critter.name} ({CRITTER_TYPES[critter.base_animal]['display_name']}):")
        print(f"- Materials: {len(critter.materials)}")
        print(f"- Adaptations: {len(critter.adaptations)}")
    
    return critters

def run_demo():
    """Run the full demonstration."""
    clear_screen()
    print_header()
    print("CRITTER-CRAFT DEMONSTRATION")
    print_separator()
    
    # Create a critter
    critter, journal = demo_create_critter()
    input("\nPress Enter to continue...")
    
    # Simulate the critter in different environments
    demo_simulate_critter(critter)
    input("\nPress Enter to continue...")
    
    # Demonstrate the zoologist's journal
    demo_zoologist_journal(journal)
    input("\nPress Enter to continue...")
    
    # Create multiple critters
    critters = demo_create_multiple_critters()
    input("\nPress Enter to continue...")
    
    print_separator()
    print("DEMONSTRATION COMPLETE")
    print_separator()
    print("\nThis concludes the demonstration of Critter-Craft: Nurture Your Inner Zoologist.")
    print("To start the full application, run 'python critter_main.py'.")

if __name__ == "__main__":
    run_demo()