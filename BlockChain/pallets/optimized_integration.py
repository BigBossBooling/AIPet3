"""
Optimized integration module for the Critter-Craft systems.

This module provides a unified interface for integrating all Critter-Craft systems
with the blockchain, including the battle system, breeding system, economy system,
and activities system.
"""

import sys
import os
import time
import random
import logging
import asyncio
from typing import Dict, List, Optional, Tuple, Any, Union, TypeVar
from pathlib import Path
from functools import lru_cache
from concurrent.futures import ThreadPoolExecutor

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler("integration.log"),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger("crittercraft.integration")

# Add the necessary directories to the Python path using pathlib for better path handling
PALLETS_DIR = Path(__file__).parent
sys.path.insert(0, str(PALLETS_DIR / 'pallet-battles' / 'src'))
sys.path.insert(0, str(PALLETS_DIR / 'pallet-ledger' / 'src'))
sys.path.insert(0, str(PALLETS_DIR / 'pallet-breeding' / 'src'))
sys.path.insert(0, str(PALLETS_DIR / 'pallet-economy' / 'src'))
sys.path.insert(0, str(PALLETS_DIR / 'pallet-quests' / 'src'))

# Import from battle system
from battle import start_battle, BattleResult, Environment

# Import from blockchain system
from ledger import ZoologistLedger
from wallet import Wallet
from models import TransactionType, ZoologistIdentity, ZoologistLevel

# Import from breeding system
from genetics import (
    GeneticCode, 
    CoreGenes, 
    PotentialGenes, 
    CosmeticGenes, 
    AuraType,
    Stat
)
from synthesis import (
    EchoSynthesizer, 
    SynthesisType, 
    SynthesisState,
    SynthesisResult
)
from catalysts import (
    StableCatalyst, 
    UnstableCatalyst,
    Catalyst
)
from lineage import (
    FamilyTree, 
    calculate_inbreeding_coefficient
)

# Import from economy system
from items import (
    ItemType,
    ItemRarity,
    Item,
    Material,
    Consumable,
    Gear,
    Blueprint,
    QuestItem,
    BridgingItem,
    BreedingCatalyst,
    GeneSplicer,
    NFTMintingKit
)
from currencies import Bits, Aura
from marketplace import (
    LocalMarketplace,
    GlobalMarketplace,
    OrderType
)
from inventory import Inventory
from crafting import (
    CraftingSystem,
    Recipe,
    CraftingResult
)

# Import from activities system
from activities import ActivityType, StatType
from activities_system import (
    ActivityManager,
    Activity,
    MiniGame,
    Job,
    Quest,
    AdventurousQuest,
    ActivityReward
)

# Type variables for better type hinting
T = TypeVar('T')


