# Alchemy Tower

`Alchemy Tower` is a cozy top-down exploration and brewing game built with Rust and Macroquad.

You gather ingredients around the valley, learn where and when they appear, brew potions through a small process-driven alchemy system, help the nearby town, and gradually restore new tower floors. The current game is centered on discovery, recipe learning, tower restoration, and town requests rather than combat.

## Current State

The playable build currently includes:

- Top-down exploration across the tower, town, and multiple surface biomes.
- Time of day, weather, seasons, gathering conditions, and day rollover.
- Journal tracking for routes, herb memories, potion memories, greenhouse state, rapport, and milestones.
- Station-based alchemy with ingredients, catalysts, heat, stirring, timing, salvage outcomes, mastery, and morphs.
- Potion use, active effects, inventory sorting, shop buy/sell flow, and a 3-slot quick potion belt.
- Tower progression through the greenhouse, containment floor, rune workshop, archives, and observatory.
- NPC schedules, quests, rapport, town turn-ins, and board quests.
- Save/load support, pause menu actions, generated placeholder art, and generated placeholder audio.

For the full current shipped feature list, use [IMPLEMENTED_SYSTEMS.md](/H:/WebHatchery/RustGames/alchemy_tower/IMPLEMENTED_SYSTEMS.md).

## Run Locally

### Requirements

- Rust toolchain with `cargo`
- A sibling checkout of `macroquad-toolkit` at `../macroquad-toolkit`

This repo depends on a local path dependency in `Cargo.toml`, so the expected workspace layout is:

```text
H:\WebHatchery\RustGames\
  alchemy_tower\
  macroquad-toolkit\
```

### Start The Game

```powershell
cargo run
```

The game opens in a `1280x720` resizable window.

## Controls

- `WASD`: move
- `E`: interact
- `Tab`: open alchemy when available
- `J`: open journal
- `Z`, `X`, `C`: quick potions
- `Esc`: pause / back
- `F5`: save
- `F9`: load

## Project Docs

- [IMPLEMENTED_SYSTEMS.md](/H:/WebHatchery/RustGames/alchemy_tower/IMPLEMENTED_SYSTEMS.md): current shipped systems and content
- [polish_backlog.md](/H:/WebHatchery/RustGames/alchemy_tower/polish_backlog.md): active polish roadmap
- [tasks.md](/H:/WebHatchery/RustGames/alchemy_tower/tasks.md): open product and design follow-ups
