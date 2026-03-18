# Alchemy System Design

## Purpose
Define a concrete alchemy system for `Alchemy Tower` that takes inspiration from Atelier-style synthesis while fitting this game's cozy top-down tower-and-town structure.

This document is meant to drive implementation, balance, and content authoring.

---

## Design Goals

### Primary Goals
- Alchemy is the core progression system, not a side crafting menu.
- Gathering should matter because ingredients have meaningful synthesis roles.
- Brewing should reward both discovery and mastery.
- Early recipes should be readable; late recipes should have depth.
- Tower upgrades should expand alchemy capability in visible ways.

### Feel Goals
- Early game: curious, readable, slightly unstable.
- Mid game: deliberate optimization and recipe branching.
- Late game: deep synthesis chains, specialization, and magical engineering.

### Scope Goals
- Avoid Atelier-scale complexity at the start.
- Borrow the strongest ideas: traits, quality, synthesis thresholds, recipe morphing, mastery.
- Use a system that can be represented clearly in Macroquad UI.

---

## Core Loop

The intended alchemy loop is:

1. Explore for ingredients
2. Learn ingredient properties
3. Combine materials in synthesis
4. Produce potions, tools, reagents, and tower components
5. Use those outputs to unlock new spaces, better gathering, and stronger synthesis
6. Return with new ingredients and repeat

Alchemy should feed:
- healing and buffs
- exploration gating
- tower restoration
- limited specimen cultivation and mutation
- quest fulfillment
- economy and trade
- recipe expansion

### Gathering Loop
Because the player does not run a full farm and combat may be absent, the practical day loop should be:

1. Check weather, season, and known wild conditions
2. Choose a route through town outskirts, forest, marsh, caves, or special sites
3. Gather wild ingredients with light daily variation in spawn quality and location
4. Process raw finds into cleaner alchemical inputs
5. Brew useful outputs
6. Reserve a few rare specimens for tower planters or future commissions
7. Return to the wild with better knowledge rather than stronger weapons

---

## Core Pillars

### 1. Ingredients Are Multi-Dimensional
Each ingredient must matter in more than one way.

Every ingredient should eventually have:
- item identity
- source biome
- rarity tier
- quality
- elemental alignment
- traits
- tags
- synthesis weight/value

Example:
- `Moon Fern`
  - biome: forest
  - elements: luminous, cold
  - traits: `soft_glow`, `cooling`, `nightbound`
  - tags: plant, leaf, magical
  - use: glow recipes, stealth brews, moon-garden mutations

### 2. Recipes Have Stable and Unstable Outcomes
Brewing should not be simple binary success/failure.

Possible outcomes:
- exact stable result
- unstable variant
- weak salvage result
- harmful residue
- hidden morph trigger

This gives experimentation value even when the player is imperfect.

### 3. Mastery Matters
Making an item once is not the same as mastering it.

Recipe mastery stages:
- unknown
- guessed
- discovered
- refined
- mastered

Mastery can improve:
- output quantity
- quality floor
- chance to preserve ingredients
- access to morph branches
- clarity of UI preview

### 4. Tower Upgrades Expand Synthesis
New tower rooms should unlock new synthesis verbs, not just more ingredients.

Examples:
- Entry Lab: base brewing
- Greenhouse: limited specimen cultivation and mutation
- Rune Workshop: enchant outputs
- Archives: recipe refinement and historical notes
- Creature Floor: essence extraction
- Philosopher Engine: high-end transformations and duplication

### 5. Wild Gathering Is The Main Supply Engine
The world, not a farm, should be the main source of alchemical variety.

Design rules:
- common ingredients come from reliable wild routes
- rare ingredients come from conditions, timing, and observation
- the player should learn places, patterns, and micro-seasons
- tower planters should support preservation and nurturing, not replace gathering

---

## Recommended Synthesis Model

## Phase 1 Model: Practical Early Version
Use a compact synthesis structure that is easy to implement and learn.

### Inputs
- 3 ingredient slots
- 1 optional catalyst slot later
- heat level
- stir count

### Hidden/Derived Values
- total quality
- elemental balance
- trait pool
- recipe match state
- process match state

