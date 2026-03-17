# Code Review Improvements

- [x] Enforce save compatibility by validating `SaveData.version` against `config.save_version` and adding a migration or explicit rejection path for incompatible saves.
- [x] Persist all intended player state on save/load, especially `vitality`, and explicitly document which `GameplayState` fields are transient versus saved.
- [x] Fix day rollover to preserve excess elapsed time by subtracting `day_length_seconds` instead of resetting the clock directly to `0.0`.
- [x] Stop incrementing NPC relationship values on every interaction key press; award relationship progress from explicit dialogue outcomes or quest events.
- [x] Remove hardcoded content progression rules from gameplay logic by moving warp milestone side effects and lock messaging into data definitions.
- [x] Split the `GameplayState` monolith into smaller subsystems so input, world simulation, overlays, persistence, and interaction logic are isolated.
- [x] Replace stringly-typed gameplay fields like `station.kind`, `effect.kind`, and `item.category` with enums or validated domain types.
- [x] Build indexed lookup tables for areas, items, routes, NPCs, quests, and mutation formulas instead of repeatedly scanning vectors.
- [x] Simplify pause/resume transitions so state ownership moves without temporary placeholder states like `GameState::Menu(MenuState::new())`.
- [x] Improve persistence and input presentation by writing saves atomically to a user-data location and rendering menu control text from the input binding data.

## Status

Mark each item complete by changing `- [ ]` to `- [x]` as the corresponding work lands.

Refactor note: the `GameplayState` split now includes dedicated `world`, `progression`, `runtime`, `ui`, and `alchemy` subsystems so state ownership follows the gameplay concern being edited.
