# pet/ai_demo.py
"""
Demonstration script for the enhanced AI integration in CritterCraft.
This script showcases the dynamic personality system and conversational capabilities.
"""

import time
from integrated_core import IntegratedPet, IntegratedPetManager
from ai_integration import AIIntegrationManager, DynamicPersonality, MemorySystem
from config import PET_ARCHETYPES, PET_AURA_COLORS, CRITTER_TYPES, ADAPTATIONS

def print_section_header(title):
    """Print a section header for the demo."""
    print("\n" + "=" * 60)
    print(f" {title} ".center(60, "="))
    print("=" * 60)

def demo():
    """Run a demonstration of the enhanced AI integration."""
    print_section_header("CritterCraft Enhanced AI Integration Demo")
    print("This demo showcases the dynamic personality system and conversational capabilities.")
    
    # Create a new pet
    print_section_header("Creating a New Pet")
    pet = IntegratedPet(
        name="Luna",
        species="sprite_crystal",
        aura_color="aura-purple"
    )
    pet_manager = IntegratedPetManager(pet)
    
    print(f"Created a new pet named {pet.name}!")
    print(f"Species: {PET_ARCHETYPES[pet.species]['display_name']}")
    print(f"Aura: {PET_AURA_COLORS[pet.aura_color]['display_name']}")
    
    # Demonstrate personality system
    print_section_header("Dynamic Personality System")
    
    print("Initial personality traits:")
    for trait, value in pet_manager.ai_manager.personality.traits.items():
        level = pet_manager.ai_manager.personality.get_trait_level(trait)
        print(f"{trait.capitalize()}: {value}/100 ({level})")
    
    print("\nDominant traits:")
    dominant_traits = pet_manager.ai_manager.personality.get_dominant_traits(2)
    for trait, value in dominant_traits:
        print(f"{trait.capitalize()}: {value}/100")
    
    print("\nAdjusting traits based on interactions...")
    
    # Simulate some interactions to change personality
    print("Simulating multiple play interactions...")
    for _ in range(5):
        pet_manager.ai_manager.process_interaction('play', True)
    
    print("Simulating multiple chat interactions...")
    for _ in range(3):
        pet_manager.ai_manager.process_interaction('chat', True)
    
    print("\nUpdated personality traits:")
    for trait, value in pet_manager.ai_manager.personality.traits.items():
        level = pet_manager.ai_manager.personality.get_trait_level(trait)
        print(f"{trait.capitalize()}: {value}/100 ({level})")
    
    # Demonstrate memory system
    print_section_header("Memory System")
    
    print("Adding memories...")
    pet_manager.ai_manager.learn_fact("Chameleons can move their eyes independently of each other.")
    pet_manager.ai_manager.learn_fact("Crystal formations occur when atoms arrange in a repeating pattern.")
    pet_manager.ai_manager.learn_fact("The purple aura is associated with intuition and spiritual awareness.")
    
    print("\nRecording preferences...")
    pet_manager.ai_manager.memory_system.add_preference_memory("User likes: playing puzzle games")
    pet_manager.ai_manager.memory_system.add_preference_memory("User dislikes: loud noises")
    
    print("\nRecording milestone...")
    pet_manager.ai_manager.record_milestone("First conversation with Luna")
    
    print("\nMemories by type:")
    for memory_type in ['fact', 'preference', 'milestone']:
        memories = pet_manager.ai_manager.memory_system.get_memories_by_type(memory_type)
        print(f"\n{memory_type.capitalize()} memories ({len(memories)}):")
        for memory in memories:
            print(f"- {memory.content} (Importance: {memory.importance:.2f})")
    
    # Demonstrate conversation engine
    print_section_header("Conversation Engine")
    
    # Set base animal for more interesting conversations
    pet_manager.set_base_animal("chameleon")
    
    # Update pet data for AI manager
    pet_data = {
        'name': pet.name,
        'species': pet.species,
        'aura_color': pet.aura_color,
        'mood': pet_manager._get_current_mood(),
        'base_animal': pet.base_animal
    }
    pet_manager.ai_manager.pet_data = pet_data
    
    # Demonstrate different types of conversations
    conversations = [
        "Hello Luna! How are you today?",
        "What do you know about chameleons?",
        "Do you want to play a game?",
        "You're such a smart and beautiful pet!",
        "Tell me about adaptations that chameleons have.",
        "I really like reading fantasy books and solving puzzles.",
        "I don't like when it's too noisy or crowded."
    ]
    
    print("Demonstrating conversations with different topics:")
    for message in conversations:
        print(f"\nYou: {message}")
        response = pet_manager.ai_manager.generate_chat_response(message)
        print(f"Luna: {response}")
        time.sleep(1)  # Pause for readability
    
    # Demonstrate memory recall in conversation
    print_section_header("Memory Recall in Conversation")
    
    print("Asking about something previously mentioned:")
    message = "What do you remember about puzzles?"
    print(f"\nYou: {message}")
    response = pet_manager.ai_manager.generate_chat_response(message)
    print(f"Luna: {response}")
    
    # Demonstrate personality influence on responses
    print_section_header("Personality Influence on Responses")
    
    # Temporarily adjust personality traits to show difference
    original_playfulness = pet_manager.ai_manager.personality.get_trait('playfulness')
    
    print("Setting playfulness to high (90)...")
    pet_manager.ai_manager.personality.traits['playfulness'] = 90
    
    message = "Do you want to play outside?"
    print(f"\nYou (with high playfulness): {message}")
    response = pet_manager.ai_manager.generate_chat_response(message)
    print(f"Luna: {response}")
    
    print("\nSetting playfulness to low (10)...")
    pet_manager.ai_manager.personality.traits['playfulness'] = 10
    
    print(f"\nYou (with low playfulness): {message}")
    response = pet_manager.ai_manager.generate_chat_response(message)
    print(f"Luna: {response}")
    
    # Restore original value
    pet_manager.ai_manager.personality.traits['playfulness'] = original_playfulness
    
    print_section_header("Demo Complete")
    print("This concludes the demonstration of the enhanced AI integration.")
    print("You can now run the full application with 'python integrated_main.py'")

