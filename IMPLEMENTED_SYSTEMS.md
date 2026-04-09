# Implemented Systems

This file describes what the game currently supports based on the source code and authored game data in `src/` and `assets/data/game_data_*.json`.

## Runtime Structure

- Macroquad game loop with `Menu`, `Gameplay`, and `Pause` states.
- JSON-driven content loading through embedded split `game_data_*.json` files.
- Native JSON save/load support with versioned save data.
- Filesystem-backed generated texture loading for world, character, item, UI, and effect art under `assets/generated/`.
- Filesystem-backed generated placeholder audio loading for footsteps, herb pickup, station open, stirring, and brew result cues under `assets/generated/audio/`.

## Core Exploration

- Top-down 8-direction movement with blocker collision.
- Multi-area travel through authored warp regions.
- Surface world layout now routes tower travel through a plains hub, with additional rock, forest, lake, desert, and rainforest branch maps.
- Each non-town surface biome now has a distinct traversal layout and authored blocker language:
  - `north_plains`: open field roads and low-grass clusters
  - `rock_fields`: broken quarry lanes and shelf choke points
  - `moonlit_forest`: winding tree paths, glades, and a charred hollow pocket
  - `lake_shore`: shoreline curves, reed beds, and shallow-water coves
  - `sunscar_desert`: dune lanes, exposed stone scars, and sparse shelter pockets
  - `tropical_rainforest`: layered jungle paths, root arches, and wet clearings
- Camera follow, impact shake, and immediate-mode UI panels/prompts.
- Gameplay HUD now uses:
  - compact vitality/coins and time/day/weather cards
  - a one-line goal tracker with quest detail when relevant
  - a temporary area-name banner on room entry
  - a bottom quick-belt slot bar for potions
  - a hover-expand right drawer for inventory and active effects
- Day clock, day rollover, weather state, season state, and time-of-day windows.
- Rest bed on the tower entry floor with voluntary sleep-to-morning interaction.
- Pause menu save/load actions, with `F5` and `F9` retained as secondary gameplay/pause shortcuts.
- Major overlays now share a stronger panel language with boxed subtitles/footers, framed content sections, consistent tab treatment, and more legible action/selection states across alchemy, shop, archive, rune, and quest screens.
- Late-night exhaustion pressure:
  - HUD time warning turns yellow after midnight
  - hitting 01:00 forces a collapse wake-up at 10:00 back at the tower bed with a full-screen warning flash
- In-world route/station/NPC prompts with contextual requirement or readiness messaging.
- First-run contextual tutorial hint timing for:
  - Crow-led first gather and first brew guidance
  - save/load
  - journal use
  - quest pickup and first delivery
  - early progression from Mira to Rowan to greenhouse restoration
  - potions and route restoration

## World Content

- Procedurally generated background plates for each authored area.
- Surface biome background plates now reflect each map's authored identity instead of sharing one generic blocker pattern.
- Procedurally generated sprites for the player, authored NPCs, stations, gatherables, inventory icons, journal tab icons, and major world effects.
- Surface gather nodes now support biome-specific world-sprite overrides so shared resources can read differently by location.
- World rendering now uses texture-backed stations, characters, gather nodes, warp glows, and feedback effects instead of only primitive placeholder shapes.
- Surface blockers now render with biome-specific procedural prop art instead of a single generic panel treatment.

Current authored areas:

- `tower_entry`
- `north_plains`
- `town_square`
- `rock_fields`
- `moonlit_forest`
- `lake_shore`
- `sunscar_desert`
- `tropical_rainforest`
- `greenhouse_floor`
- `containment_floor`
- `rune_workshop_floor`
- `archive_floor`
- `observatory_floor`

Current authored gathering routes:

- `tower_ruin_edge`
- `plains_crossing`
- `stone_quarry`
- `creekside_meadow`
- `moonlit_grove`
- `charred_hollow`
- `lake_shallows`
- `sunscar_dunes`
- `rainmist_canopy`
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
  - optional biome-specific sprite overrides
  - field journal note capture
- Node availability refreshes by day.
- Gather node silhouettes vary by item type for faster world scanning.
- Surface biome spawn plans now cover Spring, Summer, Autumn, and Winter with biome-native and shared-resource mixes.
- Herb availability rules are learned only after the first successful collection of that herb.
- The journal now distinguishes herb memories that are only `seen` from those fully `learned`.
- Learned herb conditions are surfaced in the journal memory view rather than exposed directly in map prompts.
- Gathered specimens now build herb memory entries with authored flavor summaries, first-seen vs learned state, best quality snapshots, and variant notes.
- Gather feedback includes world bursts and progression/status updates for best-quality results and variant discovery messaging.

Current biome-native surface ingredients:

- `field_bloom`
- `quarry_lichen`
- `reedflower`
- `sunspike`
- `rain_orchid`

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
- Repeat-last-brew setup recall from the alchemy station UI and keyboard flow.
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
- The journal now tracks potion memories separately from archive experiment logs, with `seen`, `learned`, and `successfully brewed` state distinctions plus authored recap text for discovered outputs.

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
  - optional Phase 1 staged dialogue maps
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
  - staged Phase 1 dialogue progression keyed off first brew, first town relief, active requests, quest completion, and greenhouse recovery
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
- Journal tabs surface routes, herb memories, potion memories, greenhouse state, rapport, and milestone/readiness summaries.

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
- herb memories
- potion memories
- legacy field journal entries for backward-compatible save migration
- started/completed quests
- unlocked warps
- planter states
- habitat states
- journal milestones
- NPC relationships

## Audio

- Procedurally generated placeholder one-shots for:
  - tower/surface/greenhouse footsteps
  - herb pickup
  - alchemy station open
  - alchemy stirring
  - brew success and brew collapse
- Footstep placeholders are tuned toward short low-impact thumps with light surface grit rather than broad hiss/noise.
- Audio playback is currently hooked into:
  - player movement
  - successful herb gathering
  - opening the alchemy station
  - stirring at the cauldron
  - brew result resolution

## Current Gaps Relative to the Implemented Codebase

The codebase does not currently show support for:

- combat systems
- enemy AI
- stamina depletion mechanics
- farming beyond greenhouse planters
- creature feeding/care needs beyond habitat placement and timed harvest
- dialogue trees with player choices
- cutscenes or branching endings
