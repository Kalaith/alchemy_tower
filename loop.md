# Alchemy Tower — Content Depth Loop

Run with the `/loop` skill, e.g. `/loop Read loop.md in this directory and run exactly one iteration of it.`
(Add an interval like `/loop 45m ...` only if you want a wall-clock cadence; otherwise let it self-pace.)

---

## Mission

Deepen Alchemy Tower as a **game world**, not as an engine. The systems already exist —
brewing with elements/traits/quality/overcharge, gathering with season/weather/time conditions
and wild variants, NPC rapport with friendship gifts, quests with quality/trait gates,
repeatable board requests, warp gating, journal/archive. What the game is short on is
**content authored into those systems**: places to go, things to pick, people to know,
formulae to find, and reasons to care.

Every iteration must leave the game with **more world**, and must leave it playable.

## Prime directive: content depth over new systems

- Default to **data-only work** in `assets/data/*.json`. That is the win condition, not a fallback.
- Touching Rust is allowed when — and only when — the content *needs* it:
  - a small `#[serde(default)]` field on an existing schema struct so authored content can express
    something the schema can't yet say,
  - a surface that shows authored content the player currently can't see (a journal line, a hint,
    a preview row),
  - a bug the new content exposes.
- **Do not** add a new subsystem, a new overlay, a new resource, or a new verb. If an idea needs one,
  write it into the Deferred list at the bottom and pick something else.
- Breadth without texture is not depth. Ten flavourless herbs is worse than three herbs that each
  change where you go, when you go there, and what you can brew when you get back.

## Every piece of content must earn its place

Before authoring anything, answer these in one line each in your iteration notes. If you can't,
pick different content.

1. **What does this connect to that already exists?** (a recipe, a biome, an NPC, a quest chain)
2. **What decision does it create?** (a route worth walking at night, an ingredient worth saving,
   a request worth turning down)
3. **How does the player find out it exists?** (a gather node they'll walk past, an NPC line,
   a journal hint, a recipe that names it)

Content that has no inbound reference is invisible content. Wire it both ways.

---

## Where the depth is thin (current state, 2026-08-01)

Counts, for calibration — re-check them each iteration rather than trusting this list:

| Axis | Now | Notes |
|---|---|---|
| Ingredients | **13** | vs 33 potions. The gathering half of the loop is starved. |
| Wild variants | sparse | The conditions system is authored for, but barely used. |
| Gather nodes | 0–5 per area | `tower_entry`, `town_square` and 4 tower floors have **none**. |
| Areas | 13 | 7 wild + 5 tower floors + town. Several are one-note. |
| Gathering routes | 13 | One per area — routes could subdivide a biome. |
| NPCs | 7 | 6 townsfolk + the Crow. No minor/seasonal/visiting characters. |
| Quests | 10 | 5 NPC + 4 board + 1 gate. No multi-step chains. |
| Recipes | 20 (+6 morph, 3 rune, 3 mutation) | Good spread; morph paths are the thin part. |
| Narrative text | `narrative_text.json` is 4.5KB | The smallest data file in the game. |

### Standing themes worth several iterations each

- **Signature biome hooks** — give each wild area one thing only it does (a night-only bloom, a
  post-rain spawn, a variant that needs two conditions at once), so the season/weather/time system
  is felt rather than merely present.
- **Ingredient families** — herbs that share a trait or element and pull into overlapping recipes,
  so inventory choices bite.
- **Tower floors as places** — greenhouse, containment, rune workshop, archive and observatory are
  gates and stations more than they are rooms. Give them gatherables, incidents, and things to read.
- **NPC three-beat arcs** (setup → complication → payoff) tied to rapport, per `TODO.md`.
- **Quest chains** — the failing-harvest and pollinator-collapse chains named in `TODO.md`.
- **Board request variety** — more repeatable requests with different quality/trait shapes so the
  mid-game has texture between story beats.
- **Morph and discovery paths** — more precision-reward branches, and journal beats that celebrate
  a discovery instead of silently logging it.
- **Voice** — `narrative_text.json`, `ui_text.json` flavour, herb `description`/`source_conditions`,
  gather-node `note`. Prose is content.

---

## One iteration

### 1. Orient (cheap)
- `git -C . status` and `git log --oneline -8` — know what the last iteration did.
- Read the **Ledger** at the bottom of this file. Do not repeat the last two iterations' axis.
- Re-count the thin axes above from the JSON before trusting the table.

### 2. Choose one slice
Pick **one** coherent slice, sized to finish and verify in a single iteration. Good sizes:
- 2–4 new ingredients that share a family, with their gather nodes, variants and one recipe that uses them;
- one biome's signature hook, end to end;
- one NPC's three-beat arc plus the dialogue and journal beats that carry it;
- one quest chain of 2–3 linked steps;
- one tower floor turned into a place you'd visit for its own sake.

