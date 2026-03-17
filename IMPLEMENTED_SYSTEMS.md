# Implemented Systems

This file describes what the game currently supports based on the source code and authored game data in `src/` and `assets/data/game_data.json`.

## Runtime Structure

- Macroquad game loop with `Menu`, `Gameplay`, and `Pause` states.
- JSON-driven content loading through embedded `game_data.json`.
- Native JSON save/load support with versioned save data.

## Core Exploration

- Top-down 8-direction movement with blocker collision.
- Multi-area travel through authored warp regions.
- Camera follow and simple immediate-mode UI panels/prompts.
- Day clock, day rollover, weather state, season state, and time-of-day windows.

## World Content

Current authored areas:

- `tower_entry`
- `town_square`
- `moonlit_forest`
- `greenhouse_floor`
- `containment_floor`
- `rune_workshop_floor`
- `archive_floor`
- `observatory_floor`

Current authored gathering routes:

- `tower_ruin_edge`
- `creekside_meadow`
- `moonlit_grove`
- `charred_hollow`
- `greenhouse_walk`
- `containment_ring`
- `archive_stack`
- `observatory_span`

## Gathering

- Authored gather nodes with:
  - route association
  - season gating
  - weather gating
  - time-window gating
  - daily spawn chance
  - field journal note capture
- Node availability refreshes by day.
- Gathered specimens are tracked in a field journal with route, season, weather, time window, best quality snapshot, and variant label.

## Inventory and Economy

- Stack-based inventory keyed by item id.
- Buying from a shop station.
- Selling items back through the shop UI.
- Quest-protected sell filtering for active delivery items.
- Quick potion belt for consuming up to three carried potions.

## Item Model

Current supported item categories in data:

- `ingredient`
- `potion`
- `catalyst`
- `creature`
- `rune`

Ingredient/item data supports:

- base quality
- rarity
- 4-axis element profile
- traits
- source conditions
- wild variants
- synthesis weight/value
- catalyst tags
- potion effect definitions

## Alchemy

- Station-based brewing UI.
- Three ingredient slots plus one catalyst slot.
- Heat, stir count, and timing selection.
- Stable recipe matching by exact ingredient counts.
- Unstable output fallback when process or thresholds miss.
- Trait-derived salvage fallback for non-recipe brews.
- Quality calculation.
- Output quality bands:
  - `Crude`
  - `Serviceable`
  - `Fine`
  - `Excellent`
  - `Masterwork`
- Recipe mastery tracking and mastery stages.
- Preferred and guaranteed trait inheritance on outputs.
- Element minimum checks.
- Catalyst checks.
- Room bonus checks.
- Morph output support for authored recipes.
- Preview UI showing:
  - known/uncertain result
  - quality forecast
  - mastery stage
  - inherited traits
  - process/quality/element pass state
  - catalyst/timing/sequence/room-bonus state

Current authored brew recipes:

- `Healing Draught`
- `Glow Potion`
- `Stamina Tonic`
- `Beastcalm Extract`

Current authored morph path:

- `Glow Potion` can morph to `Star Lantern Elixir`

## Potion Use and Effects

- Immediate potion consumption from inventory.
- Active effect tracking over time.
- Implemented effect kinds in gameplay logic:
  - `restore`
  - `speed`
  - `glow`
  - `misfire`

## Tower Progression

- Floor unlocks are driven by warp requirements.
- Warp requirements can use:
  - total brew count
  - coin cost
  - required item and amount
  - required journal milestone
- Unlock state persists in save data.
- Journal milestones persist in save data.

Current tower progression path in data:

1. Entry Laboratory
2. Greenhouse Floor
3. Creature Containment
4. Rune Workshop
5. Archives
6. Observatory

## Greenhouse and Cultivation

- Greenhouse floor unlock and persistence.
- Greenhouse still as a second alchemy station.
- Planter stations with persistent per-bed state.
- Planting from allowed seed/item lists.
- Daily tending.
- Growth stages:
  - `seeded`
  - `sprouting`
  - `budding`
  - `ripe`
- Harvest yields with per-bed bonus.
- Quest-gated greenhouse expansion bed.
- Greenhouse bed status shown in the journal UI.

## Creature Containment

- Non-combat creature collection via gather nodes.
- Creature items can be placed into habitat stations.
- Habitat stations produce renewable output items after day progression.
- Persistent habitat state:
  - placed creature
  - placed day
  - last harvest day

Current collectible creatures:

- `Glow Moth`
- `Dew Slug`

Current renewable habitat outputs:

- `Lumen Dust`
- `Dew Slime`

## Rune Workshop

- Rune workshop floor unlock and persistence.
- Rune workshop UI.
- Authored rune augmentation recipes.
- Post-brew potion augmentation by consuming:
  - one potion
  - one rune
- Resulting augmented potion added to inventory.

Current rune items:

- `Splash Rune`
- `Echo Rune`
- `Delay Rune`

Current rune outputs:

- `Splash Glow Potion`
- `Echo Healing Draught`
- `Delayed Stamina Tonic`

## NPCs, Town Layer, and Quests

- Authored NPCs with:
  - role
  - dialogue variants
  - quest assignment
  - time-of-day schedules
- NPC visibility and position change by time window.
- Relationship/rapport counter per NPC.
- Direct NPC quest acceptance and turn-in.
- Quest board station with accept-able board quests.
- Quest availability gates:
  - prerequisite quests
  - required unlocked warp
  - minimum total brews
- Advanced quest requirement checks:
  - required item/amount
  - minimum quality band
  - required inherited trait
  - required effect kind

Current authored NPCs:

- `Mira`
- `Rowan`
- `Mayor Elric`
- `Ione`
- `Brin`
- `Lyra`

Current authored quests:

- `healing_for_mira`
- `glow_for_rowan`
- `cultivation_for_brin`
- `stamina_for_elric`
- `star_elixir_for_ione`
- `board_glasshouse_stock`
- `board_restorative_stash`
- `containment_for_lyra`

## Archives and End State

- Archive floor unlock and archive console station.
- Archive overlay that displays recovered milestones.
- Archive reconstruction gate based on completed quest and milestone state.
- Archive reconstruction creates the `archive_revelation` milestone.
- Observatory floor unlock gated by archive revelation.
- Final observatory interaction through an ending-focus station.
- Ending overlay text exists and is triggerable in gameplay.

## Save Data

Persistent save state currently includes:

- current area
- player position
- day clock and day index
- total brew count
- coins
- inventory
- gathered nodes
- known recipes
- recipe mastery
- crafted item profiles
- field journal entries
- started/completed quests
- unlocked warps
- planter states
- habitat states
- journal milestones
- NPC relationships

## Current Gaps Relative to the Implemented Codebase

The codebase does not currently show support for:

- combat systems
- enemy AI
- stamina depletion mechanics
- sleep/bed flow
- farming beyond greenhouse planters
- creature feeding/care needs beyond habitat placement and timed harvest
- NPC schedule pathing between points
- dialogue trees with player choices
- cutscenes or branching endings
- audio systems
- sprite/animation asset systems beyond simple shape rendering
