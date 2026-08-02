# TODO — Alchemy Tower

## Scope — decided

**Long tail. A finished product is 20–25 hours of play.** Everything below is
measured against that target; anything not listed here is considered done.

Where the content currently stands: 14 areas, 49 recipes across 6 benches, 9
townsfolk, 24 story-arc requests, 26 repeatable board orders, 11 biome
ingredient tables, plus an epilogue. That is the mid-game; the remaining work is
what makes it a 20–25 hour game rather than a well-furnished 8-hour one.

## Applied alchemy — the largest open gap

- ~~Drinking potions is dead code~~ **Fixed 2026-08-02.** `quick_potions` had
  been stubbed to `Vec::new()` in commit `a30bf77` (a screenshots commit),
  which made `consume_potion`/`apply_effect` unreachable and left the HUD belt
  permanently empty. The original body is restored and two tests in
  `state/gameplay_inventory_views.rs` now pin the belt contents and the
  drink-decrements-and-applies path so a future stub can't pass CI.
- With drinking restored, two of the four `EffectKind`s still do nothing:
  `vitality` starts at 100 and nothing in the game ever decrements it, so
  `Restore` (51 of 110 authored effect blocks) is a no-op; `Glow` only tints
  the player sprite because there is no darkness/visibility system. Effects
  need something to matter *against* — a vitality drain (sleep pressure,
  brewing cost) and a dark-hours gathering rule would activate both.
- Implement the apply-potion-to-target flow (wilted route plant, frightened
  creature, blocked path) on top of the existing `EffectKind` system. Delivery
  is still just handing a bottle to an NPC; blockers are collision and art only
  (`data/schema_render.rs`, `art/props.rs`), never something a brew can act on.
  Until this exists the game's stated premise is unexpressed.
- Once targets exist, gate two or three route/floor openings behind applying a
  potion rather than delivering one, so the mechanic is on the critical path.

## Unconnected systems — audit 2026-08-02

Straight bugs first, then mechanics that run but feed nothing, then authored
content with no destination.

### Bugs

- ~~The Southern Pass gate does not exist at runtime~~ **Fixed 2026-08-02.**
  `WarpDefinition` now carries `required_completed_quest`, and it is read by
  `warp_is_unlocked`/`can_unlock_warp`/`warp_progress_score` and surfaced in
  the requirement summary, so the switchback locks until
  `nightwatch_for_elric` is delivered, `restore_warp_route` fires, and the
  `pass_road_open` milestone behind three NPC lines is recorded. The struct
  took `deny_unknown_fields` so the next dropped gate fails to load rather
  than silently opening. Two new tests: `gameplay_warps` pins locked → quest →
  unlocked → milestone, and `a_story_gate_never_locks_away_its_own_key` walks
  the gate quest's ingredient tree so a future gate can't strand its own key
  on the far side (the pass carries five ingredients found nowhere else, which
  feed six brews).
- ~~The experiment log disagrees with the game about stability~~ **Fixed
  2026-08-02.** The rule was written out in three places and one copy left
  `!destabilized` off, so an overcharge collapse filed as a stable brew and —
  since `gameplay_memory_rebuild.rs` rebuilds potion memory from the log —
  survived a save/load as a success. There is now one definition:
  `alchemy::stable_brew` plus `BrewResolution::is_stable()`, used by the
  brewer, the log, the preview, and the result feedback. `brew_is_stable` on
  `GameplayState` is gone. A test in `gameplay_brew_records.rs` overcharges a
  clean recipe until it collapses and asserts both the log entry and the
  memory rebuilt from it; it fails against the old expression.
- ~~Planter tending is discarded at midnight~~ **Fixed 2026-08-02.** The two
  models are composed rather than one being dropped: elapsed time is the floor
  (a forgotten bed still comes good) and each day tended is worth a day on top,
  held in a new persisted `tended_days` so the rollover can no longer erase it.
  `planter_growth_days` is the single definition and the rollover now shares
  `planter_growth_target` instead of re-deriving it. Two further bugs fell out
  of writing the test: the first approach to a never-touched bed reported "no
  seed for this" while the player held one (the seed lookup read the *existing*
  entry, and a fresh bed has none), and a bed planted on day zero could never
  be tended, because `tended_day` initialises to 0 and day zero is also 0.

### Mechanics that run but connect to nothing

- ~~Rapport above FRIEND is a label~~ **Fixed 2026-08-02.** Board orders now
  carry `rapport_npc_id` — the townsperson whose work they serve, which the
  prose already named ("the infirmary", "the lamplighters", "the carters") —
  and delivering one awards them +1, so the repeatable layer finally feeds
  relationships. All 31 existing orders are assigned across the eight
  townsfolk. Requests can gate on `required_rapport_npc_id`/`required_rapport`,
  which is what CONFIDANT is now *for*; two confidant-only orders exist
  (Ione/`coldread_solution`, Wren/`keptwarm_tonic`, both drawn from the
  no-sink potion list). KIN stays honest: board rapport can carry the number
  to 9, so the top tier's label also requires the arc finished — a reliable
  supplier is a confidant, not kin. New `game_data_rapport_tests.rs` asserts
  every order names a real beneficiary and every standing gate can have its
  standing earned without already having it.
