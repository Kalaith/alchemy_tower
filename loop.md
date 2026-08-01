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
| Stations | `assets/data/world/stations/<area_id>.json` (one file per room, like the areas) |
| Ingredients | `assets/data/items/ingredients_<biome>.json` — filed under the biome that anchors the herb; `ingredients_shared.json` for herbs found in 3+ areas or produced rather than gathered |
| Potions | `assets/data/items/potions_<effect>.json` — filed under the effect kind the potion leads with; `potions_unstable.json` for salvage outputs |
| Creatures, catalysts, runes | `assets/data/items/materials.json` |
| Recipes and their morph targets | `assets/data/crafting/recipes_<effect>[_<station>].json` — filed under the effect kind the output potion leads with (`restore`, `glow`, `speed`) |
| Rune recipes, mutation formulas | `assets/data/crafting/rune_recipes.json`, `crafting/mutation_formulas.json` |
| NPCs | `assets/data/town/npcs.json` |
| Quests | `assets/data/town/quests_arcs.json` for a townsperson's own arc, `quests_board.json` for request-board orders |
| Art requirements | `assets/data/sprites/<section>.json` (`gatherables`, `gatherable_variants`, `item_icons`, `npcs`, `stations`, `areas`, `ui_and_effects`), read by `tools/generate_art.py` |

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
- **Station positions need a capture, not arithmetic.** The HUD occupies every edge of the
  screen: vitality and coins top-left, the area plaque top-centre, clock and minimap top-right,
  the goal note down the left, the bag down the right, the potion belt across the bottom. A
  station near a room's edge disappears behind one of them. This has happened three times now —
  the cold bench, the ward-cooled bed and the trader's stall — so place stations in the middle
  band and then look at them.
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
- **2026-08-01 — recipes / morphs.** The catalyst system was fully built and 75% unused: every
  catalyst-gated morph in the game keyed on `starlight`, the only catalyst that existed. Added
  `kiln_geode` (tag `kilnfire`, quarry cuts, summer/autumn middays) and `stillwater_pearl` (tag
  `stillwater`, the dead water behind the reed bar, misty spring/autumn mornings) — both *gathered*
  under hard conditions, where starlight is *bought*, so which branch you take costs differently.
  Five new morph branches: `healing_draught_recipe` now morphs (the starter recipe teaches the whole
  layer), and `verdant_restorative` forks two ways on catalyst. Morph count 7 → 12.
  *Two bugs this exposed: `morph_trigger_hint` only ever described `morph_targets.first()`, so a
  second branch was invisible at the bench — it now hints at whichever branch the current setup is
  closest to. And an authored morph asked for heat 4 when the dial clamps to 1–3. Those bounds are
  now named constants and `every_recipe_and_morph_is_reachable_at_the_bench` fails any recipe or
  morph wanting an off-dial heat, an unknown timing, or a catalyst tag no item carries.*
  *Next: prose is the one axis never touched — `narrative_text.json` is still the smallest file in
  the game. Or the two empty tower routes, or the five one-shot townsfolk.*
- **2026-08-01 — prose.** Prose had never grown because it was gated on code: town reactions were a
  fixed 11-field `NarrativePhase1` struct read by a hardcoded `match`, so every new line meant a Rust
  field and a match arm, and the last three iterations of story beats had gone completely unremarked
  by the town. `narrative_text.json` now carries a `reactions` list — `{npc_id, after_quest,
  after_milestone, order, line}` — and the highest-ordered earned line for that person is the one
  they speak. Authored 35 reactions across all 7 townsfolk and 9 story beats (75 → 309 lines), each
  in that character's own register, including the beats nobody had ever acknowledged: the harvest
  fault being traced, the bed rows turning, the habitats holding, the archive resolving.
  *Adding a reaction is now writing, not code — the point of the change.* New tests:
  `town_reactions_are_gated_on_real_beats` (a typo'd quest or milestone id is prose that ships and is
  never spoken) and `town_reactions_move_on_as_the_story_does` (a late-arriving early beat must not
  drag the conversation backwards). Also fixed `open_dialogue_at_arc_beat` not recording the journal
  milestones a completed quest would really push, which made the harness show conversations the
  player could never have.
  *Next: the two empty tower routes (`archive_stack`, `observatory_span`), the rune workshop still
  being only a gate, or the five townsfolk still on one-shot requests — Mira, Brin, Elric, Ione and
  Lyra could each take the arc treatment Rowan got.*
- **2026-08-01 — quests / NPCs.** Lyra's very first line has always said "fewer pollinators, stranger
  nesting" and nothing ever paid it off. The pollinator-collapse chain from `TODO.md` is now her
  three-beat arc: shelter the creatures, then count what is still flying (eleven where the old log
  says ninety), then give the valley one week of flowering worth coming back for. The last beat
  demands an **Excellent Bloomrise Elixir** — a morph output — so the chain cannot be finished
  without having actually reached the morph layer. Payoff uses `required_completed_quest` in two
  shapes: a third habitat station (`bloomwing` → `bloomwing_pollen`, the containment floor's first
  new occupant) and wild bloom rows opening in the plains, rainforest and moonlit forest. One recipe
  (`pollenwind_draught`) whose ingredients did not both exist in the valley a season ago. Nine town
  reactions to the two new beats — the reactions list from last iteration paid for itself immediately.
  Split `game_data_npcs.json` (752) into `town/npcs.json` + `quests_arcs`/`quests_board`, and
  extended the integrity test to station gates, habitat creatures, harvests and shop stock.
  *Next: the two empty tower routes, the rune workshop, or arcs for Mira, Brin, Elric and Ione.*