### Resolution
The system checks, in order:

1. ingredient pattern match
2. minimum quality or elemental threshold
3. process requirement match
4. trait transfer rules
5. morph trigger conditions

If all requirements pass:
- create stable item

If ingredients match but process is off:
- create unstable variant

If recipe does not match:
- create trait-derived salvage item or residue

This is the right starting compromise. It preserves experimentation and process without needing a giant synthesis board immediately.

---

## Ingredient Data Model

Each ingredient should support these fields.

```json
{
  "id": "moon_fern",
  "name": "Moon Fern",
  "category": "ingredient",
  "quality": 42,
  "rarity": 2,
  "elements": {
    "vita": 0,
    "ember": 0,
    "mist": 2,
    "lux": 3
  },
  "traits": ["soft_glow", "cooling", "nightbound"],
  "tags": ["plant", "leaf", "forest", "magical"],
  "base_value": 7
}
```

### Recommended Core Stats
- `quality`: numeric base quality
- `rarity`: 1-5
- `elements`: 4-axis or 5-axis synthesis values
- `traits`: effect-bearing descriptors
- `tags`: recipe compatibility and authoring convenience
- `source_conditions`: when and where this ingredient is likely to appear
- `wild_variants`: optional altered-trait or high-quality versions tied to specific conditions

### Recommended Element Set
Keep the number small.

Use 4 elements:
- `vita` for healing, growth, restoration
- `ember` for heat, force, volatility
- `mist` for cold, stealth, fluidity
- `lux` for light, arcane resonance, perception

This is simpler than Atelier's broader elemental presentation and easier to visualize.

---

## Trait System

Traits are the main source of alchemical personality.

### Trait Categories
- potency traits
- duration traits
- side-effect traits
- instability traits
- utility traits
- economic traits

### Examples
- `soft_glow`: grants glow effect
- `restorative`: increases healing value
- `volatile`: improves power, raises instability
- `sticky`: increases duration
- `pure`: increases quality carryover
- `bitter`: adds side-effect chance
- `fragrant`: boosts sale value for perfumes/tonics

### Transfer Rules
Do not transfer every trait from every ingredient.

Recommended early rule:
- potion can inherit up to 2 traits
- inherited traits are chosen from:
  - guaranteed recipe traits
  - strongest shared ingredient traits
  - catalyst traits later

### Why This Matters
This creates a distinction between:
- making the right potion
- making a good version of the right potion

It should also create a distinction between:
- finding a common ingredient
- finding an exceptional wild specimen

---

## Quality System

Quality should affect more than sale value.

### Quality Sources
- ingredient quality
- recipe fit
- process correctness
- room bonus
- catalyst bonus
- mastery bonus
- source condition bonus
- freshness bonus

### Quality Effects
- stronger item values
- higher sell price
- increased buff duration
- improved healing amount
- better chance for extra effect unlocks
- prerequisite for morphs or advanced orders

### Output Bands
- `Crude`
- `Serviceable`
- `Fine`
- `Excellent`
- `Masterwork`

Use bands in the UI even if the underlying system is numeric.

---

## Process Layer

This is the part that turns synthesis into gameplay instead of recipe checking.

### Early Process Variables
- heat: 1-3
- stir count: integer

### Midgame Additions
- catalyst
- timing window
- sequence order
- vessel type

### Recommended Rules
- simple healing brews prefer low heat
- luminous brews prefer medium heat and repeated stirring
- volatile vigor brews prefer high heat and exact stir count

### Process Outcomes
- exact process: stable result
- near miss: unstable variant
- bad miss: weak salvage or residue

This gives room for skill and recipe knowledge without demanding reflex gameplay.

---

## Gathering System Design

Gathering is not a support mechanic. It is the acquisition half of the alchemy system.

### Core Gathering Goals
- Make the world feel alive through seasonal and weather-based ingredient shifts.
- Reward route knowledge and observation more than combat strength.
- Keep input friction low with minimal tool use and clear environmental readability.
- Ensure wild gathering remains more valuable than limited tower cultivation.

### Core Gathering Rule
Most alchemical variety should come from the wild.

