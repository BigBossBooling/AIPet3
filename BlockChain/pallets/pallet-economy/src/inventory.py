"""
Inventory module for the Dual-Layer Economy System.

This module implements the player inventory in the Critter-Craft economy.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Set, Tuple, Union, Any

from items import Item, ItemType # Changed from relative to direct import


@dataclass
class InventorySlot:
    """
    A slot in the player's inventory.
    
    This represents a stack of items.
    """
    item: Item
    quantity: int
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "item": self.item.to_dict(),
            "quantity": self.quantity
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'InventorySlot':
        """Create from a dictionary."""
        return cls(
            item=Item.from_dict(data["item"]),
            quantity=data["quantity"]
        )


class Inventory:
    """
    The player's inventory.
    
    This stores all the items the player owns.
    """
    
    def __init__(self, player_id: str, max_slots: int = 100):
        """
        Initialize the inventory.
        
        Args:
            player_id: The ID of the player who owns this inventory.
            max_slots: The maximum number of slots in the inventory.
        """
        self.player_id = player_id
        self.max_slots = max_slots
        self.slots: Dict[str, InventorySlot] = {}  # item_id -> slot
    
    def add_item(self, item: Item, quantity: int = 1) -> bool:
        """
        Add an item to the inventory.
        
        Args:
            item: The item to add.
            quantity: The quantity to add.
            
        Returns:
            True if the item was added successfully, False otherwise.
        """
        # Check if the item already exists in the inventory
        if item.id in self.slots:
            # Check if the item can stack
            if item.stack_size > 1:
                # Calculate the new quantity
                new_quantity = self.slots[item.id].quantity + quantity
                
                # Check if the new quantity exceeds the stack size
                if new_quantity > item.stack_size:
                    # Calculate the overflow
                    overflow = new_quantity - item.stack_size
                    
                    # Update the existing slot to the max stack size
                    self.slots[item.id].quantity = item.stack_size
                    
                    # Try to add the overflow as a new stack
                    return self.add_item(item, overflow)
                else:
                    # Update the existing slot
                    self.slots[item.id].quantity = new_quantity
                    return True
            else:
                # Item cannot stack, so we need to add it as a new slot
                # Check if we have enough slots
                if len(self.slots) >= self.max_slots:
                    return False
                
                # Add the item as a new slot
                self.slots[f"{item.id}_{len(self.slots)}"] = InventorySlot(
                    item=item,
                    quantity=quantity
                )
                return True
        else:
            # Check if we have enough slots
            if len(self.slots) >= self.max_slots:
                return False
            
            # Add the item as a new slot
            self.slots[item.id] = InventorySlot(
                item=item,
                quantity=quantity
            )
            return True
    
    def remove_item(self, item_id: str, quantity: int = 1) -> bool:
        """
        Remove an item from the inventory.
        
        Args:
            item_id: The ID of the item to remove.
            quantity: The quantity to remove.
            
        Returns:
            True if the item was removed successfully, False otherwise.
        """
        # Check if the item exists in the inventory
        if item_id not in self.slots:
            return False
        
        # Check if we have enough of the item
        if self.slots[item_id].quantity < quantity:
            return False
        
        # Update the quantity
        self.slots[item_id].quantity -= quantity
        
        # Remove the slot if the quantity is 0
        if self.slots[item_id].quantity <= 0:
            del self.slots[item_id]
        
        return True
    
    def get_item(self, item_id: str) -> Optional[InventorySlot]:
        """
        Get an item from the inventory.
        
        Args:
            item_id: The ID of the item to get.
            
        Returns:
            The inventory slot containing the item, or None if the item is not in the inventory.
        """
        return self.slots.get(item_id)
    
    def get_items_by_type(self, item_type: ItemType) -> List[InventorySlot]:
        """
        Get all items of a specific type from the inventory.
        
        Args:
            item_type: The type of items to get.
            
        Returns:
            A list of inventory slots containing items of the specified type.
        """
        return [
            slot for slot in self.slots.values()
            if slot.item.item_type == item_type
        ]
    
    def get_total_quantity(self, item_id: str) -> int:
        """
        Get the total quantity of an item in the inventory.
        
        Args:
            item_id: The ID of the item to get the quantity of.
            
        Returns:
            The total quantity of the item in the inventory.
        """
        # Check if the item exists in the inventory
        if item_id not in self.slots:
            return 0
        
        return self.slots[item_id].quantity
    
    def has_item(self, item_id: str, quantity: int = 1) -> bool:
        """
        Check if the inventory has a specific quantity of an item.
        
        Args:
            item_id: The ID of the item to check.
            quantity: The quantity to check for.
            
        Returns:
            True if the inventory has at least the specified quantity of the item, False otherwise.
        """
        return self.get_total_quantity(item_id) >= quantity
    
    def get_free_slots(self) -> int:
        """
        Get the number of free slots in the inventory.
        
        Returns:
            The number of free slots in the inventory.
        """
        return self.max_slots - len(self.slots)
    
    def is_full(self) -> bool:
        """
        Check if the inventory is full.
        
        Returns:
            True if the inventory is full, False otherwise.
        """
        return len(self.slots) >= self.max_slots
    
    def clear(self) -> None:
        """Clear the inventory."""
        self.slots.clear()
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "player_id": self.player_id,
            "max_slots": self.max_slots,
            "slots": {slot_id: slot.to_dict() for slot_id, slot in self.slots.items()}
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Inventory':
        """Create from a dictionary."""
        inventory = cls(
            player_id=data["player_id"],
            max_slots=data["max_slots"]
        )
        
        for slot_id, slot_data in data["slots"].items():
            inventory.slots[slot_id] = InventorySlot.from_dict(slot_data)
        
        return inventory