"""
Evolution Service for managing pet evolution mechanics, including checking requirements and processing evolutions.
"""

from pet.advanced_constants import EVOLUTION_PATHS

class EvolutionService:
    def __init__(self, pet):
        self.pet = pet

    def check_evolution_requirements(self, evolution_name):
        """
        Check if the pet meets the requirements for a specific evolution.
        """
        evolution_path = EVOLUTION_PATHS.get(evolution_name)
        if not evolution_path:
            return False
        
        current_stage = self.get_current_stage(evolution_path)
        if current_stage is None:
            return False
        
        requirements = current_stage['requirements']
        return (self.pet.age >= requirements.get('min_maturity', 0) and
                all(self.pet.stats.get(stat, 0) >= value for stat, value in requirements.get('min_stats', {}).items()) and
                all(achievement in self.pet.achievements for achievement in requirements.get('achievements', [])))

    def evolve(self, evolution_name):
        """
        Process the evolution of the pet if requirements are met.
        """
        if self.check_evolution_requirements(evolution_name):
            evolution_path = EVOLUTION_PATHS[evolution_name]
            current_stage = self.get_current_stage(evolution_path)
            next_stage = self.get_next_stage(evolution_path, current_stage)

            if next_stage:
                self.pet.evolution_stage = next_stage['name']
                self.apply_bonuses(next_stage['bonuses'])
                return True
        return False

    def get_current_stage(self, evolution_path):
        """
        Retrieve the current evolution stage of the pet.
        """
        for stage in evolution_path:
            if stage['name'] == self.pet.evolution_stage:
                return stage
        return None

    def get_next_stage(self, evolution_path, current_stage):
        """
        Get the next evolution stage based on the current stage.
        """
        current_index = evolution_path.index(current_stage)
        if current_index + 1 < len(evolution_path):
            return evolution_path[current_index + 1]
        return None

    def apply_bonuses(self, bonuses):
        """
        Apply bonuses granted by the evolution to the pet's stats.
        """
        for stat, value in bonuses.get('stats', {}).items():
            self.pet.stats[stat] = self.pet.stats.get(stat, 0) + value
"""