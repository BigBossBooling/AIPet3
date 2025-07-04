"""
Items module for the battle system.

This module defines all the craftable items that can be used in battle,
including consumables (used during battle) and gear (equipped before battle).
"""

from abc import ABC, abstractmethod
from typing import Dict, List, Optional, Tuple

from .state import BattlePet, StatusEffect


class Item(ABC):
    """Abstract base class for all items."""
    
    def __init__(self, name: str, description: str, rarity: str = "Common"):
        self.name = name
        self.description = description
        self.rarity = rarity
    
    @abstractmethod
    def get_effects(self) -> Dict:
        """Get the effects of this item."""
        pass


class Consumable(Item):
    """An item that can be used during battle for a one-time effect."""
    
    def __init__(self, name: str, description: str, ap_cost: int = 1, rarity: str = "Common"):
        super().__init__(name, description, rarity)
        self.ap_cost = ap_cost
    
    @abstractmethod
    def use(self, user: BattlePet, target: Optional[BattlePet] = None) -> Tuple[List[str], Dict]:
        """
        Use the consumable item.
        
        Args:
            user: The pet using the item
            target: The target pet (if applicable)
            
        Returns:
            A tuple containing:
            - A list of messages describing what happened
            - A dictionary of additional effects/data
        """
        pass
    
    def get_effects(self) -> Dict:
        """Get the effects of this consumable."""
        return {"type": "consumable", "ap_cost": self.ap_cost}
    
    def can_use(self, user: BattlePet) -> bool:
        """Check if the pet can use this item."""
        return user.current_ap >= self.ap_cost and not user.is_pacified()


class Gear(Item):
    """An item that is equipped before battle for passive effects."""
    
    def __init__(self, name: str, description: str, slot: str, rarity: str = "Common"):
        super().__init__(name, description, rarity)
        self.slot = slot  # e.g., "armor", "accessory", "tool"
    
    @abstractmethod
    def apply_effects(self, pet: BattlePet) -> None:
        """
        Apply the gear's effects to a pet.
        
        Args:
            pet: The pet equipping the gear
        """
        pass
    
    def get_effects(self) -> Dict:
        """Get the effects of this gear."""
        return {"type": "gear", "slot": self.slot}


