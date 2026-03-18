# Implemented Systems

This file describes what the game currently supports based on the source code and authored game data in `src/` and `assets/data/game_data.json`.

## Runtime Structure

- Macroquad game loop with `Menu`, `Gameplay`, and `Pause` states.
- JSON-driven content loading through embedded `game_data.json`.
- Native JSON save/load support with versioned save data.

## Core Exploration

- Top-down 8-direction movement with blocker collision.
- Multi-area travel through authored warp regions.
- Camera follow, impact shake, and immediate-mode UI panels/prompts.
- Day clock, day rollover, weather state, season state, and time-of-day windows.
- In-world route/station/NPC prompts with contextual requirement or readiness messaging.
- First-run contextual tutorial toasts for:
  - Crow-led first gather and first brew guidance
  - save/load
  - journal use
  - quest pickup and first delivery
  - early progression from Mira to Rowan to greenhouse restoration
  - potions and route restoration

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
- Gather node silhouettes vary by item type for faster world scanning.
- Herb availability rules are learned only after the first successful collection of that herb.
- Learned herb conditions are surfaced in the field journal rather than exposed directly in map prompts.
- Gathered specimens are tracked in a field journal with route, season, weather, time window, best quality snapshot, variant label, and learned availability context.
- Gather feedback includes world bursts, HUD toasts, best-quality notifications, and variant discovery messaging.

## Inventory and Economy

- Stack-based inventory keyed by item id.
- Buying from a shop station.
- Selling items back through the shop UI.
- Quest-protected sell filtering for active delivery items.
- Inventory sort modes:
  - `priority`
  - `type`
  - `name`
- Inventory context labels for:
  - quest hold
  - recipe use
  - best record
  - safe stock
- Reserved-count visibility in inventory and alchemy selection flows.
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
- Preview classification for:
  - unknown salvage
  - unlogged formula
  - known base, uncertain branch
  - known formula with unstable or imperfect setup
- Instability/failure reasons listed explicitly in the preview panel.
- Known formula memory and mastery summaries surfaced directly in the alchemy/archive UI.

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
- HUD and journal next-goal messaging surfaces closest unlocks and active progression requirements before the player hits a gate.
- Warp readiness and restoration state are signposted both in-world and in the journal.

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
- NPC motion/pathing between scheduled locations, including cross-area travel.
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
- NPC and quest UI surfaces:
  - now/later/usually schedule hints
  - active quest turn-in guidance
  - in-world request/turn-in highlighting for key NPCs
- Current authored story framing emphasizes:
  - people-first quest motivations
  - a town crisis driven by ecological decline
  - the Crow as an early tower guide
  - cross-NPC Phase 1 reaction lines after early quest progress
  - visible early town recovery flourishes tied to quest completion
- Early journal milestones now mark:
  - first useful brew
  - first town relief through alchemy

Current authored NPCs:

- `Crow`
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
- Archive tabs for timeline, experiments, mastery, morphs, disassembly, and duplication.
- Archive reconstruction gate based on completed quest and milestone state.
- Archive reconstruction creates the `archive_revelation` milestone.
- Observatory floor unlock gated by archive revelation.
- Final observatory interaction through an ending-focus station.
- Ending overlay text exists and is triggerable in gameplay.
- Experiment archive retention includes:
  - 60-entry experiment log cap
  - all/stable/unstable filtering
  - page-aware browsing
  - cross-links into recipe mastery and morph history
- Journal tabs surface routes, notes, brew ledger, greenhouse state, rapport, and milestone/readiness summaries.

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
- dialogue trees with player choices
- cutscenes or branching endings
- audio systems
- sprite/animation asset systems beyond simple shape rendering