- **2026-08-01 — tower floors / runes.** The rune workshop was a gate with a bench in it and no
  route at all. It has one now (`rune_bench_row`) with two things on it: `rune_ash`, which collects
  in the bench channels whether or not anyone has been working, and `ward_rune` — a **fourth rune
  that cannot be bought**, only prised from frames that have already let go, evening and night only.
  The rune layer went 3 → 9 patterns (splash widens, echo repeats, delay holds back, ward turns the
  effect inward onto the drinker), all on potions a player at that tier actually carries.
  *Bug this was one recipe away from: the drafts list drew every entry at 64px with no scroll or cap
  inside a fixed box — at 3 patterns it never overflowed, at 9 it would have run over the footer.
  Now windowed 5 at a time via `visible_window_start`, which is a pure function tested exhaustively
  over every (total, selected) pair up to 40, because clamping to the first five instead would have
  made later patterns silently unreachable.* Rune recipes are now integrity-checked too — station,
  input, output, and that the rune slot holds something whose category is actually `rune`.
  *Next: `archive_stack` and `observatory_span` are the last two empty routes, or arcs for Mira,
  Brin, Elric and Ione.*
- **2026-08-01 — cultivation.** The planter/mutation system had gone untouched for seven passes and
  showed it: three beds accepting **four** seeds between them, all original herbs, and three mutation
  formulas that between them only ever triggered on `glow` and `speed`. None of the fifteen-odd herbs
  added since could be planted at all. Beds now each keep a character (west = heat and stone, east =
  light and water, north = shade), a fourth **Pollinated Bed** unlocks off the pollinator chain and
  takes the desert herbs plus bloomwing pollen, and mutations went 3 → 11 with `restore` finally
  used as a trigger.
  *Bug found before authoring a line of it: `planter_seed_choice` required `rarity >= 2` **on top of**
  the station's seed list, so any common herb named in `planter_seed_ids` was advertised to the
  player as accepted and then silently refused. The list is now authoritative and the rarity floor
  only applies to beds that name nothing. Two tests: every advertised seed must be one the bed will
  actually take (checked against a deliberately broken copy), and every mutation formula must have a
  bed that grows its seed.*
  *Next: `archive_stack` and `observatory_span` remain the only empty routes, or arcs for Mira, Brin,
  Elric and Ione.*
- **2026-08-01 — NPC arcs.** Mira and Elric, chosen over Brin and Ione because their registers
  (medicine, and civic trust) do not repeat the ecology chains Rowan and Lyra already carry. Mira:
  the headaches were never stress, they were the same street drinking from the same well for two
  years, and the fix is something that treats a street rather than a patient — which routes the
  player through the rune workshop for a Fieldwide Poultice. Elric: the council does not doubt you
  can do it, it doubts you can do it twice, so two matched Excellent draughts put the tower in the
  ledger, and two Nightwatch Lanterns take the last dark mile. Payoffs differ deliberately from
  earlier chains — Mira unlocks a **bulk shelf** (a second gated shop station), Elric unlocks
  **standing orders** (higher-value repeatable board requests). Ten town reactions. **No new items:**
  both arcs consume potions earlier passes already added, which is what a mature content set should
  let you do.
  *Three findings. A stale test asserted Mira was a one-shot giver — rewritten to test the
  `quest_ids`/`quest_id` fallback on a stripped clone, so it cannot go stale as arcs are written.
  New check `every_quest_asks_for_something_obtainable` plus its wider sibling over all items caught
  `bloomwing`: iteration 6 added the creature and the habitat that houses it but no way to ever meet
  one — it now works the plains verge and the canopy, gated on the same chain. `murky_concoction`
  surfaced as a false positive and led to `SALVAGE_OUTPUT_ITEM_IDS`, since salvage outputs are picked
  in code and are no recipe's declared output.*
  *Constraint worth knowing: a giver line plus an appended town reaction wraps to five lines and that
  is the dialogue panel's practical ceiling. Do not author a sixth line's worth.*
  *Next: `archive_stack` and `observatory_span` are still the only empty routes; Brin and Ione are
  the last two one-shot givers.*
