"""
Currencies module for the Dual-Layer Economy System.

This module defines the dual-currency system in the Critter-Craft economy.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, Union, Any


class Currency(ABC):
    """
    Base class for all currencies in the Critter-Craft economy.
    """
    
    def __init__(self, name: str, symbol: str, description: str, is_on_chain: bool):
        """
        Initialize a currency.
        
        Args:
            name: The name of the currency.
            symbol: The symbol of the currency.
            description: The description of the currency.
            is_on_chain: Whether this currency is on-chain.
        """
        self.name = name
        self.symbol = symbol
        self.description = description
        self.is_on_chain = is_on_chain
    
    @abstractmethod
    def format_amount(self, amount: int) -> str:
        """
        Format an amount of this currency as a string.
        
        Args:
            amount: The amount to format.
            
        Returns:
            The formatted amount.
        """
        pass
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "name": self.name,
            "symbol": self.symbol,
            "description": self.description,
            "is_on_chain": self.is_on_chain
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Currency':
        """Create from a dictionary."""
        if data.get("symbol") == "BITS":
            return Bits.from_dict(data)
        elif data.get("symbol") == "AURA":
            return Aura.from_dict(data)
        
        raise ValueError(f"Unknown currency symbol: {data.get('symbol')}")


class Bits(Currency):
    """
    The soft currency used in the Local Economy (off-chain).
    
    This is where 99% of daily transactions occur. It deals with common, fungible items.
    """
    
    def __init__(self):
        """Initialize the Bits currency."""
        super().__init__(
            name="Bits",
            symbol="BITS",
            description="The soft currency used for everyday transactions in Critter-Craft.",
            is_on_chain=False
        )
    
    def format_amount(self, amount: int) -> str:
        """
        Format an amount of Bits as a string.
        
        Args:
            amount: The amount to format.
            
        Returns:
            The formatted amount.
        """
        return f"{amount:,} {self.symbol}"
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Bits':
        """Create from a dictionary."""
        return cls()


class Aura(Currency):
    """
    The hard currency used in the Global Economy (on-chain).
    
    This is for assets of true scarcity and provenance. It is rare and earned
    through high-level gameplay.
    """
    
    def __init__(self):
        """Initialize the Aura currency."""
        super().__init__(
            name="Aura",
            symbol="AURA",
            description="The hard currency used for high-value transactions in Critter-Craft.",
            is_on_chain=True
        )
    
    def format_amount(self, amount: int) -> str:
        """
        Format an amount of Aura as a string.
        
        Args:
            amount: The amount to format in the smallest unit (wei).
            
        Returns:
            The formatted amount.
        """
        # Convert from wei to Aura (1 Aura = 10^18 wei)
        aura_amount = amount / 10**18
        return f"{aura_amount:.6f} {self.symbol}"
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Aura':
        """Create from a dictionary."""
        return cls()