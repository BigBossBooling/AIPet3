def validate_pet_name(name):
    if not isinstance(name, str) or len(name) < 1:
        raise ValueError("Pet name must be a non-empty string.")
    return True

def validate_pet_age(age):
    if not isinstance(age, int) or age < 0:
        raise ValueError("Pet age must be a non-negative integer.")
    return True

def format_reward_points(points):
    if not isinstance(points, int) or points < 0:
        raise ValueError("Reward points must be a non-negative integer.")
    return f"{points} points"

def calculate_experience(current_exp, exp_gain):
    if not isinstance(current_exp, int) or not isinstance(exp_gain, int):
        raise ValueError("Experience values must be integers.")
    return current_exp + exp_gain

def is_valid_stat(stat, min_value):
    if not isinstance(stat, (int, float)) or stat < min_value:
        return False
    return True

def format_achievement_message(achievement_name, points):
    return f"Achievement Unlocked: {achievement_name}! You earned {points} points."