Rotate axes across iterations — ingredients, world, NPCs, quests, recipes, prose. Don't grind one.

### 3. Author it, data-first
Write the content into `assets/data/`. Match the existing entries' shape and voice exactly —
read three neighbours before adding a fourth. Put it in the right file (see **Where content
lives** below), and if that file crosses ~800 lines, split it and update
`src/data/loader_embedded.rs` before adding more.

### 4. Wire it in
New content must be reachable and referenced. Typically: a gather node in an area, a recipe that
names the ingredient, an NPC line or request that wants the potion, a journal hint that points at it.

### 5. Verify
- `cargo fmt -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- Add or extend a test when the content encodes a rule worth protecting (a gate condition, a
  uniqueness constraint, a chain prerequisite).
- Visual check via the capture harness when the slice changes something on screen —
  scenes: `brew`, `board`, `journal`, `dialogue:<npc_id>`. See `scripts/capture_ui.ps1`.
- Then run `.\publish.ps1` from this directory. That is the sanctioned end-to-end validation path
  per `AGENTS.md`; report honestly if it fails.

### 6. Commit
One commit per iteration, in the catalog's style: subject narrates the change in the game's own
voice and ends with a plain-terms parenthetical tag; body is honest prose — problem, change,
reasoning. No `feat:`/`fix:` prefixes. See `rust_management/docs/COMMIT_STYLE.md`.

### 7. Log it
Append one line to the **Ledger** below: date, axis, what landed, and anything the next iteration
should know. Move anything you deliberately skipped into **Deferred**. Keep both lists tight —
this file is the loop's memory.

---

## Where content lives

Data files follow the same ~800-line rule as `.rs` files. `src/data/loader_embedded.rs` holds the
`include_str!` tables that stitch them back together — **adding a file means adding a line there**.

| Content | File |
|---|---|
| An area's blockers, warps, gather nodes | `assets/data/world/areas/<area_id>.json` (one area per file) |
| Gathering routes | `assets/data/world/gathering_routes.json` |
| Stations | `assets/data/world/stations.json` |
| Ingredients | `assets/data/items/ingredients_<biome>.json` — filed under the biome that anchors the herb; `ingredients_shared.json` for herbs found in 3+ areas or produced rather than gathered |
| Potions | `assets/data/items/potions_<effect>.json` — filed under the effect kind the potion leads with; `potions_unstable.json` for salvage outputs |
| Creatures, catalysts, runes | `assets/data/items/materials.json` |
| Recipes and their morph targets | `assets/data/crafting/recipes_<effect>.json` — filed under the effect kind the output potion leads with (`restore`, `glow`, `speed`) |
| Rune recipes, mutation formulas | `assets/data/crafting/rune_recipes.json`, `crafting/mutation_formulas.json` |
| NPCs and quests | `assets/data/game_data_npcs.json` — split npcs/quests apart when it crosses 800 lines |
| Art requirements | `assets/data/sprites/<section>.json` (`gatherables`, `item_icons`, `npcs`, `stations`, `areas`, `ui_and_effects`), read by `tools/generate_art.py` |

When you split a file: move entries, add the new source to the right table in
`loader_embedded.rs`, and confirm the counts survive (`cargo test` covers the references).

## Hard constraints (violating these breaks the build)

- **New item id ⇒ new icon, or the art manifest test fails.** Add the id to
  `assets/data/sprites/gatherables.json` (herbs and world nodes) or `sprites/item_icons.json`
  (potions), add a themed hex to the colour dict in `tools/generate_art.py` `icon()`, then run
  `python tools/generate_art.py`.
  Same idea for a new area (`assets/generated/areas/<id>.png`) or NPC
  (`assets/generated/characters/<id>.png`). `art::asset_manifest::tests` is the gate.
- **Recipe ingredient multisets must be unique per station** — the matcher keys on exact
  ingredient counts. Check before adding.
- **`starter_known: true` belongs only on the three entry basics.** Never flag a new recipe;
  discovery is the design.
- **Keep brewing deterministic.** No RNG in resolution — risk is shown to the player
  (see the instability line) rather than rolled behind their back.
- **800-line limit on every `.rs` file**, non-test lines. No new `mod.rs`.
- **New save fields must be `#[serde(default)]`** and wired through snapshot/restore, so existing
  saves keep loading. Mirror the `relationships` / `board_quest_cooldowns` pattern.
