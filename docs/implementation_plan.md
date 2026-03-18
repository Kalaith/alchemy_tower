# Alchemy Tower: Implementation Plan

## Goal
Build a 2D top-down Rust game inspired by Stardew Valley where the player restores a wizard's alchemy tower, explores a nearby town and surrounding wilderness, gathers ingredients, brews potions, and unlocks deeper magical systems over time.

## Project Direction

### Core Pillars
- Cozy daily routine with light schedule pressure, not harsh survival.
- Strong home-base progression through tower restoration.
- Exploration loop across town, forest, marsh, caves, and special magic biomes.
- Experiment-driven alchemy with discovery, side effects, and recipe mastery.
- Mystery narrative delivered through journals, townsfolk, and tower unlocks.

### Scope Guardrails
- Start with one polished tower floor, one town, and two overworld gathering zones.
- Prefer hand-authored maps over procedural generation.
- Keep combat optional or light in the first playable version.
- Delay breeding, automation, and advanced mutation systems until the base loop is fun.

### Technical Direction
- Engine: `macroquad`
- Shared local pattern: use the state-machine and project layout from `GAME_DEVELOPMENT_GUIDE.md`
- Rendering style: tile-based top-down 2D with simple sprite layers and immediate-mode UI
- Persistence: JSON save files first, versioned from the start
- Target platforms: Windows native first, Web export after controls and save behavior are stable

---

## Recommended Architecture

### Initial Project Structure
```text
alchemy_tower/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── game.rs
│   ├── state/
│   │   ├── mod.rs
│   │   ├── menu.rs
│   │   ├── gameplay.rs
│   │   └── pause.rs
│   ├── world/
│   │   ├── mod.rs
│   │   ├── map.rs
│   │   ├── area.rs
│   │   ├── collision.rs
│   │   └── travel.rs
│   ├── player/
│   │   ├── mod.rs
│   │   ├── movement.rs
│   │   ├── inventory.rs
│   │   └── stamina.rs
│   ├── tower/
│   │   ├── mod.rs
│   │   ├── rooms.rs
│   │   ├── upgrades.rs
│   │   └── stations.rs
│   ├── alchemy/
│   │   ├── mod.rs
│   │   ├── ingredients.rs
│   │   ├── recipes.rs
│   │   ├── brewing.rs
│   │   └── effects.rs
│   ├── farming/
│   │   ├── mod.rs
│   │   ├── crops.rs
│   │   └── growth.rs
│   ├── creatures/
│   │   ├── mod.rs
│   │   ├── entities.rs
│   │   └── harvesting.rs
│   ├── npc/
│   │   ├── mod.rs
│   │   ├── schedules.rs
│   │   ├── dialogue.rs
│   │   └── shops.rs
│   ├── quests/
│   │   ├── mod.rs
│   │   └── journal.rs
│   ├── data/
│   │   ├── mod.rs
│   │   ├── definitions.rs
│   │   └── loader.rs
│   ├── save/
│   │   ├── mod.rs
│   │   └── save_data.rs
│   └── ui/
│       ├── mod.rs
│       ├── hud.rs
│       ├── menus.rs
│       └── dialogue_box.rs
└── assets/
    ├── maps/
    ├── data/
    ├── sprites/
    └── audio/
```

### High-Level Runtime Model
- `Game` owns asset caches, save/profile data, and the active state.
- `GameplayState` owns the simulation snapshot for one active save.
- The world is divided into authored areas: `Tower`, `Town`, `Forest`, `Marsh`, `Caves`.
- Each area updates only the entities relevant to the current screen plus lightweight background timers.
- Interactions produce intents first; systems validate and apply changes to state second.

### Data-Driven Content
- Put item, recipe, crop, NPC, room, and dialogue definitions in JSON.
- Keep authored maps in JSON or Tiled-exported data, but do not build custom tooling initially.
- Use stable string IDs everywhere: `moon_fern`, `entry_lab`, `mayor_elric`, `healing_draught`.

---

## Gameplay Slice Order

### Vertical Slice Target
The first meaningful playable build should support:
- Walking around the tower, town square, forest path, and a small cave.
- Gathering herbs, mushrooms, ore, and bug ingredients.
- Talking to a few NPCs and selling items.
- Brewing 3 to 5 basic potions in the entry laboratory.
- Spending resources to repair and unlock the greenhouse floor.
- Reading early journals that establish the mystery.

If this slice is fun, expand breadth from there.

---

## Phased Plan

## Phase 1: Foundation
Establish the engine shell and the lowest-risk game loop.

