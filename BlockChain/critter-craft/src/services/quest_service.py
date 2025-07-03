from src.pet.advanced_constants import AVAILABLE_QUESTS

class QuestService:
    def __init__(self):
        self.active_quests = {}

    def start_quest(self, user_id, quest_id):
        if quest_id in AVAILABLE_QUESTS:
            if user_id not in self.active_quests:
                self.active_quests[user_id] = []
            self.active_quests[user_id].append({
                "quest_id": quest_id,
                "progress": 0,
                "completed": False
            })
            return f"Quest '{AVAILABLE_QUESTS[quest_id]['name']}' started!"
        else:
            return "Quest not found."

    def update_progress(self, user_id, quest_id, progress_increment):
        if user_id in self.active_quests:
            for quest in self.active_quests[user_id]:
                if quest['quest_id'] == quest_id and not quest['completed']:
                    quest['progress'] += progress_increment
                    if quest['progress'] >= AVAILABLE_QUESTS[quest_id]['required_progress']:
                        quest['completed'] = True
                        return f"Quest '{AVAILABLE_QUESTS[quest_id]['name']}' completed!"
                    return f"Progress updated for quest '{AVAILABLE_QUESTS[quest_id]['name']}': {quest['progress']}/{AVAILABLE_QUESTS[quest_id]['required_progress']}"
        return "Quest not found or not started."

    def get_active_quests(self, user_id):
        return self.active_quests.get(user_id, [])

    def reward_user(self, user_id, quest_id):
        if user_id in self.active_quests:
            for quest in self.active_quests[user_id]:
                if quest['quest_id'] == quest_id and quest['completed']:
                    return f"User rewarded with {AVAILABLE_QUESTS[quest_id]['reward_points']} points for completing '{AVAILABLE_QUESTS[quest_id]['name']}'!"
        return "No rewards available."