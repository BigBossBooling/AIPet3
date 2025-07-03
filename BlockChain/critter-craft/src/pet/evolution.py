"""
This file implements evolution mechanics for pets, including evolution paths, requirements for evolution, and bonuses granted upon evolution.
"""

from advanced_constants import EVOLUTION_PATHS

class Evolution:
    def __init__(self, pet):
        self.pet = pet

    def can_evolve(self, evolution_name):
        """Check if the pet can evolve to the specified evolution."""
        evolution_path = EVOLUTION_PATHS.get(evolution_name)
        if not evolution_path:
            return False
        
        current_stage = self.get_current_stage(evolution_path)
        if current_stage is None:
            return False
        
        requirements = current_stage['requirements']
        return (self.pet.age >= requirements.get('min_maturity', 0) and
                all(self.pet.stats[stat] >= value for stat, value in requirements.get('min_stats', {}).items()) and
                all(achievement in self.pet.achievements for achievement in requirements.get('achievements', [])))

    def evolve(self, evolution_name):
        """Evolve the pet to the next stage in the specified evolution path."""
        if self.can_evolve(evolution_name):
            evolution_path = EVOLUTION_PATHS[evolution_name]
            current_stage = self.get_current_stage(evolution_path)
            if current_stage is not None:
                next_stage_name = current_stage['potential_next'][0] if current_stage['potential_next'] else None
                if next_stage_name:
                    self.pet.evolution_stage = next_stage_name
                    self.apply_bonuses(current_stage['bonuses'])
                    return True
        return False

    def get_current_stage(self, evolution_path):
        """Get the current stage of the pet in the evolution path."""
        for stage in evolution_path:
            if stage['name'] == self.pet.evolution_stage:
                return stage
        return None

    def apply_bonuses(self, bonuses):
        """Apply bonuses granted upon evolution."""
        for stat, bonus in bonuses.get('stats', {}).items():
            self.pet.stats[stat] += bonus
```