### Deliverables
- Create the Rust crate and Macroquad entry point.
- Implement game state machine: menu, gameplay, pause.
- Add camera, tile map rendering, collision, and player movement.
- Add asset loading and placeholder fallback rendering.
- Add JSON save/load with versioning.

### Output
- Playable empty prototype where the player can move between test maps.

### Notes
- Reuse the simple top-down movement approach proven in `scrapyard/src/ship/player.rs`.
- Keep movement and collision deterministic and boring before adding content.

## Phase 2: World Navigation
Build the core explorable world structure.

### Deliverables
- Author a connected overworld covering the tower exterior/interior ground floor, town square, forest gathering zone, and cave gathering zone.
- Add screen transitions or edge-based area travel.
- Add interactables: doors, forage nodes, resource rocks, herb patches, containers.
- Add day clock and simple weather state without seasonal complexity yet.

### Output
- Player can complete a full exploration loop and return home with gathered items.

## Phase 3: Inventory and Gathering
Support item acquisition, tool use, and selling.

### Deliverables
- Inventory grid with stacking, categories, and item tooltips.
- Gatherable definitions with respawn rules and drop tables.
- Basic tools for hand gathering, plant cutting, ore mining, and optional bug capture.
- Vendor UI for buying starter supplies and selling ingredients/potions.
- Stamina costs for gathering and movement tuning.

### Output
- A simple day can now be played as gather, return, sell, rest.

## Phase 4: Alchemy Core
Implement the main fantasy of the game.

### Deliverables
- Brewing station interaction in the entry lab.
- Ingredient slots, heat/time or stirring variables, and recipe outcome resolution.
- Recipe knowledge model covering known recipes, unstable discoveries, failed concoctions, consumable effects, and sell values.
- Side-effect system for unstable brews.
- Recipe journal UI.

### Output
- Core loop becomes gather, brew, test, sell, unlock.

### Design Constraint
- Keep the first version readable. Use fixed recipes plus one or two controlled wildcard combinations instead of a huge combinatorial system.

## Phase 5: Tower Progression
Turn the tower into the main progression tree.

### Deliverables
- Tower room unlock data and repair costs.
- Floor 1 Entry Laboratory fully interactive.
- Floor 2 Greenhouse unlock with planting plots and magical crops.
- Room state persistence across days.
- Journal fragments and repair milestones tied to room unlocks.

### Output
- The player sees meaningful home-base growth and a reason to keep exploring.

## Phase 6: Town and Quest Layer
Make the world feel inhabited and directed.

### Deliverables
- 5 to 8 key NPCs with schedules and simple relationship state.
- Quest board and direct requests from townsfolk.
- Shopkeeper, herbalist rival, mayor, and archivist roles.
- Dialogue conditions based on progress, weather, and tower state.
- Orders that pull the player into specific ingredients and potion crafting goals.

### Output
- The player has structure beyond self-directed gathering.

## Phase 7: Greenhouse and Ingredient Cultivation
Expand into managed production.

### Deliverables
- Planting, watering or magical tending, growth stages, and harvest yields.
- Mutation rules for selected magical crops.
- Soil/planter upgrades and greenhouse expansion.
- Crop use in brewing recipes and quests.

### Output
- The player can shift from pure foraging to curated ingredient production.

## Phase 8: Creature Containment
Add the first advanced tower system.

### Deliverables
- Capture rules for a small set of magical creatures.
- Creature habitat objects inside the containment floor.
- Renewable harvesting of venom, scales, dust, or essence.
- Minimal care loop: feed, calm, harvest.

### Output
- Late-midgame ingredients come from managed systems rather than only map respawns.

## Phase 9: Rune Workshop
Introduce build variation and deeper crafting.

### Deliverables
- Rune item definitions and socketing rules.
- Potion modifiers such as duration, duplication, splash, delayed trigger.
- Basic enchantable tools or utility items.
- Balance pass to prevent rune stacking from breaking progression.

### Output
- Brewing evolves from fixed recipes to player-shaped builds.

## Phase 10: Mystery, Archives, and Endgame
Deliver the narrative payoff and late systems.

### Deliverables
- Archive puzzles, journal timeline, and revelation scenes.
- Time-locked or condition-locked lore delivery.
- Final choice or ending sequence tied to the wizard's fate.
- Optional weird rooms for post-core content such as the portal observatory, homunculus lab, weather engine, and philosopher engine.

### Output
- The game has a complete arc rather than endless sandbox only.

---

## System Design Priorities

