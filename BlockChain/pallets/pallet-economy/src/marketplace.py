"""
Marketplace module for the Dual-Layer Economy System.

This module implements the dual-layer marketplace in the Critter-Craft economy.
"""

import time
import uuid
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, Union, Any

from .items import Item
from .currencies import Currency, Bits, Aura


class OrderType(Enum):
    """Types of orders in the marketplace."""
    BUY = auto()   # Buy order (bid)
    SELL = auto()  # Sell order (ask)


@dataclass
class Order:
    """
    An order in the marketplace.
    
    This can be a buy order (bid) or a sell order (ask).
    """
    id: str
    player_id: str
    item_id: str
    quantity: int
    price_per_unit: int
    currency_symbol: str
    order_type: OrderType
    timestamp: int = field(default_factory=lambda: int(time.time()))
    is_fulfilled: bool = False
    
    def __post_init__(self):
        """Initialize with a UUID if not provided."""
        if not self.id:
            self.id = str(uuid.uuid4())
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "id": self.id,
            "player_id": self.player_id,
            "item_id": self.item_id,
            "quantity": self.quantity,
            "price_per_unit": self.price_per_unit,
            "currency_symbol": self.currency_symbol,
            "order_type": self.order_type.name,
            "timestamp": self.timestamp,
            "is_fulfilled": self.is_fulfilled
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Order':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            player_id=data["player_id"],
            item_id=data["item_id"],
            quantity=data["quantity"],
            price_per_unit=data["price_per_unit"],
            currency_symbol=data["currency_symbol"],
            order_type=OrderType[data["order_type"]],
            timestamp=data["timestamp"],
            is_fulfilled=data["is_fulfilled"]
        )


@dataclass
class Listing:
    """
    A listing in the marketplace.
    
    This represents an item listed for sale.
    """
    id: str
    player_id: str
    item: Item
    quantity: int
    price: int
    currency_symbol: str
    timestamp: int = field(default_factory=lambda: int(time.time()))
    is_sold: bool = False
    
    def __post_init__(self):
        """Initialize with a UUID if not provided."""
        if not self.id:
            self.id = str(uuid.uuid4())
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "id": self.id,
            "player_id": self.player_id,
            "item": self.item.to_dict(),
            "quantity": self.quantity,
            "price": self.price,
            "currency_symbol": self.currency_symbol,
            "timestamp": self.timestamp,
            "is_sold": self.is_sold
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Listing':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            player_id=data["player_id"],
            item=Item.from_dict(data["item"]),
            quantity=data["quantity"],
            price=data["price"],
            currency_symbol=data["currency_symbol"],
            timestamp=data["timestamp"],
            is_sold=data["is_sold"]
        )


@dataclass
class Transaction:
    """
    A transaction in the marketplace.
    
    This represents a completed sale.
    """
    id: str
    buyer_id: str
    seller_id: str
    item_id: str
    quantity: int
    price_per_unit: int
    total_price: int
    currency_symbol: str
    timestamp: int = field(default_factory=lambda: int(time.time()))
    
    def __post_init__(self):
        """Initialize with a UUID if not provided."""
        if not self.id:
            self.id = str(uuid.uuid4())
        if not self.total_price:
            self.total_price = self.price_per_unit * self.quantity
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "id": self.id,
            "buyer_id": self.buyer_id,
            "seller_id": self.seller_id,
            "item_id": self.item_id,
            "quantity": self.quantity,
            "price_per_unit": self.price_per_unit,
            "total_price": self.total_price,
            "currency_symbol": self.currency_symbol,
            "timestamp": self.timestamp
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Transaction':
        """Create from a dictionary."""
        return cls(
            id=data["id"],
            buyer_id=data["buyer_id"],
            seller_id=data["seller_id"],
            item_id=data["item_id"],
            quantity=data["quantity"],
            price_per_unit=data["price_per_unit"],
            total_price=data["total_price"],
            currency_symbol=data["currency_symbol"],
            timestamp=data["timestamp"]
        )


