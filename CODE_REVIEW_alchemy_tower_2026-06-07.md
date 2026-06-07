# Code Review: alchemy_tower

Date: 2026-06-07

Scope: Fresh review of `alchemy_tower` using the synced local `AGENTS.md`, `CODE_STANDARDS.md`, `MACROQUAD_TOOLKIT.md`, and `GAME_DEVELOPMENT_GUIDE.md`.

## Findings

### High: WebGL save/load is still stubbed out

- `src/save/save_wasm.rs:6` returns `SAVE_ERROR_WASM_SAVE_UNAVAILABLE`.
- `src/save/save_wasm.rs:10` always reports no save exists.
- `src/save/save_wasm.rs:14` returns `SAVE_ERROR_WASM_LOAD_UNAVAILABLE`.

The project presents save/load UI and publishes a WebGL build, but browser players cannot persist progress. The WebGL release warnings for unused `save_codec` functions are another symptom: `src/save/save_data.rs:12` always includes `save_codec`, but the wasm platform module never calls `src/save/save_codec.rs:3` or `src/save/save_codec.rs:7`.

Recommended fix: implement wasm persistence using the shared toolkit or browser local storage bindings, and reuse the existing JSON codec so native and WebGL saves share one format.

### Medium: The project builds with compiler warnings, which conflicts with the no-unused-code standard

`cargo check` passes but emits 19 warnings. `cargo test` passes but emits 18 warnings. `publish.ps1` passes, but the native release emits 19 warnings and the WebGL release emits 21 warnings.

Warnings to fix:

- `src/data/schema.rs:21`: unused `RecipeIngredient` re-export.
- `src/data/schema.rs:25`: unused `WildVariantDefinition` re-export.
- `src/data/schema.rs:27`: unused `CrowPhase1DialogueDefinition`, `NpcPhase1DialogueDefinition`, and `NpcScheduleEntry` re-exports.
- `src/data/schema.rs:33`: unused `ShopStockDefinition` re-export.
- `src/data.rs:12`: unused `load_embedded` re-export in the normal bin build.
- `src/state/gameplay_npc_motion.rs:1`: unused `TravelSegment` import.
- `src/ui/hud.rs:3`: unused `HudGoal` import.
- `src/ui/hud_header.rs:4`: unused `super::hud_chrome::*` import.
- `src/ui/hud_status_goal.rs:5`: unused `draw_wrapped_text` import.
- `src/ui/hud_status_time.rs:2`: unused `super::hud_compass::*` import.
- `src/ui/world_entity_markers.rs:5`: unused `draw_unlock_ready_warp_glow` re-export.
- `src/ui.rs:75`: unused `draw_world_marker_plate` re-export.
- `src/state/gameplay_station_prompt.rs:24`: unreachable `_` match arm.
- `src/state/gameplay_world_prompt.rs:58`: unused `station` binding.
- `src/state/gameplay_overlay_state.rs:32`: unused `archive_tab_selected`.
- `src/state/gameplay_overlay_state.rs:44`: unused `shop_sell_tab_active`.
- `src/state/gameplay_overlay_state.rs:64`: unused `journal_tab_selected`.
- `src/state/gameplay_alchemy_input_text.rs:3`, `:11`, `:15`, `:19`: dead helper functions in one compiled copy of this text module.
- WebGL release only: `src/save/save_codec.rs:3` and `src/save/save_codec.rs:7` are dead because wasm save/load does not use the codec.

Most of these are safe cleanup. The alchemy text warning needs a little care: `src/state/gameplay_alchemy_input.rs:14` and `src/state/gameplay_alchemy_mouse_input.rs:11` both include `gameplay_alchemy_input_text.rs`, creating separate module instances. Hoist that text module once under `gameplay.rs`, then import it from both input modules.

### Medium: Embedded data and asset failures are still too easy to miss

- `src/data/loader.rs:9` falls back to `GameData::runtime_fallback()` after embedded data load failure.
- `src/art/assets.rs:24` discards asset-pack load errors.
- `src/art/assets.rs:27` ignores how many expected textures loaded.
- `src/audio_loading.rs:7` and `src/audio_loading.rs:36` discard audio load failures.

The current authored texture paths exist, but future data or asset regressions could pass publish while shipping fallback content, invisible textures, or missing audio. The local standards call for clear error handling for asset loading and publishing.

Recommended fix: make publish/release builds fail on invalid embedded data and missing required assets. If runtime fallback remains useful during development, gate it explicitly behind a dev-only path or visible diagnostic state.

### Low: Some progression and presentation rules remain hardcoded instead of data-driven

Examples:

- `src/audio.rs:39`: area IDs are hardcoded to footstep sound sets.
- `src/state/gameplay_archive_progress.rs:18`: archive reconstruction gates are hardcoded to quest and milestone IDs.
- `src/state/gameplay_dialogue.rs:79`: quest completion milestones are hardcoded by quest ID.
- `src/ui/world_story_flourishes.rs:12`: area IDs and flourish placements are hardcoded in rendering.

These are not immediate correctness bugs, but they work against `CODE_STANDARDS.md` section 1.3 and section 5.3. New areas, quests, and milestones still require Rust edits when they should mostly be authored data changes.

## Checks Run

- `..\docs\check-project-docs.ps1 -ProjectRoot alchemy_tower`: passed.
- `rg --files -g mod.rs`: no `mod.rs` files found.
- Rust file size check: largest file is `src/state/gameplay.rs` at 338 lines; no `.rs` file is at or above 800 lines.
- `cargo check`: passed with 19 warnings.
- `cargo test`: passed, 12 tests passed, with 18 warnings.
- `.\publish.ps1`: passed, packaged Windows and WebGL builds, deployed preview to `D:\xampp\htdocs\games\alchemy_tower`; native release emitted 19 warnings and WebGL release emitted 21 warnings.

## Residual Risk

I did not run an interactive gameplay session. The main remaining risk is that compiler warnings are currently tolerated by the publish path, so unused/dead code can drift further unless warning cleanup becomes part of the validation checklist.