class BlockchainIntegration:
    """
    A unified interface for integrating all Critter-Craft systems with the blockchain.
    
    This class provides methods for interacting with the blockchain and integrating
    the various systems in Critter-Craft, including the battle system, breeding system,
    economy system, and activities system.
    
    Attributes:
        ledger: The Zoologist's Ledger instance.
        activity_manager: The activity manager.
        crafting_system: The crafting system.
        marketplace: The marketplace.
        thread_pool: A thread pool for executing blockchain operations asynchronously.
    """
    
    def __init__(
        self,
        ledger: ZoologistLedger,
        activity_manager: Optional[ActivityManager] = None,
        crafting_system: Optional[CraftingSystem] = None,
        marketplace: Optional[Union[LocalMarketplace, GlobalMarketplace]] = None
    ):
        """
        Initialize the blockchain integration.
        
        Args:
            ledger: The Zoologist's Ledger instance.
            activity_manager: The activity manager.
            crafting_system: The crafting system.
            marketplace: The marketplace.
        """
        self.ledger = ledger
        self.activity_manager = activity_manager or ActivityManager()
        self.crafting_system = crafting_system or CraftingSystem()
        self.marketplace = marketplace or GlobalMarketplace()
        self.thread_pool = ThreadPoolExecutor(max_workers=4)
        
        logger.info("BlockchainIntegration initialized")
    
    # ===== Battle System Integration =====
    
    async def battle_with_blockchain(
        self,
        player_wallet: Wallet,
        opponent_wallet: Wallet,
        player_pet: Dict[str, Any],
        opponent_pet: Dict[str, Any],
        environment_type: str,
        player_items: List[Item]
    ) -> Dict[str, Any]:
        """
        Run a battle and record significant events on the blockchain.
        
        This method is asynchronous to allow for non-blocking blockchain operations.
        
        Args:
            player_wallet: The wallet of the player.
            opponent_wallet: The wallet of the opponent.
            player_pet: The player's pet data.
            opponent_pet: The opponent's pet data.
            environment_type: The type of environment for the battle.
            player_items: The items the player has available.
            
        Returns:
            The result of the battle.
        """
        logger.info(f"Starting battle between {player_pet.get('name')} and {opponent_pet.get('name')}")
        
        # Convert items to the format expected by the battle system
        battle_items = self._convert_items_to_battle_format(player_items)
        
        # Start the battle (this is synchronous but could be made async in the future)
        try:
            battle_result = await asyncio.to_thread(
                start_battle,
                player_pet,
                opponent_pet,
                environment_type,
                battle_items
            )
            
            # Record significant events on the blockchain asynchronously
            if battle_result["winner"] == "player":
                asyncio.create_task(self._record_battle_victory(
                    player_wallet=player_wallet,
                    opponent_wallet=opponent_wallet,
                    player_pet=player_pet,
                    opponent_pet=opponent_pet,
                    battle_result=battle_result
                ))
            
            return battle_result
        
        except Exception as e:
            logger.error(f"Error during battle: {e}", exc_info=True)
            return {"winner": "opponent", "error": str(e)}
    
    async def _record_battle_victory(
        self,
        player_wallet: Wallet,
        opponent_wallet: Wallet,
        player_pet: Dict[str, Any],
        opponent_pet: Dict[str, Any],
        battle_result: Dict[str, Any]
    ) -> bool:
        """
        Record a significant battle victory on the Zoologist's Ledger.
        
        Args:
            player_wallet: The wallet of the winning player.
            opponent_wallet: The wallet of the losing player.
            player_pet: The winning pet's data.
            opponent_pet: The losing pet's data.
            battle_result: The result of the battle.
            
        Returns:
            True if the victory was recorded successfully, False otherwise.
        """
        logger.info(f"Recording battle victory for {player_wallet.address}")
        
        # Only record significant victories (e.g., against Alpha critters)
        if not opponent_pet.get("is_alpha", False):
            return False
        
        try:
            # Update the winner's reputation
            reputation_tx = player_wallet.create_reputation_update_transaction(
                target_did=player_wallet.address,
                change_amount=10,  # Significant reputation boost for defeating an Alpha
                reason_code="ALPHA_VICTORY"
            )
            
            # Submit the transaction
            if not await asyncio.to_thread(self.ledger.submit_transaction, reputation_tx):
                logger.error(f"Failed to submit reputation transaction for {player_wallet.address}")
                return False
            
            # If the winner's pet has an NFT ID, record its evolution
            if "nft_id" in player_pet:
                evolve_tx = player_wallet.create_pet_evolve_transaction(
                    pet_id=player_pet["nft_id"],
                    new_form_id=f"evolved_{player_pet['species'].lower()}_1"
                )
                
                # Submit the transaction
                if not await asyncio.to_thread(self.ledger.submit_transaction, evolve_tx):
                    logger.error(f"Failed to submit pet evolution transaction for {player_pet['nft_id']}")
                    return False
            
            # Create a block to confirm the transactions
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            return True
        
        except Exception as e:
            logger.error(f"Error recording battle victory: {e}", exc_info=True)
            return False
    
    def _convert_items_to_battle_format(self, items: List[Item]) -> List[Dict[str, Any]]:
        """
        Convert items to the format expected by the battle system.
        
        Args:
            items: The items to convert.
            
        Returns:
            The items in battle system format.
        """
        battle_items = []
        
        for item in items:
            if item.item_type == ItemType.CONSUMABLE:
                battle_items.append({
                    "id": item.id,
                    "name": item.name,
                    "effect_type": getattr(item, "effect_type", "healing"),
                    "effect_value": getattr(item, "effect_value", 10),
                    "duration": getattr(item, "duration", 0)
                })
        
        return battle_items
    
    # ===== Breeding System Integration =====
    
    async def perform_breeding(
        self,
        player_wallet: Wallet,
        parent_a_id: str,
        parent_b_id: str,
        synthesis_type: SynthesisType,
        catalyst: Optional[Catalyst] = None,
        gene_splicers: Optional[List[GeneSplicer]] = None
    ) -> Optional[str]:
        """
        Perform breeding between two pets and record the result on the blockchain.
        
        Args:
            player_wallet: The wallet of the player performing the breeding.
            parent_a_id: The ID of the first parent pet.
            parent_b_id: The ID of the second parent pet.
            synthesis_type: The type of synthesis to perform.
            catalyst: The catalyst to use.
            gene_splicers: List of gene splicers to use.
            
        Returns:
            The ID of the offspring pet, or None if breeding failed.
        """
        logger.info(f"Performing breeding between pets {parent_a_id} and {parent_b_id}")
        
        try:
            # Get the parent pets from the blockchain
            parent_a_nft = await asyncio.to_thread(self.ledger.get_pet, parent_a_id)
            parent_b_nft = await asyncio.to_thread(self.ledger.get_pet, parent_b_id)
            
            if not parent_a_nft or not parent_b_nft:
                logger.error("One or both parent pets not found")
                return None
            
            # Fetch the off-chain metadata (in a real implementation)
            # For this prototype, we'll generate random metadata
            parent_a_metadata = self._generate_random_pet_metadata()
            parent_b_metadata = self._generate_random_pet_metadata()
            
            # Convert the pets to genetic codes
            parent_a = self._convert_blockchain_pet_to_genetic_code(parent_a_nft, parent_a_metadata)
            parent_b = self._convert_blockchain_pet_to_genetic_code(parent_b_nft, parent_b_metadata)
            
            # Create a family tree
            family_tree = FamilyTree()
            family_tree.add_pet(parent_a)
            family_tree.add_pet(parent_b)
            
            # Check for inbreeding
            inbreeding_coefficient = calculate_inbreeding_coefficient(family_tree, parent_a, parent_b)
            
            if inbreeding_coefficient > 0.25:
                logger.warning(f"High inbreeding coefficient: {inbreeding_coefficient:.2f}")
            
            # Create a default catalyst if none provided
            if not catalyst:
                if synthesis_type == SynthesisType.INTRA_SPECIES:
                    catalyst = StableCatalyst(quality=random.randint(1, 5))
                else:
                    catalyst = UnstableCatalyst(quality=random.randint(1, 5))
            
            # Get the player's zoologist level
            zoologist = await asyncio.to_thread(self.ledger.get_zoologist, player_wallet.address)
            zoologist_level = self._get_zoologist_level(zoologist)
            
            # Create the Echo-Synthesizer
            synthesizer = EchoSynthesizer()
            
            # Set parent happiness (in a real game, this would be the actual happiness values)
            parent_a_happiness = 80
            parent_b_happiness = 90
            
            # Perform the synthesis
            result = await asyncio.to_thread(
                synthesizer.synthesize,
                parent_a=parent_a,
                parent_b=parent_b,
                parent_a_happiness=parent_a_happiness,
                parent_b_happiness=parent_b_happiness,
                synthesis_type=synthesis_type,
                zoologist_level=zoologist_level,
                catalysts=[catalyst],
                gene_splicers=gene_splicers or []
            )
            
            # Check the result
            if result.state != SynthesisState.COMPLETED or not result.offspring:
                logger.error(f"Breeding failed: {result.error_message}")
                return None
            
            # Apply inbreeding penalty if necessary
            offspring = result.offspring
            if inbreeding_coefficient > 0.25:
                offspring = self._apply_inbreeding_penalty(offspring, inbreeding_coefficient)
            
            # Record the breeding on the blockchain
            logger.info("Recording breeding on the Zoologist's Ledger")
            
            # Create a transaction to mint the offspring as a pet NFT
            pet_tx = player_wallet.create_pet_mint_transaction(
                species=offspring.core.species,
                aura_color=offspring.core.aura.name,
                genetic_hash=offspring.calculate_genetic_hash(),
                metadata_uri=f"https://api.crittercraft.com/pets/{offspring.calculate_genetic_hash()}"
            )
            
            if not await asyncio.to_thread(self.ledger.submit_transaction, pet_tx):
                logger.error("Failed to record breeding on the blockchain")
                return None
            
            # Create a block to confirm the transaction
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            # Get the minted pet
            player_pets = await asyncio.to_thread(self.ledger.get_pets_by_owner, player_wallet.address)
            
            if not player_pets:
                logger.error("Failed to retrieve the offspring from the blockchain")
                return None
            
            # Return the ID of the most recently minted pet
            return player_pets[-1].token_id
        
        except Exception as e:
            logger.error(f"Error performing breeding: {e}", exc_info=True)
            return None
    
    def _generate_random_pet_metadata(self) -> Dict[str, Any]:
        """
        Generate random metadata for a pet.
        
        Returns:
            Random pet metadata.
        """
        return {
            "stat_potential": {stat.name: random.randint(50, 80) for stat in Stat},
            "adaptation_slots": random.randint(3, 5),
            "size": random.choice(["TINY", "SMALL", "STANDARD", "LARGE", "HUGE"]),
            "pattern": random.choice(["SOLID", "SPOTTED", "STRIPED", "MOTTLED", "IRIDESCENT"]),
            "marking_color": f"#{random.randint(0, 0xFFFFFF):06x}",
            "glow_intensity": random.uniform(0.0, 1.0)
        }
    
    def _convert_blockchain_pet_to_genetic_code(self, pet_nft, metadata: Dict[str, Any]) -> GeneticCode:
        """
        Convert a pet NFT from the blockchain to a genetic code.
        
        Args:
            pet_nft: The pet NFT from the blockchain.
            metadata: The off-chain metadata for the pet.
            
        Returns:
            The pet's genetic code.
        """
        # Create core genes
        core = CoreGenes(
            species=pet_nft.species,
            aura=AuraType[pet_nft.aura_color],
            genesis_id=pet_nft.token_id,
            lineage=pet_nft.evolution_history
        )
        
        # Create potential genes
        potential = PotentialGenes(
            stat_potential=metadata.get("stat_potential", {}),
            adaptation_slots=metadata.get("adaptation_slots", 3)
        )
        
        # Create cosmetic genes
        cosmetic = CosmeticGenes(
            size=metadata.get("size", "STANDARD"),
            pattern=metadata.get("pattern", "SOLID"),
            marking_color=metadata.get("marking_color", "#FFFFFF"),
            glow_intensity=metadata.get("glow_intensity", 0.0)
        )
        
        return GeneticCode(
            core=core,
            potential=potential,
            cosmetic=cosmetic
        )
    
    def _get_zoologist_level(self, zoologist) -> int:
        """
        Get the numeric level of a zoologist.
        
        Args:
            zoologist: The zoologist object.
            
        Returns:
            The numeric level of the zoologist.
        """
        if not zoologist:
            return 1
        
        return {
            ZoologistLevel.NOVICE: 1,
            ZoologistLevel.APPRENTICE: 2,
            ZoologistLevel.JOURNEYMAN: 3,
            ZoologistLevel.EXPERT: 4,
            ZoologistLevel.MASTER: 5,
            ZoologistLevel.GRANDMASTER: 6
        }.get(zoologist.level, 1)
    
    def _apply_inbreeding_penalty(self, offspring: GeneticCode, inbreeding_coefficient: float) -> GeneticCode:
        """
        Apply an inbreeding penalty to an offspring.
        
        Args:
            offspring: The offspring to apply the penalty to.
            inbreeding_coefficient: The inbreeding coefficient.
            
        Returns:
            The offspring with the penalty applied.
        """
        # Apply a negative mutation to a random stat
        stat = random.choice(list(Stat))
        current_potential = offspring.potential.stat_potential.get(stat, 50)
        penalty = int(inbreeding_coefficient * 20)  # Higher coefficient = higher penalty
        offspring.potential.stat_potential[stat] = max(1, current_potential - penalty)
        
        logger.info(f"Inbreeding penalty applied: {stat.name} potential reduced by {penalty}")
        
        return offspring
    
    # ===== Economy System Integration =====
    
    async def craft_item(
        self,
        player_wallet: Wallet,
        inventory: Inventory,
        recipe_id: str
    ) -> Optional[Item]:
        """
        Craft an item using the crafting system.
        
        Args:
            player_wallet: The wallet of the player crafting the item.
            inventory: The player's inventory.
            recipe_id: The ID of the recipe to use.
            
        Returns:
            The crafted item, or None if crafting failed.
        """
        logger.info(f"Crafting item with recipe {recipe_id}")
        
        try:
            # Get the player's zoologist level
            zoologist = await asyncio.to_thread(self.ledger.get_zoologist, player_wallet.address)
            zoologist_level = self._get_zoologist_level(zoologist)
            
            # Check if the player knows the recipe
            if not self.crafting_system.knows_recipe(player_wallet.address, recipe_id):
                logger.error(f"Player does not know the {recipe_id} recipe")
                return None
            
            # Craft the item
            result, item = await asyncio.to_thread(
                self.crafting_system.craft_item,
                player_id=player_wallet.address,
                recipe_id=recipe_id,
                inventory=inventory,
                player_level=zoologist_level
            )
            
            if result != CraftingResult.SUCCESS or not item:
                logger.error(f"Failed to craft item: {result.name}")
                return None
            
            # Record the crafting on the blockchain if it's a significant item
            if item.rarity in [ItemRarity.RARE, ItemRarity.EPIC, ItemRarity.LEGENDARY]:
                await self._record_significant_crafting(player_wallet, item)
            
            return item
        
        except Exception as e:
            logger.error(f"Error crafting item: {e}", exc_info=True)
            return None
    
    async def _record_significant_crafting(self, player_wallet: Wallet, item: Item) -> bool:
        """
        Record a significant crafting event on the blockchain.
        
        Args:
            player_wallet: The wallet of the player who crafted the item.
            item: The crafted item.
            
        Returns:
            True if the crafting was recorded successfully, False otherwise.
        """
        logger.info(f"Recording significant crafting of {item.name}")
        
        try:
            # Create a transaction to record the crafting
            craft_tx = player_wallet.create_item_craft_transaction(
                item_id=item.id,
                item_type=item.item_type.name,
                rarity=item.rarity.name,
                metadata_uri=f"https://api.crittercraft.com/items/{item.id}"
            )
            
            # Submit the transaction
            if not await asyncio.to_thread(self.ledger.submit_transaction, craft_tx):
                logger.error(f"Failed to record crafting of {item.name}")
                return False
            
            # Create a block to confirm the transaction
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            return True
        
        except Exception as e:
            logger.error(f"Error recording significant crafting: {e}", exc_info=True)
            return False
    
    async def list_item_on_marketplace(
        self,
        player_wallet: Wallet,
        item_id: str,
        price: int,
        currency_type: str = "BITS"
    ) -> bool:
        """
        List an item on the marketplace.
        
        Args:
            player_wallet: The wallet of the player listing the item.
            item_id: The ID of the item to list.
            price: The price of the item.
            currency_type: The type of currency to use.
            
        Returns:
            True if the item was listed successfully, False otherwise.
        """
        logger.info(f"Listing item {item_id} on marketplace for {price} {currency_type}")
        
        try:
            # Create a transaction to list the item
            list_tx = player_wallet.create_marketplace_list_transaction(
                item_id=item_id,
                price=price,
                currency_type=currency_type
            )
            
            # Submit the transaction
            if not await asyncio.to_thread(self.ledger.submit_transaction, list_tx):
                logger.error(f"Failed to list item {item_id}")
                return False
            
            # Create a block to confirm the transaction
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            return True
        
        except Exception as e:
            logger.error(f"Error listing item on marketplace: {e}", exc_info=True)
            return False
    
    # ===== Activities System Integration =====
    
    async def complete_activity(
        self,
        player_wallet: Wallet,
        activity_id: str,
        score: int
    ) -> Optional[ActivityReward]:
        """
        Complete an activity and record the completion on the blockchain.
        
        Args:
            player_wallet: The wallet of the player completing the activity.
            activity_id: The ID of the activity to complete.
            score: The player's score in the activity.
            
        Returns:
            The rewards for completing the activity, or None if completion failed.
        """
        logger.info(f"Completing activity {activity_id} with score {score}")
        
        try:
            # Get the player's zoologist level
            zoologist = await asyncio.to_thread(self.ledger.get_zoologist, player_wallet.address)
            player_level = self._get_zoologist_level(zoologist)
            
            # Complete the activity
            rewards = await asyncio.to_thread(
                self.activity_manager.complete_activity,
                player_id=player_wallet.address,
                activity_id=activity_id,
                score=score,
                player_level=player_level
            )
            
            # Record the completion on the blockchain
            await self._record_activity_completion(player_wallet, activity_id, rewards)
            
            return rewards
        
        except Exception as e:
            logger.error(f"Error completing activity: {e}", exc_info=True)
            return None
    
    async def _record_activity_completion(
        self,
        player_wallet: Wallet,
        activity_id: str,
        rewards: ActivityReward
    ) -> bool:
        """
        Record an activity completion on the blockchain.
        
        Args:
            player_wallet: The wallet of the player who completed the activity.
            activity_id: The ID of the completed activity.
            rewards: The rewards for completing the activity.
            
        Returns:
            True if the completion was recorded successfully, False otherwise.
        """
        logger.info(f"Recording completion of activity {activity_id}")
        
        try:
            # Create a transaction to record the completion
            complete_tx = player_wallet.create_activity_complete_transaction(
                activity_id=activity_id,
                bits_reward=rewards.bits,
                aura_reward=rewards.aura,
                reputation_reward=rewards.reputation
            )
            
            # Submit the transaction
            if not await asyncio.to_thread(self.ledger.submit_transaction, complete_tx):
                logger.error(f"Failed to record completion of activity {activity_id}")
                return False
            
            # Create a block to confirm the transaction
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            return True
        
        except Exception as e:
            logger.error(f"Error recording activity completion: {e}", exc_info=True)
            return False