- **2026-08-01 — tower floors (the last two empty routes).** Rather than a third pass of "this floor
  sheds a luminous thing", the archive got the one gatherable only an archive could have: **pressed
  specimens**, cuttings flattened into the sealed floor logs twenty years ago. They are deliberately
  poor as reagents (quality 16) because they are dead. Brewed with **bloomwing pollen** they come
  back as `hollowroot` — a plant the valley lost precisely because nothing was left to carry pollen
  between its flowers, so the revival is impossible until Lyra's chain has flown. Hollowroot is then
  plantable in the Pollinated Bed and brews a cordial that morphs on starlight. Archive → revive →
  cultivate, threading through three earlier passes. The observatory got **one** node and no more:
  starlight shards left on the focus ring, clear nights only, after Ione's elixir — the counter still
  sells them and still will not say where theirs come from.
  *Design change worth flagging: the archive reconstruction — the game's ending — still demanded what
  it demanded nine passes ago. It now also requires `harvest_beds_turned` and `pollinators_returned`,
  because the revelation is about the tower's ecosystem model and should land on the two ecological
  chains. The civic chains (Mira, Elric) stay optional on purpose. New test
  `the_ending_can_still_be_reached` resolves every ending requirement against the quests and beats
  that exist — verified against a deliberately broken config, since this is the one content mistake
  that makes the game uncompletable rather than merely poorer.*
  *A recipe can output an ingredient, not just a potion — the brew path handles it and skips the
  potion-memory bookkeeping. That is how the revival works.*
  *Next: Brin and Ione are the last one-shot givers. Every route now has something on it.*
- **2026-08-01 — recipes / rooms.** The room-bonus system is fully built — a station can favour
  traits and categories, grant a quality bonus, and gate a morph branch on having earned it — and it
  was nearly unused for one structural reason: **there were only two alchemy stations in the game.**
  Five restored floors and you carried everything back down to the entry cauldron or the greenhouse
  still. Two benches added: a **ward-cooled bench** on the containment floor (favours `cold`, `calm`,
  `pure` and the creature category) and a **reading bench** in the archives (favours `arcane`,
  `luminous`, catalysts). Three recipes made of upper-floor materials moved to the rooms they belong
  in — they were at the greenhouse still only because nothing else existed above the entry floor.
  One new formula per bench, each with a `room_bonus_required` branch, so *where* you brew is finally
  a decision rather than a formality.
  *New check `room_gated_morphs_can_earn_their_room_bonus`: a branch needing a room bonus must sit at
  a station that grants one **and** whose favoured list something in the brew can actually match.
  Worth noting the first version of this test was wrong — it flagged the pre-existing
  `wellspring_elixir` because no reagent hits entry_cauldron's favoured traits, missing that the
  station favours the **catalyst category** and the player fills that slot freely. The data was fine;
  the check was too strict. Corrected before trusting it.*
  *The art-manifest test earned its keep again: four new potions, four missing icons, caught
  immediately.*
  *Next: Brin and Ione are the last one-shot givers, and `required_sequence` is used by only 4 of 28
  recipes — the ingredient-order mechanic is the next nearly-unused system.*
- **2026-08-01 — NPC arcs (the last two).** Brin and Ione. **Correction first: last pass's ledger
  claimed every route had something on it, and that was wrong — `tower_ruin_edge` and
  `creekside_meadow` were both still bare.** Brin's arc fixes the first as its payoff: there are
  terraces cut into the slope under the tower wall that he has walked past twice a day for thirty
  years calling them rubble, and under two feet of mortar there is bed structure, drainage and root
  stock laid out by somebody who understood the slope better than anyone alive. He finishes their
  work rather than starting his own. `creekside_meadow` is now the town commons — two ungated nodes,
  which also gives the opening hours somewhere to gather that is not a half-hour walk.
  Ione's arc is the archive's gaps rather than its contents: eleven months removed from the floor
  logs, cleanly, by someone who left the binding intact so nobody would count. It consumes exactly
  the last two passes' output — the **marginalia lantern** (iteration 11) raises the scraping, and a
  **hollowroot cordial** (iteration 10) is what the recovered page describes, so she can check a
  claim against the thing for the first time in her career. Her payoff is three journal beats at
  once: the record, reconciled.
  **All six townsfolk now have three-beat arcs; every route has nodes.** The route claim is now a
  test (`every_gathering_route_has_something_on_it`) rather than something asserted from memory.
  *Ione's final beat plus her reaction wraps to six lines and is visibly at the dialogue panel's
  ceiling. Treat that pair as the maximum; do not author longer.*
  *Next: no backlog item is outstanding. `required_sequence` (4 of 29 recipes) is the last
  nearly-unused system; otherwise the open axes are prose, biome signature hooks for the five wild
  areas that still lack one, and board-request variety.*
