"""
Abilities module for the battle system.

This module defines all the unique adaptation abilities that critters can use in battle.
Each ability is implemented as a class that inherits from the Ability abstract base class.
"""

from abc import ABC, abstractmethod
from typing import Dict, List, Optional, Tuple

from .state import BattlePet, StatusEffect


class Ability(ABC):
    """Abstract base class for all abilities."""
    
    def __init__(self, name: str, description: str, ap_cost: int):
        self.name = name
        self.description = description
        self.ap_cost = ap_cost
    
    @abstractmethod
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        """
        Execute the ability.
        
        Args:
            user: The pet using the ability
            target: The target pet
            
        Returns:
            A tuple containing:
            - A list of messages describing what happened
            - A dictionary of additional effects/data
        """
        pass
    
    def can_use(self, user: BattlePet) -> bool:
        """Check if the pet can use this ability."""
        return user.current_ap >= self.ap_cost and not user.is_pacified()


class BasicManeuver(Ability):
    """A simple, low-cost attack or defensive action."""
    
    def __init__(self):
        super().__init__(
            name="Basic Maneuver",
            description="A simple, reliable action that deals modest damage.",
            ap_cost=1
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import calculate_damage, check_hit
        
        messages = []
        result = {"damage": 0, "hit": False, "critical": False}
        
        # Check if the attack hits
        hit, critical = check_hit(user, target)
        result["hit"] = hit
        result["critical"] = critical
        
        if hit:
            # Calculate base damage
            base_damage = 5  # Low base damage for basic maneuver
            damage = calculate_damage(user, target, base_damage, critical)
            
            # Apply damage
            target.current_stamina = max(0, target.current_stamina - damage)
            user.damage_dealt += damage
            target.damage_received += damage
            
            result["damage"] = damage
            
            # Create message
            if critical:
                messages.append(f"{user.name} executes a perfect maneuver! {target.name} loses {damage} stamina!")
            else:
                messages.append(f"{user.name} performs a basic maneuver. {target.name} loses {damage} stamina.")
            
            # Check if target is pacified
            if target.current_stamina <= 0:
                messages.append(f"{target.name} is pacified and can no longer battle!")
                target.add_status_effect(StatusEffect.PACIFIED, 999, source="Stamina depleted")
        else:
            messages.append(f"{user.name} attempts a maneuver, but {target.name} avoids it!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Camouflage(Ability):
    """Ability to blend with surroundings, increasing evasion."""
    
    def __init__(self):
        super().__init__(
            name="Camouflage",
            description="Blend with surroundings to become highly evasive for two turns.",
            ap_cost=3
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        messages = []
        result = {"status_applied": True}
        
        # Apply camouflaged status
        user.add_status_effect(
            StatusEffect.CAMOUFLAGED,
            duration=2,
            potency=1.0,
            source="Camouflage ability"
        )
        
        messages.append(f"{user.name} blends with the surroundings, becoming harder to hit!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Bioluminescence(Ability):
    """Ability to produce light, potentially blinding opponents."""
    
    def __init__(self):
        super().__init__(
            name="Bioluminescence",
            description="Emit a bright flash of light that may blind the opponent.",
            ap_cost=2
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import check_status_effect_application
        
        messages = []
        result = {"status_applied": False}
        
        # Check if the status effect is applied
        success = check_status_effect_application(user, target, 0.75)  # 75% base chance
        
        if success:
            # Apply blinded status
            target.add_status_effect(
                StatusEffect.BLINDED,
                duration=2,
                potency=1.0,
                source="Bioluminescence ability"
            )
            
            messages.append(f"{user.name} emits a bright flash! {target.name} is blinded!")
            result["status_applied"] = True
        else:
            messages.append(f"{user.name} emits a flash of light, but {target.name} shields their eyes!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class ColorfulDisplay(Ability):
    """Ability to display vibrant colors to intimidate opponents."""
    
    def __init__(self):
        super().__init__(
            name="Colorful Display",
            description="Show off vibrant colors to intimidate the opponent, lowering their attack power.",
            ap_cost=3
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import check_status_effect_application
        
        messages = []
        result = {"status_applied": False}
        
        # Check if the intimidation works
        success = check_status_effect_application(user, target, 0.7)  # 70% base chance
        
        if success:
            # We'll use BURNED status with negative potency to represent lowered attack
            target.add_status_effect(
                StatusEffect.BURNED,
                duration=2,
                potency=0.8,  # 80% potency means 12% attack reduction
                source="Intimidation from Colorful Display"
            )
            
            messages.append(f"{user.name} displays vibrant colors! {target.name} is intimidated and loses attack power!")
            result["status_applied"] = True
        else:
            messages.append(f"{user.name} displays vibrant colors, but {target.name} isn't impressed!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Echolocation(Ability):
    """Ability to use sound waves to detect opponents, increasing accuracy."""
    
    def __init__(self):
        super().__init__(
            name="Echolocation",
            description="Use sound waves to detect the opponent's position, increasing accuracy and revealing camouflaged targets.",
            ap_cost=2
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        messages = []
        result = {"status_removed": False, "accuracy_boosted": True}
        
        # Boost user's accuracy via INSPIRED status
        user.add_status_effect(
            StatusEffect.INSPIRED,
            duration=2,
            potency=1.0,
            source="Echolocation ability"
        )
        
        messages.append(f"{user.name} uses echolocation to track {target.name}'s movements!")
        
        # Remove CAMOUFLAGED status from target if present
        camouflaged = any(s.effect == StatusEffect.CAMOUFLAGED for s in target.status_effects)
        if camouflaged:
            target.remove_status_effect(StatusEffect.CAMOUFLAGED)
            messages.append(f"{target.name}'s camouflage is rendered ineffective!")
            result["status_removed"] = True
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class VenomStrike(Ability):
    """Ability to inject venom, causing damage over time."""
    
    def __init__(self):
        super().__init__(
            name="Venom Strike",
            description="Inject venom that causes damage over time.",
            ap_cost=3
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import calculate_damage, check_hit, check_status_effect_application
        
        messages = []
        result = {"damage": 0, "hit": False, "poisoned": False}
        
        # Check if the attack hits
        hit, critical = check_hit(user, target)
        result["hit"] = hit
        
        if hit:
            # Calculate base damage (lower immediate damage than basic attack)
            base_damage = 3
            damage = calculate_damage(user, target, base_damage, critical)
            
            # Apply damage
            target.current_stamina = max(0, target.current_stamina - damage)
            user.damage_dealt += damage
            target.damage_received += damage
            
            result["damage"] = damage
            
            messages.append(f"{user.name} strikes with venom! {target.name} loses {damage} stamina.")
            
            # Check if poison is applied
            poison_success = check_status_effect_application(user, target, 0.8)  # 80% base chance
            
            if poison_success:
                target.add_status_effect(
                    StatusEffect.POISONED,
                    duration=3,
                    potency=1.0,
                    source="Venom Strike ability"
                )
                
                messages.append(f"{target.name} is poisoned and will take damage over time!")
                result["poisoned"] = True
            
            # Check if target is pacified
            if target.current_stamina <= 0:
                messages.append(f"{target.name} is pacified and can no longer battle!")
                target.add_status_effect(StatusEffect.PACIFIED, 999, source="Stamina depleted")
        else:
            messages.append(f"{user.name} attempts to strike with venom, but {target.name} avoids it!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Defend(Ability):
    """Ability to defend and gain a defensive bonus."""
    
    def __init__(self):
        super().__init__(
            name="Defend",
            description="Take a defensive stance, reducing damage taken on the next turn.",
            ap_cost=1
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        messages = []
        result = {"defending": True}
        
        # We'll use EMPOWERED with negative potency to represent increased defense
        user.add_status_effect(
            StatusEffect.EMPOWERED,
            duration=1,
            potency=-0.5,  # Negative potency means it's actually a defense boost
            source="Defensive stance"
        )
        
        messages.append(f"{user.name} takes a defensive stance!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


# Dictionary mapping adaptation names to ability classes
ABILITY_MAPPING = {
    "basic_maneuver": BasicManeuver,
    "camouflage": Camouflage,
    "bioluminescence": Bioluminescence,
    "colorful_display": ColorfulDisplay,
    "echolocation": Echolocation,
    "venom_strike": VenomStrike,
    "defend": Defend,
}


def get_ability(ability_name: str) -> Optional[Ability]:
    """
    Get an ability instance by name.
    
    Args:
        ability_name: The name of the ability
        
    Returns:
        An instance of the ability, or None if not found
    """
    ability_class = ABILITY_MAPPING.get(ability_name.lower().replace(" ", "_"))
    if ability_class:
        return ability_class()
    return None"""
Abilities module for the battle system.

This module defines all the unique adaptation abilities that critters can use in battle.
Each ability is implemented as a class that inherits from the Ability abstract base class.
"""

from abc import ABC, abstractmethod
from typing import Dict, List, Optional, Tuple

from .state import BattlePet, StatusEffect


class Ability(ABC):
    """Abstract base class for all abilities."""
    
    def __init__(self, name: str, description: str, ap_cost: int):
        self.name = name
        self.description = description
        self.ap_cost = ap_cost
    
    @abstractmethod
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        """
        Execute the ability.
        
        Args:
            user: The pet using the ability
            target: The target pet
            
        Returns:
            A tuple containing:
            - A list of messages describing what happened
            - A dictionary of additional effects/data
        """
        pass
    
    def can_use(self, user: BattlePet) -> bool:
        """Check if the pet can use this ability."""
        return user.current_ap >= self.ap_cost and not user.is_pacified()


class BasicManeuver(Ability):
    """A simple, low-cost attack or defensive action."""
    
    def __init__(self):
        super().__init__(
            name="Basic Maneuver",
            description="A simple, reliable action that deals modest damage.",
            ap_cost=1
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import calculate_damage, check_hit
        
        messages = []
        result = {"damage": 0, "hit": False, "critical": False}
        
        # Check if the attack hits
        hit, critical = check_hit(user, target)
        result["hit"] = hit
        result["critical"] = critical
        
        if hit:
            # Calculate base damage
            base_damage = 5  # Low base damage for basic maneuver
            damage = calculate_damage(user, target, base_damage, critical)
            
            # Apply damage
            target.current_stamina = max(0, target.current_stamina - damage)
            user.damage_dealt += damage
            target.damage_received += damage
            
            result["damage"] = damage
            
            # Create message
            if critical:
                messages.append(f"{user.name} executes a perfect maneuver! {target.name} loses {damage} stamina!")
            else:
                messages.append(f"{user.name} performs a basic maneuver. {target.name} loses {damage} stamina.")
            
            # Check if target is pacified
            if target.current_stamina <= 0:
                messages.append(f"{target.name} is pacified and can no longer battle!")
                target.add_status_effect(StatusEffect.PACIFIED, 999, source="Stamina depleted")
        else:
            messages.append(f"{user.name} attempts a maneuver, but {target.name} avoids it!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Camouflage(Ability):
    """Ability to blend with surroundings, increasing evasion."""
    
    def __init__(self):
        super().__init__(
            name="Camouflage",
            description="Blend with surroundings to become highly evasive for two turns.",
            ap_cost=3
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        messages = []
        result = {"status_applied": True}
        
        # Apply camouflaged status
        user.add_status_effect(
            StatusEffect.CAMOUFLAGED,
            duration=2,
            potency=1.0,
            source="Camouflage ability"
        )
        
        messages.append(f"{user.name} blends with the surroundings, becoming harder to hit!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Bioluminescence(Ability):
    """Ability to produce light, potentially blinding opponents."""
    
    def __init__(self):
        super().__init__(
            name="Bioluminescence",
            description="Emit a bright flash of light that may blind the opponent.",
            ap_cost=2
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import check_status_effect_application
        
        messages = []
        result = {"status_applied": False}
        
        # Check if the status effect is applied
        success = check_status_effect_application(user, target, 0.75)  # 75% base chance
        
        if success:
            # Apply blinded status
            target.add_status_effect(
                StatusEffect.BLINDED,
                duration=2,
                potency=1.0,
                source="Bioluminescence ability"
            )
            
            messages.append(f"{user.name} emits a bright flash! {target.name} is blinded!")
            result["status_applied"] = True
        else:
            messages.append(f"{user.name} emits a flash of light, but {target.name} shields their eyes!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class ColorfulDisplay(Ability):
    """Ability to display vibrant colors to intimidate opponents."""
    
    def __init__(self):
        super().__init__(
            name="Colorful Display",
            description="Show off vibrant colors to intimidate the opponent, lowering their attack power.",
            ap_cost=3
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import check_status_effect_application
        
        messages = []
        result = {"status_applied": False}
        
        # Check if the intimidation works
        success = check_status_effect_application(user, target, 0.7)  # 70% base chance
        
        if success:
            # We'll use BURNED status with negative potency to represent lowered attack
            target.add_status_effect(
                StatusEffect.BURNED,
                duration=2,
                potency=0.8,  # 80% potency means 12% attack reduction
                source="Intimidation from Colorful Display"
            )
            
            messages.append(f"{user.name} displays vibrant colors! {target.name} is intimidated and loses attack power!")
            result["status_applied"] = True
        else:
            messages.append(f"{user.name} displays vibrant colors, but {target.name} isn't impressed!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Echolocation(Ability):
    """Ability to use sound waves to detect opponents, increasing accuracy."""
    
    def __init__(self):
        super().__init__(
            name="Echolocation",
            description="Use sound waves to detect the opponent's position, increasing accuracy and revealing camouflaged targets.",
            ap_cost=2
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        messages = []
        result = {"status_removed": False, "accuracy_boosted": True}
        
        # Boost user's accuracy via INSPIRED status
        user.add_status_effect(
            StatusEffect.INSPIRED,
            duration=2,
            potency=1.0,
            source="Echolocation ability"
        )
        
        messages.append(f"{user.name} uses echolocation to track {target.name}'s movements!")
        
        # Remove CAMOUFLAGED status from target if present
        camouflaged = any(s.effect == StatusEffect.CAMOUFLAGED for s in target.status_effects)
        if camouflaged:
            target.remove_status_effect(StatusEffect.CAMOUFLAGED)
            messages.append(f"{target.name}'s camouflage is rendered ineffective!")
            result["status_removed"] = True
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class VenomStrike(Ability):
    """Ability to inject venom, causing damage over time."""
    
    def __init__(self):
        super().__init__(
            name="Venom Strike",
            description="Inject venom that causes damage over time.",
            ap_cost=3
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import calculate_damage, check_hit, check_status_effect_application
        
        messages = []
        result = {"damage": 0, "hit": False, "poisoned": False}
        
        # Check if the attack hits
        hit, critical = check_hit(user, target)
        result["hit"] = hit
        
        if hit:
            # Calculate base damage (lower immediate damage than basic attack)
            base_damage = 3
            damage = calculate_damage(user, target, base_damage, critical)
            
            # Apply damage
            target.current_stamina = max(0, target.current_stamina - damage)
            user.damage_dealt += damage
            target.damage_received += damage
            
            result["damage"] = damage
            
            messages.append(f"{user.name} strikes with venom! {target.name} loses {damage} stamina.")
            
            # Check if poison is applied
            poison_success = check_status_effect_application(user, target, 0.8)  # 80% base chance
            
            if poison_success:
                target.add_status_effect(
                    StatusEffect.POISONED,
                    duration=3,
                    potency=1.0,
                    source="Venom Strike ability"
                )
                
                messages.append(f"{target.name} is poisoned and will take damage over time!")
                result["poisoned"] = True
            
            # Check if target is pacified
            if target.current_stamina <= 0:
                messages.append(f"{target.name} is pacified and can no longer battle!")
                target.add_status_effect(StatusEffect.PACIFIED, 999, source="Stamina depleted")
        else:
            messages.append(f"{user.name} attempts to strike with venom, but {target.name} avoids it!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Defend(Ability):
    """Ability to defend and gain a defensive bonus."""
    
    def __init__(self):
        super().__init__(
            name="Defend",
            description="Take a defensive stance, reducing damage taken on the next turn.",
            ap_cost=1
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        messages = []
        result = {"defending": True}
        
        # We'll use EMPOWERED with negative potency to represent increased defense
        user.add_status_effect(
            StatusEffect.EMPOWERED,
            duration=1,
            potency=-0.5,  # Negative potency means it's actually a defense boost
            source="Defensive stance"
        )
        
        messages.append(f"{user.name} takes a defensive stance!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


# Dictionary mapping adaptation names to ability classes
ABILITY_MAPPING = {
    "basic_maneuver": BasicManeuver,
    "camouflage": Camouflage,
    "bioluminescence": Bioluminescence,
    "colorful_display": ColorfulDisplay,
    "echolocation": Echolocation,
    "venom_strike": VenomStrike,
    "defend": Defend,
}


def get_ability(ability_name: str) -> Optional[Ability]:
    """
    Get an ability instance by name.
    
    Args:
        ability_name: The name of the ability
        
    Returns:
        An instance of the ability, or None if not found
    """
    ability_class = ABILITY_MAPPING.get(ability_name.lower().replace(" ", "_"))
    if ability_class:
        return ability_class()
    return None"""
Abilities module for the battle system.

This module defines all the unique adaptation abilities that critters can use in battle.
Each ability is implemented as a class that inherits from the Ability abstract base class.
"""

from abc import ABC, abstractmethod
from typing import Dict, List, Optional, Tuple

from .state import BattlePet, StatusEffect


class Ability(ABC):
    """Abstract base class for all abilities."""
    
    def __init__(self, name: str, description: str, ap_cost: int):
        self.name = name
        self.description = description
        self.ap_cost = ap_cost
    
    @abstractmethod
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        """
        Execute the ability.
        
        Args:
            user: The pet using the ability
            target: The target pet
            
        Returns:
            A tuple containing:
            - A list of messages describing what happened
            - A dictionary of additional effects/data
        """
        pass
    
    def can_use(self, user: BattlePet) -> bool:
        """Check if the pet can use this ability."""
        return user.current_ap >= self.ap_cost and not user.is_pacified()


class BasicManeuver(Ability):
    """A simple, low-cost attack or defensive action."""
    
    def __init__(self):
        super().__init__(
            name="Basic Maneuver",
            description="A simple, reliable action that deals modest damage.",
            ap_cost=1
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import calculate_damage, check_hit
        
        messages = []
        result = {"damage": 0, "hit": False, "critical": False}
        
        # Check if the attack hits
        hit, critical = check_hit(user, target)
        result["hit"] = hit
        result["critical"] = critical
        
        if hit:
            # Calculate base damage
            base_damage = 5  # Low base damage for basic maneuver
            damage = calculate_damage(user, target, base_damage, critical)
            
            # Apply damage
            target.current_stamina = max(0, target.current_stamina - damage)
            user.damage_dealt += damage
            target.damage_received += damage
            
            result["damage"] = damage
            
            # Create message
            if critical:
                messages.append(f"{user.name} executes a perfect maneuver! {target.name} loses {damage} stamina!")
            else:
                messages.append(f"{user.name} performs a basic maneuver. {target.name} loses {damage} stamina.")
            
            # Check if target is pacified
            if target.current_stamina <= 0:
                messages.append(f"{target.name} is pacified and can no longer battle!")
                target.add_status_effect(StatusEffect.PACIFIED, 999, source="Stamina depleted")
        else:
            messages.append(f"{user.name} attempts a maneuver, but {target.name} avoids it!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Camouflage(Ability):
    """Ability to blend with surroundings, increasing evasion."""
    
    def __init__(self):
        super().__init__(
            name="Camouflage",
            description="Blend with surroundings to become highly evasive for two turns.",
            ap_cost=3
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        messages = []
        result = {"status_applied": True}
        
        # Apply camouflaged status
        user.add_status_effect(
            StatusEffect.CAMOUFLAGED,
            duration=2,
            potency=1.0,
            source="Camouflage ability"
        )
        
        messages.append(f"{user.name} blends with the surroundings, becoming harder to hit!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Bioluminescence(Ability):
    """Ability to produce light, potentially blinding opponents."""
    
    def __init__(self):
        super().__init__(
            name="Bioluminescence",
            description="Emit a bright flash of light that may blind the opponent.",
            ap_cost=2
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import check_status_effect_application
        
        messages = []
        result = {"status_applied": False}
        
        # Check if the status effect is applied
        success = check_status_effect_application(user, target, 0.75)  # 75% base chance
        
        if success:
            # Apply blinded status
            target.add_status_effect(
                StatusEffect.BLINDED,
                duration=2,
                potency=1.0,
                source="Bioluminescence ability"
            )
            
            messages.append(f"{user.name} emits a bright flash! {target.name} is blinded!")
            result["status_applied"] = True
        else:
            messages.append(f"{user.name} emits a flash of light, but {target.name} shields their eyes!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class ColorfulDisplay(Ability):
    """Ability to display vibrant colors to intimidate opponents."""
    
    def __init__(self):
        super().__init__(
            name="Colorful Display",
            description="Show off vibrant colors to intimidate the opponent, lowering their attack power.",
            ap_cost=3
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import check_status_effect_application
        
        messages = []
        result = {"status_applied": False}
        
        # Check if the intimidation works
        success = check_status_effect_application(user, target, 0.7)  # 70% base chance
        
        if success:
            # We'll use BURNED status with negative potency to represent lowered attack
            target.add_status_effect(
                StatusEffect.BURNED,
                duration=2,
                potency=0.8,  # 80% potency means 12% attack reduction
                source="Intimidation from Colorful Display"
            )
            
            messages.append(f"{user.name} displays vibrant colors! {target.name} is intimidated and loses attack power!")
            result["status_applied"] = True
        else:
            messages.append(f"{user.name} displays vibrant colors, but {target.name} isn't impressed!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Echolocation(Ability):
    """Ability to use sound waves to detect opponents, increasing accuracy."""
    
    def __init__(self):
        super().__init__(
            name="Echolocation",
            description="Use sound waves to detect the opponent's position, increasing accuracy and revealing camouflaged targets.",
            ap_cost=2
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        messages = []
        result = {"status_removed": False, "accuracy_boosted": True}
        
        # Boost user's accuracy via INSPIRED status
        user.add_status_effect(
            StatusEffect.INSPIRED,
            duration=2,
            potency=1.0,
            source="Echolocation ability"
        )
        
        messages.append(f"{user.name} uses echolocation to track {target.name}'s movements!")
        
        # Remove CAMOUFLAGED status from target if present
        camouflaged = any(s.effect == StatusEffect.CAMOUFLAGED for s in target.status_effects)
        if camouflaged:
            target.remove_status_effect(StatusEffect.CAMOUFLAGED)
            messages.append(f"{target.name}'s camouflage is rendered ineffective!")
            result["status_removed"] = True
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class VenomStrike(Ability):
    """Ability to inject venom, causing damage over time."""
    
    def __init__(self):
        super().__init__(
            name="Venom Strike",
            description="Inject venom that causes damage over time.",
            ap_cost=3
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        from .formulas import calculate_damage, check_hit, check_status_effect_application
        
        messages = []
        result = {"damage": 0, "hit": False, "poisoned": False}
        
        # Check if the attack hits
        hit, critical = check_hit(user, target)
        result["hit"] = hit
        
        if hit:
            # Calculate base damage (lower immediate damage than basic attack)
            base_damage = 3
            damage = calculate_damage(user, target, base_damage, critical)
            
            # Apply damage
            target.current_stamina = max(0, target.current_stamina - damage)
            user.damage_dealt += damage
            target.damage_received += damage
            
            result["damage"] = damage
            
            messages.append(f"{user.name} strikes with venom! {target.name} loses {damage} stamina.")
            
            # Check if poison is applied
            poison_success = check_status_effect_application(user, target, 0.8)  # 80% base chance
            
            if poison_success:
                target.add_status_effect(
                    StatusEffect.POISONED,
                    duration=3,
                    potency=1.0,
                    source="Venom Strike ability"
                )
                
                messages.append(f"{target.name} is poisoned and will take damage over time!")
                result["poisoned"] = True
            
            # Check if target is pacified
            if target.current_stamina <= 0:
                messages.append(f"{target.name} is pacified and can no longer battle!")
                target.add_status_effect(StatusEffect.PACIFIED, 999, source="Stamina depleted")
        else:
            messages.append(f"{user.name} attempts to strike with venom, but {target.name} avoids it!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


class Defend(Ability):
    """Ability to defend and gain a defensive bonus."""
    
    def __init__(self):
        super().__init__(
            name="Defend",
            description="Take a defensive stance, reducing damage taken on the next turn.",
            ap_cost=1
        )
    
    def execute(self, user: BattlePet, target: BattlePet) -> Tuple[List[str], Dict]:
        messages = []
        result = {"defending": True}
        
        # We'll use EMPOWERED with negative potency to represent increased defense
        user.add_status_effect(
            StatusEffect.EMPOWERED,
            duration=1,
            potency=-0.5,  # Negative potency means it's actually a defense boost
            source="Defensive stance"
        )
        
        messages.append(f"{user.name} takes a defensive stance!")
        
        # Deduct AP cost
        user.current_ap -= self.ap_cost
        
        return messages, result


# Dictionary mapping adaptation names to ability classes
ABILITY_MAPPING = {
    "basic_maneuver": BasicManeuver,
    "camouflage": Camouflage,
    "bioluminescence": Bioluminescence,
    "colorful_display": ColorfulDisplay,
    "echolocation": Echolocation,
    "venom_strike": VenomStrike,
    "defend": Defend,
}


def get_ability(ability_name: str) -> Optional[Ability]:
    """
    Get an ability instance by name.
    
    Args:
        ability_name: The name of the ability
        
    Returns:
        An instance of the ability, or None if not found
    """
    ability_class = ABILITY_MAPPING.get(ability_name.lower().replace(" ", "_"))
    if ability_class:
        return ability_class()
    return None