Tower planters should:
- preserve rare ingredients
- nurture delicate specimens
- produce a tiny safety stock

Tower planters should not:
- become the main bulk source of ingredients
- erase the need to explore
- outscale the wilderness loop

## World Gathering Principles

### 1. Seasonal and Local
The world should teach the player where to look.

Examples:
- spring herbs near streams and wet stones
- summer flowers in meadow clearings
- autumn fungi under rotting logs
- winter crystals near caves and frozen ponds

### 2. Light Daily Randomness
Use light randomness, not chaos.

Daily variation should affect:
- exact node placement within a known route
- quality rolls
- rare replacement spawns
- temporary forage clusters

Daily variation should not:
- invalidate learned routes
- hide essential ingredients unpredictably
- feel roguelike

### 3. Condition-Based Rarity
Rare ingredients should mostly come from conditions rather than pure loot tables.

Examples:
- `Moon Petal` on clear nights near ruins
- `Stormwater Vial` only during thunder
- `Ember Moss` on cold mornings near chimneys
- `Glasscap Fungus` after two rainy days in cave mouths

## Resource Categories

The wild should provide:
- herbs
- flowers
- roots
- fungi
- mosses
- minerals
- waters and dew
- insects or traces if non-combat creature gathering exists
- environmental reagents like ash, clay, soot, salt, and pollen

## Node Types

Use a small set of readable node families:
- visible forage nodes
- hidden or revealable nodes
- timed or ephemeral nodes
- condition-only nodes
- rare specimen nodes

## Minimal Tool Philosophy

Tool friction should remain low.

Recommended approach:
- default hand gathering for most resources
- one general field satchel upgrade path instead of constant tool swapping
- special access should mostly come from potion use, knowledge, or conditions

If special collection items exist, they should be exceptions rather than the baseline.

## Limited Cultivation Design

The player has only a few planters or specimen beds in the tower.

### Their Purpose
- preserve rare ingredients between seasons
- nurture hard-to-source specimens into stable repeats
- grow mutated or specialized variants
- maintain a tiny reserve for favorite recipes

### Their Limits
- low slot count
- long growth time
- cannot outproduce the wilderness for common materials
- cannot generate every environmental reagent

## Gathering Knowledge and Discovery

The player should improve through observation and notes.

### Knowledge Sources
- personal field journal
- villager hints
- seasonal rumors
- archive notes
- repeated successful finds

### Journal Should Track
- biome
- season
- weather
- time of day
- known associated traits
- recent best-quality specimen found

## Processing Between Gathering and Brewing

Raw wild finds should often become cleaner alchemy-ready materials first.

Recommended early processes:
- drying
- grinding
- steeping
- distilling
- pressing
- powdering

Examples:
- herbs -> dried herbs
- petals -> fragrant extract
- roots -> tonic mash
- cave crystals -> luminous dust
- mushrooms -> stable powder

## Gathering-Oriented Potion Design

Since combat may not matter, potions should support exploration and collection.

Important gathering-facing effects:
- reveal hidden nodes
- improve specimen quality
- protect against marsh gas or cave chill
- let the player harvest at night safely
- temporarily increase rare-spawn chance
- preserve freshness for one outing

Examples:
- `Rootwake Serum`: reveals hidden roots and buried herbs
- `Glow Potion`: enables safe low-light gathering
- `Miststep Vial`: lets the player cross damp or fog-heavy ground
- `Preserver's Draught`: improves chance of pristine specimens

## Exploration Without Combat

If combat is absent or minimal, access should be gated by:
- weather
- time of day
- stamina and time management
- potion-enabled traversal
- social permission
- tower research progress

This keeps exploration layered without relying on enemy encounters.

## Gathering Progression

### Early Game
- a few reliable town and forest routes
- common herbs, mosses, roots, and dust
- obvious seasonal differences
- limited notes and few planters

### Mid Game
- weather-sensitive finds
- hidden nodes
- rare-condition reagents
- specimen preservation in planters
- route optimization through alchemical support items

### Late Game
- precise condition chasing
- morph ingredients
- biome-specific masterpiece specimens
- advanced environmental reagents
- routes that require chained alchemical preparation