class Marketplace(ABC):
    """
    Base class for all marketplaces in the Critter-Craft economy.
    """
    
    def __init__(self, name: str, description: str, currency: Currency):
        """
        Initialize a marketplace.
        
        Args:
            name: The name of the marketplace.
            description: The description of the marketplace.
            currency: The currency used in this marketplace.
        """
        self.name = name
        self.description = description
        self.currency = currency
        self.listings: Dict[str, Listing] = {}
        self.orders: Dict[str, Order] = {}
        self.transactions: Dict[str, Transaction] = {}
    
    @abstractmethod
    def create_listing(self, player_id: str, item: Item, quantity: int, price: int) -> Optional[Listing]:
        """
        Create a listing in the marketplace.
        
        Args:
            player_id: The ID of the player creating the listing.
            item: The item to list.
            quantity: The quantity to list.
            price: The price per unit.
            
        Returns:
            The created listing, or None if the listing could not be created.
        """
        pass
    
    @abstractmethod
    def create_order(self, player_id: str, item_id: str, quantity: int, price_per_unit: int, order_type: OrderType) -> Optional[Order]:
        """
        Create an order in the marketplace.
        
        Args:
            player_id: The ID of the player creating the order.
            item_id: The ID of the item to buy or sell.
            quantity: The quantity to buy or sell.
            price_per_unit: The price per unit.
            order_type: The type of order (buy or sell).
            
        Returns:
            The created order, or None if the order could not be created.
        """
        pass
    
    @abstractmethod
    def fulfill_order(self, order_id: str, player_id: str) -> Optional[Transaction]:
        """
        Fulfill an order in the marketplace.
        
        Args:
            order_id: The ID of the order to fulfill.
            player_id: The ID of the player fulfilling the order.
            
        Returns:
            The resulting transaction, or None if the order could not be fulfilled.
        """
        pass
    
    @abstractmethod
    def buy_listing(self, listing_id: str, player_id: str) -> Optional[Transaction]:
        """
        Buy a listing in the marketplace.
        
        Args:
            listing_id: The ID of the listing to buy.
            player_id: The ID of the player buying the listing.
            
        Returns:
            The resulting transaction, or None if the listing could not be bought.
        """
        pass
    
    def get_listings(self, item_id: Optional[str] = None, player_id: Optional[str] = None) -> List[Listing]:
        """
        Get listings in the marketplace.
        
        Args:
            item_id: Filter by item ID.
            player_id: Filter by player ID.
            
        Returns:
            A list of listings matching the filters.
        """
        result = []
        
        for listing in self.listings.values():
            if listing.is_sold:
                continue
                
            if item_id and listing.item.id != item_id:
                continue
                
            if player_id and listing.player_id != player_id:
                continue
                
            result.append(listing)
        
        return result
    
    def get_orders(self, item_id: Optional[str] = None, player_id: Optional[str] = None, order_type: Optional[OrderType] = None) -> List[Order]:
        """
        Get orders in the marketplace.
        
        Args:
            item_id: Filter by item ID.
            player_id: Filter by player ID.
            order_type: Filter by order type.
            
        Returns:
            A list of orders matching the filters.
        """
        result = []
        
        for order in self.orders.values():
            if order.is_fulfilled:
                continue
                
            if item_id and order.item_id != item_id:
                continue
                
            if player_id and order.player_id != player_id:
                continue
                
            if order_type and order.order_type != order_type:
                continue
                
            result.append(order)
        
        return result
    
    def get_transactions(self, item_id: Optional[str] = None, buyer_id: Optional[str] = None, seller_id: Optional[str] = None) -> List[Transaction]:
        """
        Get transactions in the marketplace.
        
        Args:
            item_id: Filter by item ID.
            buyer_id: Filter by buyer ID.
            seller_id: Filter by seller ID.
            
        Returns:
            A list of transactions matching the filters.
        """
        result = []
        
        for transaction in self.transactions.values():
            if item_id and transaction.item_id != item_id:
                continue
                
            if buyer_id and transaction.buyer_id != buyer_id:
                continue
                
            if seller_id and transaction.seller_id != seller_id:
                continue
                
            result.append(transaction)
        
        return result
    
    def to_dict(self) -> Dict:
        """Convert to a dictionary for serialization."""
        return {
            "name": self.name,
            "description": self.description,
            "currency": self.currency.to_dict(),
            "listings": {listing_id: listing.to_dict() for listing_id, listing in self.listings.items()},
            "orders": {order_id: order.to_dict() for order_id, order in self.orders.items()},
            "transactions": {transaction_id: transaction.to_dict() for transaction_id, transaction in self.transactions.items()}
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'Marketplace':
        """Create from a dictionary."""
        currency_symbol = data["currency"]["symbol"]
        
        if currency_symbol == "BITS":
            return LocalMarketplace.from_dict(data)
        elif currency_symbol == "AURA":
            return GlobalMarketplace.from_dict(data)
        
        raise ValueError(f"Unknown currency symbol: {currency_symbol}")


class LocalMarketplace(Marketplace):
    """
    The Local Marketplace (off-chain).
    
    This is the bustling, high-volume hub for the everyday player, powered by $BITS.
    """
    
    def __init__(self):
        """Initialize the Local Marketplace."""
        super().__init__(
            name="Local Marketplace",
            description="The bustling, high-volume hub for everyday transactions in Critter-Craft.",
            currency=Bits()
        )
    
    def create_listing(self, player_id: str, item: Item, quantity: int, price: int) -> Optional[Listing]:
        """
        Create a listing in the Local Marketplace.
        
        Args:
            player_id: The ID of the player creating the listing.
            item: The item to list.
            quantity: The quantity to list.
            price: The price per unit.
            
        Returns:
            The created listing, or None if the listing could not be created.
        """
        # Check if the item is tradable
        if not item.is_tradable:
            return None
        
        # Check if the item is on-chain (NFT)
        if hasattr(item, "is_legendary") and item.is_legendary:
            return None
        
        # Create the listing
        listing = Listing(
            id="",
            player_id=player_id,
            item=item,
            quantity=quantity,
            price=price,
            currency_symbol=self.currency.symbol
        )
        
        # Add the listing to the marketplace
        self.listings[listing.id] = listing
        
        # Check for matching buy orders
        matching_orders = self.get_orders(
            item_id=item.id,
            order_type=OrderType.BUY
        )
        
        # Sort by price (highest first) and timestamp (oldest first)
        matching_orders.sort(key=lambda o: (-o.price_per_unit, o.timestamp))
        
        # Try to fulfill matching orders
        remaining_quantity = quantity
        transactions = []
        
        for order in matching_orders:
            if remaining_quantity <= 0:
                break
                
            if order.price_per_unit < price:
                continue
                
            # Calculate the quantity to fulfill
            fulfill_quantity = min(remaining_quantity, order.quantity)
            
            # Create a transaction
            transaction = Transaction(
                id="",
                buyer_id=order.player_id,
                seller_id=player_id,
                item_id=item.id,
                quantity=fulfill_quantity,
                price_per_unit=order.price_per_unit,
                total_price=order.price_per_unit * fulfill_quantity,
                currency_symbol=self.currency.symbol
            )
            
            # Add the transaction to the marketplace
            self.transactions[transaction.id] = transaction
            transactions.append(transaction)
            
            # Update the order
            order.quantity -= fulfill_quantity
            if order.quantity <= 0:
                order.is_fulfilled = True
            
            # Update the remaining quantity
            remaining_quantity -= fulfill_quantity
        
        # Update the listing
        if remaining_quantity <= 0:
            listing.is_sold = True
        else:
            listing.quantity = remaining_quantity
        
        return listing
    
    def create_order(self, player_id: str, item_id: str, quantity: int, price_per_unit: int, order_type: OrderType) -> Optional[Order]:
        """
        Create an order in the Local Marketplace.
        
        Args:
            player_id: The ID of the player creating the order.
            item_id: The ID of the item to buy or sell.
            quantity: The quantity to buy or sell.
            price_per_unit: The price per unit.
            order_type: The type of order (buy or sell).
            
        Returns:
            The created order, or None if the order could not be created.
        """
        # Create the order
        order = Order(
            id="",
            player_id=player_id,
            item_id=item_id,
            quantity=quantity,
            price_per_unit=price_per_unit,
            currency_symbol=self.currency.symbol,
            order_type=order_type
        )
        
        # Add the order to the marketplace
        self.orders[order.id] = order
        
        # If it's a buy order, try to fulfill it with matching sell listings
        if order_type == OrderType.BUY:
            matching_listings = self.get_listings(item_id=item_id)
            
            # Sort by price (lowest first) and timestamp (oldest first)
            matching_listings.sort(key=lambda l: (l.price, l.timestamp))
            
            # Try to fulfill the order with matching listings
            remaining_quantity = quantity
            transactions = []
            
            for listing in matching_listings:
                if remaining_quantity <= 0:
                    break
                    
                if listing.price > price_per_unit:
                    continue
                    
                # Calculate the quantity to fulfill
                fulfill_quantity = min(remaining_quantity, listing.quantity)
                
                # Create a transaction
                transaction = Transaction(
                    id="",
                    buyer_id=player_id,
                    seller_id=listing.player_id,
                    item_id=item_id,
                    quantity=fulfill_quantity,
                    price_per_unit=listing.price,
                    total_price=listing.price * fulfill_quantity,
                    currency_symbol=self.currency.symbol
                )
                
                # Add the transaction to the marketplace
                self.transactions[transaction.id] = transaction
                transactions.append(transaction)
                
                # Update the listing
                listing.quantity -= fulfill_quantity
                if listing.quantity <= 0:
                    listing.is_sold = True
                
                # Update the remaining quantity
                remaining_quantity -= fulfill_quantity
            
            # Update the order
            if remaining_quantity <= 0:
                order.is_fulfilled = True
            else:
                order.quantity = remaining_quantity
        
        # If it's a sell order, try to fulfill it with matching buy orders
        elif order_type == OrderType.SELL:
            matching_orders = self.get_orders(
                item_id=item_id,
                order_type=OrderType.BUY
            )
            
            # Sort by price (highest first) and timestamp (oldest first)
            matching_orders.sort(key=lambda o: (-o.price_per_unit, o.timestamp))
            
            # Try to fulfill the order with matching buy orders
            remaining_quantity = quantity
            transactions = []
            
            for matching_order in matching_orders:
                if remaining_quantity <= 0:
                    break
                    
                if matching_order.price_per_unit < price_per_unit:
                    continue
                    
                # Calculate the quantity to fulfill
                fulfill_quantity = min(remaining_quantity, matching_order.quantity)
                
                # Create a transaction
                transaction = Transaction(
                    id="",
                    buyer_id=matching_order.player_id,
                    seller_id=player_id,
                    item_id=item_id,
                    quantity=fulfill_quantity,
                    price_per_unit=matching_order.price_per_unit,
                    total_price=matching_order.price_per_unit * fulfill_quantity,
                    currency_symbol=self.currency.symbol
                )
                
                # Add the transaction to the marketplace
                self.transactions[transaction.id] = transaction
                transactions.append(transaction)
                
                # Update the matching order
                matching_order.quantity -= fulfill_quantity
                if matching_order.quantity <= 0:
                    matching_order.is_fulfilled = True
                
                # Update the remaining quantity
                remaining_quantity -= fulfill_quantity
            
            # Update the order
            if remaining_quantity <= 0:
                order.is_fulfilled = True
            else:
                order.quantity = remaining_quantity
        
        return order
    
    def fulfill_order(self, order_id: str, player_id: str) -> Optional[Transaction]:
        """
        Fulfill an order in the Local Marketplace.
        
        Args:
            order_id: The ID of the order to fulfill.
            player_id: The ID of the player fulfilling the order.
            
        Returns:
            The resulting transaction, or None if the order could not be fulfilled.
        """
        # Get the order
        order = self.orders.get(order_id)
        
        if not order or order.is_fulfilled:
            return None
        
        # Create a transaction
        if order.order_type == OrderType.BUY:
            # The player is selling to the order
            transaction = Transaction(
                id="",
                buyer_id=order.player_id,
                seller_id=player_id,
                item_id=order.item_id,
                quantity=order.quantity,
                price_per_unit=order.price_per_unit,
                total_price=order.price_per_unit * order.quantity,
                currency_symbol=self.currency.symbol
            )
        else:
            # The player is buying from the order
            transaction = Transaction(
                id="",
                buyer_id=player_id,
                seller_id=order.player_id,
                item_id=order.item_id,
                quantity=order.quantity,
                price_per_unit=order.price_per_unit,
                total_price=order.price_per_unit * order.quantity,
                currency_symbol=self.currency.symbol
            )
        
        # Add the transaction to the marketplace
        self.transactions[transaction.id] = transaction
        
        # Mark the order as fulfilled
        order.is_fulfilled = True
        
        return transaction
    
    def buy_listing(self, listing_id: str, player_id: str) -> Optional[Transaction]:
        """
        Buy a listing in the Local Marketplace.
        
        Args:
            listing_id: The ID of the listing to buy.
            player_id: The ID of the player buying the listing.
            
        Returns:
            The resulting transaction, or None if the listing could not be bought.
        """
        # Get the listing
        listing = self.listings.get(listing_id)
        
        if not listing or listing.is_sold:
            return None
        
        # Create a transaction
        transaction = Transaction(
            id="",
            buyer_id=player_id,
            seller_id=listing.player_id,
            item_id=listing.item.id,
            quantity=listing.quantity,
            price_per_unit=listing.price,
            total_price=listing.price * listing.quantity,
            currency_symbol=self.currency.symbol
        )
        
        # Add the transaction to the marketplace
        self.transactions[transaction.id] = transaction
        
        # Mark the listing as sold
        listing.is_sold = True
        
        return transaction
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'LocalMarketplace':
        """Create from a dictionary."""
        marketplace = cls()
        
        # Add listings
        for listing_id, listing_data in data.get("listings", {}).items():
            marketplace.listings[listing_id] = Listing.from_dict(listing_data)
        
        # Add orders
        for order_id, order_data in data.get("orders", {}).items():
            marketplace.orders[order_id] = Order.from_dict(order_data)
        
        # Add transactions
        for transaction_id, transaction_data in data.get("transactions", {}).items():
            marketplace.transactions[transaction_id] = Transaction.from_dict(transaction_data)
        
        return marketplace


class GlobalMarketplace(Marketplace):
    """
    The Global Marketplace (on-chain).
    
    This is the prestigious, transparent exchange for high-value assets, powered by $AURA.
    """
    
    def __init__(self, ledger=None):
        """
        Initialize the Global Marketplace.
        
        Args:
            ledger: The Zoologist's Ledger instance.
        """
        super().__init__(
            name="Global Marketplace",
            description="The prestigious, transparent exchange for high-value assets in Critter-Craft.",
            currency=Aura()
        )
        self.ledger = ledger
    
    def create_listing(self, player_id: str, item: Item, quantity: int, price: int) -> Optional[Listing]:
        """
        Create a listing in the Global Marketplace.
        
        Args:
            player_id: The ID of the player creating the listing.
            item: The item to list.
            quantity: The quantity to list.
            price: The price per unit.
            
        Returns:
            The created listing, or None if the listing could not be created.
        """
        # Check if the item is on-chain (NFT)
        if not hasattr(item, "is_legendary") or not item.is_legendary:
            return None
        
        # Check if the player owns the item
        if self.ledger:
            # In a real implementation, this would check ownership on the blockchain
            pass
        
        # Create the listing
        listing = Listing(
            id="",
            player_id=player_id,
            item=item,
            quantity=quantity,
            price=price,
            currency_symbol=self.currency.symbol
        )
        
        # Add the listing to the marketplace
        self.listings[listing.id] = listing
        
        # In a real implementation, this would create a listing on the blockchain
        if self.ledger:
            pass
        
        return listing
    
    def create_order(self, player_id: str, item_id: str, quantity: int, price_per_unit: int, order_type: OrderType) -> Optional[Order]:
        """
        Create an order in the Global Marketplace.
        
        Args:
            player_id: The ID of the player creating the order.
            item_id: The ID of the item to buy or sell.
            quantity: The quantity to buy or sell.
            price_per_unit: The price per unit.
            order_type: The type of order (buy or sell).
            
        Returns:
            The created order, or None if the order could not be created.
        """
        # Create the order
        order = Order(
            id="",
            player_id=player_id,
            item_id=item_id,
            quantity=quantity,
            price_per_unit=price_per_unit,
            currency_symbol=self.currency.symbol,
            order_type=order_type
        )
        
        # Add the order to the marketplace
        self.orders[order.id] = order
        
        # In a real implementation, this would create an order on the blockchain
        if self.ledger:
            pass
        
        return order
    
    def fulfill_order(self, order_id: str, player_id: str) -> Optional[Transaction]:
        """
        Fulfill an order in the Global Marketplace.
        
        Args:
            order_id: The ID of the order to fulfill.
            player_id: The ID of the player fulfilling the order.
            
        Returns:
            The resulting transaction, or None if the order could not be fulfilled.
        """
        # Get the order
        order = self.orders.get(order_id)
        
        if not order or order.is_fulfilled:
            return None
        
        # In a real implementation, this would check ownership and balances on the blockchain
        if self.ledger:
            pass
        
        # Create a transaction
        if order.order_type == OrderType.BUY:
            # The player is selling to the order
            transaction = Transaction(
                id="",
                buyer_id=order.player_id,
                seller_id=player_id,
                item_id=order.item_id,
                quantity=order.quantity,
                price_per_unit=order.price_per_unit,
                total_price=order.price_per_unit * order.quantity,
                currency_symbol=self.currency.symbol
            )
        else:
            # The player is buying from the order
            transaction = Transaction(
                id="",
                buyer_id=player_id,
                seller_id=order.player_id,
                item_id=order.item_id,
                quantity=order.quantity,
                price_per_unit=order.price_per_unit,
                total_price=order.price_per_unit * order.quantity,
                currency_symbol=self.currency.symbol
            )
        
        # Add the transaction to the marketplace
        self.transactions[transaction.id] = transaction
        
        # Mark the order as fulfilled
        order.is_fulfilled = True
        
        # In a real implementation, this would execute the transaction on the blockchain
        if self.ledger:
            pass
        
        return transaction
    
    def buy_listing(self, listing_id: str, player_id: str) -> Optional[Transaction]:
        """
        Buy a listing in the Global Marketplace.
        
        Args:
            listing_id: The ID of the listing to buy.
            player_id: The ID of the player buying the listing.
            
        Returns:
            The resulting transaction, or None if the listing could not be bought.
        """
        # Get the listing
        listing = self.listings.get(listing_id)
        
        if not listing or listing.is_sold:
            return None
        
        # In a real implementation, this would check balances on the blockchain
        if self.ledger:
            pass
        
        # Create a transaction
        transaction = Transaction(
            id="",
            buyer_id=player_id,
            seller_id=listing.player_id,
            item_id=listing.item.id,
            quantity=listing.quantity,
            price_per_unit=listing.price,
            total_price=listing.price * listing.quantity,
            currency_symbol=self.currency.symbol
        )
        
        # Add the transaction to the marketplace
        self.transactions[transaction.id] = transaction
        
        # Mark the listing as sold
        listing.is_sold = True
        
        # In a real implementation, this would execute the transaction on the blockchain
        if self.ledger:
            pass
        
        return transaction
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'GlobalMarketplace':
        """Create from a dictionary."""
        marketplace = cls()
        
        # Add listings
        for listing_id, listing_data in data.get("listings", {}).items():
            marketplace.listings[listing_id] = Listing.from_dict(listing_data)
        
        # Add orders
        for order_id, order_data in data.get("orders", {}).items():
            marketplace.orders[order_id] = Order.from_dict(order_data)
        
        # Add transactions
        for transaction_id, transaction_data in data.get("transactions", {}).items():
            marketplace.transactions[transaction_id] = Transaction.from_dict(transaction_data)
        
        return marketplace