# Singleton instance for easy access
blockchain_integration = None


def initialize_blockchain_integration(
    ledger: ZoologistLedger,
    activity_manager: Optional[ActivityManager] = None,
    crafting_system: Optional[CraftingSystem] = None,
    marketplace: Optional[Union[LocalMarketplace, GlobalMarketplace]] = None
) -> BlockchainIntegration:
    """
    Initialize the blockchain integration singleton.
    
    Args:
        ledger: The Zoologist's Ledger instance.
        activity_manager: The activity manager.
        crafting_system: The crafting system.
        marketplace: The marketplace.
        
    Returns:
        The initialized blockchain integration.
    """
    global blockchain_integration
    
    if blockchain_integration is None:
        blockchain_integration = BlockchainIntegration(
            ledger=ledger,
            activity_manager=activity_manager,
            crafting_system=crafting_system,
            marketplace=marketplace
        )
    
    return blockchain_integration


def get_blockchain_integration() -> Optional[BlockchainIntegration]:
    """
    Get the blockchain integration singleton.
    
    Returns:
        The blockchain integration, or None if not initialized.
    """
    return blockchain_integration


if __name__ == "__main__":
    print("This module is not meant to be run directly.")
    print("Import it and use its functions to integrate the Critter-Craft systems with the blockchain.")"""
Optimized integration module for the Critter-Craft systems.

This module provides a unified interface for integrating all Critter-Craft systems
with the blockchain, including the battle system, breeding system, economy system,
and activities system.
"""