- **2026-08-01 — biome signature hook (measured first).** Counted before authoring: winter had 26
  available nodes against spring's 42, the starter plains collapsed to **1** in winter, and
  `charred_hollow` — a route named after a burn — had exactly one node on it. So the hook was already
  implied: **the burn is the one floor in the valley that still gets light in winter**, because
  nothing has closed the canopy back over it in twenty years. Three fire-follower reagents (`ashcap`
  on the fallen trunks, `emberbark_curl` cured on cold ground, and `frostcrack_seed`, whose cases
  open for one hard frost and no other condition — **winter-exclusive**), two recipes, a morph that
  uses kiln heat to argue with the calendar, and a repeatable seasonal board order.
  *Winter is 26 → 29 and still the leanest quarter, which is correct — the hook gives winter a
  destination rather than erasing the difference. New test `no_season_is_starved_of_gatherable_ground`
  is a **floor, not a balance target**: it fails if the leanest season drops below half the richest,
  which is how winter reached 62% without anyone counting.*
  *Harness fix: `preview_area` centred the room, so anything authored in a corner was off-camera and
  a capture looked like nothing had been added — that happened twice. It now stands the player on the
  first available node, and the very next capture showed all three hollow nodes with the gather
  prompt live.*
  *Next: `required_sequence` (4 of 30 recipes), prose, four wild biomes still without a signature,
  board-request variety.*
- **2026-08-01 — recipes (the ingredient-order mechanic).** `required_sequence` was on 4 of 30
  recipes and three of those just named their own two ingredients in order, which is a memory task,
  not a method. It is now on **13**, expressed as *traits* rather than item ids, all following one
  rule a player can infer and then apply unaided: **the reagent that drives the reaction goes in
  first, the one that governs it second.** All three starter recipes carry it, so the rule is met in
  the first hour — the formulae panel now reads "order any warm -> any calm".
  *Bug found before authoring: the only recipe already using trait tokens (`beastcalm_extract`)
  rendered them through `item_name`, whose fallback is the raw token — so it displayed "order
  luminous -> calm" in a list where every other entry showed item names, and a player could not tell
  a trait from an ingredient they had not met. Sequence steps now render as an item name when the
  token resolves to one and "any {trait}" when it does not.*
  *New test `every_required_sequence_is_satisfiable_by_its_own_reagents` tries every arrangement of a
  recipe's slots (they run 2–3, so exhaustive is cheap and exact). A token naming a trait none of the
  reagents carry permanently faults the recipe with nothing on screen to explain it — verified
  against a deliberately impossible sequence.*
  *Next: prose, four wild biomes without a signature, board-request variety. No system remains
  meaningfully built-but-unused.*
- **2026-08-01 — prose (the ending).** The game now has six three-beat arcs, a revived extinct plant,
  a returned pollinator population and a town charter — and the epilogue was **one fixed paragraph,
  identical however much of the valley you had put back.** The last thing the game said was the only
  thing it said that the player had no hand in. `narrative_text.json` now carries `epilogue_beats`
  (`{after_milestones, order, line}`, same shape as the reactions list), and the ending composes the
  fixed paragraph plus the earned beats. Eight authored; **the panel shows the three heaviest.**
  *That cap is a design choice, not a truncation: an epilogue that listed everything would be a
  checklist. `order` is narrative weight, so the ecological restorations outrank the civic ones.*
  *The measurement was wrong and the capture caught it.* The char budget was estimated at 85 chars
  per line; the real figure is about 78, and the first fully-earned epilogue rendered fourteen lines
  and ran the last one through the footer. Beats trimmed, budget recalibrated to 1000 **from the
  observed overflow rather than from arithmetic**, and the comment says so.
  Three tests: the fullest epilogue fits, an untouched valley gets only the fixed paragraph, and the
  epilogue never loses a beat it earned nor shows more than the cap. New capture scene `ending` —
  the one screen with no other route to it short of finishing the game.
  *Next: four wild biomes without a signature hook, board-request variety, or the six fixed
  `narrative_text` milestones, which are the last prose nobody has revisited.*
- **2026-08-01 — board-request variety.** **Correction: last pass claimed "no system remains
  meaningfully built-but-unused" and that was wrong again.** `required_traits` +
  `minimum_trait_matches` and `required_effect_kinds` + `minimum_effect_matches` had never been used
  by a single quest — every one of the 9 board requests was the same shape: deliver N of a named
  item, repeatable. The board now specifies **how a thing must be brewed, not just what**: an
  exacting all-traits commission (and the first non-repeatable board post), a two-of-three partial
  spec, a dual-effect order, a bulk run, and a rush order paying four times a common bottle's worth
  for a perfect one. Open commissions (empty `required_item_id`) are *not* supported — the item id is
  load-bearing — so this authors into the system rather than extending it.
  *Two real defects found.* The new check `every_quest_spec_can_actually_be_brewed` asks the **real**
  `inherited_traits` rule what a brew ends up carrying, and immediately caught `ruin_survey_for_brin`
  from iteration 12: it demands `restorative` from a Greenmend Salve, whose recipe guarantees `pure`
  and can only ever carry `pure`/`calm`. **Brin's arc has been uncompletable for two iterations**, and
  with it the terraces payoff. Fixed to `pure`; the salve's character was right and the spec was wrong.
  *And the board overlay overflowed the moment it held 14 requests — the available box holds three
  64px cards and drew a fourth over the "Locked Requests" heading, while the 54px locked box was
  rendering every locked summary straight through the two sections beneath it.* Both windowed;
  `visible_window_start` moved out of the rune view into shared `gameplay_overlay_window.rs` so there
  is one implementation. Both caps were **measured from captures, not assumed** — two locked summaries
  still spilled, so it shows one plus a count.
  *Next: four wild biomes without a signature hook, the six fixed `narrative_text` milestones. Before
  claiming a system is fully used, grep for its fields.*