---

## Recipe Structure

Each recipe should define:

```json
{
  "id": "glow_potion_recipe",
  "station_id": "entry_cauldron",
  "ingredients": [
    { "item_id": "arcane_dust", "amount": 1 },
    { "item_id": "moon_fern", "amount": 1 }
  ],
  "required_heat": 2,
  "required_stirs": 2,
  "minimum_quality": 20,
  "preferred_traits": ["soft_glow"],
  "output_item_id": "glow_potion",
  "unstable_output_item_id": "lantern_leak",
  "morph_targets": ["star_glass_tonic"]
}
```

### Recipe States
- hidden
- hinted
- discovered
- documented
- mastered

### Discovery Sources
- story journals
- experimentation
- NPC hints
- archive research
- ingredient analysis
- wild gathering observations

---

## Recipe Morphing

This should be one of the major midgame alchemy features.

### Basic Idea
A completed recipe can branch into a new one if extra conditions are met.

### Good Morph Triggers
- exceed quality threshold
- include compatible catalyst
- include a specific rare ingredient tag
- hit exact process requirement
- synthesize under a room bonus condition

### Example
- `Glow Potion`
  - normal result: `Glow Potion`
  - morph condition: quality 60+, catalyst `starlight_shard`, heat 2, stir 3
  - morph result: `Star Lantern Elixir`

### Why It Matters
It turns old recipes into stepping stones rather than dead content.

---

## Failure and Salvage Design

Players should rarely feel that a brew was a total waste.

### Failure Ladder
- correct recipe + bad process -> unstable variant
- good trait mix + no recipe -> salvage tonic
- conflicting trait mix -> residue
- severe instability -> cursed sludge or negative potion

### Good Failure Design Rules
- a failed result should teach something
- a failed result should still have some use or sale value
- repeated total junk outcomes should be rare

This is important because experimentation is only fun when failure has texture.

---

## Potion Effect Design

Potions should be useful in more than combat.

### Core Categories
- restoration
- mobility
- visibility
- harvesting
- mutation
- creature handling
- environmental interaction

### Early Examples
- `Healing Draught`
  - restores vitality
- `Glow Potion`
  - illuminates caves and dark interiors
- `Stamina Tonic`
  - boosts movement speed and gathering stamina

### Midgame Examples
- `Rootwake Serum`
  - reveals buried herbs
- `Miststep Vial`
  - cross shallow marsh gas
- `Beastcalm Extract`
  - lowers aggression of magical creatures

### Late Examples
- `Temporal Solvent`
  - speeds greenhouse growth for one cycle
- `Echo Tincture`
  - repeats the next potion effect at reduced strength

---

## Alchemy Progression by Tower Floor

## Floor 1: Entry Laboratory
Focus:
- base synthesis
- recipe discovery
- stable vs unstable brews
- 3 ingredient slots
- heat and stirring

Unlocks:
- healing, glow, stamina
- simple salvage potions

## Floor 2: Greenhouse
Focus:
- controlled ingredient supply
- crop quality
- mutation
- hybrid ingredients

Unlocks:
- farm-grown trait variants
- quality improvement loops

## Floor 3: Creature Containment
Focus:
- creature drops
- essences
- temperament-linked ingredients

Unlocks:
- stronger catalysts
- beast and venom recipes

## Floor 4: Rune Workshop
Focus:
- post-synthesis augmentation
- rune sockets
- potion modifiers

Unlocks:
- splash, delayed, echo, chained effects

## Floor 5: Archives
Focus:
- recipe reconstruction
- formula refinement
- synthesis theory

Unlocks:
- recipe mastery bonuses
- morph previews
- disassembly

## Floor 6+: Strange Rooms
Focus:
- duplication
- transmutation
- exotic materials
- endgame synthesis engines

---

## Connected Systems

Alchemy should connect directly to the rest of the game.

### Exploration
- potions unlock traversal routes
- alchemy mostly improves gathering through knowledge, traversal effects, and specimen preservation rather than tool complexity

### Economy
- townsfolk buy stable practical brews
- collectors buy rare unstable curiosities
- high-quality potions fulfill premium orders

