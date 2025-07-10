# config_activities.py
"""
Data-driven configuration for all activities in the Critter-Craft Universe.

This file isolates game content from game logic, allowing for easy balancing,
addition, and modification of activities without changing the core engine.
It serves as the single source of truth for what activities exist in the game.
"""
from activities import ActivityType, StatType # Direct import for sibling module

# (K) - A clear, centralized definition of all game activities.
# Adding a new mini-game is as simple as adding a new dictionary to this list.
ACTIVITIES_CONFIG = [
    # Mini-Games (Solo Skill & Stat Training)
    {
        "id": "minigame_logic_leaper",
        "type": ActivityType.MINI_GAME,
        "name": "Logic Leaper",
        "description": "A tile-based puzzle game of pathfinding and strategic planning.",
        "required_level": 1,
        "stats_trained": [StatType.IQ, StatType.ENERGY],
        "reward_config": {
            "base_bits_per_point": 5,
            "primary_stat_exp_per_point": 10,
            "secondary_stat_exp_per_point": 5,
            "reputation_per_100_points": 10,
            "special_item_drops": [{
                "item_id": "focus_root",
                "min_score": 50,
                "chance": 0.2,
            }]
        }
    },
    {
        "id": "minigame_aura_weaving",
        "type": ActivityType.MINI_GAME,
        "name": "Aura Weaving",
        "description": "A rhythm and pattern-matching game of timing and memory.",
        "required_level": 1,
        "stats_trained": [StatType.CHARISMA, StatType.SOCIAL],
        "reward_config": {
            "base_bits_per_point": 5,
            "primary_stat_exp_per_point": 10,
            "secondary_stat_exp_per_point": 5,
            "reputation_per_100_points": 10,
            "special_item_drops": [{
                "item_id": "aura_dust",
                "min_score": 50,
                "chance": 0.2,
            }]
        }
    },
    {
        "id": "minigame_habitat_dash",
        "type": ActivityType.MINI_GAME,
        "name": "Habitat Dash",
        "description": "An 'endless runner' style game through procedurally generated habitats.",
        "required_level": 1,
        "stats_trained": [StatType.ENERGY, StatType.AGILITY],
        "reward_config": {
            "base_bits_per_point": 5,
            "primary_stat_exp_per_point": 10,
            "secondary_stat_exp_per_point": 5,
            "reputation_per_100_points": 10,
            "special_item_drops": [{
                "item_id": "sunpetal_pollen",
                "min_score": 50,
                "chance": 0.3,
            }, {
                "item_id": "cave_mushroom",
                "min_score": 75,
                "chance": 0.2,
            }]
        }
    },
    
    # Two-Player Games (Cooperative & Competitive)
    {
        "id": "twoplayer_critter_tactics",
        "type": ActivityType.TWO_PLAYER_GAME,
        "name": "Critter Tactics",
        "description": "A turn-based tactical board game, the ultimate strategic duel.",
        "required_level": 5,
        "stats_trained": [StatType.IQ, StatType.SOCIAL],
        "reward_config": {
            "base_bits_per_point": 8,
            "primary_stat_exp_per_point": 15,
            "secondary_stat_exp_per_point": 8,
            "reputation_per_100_points": 20,
            "special_item_drops": [{
                "item_id": "tactical_emblem",
                "min_score": 100,
                "chance": 0.1,
            }]
        }
    },
    {
        "id": "twoplayer_cooperative_crafting",
        "type": ActivityType.TWO_PLAYER_GAME,
        "name": "Cooperative Crafting",
        "description": "Two players bring unique materials to a Synergy Workbench to craft powerful items.",
        "required_level": 3,
        "stats_trained": [StatType.SOCIAL, StatType.CHARISMA],
        "reward_config": {
            "base_bits_per_point": 7,
            "primary_stat_exp_per_point": 12,
            "secondary_stat_exp_per_point": 6,
            "reputation_per_100_points": 15,
            "special_item_drops": [{
                "item_id": "synergy_crystal",
                "min_score": 80,
                "chance": 0.15,
            }]
        }
    },
    
    # Jobs (The Economic Engine)
    {
        "id": "job_crystal_mining",
        "type": ActivityType.JOB,
        "name": "Crystal Mining",
        "description": "Mine deep caves for rare gems. Requires high Strength.",
        "required_level": 2,
        "duration_seconds": 3600,
        "primary_stat": StatType.STRENGTH,
        "reward_config": {
            "base_bits_per_hour": 500,
            "primary_stat_exp_per_hour": 100,
        }
    },
    {
        "id": "job_bioluminescent_guide",
        "type": ActivityType.JOB,
        "name": "Bioluminescent Guide",
        "description": "Guide travelers through dark areas. Requires high Charisma.",
        "required_level": 2,
        "duration_seconds": 7200,
        "primary_stat": StatType.CHARISMA,
        "reward_config": {
            "base_bits_per_hour": 400,
            "primary_stat_exp_per_hour": 80,
        }
    },
    {
        "id": "job_herbalist_assistant",
        "type": ActivityType.JOB,
        "name": "Herbalist's Assistant",
        "description": "Identify and sort rare herbs for an apothecary. Requires high IQ.",
        "required_level": 2,
        "duration_seconds": 5400,
        "primary_stat": StatType.IQ,
        "reward_config": {
            "base_bits_per_hour": 450,
            "primary_stat_exp_per_hour": 90,
        }
    },
    
    # Quests (Narrative & Progression)
    {
        "id": "quest_gathering_sunpetal",
        "type": ActivityType.QUEST,
        "name": "Gathering: Sunpetal Pollen",
        "description": "Collect 10 Sunpetal Pollens for the local herbalist.",
        "required_level": 1,
        "objectives": ["Collect 10 Sunpetal Pollens", "Return to the herbalist"],
        "reward_config": {
            "base_bits": 200,
            "base_aura": 5,
            "reputation": 20,
            "guaranteed_items": ["healing_salve"]
        }
    },
    {
        "id": "quest_crafting_salves",
        "type": ActivityType.QUEST,
        "name": "Crafting: Healing Salves",
        "description": "Craft 3 Healing Salves for the outpost.",
        "required_level": 2,
        "objectives": ["Gather ingredients", "Craft 3 Healing Salves", "Deliver to the outpost"],
        "reward_config": {
            "base_bits": 300,
            "base_aura": 8,
            "reputation": 30,
            "guaranteed_items": ["crafting_manual_basic"]
        }
    },
    {
        "id": "quest_pacification_glimmermoth",
        "type": ActivityType.QUEST,
        "name": "Pacification: Alpha Glimmer-Moth",
        "description": "A territorial Alpha Glimmer-Moth is causing trouble. Pacify it in a battle.",
        "required_level": 3,
        "objectives": ["Locate the Alpha Glimmer-Moth", "Pacify it in battle", "Report back to the village elder"],
        "reward_config": {
            "base_bits": 400,
            "base_aura": 12,
            "reputation": 40,
            "guaranteed_items": ["glimmermoth_wing"]
        }
    },
    
    # Adventurous Quests (Epic, Multi-Stage Sagas)
    {
        "id": "quest_whispering_blight",
        "type": ActivityType.ADVENTUROUS_QUEST,
        "name": "The Whispering Blight",
        "description": "Investigate a strange torpor affecting critters in the Verdant Maw.",
        "required_level": 10,
        "stages": [
            {
                "name": "The Mystery",
                "description": "A renowned Zoologist reports that critters in the remote jungle are falling into a strange torpor. Travel to the 'Verdant Maw' habitat and use pets with high IQ to investigate, finding traces of a strange, parasitic fungus.",
                "objectives": ["Travel to Verdant Maw", "Find traces of the parasitic fungus", "Collect samples"]
            },
            {
                "name": "The Research",
                "description": "The samples must be taken to a Master Crafter, who determines a cure is possible, but requires three rare, powerful components: the Heart of a Magma-Slug (found only in the Geothermal Vents after a boss battle), untainted Starlight Algae (gathered via a complex Aura Weaving challenge), and a Blueprint: Blight-Ward Charm held by a rival Zoologist who will only relinquish it after being defeated in a high-stakes Critter Tactics match.",
                "objectives": ["Obtain the Heart of a Magma-Slug", "Gather untainted Starlight Algae", "Win the Blueprint: Blight-Ward Charm"]
            },
            {
                "name": "The Culmination",
                "description": "With the Blight-Ward Charm crafted and equipped, enter the heart of the jungle, where you face the source of the blight: a colossal, ancient creature covered in the parasitic fungus. This is a multi-phase boss battle that requires using the charm at key moments to weaken the boss.",
                "objectives": ["Craft the Blight-Ward Charm", "Enter the heart of the jungle", "Defeat the ancient creature"]
            }
        ],
        "reward_config": {
            "base_bits": 5000,
            "base_aura": 50,
            "reputation": 200,
            "guaranteed_items": ["legendary_blightward_charm"]
        }
    }
]