if __name__ == "__main__":
    demo()# pet/ai_demo.py
"""
Demonstration script for the enhanced AI integration in CritterCraft.
This script showcases the dynamic personality system and conversational capabilities.
"""

import time
from integrated_core import IntegratedPet, IntegratedPetManager
from ai_integration import AIIntegrationManager, DynamicPersonality, MemorySystem
from config import PET_ARCHETYPES, PET_AURA_COLORS, CRITTER_TYPES, ADAPTATIONS

def print_section_header(title):
    """Print a section header for the demo."""
    print("\n" + "=" * 60)
    print(f" {title} ".center(60, "="))
    print("=" * 60)

def demo():
    """Run a demonstration of the enhanced AI integration."""
    print_section_header("CritterCraft Enhanced AI Integration Demo")
    print("This demo showcases the dynamic personality system and conversational capabilities.")
    
    # Create a new pet
    print_section_header("Creating a New Pet")
    pet = IntegratedPet(
        name="Luna",
        species="sprite_crystal",
        aura_color="aura-purple"
    )
    pet_manager = IntegratedPetManager(pet)
    
    print(f"Created a new pet named {pet.name}!")
    print(f"Species: {PET_ARCHETYPES[pet.species]['display_name']}")
    print(f"Aura: {PET_AURA_COLORS[pet.aura_color]['display_name']}")
    
    # Demonstrate personality system
    print_section_header("Dynamic Personality System")
    
    print("Initial personality traits:")
    for trait, value in pet_manager.ai_manager.personality.traits.items():
        level = pet_manager.ai_manager.personality.get_trait_level(trait)
        print(f"{trait.capitalize()}: {value}/100 ({level})")
    
    print("\nDominant traits:")
    dominant_traits = pet_manager.ai_manager.personality.get_dominant_traits(2)
    for trait, value in dominant_traits:
        print(f"{trait.capitalize()}: {value}/100")
    
    print("\nAdjusting traits based on interactions...")
    
    # Simulate some interactions to change personality
    print("Simulating multiple play interactions...")
    for _ in range(5):
        pet_manager.ai_manager.process_interaction('play', True)
    
    print("Simulating multiple chat interactions...")
    for _ in range(3):
        pet_manager.ai_manager.process_interaction('chat', True)
    
    print("\nUpdated personality traits:")
    for trait, value in pet_manager.ai_manager.personality.traits.items():
        level = pet_manager.ai_manager.personality.get_trait_level(trait)
        print(f"{trait.capitalize()}: {value}/100 ({level})")
    
    # Demonstrate memory system
    print_section_header("Memory System")
    
    print("Adding memories...")
    pet_manager.ai_manager.learn_fact("Chameleons can move their eyes independently of each other.")
    pet_manager.ai_manager.learn_fact("Crystal formations occur when atoms arrange in a repeating pattern.")
    pet_manager.ai_manager.learn_fact("The purple aura is associated with intuition and spiritual awareness.")
    
    print("\nRecording preferences...")
    pet_manager.ai_manager.memory_system.add_preference_memory("User likes: playing puzzle games")
    pet_manager.ai_manager.memory_system.add_preference_memory("User dislikes: loud noises")
    
    print("\nRecording milestone...")
    pet_manager.ai_manager.record_milestone("First conversation with Luna")
    
    print("\nMemories by type:")
    for memory_type in ['fact', 'preference', 'milestone']:
        memories = pet_manager.ai_manager.memory_system.get_memories_by_type(memory_type)
        print(f"\n{memory_type.capitalize()} memories ({len(memories)}):")
        for memory in memories:
            print(f"- {memory.content} (Importance: {memory.importance:.2f})")
    
    # Demonstrate conversation engine
    print_section_header("Conversation Engine")
    
    # Set base animal for more interesting conversations
    pet_manager.set_base_animal("chameleon")
    
    # Update pet data for AI manager
    pet_data = {
        'name': pet.name,
        'species': pet.species,
        'aura_color': pet.aura_color,
        'mood': pet_manager._get_current_mood(),
        'base_animal': pet.base_animal
    }
    pet_manager.ai_manager.pet_data = pet_data
    
    # Demonstrate different types of conversations
    conversations = [
        "Hello Luna! How are you today?",
        "What do you know about chameleons?",
        "Do you want to play a game?",
        "You're such a smart and beautiful pet!",
        "Tell me about adaptations that chameleons have.",
        "I really like reading fantasy books and solving puzzles.",
        "I don't like when it's too noisy or crowded."
    ]
    
    print("Demonstrating conversations with different topics:")
    for message in conversations:
        print(f"\nYou: {message}")
        response = pet_manager.ai_manager.generate_chat_response(message)
        print(f"Luna: {response}")
        time.sleep(1)  # Pause for readability
    
    # Demonstrate memory recall in conversation
    print_section_header("Memory Recall in Conversation")
    
    print("Asking about something previously mentioned:")
    message = "What do you remember about puzzles?"
    print(f"\nYou: {message}")
    response = pet_manager.ai_manager.generate_chat_response(message)
    print(f"Luna: {response}")
    
    # Demonstrate personality influence on responses
    print_section_header("Personality Influence on Responses")
    
    # Temporarily adjust personality traits to show difference
    original_playfulness = pet_manager.ai_manager.personality.get_trait('playfulness')
    
    print("Setting playfulness to high (90)...")
    pet_manager.ai_manager.personality.traits['playfulness'] = 90
    
    message = "Do you want to play outside?"
    print(f"\nYou (with high playfulness): {message}")
    response = pet_manager.ai_manager.generate_chat_response(message)
    print(f"Luna: {response}")
    
    print("\nSetting playfulness to low (10)...")
    pet_manager.ai_manager.personality.traits['playfulness'] = 10
    
    print(f"\nYou (with low playfulness): {message}")
    response = pet_manager.ai_manager.generate_chat_response(message)
    print(f"Luna: {response}")
    
    # Restore original value
    pet_manager.ai_manager.personality.traits['playfulness'] = original_playfulness
    
    print_section_header("Demo Complete")
    print("This concludes the demonstration of the enhanced AI integration.")
    print("You can now run the full application with 'python integrated_main.py'")

if __name__ == "__main__":
    demo()