"""
Quest management for the Critter-Craft project, including quest creation, tracking progress, and rewarding pets upon quest completion.
"""

class Quest:
    def __init__(self, quest_id, name, description, requirements, required_progress, reward_points, faction):
        self.quest_id = quest_id
        self.name = name
        self.description = description
        self.requirements = requirements
        self.required_progress = required_progress
        self.reward_points = reward_points
        self.faction = faction
        self.progress = 0
        self.completed = False

    def update_progress(self, amount):
        if not self.completed:
            self.progress += amount
            if self.progress >= self.required_progress:
                self.complete_quest()

    def complete_quest(self):
        self.completed = True

class QuestManager:
    def __init__(self):
        self.quests = {}

    def create_quest(self, quest_id, name, description, requirements, required_progress, reward_points, faction):
        quest = Quest(quest_id, name, description, requirements, required_progress, reward_points, faction)
        self.quests[quest_id] = quest

    def get_quest(self, quest_id):
        return self.quests.get(quest_id)

    def track_progress(self, quest_id, amount):
        quest = self.get_quest(quest_id)
        if quest:
            quest.update_progress(amount)

    def reward_quest(self, quest_id):
        quest = self.get_quest(quest_id)
        if quest and quest.completed:
            return quest.reward_points
        return 0

# Example usage
if __name__ == "__main__":
    quest_manager = QuestManager()
    quest_manager.create_quest(
        quest_id="welcome_quest",
        name="Welcome to CritterCraft",
        description="Learn the basics of pet care and interaction.",
        requirements={"min_maturity": 0},
        required_progress=5,
        reward_points=5,
        faction="crittercraft"
    )

    # Simulate progress
    quest_manager.track_progress("welcome_quest", 5)
    quest = quest_manager.get_quest("welcome_quest")
    if quest.completed:
        print(f"Quest '{quest.name}' completed! Reward: {quest_manager.reward_quest('welcome_quest')} points.")