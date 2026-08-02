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
- ~~`Restore` is a no-op because nothing decrements vitality~~ **Fixed
  2026-08-02.** Vitality is the working day now: a brew costs 5, a gather 1.5,
  and running out carries you home at 10:00 having lost the morning — the same
  collapse the small hours already caused, reusing `handle_sleep_pressure`.
  Sleeping in a bed by choice gives the day back in full (100); being carried
  home gives 55, so there is a reason to stop. A full day buys 20 brews, or 10
  brews and 33 gathers; a healing draught buys 9 more brews. All four numbers
  are in `config.balance.vitality`. The HUD warns below 20 rather than letting
  the collapse arrive unannounced. Five tests, including the collapse end to end
  and that drinking in time keeps the day.
- `Glow` still only tints the player sprite, because there is no
  darkness/visibility system for it to matter against. A dark-hours gathering
  rule is the obvious counterpart to the vitality drain above — the night time
  window already exists and gather nodes already carry `time_windows`.
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
- ~~Three relationship gifts are inert~~ **Fixed 2026-08-02.** The "used by no
  recipe" half of this was wrong: all three are catalysts, and each is the *sole*
  supplier of its tag (`starlight` feeds 9 recipe/morph slots, `saltroad` and
  `stillwater` 3 each), so they cannot be removed from shops without starving
  those formulas. The true half was that the payoff was buyable. Each of the
  three relationships now gives a better, gift-only version instead —
  `counterkept_shard` (Mira), `elevenyear_amber` (Tarn's parting gift),
  `backshelf_pearl` (Wren) — same catalyst tags, higher quality and synthesis
  value, sold and gathered nowhere. A new test asserts no gift is a single unit
  of plain shop stock; it caught a fourth case the audit missed, Tarn's
  friendship myrrh, now given by the measure rather than the pinch he sells.
  `obtainable_item_ids` also learned that a gift is a way to obtain something —
  every previous gift doubled as stock or a gatherable, so nothing had noticed.
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

- ~~`HabitatStateEntry.placed_day` and `FieldJournalEntry`'s season/weather/time
  fields~~ **Deleted 2026-08-02.** Both confirmed write-only: the journal
  migration reads every other field and skips those three, and `placed_day` was
  set twice and read nowhere. Deleting `placed_day` also surfaced a regression
  the last pass introduced — the habitat borrow fix had it seeding
  `last_harvest_day` from the *stale* `placed_day` rather than today, so a
  re-stocked habitat would have carried the wrong harvest timer. Both now read
  the clock before taking the entry's borrow.
- Still dead, and worth a decision rather than a reflex delete:
  `ItemDefinition.source_conditions` (61 authored strings — "evening glades",
  "clear nights", "gentle capture only") is real player-facing information now
  that gathering under the right sky changes the brew, so the herb journal is a
  hookup candidate rather than a delete. `RoomBonusDefinition.description` (19
  authored, preview shows only Active/Inactive) is the same shape of question.
  Item traits `spread`/`echo`/`delay` *are* on items in `materials.json`; what
  they lack is any recipe asking for them, which is a content gap, not dead
  data.
- ~~~50 dead `ui_text.json` keys~~ **Fixed 2026-08-02.** 52 removed, and two
  tests keep the file honest: `every_line_of_copy_is_asked_for_by_something`
  scans the source for each key, and `composed_copy_keys_name_real_items` covers
  the `journal_herb_summary_`/`journal_potion_recap_` families, which are built
  from item ids at runtime and so never appear as literals. A first pass at the
  count was wrong in both directions — those two families are live, and the
  `statuses`/`prompts`/`overlays` sections are typed structs read by field
  rather than by string, so all 21 of their keys are used.
- ~~`input_bindings.json` orphan labels and 20 unused world-node PNGs~~ **Fixed
  2026-08-02.** `alchemy.heat`/`alchemy.fill_slots` had no struct field, so
  serde dropped them silently — the same shape as the Southern Pass gate. Both
  removed, and every bindings struct took `deny_unknown_fields` so the next one
  fails to load instead of looking configured. The 20 PNGs were generated
  because their `gatherables.json` entries claimed `icon_and_world_node` while
  every node overrides with a biome-suffixed sprite; those entries are
  `inventory_icon` now, so regeneration no longer recreates them (85 world
  sprites down to 65).
- ~~Three recipe discovery milestones have no reaction line~~ **Fixed
  2026-08-02.** `every_recorded_moment_gets_remarked_on_by_somebody` now chains
  recipe `discovery_milestones` alongside quest and spine beats, and named
  exactly those three. Brin remarks on the rune floor finally making something
  rather than mending it, Ione on a light that reads the dent instead of the ink
  (which is her whole arc), and Rowan on the first formula in the book the
  calendar can close.

## Core loop & alchemy

- ~~Turn the unlogged-brew salvage into a discovery event~~ **Done 2026-08-02.**
  An off-book mixture is remembered by signature (bench + sorted reagents;
  loading order is how you fill the pot, not what you made), and the third
  attempt that comes to anything journals it as a formula the player worked out
  rather than read, with a toast. Familiarity reaches the brewer through the
  existing `mastery_brews` parameter, which now means "how many times have you
  done this exact thing before" for both paths: `salvage_quality` lifts its cap
  by 6 and adds 3 per attempt, stopping at 4 so an off-book mixture never
  overtakes a written recipe. The bench says so too — a discovered mixture reads
  "your hands know the shape of it" instead of the no-recipe line. Four tests,
  including that a worked-out formula genuinely brews better than a blind
  attempt; the off-book pair is *found* rather than named, so a new recipe
  covering it cannot quietly turn these into tests of the written-recipe path.
- ~~Move the last tuning constants out of Rust into data~~ **Done 2026-08-02.**
  `config.balance` now holds the rapport tiers, the salvage curve (including the
  discovery threshold), and the quality-band value multipliers the sell-price
  work had left in Rust. Every block takes `deny_unknown_fields` and none takes
  a serde default: a tuning value nobody reads is worse than a missing one,
  because the file claims it is configured and the game ignores it. A test
  turns two of the knobs and asserts the brewer moves with them — the first
  version of it turned the salvage *cap* and proved nothing, because the mixture
  it used scores well under the ceiling.
  `MASTERED_BREW_COUNT` stays in Rust deliberately: `mastery_stage`'s match arms
  encode the same threshold, so it is a shape rather than a number.

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
