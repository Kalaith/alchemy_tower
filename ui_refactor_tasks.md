# UI Refactor Tasks

- [x] Create a real `ui` module structure with [src/ui.rs](/H:/RustGames/alchemy_tower/src/ui.rs) as the entry point and focused files under `src/ui/`.
- [x] Move shared frame and panel primitives out of [screen_chrome.rs](/H:/RustGames/alchemy_tower/src/state/screen_chrome.rs) into `src/ui/panels.rs`.
- [x] Move prompt rendering out of the old root [ui.rs](/H:/RustGames/alchemy_tower/src/ui.rs) helper file into [prompts.rs](/H:/RustGames/alchemy_tower/src/ui/prompts.rs).
- [x] Extract reusable button and modal drawing helpers from [menu.rs](/H:/RustGames/alchemy_tower/src/state/menu.rs) and [pause.rs](/H:/RustGames/alchemy_tower/src/state/pause.rs) into [widgets.rs](/H:/RustGames/alchemy_tower/src/ui/widgets.rs).
- [x] Pull HUD rendering out of [gameplay.rs](/H:/RustGames/alchemy_tower/src/state/gameplay.rs) into [hud.rs](/H:/RustGames/alchemy_tower/src/ui/hud.rs).
- [x] Pull world-space prompt rendering out of [gameplay.rs](/H:/RustGames/alchemy_tower/src/state/gameplay.rs) into [world_prompts.rs](/H:/RustGames/alchemy_tower/src/ui/world_prompts.rs).
- [x] Move overlay drawing entry points from the old [gameplay_overlays.rs](/H:/RustGames/alchemy_tower/src/state/gameplay_overlays.rs) location into [overlays.rs](/H:/RustGames/alchemy_tower/src/ui/overlays.rs).
- [x] Move low-level text wrapping, badges, cards, and helper drawing from [gameplay_support.rs](/H:/RustGames/alchemy_tower/src/state/gameplay_support.rs) into [text.rs](/H:/RustGames/alchemy_tower/src/ui/text.rs) and [widgets.rs](/H:/RustGames/alchemy_tower/src/ui/widgets.rs).
- [x] Reduce `GameplayState` rendering methods so gameplay code prepares view data and the `ui` layer handles presentation.
- [x] Reuse the existing `src/ui/` folder by populating it with the new UI modules and removing duplicated rendering helpers from state files.
- [x] Run `cargo check` and `cargo test`, then mark each task complete as the UI refactor lands.