import sys
import os
import time
import random
import logging
import asyncio
from typing import Dict, List, Optional, Tuple, Any, Union, TypeVar
from pathlib import Path
from functools import lru_cache
from concurrent.futures import ThreadPoolExecutor

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler("integration.log"),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger("crittercraft.integration")

# Add the necessary directories to the Python path using pathlib for better path handling
PALLETS_DIR = Path(__file__).parent
sys.path.insert(0, str(PALLETS_DIR / 'pallet-battles' / 'src'))
sys.path.insert(0, str(PALLETS_DIR / 'pallet-ledger' / 'src'))
sys.path.insert(0, str(PALLETS_DIR / 'pallet-breeding' / 'src'))
sys.path.insert(0, str(PALLETS_DIR / 'pallet-economy' / 'src'))
sys.path.insert(0, str(PALLETS_DIR / 'pallet-quests' / 'src'))

# Import from battle system
from battle import start_battle, BattleResult, Environment

# Import from blockchain system
from ledger import ZoologistLedger
from wallet import Wallet
from models import TransactionType, ZoologistIdentity, ZoologistLevel

# Import from breeding system
from genetics import (
    GeneticCode, 
    CoreGenes, 
    PotentialGenes, 
    CosmeticGenes, 
    AuraType,
    Stat
)
from synthesis import (
    EchoSynthesizer, 
    SynthesisType, 
    SynthesisState,
    SynthesisResult
)
from catalysts import (
    StableCatalyst, 
    UnstableCatalyst,
    Catalyst
)
from lineage import (
    FamilyTree, 
    calculate_inbreeding_coefficient
)

