# pet/ai/enums.py
from enum import Enum, auto

class Topic(Enum):
    GREETING = auto()
    WELL_BEING = auto()
    PLAY = auto()
    FOOD = auto()
    LEARNING = auto()
    COMPLIMENT = auto()
    ENVIRONMENT = auto()
    WEATHER = auto()
    TIME = auto()
    EMOTIONS = auto()
    ADVENTURE = auto()
    FRIENDSHIP = auto()
    ANIMALS = auto()
    ADAPTATIONS = auto()
    CRAFTING = auto()
    GENERAL = auto()

class MemoryType(Enum):
    INTERACTION = "interaction"  # User interactions and conversations
    PREFERENCE = "preference"    # User preferences and likes/dislikes
    FACT = "fact"               # General knowledge and learned information
    MILESTONE = "milestone"     # Significant events and achievements
    BEHAVIOR = "behavior"       # User behavior patterns and habits
    EMOTIONAL = "emotional"     # Emotional responses and reactions
    LEARNING = "learning"       # Learning progress and achievements
    SOCIAL = "social"           # Social interactions and relationships
    ENVIRONMENT = "environment" # Environmental interactions and experiences
    PHYSICAL = "physical"       # Physical activities and experiences

class PersonalityTrait(Enum):
    PLAYFULNESS = "playfulness"     # Level of playfulness and fun
    CURIOSITY = "curiosity"        # Desire to learn and explore
    SOCIABILITY = "sociability"    # Preference for social interaction
    INDEPENDENCE = "independence"  # Level of self-reliance
    LOYALTY = "loyalty"           # Commitment to relationships
    ADAPTABILITY = "adaptability"  # Ability to adjust to new situations
    EMOTIONALITY = "emotionality"  # Emotional expressiveness
    INTELLIGENCE = "intelligence"  # Cognitive capabilities
    CREATIVITY = "creativity"     # Creative thinking and problem-solving
    RESILIENCE = "resilience"     # Ability to bounce back from challenges

class Mood(Enum):
    ECSTATIC = "Ecstatic"          # Extremely happy and excited
    HAPPY = "Happy"               # Generally positive and content
    NEUTRAL = "Neutral"           # Neither positive nor negative
    GRUMPY = "Grumpy"             # Irritated or annoyed
    SAD = "Sad"                  # Unhappy or disappointed
    MISERABLE = "Miserable"       # Extremely unhappy
    ANXIOUS = "Anxious"           # Worried or nervous
    CALM = "Calm"                # Peaceful and relaxed
    EXCITED = "Excited"          # Energetic and enthusiastic
    BORED = "Bored"             # Uninterested or disengaged
    CONFUSED = "Confused"        # Uncertain or perplexed
    SURPRISED = "Surprised"      # Unexpected or startled
    RELIEVED = "Relieved"        # Free from worry or distress
    FRUSTRATED = "Frustrated"    # Irritated by obstacles
    INSPIRED = "Inspired"        # Motivated and creative
    CONTENT = "Content"          # Satisfied and at ease