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
- **2026-08-01 — the pass grew two herbs nothing wanted.** Measured recipe demand per biome:
  every biome's harvest feeds 2–20 recipe uses except the **southern pass, which fed 1**. Two of its
  three herbs — `coldiron_lichen` and `rimeflower` — were authored last-but-three with conditions,
  variants, source prose and a winter-only window, and then **no recipe in the game asked for
  either**. Pickable, describable, pointless — exactly the "no inbound reference" failure this file
  warns about, committed by this loop two iterations after writing the warning.
  Both are `pure`+`cold`, which is precisely what `containment_cold_bench` favours, so the fix was
  already implied by the map. **Coldiron Tincture** answers a decade-slow lichen the only honest way
  — one lichen, three bottles, because the ingredient cannot be hurried — and **Rimeflower Cordial**
  is a fortnight-wide window worth planning a trip for, morphing to **Lastthaw** on `saltroad` amber
  so the pass and the road it carries end up in one bottle. Brin's standing objection is on the
  board *under the order itself*: ten years to cover a hand's width, and a standing order has no
  number on it.
  *Two guards, both verified against deliberate breaks:*
  `every_gatherable_ingredient_is_wanted_by_some_recipe` (named exactly the two herbs when both
  recipes were pulled — my **first** break attempt pulled only one recipe and the test correctly
  stayed green, which is worth remembering: verify the break actually breaks the thing), and an
  extension to the journal's herb-usage test, since `herb_used_in_text` had been returning `None`
  for both — a blank where every other herb explains itself.
  *Split done:* `potions_restore.json` (715) would have crossed 800, so it split along the axis the
  recipe files already use — the bench that brews them. 21 valley-cauldron, 15 tower-bench.
  *Next split candidates:* `narrative_text.json` (769) and `sprites/item_icons.json` (751).
  **41 recipes, 74 tests, 39 quests.**
- **2026-08-01 — the valley's biggest revelation landed in silence.** Measured the reverse of the
  usual check: not "does every line have a beat" but **"does every beat have a line"**. Five
  recorded moments had nobody in town remark on them — including `the_previous_hand`, the discovery
  that the wizard removed eleven months of records *after writing them*, which is the sharpest thing
  the story has to say and which the game recorded in the journal and then dropped.
  13 reactions, weighted at the thin voices: **Tarn had 4 lines to Elric and Ione's 17**. Elric gets
  the hardest one — nine years of being publicly fair about a failure that was actually a decision
  taken alone and then hidden. New lines were placed *below* each character's existing closer unless
  the beat genuinely is their last word, so nobody's ending got steamrolled by a mid-game remark.
  ***The bug the content exposed is the bigger result.*** A conversation's body is a beat **plus the
  earned reaction appended** — up to **659 characters** — and the dialogue panel was a fixed 216
  tall, which is four lines, about 360 characters. **The back third of every arc has been running
  its closing sentences through the footer and off the panel**, on the most-read surface in the
  game. The panel now sizes to its wrapped line count. Verified by temporarily lengthening a
  reaction that does fire in a capture scene, photographing the grown panel, and reverting.
  *Two guards, both verified against deliberate breaks:*
  `every_recorded_moment_gets_remarked_on_by_somebody` (also asserts reactions load at all, since
  `#[serde(default)]` would let a broken include leave the valley mute and still deserialize), and
  `the_longest_thing_a_townsperson_can_say_still_fits_the_panel`, which named all eight townsfolk
  when the budget was pinched back to the old fixed height.
  *Split done:* `narrative_text.json` hit **860 lines**, so the 105 `reactions` moved to
  `narrative_reactions.json` and are folded back in after parsing. 123 / 739.
  *Next split candidate:* `sprites/item_icons.json` (751).
  **105 reactions, 76 tests.**
- **2026-08-01 — the room the game ends in had one thing in it.** Per-area content counts:
  **observatory 1 node, 1 station in a 960×720 room** — the thinnest place in the game and the one
  the story finishes in. Only **2 gather nodes in all 14 areas were night-*only*** (18 more list night among other
  hours), in a game with a working hour system and a room whose whole premise is a lens.
  *A new area was the obvious read and it was wrong.* `saltroad_amber` says outright it is "three
  weeks of carriage from here" and that **no bed, seam or ward in this valley will ever produce
  one**; `southmarket_myrrh` is "from beyond the pass, sold in pinches". The world is deliberately
  bounded at the pass, and a coast map would have contradicted two items' worth of established
  lore. **Fifth time this loop a first instinct did not survive measurement.**
  So the lens instead: **cloudglass** off the outer mirror on clear nights, **mirrorbead** that
  stands on the silver in mist without wetting it, and **lowstar ash** scorched onto the focus ring
  in autumn and winter when the good stars run low — the tightest gate in the game, in the one room
  where "only when the sky allows" is the point. They brew at the **archive reading bench** one floor
  down (2 recipes → 4), and Ione's line is that mirror frost survives exactly two flights of stairs,
  so the wizard put the reading room the right distance below the lens and never wrote that down.
  ***Two harness gaps, both found by the capture disagreeing with the data.*** The area preview
  never set the clock, so it always rendered at ~07:00 and **every night-gated node in the game was
  invisible to every capture** — the observatory photographed as an empty floor. Scenes now take an
  hour (`area:<id>:<day>:<night>`). First attempt used 01:00 and `handle_sleep_pressure` dragged the
  player home to the entry lab mid-capture; night is 22:00 for that reason, written down in the code.
  *Guard, verified against an over-tightened gate:*
  `every_gather_node_turns_up_soon_enough_and_often_enough`. My first version measured a single
  20-day cycle and flagged the lake's stillwater pearl at 0/20 — **the frame was wrong, not the
  node**: the daily roll is `day * 31` mixed with the node id, a linear sequence with period 100,
  not noise. The pearl is genuinely absent for the **first 21 days** though, and it is the only
  source of a catalyst three morphs need. The test now sweeps 100 days and pins both first
  appearance and long-run frequency, with floors set just outside the worst node that already ships.
  *Split done:* `sprites/item_icons.json` crossed to **805**, split to mirror the potion item files
  (13 reagents / 36 restore / 26 glow / 14 speed). Rust derives icon paths from item data and never
  reads these manifests, so only `generate_art.py` needed teaching. All 89 entries verified present.
  *Next split candidates:* `narrative_reactions.json` (760), `crafting/recipes_glow.json` (704).
  **62 gather nodes across 14 areas; 43 recipes; night-only nodes 2 → 5; 77 tests.**
- **2026-08-01 — the valley had four hours in the day and only ever lived through three.** Every
  townsperson had morning, day and evening marks and **no night entry at all**, and
  `active_schedule_index` falls back to the last one — so from 21:00 until 06:00 the whole cast
  stood frozen on its teatime spot. Harmless until last pass made night worth going out in.
  All eight now have somewhere to be after dark, and it says something about each of them: **Ione in
  the archive** (she reads, and now knows the room was built under the lens on purpose), **Lyra on
  the containment floor** (she said she walks down there out of habit), **Rowan cutting in the
  forest** because half of what she grows wants dark, **Tarn at the waystation** because that is a
  night job or it is nothing, and the Crow at the entry, which has never pretended to sleep.
  ***The reachability bug is the real result, and it was found by arithmetic, not by looking.***
  Checking authored positions against area blockers turned up **three gather nodes buried deeper in
  scenery than their own reach** — blockers stop the player with a 14px body radius, so these were
  visible and untouchable. Worst: **`hollow_ashcap_01`, 88px inside a tree against a 44px reach, and
  the only source of ashcap in the game** — so `hollow_hearth_tonic` and `southmarket_salve` have
  never been brewable by anyone. Also Mayor Elric had been standing *inside the town hall* for two
  of his three windows (reachable at 48px against a 56px radius, but rendered through the wall).
  *Two guards, all four arms verified against deliberate breaks:*
  `every_townsperson_has_somewhere_to_be_at_every_hour` (window coverage + scenery), and
  `everything_the_player_must_reach_can_be_stood_next_to`, which measures **reach, not overlap** —
  the greenhouse's north planter sits inside a blocker on purpose, because the blocker *is* the
  raised bed, and 48px of reach clears it. A flag-on-overlap check would have condemned it.
  ***Third harness fix in three passes, same shape as the other two.*** NPCs are seeded onto their
  active schedule mark when the state is built, which happens *before* `preview_area` moves the
  clock — so they stood on morning marks and then walked, at walking pace, over far more frames
  than a capture runs. A night capture photographed the town at breakfast. Re-seeded after the hour
  is set; Ione appeared in the archive immediately.
  *A caution worth keeping:* my first instinct was to report all five position/blocker overlaps as
  bugs. Two were deliberate level design. **Measure the consequence, not the coincidence.**
  **8 night schedules; 3 nodes dug out of scenery; 79 tests.**
- **2026-08-01 — the game's failure state paid out in bottles nobody wanted.** Measured potion
  demand: 51 of 76 potions are never asked for by a quest, but almost all are recipe outputs. The
  four that are **input to nothing anywhere** turned out to be the **salvage bottles** — and
  *every one of the 43 recipes* falls back to one of them, `soothing_tonic` alone catching 21.
  Worth 2 to 14 coins, wanted by nothing. Fail a brew and the game handed you litter.
  The answer was a system already in the game: **rune reworks** take a potion and a rune and return
  a different potion, and all 9 existing ones took *successful* mid-tier brews. Four more now take
  the failures — ward makes a weak tonic **the same weak tonic every time** (which is what an
  infirmary actually orders), splash stops asking a leak to hold its light, delay spreads a rough
  stimulant over an afternoon, and echo repeats a misfire quietly enough to watch. Runes cost coins,
  so salvaging is a real trade against saving the rune for a good brew. Ione's line is the thesis:
  *a rune does not repair a brew, it decides what the brew's fault gets used for.*
  *Guard, verified against a deliberate break:* `every_salvaged_brew_can_be_turned_into_something`,
  which named exactly the two salvage bottles whose reworks I removed. The codebase already had a
  `SALVAGE_OUTPUT_ITEM_IDS` constant with a comment noting content checks "have to be told about
  them separately" — nothing had ever told them.
  ***Fourth harness pass, and the first that found a bug in my own writing.*** The rune workbench
  had **no capture scene at all** despite being a whole overlay, so its drafts list had never been
  photographed. Added one — pointed at the *end* of the list, since drafts are appended and rows 1-5
  would look identical before and after. The capture immediately showed two of my four descriptions
  **cut off mid-sentence**: the nine originals run 55-83 characters and mine ran 123-171. Trimmed to
  house style and pinned at 120 by `every_rune_draft_description_reads_in_full` — budget set to the
  house style rather than the truncation point, because a row that only just fits stops fitting the
  next time the font or panel width moves.
  **13 rune reworks; 81 tests; 80 potions.**