### Player Controller
- 8-direction top-down movement.
- Collision against tile blockers and large props.
- Interaction cone or short-radius prompt.
- Separate movement tuning for indoors and outdoors if needed.

### World Maps
- Use authored maps with separate ground, blocker, decoration, interaction-marker, and spawn-marker layers.
- Keep area size modest so traversal stays cozy.

### Time and Day Loop
- Day length around 10 to 15 real-time minutes for early tests.
- Sleep at tower bed advances day and processes crop growth, restocks, and NPC schedule reset.
- Do not add seasons until the single-day loop is satisfying.

### Economy
- Ingredients have low sell value.
- Potions create the main value multiplier.
- Tower unlocks consume a separate blend of coin plus rare materials.
- Avoid infinite money loops from greenhouse automation early.

### Alchemy Resolution
- Start with a deterministic recipe table.
- Add a small trait system on ingredients later: `healing`, `volatile`, `cold`, `luminous`, `toxic`.
- Use trait interactions to support hidden discoveries without making outcomes impossible to reason about.

### NPC Simulation
- Use simple waypoint schedules, not fully dynamic AI.
- Important NPCs need only morning/day/evening locations at first.
- Dialogue should react to tower unlock milestones and recent potion deliveries.

### Narrative Delivery
- Journals are unlocked by tower progression.
- Town gossip hints at off-screen history.
- Strange environmental details inside the tower escalate over time.

---

## Asset Strategy

### Start With Placeholders
- Colored rectangles and icon glyphs for items/systems.
- One readable tileset for outdoor terrain.
- One readable tileset for tower interior.
- Basic portrait placeholders for dialogue if needed.

### Art Priorities
1. Player sprite and movement readability
2. Tile readability for collision and gathering nodes
3. Distinct tower room props
4. Item icons for ingredients and potions
5. NPC silhouettes

### Audio Priorities
- Footsteps by surface type later, not immediately
- Gather, brew, unlock, and dialogue UI sounds first
- One ambient loop per main area

---

## Testing Plan

### Continuous Checks
- Verify save/load after every new persistent system.
- Verify every tower unlock preserves old saves.
- Test collision at doors, corners, and map transitions.
- Test that each day loop can be completed without soft-locking inventory or stamina.

### Playtest Questions
- Is gathering relaxing or tedious?
- Is brewing understandable before reading external notes?
- Does the tower feel like the center of progression?
- Does the town create purpose or just extra walking?
- Do unlock costs create momentum or grind?

### Tooling
- Add debug overlays for current area, collision bounds, time of day, interact target, recipe resolution details, and save version.

---

## Risks and Mitigations

### Risk: Scope Explosion
- Mitigation: ship the vertical slice before adding creatures, runes, or automation.

### Risk: Alchemy System Becomes Opaque
- Mitigation: start deterministic and expose partial clues through item traits and journals.

### Risk: World Feels Empty
- Mitigation: prioritize a small number of strong NPC routines and dense interactables over a large map.

### Risk: Too Much Content Hardcoded
- Mitigation: move item, recipe, NPC, and room definitions into JSON early.

### Risk: Web Build Complexity
- Mitigation: stabilize native build first; defer web save quirks and asset loading edge cases.

---

## Milestone Roadmap

### Milestone A: Movement Prototype
- Player moves around a tower room and one outdoor map.

### Milestone B: Gather and Return
- Inventory works, nodes drop ingredients, selling works.

### Milestone C: First Potion
- Brewing station creates usable and sellable potions.

### Milestone D: First Upgrade
- Greenhouse floor unlocks and persists in save data.

### Milestone E: First Town Loop
- NPC request asks for a potion, player fulfills it, story advances.

### Milestone F: Vertical Slice Complete
- One satisfying day loop with tower, town, wilderness, brewing, selling, and unlock progression.

---

## Suggested Build Order for the Actual Repo
1. Create the crate and base `src/` structure from `GAME_DEVELOPMENT_GUIDE.md`.
2. Implement movement, camera, collision, and area transitions.
3. Add inventory and gatherables.
4. Add alchemy station and basic recipes.
5. Add sell/shop UI and economy.
6. Add tower unlock persistence.
7. Add town NPCs and quest requests.
8. Expand content only after the vertical slice is fun.

## Definition of "Good First Playable"
- 15 to 20 minutes of coherent play.
- At least 20 ingredients, 5 potions, 4 NPCs, 4 explorable areas.
- One tower unlock.
- One narrative reveal.
- No placeholder-only blocker for understanding the loop.