# Import from economy system
from items import (
    ItemType,
    ItemRarity,
    Item,
    Material,
    Consumable,
    Gear,
    Blueprint,
    QuestItem,
    BridgingItem,
    BreedingCatalyst,
    GeneSplicer,
    NFTMintingKit
)
from currencies import Bits, Aura
from marketplace import (
    LocalMarketplace,
    GlobalMarketplace,
    OrderType
)
from inventory import Inventory
from crafting import (
    CraftingSystem,
    Recipe,
    CraftingResult
)

# Import from activities system
from activities import ActivityType, StatType
from activities_system import (
    ActivityManager,
    Activity,
    MiniGame,
    Job,
    Quest,
    AdventurousQuest,
    ActivityReward
)

# Type variables for better type hinting
T = TypeVar('T')


class BlockchainIntegration:
    """
    A unified interface for integrating all Critter-Craft systems with the blockchain.
    
    This class provides methods for interacting with the blockchain and integrating
    the various systems in Critter-Craft, including the battle system, breeding system,
    economy system, and activities system.
    
    Attributes:
        ledger: The Zoologist's Ledger instance.
        activity_manager: The activity manager.
        crafting_system: The crafting system.
        marketplace: The marketplace.
        thread_pool: A thread pool for executing blockchain operations asynchronously.
    """
    
    def __init__(
        self,
        ledger: ZoologistLedger,
        activity_manager: Optional[ActivityManager] = None,
        crafting_system: Optional[CraftingSystem] = None,
        marketplace: Optional[Union[LocalMarketplace, GlobalMarketplace]] = None
    ):
        """
        Initialize the blockchain integration.
        
        Args:
            ledger: The Zoologist's Ledger instance.
            activity_manager: The activity manager.
            crafting_system: The crafting system.
            marketplace: The marketplace.
        """
        self.ledger = ledger
        self.activity_manager = activity_manager or ActivityManager()
        self.crafting_system = crafting_system or CraftingSystem()
        self.marketplace = marketplace or GlobalMarketplace()
        self.thread_pool = ThreadPoolExecutor(max_workers=4)
        
        logger.info("BlockchainIntegration initialized")
    
    # ===== Battle System Integration =====
    
    async def battle_with_blockchain(
        self,
        player_wallet: Wallet,
        opponent_wallet: Wallet,
        player_pet: Dict[str, Any],
        opponent_pet: Dict[str, Any],
        environment_type: str,
        player_items: List[Item]
    ) -> Dict[str, Any]:
        """
        Run a battle and record significant events on the blockchain.
        
        This method is asynchronous to allow for non-blocking blockchain operations.
        
        Args:
            player_wallet: The wallet of the player.
            opponent_wallet: The wallet of the opponent.
            player_pet: The player's pet data.
            opponent_pet: The opponent's pet data.
            environment_type: The type of environment for the battle.
            player_items: The items the player has available.
            
        Returns:
            The result of the battle.
        """
        logger.info(f"Starting battle between {player_pet.get('name')} and {opponent_pet.get('name')}")
        
        # Convert items to the format expected by the battle system
        battle_items = self._convert_items_to_battle_format(player_items)
        
        # Start the battle (this is synchronous but could be made async in the future)
        try:
            battle_result = await asyncio.to_thread(
                start_battle,
                player_pet,
                opponent_pet,
                environment_type,
                battle_items
            )
            
            # Record significant events on the blockchain asynchronously
            if battle_result["winner"] == "player":
                asyncio.create_task(self._record_battle_victory(
                    player_wallet=player_wallet,
                    opponent_wallet=opponent_wallet,
                    player_pet=player_pet,
                    opponent_pet=opponent_pet,
                    battle_result=battle_result
                ))
            
            return battle_result
        
        except Exception as e:
            logger.error(f"Error during battle: {e}", exc_info=True)
            return {"winner": "opponent", "error": str(e)}
    
    async def _record_battle_victory(
        self,
        player_wallet: Wallet,
        opponent_wallet: Wallet,
        player_pet: Dict[str, Any],
        opponent_pet: Dict[str, Any],
        battle_result: Dict[str, Any]
    ) -> bool:
        """
        Record a significant battle victory on the Zoologist's Ledger.
        
        Args:
            player_wallet: The wallet of the winning player.
            opponent_wallet: The wallet of the losing player.
            player_pet: The winning pet's data.
            opponent_pet: The losing pet's data.
            battle_result: The result of the battle.
            
        Returns:
            True if the victory was recorded successfully, False otherwise.
        """
        logger.info(f"Recording battle victory for {player_wallet.address}")
        
        # Only record significant victories (e.g., against Alpha critters)
        if not opponent_pet.get("is_alpha", False):
            return False
        
        try:
            # Update the winner's reputation
            reputation_tx = player_wallet.create_reputation_update_transaction(
                target_did=player_wallet.address,
                change_amount=10,  # Significant reputation boost for defeating an Alpha
                reason_code="ALPHA_VICTORY"
            )
            
            # Submit the transaction
            if not await asyncio.to_thread(self.ledger.submit_transaction, reputation_tx):
                logger.error(f"Failed to submit reputation transaction for {player_wallet.address}")
                return False
            
            # If the winner's pet has an NFT ID, record its evolution
            if "nft_id" in player_pet:
                evolve_tx = player_wallet.create_pet_evolve_transaction(
                    pet_id=player_pet["nft_id"],
                    new_form_id=f"evolved_{player_pet['species'].lower()}_1"
                )
                
                # Submit the transaction
                if not await asyncio.to_thread(self.ledger.submit_transaction, evolve_tx):
                    logger.error(f"Failed to submit pet evolution transaction for {player_pet['nft_id']}")
                    return False
            
            # Create a block to confirm the transactions
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            return True
        
        except Exception as e:
            logger.error(f"Error recording battle victory: {e}", exc_info=True)
            return False
    
    def _convert_items_to_battle_format(self, items: List[Item]) -> List[Dict[str, Any]]:
        """
        Convert items to the format expected by the battle system.
        
        Args:
            items: The items to convert.
            
        Returns:
            The items in battle system format.
        """
        battle_items = []
        
        for item in items:
            if item.item_type == ItemType.CONSUMABLE:
                battle_items.append({
                    "id": item.id,
                    "name": item.name,
                    "effect_type": getattr(item, "effect_type", "healing"),
                    "effect_value": getattr(item, "effect_value", 10),
                    "duration": getattr(item, "duration", 0)
                })
        
        return battle_items
    
    # ===== Breeding System Integration =====
    
    async def perform_breeding(
        self,
        player_wallet: Wallet,
        parent_a_id: str,
        parent_b_id: str,
        synthesis_type: SynthesisType,
        catalyst: Optional[Catalyst] = None,
        gene_splicers: Optional[List[GeneSplicer]] = None
    ) -> Optional[str]:
        """
        Perform breeding between two pets and record the result on the blockchain.
        
        Args:
            player_wallet: The wallet of the player performing the breeding.
            parent_a_id: The ID of the first parent pet.
            parent_b_id: The ID of the second parent pet.
            synthesis_type: The type of synthesis to perform.
            catalyst: The catalyst to use.
            gene_splicers: List of gene splicers to use.
            
        Returns:
            The ID of the offspring pet, or None if breeding failed.
        """
        logger.info(f"Performing breeding between pets {parent_a_id} and {parent_b_id}")
        
        try:
            # Get the parent pets from the blockchain
            parent_a_nft = await asyncio.to_thread(self.ledger.get_pet, parent_a_id)
            parent_b_nft = await asyncio.to_thread(self.ledger.get_pet, parent_b_id)
            
            if not parent_a_nft or not parent_b_nft:
                logger.error("One or both parent pets not found")
                return None
            
            # Fetch the off-chain metadata (in a real implementation)
            # For this prototype, we'll generate random metadata
            parent_a_metadata = self._generate_random_pet_metadata()
            parent_b_metadata = self._generate_random_pet_metadata()
            
            # Convert the pets to genetic codes
            parent_a = self._convert_blockchain_pet_to_genetic_code(parent_a_nft, parent_a_metadata)
            parent_b = self._convert_blockchain_pet_to_genetic_code(parent_b_nft, parent_b_metadata)
            
            # Create a family tree
            family_tree = FamilyTree()
            family_tree.add_pet(parent_a)
            family_tree.add_pet(parent_b)
            
            # Check for inbreeding
            inbreeding_coefficient = calculate_inbreeding_coefficient(family_tree, parent_a, parent_b)
            
            if inbreeding_coefficient > 0.25:
                logger.warning(f"High inbreeding coefficient: {inbreeding_coefficient:.2f}")
            
            # Create a default catalyst if none provided
            if not catalyst:
                if synthesis_type == SynthesisType.INTRA_SPECIES:
                    catalyst = StableCatalyst(quality=random.randint(1, 5))
                else:
                    catalyst = UnstableCatalyst(quality=random.randint(1, 5))
            
            # Get the player's zoologist level
            zoologist = await asyncio.to_thread(self.ledger.get_zoologist, player_wallet.address)
            zoologist_level = self._get_zoologist_level(zoologist)
            
            # Create the Echo-Synthesizer
            synthesizer = EchoSynthesizer()
            
            # Set parent happiness (in a real game, this would be the actual happiness values)
            parent_a_happiness = 80
            parent_b_happiness = 90
            
            # Perform the synthesis
            result = await asyncio.to_thread(
                synthesizer.synthesize,
                parent_a=parent_a,
                parent_b=parent_b,
                parent_a_happiness=parent_a_happiness,
                parent_b_happiness=parent_b_happiness,
                synthesis_type=synthesis_type,
                zoologist_level=zoologist_level,
                catalysts=[catalyst],
                gene_splicers=gene_splicers or []
            )
            
            # Check the result
            if result.state != SynthesisState.COMPLETED or not result.offspring:
                logger.error(f"Breeding failed: {result.error_message}")
                return None
            
            # Apply inbreeding penalty if necessary
            offspring = result.offspring
            if inbreeding_coefficient > 0.25:
                offspring = self._apply_inbreeding_penalty(offspring, inbreeding_coefficient)
            
            # Record the breeding on the blockchain
            logger.info("Recording breeding on the Zoologist's Ledger")
            
            # Create a transaction to mint the offspring as a pet NFT
            pet_tx = player_wallet.create_pet_mint_transaction(
                species=offspring.core.species,
                aura_color=offspring.core.aura.name,
                genetic_hash=offspring.calculate_genetic_hash(),
                metadata_uri=f"https://api.crittercraft.com/pets/{offspring.calculate_genetic_hash()}"
            )
            
            if not await asyncio.to_thread(self.ledger.submit_transaction, pet_tx):
                logger.error("Failed to record breeding on the blockchain")
                return None
            
            # Create a block to confirm the transaction
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            # Get the minted pet
            player_pets = await asyncio.to_thread(self.ledger.get_pets_by_owner, player_wallet.address)
            
            if not player_pets:
                logger.error("Failed to retrieve the offspring from the blockchain")
                return None
            
            # Return the ID of the most recently minted pet
            return player_pets[-1].token_id
        
        except Exception as e:
            logger.error(f"Error performing breeding: {e}", exc_info=True)
            return None
    
    def _generate_random_pet_metadata(self) -> Dict[str, Any]:
        """
        Generate random metadata for a pet.
        
        Returns:
            Random pet metadata.
        """
        return {
            "stat_potential": {stat.name: random.randint(50, 80) for stat in Stat},
            "adaptation_slots": random.randint(3, 5),
            "size": random.choice(["TINY", "SMALL", "STANDARD", "LARGE", "HUGE"]),
            "pattern": random.choice(["SOLID", "SPOTTED", "STRIPED", "MOTTLED", "IRIDESCENT"]),
            "marking_color": f"#{random.randint(0, 0xFFFFFF):06x}",
            "glow_intensity": random.uniform(0.0, 1.0)
        }
    
    def _convert_blockchain_pet_to_genetic_code(self, pet_nft, metadata: Dict[str, Any]) -> GeneticCode:
        """
        Convert a pet NFT from the blockchain to a genetic code.
        
        Args:
            pet_nft: The pet NFT from the blockchain.
            metadata: The off-chain metadata for the pet.
            
        Returns:
            The pet's genetic code.
        """
        # Create core genes
        core = CoreGenes(
            species=pet_nft.species,
            aura=AuraType[pet_nft.aura_color],
            genesis_id=pet_nft.token_id,
            lineage=pet_nft.evolution_history
        )
        
        # Create potential genes
        potential = PotentialGenes(
            stat_potential=metadata.get("stat_potential", {}),
            adaptation_slots=metadata.get("adaptation_slots", 3)
        )
        
        # Create cosmetic genes
        cosmetic = CosmeticGenes(
            size=metadata.get("size", "STANDARD"),
            pattern=metadata.get("pattern", "SOLID"),
            marking_color=metadata.get("marking_color", "#FFFFFF"),
            glow_intensity=metadata.get("glow_intensity", 0.0)
        )
        
        return GeneticCode(
            core=core,
            potential=potential,
            cosmetic=cosmetic
        )
    
    def _get_zoologist_level(self, zoologist) -> int:
        """
        Get the numeric level of a zoologist.
        
        Args:
            zoologist: The zoologist object.
            
        Returns:
            The numeric level of the zoologist.
        """
        if not zoologist:
            return 1
        
        return {
            ZoologistLevel.NOVICE: 1,
            ZoologistLevel.APPRENTICE: 2,
            ZoologistLevel.JOURNEYMAN: 3,
            ZoologistLevel.EXPERT: 4,
            ZoologistLevel.MASTER: 5,
            ZoologistLevel.GRANDMASTER: 6
        }.get(zoologist.level, 1)
    
    def _apply_inbreeding_penalty(self, offspring: GeneticCode, inbreeding_coefficient: float) -> GeneticCode:
        """
        Apply an inbreeding penalty to an offspring.
        
        Args:
            offspring: The offspring to apply the penalty to.
            inbreeding_coefficient: The inbreeding coefficient.
            
        Returns:
            The offspring with the penalty applied.
        """
        # Apply a negative mutation to a random stat
        stat = random.choice(list(Stat))
        current_potential = offspring.potential.stat_potential.get(stat, 50)
        penalty = int(inbreeding_coefficient * 20)  # Higher coefficient = higher penalty
        offspring.potential.stat_potential[stat] = max(1, current_potential - penalty)
        
        logger.info(f"Inbreeding penalty applied: {stat.name} potential reduced by {penalty}")
        
        return offspring
    
    # ===== Economy System Integration =====
    
    async def craft_item(
        self,
        player_wallet: Wallet,
        inventory: Inventory,
        recipe_id: str
    ) -> Optional[Item]:
        """
        Craft an item using the crafting system.
        
        Args:
            player_wallet: The wallet of the player crafting the item.
            inventory: The player's inventory.
            recipe_id: The ID of the recipe to use.
            
        Returns:
            The crafted item, or None if crafting failed.
        """
        logger.info(f"Crafting item with recipe {recipe_id}")
        
        try:
            # Get the player's zoologist level
            zoologist = await asyncio.to_thread(self.ledger.get_zoologist, player_wallet.address)
            zoologist_level = self._get_zoologist_level(zoologist)
            
            # Check if the player knows the recipe
            if not self.crafting_system.knows_recipe(player_wallet.address, recipe_id):
                logger.error(f"Player does not know the {recipe_id} recipe")
                return None
            
            # Craft the item
            result, item = await asyncio.to_thread(
                self.crafting_system.craft_item,
                player_id=player_wallet.address,
                recipe_id=recipe_id,
                inventory=inventory,
                player_level=zoologist_level
            )
            
            if result != CraftingResult.SUCCESS or not item:
                logger.error(f"Failed to craft item: {result.name}")
                return None
            
            # Record the crafting on the blockchain if it's a significant item
            if item.rarity in [ItemRarity.RARE, ItemRarity.EPIC, ItemRarity.LEGENDARY]:
                await self._record_significant_crafting(player_wallet, item)
            
            return item
        
        except Exception as e:
            logger.error(f"Error crafting item: {e}", exc_info=True)
            return None
    
    async def _record_significant_crafting(self, player_wallet: Wallet, item: Item) -> bool:
        """
        Record a significant crafting event on the blockchain.
        
        Args:
            player_wallet: The wallet of the player who crafted the item.
            item: The crafted item.
            
        Returns:
            True if the crafting was recorded successfully, False otherwise.
        """
        logger.info(f"Recording significant crafting of {item.name}")
        
        try:
            # Create a transaction to record the crafting
            craft_tx = player_wallet.create_item_craft_transaction(
                item_id=item.id,
                item_type=item.item_type.name,
                rarity=item.rarity.name,
                metadata_uri=f"https://api.crittercraft.com/items/{item.id}"
            )
            
            # Submit the transaction
            if not await asyncio.to_thread(self.ledger.submit_transaction, craft_tx):
                logger.error(f"Failed to record crafting of {item.name}")
                return False
            
            # Create a block to confirm the transaction
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            return True
        
        except Exception as e:
            logger.error(f"Error recording significant crafting: {e}", exc_info=True)
            return False
    
    async def list_item_on_marketplace(
        self,
        player_wallet: Wallet,
        item_id: str,
        price: int,
        currency_type: str = "BITS"
    ) -> bool:
        """
        List an item on the marketplace.
        
        Args:
            player_wallet: The wallet of the player listing the item.
            item_id: The ID of the item to list.
            price: The price of the item.
            currency_type: The type of currency to use.
            
        Returns:
            True if the item was listed successfully, False otherwise.
        """
        logger.info(f"Listing item {item_id} on marketplace for {price} {currency_type}")
        
        try:
            # Create a transaction to list the item
            list_tx = player_wallet.create_marketplace_list_transaction(
                item_id=item_id,
                price=price,
                currency_type=currency_type
            )
            
            # Submit the transaction
            if not await asyncio.to_thread(self.ledger.submit_transaction, list_tx):
                logger.error(f"Failed to list item {item_id}")
                return False
            
            # Create a block to confirm the transaction
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            return True
        
        except Exception as e:
            logger.error(f"Error listing item on marketplace: {e}", exc_info=True)
            return False
    
    # ===== Activities System Integration =====
    
    async def complete_activity(
        self,
        player_wallet: Wallet,
        activity_id: str,
        score: int
    ) -> Optional[ActivityReward]:
        """
        Complete an activity and record the completion on the blockchain.
        
        Args:
            player_wallet: The wallet of the player completing the activity.
            activity_id: The ID of the activity to complete.
            score: The player's score in the activity.
            
        Returns:
            The rewards for completing the activity, or None if completion failed.
        """
        logger.info(f"Completing activity {activity_id} with score {score}")
        
        try:
            # Get the player's zoologist level
            zoologist = await asyncio.to_thread(self.ledger.get_zoologist, player_wallet.address)
            player_level = self._get_zoologist_level(zoologist)
            
            # Complete the activity
            rewards = await asyncio.to_thread(
                self.activity_manager.complete_activity,
                player_id=player_wallet.address,
                activity_id=activity_id,
                score=score,
                player_level=player_level
            )
            
            # Record the completion on the blockchain
            await self._record_activity_completion(player_wallet, activity_id, rewards)
            
            return rewards
        
        except Exception as e:
            logger.error(f"Error completing activity: {e}", exc_info=True)
            return None
    
    async def _record_activity_completion(
        self,
        player_wallet: Wallet,
        activity_id: str,
        rewards: ActivityReward
    ) -> bool:
        """
        Record an activity completion on the blockchain.
        
        Args:
            player_wallet: The wallet of the player who completed the activity.
            activity_id: The ID of the completed activity.
            rewards: The rewards for completing the activity.
            
        Returns:
            True if the completion was recorded successfully, False otherwise.
        """
        logger.info(f"Recording completion of activity {activity_id}")
        
        try:
            # Create a transaction to record the completion
            complete_tx = player_wallet.create_activity_complete_transaction(
                activity_id=activity_id,
                bits_reward=rewards.bits,
                aura_reward=rewards.aura,
                reputation_reward=rewards.reputation
            )
            
            # Submit the transaction
            if not await asyncio.to_thread(self.ledger.submit_transaction, complete_tx):
                logger.error(f"Failed to record completion of activity {activity_id}")
                return False
            
            # Create a block to confirm the transaction
            if player_wallet.address in self.ledger.consensus.validators:
                await asyncio.to_thread(self.ledger.create_block, player_wallet)
            
            return True
        
        except Exception as e:
            logger.error(f"Error recording activity completion: {e}", exc_info=True)
            return False


