import logging
from typing import Dict, Any, Optional
from .memory import MemorySystem, Memory
from .personality import DynamicPersonality
from .conversation import ConversationEngine
from .enums import PersonalityTrait, MemoryType

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class AIIntegrationManager:
    """A façade that manages and integrates all AI subsystems for a pet."""

    def __init__(self, pet_data: Dict[str, Any], saved_state: Optional[Dict] = None):
        self.pet_data = pet_data
        if saved_state:
            self.personality = DynamicPersonality.from_dict(saved_state['personality'])
            self.memory = MemorySystem.from_dict(saved_state['memory'])
        else:
            self.personality = DynamicPersonality()
            self.memory = MemorySystem()
            self.initialize_personality_traits(pet_data.get('personality_traits', {}))

        self.conversation_engine = ConversationEngine(self.personality, self.memory)

    def initialize_personality_traits(self, traits: Dict[str, Any]):
        """Initializes personality traits from provided data."""
        for trait, level in traits.items():
            try:
                self.personality.adjust_trait(PersonalityTrait[trait], level)
            except KeyError:
                logger.warning(f"Unknown personality trait: {trait}")

    def generate_chat_response(self, message: str) -> str:
        """Generates a chat response based on the user's message."""
        response = self.conversation_engine.generate_response(message, self.pet_data)
        logger.info(f"Generated response: {response}")
        return response

    def process_interaction(self, interaction_type: str, success: bool, details: Optional[Dict] = None):
        """Processes an interaction and updates personality traits and memory."""
        logger.info(f"Processing interaction: {interaction_type}, success: {success}, details: {details}")
        # Example: adjust sociability for all interactions, can be expanded per interaction_type
        self.personality.adjust_trait(PersonalityTrait.SOCIABILITY, 1 if success else -1)
        self.personality.process_interaction(interaction_type, success, details)

        # Add memory of the interaction
        memory = Memory(
            type=MemoryType.INTERACTION,
            content=f"Interaction Type: {interaction_type}, Success: {success}",
            importance=0.5 if success else 0.2,
            context=details or {}
        )
        self.memory.add_memory(memory)

    def to_dict(self) -> Dict[str, Any]:
        """Converts the AI integration manager state to a dictionary."""
        return { 
            'pet_data': self.pet_data,
            'personality': self.personality.to_dict(),
            'memory': self.memory.to_dict()
        }