- **2026-08-01 — biome signature hook (rock fields).** Took the previous note's advice first and
  swept every optional schema field against every authored record: **only `minimum_effect_matches` is
  never set**, so the systems really are exercised now. That freed the pass for the measured gap.
  Counted across the four unsignatured biomes: rock fields had the fewest nodes (4), no night, and —
  in a game where weather is a core axis — **no rain at all**. It was the one biome weather could not
  reach. So the quarry becomes the one place rain *improves*: a second route (`quarry_sump`), a
  washvein seam that is invisible in dry grey stone and obvious while the cut face runs, and a
  sumpflower that only grows while the sump is actually holding water. One recipe whose two halves
  both only exist while the workings are wet, so the formula has a weather in it. Brin points at it
  early, right after the first town relief.
  *Method note worth keeping: both biome hooks so far came from counting rather than inventing — the
  hollow from winter being 62% of spring, the quarry from rain never appearing in one biome's
  conditions. Measure the absence, then write into it.*
  *Next: north plains and the rainforest have no exclusive gatherables at all, and the six fixed
  `narrative_text` milestones are still the last prose nobody has revisited.*
- **2026-08-01 — the journal (making authored content visible).** Measured before choosing: the
  Routes tab was showing **7 of 17 routes and about 1 of 25 herb memories**, both cut with a silent
  `break`. Seventeen passes of authored gathering content and the record of it was the least visible
  thing in the game. The herb column was not a paging problem — at full detail one entry filled it —
  so both columns now use the archive's list-and-detail shape: names listed, the selected one shown
  in full, `select` walks them, and each column says "showing X-Y of N". Third instance of this
  family after the rune drafts and the board, so `visible_window_start` earned being shared.
  *This also fixed a defect flagged in an early session and never actioned: route descriptions were
  drawn as one unwrapped line at `x+20` and ran straight through the herb column at `x+420`. The
  first fix — wrapping them in place — showed only two routes, because a route paragraph is three
  lines in a 380px column; hence the list-and-detail shape rather than a wrap.*
  *The capture scene seeded two herbs, which is why this was never seen. It seeds every gatherable in
  the valley now — a sample that cannot exercise the failure is not verification.*
  *Also measured: 14 of 29 ingredients are not plantable, but most are frost, ash, shards and pressed
  specimens that should not be. The genuinely plantable omissions are ashcap, sumpflower, ruinbell,
  field_bloom and frostcrack_seed — a winter-only seed in a warm bed is an interesting question for a
  cultivation pass.*
- **2026-08-01 — cultivation (answering the winter-seed question).** All four beds were in one room,
  so cultivation had no geography. There is a **ward-cooled bed** on the containment floor now, beside
  the cold bench, gated on Lyra's first request — and it is the answer to the question the last pass
  left: a frostcrack case *will* open out of season, but only there, and it takes **five days**, the
  slowest bed in the game. The seed still waits; it just waits somewhere else. That gives the
  `hardwinter_draught` morph a second, slower rival rather than remaining the only way to argue with
  the calendar.
  Plantable ingredients went 15 → 19 (ashcap, frostcrack seed, sumpflower, field bloom), mutations
  20 → 24. **The remaining ten are deliberately not plantable** — slime, cured bark, dust, ward
  shards, salt flakes, rune ash, a pressed specimen, and ruinbell, which only takes in old wall
  mortar. Chasing the number to 29 would have meant pretending those are plants.
  Split `world/stations.json` (749) into `world/stations/<area_id>.json`, matching the areas.
  *The bed's first position put it under the belt HUD — the same framing mistake as the cold bench in
  the benches pass. Stations near a room's bottom edge need checking in a capture, not on paper.*
- **2026-08-01 — biome signature hook (north plains).** Two measurements agreed. The plains — the
  **first biome a player ever walks** — had five nodes and **not one exclusive to it**; and wind was
  the thinnest weather at 23 nodes against clear's 40. So the plains become the wind, which completes
  the set: the desert is time of day, the hollow is season, the quarry is rain. `driftseed` piles
  against the leeward hedge only while it is still blowing; `lieflat_clover` is invisible in still
  air and shows its whole crop when the wind lays the grass over. Both are **ungated and early**, and
  Rowan says so unprompted at order 12 — so the weather system is taught in the first hour by the
  place the player is already standing in.
  *Checked the other outstanding item and dropped it: the six fixed `narrative_text` milestones are
  **not** stale. They describe first-time moments (first floor usable, first brew, first relief),
  not world state, so nineteen passes of content did not date them. Hypothesis was wrong; the text
  was left alone.*
  New test `no_weather_is_starved_of_gatherable_ground`, the counterpart to the season floor — same
  0.5 floor, same framing: leanness is fine, an axis nothing uses is not.
  *Harness: `preview_area` focused the **first** available node, which is always the oldest content
  in the file, so a capture framed the wrong thing twice running. It focuses the last one now —
  content is appended, so the newest thing in a room is the thing worth looking at.*