- ~~Wild variants are a journal cosmetic~~ **Fixed 2026-08-02.** A gathered
  variant now sticks to the stock it went into: `variant_stock`
  (item -> variant -> count, persisted) records which of the held units came up
  under the right sky, alongside the plain inventory count. `brew_ingredients`
  folds the best held variant into the reagent it stands in for and hands the
  *adjusted* items to `resolve_brew`, so quality, traits, elements, volatility
  and synthesis all pick the difference up without any of them knowing variants
  exist — every dead field went live at once. `sequence_matches` now reads the
  ingredients rather than looking them up by id, so a variant's bonus trait can
  satisfy a reagent-order token. Brewing spends the unit; the preview reads the
  same stock the bench will. Remaining gap: the *belt* still shows one stack per
  id, so the player cannot see or choose which units are variant-grade — the
  bench spends the best one automatically.
- ~~Quest quality gates check history, not the bottle~~ **Fixed 2026-08-02.**
  Bottles carry the quality and traits they were brewed at, in a persisted
  `bottle_stock` (item -> batches, worst first), and a request is now checked
  against `qualifying_bottle_count` — what is on the shelf — instead of
  `crafted_item_profiles`, which is a best-ever record. Delivery spends the
  *worst* bottle that still qualifies, so brewing well is not a tax. Bottles
  from anywhere but the bench (bought, gifted, granted) have no batch and count
  as a plain example of the item. All inventory removal now goes through one
  `take_from_inventory` choke point that reconciles the batch list, so a stale
  batch cannot outlive the bottle it described and re-grade its replacement —
  a lazy read-time trim missed exactly that case and the test caught it.
- ~~Quality never touches payment, rapport, or sell price~~ **Fixed
  2026-08-02**, on top of the above. `sell_price` scales by the band of the
  bottle a sale would actually part with (the worst held, matching the order
  `reconcile_bottle_stock` trims in, so clearing shelf space cannot cost the
  player their best work). Delivering returns the worst grade handed over, and
  beating a request's stated bar pays a quarter of the fee per band above it;
  a delivery two bands over — or any Masterwork against a stated bar — also
  earns +1 rapport with the giver on top of the usual +2. A request naming no
  band has nothing to beat and still pays flat. Remaining: NPC reaction lines
  do not vary with delivered quality, and the band multipliers are Rust
  constants (see the tuning-into-data item below).
- ~~The seventh brew — the one that flips "Mastered" — adds nothing~~ **Fixed
  2026-08-02.** All three ramps capped at six, one short of the step that names
  them: the quality bonus and the instability reduction now run to
  `MASTERED_BREW_COUNT`, and the extra bottle arrives at mastery rather than the
  brew before it. Mastery also earns what the code always said it meant — being
  able to make one thing the same way twice — as a floor: a mastered formula
  never scores below its own `minimum_quality`, so it cannot fail on quality
  however poor the reagents. Process and stability still apply.
  `worst_case_shelf_for_wren` is now gated on mastering `purified_draught_recipe`
  (its prose asked for exactly that reliability), so a story arc uses the gate
  rather than only the board and one warp; a progression test keeps at least one
  arc beat asking for it.

### Authored content with no destination

- **53 of 96 potions have no structural sink** — no quest, no board order, no
  rune input, no recipe use; sale and planter-mutation fuel only. The worst
  bench is `archive_reading_bench`: behind the deepest gate in the game, and
  five of its six outputs are vendor trash. 11 of 17 rune-bench outputs are
  requested by nothing, and no rune output feeds another rune recipe. When
  building the late-game recipe tier, route requests at these before adding
  new recipes.
- **Three relationship gifts are inert**: `starlight_shard` (Mira, FRIEND),
  `saltroad_amber` (Tarn, trusted), `stillwater_pearl` (Wren, trusted) are
  used by no recipe or quest and are also plain shop stock — the friendship
  payoff item is something the player could already buy. The other ten gifts
  are real recipe inputs; these three should be too.
- ~~36 of 160 NPC reaction lines can never be spoken~~ **Fixed 2026-08-02.**
  The selector rotates through earned-but-unsaid lines, earliest first, so a run
  of beats that came due together is worked through one conversation at a time;
  once everything is said the latest line stands as their current word. It still
  only moves forward — a line earned *after* later ones were already spoken is
  skipped rather than dragging the townsperson back. Advancing a conversation is
  what marks a line said (persisted `spoken_reactions`). Keyed on an FNV-1a hash
  of speaker + line, because seven reactions already share a speaker and order,
  and a narrative test keeps those hashes distinct. Against the old selector the
  new reachability test gives Ione 1 of her 25 lines.