- **Never write anything under `D:\xampp\htdocs`** — it is a publish target only.
- `assets/data/game_data_fallback.json` and the embedded-JSON path exist for WASM; if you add a
  data file, check `src/data/embedded_json.rs`.

## Reference

- `docs/alchemy_system_design.md` — live spec for elements, traits, quality bands, mastery, morphs.
- `TODO.md` — the standing backlog this loop is chewing through. Tick items off as they land.
- `AGENTS.md`, `CODE_STANDARDS.md` — project rules.
- `README.md` — the game's stated premise; content should serve it.

## Stop conditions

Stop the loop and report if:
- `publish.ps1` fails twice for the same reason;
- an axis genuinely runs out of good content and the remaining ideas all need new systems;
- the same slice fails verification twice.

---

## Ledger

<!-- One line per iteration. Newest at the bottom. -->

- **2026-08-01 — world / ingredients.** Split `game_data_world.json` (2717 lines) into
  `world/areas/*.json` + routes + stations, and `game_data_items.json` (1565) into per-biome
  ingredient files + materials + potions; `loader_embedded.rs` now stitches them. Then gave
  `sunscar_desert` its signature hook — three shift-locked ingredients (`nightglass_bloom` night,
  `scorchvine_resin` noon, `saltmirror_flake` the morning after rain), a second route
  (`sunscar_saltpan`), two recipes (`sunscar_glass_elixir` with a `nightglass_lantern` starlight
  morph, `mirrorsalt_draught` pulling desert+lake), and a repeatable board request. Added four
  content-integrity tests to `game_data.rs` — they now guard every later iteration.
  *Next: don't touch the desert or ingredients; rotate to NPCs, quests or the tower floors.*
- **2026-08-01 — quests / NPCs.** Built the failing-harvest chain as Rowan's three-beat arc
  (`glow_for_rowan` → `harvest_blight_for_rowan` → `harvest_recovery_for_rowan`), which needed four
  optional schema fields: `NpcDefinition.quest_ids` (an ordered arc; only the first unfinished step
  is ever offered), `QuestDefinition.giver_intro_line`/`giver_active_line` (per-beat voice), and
  `GatherNodeDefinition.required_completed_quest`. That last one buys the visible town-state change
  `TODO.md` asked for: finishing the arc turns the bed rows behind the square, and `town_square`
  grows whisper moss and field bloom for the first time (it had zero gather nodes). Also split
  `game_data_crafting.json` (989) into `crafting/recipes_<effect>.json` and `sprite_requirements.json`
  (1262) into `sprites/<section>.json`; fixed raw quest ids leaking into the locked-request line.
  Capture harness gained `dialogue:<npc>:<beat>` for seeing mid-arc conversations.
  *Next: five townsfolk still have single one-shot requests — Mira, Brin, Elric, Ione and Lyra all
  want arcs, and the pollinator-collapse chain is still unwritten. Or rotate to the tower floors,
  which remain gates rather than places.*
- **2026-08-01 — tower floors.** `containment_ring`, `archive_stack` and `observatory_span` were
  routes with nothing on them; the containment floor now fills the first. Three tower-native
  gatherables that exist nowhere in the valley — `wardglass_frost` (the ward cold cycle, dusk to
  first light), `quietbloom_spore` (under the cells, after the lamps drop), `resonance_shard` (shed
  by tuned ward frames, gated on `containment_for_lyra`) — all rarity 3 and high quality, so the
  climb pays. Two recipes at the underused `greenhouse_still` (`wardfrost_tonic` is the first brew
  the tower supplies end to end; `resonance_draught` ties ward shard to habitat lantern dust) and a
  repeatable Excellent-band board order. Split `items/potions.json` (832) by effect kind. Capture
  harness gained `area:<area_id>[:<day>]` — the first way to look at a room at all.
  *Gotcha found: a node's `spawn_chance` interacts with the per-day roll, so 64 meant the floor was
  bare several days running. Raised to 80–92; a floor you climb to deliberately should not be empty.
  New test `every_gather_node_can_actually_spawn` sweeps 60 days × 4 windows and fails any node whose
  conditions can never all be true.*
  *Next: `archive_stack` and `observatory_span` are still empty routes, the rune workshop is still a
  gate, and five townsfolk still have one-shot requests.*

## Deferred (needs a new system; not for this loop)

- Apply-potion-to-target flow (wilted plant, frightened creature, blocked path) — `TODO.md` calls it
  out as the unexpressed premise, but it is a new verb, not content.
- World/character art pass and hand-authored ambient audio.
- Typed overlay state model unifying archive/journal/pause/dialogue/alchemy.
- Post-ending sandbox, if the scope question resolves toward a long tail.