- **2026-08-01 — the archive (swept for the list bug rather than tripping over it).** Having hit
  unbounded/silently-capped lists three times, swept every draw loop in the game instead of waiting
  for a fourth. Found it: **four of the archive's five lists took the first six rows while
  `archive_selected_index` ranged over the whole list.** So with 32 recipes a player could select row
  20, see no highlight anywhere, and — on the **disassembly and duplication tabs** — press Enter and
  destroy or copy an item they had never seen selected. That is an action on an invisible selection,
  not a display bug. Only the experiments list paged correctly; its arithmetic is now extracted as
  `paged_window` and all five share it, so they cannot drift apart again. `journal_brews` was
  uncapped too (50-odd potions, a draw-level `break`) and now pages the same way.
  *Verified the test against a deliberately reverted list — it fails at index 6, exactly the first
  page boundary.*
  New capture scene `archive:<tab>:<index>`; the console has five tabs and there was no way to look
  at any of them. Page label shortened after the first capture showed it colliding with the detail
  heading.
  *Method note: three of the last four defects came from sweeping a known failure family rather than
  from reading new code. When a bug class repeats, grep for the rest of it.*
- **2026-08-01 — the economy (an axis never touched).** Measured: **~2,950 coins of one-off quest
  reward** plus unbounded repeatables, against **250 coins of floor gates** and a shop ceiling of
  **38**. Money stopped being a decision within the first hour or two, and every content pass since
  has widened the gap.
  The answer is content, not tuning — retuning 32 authored rewards downward would be churn. Elric's
  charter already says goods move on the outer road after dark, so **the road brings a trader**: a
  stall gated on `nightwatch_for_elric`, selling what the valley has no source for.
  *It deliberately undercuts nothing already written* — kiln geodes are dug, ward runes are prised
  off frames, starlight is the counter's own business. What a road brings is imports:
  `saltroad_amber` (210c, a catalyst that holds a reaction open, tag `saltroad`) and
  `southmarket_myrrh` (128c). The money buys capability: two morph branches only `saltroad` reaches,
  and a salve that yields two to a batch, which is the only thing justifying the myrrh at all.
  New test `there_is_something_worth_saving_for` — a floor on ambition rather than a balance model:
  it fails if the dearest purchasable item is not worth deliberately saving for, which is precisely
  the state the game was in.
  Split `crafting/recipes_restore.json` (804) by the bench that brews each recipe.
- **2026-08-01 — rapport (the track finished before the relationships did).** Measured: rapport gains
  +1 on accepting and +2 on finishing, so a three-beat arc is worth **9**, and the top tier sat at
  **6**. Every townsperson became a Confidant partway through their second request, and the single
  payoff — the friendship gift — fires at **3**. Six arcs were written over eleven passes and the
  relationship track resolved before any of them did.
  A **Kin** tier now sits at 9, which is exactly "accepted and finished all three", and a parting
  gift lands with it. The gift is gated on the arc genuinely being complete rather than on the
  number, because that is what it means; the number is only how the journal says it. Four
  `serde(default)` fields mirroring the friendship ones.
  *The gifts are products of the arcs rather than stock from a shelf* — Rowan hands over cuttings off
  the turned row, Lyra pollen the player is the reason there is any of, Brin the **first** cut off the
  restored terraces rather than the best of it, Ione a recovered page she checked four times hoping
  it was a duplicate. Elric, characteristically, adds a second line to the ledger and notes that it
  recurs.
  *Verification note: the gift fires on confirming a conversation, which the capture harness cannot
  drive — it renders frames, it does not press keys. Covered by test (coins, item, milestone,
  idempotence, and that it waits for the last beat) rather than by a screenshot, and said so rather
  than implied.*
- **2026-08-01 — the opening (the one hour nobody had re-read).** Checked the tutorial hints, a system
  never examined in twenty-three passes, and found the **first instruction in the game is
  impossible**: it told the player to gather whisper moss and arcane dust *inside the tower*. The
  entry lab has no ungated nodes at all — its only two are gated behind Brin's terraces, added in a
  much later pass — and arcane dust is several areas away in the quarry. A second hint promised
  potions could be applied to "a person, plant, animal, or blocked path", which is the
  apply-to-target verb `TODO.md` lists as deferred and never built; potions are drunk by the player.
  Both rewritten to describe the game that exists: go north to the plains, sunleaf in open light and
  whisper moss on shaded stone, both morning work.
  *The fix was the text, not the world.* Nothing grows in a laboratory; the entry lab being bare is
  correct and the instruction was simply stale.
  New test `the_opening_can_be_completed_from_a_new_game`: walks ungated warps out from
  `config.starting_area`, collects what can be gathered or bought there without satisfying anything,
  and asserts every prerequisite-free quest's item can actually be brewed from it at a reachable
  bench. Two sabotage attempts failed to trip it — sunleaf is on the plains *and* the town commons
  *and* the apothecary counter — which is worth knowing: the opening is genuinely redundant. It fails
  correctly when the first recipe itself is made unreachable.
  *Same verification caveat as the parting gift: tutorial hints are transient toasts the harness
  cannot trigger, so the text change is verified by reading and the world claim by test.*