# Singleton instance for easy access
blockchain_integration = None


def initialize_blockchain_integration(
    ledger: ZoologistLedger,
    activity_manager: Optional[ActivityManager] = None,
    crafting_system: Optional[CraftingSystem] = None,
    marketplace: Optional[Union[LocalMarketplace, GlobalMarketplace]] = None
) -> BlockchainIntegration:
    """
    Initialize the blockchain integration singleton.
    
    Args:
        ledger: The Zoologist's Ledger instance.
        activity_manager: The activity manager.
        crafting_system: The crafting system.
        marketplace: The marketplace.
        
    Returns:
        The initialized blockchain integration.
    """
    global blockchain_integration
    
    if blockchain_integration is None:
        blockchain_integration = BlockchainIntegration(
            ledger=ledger,
            activity_manager=activity_manager,
            crafting_system=crafting_system,
            marketplace=marketplace
        )
    
    return blockchain_integration


def get_blockchain_integration() -> Optional[BlockchainIntegration]:
    """
    Get the blockchain integration singleton.
    
    Returns:
        The blockchain integration, or None if not initialized.
    """
    return blockchain_integration


if __name__ == "__main__":
    print("This module is not meant to be run directly.")
    print("Import it and use its functions to integrate the Critter-Craft systems with the blockchain.")