"""
Formulas module for the battle system.

This module centralizes all game-balancing math, including formulas for
calculating damage, evasion chance, status effect probability, etc.
"""

import random
from typing import Tuple

from .state import BattlePet, StatusEffect


def calculate_damage(attacker: BattlePet, defender: BattlePet, base_power: int, critical: bool = False) -> int:
    """
    Calculate the damage dealt by an attack.
    
    Args:
        attacker: The attacking pet
        defender: The defending pet
        base_power: The base power of the attack
        critical: Whether this is a critical hit
        
    Returns:
        The amount of damage to be dealt
    """
    # Get effective attack and defense values
    attack = attacker.get_effective_attack()
    defense = defender.defense
    
    # Calculate raw damage
    damage_ratio = attack / (attack + defense)
    raw_damage = base_power * damage_ratio * 2  # Multiply by 2 to make the formula more impactful
    
    # Apply critical hit bonus
    if critical:
        raw_damage *= 1.5
    
    # Apply random variance (±10%)
    variance = random.uniform(0.9, 1.1)
    final_damage = max(1, int(raw_damage * variance))
    
    return final_damage


def check_hit(attacker: BattlePet, defender: BattlePet) -> Tuple[bool, bool]:
    """
    Check if an attack hits and if it's a critical hit.
    
    Args:
        attacker: The attacking pet
        defender: The defending pet
        
    Returns:
        A tuple of (hit, critical) booleans
    """
    # Get effective accuracy and evasion
    accuracy = attacker.get_effective_accuracy()
    evasion = defender.get_effective_evasion()
    
    # Calculate hit chance
    hit_chance = min(95, max(5, accuracy - evasion))  # Clamp between 5% and 95%
    
    # Check if the attack hits
    hit = random.random() * 100 < hit_chance
    
    # Check for critical hit (base 10% chance)
    critical_chance = 10
    
    # Increase critical chance if attacker has INSPIRED status
    for status in attacker.status_effects:
        if status.effect == StatusEffect.INSPIRED:
            critical_chance += 15 * status.potency
    
    critical = random.random() * 100 < critical_chance
    
    # If the attack misses, it can't be critical
    if not hit:
        critical = False
    
    return hit, critical


def check_status_effect_application(user: BattlePet, target: BattlePet, base_chance: float) -> bool:
    """
    Check if a status effect is successfully applied.
    
    Args:
        user: The pet applying the status effect
        target: The target pet
        base_chance: The base chance of success (0.0 to 1.0)
        
    Returns:
        True if the status effect is applied, False otherwise
    """
    # Calculate success chance based on relative levels and stats
    level_factor = 1.0 + (user.level - target.level) * 0.05  # ±5% per level difference
    
    # Adjust for any relevant status effects
    for status in user.status_effects:
        if status.effect == StatusEffect.EMPOWERED:
            level_factor *= 1.1  # 10% boost to status effect application
    
    final_chance = min(0.95, max(0.05, base_chance * level_factor))  # Clamp between 5% and 95%
    
    return random.random() < final_chance


def calculate_turn_order(pets: list[BattlePet]) -> list[BattlePet]:
    """
    Calculate the turn order based on pet speed.
    
    Args:
        pets: List of pets in the battle
        
    Returns:
        List of pets in turn order
    """
    # Sort by speed, highest first
    return sorted(pets, key=lambda pet: pet.speed, reverse=True)


def apply_status_effect_damage(pet: BattlePet) -> Tuple[int, list[str]]:
    """
    Apply damage from status effects like poison and burn.
    
    Args:
        pet: The pet to apply status effect damage to
        
    Returns:
        A tuple of (total_damage, messages)
    """
    total_damage = 0
    messages = []
    
    for status in pet.status_effects:
        if status.effect == StatusEffect.POISONED:
            # Poison deals 5% of max stamina per turn
            damage = max(1, int(pet.max_stamina * 0.05 * status.potency))
            pet.current_stamina = max(0, pet.current_stamina - damage)
            total_damage += damage
            messages.append(f"{pet.name} takes {damage} poison damage!")
        
        elif status.effect == StatusEffect.BURNED:
            # Burn deals 3% of max stamina per turn
            damage = max(1, int(pet.max_stamina * 0.03 * status.potency))
            pet.current_stamina = max(0, pet.current_stamina - damage)
            total_damage += damage
            messages.append(f"{pet.name} takes {damage} burn damage!")
    
    # Check if pet is pacified
    if pet.current_stamina <= 0 and not pet.is_pacified():
        pet.add_status_effect(StatusEffect.PACIFIED, 999, source="Stamina depleted")
        messages.append(f"{pet.name} is pacified and can no longer battle!")
    
    return total_damage, messages