- **2026-08-01 — audio (never once examined in twenty-four passes).** Started from a wrong
  hypothesis and checked it: seven areas name no `footstep_sound_set`, which looked like silent
  outdoors, but the schema defaults to `dirt_path` and they had it all along. **Second time this loop
  a "stale content" hunch turned out to be working content** — read the schema before believing a
  count.
  The real gap was underneath it: three sets for thirteen areas, so **the desert, the lake shore, the
  quarry and the burnt hollow all sounded like a garden path.** Four sets added to
  `tools/generate_audio.py`: `sand` (no transient at all, a long soft collapse), `shore` (damp thud
  with a thin splash over it), `gravel` (one impact then several small stones shifting), `leaf`
  (crush first, muffled ground second, short decay). North plains and the town square deliberately
  keep `dirt_path` — field roads and a worn square are what it was made for.
  *Verified without being able to listen:* every generated file was checked for length, RMS and peak,
  and the profiles match the intent — sand longest and softest, gravel highest peak, leaf shortest
  and driest. A synth that emitted silence or four identical files would have shown up.
  New test `every_footstep_set_is_actually_walked_on` — a set that is synthesised, shipped and
  assigned to no area is dead weight, which is exactly what the wrong hypothesis accused `dirt_path`
  of being.
- **2026-08-01 — save/load. A correctness pass, not a content one; saying so plainly.** The save
  system had **no test of any kind** — not in `save.rs`, the codec, the snapshot, the restore or the
  migrations — while twenty-five passes added roughly twenty fields to the progression state. A
  field present in both the state and the save file but not carried by snapshot/restore loses a
  player's run silently, which is the worst way to lose one.
  Wrote a round trip that sets **every** tracked field to something distinctive (so a dropped one
  cannot pass by matching a fresh game's default), saves, loads into a new state and compares. **The
  save system is sound** — nothing is lost. Verified the test bites by dropping `total_brews` from
  the snapshot; it fails on exactly that field and passes again when restored.
  *One surprise worth writing down: restoring produces **more** potion memories than were saved.*
  `rebuild_memory_state` deliberately reconstructs them from inventory, the experiment log, known
  recipes and crafted profiles, so a loaded save can hold more than the one written. The first
  assertion was too strict and the code was right — it now checks the saved entry survives intact and
  that nothing is ever dropped.
  *Also closed the last open question from the schema sweep: `minimum_effect_matches` is the one
  field no content sets, and it turns out it **cannot** bite. `effect_kinds` is read straight off the
  item definition and `required_item_id` is mandatory, so "at least N of these effects" on a named
  item is always trivially true or trivially false. It is unused because it is unusable, not because
  content missed it — do not contrive a use for it.*
- **2026-08-01 — a fourteenth area. Content, deliberately.** Two passes running had been repairs, and
  looking back at the original brief it asked for **more maps** — in twenty-six passes this loop had
  added exactly zero areas. So: the **Southern Pass**, the switchback climbing out of the valley,
  reached from the town square and gated on Elric's charter, because the road below it closing at
  sundown is precisely why nobody walked it.
  *It answers a question an earlier pass planted rather than inventing a new one.* The plains hook
  established `driftseed` as something that arrives on the wind from a plant nobody in the valley
  grows. **`driftbloom` grows on the switchback, in quantity, every seed head pointing downhill** —
  an hour's climb from the hedgerow Rowan has been picking seed out of for nine years, and she says
  so. `coldiron_lichen` covers a hand's width per decade and Brin has now said out loud twice that
  you should take only what you need. Ione notes that every map in the archive stops at the pass
  precisely, as though the far side were somebody else's business, and suspects she knows who decided
  that.
  One recipe brews the plant and its own seed together for the first time.
  *Registration is the single point of failure for a new area: until it was added to
  `loader_embedded.rs` four tests failed at once — route resolution, recipe references and both NPC
  pathing tests. The suite caught it immediately, which is the argument for those tests.*
  Split `sprites/gatherables.json` (810) into `gatherables` and `gatherable_variants` — a reagent's
  own art and the staged copies of it per biome are different jobs — and moved four stray bag-only
  icons into `item_icons` where they belonged.
  **The valley is now 14 areas, 19 routes, 112 items.**
- **2026-08-01 — an eighth townsperson, and the first who is not one.** The brief asked for more
  NPCs and the cast had been **seven since before this loop started** — deepened repeatedly, never
  added to. And the pass built last pass had nobody on it.
  **Tarn** keeps the waystation at the top of the switchback and has carried past this valley for
  eleven years without once having a reason to walk down into it. Every other character here is a
  native looking inward; he is the valley seen from outside, and he is blunt that the road's opinion
  of the place was "the one with the dead tower" and that he never thought to check.
  *His three beats are built entirely from three earlier passes' outputs, in three different
  systems:* a **Nightwatch Lantern** (rune workshop) for a station nobody can see at dusk, two
  **Stillkeeper Tonics** (the ward-cooled bench) for animals that arrive too spent to eat — he is
  explicit that the carters chose this and the mules did not — and two **Hardwinter Draughts** (the
  kiln-forced frostcrack seed) to work the pass through a winter, because if a seed can be argued
  with so can a road.
  The payoff is a thing nobody in the valley had ever seen: `rimeflower` opens on the high stone in
  the coldest weeks and closes by the thaw, and has been doing that unwatched for as long as there
  has been a pass. Ione's line about it is the one I would keep if I kept one.
  *Nothing needed extending to add him — `quest_ids`, the friendship and parting gifts, schedules,
  reactions and the sprite pipeline all took an eighth entry without a schema change, which is the
  first time this loop has added something and touched no Rust at all.*
  **Cast is 8; arc quests 21.**
- **2026-08-01 — the board learns to tell a story.** Checked a hypothesis first and dropped it:
  twenty-eight passes might have piled everything into the late game, but the distribution is
  healthy — **37 of 59 gather nodes are reachable at a new game**, all seven wild biomes are ungated,
  and quests spread evenly across brew tiers. **Third time this loop a suspicion did not survive
  measurement.**
  So the brief's last item instead. Every one of the thirty-five quests was a delivery, and the board
  in particular only ever posted orders. It now carries **three unsigned commissions**, chained by
  prerequisite, that are individually ordinary and in sequence are not: a clean pour, then a note
  specifying *which bench* to use, then a plant that had been extinct for twenty years and had no
  business being known about outside the valley. Read in order they stop being purchases — somebody
  was checking, item by item, whether the tower works, and knew the list before anyone here did.
  Ione gets the reading, Tarn notes that whoever collected them came up the road or down it and he
  saw neither, and no fourth order appears.
  *Same delivery verb, no new mechanic — the structure was already in the schema and content had
  simply never used the board for anything but stock.*
  *A real trap found while building it, currently unsprung:* delivering a **repeatable** request puts
  it on a cooldown rather than into `completed_quests`, so anything listing one as a prerequisite
  waits on a flag that is never set and is locked forever. Nothing does today, but there are thirteen
  repeatable requests to point at by accident. `nothing_waits_on_a_repeatable_quest` guards it,
  verified against a deliberate mispoint.
  **38 quests across 8 givers and the board.**
- **2026-08-01 — a floor with no work on it, and a capture that had been lying.** Measured where
  recipes live: `entry_cauldron` holds **23 of 34**, and the **rune workshop floor had zero alchemy
  stations and zero recipes** — a whole restored floor you visit once to imbue and never again.
  Worse, **no bench anywhere favoured `warm`, `volatile` or `vigor`**, so eight recipes' worth of the
  trait vocabulary had no room that was the right room. (The premise I started from — that late
  materials default to the starter cauldron for lack of a gate — did not survive measurement; all
  seven wild biomes are ungated. Fourth time this loop.)
  The **Channel Forge** goes on that floor: the imbuing channels run hot the whole length of the
  bench, which is a second use somebody cut into it twenty years ago and nobody wrote down. Two
  recipes brewed rather than reworked — a **Channelfire Lantern** (morphing to **Hearthchannel** only
  with the room bonus) and a **Forgestep Tonic** that batches two, because there is no sense heating
  the whole bench for one. Ione's line is that in twenty years of notes on that floor nobody,
  including her, recorded that it was somewhere you could also brew.
  *Guard added:* `every_brewing_trait_has_a_bench_that_favours_it` — verified against a stripped
  forge, which reported exactly `{vigor, volatile, warm}`. Rune traits are excluded by category,
  since runes rework potions and are never reagents.
  ***The harness itself was the real find.*** `preview_area`'s comment promised "every gate already
  satisfied" and it satisfied quests and warps but **not journal milestones** — so every
  milestone-gated bench has been absent from every area capture, and those rooms photographed as
  bare floors. The first forge capture came back empty and the bench was fine; **the camera was
  wrong.** Fixed, and the rune workbench became visible for the first time too. A capture that
  silently omits the thing being verified is worse than no capture.
  *Restructure done in-task:* the new guard pushed `game_data.rs` to 817 lines, so its test module
  split by responsibility into `game_data_reference_tests.rs` (do ids resolve, is everything
  reachable) and `game_data_progression_tests.rs` (can a new game be finished). 83 / 258 / 491.
  **5 alchemy benches, 36 recipes, 73 tests.**

## Deferred (needs a new system; not for this loop)

- Apply-potion-to-target flow (wilted plant, frightened creature, blocked path) — `TODO.md` calls it
  out as the unexpressed premise, but it is a new verb, not content.
- World/character art pass and hand-authored ambient audio.
- Typed overlay state model unifying archive/journal/pause/dialogue/alchemy.
- Post-ending sandbox, if the scope question resolves toward a long tail.