- ~~The NPC schema's `dialogue_start/progress/complete` and the phase-1
  `active_request` strings are always overwritten~~ **Fixed 2026-08-02.** Two
  branches returned outright and swallowed everything below them. The
  town-recovery observation now fills the `complete` slot only when nothing
  warmer applies, and opens the conversation only when nothing is pending, so
  `post_help_relief` reaches all eight. The arc beat line takes `progress` and
  leaves the opener to the townsperson's own voice, which reaches
  `active_request` (was dead for seven) and Mira's `intro` (dead because her
  first errand is offered from the opening minute). `dialogue_complete` is now
  their settled word once their whole arc is finished. `dialogue_start` and
  `dialogue_progress` are deleted: an earlier, blunter draft of beats the
  `phase1_dialogue` block covers better, and no honest slot was left for them —
  the prose is in git history. A new `every_line_a_townsperson_has_is_reachable`
  walks each of them through the states the game puts them in and fails on any
  authored string nothing can say.
- ~~The ending shows 3 of 12 epilogue beats~~ **Fixed 2026-08-02.** The panel is
  a fixed box and cannot grow, so the epilogue is paged instead of truncated:
  the opening page keeps the fixed paragraph plus three beats, later pages carry
  four each (five overran by 134 characters — the existing char-budget test now
  runs over every page, and said so). Confirm turns the page and closes on the
  last; cancel still closes outright. Three tests: every page fits, a
  completionist hears every beat they earned, and a page index past the end
  clamps rather than showing an empty panel. Against the old single-page view
  the reachability test names the nine that were invisible.

### Dead data (low stakes, cheap deletes or one-line hookups)

- `ItemDefinition.source_conditions` (61 JSON occurrences), `RoomBonusDefinition.description`
  (19 authored, preview shows only Active/Inactive), item traits
  `spread`/`echo`/`delay` (in no trait set anywhere), `HabitatStateEntry.placed_day`,
  and `FieldJournalEntry`'s season/weather/time fields (written blank, read
  only by an unreachable migration).
- ~50 dead `ui_text.json` keys — superseded HUD/menu/gather strings, the four
  `effect_name_*` keys (no composer exists), and `brew_failure_heat_high`/
  `_stirs_high`, which contradict the current deliberate-overfire design.
- `input_bindings.json` declares `alchemy.heat` and `alchemy.fill_slots`
  labels that `AlchemyBindings` doesn't have; 20 bare-id world-node PNGs are
  generated but never loaded (every node uses a biome-suffixed sprite).
- Three recipe discovery milestones (`the_channels_hold`, `the_raking_light`,
  `the_fortnight`) have no reaction line — the "every moment gets remarked
  on" test iterates quest milestones but skips recipe discoveries.

## Core loop & alchemy

- Turn the unlogged-brew salvage into a discovery event. Salvage currently lands
  on fixed outputs (`alchemy/fallback.rs`) that the rune bench re-reads; a
  repeated combination that resolves stably should journal and celebrate itself
  as a formula the player found without a recipe. This is the one place the
  engine can still surprise someone who is not following instructions.
- Move the last tuning constants out of Rust into data: the rapport tiers
  (`FRIEND`/`CONFIDANT`/`KIN` in `state/gameplay_rapport.rs`) and the salvage
  quality curve in `alchemy/fallback.rs` are the remaining hardcoded balance.

## Long tail content

- Decide what the last third is *for* and build it: the ending overlay is
  dismissible and board orders continue, but nothing new opens after the
  epilogue. Standing contracts that escalate, a late-game coin/material sink, or
  a reason to keep restoring past the final gate. The sink gap is now measured:
  one-off income is ~5,000 coins plus ~3,100 per repeatable board cycle plus
  unbounded sales, against ~800 of shop stock and 250 of one-time warp tolls.
- Late-game recipe tier — the current 49 recipes bottom out well before 20 hours
  of brewing decisions. New ones should extend the trait/morph lattice rather
  than add flat variants.

## Story & world state

- Write the story bible locking the wizard's backstory, the failed intervention,
  the ecosystem model, and the act-by-act reveal order. The arcs were written
  ahead of it, and nine townsfolk now depend on staying consistent.
- Extend visible town-state change past the four hardcoded cases.
  `draw_phase1_story_flourishes_view` matches on area id in Rust for two areas;
  the gating data (`required_completed_quest`) is already used in twelve files,
  so the flourishes should be data-driven and cover every completed chain — a
  reopened stall, fuller beds, lit streets.

## Presentation

- World and character art pass. The art is procedurally generated
  (`tools/generate_art.py`) and the ornate HUD frames a placeholder world; that
  inversion is still the weakest first impression.
- Offer a quieter HUD option so the world reads as the visual star.
- Replace the procedural one-shots (`tools/generate_audio.py`) with
  hand-authored ambient audio and music.