### Farming
- replace broad farming with a tiny tower cultivation layer for rare specimen preservation, mutation, and out-of-season support

### Creatures
- if creature systems remain, keep them non-combat and secondary to wild forage

### Quests
- quests should ask for:
  - exact recipe
  - minimum quality
  - specific trait
  - specific effect band

### Story
- the wizard's journals should teach synthesis theory over time
- late notes should reveal dangerous morph chains and taboo alchemy

---

## UI Recommendations

The alchemy UI must explain enough to invite experimentation without solving everything immediately.

### Early UI Should Show
- selected ingredients
- available quantity
- heat
- stir count
- predicted output category
- stable/unstable warning
- source notes when relevant, such as season or weather-linked trait boosts

### After Discovery
- show exact recipe name
- show known process requirements
- show expected traits/effects

### After Mastery
- show quality forecast
- show morph chances
- show trait inheritance preview

### Important Rule
Unknown recipes should still give partial information, not opaque nonsense.

Good unknown preview examples:
- “restorative reaction”
- “volatile luminous brew”
- “likely unstable outcome”

---

## Implementation Phases

## Phase A: Foundation
- ingredient traits
- recipe process requirements
- unstable outputs
- known recipe log
- potion consumption with effects
- authored gathering routes with light daily variation
- condition-based rare spawns
- field journal entries for discovered ingredients

## Phase B: Quality and Trait Transfer
- quality math
- inherited traits
- output quality bands
- better preview feedback
- wild ingredient quality modifiers from weather, time, and route conditions
- processing steps between gathering and brewing

## Phase C: Morphing and Mastery
- mastery levels
- morph conditions
- improved archive UI
- formula notes and experiment history
- advanced field notes
- planter-grown rare specimen support

## Phase D: Advanced Alchemy
- catalysts
- disassembly
- duplication
- rune imbuing
- specimen mutation formulas in limited tower planters

---

## Data Requirements

The JSON content pipeline should eventually support:
- item definitions
- ingredient traits
- element values
- gathering node definitions
- biome and route definitions
- spawn condition definitions
- weather and season hooks
- field journal discovery records
- recipe definitions
- unstable outputs
- morph targets
- catalyst definitions
- effect definitions
- mastery thresholds
- station bonuses

---

## Balancing Rules

### Early Game
- reliable recipes must be easy to learn
- failure should sting lightly, not hard
- 1-2 ingredient recipes are fine
- reliable gathering routes must exist for essential materials
- wild nodes should be readable and low-friction to harvest

### Mid Game
- quality starts to matter
- unstable variants become situationally valuable
- morphing begins
- condition-based rare gathering becomes important
- planters help preserve edge-case ingredients without replacing routes

### Late Game
- optimization matters
- ingredient sourcing matters
- room bonuses and catalysts matter
- mastering old recipes remains useful
- route planning, weather reading, and specimen timing should matter more than raw rarity

---

## Non-Goals

Do not do these early:
- huge procedural recipe spaces
- dozens of hidden stats per ingredient
- fully random output systems
- complicated real-time minigame brewing
- trait overload where every item becomes unreadable

The system should feel rich, not chaotic.

---

## Recommended Next Implementation Steps

1. Add ingredient element values, trait arrays, and source-condition metadata to all current materials.
2. Define authored gathering routes and light-randomness spawn rules.
3. Add biome, season, weather, and condition hooks to ingredient and node definitions.
4. Expand recipe data with `required_heat`, `required_stirs`, and unstable outputs.
5. Add quality calculation and output quality bands.
6. Add experiment log/history plus field journal tracking.
7. Add one or two tower planters for rare specimen preservation.
8. Add post-synthesis trait inheritance display.

## Definition of Success

The alchemy system is working when:
- players can intentionally make reliable core brews
- players can learn from unstable outcomes
- ingredient choice matters beyond recipe identity
- tower upgrades open new synthesis possibilities
- alchemy drives the rest of the game instead of merely supporting it
- wild gathering remains the primary source of meaningful ingredient variety
- limited cultivation supports the loop without replacing exploration
