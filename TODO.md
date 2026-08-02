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

- **The Southern Pass gate does not exist at runtime.** `town_to_pass` in
  `town_square.json` sets `required_completed_quest: nightwatch_for_elric`,
  but `WarpDefinition` (`data/schema_world.rs:40`) has no such field — gather
  nodes have it, warps don't — so serde silently drops it and the pass is
  walkable from minute one (four ungated nodes, including the
  `coldiron_tincture` input). Because the warp is never locked,
  `restore_warp_route` never fires and the `pass_road_open` milestone is never
  recorded, killing its three NPC reaction lines and the authored
  `locked_note`. Add the field to the schema; consider `deny_unknown_fields`
  (no schema in `data/` uses it) so the next dropped gate fails loudly.
- **The experiment log disagrees with the game about stability.**
  `record_experiment_log` (`state/gameplay_brew_records.rs:51`) omits
  `!destabilized` from the `stable` flag that `brew_is_stable`
  (`state/gameplay_inventory.rs:8`) includes, so an overcharge collapse files
  as a stable brew in the archive — and `gameplay_memory_rebuild.rs` rebuilds
  potion memory from the log on load, so save/load flips recorded stability.
- **Planter tending is discarded at midnight.** `tend_or_report_planter`
  increments `growth_days`, then the day rollover (`gameplay_world.rs:74-95`)
  overwrites it with pure elapsed time. Tending only matters within the
  current day. Pick one model or compose them.

### Mechanics that run but connect to nothing

- **Rapport above FRIEND is a label.** `CONFIDANT_RAPPORT` and `KIN_RAPPORT`
  (`state/gameplay_rapport.rs`) are read only by the journal tier string;
  trusted gifts gate on arc completion, not rapport. Board orders never award
  rapport at all, so the repeatable layer is fully decoupled from
  relationships. Either give the two upper tiers an unlock each and let board
  deliveries earn +1, or drop the tiers.
- **Wild variants are a journal cosmetic.** Of the eight
  `WildVariantDefinition` fields, only `required_conditions`, `quality_bonus`,
  and `name` are read — and even `quality_bonus` never reaches brewing,
  because inventory is `BTreeMap<id, count>` with no per-instance quality;
  the only consumer is the journal's best-quality-seen line. All 40 variants'
  `bonus_traits`, `elements`, and synthesis bonuses are dead. The whole
  "gather in the right season/weather" loop currently changes a string.
- **Quest quality gates check history, not the bottle.** Delivery reads
  `best_quality_band` — the best ever brewed for that item id — so one
  Masterwork brew permanently satisfies the gate for every later Crude one,
  and quality never touches payment (flat `reward_coins`), rapport, or
  reactions. Sell price ignores quality and traits too.
- **The seventh brew — the one that flips "Mastered" — adds nothing.** The
  quality bonus caps at `min(6)*3` and the output bonus lands at 6, so the
  brew that opens the mastery gates is mechanically empty. No story-arc quest
  uses `required_mastered_recipe`; only one warp and three board orders do.

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
- **36 of 160 NPC reaction lines can never be spoken.**
  `npc_phase1_followup_line` takes `max_by_key(order)` over all *earned*
  reactions, and earning is monotonic — so any line sharing a gate with (or
  gated behind an ancestor of) a higher-order line is permanently shadowed.
  Worst: Ione's `first_rune_imbuing` lines at orders 72/74/82 all lose to 86.
  Either rotate through earned-but-unspoken lines or make the narrative tests
  assert reachability, not just existence.
- **The NPC schema's `dialogue_start/progress/complete` and the phase-1
  `active_request` strings are always overwritten** before display
  (`gameplay_npc_dialogue.rs` — every branch that would show them is
  preempted by quest lines or recovery observations), and `post_help_relief`
  is reachable for only 3 of 8 townsfolk. Dead words from nine authored NPCs.
- **The ending shows 3 of 12 epilogue beats, two of which are always the same
  two** — reaching the ending requires the milestones that earn beats 80 and
  100, so only one slot is ever contested and nine beats are invisible in a
  completionist run (`MAX_EPILOGUE_BEATS`, `gameplay_ending_overlay_view.rs`).

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