- **2026-08-01 — the greenhouse never learned anything the valley found.** Cross-checked plantable
  seeds against the ingredient list: **19 ingredients cannot be sown anywhere**, and every plant
  added by this loop in nine passes was among them. The beds were still growing exactly what they
  grew before the loop started.
  *Most of that 19 is correct, though.* Cloudglass is mirror frost, lowstar ash is a scorch on a
  ring, rune ash and wardglass frost are residues — none are seeds. And three are refused **on
  purpose**, which is content: `ruinbell` only takes mortar nobody has maintained (Brin: "an insult
  with petals"), `coldiron_lichen` needs a decade per hand's width, and `southmarket_myrrh` is the
  tree Rowan has failed to grow four times and will not discuss. Sowing any of those would have
  erased the thing that made it interesting.
  So three, each with a reason: **lieflat clover** in the west bed, **driftbloom** in the bloom bed
  — which finally answers `driftseed`'s "nobody in the valley grows the plant they come from" — and
  **rimeflower** in the ward-cooled cold bed, out of season and two floors underground, which Lyra
  is openly unsure counts as saving it. Plus six mutation formulas, three of them for seeds
  (`sunspike`, `bloomwing_pollen`, `mist_moth_wing`) that had been sowable since before this loop
  and could never be steered.
  ***A guard I wrote and then removed.*** I added
  `every_seed_the_beds_accept_can_mutate` and verified it against a break — it named exactly the
  three. Then I read the neighbouring test and found `every_mutation_formula_has_a_bed_that_grows_its_seed`,
  whose comment says outright that a seed without a formula "can be grown but never steered, **which
  is fine**." My rule contradicted a stance the codebase had already considered and written down.
  Removed it; the six formulas stand as content without needing to be law. **Check whether the
  codebase already has an opinion before legislating one.**
  *Split done:* `narrative_reactions.json` hit **802**, split **one file per speaker** into
  `narrative/reactions_<npc>.json` (6–20 entries each) with a `REACTION_SOURCES` table. "What does
  Brin say" is now one small file. Guarded: a speaker missing from the table compiles fine and just
  goes mute, so `every_recorded_moment_gets_remarked_on_by_somebody` now fails on any townsperson
  with zero reactions — verified by dropping Tarn's entry, which named him.
  *Next split candidate:* `crafting/recipes_glow.json` (704).
  **114 reactions in 8 files; 22 mutation formulas; 81 tests.**
- **2026-08-01 — nothing in the valley had ever asked for the player's best work.** Quest band
  census: **Excellent 20, Fine 15, Serviceable 4, Crude 1, Masterwork 0.** The top band exists, and
  the deepest mechanics in the game — mastery to seven brews, overcharge, room bonus, catalyst tag,
  stir sequence — all exist to push past Excellent into it. Forty requests and not one recognised
  that they had.
  Two now do, framed as **need rather than achievement**, because the epilogue's whole line is that
  the tower is not yours to conquer. A child on the north road a good cordial will not be enough
  for, with Brin writing *nine hollowroot plants in this valley* under the notice and saying he is
  not objecting, only recording. And Lyra asking, in her own hand, for the best stillkeeper tonic
  ever made — after six years of arguing this valley out of exactly that kind of request. The Crow's
  reply is the counterweight: *this is the part he liked.*
  ***Two candidate slices measured and dropped before writing anything.***
  `minimum_effect_matches` is used by **0 of 40** quests and `required_effect_kinds` by 1, which
  looked like the classic built-but-unused find. It is not: `required_item_id` pins the item, so an
  effect filter on top of a named item is decorative. There is even a unit test exercising the
  summary. Left alone. Then the obvious guard — *is the requested band reachable?* — turned out to
  be trivially true: a probe running the real `calculate_quality` with best-case arguments showed
  **all 43 recipes reach 100**. Probe deleted rather than shipped.
  *The guard that was worth having is a quieter one.* `quality_band_rank` matches five UI strings
  and falls through to **0 — Crude — for anything unrecognised**, so a request misspelling its band
  silently becomes the *easiest* request in the game. A note demanding the finest work in the valley
  would be filled by the worst brew in the bag. Two tests:
  `every_quest_asks_for_a_quality_band_the_game_knows` (verified by lowercasing "Masterwork", the
  realistic slip — it named the quest), and a behaviour test pinning that Excellent ranks strictly
  below Masterwork and that an unknown string outranks nothing.
  **42 quests; first Masterwork requests; 83 tests.**
- **2026-08-01 — the flattest prose in the game was the first prose anyone reads.** Description
  census: 26 items under 70 characters, and the shortest were the **starter** herbs and brews —
  "Soft moss that carries faint magical resonance", "A warming herb favored by local healers". Every
  item this loop authored got 150–250 characters with a voice; the openers never got revisited. A
  player would feel that gradient backwards.
  Rewrote all 26 to the standard the rest of the game reached.
  ***Then the capture disagreed with the data and the finding turned out to be half wrong.*** The
  journal showed text for Arcane Dust that was **neither** the old description nor my new one.
  `journal_herb_summary_<id>` / `journal_potion_recap_<id>` keys in `ui_text.json` win over
  `item.description`, and **23 items have one** — so `item.description` is *only* a fallback, used
  nowhere else in the game. **17 of my 26 rewrites are shadowed and currently invisible.** The real
  player-facing gain is the other 9, plus better fallbacks for the rest.
  *My first census said "zero overrides" and was wrong* — `ui_text.json` nests everything under
  `copy`, and I read the top level. The grep that found the real string is what corrected it.
  **Read the surface, not the field.**
  *Guard, verified two ways:* `nothing_the_journal_shows_is_a_placeholder` measures the **effective**
  text — override if present, description otherwise. A stub on an overridden item passes (nobody
  sees it); a stub on a non-overridden item fails. Checking `item.description` directly would have
  reported both backwards.
  **129 items, none described by a stub; 84 tests.**
- **2026-08-01 — the last room with one thing in it.** Nodes per area: the **archive floor had a
  single gather node** in a 960×720 room, the last survivor of a shape iteration 33 fixed for the
  observatory and explicitly left for later. It is also the room the story's largest revelation
  happens in, which made it the worst possible place to leave as a corridor with a stop sign.
  Two gatherables that could only exist in an archive: **foxed leaf**, rust blooming that comes up
  through old paper in wet weather — Ione permits it only from leaves already past saving, and she
  decides what that means — and **inkgall bead**, iron-gall ink that weeps out of warm bindings in
  summer and hardens on the shelf below overnight. Every bead is a sentence that has left its page.
  They brew at the reading bench into **Ghostline Solution**, which ignores ink entirely and shows
  only *pressure*: where a nib pushed. It cannot read. It can only point at the fact that there was
  something to read — which is the answer to `the_previous_hand`, because he scraped the sentences
  and left the handwriting. The Crow: *he was careful; he was not careful enough.*
  *Three axes measured and cleared before landing here:* warp topology (a clean bidirectional graph,
  no one-way links, no orphans), warp gate vocabulary (**all seven gate kinds used at least once**,
  including `required_mastered_recipe`), and route density (19 routes, none empty). None was a hole.
  *Guard, verified against a break:* `no_room_is_worth_only_one_stop` — stripping the archive back to
  its single node named it exactly. Floor of 2, set at the leanest rooms that ship (entry lab and
  rune workshop, which are mostly benches on purpose).
  *A capture that lied by omission, caught by arithmetic:* the first archive capture used day 2 and
  showed nothing new, because the foxing node's daily roll failed that day. Computing per-day
  availability found day 6 evening carries both, and that capture shows the room with two.
  *Split done proactively:* `recipes_glow.json` reached **750** — the flagged next candidate, and
  this pass was pushing it. Split by bench, as the restore recipes already are: the archive reading
  bench's five moved out (511 / 243). Everything is under 700 lines again.
  **64 nodes, 132 items, 44 recipes, 85 tests — and no room left with one thing in it.**
- **2026-08-01 — the ending did not know how the story ended.** The epilogue earns beats from
  journal milestones, and it **never looked at 26 of the 32 that exist**. Worse than the count: the
  panel shows only the three highest-order *earned* beats (`MAX_EPILOGUE_BEATS = 3`), and every
  high-order beat was a mid-game restoration note. So a player who finished everything — the pass,
  the unsigned orders, the first Masterwork request — got an ending that stopped at the greenhouse.
  **The last twelve passes of story were structurally unable to appear.**
  Three beats, ordered to sit *just under* the flowering-valley note rather than over it, so the
  restoration payoff still leads: the **unsigned orders** (somebody found out what they came to find
  out, and it was never yours to close), the **Masterwork ask** with Brin still writing the cost
  under every notice, and the **pass** — eleven years the road went past this valley, now it stops.
  *Sized against the panel, not guessed.* First draft put the fullest epilogue at **985 of a 1000
  budget** — a budget itself calibrated from a real overflow at 1047. Fifteen characters of headroom
  is not headroom. Trimmed to **933**, which is where the panel sat before this pass, and confirmed
  by capture: 12 lines in a 13-line box.
  ***No new guard, deliberately.*** The ending already has three tests — budget, empty case, and
  monotonic growth to the cap — and epilogue milestone ids are already validated alongside the
  reactions. Nothing here needed legislating; inventing a fourth test would have repeated the
  mistake of iteration 36.
  *Cleared while looking:* rapport is complete (every townsperson has both gift tiers; Elric pays in
  coin at both, which is in character), and all eight NPCs carry the full `phase1_dialogue` key set.
  **11 epilogue beats; the ending reaches the end of the story; 85 tests.**
- **2026-08-01 — the newest biomes never got the oldest reward.** Wild-variant coverage per area was
  near-total everywhere except the three places this loop built: **archive 0/3, observatory 0/3,
  southern pass 1/3.** Same shape as the greenhouse finding — an established system that new content
  never plugged into. Eight variants added, all condition-checked against their node's own gates
  before being written.
  ***The measurement found a live bug on the way past.*** Sweeping all 26 existing variants for
  reachability turned up **`quarry_lichen_sparked`: it wants wind, and its ledge only ever appeared
  in clear or mist.** Eight quality points and a `volatile` trait that had never once been
  obtainable. The wind is the entire idea of a *sparked* lichen, so the ledge gained the weather
  rather than the variant losing its premise — fix the side that is wrong, not the side that is
  easier.
  *Guard, verified against the real bug rather than a synthetic one:*
  `every_wild_variant_can_actually_be_found` walks a 20-day cycle per node and asks the **real**
  `condition_matches` rather than reimplementing it. Reverting the quarry ledge made it name
  `quarry_lichen_sparked` exactly.
  *Plus a payoff test:* reachability proves the conditions *can* align; `a_clear_winter_morning_opens_the_rimeflower`
  proves the reward actually resolves through the same call the game makes.
  **34 wild variants; 87 tests; every project file under 700 lines.**
- **2026-08-01 — the biggest thread in the game was delivered to a noticeboard.** Quest census by
  effect: **25 of 42 want a restore potion**, and **14 of those come from the anonymous board** —
  more than all eight named characters put together. The **infirmary** is named in four board
  orders, in a rune recipe, in a potion description and in Mira's lines, and had **no person**.
  **Wren**, ninth of the cast and the first whose want runs against everyone else's. The valley
  wants the tower to be exceptional; she wants it to be **boring and repeatable**, and says so in
  her first sentence. Her arc is three of the same healing draught ("not two and a promise"), two
  Fine calmleaf infusions for the second room, and then the payoff — **the shelf at the back she has
  kept for twenty years of cases she could not fix**, which she clears and writes the day's stock on
  instead. It also makes iteration 37's Masterwork order land harder: she is the one who named the
  band, and she has a line about having argued against naming the band her whole working life.
  *Cleared before landing here:* catalyst tags (4 supplied, 4 wanted, none orphaned either way) and
  the archive console — which is a **view of the player's own data**, not authored content, so it
  cannot be thin.
  ***Two guards from earlier passes caught this pass's own mistakes.*** The reaction guard
  (iteration 32) failed on `infirmary_ward_quieted` — I had written a milestone nobody remarked on,
  which is the exact hole that guard exists for. And the schedule guard (iteration 34) forced all
  four time windows, which is why Wren has a night mark at all: the infirmary does not close, so
  neither does she. **The tests are now catching me faster than I catch myself.**
  *No new guard — nothing here established a rule the suite did not already hold.*
  **9 townsfolk, 45 quests, 125 reactions in 9 files; 87 tests.**
- **2026-08-01 — a third of the brewing system had almost nobody asking for it.** Demand by effect
  kind: **restore 28, glow 14, speed 2.** Eighteen speed potions exist and **fourteen of them are
  never asked for by anybody**. The valley's road crews and carters are referenced constantly — in
  potion descriptions, in recipe lore, in Tarn's whole arc — and had never once put an order up.
  Four road orders, each wanting a *different kind* of speed rather than four of the same:
  **trailblaze** for the switchback crews with a note not to substitute the gentle version because
  somebody tried and the crew walked back down at noon; **briskstep** for the market walk, firm that
  a stronger tonic is not a better one *here*; **forgestep**, which the crews complain about every
  week and reorder every week; and **longstride**, where the carters have worked out that the
  salvaged one is not the tonic you take at the start of a haul but the one you take when you are
  already tired and the far side is four hours off. Speed demand 2 → 6.
  *The salvage thread gets a second destination* — longstride comes from a failed brew reworked
  with a delay rune, so the road is now a reason to keep failures.
  *Checked rather than assumed:* `obtainable_item_ids` counts rune-recipe outputs, so the guard
  genuinely validated that salvage → rework → delivery chain rather than passing it by default.
  Second use of the `Crude` band, which suits a bottle nobody claims is pleasant.
  *No new guard.* `nothing_waits_on_a_repeatable_quest` already covers the trap these four sit in
  (all repeatable), and an "effects must be evenly demanded" rule would be legislating taste.
  **49 quests; speed is no longer a supply with no demand.**
- **2026-08-01 — the hot bench never got the work it was cut for.** Recipes per bench:
  `entry_cauldron` **23**, greenhouse 9, containment 5, archive 5, and **`rune_forge_bench` 2** —
  untouched since the pass that built it, and the only bench in the tower favouring
  warm/volatile/vigor. It also had **zero restore recipes**, which is odd for the one room that is
  deliberately warm.
  Three recipes, one per effect the bench lacked breadth in. **Bankfire Tonic** is for the kind of
  cold that has had time to settle in, and works by refusing to hurry — Wren asked for the slow
  version specifically, having seen the fast one used on a carter brought down off the pass.
  **Cinderlight Lamp** is a charred-hollow cap and a desert stalk, two things that survived a fire,
  lit on a bench designed for neither; it morphs to **Banked Cinderlight** on kilnfire. **Shiftlong
  Tonic** batches three because a crew is three people, with a board order to match so it is not
  another speed potion nobody asks for. Forge: 2 → 5, and 0 → 1 restore.
  *Placement caught before writing:* a restore recipe at the forge belongs in
  `recipes_restore_rune_forge_bench.json`, not bundled into the cold bench's file — restore recipes
  are filed per bench. New file, registered in the loader.
  *And a bug in my own script, caught before it ran:* the ingredient-set uniqueness check loaded
  every crafting file and read `["recipes"]` **before** filtering by filename, so
  `mutation_formulas.json` would have thrown. Rewritten as a plain function.
  *Verifying took three attempts and that is the note worth keeping.* The archive morphs list is
  sorted **alphabetically by recipe name** — not file order, not loader order, both of which I tried
  first. **Check how a list is sorted before indexing into it.**
  **47 recipes, 50 quests; every bench between 5 and 23; all files under 700 lines.**
- **2026-08-02 — the journal remembered every delivery and nothing the player worked out.**
  **44 of 47 recipes are discovery-only**, and quests have written to the journal since the
  beginning while discoveries never have. The game's own memory held every errand run for somebody
  else and not one thing figured out at a bench. This is on the standing themes list — *"journal
  beats that celebrate a discovery instead of silently logging it"* — and it was still true.
  One `#[serde(default)]` field, `discovery_milestones`, mirroring quests' `completion_milestones`,
  pushed where the discovery toast already fires. **Five beats, only for turning points**: the
  raking light that shows pressure rather than ink; a plant the valley finished burying, brought
  back from a cutting and a guess; the imbuing channels turning out to be usable; mirror frost
  surviving *exactly* the stairs between lens and reading room, so **the distance is the
  instruction**; and the fortnight-wide window the calendar can close. Rowan and Ione react.
  *Two things measured and cleared first:* season/weather/hour balance (0.73 / 0.63 / 0.45 — night
  is leaner **on purpose**, same reasoning as winter), and the 23 recipes nothing points at, which
  is discovery working as designed rather than a hole. **Not every asymmetry is a bug.**
  ***Guard for a risk this pass created:*** recipes are now a **third writer** into a flat journal
  id space, and `push_journal_milestone` silently no-ops on a duplicate — so a clashing id means the
  later beat never appears and quietly inherits the earlier one's title and text.
  `no_two_journal_beats_share_an_id` names both owners; verified by pointing a discovery beat at
  `the_previous_hand`.
  *And the wiring is tested, not eyeballed:* `working_out_a_formula_is_written_into_the_journal`
  resolves a real brew, hands it to the real outcome code, and checks the beat lands once and only
  once. Extending `known_milestones` in the reactions test was a **correctness fix**, not a
  workaround — recipe beats are real journal entries now.
  **89 tests; the journal records what you worked out.**
- **2026-08-02 — two route descriptions were describing rooms that no longer exist.** The same drift
  as the item prose in iteration 38: routes written early sat at catalogue length (76–90 chars)
  while everything authored later ran two to three times longer. Worse than thin, **two were
  stale** — `observatory_span` still described a room with nothing in it but a lens, six passes
  after it stopped being one, and `archive_stack` described a sealed hall with nothing to pick up.
  Nine rewritten.
  ***And the rewrite immediately created the opposite bug.*** The pane draws the description as a
  wrapped block whose **start** is bounds-checked and whose **height is not**, so it grows down into
  the Tower Access panel underneath. Four of my nine came out at 236–262 characters against room
  for about five lines. Trimmed to the longest description that already ships — 215, the one I can
  see rendering — rather than to the collision point.
  *Guard pins **both** ends,* verified in one run: a stub at 76 and a padded 348 named separately.
  A floor alone would have let this pass do exactly the damage it nearly did.
  ***A bug I went looking for and did not find.*** One route used a curly apostrophe (U+2019) where
  every other string in the dataset uses ASCII — **one occurrence in 132 files**. Rather than
  "fixing" it I put a probe glyph in the route the journal actually selects and captured it: the
  font renders U+2019 **and** U+2014 correctly. Not a defect. **The one-off was the smell; the
  render was the evidence.**
  *Cleared first:* disassembly (generic over recipe data, nothing to author) and audio — 34 of 40
  declared sounds are neither generated nor played, but hand-authored ambient audio is on the
  **Deferred** list and event feedback is polish, not world depth.
  **90 tests; route prose 110–215 chars and none of it stale.**
- **2026-08-02 — the southern pass was half a biome, and seven doors were signed wrong.** Distinct
  gatherables per wild area: lake 6, forest 8, plains 6, quarry 6, desert 6, rainforest 5 — and
  **the pass 3**, with only three traits against everyone else's six to nine. It is the road out and
  Tarn's whole home and it had been three nodes since the pass that built it.
  Two herbs that could only grow up there: **leanaway thrift**, a cushion plant that puts nothing
  into standing up — Brin's line is that everything else on this mountain is dead because it argued
  — and the **thinair bell**, which will not open below the cloud line and which Rowan spent four
  years failing to grow downhill and can still tell you the exact number. Each feeds a recipe
  (the guard from iteration 31 would have failed otherwise): a **holding** salve that stops damage
  worsening for four hours of walking rather than mending it, and a light **nothing in the room
  reacts to**, which is the shortest brief Lyra has ever given. Pass: 3 → 5 nodes, 3 → 6 traits.
  ***Seven doors named a room something the room does not call itself.*** A door marked "Greenhouse
  Floor" opens on a banner reading "Tower Greenhouse"; the same archive was "Archives" from one
  floor and "Tower Archives" from another. **19 of 26 matched exactly**, so this was drift, not
  intent. Guarded and verified.
  *Two suspicions that did not survive the check:* "East Forest" and "South Desert" looked like
  stale labels and are the areas' **actual names**. Measure before correcting.
  *My own placement check earned its keep* — it rejected the bell node at 28px from a blocker
  (30px clearance) **and aborted mid-script**, leaving the herbs written and the nodes not.
  `git checkout -- assets/data` reverted cleanly; the lesson is that these authoring scripts are not
  idempotent, so a failed run needs a revert rather than a re-run.
  *Next split candidate:* `items/potions_glow.json` (718).
  **140 items, 49 recipes, 91 tests; every wild biome now carries at least five things.**
- **2026-08-02 — the ending had not heard about the newest person in it.** Wren arrived six passes
  after the epilogue pass, so her **entire arc was invisible to the ending** — including
  `worst_case_shelf_cleared`, the shelf she kept twenty years for the cases she could not fix, which
  is the most human payoff in the game. Beat added at **order 96**, so a completionist now closes on
  valley → open question → **empty shelf**: the quietest note last. Fullest epilogue 926 of 1000.
  *The Masterwork beat drops to fourth by design* — it and Wren's shelf are the **same character**,
  and two beats on one person in a three-beat ending is redundancy, not depth.
  **Reactions per townsperson: Wren 4, Tarn 6, against a mean of 14** — the two newest are the two
  quietest, the exact pattern iteration 32 found for Tarn. Nine lines fill both, and they are the
  two characters who see the valley from **underneath and from outside**: Wren on the tower and the
  infirmary keeping notes on the same eleven months without knowing it, and Tarn on carts leaving
  this valley loaded for the first time in eleven years. Wren 4→9, Tarn 6→10.
  *Two axes measured and left alone.* The **coin economy** looks lopsided — ~4000 one-off quest
  coins against 250 of warp gates and a dearest purchase of 210 — but
  `there_is_something_worth_saving_for` already encodes that decision and passes. And 25 of 34
  milestones go unreferenced by the epilogue, which is **forced by `MAX_EPILOGUE_BEATS = 3`**, not a
  gap. *Do not re-litigate a decision the suite already records.*
  *Next split candidate:* `items/potions_glow.json` (718).
  **12 epilogue beats, 136 reactions; 91 tests.**
- **2026-08-02 — five morph branches hung on one node that shows twelve days in a hundred.**
  Catalyst supply against morph demand came out **inverted**: `starlight` has the highest demand (8)
  and three sources including two shops, while **`kilnfire` gated 5 morphs behind a single quarry
  node available 12 days in 100 and purchasable nowhere**. `stillwater` was second-worst — one node,
  15 days, first appearance day 21.
  **Channel slag** answers it in fiction rather than by tuning a number: twenty years of imbuing
  left a glassy crust in the bottom of the channels and nobody ever swept them, *because nobody was
  ever coming back*. Found by residual glow, so it wants the room cold and dark. It also gives the
  **leanest tower room its third node**, off the 2-node floor.
  ***I over-corrected and caught it by re-measuring.*** First cut put kilnfire at **88 node-days of
  100** — from tightest catalyst to loosest in one move. Seasonal gating brought it to 48, which
  sits fairly against starlight's 17-plus-shops and stillwater's 15. **Fixing a bottleneck is not
  the same as removing the constraint.**
  *Guard is about **routes, not rates**:* `no_morph_branch_hangs_on_a_single_gather_node` — each
  catalyst tag needs a second carrier **or** a counter that sells it. Counting days would mean
  inventing a threshold; a shop line is a real answer to scarcity even when the wild source stays
  rare. Stillwater got a wellstock line at 52 so the rule holds honestly rather than being written
  loose enough to excuse it. Verified by removing both — it named both original bottlenecks.
  *Five axes measured and found healthy, which is worth recording after 48 passes:* quest pacing
  (no gap over 4 brews across 0–40), the opening (5 quests at ≤6 brews, 8 ungated areas), wild
  variants (surfaced on pickup **and** in the journal), the coin economy, and epilogue coverage.
  *Next split candidate:* `items/potions_glow.json` (718).
  **141 items, 92 tests; no morph branch resting on a single node.**

- **2026-08-02 — three creatures, forty-four herbs, and a bench that favours creatures.** Item census
  by category: **44 ingredients, 89 potions, and 5 catalysts against 3 creatures** — and the last
  creature added was the bloomwing, **forty-five passes ago**. The containment cold bench favours the
  **creature category** in its room bonus and there were three creatures in the entire game to feed
  it; six of the areas built since have no living thing on them at all.
  Both new creatures were written into fiction earlier passes had already laid down. The quarry sump
  note has said since iteration 21 that *"the sump has been flooding and drying since before anybody
  stopped cutting here — nothing else in the valley grows on that schedule"*, which is a description
  of fairy shrimp with the shrimp left out: **dustwake shrimp** hatch out of dust that has been dry
  for years, in the hours after the sump fills. And the archive's shelves are undisturbed by
  definition, so **shelf silverfish** live in the bindings; Ione has never had one killed, and where
  they are thickest is where nobody has looked in twenty years.
  ***The second one turned into a plot instrument rather than a reagent.*** The case yields an
  **etched leaf** — a page eaten down to the ink, because they will not touch iron-gall — and the
  light brewed from it **sorts one hand from another**. The first floor log it is held over comes
  apart into two hands where it should hold one: he did not only remove eleven months, he wrote some
  of what was left. Discovery beat plus Ione's reply. The shrimp side is deliberately plain — they
  strip the tank water of everything, and **blankwater** brews the one restorative with nothing in it
  to argue with an earlier dose, which is the brief Wren has been writing for twenty years.
  *Guard, verified against the real failure:* `every_habitat_creature_can_be_met` — a habitat whose
  creature can be neither gathered nor bought is furniture that can be built, gated, drawn and never
  stocked. That has shipped once already (the bloomwing habitat preceded any way to meet a bloomwing,
  and was caught by hand). Removing the quarry node made it name `dustwake_shrimp` exactly.
  *Split:* `items/potions_glow.json` (718) into the carried lanterns and `potions_glow_reading.json`
  — the six lights that show a surface rather than a distance, which is the cut the archive's recipe
  file already took.
  ***The mistake is the note worth keeping.*** After sabotaging one area file to prove the new guard
  bites, I restored it with `git checkout --` and **re-ran the whole authoring script**, which
  appended a second copy of everything into six files that had never been reverted. Iteration 47
  wrote down that these scripts are not idempotent and that a failed run needs a revert; I did the
  reverse. **Revert every file the script touched, or dedupe — never re-run a script over a partially
  reverted tree.** Caught by counting ids immediately afterwards rather than by a test.
  **147 items, 5 creatures, 5 habitats, 51 recipes, 69 gather nodes; 93 tests.**

- **2026-08-02 — the answer was never to carry it down.** Cultivation had gone **thirty-one passes**
  untouched and it showed: **22 of 44 ingredients cannot be planted**, and all five beds are inside
  the tower's first two floors. Most of the 22 are correctly not plants — frost, shards, ash, dust,
  slime, panes, beads, imports — but three are unmistakably plants and all three are **on the
  southern pass**, which the last seven passes built and never gave anywhere to grow.
  The hook was already written and I only had to notice it. Rowan's own item text says she spent
  **four years** failing to carry the thinair bell downhill and make it open, and can tell you the
  exact number; and the cloudglass note says the observatory's outer mirror **is above the weather**.
  So the **Cloud Frame** goes on the observatory shutter ring — the one flat ground in the valley at
  the bell's air — and takes the three pass plants (bell, leanaway thrift, driftseed). Four days a
  crop, the second-slowest bed in the game.
  ***It is gated on a journal beat rather than a quest, and that is the whole design.*** The
  precondition is `discovered_two_flights_down` — iteration 50's discovery that mirror frost survives
  exactly the stairs between the lens and the reading room, *so the distance is the instruction*.
  Ione read that as a fact about mirrors. Rowan reads it as a fact about air and does not sleep. A
  prose beat from two passes ago is now the thing that unlocks a mechanic.
  *Payoff without inventing supply:* `leanaway_salve` already wanted **2** thrift and
  `farcarried_tonic` already wanted **2** driftseed, so the frame is a supply line for recipes that
  were already short. One new formula (**Underglass Tonic**, thrift ×2 + bell ×1 at the still, which
  competes with the salve for the same two thrift) and three mutations, 22 → 25.
  *Two exclusions on purpose, and they are content:* **coldiron lichen** covers a hand's width per
  decade and Brin has twice said to take only what you need, and **ruinbell** only takes in mortar
  nobody has maintained — a tended bed is the one place it will not grow. Chasing 44/44 would mean
  pretending.
  ***Guard for a hole this pass walked straight into.*** Station and node **quest** gates have been
  checked since the first arc; **journal-milestone** gates never were, and four things already used
  one — the reading bench, the channel forge, the astral lens, the archive warp. A typo there does
  not fail to load and does not read as broken: the bench is simply never available.
  `every_milestone_gate_points_at_something_that_happens`, verified by dropping one character off the
  frame's gate. It also caught that this file's milestone set had **three near-identical copies**,
  only one of which knew recipes write to the journal — now one helper.
  *Split:* `game_data_progression_tests.rs` (692) into playability and
  `game_data_narrative_tests.rs` — can the game be finished, versus does it say anything about it.
  *Method note: this pass authored **one** new item. Both hooks were sentences earlier passes had
  already written and left unanswered. At this size the valley is large enough to be read for
  questions rather than mined for gaps.*
  **5 beds, 25 mutations, 148 items; 94 tests; largest test file 697 lines.**

- **2026-08-02 — nine townsfolk and not one of them had heard the game end.** Counted reactions
  against the **fixed narrative spine** rather than against the content each pass had just added,
  which is the measurement nobody had run: the reactions list has 131 lines and covers arcs, board
  beats and discoveries thoroughly — and **`observatory_ending` had zero.** The player finishes the
  tower's whole story, the epilogue runs, and every person in the valley says exactly what they said
  the day before. `containment_started` had zero as well.
  ***The reason is structural and it was in the guard.*** `every_recorded_moment_gets_remarked_on_by_somebody`
  only ever checked **quest** completion milestones, so each pass wired its own new content in and
  the spine went unwatched. Extended to the fixed milestones; verified by stripping all nine ending
  lines, which names `observatory_ending` exactly.
  **Nine last words, one per townsperson, at the highest order in the game** — so after the epilogue
  every conversation in the valley has changed, which is the *post-ending content* `TODO.md` asks
  for, delivered as voice rather than errands. They deliberately disagree: Elric will not give the
  council a clean sentence, Lyra objects to the word restored, Wren wants the whole thing to become
  dull, Brin points out the rows still want turning in the same week, Tarn says the road does not
  forget a place twice. Ione has stopped looking for anything and has started a new set of logs in
  her own hand. Plus three on `containment_started`, the floor built to lock from the outside.
  ***A wrong assumption, caught by an existing test within a minute.*** I also authored six lines on
  `entry_lab_recovered`, on the theory that the game's first achievement was unremarked too. It is
  not an achievement: `initial_journal_milestones()` hands it out at **new game**, so those lines
  fired before the player had done anything, and `town_reactions_move_on_as_the_story_does` failed
  on exactly that. Retargeted to `first_true_brew` — a beat that *is* earned and had one speaker —
  and the guard now excludes the starting condition by name, with the reason written down.
  *Cleanup found on the way:* `narrative_text.json` declared **three milestones the struct does not
  read** — byte-identical copies of beats the quests already record. Nothing broke, which is the
  problem: rewriting one would have changed nothing and looked like it should have, which is
  iteration 38's bug exactly. Deleted, and `NarrativeMilestones` now carries
  `#[serde(deny_unknown_fields)]` so a stray entry is a load failure rather than prose that goes
  nowhere.
  *Harness:* new scene `afterword:<npc_id>` — the whole story recorded, then a conversation. The
  `ending` scene shows the epilogue panel and nothing after it, so nine authored lines had no route
  to a screenshot short of finishing the game by hand. Ione's renders at **six lines, which is the
  documented ceiling**, and fits.
  **158 reactions across 9 speakers; 94 tests; no data file over 664 lines.**

- **2026-08-02 — the tower's deepest verb topped out below every serious brew.** Sorted the rune
  layer's inputs by value and the result is a whole floor out of place: **every one of the thirteen
  imbuable potions was worth between 2 and 32 coins**, and rune outputs capped at 72. Meanwhile the
  game's **fourteen dearest bottles — 96 to 210 coins, all of them products of the last twenty
  passes — were imbuable by nothing at all.** The rune workshop is a restored upper floor behind a
  mastery gate, using a rune that cannot be bought, and it only ever took the starter shelf.
  Four patterns, one per rune, each chosen where the rune's meaning changes what the bottle *is*
  rather than scaling it. **Ward on the Heldstar Lantern** is the one worth keeping: the lantern's
  second effect has read *"steadying to sit beside, for reasons nobody has written down"* since it
  was authored, and warding it turns the light inward, loses most of the lamp, and leaves exactly
  that. Somebody up here worked long nights alone. **Splash on the Handmark Solution** takes the
  hand-sorting light off one page and across a table, so Ione can lay out eleven months of loose
  sheets at once. **Echo on the Twicegiven Tonic** — the brew that argues with nothing — repeats a
  small measure on its own hours, which is a schedule rather than a treatment, and is Wren's own
  brief taken one step past where she wanted it. **Delay on the Deepkeeper** holds the strongest
  restorative back until the body asks. Two board orders carry the demand, both post-arc.
  ***Guard from an invariant the content already honoured.*** Imbuing spends a rune and a finished
  bottle, so a pattern whose output is worth *less* is a trap the drafts list advertises like any
  other. Checked all seventeen before writing it — none violate it, including the four salvage
  reworks — so `imbuing_is_never_a_downgrade` encodes a real rule rather than inventing one.
  Verified by knocking the vigil down 30 coins.
  ***Iteration 8's guard caught this pass immediately:*** all four descriptions came in at 121-139
  characters against a 120 budget and would have been cut mid-sentence. Trimmed to 94-113 and
  confirmed by capture, which also shows the list correctly paging *"showing 13-17 of 17"*.
  *Patterns per rune: splash 5, echo 5, delay 4, ward 3 — the unbuyable rune is still the rarest and
  now has the best thing to do.*
  **17 rune patterns, 94 potions, 28 board orders; 95 tests.**

- **2026-08-02 — six nodes on the lake and one thing that was actually its own.** Counted
  **exclusive** gatherables per area rather than node counts, which is the measurement that says
  whether a place is a *source* or a corridor. The lake shore is the second-largest wild biome by
  nodes and had **one** item found nowhere else — against 5 on the pass, 5 in the quarry, 4 in the
  desert and forest. Six of its nine reagents were shared herbs you could pick closer to home, on a
  single route, and it is one of only two wild biomes with no signature.
  The hook is the one thing a lake does that nothing else here does: **it collects what the valley
  loses.** A second route, the **strandline**, walked before the sun reaches it. `nightwrack` grew
  somewhere up the inflow and let go. `tumbled_glass` is the tower's **own broken glassware**, swept
  out of a door twenty years ago and rolled smooth ever since — and it still takes a reaction the
  way the vessel it used to be did, which is why a shard off a window will not do. And at the warm
  end where the hill drains, `downwash_bloom`: the richest thing on that shore, which was not there
  before, gated on Mira's water-baseline chain **because that is the only reason anybody can date
  it**. Lyra asked on your first good brew to be told what goes down the hill; this is the answer,
  and she is explicit that she is not asking you to stop picking it.
  *Two recipes so all three are wanted, and both of them say it:* the tonic brews something the
  water brought down inside something the tower threw out, and the draught **batches two off one
  handful**, which is the tell.
  *Deliberately plugged into the system new content keeps skipping* — iteration 46's finding was that
  the newest biomes never got wild variants. All three carry one, and
  `every_wild_variant_can_actually_be_found` checked them without being asked.
  ***A guard idea measured and discarded before writing it.*** "Every route must carry something of
  its own" sounds right and is wrong: `creekside_meadow`, `plains_crossing`, `greenhouse_walk` and
  `town_bed_rows` have zero exclusives **on purpose** — shared starter ground within walking distance
  is what they were built for. Encoding that rule would have made four correct decisions look like
  bugs. No new guard this pass; nothing here established a rule the suite did not already hold.
  *Iteration 46's pane guard caught the route description at **347 characters** against its 215
  ceiling and would have run it through the Tower Access panel. Trimmed to 188.*
  **Lake: 6 → 9 nodes, 1 → 2 routes, 1 → 4 exclusives. 157 items, 72 nodes, 20 routes, 40 wild
  variants; 95 tests.**
  *Next thinnest by the same measure: `tropical_rainforest`, 6 nodes and 1 exclusive, one route, no
  signature — the last wild biome at the floor.*

- **2026-08-02 — four schedule marks each, and six of nine never left the room.** The schedule
  system has a guard forcing all four time windows, and content had been satisfying it by naming the
  **same area four times**: Mira, Elric, Brin and Wren stood in the town square all day, the Crow
  never left the entry hall, Tarn never came off the pass. Only three of the nine moved at all. A
  guard that demands four marks got four marks; what it got was an address, not a schedule.
  *It was visible, too.* The journal's rapport tab prints **now**, **later** and **usually** for each
  townsperson, and for six of them all three lines said the same thing.
  Seven marks moved, each to somewhere the content already says that person goes. **Mira takes the
  lake reading in the evening** — she and Lyra keep water numbers and compare them, which is last
  pass's line paying itself off, and the capture has her standing on the shore beside the downwash
  bloom. **Brin checks the greenhouse beds** in the warm part of the day, since he reset those wards.
  **Elric comes up to the tower at dusk** now that it is something the council is answerable for.
  **Wren collects her own stock** rather than waiting for it, and keeps her night mark in town
  because the infirmary does not close. **Tarn walks down for the evening** — eleven years of passing
  the top of this valley, and there is a reason now. And the **Crow moves through three areas**,
  because a guide that is only ever in one hall is a fixture.
  ***Guard: `a_schedule_is_not_an_address`.*** Every townsperson's marks must name at least two
  areas. Verified by putting Tarn back on the pass all day, which names him exactly. It is a narrow
  rule and a fair one: the game prints three lines per person about where they will be, and a
  fourteen-area valley where nobody goes anywhere is a valley where the world is scenery.
  *Cleanup:* `npc_context_line` in `ui_text.json` had **no reader** — the rapport tab builds those
  rows from `overlay_now`/`overlay_later`/`overlay_usually`. Second dead-string find in three passes;
  deleted.
  ***Split, and it turned up a real mess.*** `game_data_reference_tests.rs` had reached **731**, so
  the walk-the-map checks moved to `game_data_world_tests.rs` — references resolving versus content
  being where it says it is. Doing it exposed **two doc paragraphs detached from their tests**: a
  one-liner left behind when `recordable_milestone_ids` moved out two passes ago, and the whole
  ingredient-demand rationale sitting four hundred lines above the test it describes, which had no
  doc at all. Both reattached. *A blind line-range split would have carried them to the wrong file —
  check what the doc comments above a test are actually describing before moving it.*
  **558 + 184 lines; largest file in the project now 646 (data) and 558 (code); 96 tests.**

- **2026-08-03 — the deepest bench in the tower could not tell a Masterwork bottle from a bought
  one.** Second-order brewing shipped two passes ago as *the* late tier, and every potion in the
  data files leaves `quality` unset, so the schema default of **20** stood in for every bottle
  poured into a compound brew. The tier's entire premise — brew the input well, then fold it —
  bought **nothing**, and the materials list said so out loud: a row reading *quality 20* beside a
  bench about to pour a solution the player had spent four reagents getting to Excellent.
  The fix was already half-built. Bottles have carried their grade in `bottle_stock` since the
  quality pass; `brew_ingredients` now folds the best held bottle into the reagent exactly the way
  it folds a wild variant, so quality, traits, preferred-trait matches and **sequence tokens** all
  pick it up with nothing downstream knowing bottles are graded. **On spec the five compound
  recipes score 51–73 on plain bottles and 90–100 on Masterwork ones.**
  ***The subtle half is what gets spent.*** `take_from_inventory` trims the **worst** batch, which
  is correct for a sale and wrong here — pour a Masterwork and the shelf would have quietly kept
  it and dropped a Crude one, so one lucky bottle would have improved every future brew of that
  recipe forever. `spend_brew_bottles` runs *inside* `consume_brew_inputs`, before the count drops,
  and the test drives that real path rather than the helper: I first wrote it against the helper,
  deleted the wiring to check it bit, and it **passed**. A test that cannot fail is not a guard.
  *Content, one per effect kind the bench lacked:* **Shelf-Wide Reading** folds the two archive
  lights the sinkless audit named as vendor trash and reads a rank of spines instead of a page —
  dust says which volumes came off the shelf, so the shelves kept the record he removed. **Carry-
  Down Cordial** treats the journey rather than the injury. **Long-Haul Draught** takes a **rune
  output** and a greenhouse draught whose faults are the same length and cancels one against the
  other — the first time anything the rune floor makes feeds the floor above it. Tier 2 → 5, and
  six previously sinkless potions are required reagents.
  *Filing drift fixed on the way:* a restore and a speed recipe do not belong in the *glow* file,
  so `recipes_restore_archive_reading_bench.json` and `recipes_speed_archive_reading_bench.json`
  now exist and `longheld_cordial` moved into the first — it had been misfiled since it was written.
  *Harness:* new `compound` scene. The plain `brew` scene **cannot** show this, because a bench
  that refuses bottles never lists one; and the first capture came back with the overlay shut,
  because a second-order bench is gated by definition and the sample had not opened its milestone.
  **101 potions, 59 recipes (5 second-order), 48 sinkless; 161 tests.**

- **2026-08-03 — the tier built to stop making vendor trash was making its own.** Last pass'
  own note said the follow-up was *requests, not recipes*, and the count said why: five compound
  bottles, worth 188-240 each and costing two finished brews apiece to make, and **not one of them
  was asked for by anything**. The chain had simply terminated one layer higher up.
  Two **commissions** and three **standing orders** route all five. The commissions are the last
  third doing what the story bible says it does — the valley stops asking for emergencies and asks
  for standards, and the player funds them. **The Relief Post** (1,800, five Carry-Down Cordials)
  is a stretcher, a filled lamp and something that holds a person still at the head of the
  switchback; **Wren costed it eleven years ago and was told the valley could not afford it**, and
  her line is that the number was never the difficulty — nobody had brought the valley anything it
  could sell. **The Standing Road** (3,400, six Long-Haul Draughts) buys two carts a week both
  ways, in weather, **half of them under-loaded on purpose**, because a road remembers a place that
  sends things out and forgets one that only sends for things. Sink 4,900 → **10,100** against
  4,001 of one-off income and 4,766 a full board cycle.
  *The third order is the one worth noting:* the survey commission now has a **downstream** — Lyra
  wants a Masterwork Double-Read Solution for the old well books, because *a number written down
  carefully is not the same as a number taken carefully*. Commissions used to end at a milestone.
  ***Guard:*** `the_late_tier_does_not_make_its_own_vendor_trash`. A morph target deliberately does
  **not** count as demand — that is another way to *make* the thing, not a reason to have one.
  Verified by deleting the shelf-wide order, which names it exactly.
  ***And the verification nearly lied.*** Restoring that order with `mv` kept the file's old mtime,
  so cargo did not rebuild the `include_str!` and the guard failed against data that was already
  fixed. **After restoring a data file by move or copy, touch it before re-running.**
  ***A capture that omits the thing being verified:*** `preview_area` claims *every gate satisfied*
  and seeded **station** milestones only — so every flourish waiting on a beat had been invisible to
  the harness since flourishes were built, including four shipped payoffs. It now seeds all four
  milestone writers (stations, nodes, warps, flourishes). The comment above that code already
  described this exact bug being fixed for stations; the fix had not been generalised.
  *And the first relief post I placed rendered **behind the Current Goal panel** — the station-
  placement rule in `AGENTS.md` applies to anything drawn in the world, not just stations. Moved,
  recaptured, moved again off a gather node.*
  *Board file split three ways by what it takes to be offered the work:* the open board (11), the
  standing orders you have to have earned (25), the commissions you pay into (5). 876 → 240/616/144.
  **65 requests, 5 commissions, 43 of 101 potions sinkless; 162 tests.**

- **2026-08-03 — sixty-one strings about where herbs grow, and a box already dropping the half of
  the entry the player opens it for.** `source_conditions` is 61 authored lines — *"rain and mist,
  when the floor takes up damp faster than the wards dry it"*, *"the shelf below the binding, never
  the binding"* — with **no reader in the game**, and `room_bonus.description` is five more.
  Both are hooked up rather than deleted. The herb journal shows the learned conditions when they
  are known exactly and **the hearsay when they are not**, so a seen-but-unlearned entry stops
  saying *"the memory is still only a glimpse"* and starts telling you when to go looking — the
  authored prose is one step vaguer than the mechanical line, which is precisely what half-knowledge
  should be. The room descriptions head the bench overlay in place of the same sentence shown at
  every bench in the tower.
  ***The hookup uncovered a live bug under it, which is the real find.*** The herb detail box holds
  about four lines and **every entry led with the item description, which wraps to three for two
  thirds of the shelf** — so the gathering conditions ran down through the Tower Access panel and
  the "brews into" line fell off the bottom with nothing to say it had. The entry is ordered by what
  it is consulted *for* now — conditions, uses, numbers, flavour last and cut to its opening
  sentence — block **heights** are checked rather than only block starts (iteration 46's routes-pane
  bug, still present here), and the shelf shows five rows instead of six.
  ***Guard:*** `every_herb_entry_gets_its_conditions_and_its_uses` walks all forty herbs in both
  states using the renderer's own layout constants, exported for the purpose so the test cannot
  drift from the code. At six rows it names Lowstar Ash and Washvein Crystal exactly.
  *The shared overlay subtitle grew too: a fixed 36px box is one line, and the first capture put the
  archive's description half behind the Close button and half outside the box. It sizes to its text
  and wraps short of the button now — the strip stays panel-width so the other four overlays look
  unchanged.*
  *Also: the journal capture scene seeded **every** herb as learned, so the unlearned state had
  never been photographed at all. It now seeds a third of them as hearsay and opens on one.*
  **61 + 5 authored strings live; 163 tests.**

- **2026-08-03 — the deepest verb in the tower left no mark anything could read.** The standing
  entry was that `spread`, `echo` and `delay` were on items and no recipe asked for them. The real
  measurement was worse: those traits sat on the three runes **and on nothing else**, because all
  **17 rune outputs carried no traits at all** — so a bottle that had been through the rune floor
  was, to every trait check in the game, the same object as one that had not.
  Each output now carries the pattern its rune put into it (`pure` for the ward, which is the ward
  rune's own second trait). That lands in two places at once, both already built: an imbued bottle
  can satisfy a **trait-gated request**, and it contributes its pattern to a **compound brew** when
  poured in — `pour_bottle` hands an unbatched bottle's authored traits straight through.
  *Demand, so the marks are wanted rather than decorative:* the two standing orders whose prose
  already described a pattern now **ask** for it — Wren's standing doses have to be the echoed one
  (*"a second bottle beside the bed is not the same thing"*), the archive's tablewide reading has to
  be splashed — and a new order wants a **keptback draught**, held rather than spent, which routes
  another sinkless potion. `longhaul_draught_recipe` prefers `echo`, so folding the echo-imbued
  relay draught into it pays.
  ***A test was quietly lying, and the content walked straight into it.*** `reachable_traits` —
  which decides whether a request can be met at all — walked only **recipes**, so every bottle the
  rune floor makes looked traitless and all three new gates read as impossible. It reads the item's
  own authored traits now, which is exactly what `plain_bottle_qualifies` has always checked a
  delivery against. **A guard that models half the rules will fail correct content.**
  *Two guards added:* `an_imbued_bottle_carries_the_pattern_it_was_given` (per rune, so a fifth rune
  is covered the day it is authored — verified by stripping `echo` off the relay draught) and
  `every_rune_pattern_is_asked_for_by_something` (verified by dropping the delay demand; it names
  `delay` exactly).
  *Applied last pass's lesson:* both sabotage-and-restore runs finished with `os.utime` on the
  restored file, because a `move` keeps the old mtime and cargo will not rebuild the `include_str!`.
  *Bookkeeping:* the TODO's content census was four passes stale (49 recipes, 26 board orders). Now
  re-counted: **14 areas, 77 nodes, 165 items (101 potions, 47 ingredients), 59 recipes across 5
  benches, 17 rune patterns, 25 mutations, 9 townsfolk with 171 reaction lines, 11 + 26 + 5
  requests.**
  *Note for the next pass:* `game_data_progression_tests.rs` is at **734** lines; the next test that
  belongs there should trigger a split instead.
  **42 of 101 potions sinkless; 165 tests.**

- **2026-08-03 — every toast the game has ever raised was thrown away on arrival.** I went looking
  for a surface to hang "you brought me better than I asked" on, opened the feedback module, and
  found `push_event_toast_with_icon(_text, _color, _icon_key)` pushing a struct whose only field is
  a countdown. **The entire payoff channel was a timer.** A beat recorded, a request delivered, a
  route reopened, a formula worked out, a commission funded, a mastery reached — thirteen call
  sites — plus **the whole tutorial hint layer**, all formatted and dropped.
  ***Three layers of it were dead at once.*** Six icons sit in `assets/generated/ui/toasts/`, and
  the two `ui_art.json` keys naming them (`toast_icons`, `default_toast_icon`) had **no struct
  field**, so serde discarded them without a word — the *third* instance of that exact failure after
  the Southern Pass gate and the `alchemy.heat` bindings. `UiArtCatalog` now takes
  `deny_unknown_fields`.
  The toast carries text, colour and icon key; the icons are in the texture manifest; the HUD draws
  the stack above the status strip, newest nearest the eye, capped at three, fading out. **Quiet
  mode keeps them** — they are the payoff channel, not framing, so the density policy lists them
  beside vitality and the clock.
  *Method note worth keeping: this was found by **reading the module I was about to extend**, not by
  a test or a capture. Nothing failed. The suite was green, the game shipped, and the whole
  celebratory layer was missing.* Iteration 8's lesson generalises — when a pass is about to add
  content to a channel, look at what the channel actually does with what it already has.
  *Guards:* a toast carries the words it was raised with (fails outright against the old stub), the
  newest is first and the stack is capped, and `default_toast_icon` has to name a real icon — a key
  that has never once been checked against the list directly beneath it.
  *Harness:* new `toasts` scene, because a banner lasts 2.2 seconds and there is no catching one by
  hand; captured at 20 frames rather than the default 150, which would have caught the fade instead.
  **168 tests; the payoff channel says what it has been trying to say all along.**

- **2026-08-03 — the errand the last pass was blocked on, now that there is somewhere to say it.**
  Quality has paid coin and standing since the quality pass and **not one townsperson had ever
  remarked on it**. Eight `exceptional_delivery_line`s, one each and in their own idiom — Rowan:
  *"You over-made it. I noticed, obviously. That is the whole of what I am going to say about it."*;
  Wren: *"You have given me margin. I do not often get margin."* — raised when a delivery beats the
  stated bar by two grades or lands a Masterwork against any bar, and **silent when it merely
  cleared the bar**, because praise for everything is praise for nothing.
  *An asymmetry fixed on the way:* the board path awarded flat rapport and ignored quality entirely,
  so the same bottle was worth more standing depending on which counter it crossed. Both paths now
  read the same rule.
  ***A rendering bug the capture caught and the test could not.*** The banner grew to two lines for
  these, the box grew with it — and the text still came out elided on one line. `wrap_text` returned
  two lines of 79 characters and `truncate_text_to_width` then cut the first one, because **the wrap
  applies the UI scale to the font size and the truncate does not**. A wrapped line must be drawn as
  wrapped, never re-measured. Found with an `eprintln` in the draw loop and a six-frame capture,
  which is the cheapest debugger this project has.
  *Guards:* the remark reaches the screen and stays quiet when unearned; everyone who can receive a
  delivery has a line (both arc givers and board beneficiaries); and every line fits the banner it
  is raised in, at 2 × 60 characters measured off the capture.
  **171 tests; the valley finally says thank you for the good stuff.**

- **2026-08-03 — the first words a new player reads had never been read by anybody.** The banners
  came alive last pass, and the tutorial hints ride the same channel, so this is the first time the
  opening instructions could be seen at all. Reading them turned up **three defects, all of which
  cost nothing while the channel was dead**:
  the shown-flags lived in **runtime** state, which is rebuilt on load — so the crow's introduction,
  the save hint and the journal hint replayed **every time a save was opened**;
  `tutorial_potions` was formatted with a `{quick_potions}` substitution its copy **had no
  placeholder for**, so the belt keys were looked up, joined and dropped (the banner bug one layer
  down); and three hints **spelled keys out as literals** — "Press J", "with E" — while
  `input_bindings.json` owns them and the rest of the HUD reads it.
  Flags moved to `progression.shown_tutorial_hints`; six copy lines rewritten to ask for the
  binding; the selector is a **list** rather than a ladder of `if`s so a guard can walk it.
  ***The binding guard had to check the copy, not the render.*** My first version asserted the
  rendered hint contains the bound key — and *"Press J to open the field journal"* contains `J` by
  coincidence, so it passed against the exact string it was written to catch. It asserts the copy
  asks for `{journal}` now; verified against the old line.
  *The banner cap went 2 → 3 lines:* the crow's opening instruction is the longest thing the channel
  carries and was being cut mid-sentence the day it became visible. The fit guard from last pass
  covered only the townsfolk's remarks; **the other family of authored line overran it the same
  afternoon**, so it walks both now.
  ***Harness note worth keeping.*** A headless capture runs about **twenty times faster than real
  time** and hint pacing is real-time, so the default 150 frames photographs roughly a tenth of a
  second — three captures showed an empty screen before an `eprintln` in the update loop showed the
  delay ticking down at 0.0008s a frame. Anything on a timer needs frame counts in the **thousands**;
  this one took 3,000.
  **174 tests; `screenshots/hud/opening_hint.png` is the crow, finally saying it.**

- **2026-08-03 — the decision the variant system exists for was being made blind.** The last open
  line in the unconnected-systems audit, written down six passes ago and left: gathering under the
  right sky changes the brew, the bench spends the best held unit automatically, and **the belt
  shows one stack per id**, so nothing on screen ever said which stacks had a good unit in them.
  Two surfaces, because they answer different questions. The **materials list** marks a stack that
  holds one and reads the **variant-adjusted** quality — the number the pot will actually get, which
  had stayed the plain data-file value even after the bottle work taught that same row to read a
  poured grade. The **journal** says what is in the bag rather than only what was once seen:
  *"Noted strain: Static Arcane Dust — 2 in the bag"*.
  *The mark is a mark on purpose.* The meta column is capped at 34% of a 292px card — about twelve
  characters at font 16 — so there is room for `q34` and an asterisk and nothing else. Measuring the
  column before writing the copy is what stopped this being a sentence nobody would see.
  *Guard covers both surfaces, verified by putting the plain title back.* Both capture scenes seed a
  held variant now: **a bench with nothing good in the bag proves nothing about a marker that only
  appears when there is.**
  **175 tests; the audit's last remaining-gap line is closed.**

- **2026-08-03 — the bed was eating the most expensive bottle in the bag, alphabetically.** Went
  looking at the sinkless tail (42 of 101, and **16 of them off the entry cauldron** — everything you
  learn to make first) and found the planter instead. A mutation asks for an effect *kind* rather
  than a named brew, which is the one place in the game that already worked the way an "open order"
  would; `planter_mutation_candidate` then walked `self.inventory`, **a `BTreeMap`**, and took the
  first match. Planting a bed could spend a **284-coin Heldstar Vigil because `h` sorts before `k`**,
  with a 22-coin Kindling Tonic beside it.
  Every other spend already knows better — delivery hands over the worst that qualifies, a sale
  parts with the worst held, the bench pours the best on purpose. **Cheapest that fits** now, which
  is also what the tail is *for*: the four salvage bottles are the cheapest in the game, so a bed
  reaches for a failed brew before a good one. The banner names the bottle as well as the bed.
  *Content:* the **murky concoction** — two coins, the game's only `misfire`, wanted by nothing —
  gets three formulas. **The ordering is the design:** `mutation_formulas_for_seed` returns data
  order and the picker takes the first formula with a candidate, so a bed prefers a proper brew and
  takes the unlabelled one only when that is all there is. Its strain comes up faster and no more of
  it, so it is a trade rather than an upgrade. 25 → 28.
  ***A test I wrote wrong and the suite caught in one run.*** I named two bottles by hand for the
  cheapest-first check and asserted the dear one sorted first — it did not (`duskbell` before
  `heldstar`), so the test would have proved nothing. It now **finds** a qualifying pair in the data,
  which also means a re-priced bottle cannot quietly turn it into a test of nothing.
  *Verified against the old pick by putting `.next()` back.*
  **178 tests; the cheapest thing on the shelf is the thing the bed wants.**

- **2026-08-03 — the archive console was printing reagents, and the question that found it was the
  same one as yesterday.** Last pass's planter bug was *which item does the system take*. I asked it
  of the two archive verbs and both answered wrong.
  **Disassembly returned every ingredient at full amount for one bottle**, and nine recipes brew more
  than one at a time — `coldiron_tincture` and `shiftlong_tonic` turn **three reagents into three
  bottles**, each handing back three. Six free reagents a brew, no travel, no season, forever, in a
  game whose whole outer loop is deciding where to walk. A bottle gives back its **share of the
  pour** now, rounded down; six batch recipes divide away to nothing and are dropped from the list
  rather than eating a bottle for an empty hand.
  ***The mastery bottle is deliberately not in the divisor, and the arithmetic is the reason.*** A
  brew costs 5 vitality and a gather 1.5 — gathering is ~3.3 units per 5 vitality, and a mastered
  one-bottle recipe hands back 2–3 units for the same 5. **Worse than walking out and picking them,
  so it is not a hole.** I built it the other way first and the existing test named the cost
  immediately: the healing draught returned *nothing*. **Measure the exploit before pricing the
  fix.**
  *The panel was lying as well* — it listed the authored ingredients under "Recovered Inputs" while
  the console handed over a share. It shows what comes back now.
  **Duplication burned the gift catalyst first:** it took the *highest-quality* starlight catalyst
  held, and duplication reads nothing from catalyst quality. It reached past a 24-coin shard two
  counters sell for Mira's `counterkept_shard` — a friendship gift, sold and gathered nowhere — and
  spent it for exactly the same result.
  *Both guards verified against the old behaviour; the disassembly one names four recipes by ident.*
  **181 tests; nothing comes out of that room that did not go into the pot.**

- **2026-08-03 — two money printers, one cause, found by pricing every line in the game.** The shop
  was the last unread load-bearing system, and the question was the obvious one: **does any counter
  pay more for a thing than it charges?** One did — the apothecary sold a starlight shard for 28 and
  bought it back for **33**, five coins a keypress, unbounded. Then the same arithmetic against the
  duplication verb: Tarn's `elevenyear_amber` copies for **360** and sells for **640**. Two more
  catalysts at smaller margins.
  ***One cause.*** The quality-band multipliers were being applied to **raw materials as well as
  brews**. A bottle's grade is a fact about the work that went into it — that is what the multipliers
  are for, and why brewing well is worth something at a counter. A herb's or a catalyst's `quality`
  is *potency*, authored once, identical for every unit: running it through a **craft** multiplier
  expressed nothing and inflated everything. The amber is quality 82, which paid 200%.
  ***My first fix was wrong and I reverted it.*** I repriced the shard (28 → 38) — a patch on one
  authored line that left the rule unguarded and would have said nothing about the amber. Once the
  cause was found, the price went back to what the designer wrote. **Reprice the symptom, and the
  next one ships.**
  *Two guards, each verified against the old multiplier and each naming its own cases:* no counter
  pays more than it charges (all 20 stocked lines), and a copy never sells for more than it cost
  (every duplicable item, which names all three catalysts at once).
  *Third pass running on the same question — "which item, and at what price, does this verb take?" —
  and the third pass it has found something: the planter, then the archive console, now the
  counters.*
  **183 tests; the economy has no faucet in it.**

- **2026-08-03 — the rainforest's signature had been written down for passes and nothing was built on
  it.** Habitats read clean first (slow, day-gated, 8–26 coin ingredients — no faucet), so the audit
  streak ended and this went back to content. Counting **exclusive** gatherables rather than nodes:
  lake 4, forest 4, quarry 5, pass 5, desert 4, plains 2 — **rainforest 1**, one route, six nodes,
  five of which grow closer to home. Exactly where iteration 52 said the floor was.
  ***The hook was already in the route description:*** *"the canopy holds the rain long after it has
  stopped falling — the mist under it is not weather, it is the ceiling draining, and it keeps going
  on days the sky is clear."* So the new content is gated **inversely to every other rain thing**:
  **Heldrain Bead** stands in a leaf axil having touched nothing since it was cloud, and is picked on
  **clear mornings**; **Stranglerfig Sap** runs off a host trunk in the hour the heat leaves, out of
  a tree the fig spent twelve years growing down. Second route (**The Drip Line**), four nodes:
  6 → 10 nodes, 1 → 2 routes, 1 → 3 exclusives.
  Both feed **Truemeasure Tonic** — a middling dose that is *identical every time*, which is Lyra's
  survey's whole problem — and it has a standing order the day it ships, so the tier does not add
  another sinkless bottle.
  *Guard:* `every_wild_biome_is_a_source_of_something`, at **two** exclusives — a floor, not a
  target. `north_plains` sits on it on purpose and the square is excluded by name, which is the same
  judgement iteration 55 made when it *rejected* "every route must carry something of its own".
  ***And the capture found a live one.*** The rainforest shot came back with three banners reading
  **"New journal note: ."** — `preview_area` seeds gate milestones with empty titles, which was
  invisible until banners started drawing two passes ago. An untitled beat is a harness artifact, so
  it raises nothing now, and a new guard asserts every *authored* beat has a title and text, which is
  what makes that rule safe rather than a hiding place. Fixing it broke the sound-queue test, whose
  fixture pushed twelve untitled beats — the fixture now titles them, which is what a beat is.
  **168 items, 185 tests; no biome is a corridor.**

- **2026-08-03 — the verb `TODO.md` calls "the largest open gap" shipped with three examples and
  never grew past them.** `apply_targets` went in on 2026-08-02 with exactly the three the TODO
  itself listed — **2 `Restore`, 1 `Misfire`, over 14 areas** — so two of the four `EffectKind`s had
  nothing anywhere in the world to pour them on. Brew for glow or speed and there was no target for
  it; the same shape as the four dead effect kinds one layer out.
  Three authored, doubling the count and covering both missing kinds, each one opening ground the
  way the first three do: **The Drowned Gallery** (rock_fields, glow, Fine — a flooded working; light
  on the water shows the seam) opens stillwater pearl and washvein crystal; **The Closed Hedgerow**
  (north_plains, speed, any grade — speed the hedge through to seed and it opens rather than
  thickens) opens driftseed; **The Fogged Mirror** (observatory_floor, glow, **Excellent**, the only
  one asking the top grade) opens mirrorbead. 6 targets, 6 areas, 4/4 kinds sinkable.
  ***And extending the reaction guard found that no target had ever been remarked on.***
  `every_recorded_moment_gets_remarked_on_by_somebody` walked three of the journal's four writers;
  apply targets were the fourth, added the day before, and the guard had never been taught about
  them. Extended, it named **six** unremarked beats in one go — including the three from the pass
  that built the verb. Six lines authored, one speaker each: Rowan on the bed, Lyra on the roost,
  Tarn on the root wall, Brin on the gallery, Elric on the hedgerow, Ione on the mirror.
  *Second guard:* `every_effect_a_bottle_can_carry_has_something_to_pour_it_on`, verified to bite —
  it names `"speed"` when the two new targets are removed.
  *The mistake worth keeping:* a guard that walks a class of thing has to be revisited when a new
  writer joins that class, and nothing enforces that. The guard read as coverage while covering
  three quarters of the ground.
  **186 tests; all four effect kinds have somewhere to land.**

- **2026-08-03 — the hardest thing the brewing system asks for paid out in vendor trash.** Axis:
  recipes/requests, deliberately off the last two passes' world-content axis. A **morph branch** is
  the deepest verb in the tower: the quality bar, the exact heat and stir count, the timing word,
  often a named catalyst, sometimes a reagent order *and* the room bonus, all at once. 33 branches
  make 29 distinct bottles — and **13 of those were wanted by nothing**: no request, no reagent slot,
  no rune pattern. 13 of the 42 sinkless potions were the *reward for precision*.
  ***And the orders were already written.*** Every one of the thirteen carries a description that
  names its own buyer and had never been asked to: "the only property the crews actually asked
  about", "Ione has one on a shelf and has not written a label for it", "the desert's own lantern",
  "the plainest recipe in the book, taken as far as it goes". The authoring was demand for prose
  that already existed.
  Seven on the **open board**, gated only on brew count (16–26) so a player's first branch is also
  the first time somebody wants what it made. Six **standing orders** off the deep benches and the
  rune floor, gated on the arc that earns them. The thirteenth is the **fourth unsigned note** —
  wildfire draught, "hot and past where the recipe stops", three times the going rate, no address.
  Spread over all eight townsfolk rather than pooling on the two who buy most.
  *Guard:* `a_morph_branch_pays_out_in_something_somebody_wants`, verified by deleting the thirteen —
  it names all of them. A bottle an ordinary recipe *also* makes is excluded on purpose; this asks
  what the branch is worth reaching for.
  **42 → 29 sinkless potions; 187 tests.** What is left is measured, not guessed: 16 plain recipe
  outputs, 9 rune outputs, 4 recipe-and-branch. *Next iteration's obvious group is the rune nine —*
  *the rune floor makes seventeen things and nine are wanted by nothing, same shape one verb over.*
  *Filing:* `quests_board_standing.json` is at 776 lines; split it before adding another order.

- **2026-08-03 — the tower does not notice what you do to it.** Axis: world/place, rotating off two
  passes of demand-routing. `the_world_changes_in_more_than_a_couple_of_places` is a *count*, and a
  count is satisfied by putting everything in one room — which is exactly what had happened:
  **nine of the fourteen flourishes were in the town square.** The tower, the building this game is
  about reopening, changed in two of its six rooms. `containment_floor`, `rune_workshop_floor` and
  `observatory_floor` changed for nothing — and so did **`tower_entry`**, the room the player starts
  in, sleeps in, brews in for hours and crosses on the way to everything else.
  Seven flourishes on beats that already existed, no new schema and no new milestones: the entry lab
  **in use** (a second stool, crated stock, a drying line — `first_town_relief`); the **ledger post**
  by the door (`tower_entered_the_ledger`); **the previous hand** — the wizard's eleven months
  stacked back on the case they were taken from with a lamp left burning over them
  (`eleven_months_restored`); the **pens settled** (`containment_stable`); the **channels running**
  and a finished lamp on the forge bench (`discovered_the_channels_hold`); the **mirror silvered**,
  a clean fan of light from the lens (`observatory_mirror_cleared` — the apply target from earlier
  the same day, now with a visible payoff as well as ground); the **shelves' own record**
  (`the_shelves_kept_their_own_record`).
  *Guard:* `every_room_the_player_works_in_changes_for_something`, and the definition of "a room the
  player works in" is **derived from where the stations are** rather than listed, so a bench on a new
  floor is covered the day it is placed. Verified by deleting the four; it names all four.
  ***The placement lesson, re-learned twice in one pass.*** First draft put the drying line and the
  wizard's notes in the band the title banner and the clock own, and the observatory's chart-floor
  lines under the potion belt. Reasoning about screen coordinates does not work here — the camera
  follows the player, so the world→screen offset changes with where they stand. Capture, look, move.
  *Toolkit fix on the way:* `capture_ui.ps1` built its output filename straight from the scene name,
  so any game addressing scenes as `area:rock_fields:0:day` got an InvalidFilename panic on Windows
  reported as "capture failed". The filename is sanitised now; the game still gets the scene
  verbatim. Committed separately in `macroquad-toolkit`.
  **14 → 21 flourishes, 5 → 9 areas; 188 tests.**

- **2026-08-03 — the ending was a wall, and the Deferred entry saying it needed a new system was
  wrong.** Axis: story/world state. Every request, node, warp and flourish in the game was reachable
  *before* the observatory, so the moment a player finished the thing the whole game builds towards,
  the valley had **nine sentences of last words and then never changed again** — against a scope note
  that says a finished product is 20–25 hours.
  It needed no new system. `observatory_ending` is a journal beat, and beats are the currency warps,
  stations and gather nodes already gate with. **Quests were the one thing that could not read
  them** — a request could wait on another request, a warp, a brew count, a mastered formula or
  somebody's standing, and none of those can say "after the ending". One `#[serde(default)]` field,
  `QuestDefinition.required_journal_milestone`, read by `quest_is_available` and named by title (not
  id) in the locked line. `QuestDefinition` took `deny_unknown_fields` in the same pass, because a
  gate key with no reader is this project's most repeated failure and a request is the worst place
  for it.
  **The Second Bench** — 5,200 coins, six purified draughts, formula must be *mastered* — is the
  sixth commission and the last thing the game asks for. It buys a second bench in the entry lab and
  a year of somebody standing at it: stipend, reference shelf, and a formula book allowed to be wrong
  in the margins. *The last thing the game asks for is the first thing you learned, made well enough
  that somebody else can learn it from the bottle.* Lands visibly in `tower_entry`; Elric, Ione and
  the Crow each have a word ("You just bought the thing that makes you unnecessary").
  Three post-ending standing orders in a new `quests_board_afterward.json` — the square's lamps as a
  budget line rather than a favour, the infirmary's own restock list, the survey's rounds handed to
  a keeper who is not Lyra. All three are the valley placing ordinary business instead of being
  rescued. Plus a **fifth unsigned note**, the first after the observatory, with no question in it,
  which is worse; Ione files all five and writes "still open", because the bible marks that question
  deliberately unclosed.
  *Guards:* `something_in_the_game_happens_after_the_ending` (reads the beat off the narrative spine,
  so a rename cannot hollow it out — verified by stripping the gates, it names 0 requests) and
  `a_beat_gated_request_waits_for_the_beat`, which drives shut → named → open rather than trusting
  the expression (verified by breaking the reader).
  ***A test was quietly wrong too.*** `the_opening_can_be_completed_from_a_new_game` decided what an
  opening quest is from three gates and did not know about mastery, rapport or beats — so the whole
  post-ending board read as available at minute one. It checks every gate now.
  *Filing done as promised:* `quests_board_standing.json` was 776 lines; the unsigned chain moved to
  its own file (it is a story with its own beats, not a supply arrangement), leaving 658.
  **Sink 15,300 against 4,881 one-off and 8,574 a cycle; 85 quests; 190 tests.**

- **2026-08-03 — the rune floor's *first* eight imbuings were all wasted.** Axis: the rune verb and
  its demand, the group the last-but-two pass wrote down as next. Nine of seventeen imbuings made
  something wanted by nothing — and the ordering is the finding. The nine inputs are the **glow
  potion, healing draught, lantern draught, calmleaf, verdant restorative and stamina tonic** —
  everything a player learns in act one — plus two salvage bottles and the top of the whole chain.
  The eight imbuings the valley *did* want were all late-game. So the most natural first use of a
  newly opened floor — *improve the thing I am already good at* — paid out in vendor trash every
  time, and a player only found the verb worth using hours later.
  Nine orders, buyer drawn from what the imbuing actually does rather than invented: splashed glow
  lights the whole cut at once instead of eleven people passing a lamp; the echoed healing draught
  arrives twice for patients who will not sit still; the delayed stamina tonic is drunk at the bottom
  of the pass and works at the top; the beacon burst is not for walking by, it is for **being
  found**; warded calmleaf holds a *shallow* sleep to morning rather than going deeper, which is
  what Wren keeps saying she wants; the second-spring tonic feeds a bed again in the second week,
  when a bed decides whether it took; three leakfire flares because the crews would rather have three
  bad ones than one good one; two second readings because Ione will not accept a reading she cannot
  check against itself; and the heldstar vigil burning dusk to dawn unattended on the hall steps the
  nights the pass is shut.
  Two of the nine route **salvage** bottles — a failed glow brew and the unnamed murky mixture — so
  the failure tail now pays at the top of the tower, which is a nicer shape than pricing it up.
  *Guard:* `every_imbuing_the_rune_floor_makes_is_wanted_by_something`, the rune analogue of the
  morph one, verified by deleting the nine — it names all nine.
  **29 → 20 sinkless potions; cycle 8,574 → 10,050; 191 tests.** What is left is one flat class,
  16 plain recipe outputs plus 4 a recipe and a branch both make, with no shared cause — a tail
  rather than a hole, and probably a *recipe* problem rather than a demand one.

- **2026-08-03 — second-order brewing was a feature of one room, not a tier.** Axis: the recipe
  lattice, taking the previous entry's own suggestion that the rest of the tail is a *recipe*
  problem. The measure: per bench, how many of its own outputs anything anywhere consumes as a
  reagent. Archive 4, greenhouse 4, cold bench 1 — and **the entry cauldron's 24 outputs and the
  rune forge's 5 fed nothing at all, anywhere.** The forge was the worst room in the building: five
  recipes, three of them making bottles nobody wanted, behind a whole floor of the tower.
  *The fix routes the dead stock by eating it rather than by writing more orders for it.* The
  **channel forge accepts finished bottles** now — the most natural place in the tower, because the
  rune workbench on that same floor already reworks finished bottles, so it is the floor's own
  premise. Its room bonus already said what the bench is for ("the channels that take an imbuing run
  hot the whole length of the bench"), so its second-order character is *heat nobody has to stand
  over*, against the archive's *reading*.
  **Banked-Through Tonic** (bankfire + kindling + emberbark): nothing happens for a while and then
  you are simply not cold until it gets light. **Held-Heat Lamp** (channelfire + cinderlight +
  scorchvine): a working light that is also a brazier, because a crew in a wet cut has only ever
  been able to carry one of the two down and has always chosen the light and then stood about being
  cold. Both are the first recipes in the game to *require* `kilnfire` — a tag five morph branches
  asked for and no recipe ever did.
  ***Every branch on both reaches a bottle something already wants***, so a tier that would have
  added six bottles added two: lastthaw / kept-warm off the tonic, hearthchannel / banked-cinderlight
  off the lamp. Cool the tonic right down and you get the keeper the winter stores want; drive it
  with road amber and you get the cordial that survives the cart — a three-way choice between things
  the valley wants, which is what a branch is for.
  *Guard:* `more_than_one_bench_takes_a_finished_bottle_and_means_it` — a tier is not one room, and
  a bench that advertises it takes a bottle must have something asking for one. Verified by turning
  the forge back off.
  **Forge 3-of-5 sinkless → 0-of-7; 20 → 16 sinkless potions; 60 → 62 recipes; 192 tests.**
  The `compound` capture scene now finds the *forge* rather than the archive, which is a nice
  accident of it picking the first second-order recipe in data order.

- **2026-08-03 — the ground the newest verb opens was the only ground with no name and no season.**
  Axis: world/gathering, rotating off two passes of recipes and demand. Two rules the entire world
  obeys, each broken in exactly one place, which is why neither had ever been written down:
  **a node belongs to a route** — `route_id` is what the herb journal writes into
  `first_seen_route_id`/`learned_route_id`, so a node without one files the pickup as *"an unknown
  place"* — and **no ground in this valley is available all the time** (every node constrains at
  least one of season/weather/time; 44 constrain all three).
  The exception on both counts was the **five nodes the first apply-target pass opened**: three on
  the re-seated bank above the switchback, two at the settled roost. So a player poured a brew on a
  thing, opened new ground, walked it, picked something — and the journal could not say where they
  had been, and the ground itself could be worked at any hour of any day of any year. *The three
  nodes the second apply-target pass opened do have routes and conditions, so it was one pass's
  blind spot, not a habit.*
  **The Upper Bank** (southern_pass, also fixing 8 nodes on 1 route) hands you a different plant per
  season — driftbloom early, thrift through the heat, coldiron lichen once the cold is in. **The
  Settled Roost** (moonlit_forest) can only be worked after dark and in still air, which for twenty
  years meant not at all. Notes rewritten so prose and conditions agree.
  *Guard:* `every_gather_node_has_a_place_and_a_season`, **both halves verified separately** —
  strip the routes and it names all five, strip only the conditions and it names the three on the
  bank. The lesson is the shape of the finding rather than the fix: a rule followed 80 times out of
  85 is invisible precisely because it looks like the way things are.
  **21 → 23 routes; 193 tests.**

- **2026-08-03 — the game has sixty-two formulae and tells you how to make three.** Axis: the
  discovery layer, which is not on the standing-themes list and had never been examined in sixty-odd
  passes. Measured before choosing, and the measurement is the whole finding. There are exactly
  three routes to knowing a formula: `starter_known` (3 recipes), hitting its exact ingredient
  multiset at its exact bench, or — nothing. **Quests carry no `teaches_recipe` field, no counter
  sells a formula, and disassembly requires the recipe to be known already.** So the other **59**
  are guessed, against **46 two-reagent recipes** (1,485 pairs across the 54 things that can go in
  a pot) and **16 three-reagent** ones. The bench does confirm an exact hit *before* you spend
  anything, so it is browsing rather than blind — tedium rather than impossibility, and no better a
  use of an evening.
  ***And the journal, the game's own memory, was where the information should have been and wasn't.***
  Every undiscovered use of every herb read *"Used in formulae you have not yet discovered."* A
  count. Sixty passes of authored recipes behind a line that names the gap and not one thing to do
  about it.
  **Discovery stays the design** — `starter_known` is untouched and no formula is named. The line
  points instead: **where the missing half comes from**, as ground you can walk to, a counter that
  stocks it, or the fact that it must be brewed rather than picked (which is also the only way the
  second-order tier announces itself). Once everything the nearest formula wants has been met, it
  stops sending the player out and names **the bench** — for anyone who has been round the valley
  that is the whole of the useful answer, and pointing them at ground they have already worked
  would be the old useless count in a longer sentence.
  *Derived, not authored:* nothing is written per recipe, so a formula added tomorrow is pointed at
  the day it ships. 57 reagents resolve to ground, 5 to a counter, 105 to a bench.
  *Two rules the derivation had to learn, both from reading the output rather than from theory.*
  Ground still shut is worse than no hint — "not here" and "not yet" read identically — and whisper
  moss grows in seven places with Brin's terraces sorting ahead of the plains. And season, weather
  and hour are deliberately **not** consulted: the hint answers *where*, the conditions line
  immediately above it already answers *when*.
  *Guards:* every reagent that feeds a formula points somewhere (verified by cutting the bench
  branch — it names all 14 compound reagents); the hint names neither the formula nor the reagent it
  wants; and nothing is sent to shut ground (verified by dropping the preference — it names whisper
  moss to the terraces and stillwater pearl to the quarry).
  ***The entry box is full now, and the guard from the hearsay pass proved it.*** The first copy
  overran by **6px** on Inkgall Bead — three lines of conditions plus two of hint against room for
  four. Shortened, and the harness took a `journal:<herb>` scene so the worst entry in the game gets
  photographed rather than trusted to a `chars-per-line` estimate the comment itself calls generous.
  `screenshots/hud/journal_formula_hint.png`, `journal_hearsay.png`. **197 tests.**
  *Next: the obvious follow-on is a `teaches_recipe_ids` field on quests — Ione's recovered eleven*
  *months and the wizard's working notes are already in the fiction and already flourishes in the*
  *entry lab, and nothing in the game hands over a formula. That is content, not machinery.*

- **2026-08-03 — the ending was answered entirely in paperwork.** Axis: world/gathering content,
  rotating off a UI-surface pass. The entry that said "the ending is a wall" was ticked three passes
  ago by three standing orders, a commission and an unsigned note — all of it **requests**. Counted
  afterwards, the valley itself had not moved: of **85 gather nodes, 23 routes, 6 apply targets and
  14 areas, not one waited on `observatory_ending`.** Somebody who finished the thing the whole game
  builds towards could go on being paid, and had nowhere new to stand while it happened. The
  Deferred entry said as much in its own last sentence and nothing had acted on it.
  ***The fix came out of the story bible rather than out of me.*** Three of its rules decide the
  whole design: **restoration is to the ground, not to the tower** ("anything that reads as the
  tower reclaiming the valley is off-model"); **recovery is measurable, and the measurements are the
  drama**; and **new content should diagnose before it fixes.** So the post-ending world change is
  two pieces of ordinary ground that came back *on their own*, each a second-order payoff of an arc
  the ending already requires, and each a thing somebody counts rather than announces.
  **The Seed Year** (north_plains): **Rattleseed** cannot set its own seed, and for twenty years was
  the same crowns making no pods, because there was nothing flying to set it. Lyra counted eleven
  fliers where the book said ninety; she is counting pods on four marked crowns now, "which is the
  only honest way to find out whether one number caused the other." Autumn, and dry — a wet pod does
  not rattle and does not keep. **The Clear Shelf** (lake_shore): **Sunkbell** flowers on the bed
  rather than the surface, so it opens only where light reaches the bottom; the shelf carried a foot
  of suspended silt on a still day for twenty years and everybody put that down to the lake being
  the lake. Nobody dredged it. It settled.
  Both feed **Seedhold Solution** at the ward-cooled bench — deliberately a *poor* draught, on the
  truemeasure precedent, because what it is for is that seed steeped in it goes on being seed. Rowan
  buys it as the start of a four-year seed store she has wanted to place since she was nineteen, so
  a valley that has only just got its seed back never bets the whole of it on one autumn again.
  One beat, `the_ground_answered`; Lyra and Rowan each have a line. Rowan's is the one I would keep:
  *"I had decided it was a plant that did not rattle. That is the part I keep coming back to — not
  that it stopped, but that I had already finished being surprised about it."*
  *Guard:* `the_ending_opens_ground_and_not_only_paperwork`, which asks for **two routes** rather
  than a node count — one route is a corner, and the flourish pass already established that a world
  change satisfied in a single place is not a world change. Verified by pointing it at a milestone
  that does not exist.
  ***Four existing guards bit during authoring, which is the argument for all of them:*** two nodes
  were dropped inside blockers (`everything_the_player_must_reach_can_be_stood_next_to`, 58 against
  a 44 reach), two were so narrowly conditioned they turned up **4 days in 100**
  (`every_gather_node_turns_up_soon_enough_and_often_enough`), both route descriptions overran the
  pane by ~30 characters, and the art manifest caught three missing icons. None of that was visible
  by reading.
  **23 → 25 routes, 85 → 89 nodes, 170 → 173 items, 62 → 63 recipes, 96 → 97 quests; 198 tests.**
  `screenshots/hud/plains_seed_year.png`, `lake_clear_shelf.png`.
  *Next: post-ending ground exists but the post-ending **tower** does not — no floor, station or*
  *apply target waits on the ending either. And the sinkless tail is now 17 of 105.*

- **2026-08-03 — nine people have walked to the same places every day for sixty passes and never once
  said why.** Axis: NPCs and prose, rotating off two world passes and a UI surface. Measured first
  across the whole cast — arc quests, board orders bought, rapport orders, reaction lines — and it
  came out even, 3 arc beats each and 12–31 reactions, so the cast is not thin. The gap was one axis
  over: **the schedule.** Every NPC carries four stops, `npc_now_hint`/`npc_later_hint` will tell you
  where somebody is, the sprite walks there — and their *words* never moved with them. **Ten of the
  thirty-six stops are away from home and not one had ever been asked to explain itself.** Mira at
  the lake shore at dusk said exactly what Mira behind her counter said.
  `NpcScheduleEntry.while_here_line`, one `#[serde(default)]` field, read as the conversation's
  opener when they are on that stop with nothing of the player's pending. **An errand still wins** —
  forward motion beats flavour — so this is what they say between beats and once their arc is done,
  which is most of act three onward. Verified on screen, and the capture also confirmed the
  precedence: Mira's away line is correctly suppressed at day one because her first errand is
  available from the opening minute.
  Ten lines, each out of that person's own arc rather than out of scenery. Mira tests the lake
  because the well row and the lake drink out of the same table and she would rather find out about
  the next one two years early than two years late. Lyra counts at night because walking into the
  pens at noon is counting how frightened they are of you. Ione works by raking light because raking
  light only works when there is no other light. Wren walks up with the list because the list has
  never once come to her — *"That is not anybody's fault. It is only what happens when there is no
  one at the top of the hill to send it to."*
  *Guard:* `a_townsperson_away_from_home_has_a_reason_to_be_there`, which makes it a rule a tenth
  townsperson inherits rather than a pass that happened. The crow is exempt on purpose: its four
  lines are a tutorial ladder an away line would shadow, and it does not live anywhere, which is the
  joke and the exemption.
  ***And the second guard had to be taught the clock.***
  `every_line_a_townsperson_has_is_reachable` walks each person through arcs, brews and town
  recovery — and never through the *hour*, so all ten new strings would have passed it in silence.
  Extended, it fails against the old selector and names all ten. **This is the second time in this
  loop that a guard walking a class of thing has missed a new writer joining the class**, after the
  apply-target reactions; there is still nothing that enforces the revisit.
  *Harness:* `dialogue:<npc>:<beat>:<window>`, and the window→minutes mapping moved out of
  `preview_area` into one `set_time_window` rather than being copied.
  **36 scheduled stops, 10 of them explained; 199 tests.**
  `screenshots/hud/dialogue_away_from_home.png`.
  *Care needed next time: `git checkout <file>` to undo a sabotage discarded ten uncommitted*
  *authored lines. Copy the file first, or sabotage in the Rust rather than in the data.*

- **2026-08-03 — the sinkless tail had a cause after all, and it was a room.** Axis: recipes and
  demand, rotating off world and NPC passes. Three earlier readings of this list called it "a flat
  class with no shared cause" and moved on. Counted **per bench** rather than per bottle it has one:
  **the greenhouse still made twelve bottles and six of them were wanted by nothing at all.** That is
  the worst room in the building — entry cauldron 7 of 24, cold bench 2 of 9, reading bench 1 of 11,
  rune forge 0 of 7 — and it is the bench act two is spent at.
  ***And the six were not a random half.*** Every wanted greenhouse output has a morph branch or
  feeds another recipe; every unwanted one is flat — two reagents, one bottle, no branch, no
  downstream, no buyer. They are the **place** recipes: the brew that exists because a particular
  piece of ground does. The salve made of what grows on the terraces Brin spent thirty years calling
  rubble. The tonic that brews a plant together with the seed it throws, from opposite ends of a
  question Rowan asked for nine years. The draught made of pollen that did not exist in this valley
  a season ago. **The game opens the ground, authors the brew that ground exists for, and then
  nobody ever asks for the result.**
  Six orders, and — as with the thirteen morph branches — *every buyer was already written into the
  bottle's own description*: "Brin uses it on ground that has been let go", "Mira says the valley
  has been paying for half of this unnecessarily for years", "Rowan wanted it on record that she had
  asked where the seed came from for nine years", "Lyra will not call the pollen a harvest, she
  calls it rent". Filed in a new `quests_board_ground.json` because they share a cause rather than a
  tier, and because the standing file is at 658. Spread over six townsfolk with the two lightest
  buyers picking up one each; each gated on the arc that opened its ground.
  *Guard:* `no_bench_makes_more_vendor_trash_than_it_makes_work` — a floor, not a target: a bench
  must want more of its own output than it wastes. Verified in the Rust (not the data, see below)
  by dropping the new orders out of the demand set; it names the greenhouse at 6 against 6.
  ***A second finding fell out of the arithmetic and is now a tripwire.*** Every demand pass routes
  a bottle by writing a **repeatable** order, and a repeatable order is unbounded income. Across
  four passes a full board cycle went **4,766 → 8,574 → 10,050 → 13,086** while the commission sink
  was set once at **15,300** and never moved. Nothing noticed, because the existing balance guard
  compares the sink to *one-off* income, which barely changes.
  `a_single_board_cycle_does_not_pay_for_the_whole_last_third` now fails if one lap of the board
  funds the whole last third. About 15% of headroom left, which is roughly one more pass of this
  size; when it goes, the answer is another commission rather than smaller rewards — the demand is
  the content and the sink is the tuning.
  **16 → 10 sinkless potions; 97 → 103 quests; 201 tests.** Board overlay checked at 103 quests and
  still windowing correctly ("showing 1-3 of 5", "+73 more locked").
  *Sabotage done in the Rust this time rather than in the data, per last pass's note. It works and*
  *is safer: there is no uncommitted authored prose to lose.*

- **2026-08-03 — the rest of the sweep for the bug this project keeps having. A correctness pass,
  not a content one; saying so plainly.** Started by measuring the cast, the creatures and the
  habitats for a content slice and finding all three healthy — five creatures each caught in the
  wild and each feeding a recipe, nine townsfolk evenly served. So I swept a known failure family
  instead, which this file's own method note has recommended since the archive pass and which nobody
  had done for the biggest one.
  **A key in a data file that no struct claims.** Serde drops it in silence, the file reads as
  configured, the game ignores it, and nothing anywhere says so. Four instances on record: the
  Southern Pass's `required_completed_quest` (the gate the southern half of the map sits behind did
  not exist at runtime); `alchemy.heat` and `alchemy.fill_slots` in the input bindings;
  `toast_icons`/`default_toast_icon` in `ui_art.json`, which is why six generated icons were never
  loaded; and three duplicate entries in the narrative milestone block. **Every time, the fix was
  the attribute on that one struct.** Counted: **11 of 48 deserialised structs strict, 37 not** —
  `ItemDefinition`, `RecipeDefinition`, `AreaDefinition`, `StationDefinition`, `NpcDefinition`,
  `GatherNodeDefinition`, and the nine file-envelope structs where a stray *top-level* key vanishes.
  All 37 strict now, and **the sweep found no dead keys in the current data.** That is the good
  outcome and it should be reported as one rather than dressed up: the value is that the fifth
  instance is a red test rather than a mystery six months later.
  *Two exclusions, deliberate:* the save-side files stay lenient. They parse what **older builds**
  wrote, not what an author typed — `HabitatStateEntry.placed_day` was deleted as dead in an earlier
  pass and every save from before that still carries it. Strictness is right for content you control
  and wrong for a record the player already has on disk.
  *Three guards, in a new `game_data_schema_tests.rs`.*
  `every_content_schema_rejects_a_key_it_does_not_read` walks **every `.rs` file under `src`** and
  keys off the `Deserialize` derive rather than off a hand-written list of "the schema files" —
  which matters: `UiArtCatalog`, the struct that motivated the whole thing, lives in `src/art`, and
  my first draft's hardcoded list missed exactly it. Widening the scan then named twelve more,
  including all nine envelopes. It also asserts it found ≥35 structs, because **a source-scanning
  guard fails open** — rename a file and it checks nothing while still passing.
  `a_key_nothing_reads_is_now_a_load_failure` drives a misspelling through the real loader and
  asserts the error names it, so the guard's belief about what the attribute *does* is tested rather
  than assumed. `every_embedded_content_file_still_parses` moves the failure from runtime to CI:
  `ui_text.json` loads through `parse_json_or_else`, which prints to stderr and carries on with
  `[missing ...]` placeholders, so without it a typo ships as a game with no words in it.
  **201 → 204 tests. No content added, and none was the point.**
  *Lesson worth keeping: the hand-written file list was the same mistake one level up — a sweep*
  *scoped to where I remembered the bug being rather than to where the class lives.*

- **2026-08-03 — the ending reached the whole valley except the building the game is about.** Axis:
  world/place, back on content after a correctness pass. Two passes ago the ending was a wall; one
  pass fixed it in **paperwork** and the next fixed it with **ground** — and both pieces of ground
  were outdoors. Counted after: of the six tower rooms, one carried a single post-ending flourish
  and **no room carried a post-ending node, station, apply target or warp.**
  ***The design is the story bible's rather than mine.*** The ending's thesis is that a tower *used*
  is a different thing from a tower *run*, and the last commission buys **a second alchemist** — a
  stipend, a reference shelf, and a formula book allowed to be wrong in the margins. So what changes
  in the tower is that somebody else works there, and both routes are that person's leavings.
  **The Second Bench** (tower_entry): **Firsthand Dross**, what cools in the new hand's discard tray
  overnight — half reacted, abandoned, and *not yours*, because you stopped making this particular
  mistake a long time ago. **The Copying Table** (archive_floor): **Margin Ink**, lifted off a page
  the copyist got wrong, under the same raking light Ione reads pressure by.
  Both feed **Second-Draft Tonic** at the *entry cauldron* — the plainest bench in the building, and
  the reason is in the recipe: dross is half a reaction that stopped, so it wants continuing rather
  than starting. A restorative made **entirely of two people's mistakes**, which is the argument
  itself: the tower now employs enough people for being wrong to be affordable. Ione buys it for the
  shelf rather than the room.
  Journal beat `a_second_hand_in_the_room`, deliberately the answer to `the_previous_hand` — the
  previous hand took eleven months out of the record so nobody would find the working; the second
  hand leaves theirs in a tray, dated, face up, and the first thing anybody did with it was make
  something out of it. Ione and the Crow each have a line. The Crow's: *"They leave the pot cold and
  the slate written up. You did neither for a year and a half. I am not making a point. I am simply
  the only one here who watched both."*
  *Guard:* `the_rooms_change_after_the_ending_and_not_only_the_valley`, deriving "a room the player
  works in" from where the **stations** are — the same derivation the flourish guard uses, so the
  valley's outdoor routes correctly do not count. Verified by ignoring the new nodes: it names one
  room, `tower_entry`.
  ***The placement lesson, for the third time in this loop.*** The first draft dropped a dross tray
  50px from the Crow and an ink sheet 60px from Ione, so the NPC prompt won and neither node could
  be gathered at all. Nothing in the data says a node and a person occupy the same square; only the
  capture does. **Check new nodes against the NPC schedule for the hours they spawn.**
  Two existing guards bit on the way: both route descriptions overran the journal pane by ~30
  characters, and the art manifest caught three missing icons.
  **25 → 27 routes, 89 → 93 nodes, 173 → 176 items, 63 → 64 recipes, 103 → 104 quests; 205 tests.**
  `screenshots/hud/entry_second_bench.png`, `archive_copying_table.png`.

- **2026-08-03 — fourteen thousand characters of the player's own record, and five of them readable.**
  Axis: making authored content visible, which is the journal pass one system over. Went looking for
  a content slice, measured the shop shelf and the request pacing curve and found both healthy, then
  counted what the game *writes down*: **54 journal beats** from quests, recipe discoveries and
  apply targets, averaging **244 characters** and running to 413. Then counted the readers. The
  Notes tab drew `.rev().take(5)`; the archive timeline `.rev().take(7)`. **There is no third
  reader.** Everything older than the last five entries was written into the player's journal and
  then permanently out of reach — in a game whose scope note says twenty to twenty-five hours, act
  one is unreadable by act three.
  ***And the five it did show were broken three ways, all visible in one capture.*** The row advance
  was a fixed 74px while the text was laid out to its real wrapped height, so three- and four-line
  beats overlapped each other. The section began at y+448 while the milestone rows above it ended at
  y+480, so the last milestone's detail was overprinted by the first note's title on every full
  record — in the shipped game. And the section had about 80px of panel for beats that need 140.
  The tab is two columns now: Active Work and Tower Milestones left, **The Record** right — titles
  newest first, the selected beat written out beneath them, "showing 49-54 of 54", walked with the
  keys the routes tab already binds and reset on a tab switch so the two lists do not inherit each
  other's position. Fifth use of the shared `visible_window_start`, same list-and-detail shape as the
  routes tab and the archive.
  *Two guards.* `every_recorded_note_can_be_read_again` walks a finished campaign's whole record and
  fails on any beat the tab can never select — against the old `take(5)` it names forty-nine.
  `the_longest_recorded_beat_fits_the_panel` does the layout arithmetic for every authored beat with
  the **renderer's own exported constants**, so a 500-character beat is a red test rather than a
  paragraph through the frame; verified by widening the row window until the longest beat overran.
  *Deletions:* `recent_journal_milestones` and `JournalMilestoneSummary` are gone. The archive keeps
  its own seven-line summary on purpose — that is a status panel, not a record, and it is right to
  be short.
  *Harness:* a `notes[:<index>]` scene, because a long record is the only state that shows whether
  the section copes, and no other route to one exists short of finishing the game.
  **208 tests.** `screenshots/hud/journal_notes.png`.
  ***Method note:*** this is the fifth instance of the capped-list family and the first where the
  data had **no other reader at all** — the earlier four were display bugs over data reachable some
  other way. Worth asking of any list: not "does it page" but "is there anywhere else this can be
  read".

- **2026-08-03 — the room whose entire purpose is growing grew nothing of its own.** Axis: world and
  ingredients, back on content after a UI pass. Counted exclusive gatherables per area, which is the
  measure the biome-signature passes used: containment 3, rune workshop 3, archive 5, observatory 4,
  entry lab 2 — **every room in the tower sheds something found nowhere else except the
  greenhouse**, which had three nodes on one route carrying sunleaf, whisper moss and a dew slug.
  It is the *first floor a player restores*, it holds four of the game's six planters, and
  everything in it could be picked in the plains.
  **The Glass Line** — the walk between the beds and the outer glazing — and its signature is the one
  thing a glasshouse actually is: **ground the weather does not reach**. *Barlight Fern* grows in the
  stripe of shade a glazing bar lays across the beds and nowhere else, so it has never been rained
  on: daylight only, because after dark there is no bar to be under, and otherwise free of season and
  weather, which nothing else in the game is. *Panewater Moss* lives on the inside of the glass off
  the house's own breath running back down it — mornings, autumn and winter, because by noon the
  panes are dry.
  ***The winter half is the strategic point.*** Winter is the leanest quarter by a wide margin — 55
  available nodes against 77 in autumn — and the greenhouse is now the one place that gets *better*
  when the valley goes quiet. Restoring the first floor buys a winter, which is a reason to restore
  it beyond getting a second bench.
  Both feed **Takehold Solution**: stand a cutting in it overnight and it roots, which is the
  difference between a herbalist who can give a plant away and one who can only lend it. Rowan buys
  it, and not for her own stock — so that a cutting handed to somebody who has never grown anything
  is a gift rather than a test. Her friendship gift has been cuttings since it was written.
  Beat `the_house_grows_its_own`; Brin and Rowan react. Brin's: thirty years up and down that walk
  with a barrow and he never once looked to his left, because it is a foot of ground between a bed
  and a wall and nothing is supposed to be there.
  *Guard:* `every_tower_floor_that_grows_anything_grows_something_of_its_own`, the building's
  counterpart to the wild-biome rule, which skips tower floors by name. A floor with no nodes is not
  covered — not every room has to be ground, but a room that grows things has to grow something the
  valley does not.
  ***A verification mistake worth writing down.*** My first sabotage run "passed", and I nearly
  recorded that as the guard being weak — the string replace had silently not matched after
  `cargo fmt` reflowed the code. Applied with an assertion on the match count, it fails and names
  the greenhouse. **A sabotage that passes is more likely to be a sabotage that did not land than a
  guard that does not work; assert the edit applied.**
  **27 → 28 routes, 93 → 96 nodes, 176 → 179 items, 64 → 65 recipes, 104 → 105 quests; 209 tests.**
  `screenshots/hud/greenhouse_glass_line.png`.

- **2026-08-05 — the last ten bottles finally reached the people their labels already named.**
  Axis: board demand, finishing the sinkless-output sweep. The greenhouse pass left **ten** ordinary
  recipe outputs wanted by nothing: seven at the entry cauldron, two at the cold bench and one at
  the reading bench. They have no shared tier or mechanic. They do share authored descriptions that
  already say who uses them — the road crews ask for Sluicewater Tonic by name, Wren wrote a margin
  note about Downwash Draught, and Lyra spent a season proving creatures do not look up when a
  Cloudfloor Lantern comes on. The missing content was not another recipe. It was letting those
  buyers place the order.
  Ten repeatable supply lines now do that, spread across seven townsfolk and gated where their use
  requires it: the cold bench, the pass, the second bench, a buyer's own arc, or simply enough work
  at the cauldron to make the request timely. They are filed together in `quests_board_supply.json`
  because they are ordinary stock rather than crises — somebody has run out, so the board asks for
  more. `every_plain_brew_has_somewhere_to_go` makes the result a rule: every normal recipe output
  must be named by a request, a reagent slot, a rune input or a gate. The morph and rune-output
  guards remain separate because those protect the rewards for different verbs.
  ***The economy tripwire fired.*** The board cycle had crept from 13,086 to 13,662 after the last
  two world passes; these ten orders take it to **16,006**, over the 15,300-coin endgame sink. The
  guard's own instruction is the right one: keep the demand and author another commission. **The
  Teaching Place** follows the second bench: eight Masterwork Second-Draft Tonics and **6,800 coins**
  turn its one-year stipend into a standing teaching line — wages, lodging, paper and a budget entry
  that comes back next year whoever sits there. It is the most expensive and deepest commission,
  asks for a mastered post-ending formula, and lands as a second working table in the entry lab.
  Elric writes the next year's line before this one is half spent; Ione adds a column for who made
  each correction. A tower that can teach its work is no longer one person's tower.
  **10 → 0 sinkless plain brews; 105 → 116 quests; board cycle 13,662 → 16,006; commission sink
  15,300 → 22,100; 209 → 210 tests.**

## Deferred (needs a new system; not for this loop)

- ~~Apply-potion-to-target flow (wilted plant, frightened creature, blocked path)~~ **Built
  2026-08-02, widened 2026-08-03.** It was a new verb, so it sat here; once built it became content
  like anything else — six targets across six areas now, all four effect kinds covered.
- World/character art pass and hand-authored ambient audio.
- ~~Post-ending sandbox~~ **Started 2026-08-03, and it never needed a new system.** This entry sat
  here for a week on the belief that "somewhere for the game to go after the epilogue" was
  machinery. It was one `#[serde(default)]` field letting a request wait on a journal beat, which is
  what every other gate in the game already reads. A commission, three standing orders, a fifth
  unsigned note and a flourish now come *after* `observatory_ending`. Still open as content: four
  post-ending requests is a coda rather than a sandbox, and nothing after the ending changes where
  the player can walk.
  **The walking half fixed 2026-08-03**: two routes, four nodes, two reagents, a recipe and a fifth
  standing order now wait on the ending — the seed year in the plains and the clear shelf on the
  lake. **And the tower half the same day**: the second bench and the copying table, so the building
  the game is about reopening changes too. Still open: no post-ending *station* or *apply target*
  exists, and the second bench the last commission pays for is drawn rather than usable.
