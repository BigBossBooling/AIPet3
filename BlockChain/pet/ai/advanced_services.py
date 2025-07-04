from .advanced_constants import (
    JOB_TYPES, BATTLE_OPPONENTS, AVAILABLE_QUESTS, EDUCATION_SUBJECTS,
    EDUCATION_DEGREES, EDUCATION_CERTIFICATIONS, EVOLUTION_PATHS,
    ACHIEVEMENTS, DNA_TRAITS, DNA_MUTATIONS
)

# --- Job System ---
def can_take_job(pet, job_key):
    job = JOB_TYPES[job_key]
    for stat, min_val in job["requirements"].get("min_stats", {}).items():
        if pet.stats.get(stat, 0) < min_val:
            return False
    if pet.age < job["requirements"].get("min_age", 0):
        return False
    return True

def perform_job(pet, job_key):
    if not can_take_job(pet, job_key):
        return False, "Requirements not met."
    job = JOB_TYPES[job_key]
    pet.exp += job["exp_per_work"]
    pet.coins += job["base_salary"]
    # Optionally: increase skills, log job, etc.
    return True, f"{pet.name} worked as a {job['display_name']}!"

# --- Battle System ---
def battle(pet, opponent_key):
    opponent = BATTLE_OPPONENTS[opponent_key]
    pet_power = pet.stats.get("energy", 0) + pet.stats.get("iq", 0) // 2
    if pet_power >= opponent["power"]:
        pet.coins += opponent["reward"]
        # Optionally: track wins, unlock achievements
        return True, f"{pet.name} defeated {opponent['display_name']}!"
    else:
        # Optionally: apply stat penalty
        return False, f"{pet.name} lost to {opponent['display_name']}."

# --- Quest System ---
def can_start_quest(pet, quest_key):
    quest = AVAILABLE_QUESTS[quest_key]
    if pet.maturity < quest["requirements"].get("min_maturity", 0):
        return False
    return True

def complete_quest(pet, quest_key):
    quest = AVAILABLE_QUESTS[quest_key]
    pet.quest_points += quest["reward_points"]
    # Optionally: log quest, unlock achievements
    return f"{pet.name} completed quest: {quest['name']}!"

# --- Education System ---
def can_earn_degree(pet, degree_key):
    degree = EDUCATION_DEGREES[degree_key]
    for subj, req in degree["requirements"].items():
        if pet.education.get(subj, 0) < req:
            return False
    return True

def earn_degree(pet, degree_key):
    if not can_earn_degree(pet, degree_key):
        return False, "Requirements not met."
    degree = EDUCATION_DEGREES[degree_key]
    pet.level += degree["level_increase"]
    # Optionally: add degree to pet record
    return True, f"{pet.name} earned {degree['display_name']}!"

# --- Evolution System ---
def can_evolve(pet, path_key, stage_idx):
    stage = EVOLUTION_PATHS[path_key][stage_idx]
    if pet.maturity < stage["requirements"]["min_maturity"]:
        return False
    for stat, val in stage["requirements"]["min_stats"].items():
        if pet.stats.get(stat, 0) < val:
            return False
    for ach in stage["requirements"].get("achievements", []):
        if ach not in pet.achievements:
            return False
    return True

def evolve(pet, path_key, stage_idx):
    if not can_evolve(pet, path_key, stage_idx):
        return False, "Requirements not met."
    stage = EVOLUTION_PATHS[path_key][stage_idx]
    for stat, bonus in stage["bonuses"]["stats"].items():
        pet.stats[stat] = min(100, pet.stats.get(stat, 0) + bonus)
    pet.evolution_stage = stage["name"]
    # Optionally: log evolution, unlock achievements
    return True, f"{pet.name} evolved into {stage['name']}!"

# --- DNA/Mutation System ---
def apply_mutation(pet, mutation_idx):
    mutation = DNA_MUTATIONS[mutation_idx]
    for stat, change in mutation["stat_changes"].items():
        pet.stats[stat] = max(0, min(100, pet.stats.get(stat, 0) + change))
    # Optionally: add mutation to pet record
    return f"{pet.name} gained mutation: {mutation['name']}!"

# --- Achievement System ---
def check_achievements(pet):
    unlocked = []
    for key, ach in ACHIEVEMENTS.items():
        progress = pet.achievement_progress.get(key, 0)
        if progress >= ach["required_progress"] and key not in pet.achievements:
            pet.achievements.append(key)
            unlocked.append(ach["name"])
    return unlocked

# --- Example Pet Data Structure (for reference) ---
# class Pet:
#     name: str
#     stats: dict
#     age: int
#     exp: int
#     coins: int
#     maturity: int
#     education: dict
#     level: int
#     evolution_stage: str
#     achievements: list
#     achievement_progress: dict
#     quest_points: int