class HealingSalve(Consumable):
    """A consumable that restores stamina."""
    
    def __init__(self, potency: float = 1.0):
        super().__init__(
            name="Healing Salve",
            description="A soothing salve that restores stamina.",
            ap_cost=1,
            rarity="Common"
        )
        self.potency = potency
    
    def use(self, user: BattlePet, target: Optional[BattlePet] = None) -> Tuple[List[str], Dict]:
        messages = []
        result = {"healing": 0}
        
        # Calculate healing amount (20% of max stamina, modified by potency)
        healing = int(user.max_stamina * 0.2 * self.potency)
        
        # Apply healing
        old_stamina = user.current_stamina
        user.current_stamina = min(user.max_stamina, user.current_stamina + healing)
        actual_healing = user.current_stamina - old_stamina
        
        result["healing"] = actual_healing
        
        messages.append(f"{user.name} uses a Healing Salve and recovers {actual_healing} stamina!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class AdrenalineBerry(Consumable):
    """A consumable that grants additional AP for the current turn."""
    
    def __init__(self):
        super().__init__(
            name="Adrenaline Berry",
            description="A stimulating berry that grants +2 AP for the current turn.",
            ap_cost=1,
            rarity="Uncommon"
        )
    
    def use(self, user: BattlePet, target: Optional[BattlePet] = None) -> Tuple[List[str], Dict]:
        messages = []
        result = {"ap_gained": 2}
        
        # Grant additional AP
        user.current_ap += 2  # +2 AP (net +1 after the cost of using the item)
        
        messages.append(f"{user.name} eats an Adrenaline Berry and gains 2 AP!")
        
        return messages, result


class FocusRoot(Consumable):
    """A consumable that cures the Blinded status effect."""
    
    def __init__(self):
        super().__init__(
            name="Focus Root",
            description="A medicinal root that cures the Blinded status effect.",
            ap_cost=1,
            rarity="Common"
        )
    
    def use(self, user: BattlePet, target: Optional[BattlePet] = None) -> Tuple[List[str], Dict]:
        messages = []
        result = {"status_cured": False}
        
        # Check if the user is blinded
        is_blinded = any(s.effect == StatusEffect.BLINDED for s in user.status_effects)
        
        if is_blinded:
            # Remove the blinded status
            user.remove_status_effect(StatusEffect.BLINDED)
            
            messages.append(f"{user.name} chews on a Focus Root and can see clearly again!")
            result["status_cured"] = True
        else:
            messages.append(f"{user.name} uses a Focus Root, but it has no effect!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class ThickMud(Consumable):
    """A consumable that can be thrown to slow the opponent."""
    
    def __init__(self):
        super().__init__(
            name="Thick Mud",
            description="Can be thrown at an opponent to slow them for one turn.",
            ap_cost=1,
            rarity="Common"
        )
    
    def use(self, user: BattlePet, target: Optional[BattlePet] = None) -> Tuple[List[str], Dict]:
        from .formulas import check_status_effect_application
        
        messages = []
        result = {"status_applied": False}
        
        if not target:
            messages.append(f"{user.name} has no target to throw the Thick Mud at!")
            return messages, result
        
        # Check if the mud hits
        success = check_status_effect_application(user, target, 0.7)  # 70% base chance
        
        if success:
            # Apply slowed status
            target.add_status_effect(
                StatusEffect.SLOWED,
                duration=1,
                potency=1.0,
                source="Thick Mud"
            )
            
            messages.append(f"{user.name} throws Thick Mud at {target.name}, slowing them down!")
            result["status_applied"] = True
        else:
            messages.append(f"{user.name} throws Thick Mud, but {target.name} dodges it!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class ToughenedBarkArmor(Gear):
    """Gear that provides a passive defense bonus."""
    
    def __init__(self):
        super().__init__(
            name="Toughened Bark Armor",
            description="Provides a passive +10 defense.",
            slot="armor",
            rarity="Common"
        )
    
    def apply_effects(self, pet: BattlePet) -> None:
        # Increase defense
        pet.defense += 10
    
    def get_effects(self) -> Dict:
        effects = super().get_effects()
        effects["defense_bonus"] = 10
        return effects


class PolishedRiverStone(Gear):
    """Gear that increases resistance to burn effects."""
    
    def __init__(self):
        super().__init__(
            name="Polished River Stone",
            description="Increases resistance to Burn effects.",
            slot="accessory",
            rarity="Uncommon"
        )
    
    def apply_effects(self, pet: BattlePet) -> None:
        # No direct stat changes, but we'll handle this in the battle logic
        # when burn effects are applied
        pass
    
    def get_effects(self) -> Dict:
        effects = super().get_effects()
        effects["burn_resistance"] = 0.5  # 50% reduction in burn effect potency
        return effects


class AmplifyingCrystal(Gear):
    """Gear that increases the power of elemental abilities."""
    
    def __init__(self):
        super().__init__(
            name="Amplifying Crystal",
            description="Increases the power of aura-based or elemental abilities.",
            slot="accessory",
            rarity="Rare"
        )
    
    def apply_effects(self, pet: BattlePet) -> None:
        # No direct stat changes, but we'll handle this in the battle logic
        # when abilities are used
        pass
    
    def get_effects(self) -> Dict:
        effects = super().get_effects()
        effects["ability_power_boost"] = 0.15  # 15% boost to ability power
        return effects


# Dictionary mapping item names to item classes
ITEM_MAPPING = {
    "healing_salve": HealingSalve,
    "adrenaline_berry": AdrenalineBerry,
    "focus_root": FocusRoot,
    "thick_mud": ThickMud,
    "toughened_bark_armor": ToughenedBarkArmor,
    "polished_river_stone": PolishedRiverStone,
    "amplifying_crystal": AmplifyingCrystal,
}


def get_item(item_name: str) -> Optional[Item]:
    """
    Get an item instance by name.
    
    Args:
        item_name: The name of the item
        
    Returns:
        An instance of the item, or None if not found
    """
    item_class = ITEM_MAPPING.get(item_name.lower().replace(" ", "_"))
    if item_class:
        return item_class()
    return None