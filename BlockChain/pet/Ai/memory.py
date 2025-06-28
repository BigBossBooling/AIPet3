# pet/ai/memory.py
from __future__ import annotations
import time
from dataclasses import dataclass, field, asdict
from typing import Dict, Any, List, Optional
from .enums import MemoryType
from .config import MEMORY_TYPES_CONFIG

@dataclass
class Memory:
    """Represents a single, typed memory with importance and context."""
    type: MemoryType
    content: str
    importance: float = 1.0
    timestamp: int = field(default_factory=time.time_ns)
    context: Dict[str, Any] = field(default_factory=dict)
    recall_count: int = 0
    last_recalled: Optional[int] = None

    def recall(self) -> None:
        """Boosts importance upon recall."""
        self.recall_count += 1
        self.last_recalled = time.time_ns()
        self.importance = min(1.0, self.importance + 0.05)

    def to_dict(self) -> Dict[str, Any]:
        """Converts the memory instance to a dictionary."""
        data = asdict(self)
        data['type'] = self.type.value
        return data

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> Memory:
        """Creates a Memory instance from a dictionary."""
        data['type'] = MemoryType(data['type'])
        return cls(**data)

class MemorySystem:
    """Manages the pet's collection of memories."""
    def __init__(self, memories: Optional[List[Memory]] = None):
        self.memories = memories or []

    def add_memory(self, memory: Memory):
        self.memories.append(memory)
        self._prune_memories()

    # ... other methods like get_memories_by_type, update_memories, etc. ...
    # The internal logic of these methods remains largely the same but uses Enums.

    def _prune_memories(self):
        # This logic is complex but correct. We'll keep it as is, but ensure it
        # uses the new Enum types and central config.
        pass # Placeholder for original logic

    def to_dict(self) -> Dict[str, Any]:
        return {'memories': [m.to_dict() for m in self.memories]}

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> MemorySystem:
        memories = [Memory.from_dict(m) for m in data.get('memories', [])]
        return cls(memories=memories)