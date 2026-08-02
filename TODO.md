# TODO — Alchemy Tower

## Scope — decided

**Long tail. A finished product is 20–25 hours of play.** Everything below is
measured against that target; anything not listed here is considered done.

Where the content currently stands (re-counted 2026-08-03; the old figures here
were four passes stale): 14 areas and 77 gather nodes, 165 items of which 101
are potions and 47 ingredients, 59 recipes across 5 benches plus 17 rune
patterns and 25 mutations, 9 townsfolk with 171 reaction lines, 24 story-arc
requests, 11 open board orders, 26 standing orders and 5 commissions, plus an
epilogue. That is the mid-game and most of the way into the last third; the
remaining work is what makes it a 20–25 hour game rather than a well-furnished
8-hour one.

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
- ~~`Glow` only tints the player sprite~~ **Fixed 2026-08-02.** Gathering during
  a dark hour now needs a light, and a lit brew is one. Twenty-one nodes carry a
  night window and seven appear *only* in the dark — including four on the
  observatory floor, so the endgame area now expects you to bring something that
  throws a light. Which windows count as dark is
  `config.balance.gathering.dark_time_windows` (a list, so evening can be added
  without touching Rust). Three tests: daylight needs no help and a glow potion
  buys the night shift; the dark-hours list is genuinely read; and nothing the
  glow potion itself requires is night-only, so the rule cannot bootstrap
  badly — `starlight_shard` is night-only but also shop stock, and the recipe's
  two reagents are day-gatherable.
  All four `EffectKind`s now do something.
- ~~Implement the apply-potion-to-target flow~~ **Done 2026-08-02.** Areas carry
  `apply_targets`: things a brew is poured *on* rather than drunk or handed
  over. Each names an effect kind and optionally a grade; treating one spends a
  qualifying bottle (worst acceptable first, the same courtesy delivery pays)
  and records journal milestones. Those milestones are deliberately the same
  currency every existing gate reads, so warps, stations and nodes can wait on a
  treated target with no new gating machinery. Three authored to the TODO's own
  examples: a stalled propagation bed (greenhouse), a startled moth roost
  (forest), and a slumped root wall across the upper switchback (pass). Targets
  draw as a pulsing ring from primitives — legible before there is art for them.
- ~~Gate route/floor openings behind applying a potion~~ **Done 2026-08-02.**
  The containment lift waits on the greenhouse bed being revived. Gather nodes
  gained `required_journal_milestone` — the same gate stations and warps already
  had — which is what lets a *treated* thing open ground: waking the slumped root
  wall puts three nodes on the bank above the upper switchback, and settling the
  moth roost makes it workable, two nodes that a panicking roost never allowed.
  All three targets now lead somewhere. `recordable_milestone_ids` knows targets
  are a fourth writer into the journal, and a new test asserts every target and
  every commission changes something beyond the journal.

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

- **48 of 101 potions have no structural sink** — no quest, no board order, no
  rune input, no recipe use; sale and planter-mutation fuel only. **Six routed
  2026-08-03** by the second-order tier, which is the mechanism this entry
  asked for: two archive outputs (`annotated_light`, `benchlight_solution`),
  two mid-bench restoratives (`hushwater_draught`, `leanaway_salve`) and two
  speed draughts including a *rune* output (`relay_draught`, `firstthaw_draught`)
  are now required reagents. That answers "no rune output feeds anything" —
  the rune floor's product is an input to the floor above it — but the tier
  also makes three new top-of-chain bottles whose only destination is sale, so
  the count moved by three rather than six. The next pass at this should be
  requests, not recipes: a board order or commission wanting a compound bottle.
  **Done 2026-08-03**, and it is now a rule rather than an intention: two
  commissions (Carry-Down, Long-Haul) and three standing orders (Shelf-Wide for
  the archive, Longheld for the infirmary, Double-Read for the survey — the
  first thing the survey commission asks for, so a commission finally has a
  downstream) route all five compound bottles.
  `the_late_tier_does_not_make_its_own_vendor_trash` fails if a second-order
  recipe's output is wanted by no request, no reagent slot and no rune pattern;
  a morph target deliberately does not count, since that is another way to make
  the thing rather than a reason to have one. **43 of 101 sinkless.**
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
- ~~`source_conditions` and `RoomBonusDefinition.description` are dead~~
  **Hooked up 2026-08-03, both kept rather than deleted.** The 61
  `source_conditions` strings are what a herb entry says *before* you have
  worked it out: the journal now shows the learned conditions when they are
  known exactly and this hearsay when they are not, so a seen-but-unlearned
  entry stops reading "the memory is still only a glimpse" and starts telling
  you when to go looking. The room-bonus descriptions (**5** authored, not 19 —
  the old count was wrong) head the bench overlay in place of the same
  "Select materials, set the process, then confirm." shown at every bench in
  the tower, which is the one thing a bench subtitle should not be on a floor
  whose whole point is that the room changes the brew.
  ***The hookup found a live bug underneath it.*** The herb detail box holds
  about four lines and every entry led with the item description, which wraps
  to three for two thirds of the shelf — so the gathering conditions ran down
  through the Tower Access panel and the "brews into" line fell off the bottom
  with no mark to say so. The entry is ordered by what it is consulted for now
  (conditions, uses, numbers, flavour last, and the flavour is cut to its
  opening sentence), block heights are checked before drawing rather than only
  block *starts*, and the shelf shows five rows instead of six.
  `every_herb_entry_gets_its_conditions_and_its_uses` walks all forty herbs in
  both states; at six rows it names Lowstar Ash and Washvein Crystal.
  The shared overlay subtitle also grew: it was a fixed 36px box, which was one
  line, and it now sizes to its text and wraps short of the close button.
  `screenshots/hud/journal_hearsay.png`, `compound_bench.png`.
- ~~Item traits `spread`/`echo`/`delay` have nothing asking for them~~ **Fixed
  2026-08-03, and the gap was wider than the entry said.** Those three traits sat
  on the three runes and on *nothing else*: every one of the 17 rune outputs
  carried no traits at all, so a bottle that had been through the deepest verb in
  the tower was, to every trait check in the game, indistinguishable from one
  that had not. Each output now carries the pattern its rune put into it
  (`spread`/`echo`/`delay`, and `pure` for the ward, which is the ward rune's own
  trait), which makes an imbued bottle both *deliverable against a trait gate*
  and *readable by a compound brew* when it is folded in.
  Demand to match: the two standing orders whose prose already described a
  pattern now ask for it (Wren's standing doses have to be the echoed one; the
  archive's tablewide reading has to be splashed), and a new order wants a
  keptback draught — held rather than spent — which routes another sinkless
  potion. `longhaul_draught_recipe` prefers `echo`, so folding the echo-imbued
  relay draught into it pays. **42 of 101 sinkless.**
  Two guards: `an_imbued_bottle_carries_the_pattern_it_was_given` (per rune, so
  a fifth rune is covered the day it is authored) and
  `every_rune_pattern_is_asked_for_by_something`.
  ***And a test was quietly lying.*** `reachable_traits`, which decides whether a
  request can be met at all, walked only *recipes* — so every bottle the rune
  floor makes looked traitless and a request for the pattern just imbued into it
  read as impossible. It reads the item's own authored traits now, which is what
  `plain_bottle_qualifies` has always checked a delivery against.
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

- ~~Decide what the last third is *for* and build it~~ **Answered and started
  2026-08-02.** The answer comes out of the story bible: the valley stops asking
  for emergencies and starts asking for standards, and the player funds them.
  **Commissions** are requests with a `coin_cost` — you pay in rather than being
  paid, the reward is a milestone and a changed valley rather than money. Three
  escalate: the winter stores (900, Tarn), the reading room (1,400, Ione), the
  standing survey (2,600, Lyra, gated behind the reading room). Each demands
  four to eight bottles off a deep bench at Excellent or Masterwork, which routes
  demand at the sinkless-potion list as that entry asks. The whole change was one
  schema field plus two lines of arithmetic, because the board flow already knew
  how to gate, accept, check quality, deliver and record.
  The sink now stands at 4,900 against 4,001 of one-off income, so coins are a
  decision again; two tests hold that ratio and the escalation. Each commission
  now visibly lands: the winter stores stack in the square, the reading room
  lights the archive's middle table, and the standing survey plants marked posts
  along the well row. Still open: only three commissions exist, which is a start
  on the last third rather than a last third.
- ~~Only three commissions exist~~ **Five, 2026-08-03**, and the two new ones are
  what the compound tier is *for*. **The Relief Post** (1,800, five Carry-Down
  Cordials, Wren, after the winter stores) puts a stretcher, a filled lamp and
  something that holds a person still at the head of the switchback — Wren
  costed it eleven years ago and was told the valley could not afford it.
  **The Standing Road** (3,400, six Long-Haul Draughts, Tarn, after the post)
  buys two carts a week both ways in weather, half of them under-loaded on
  purpose, because a road forgets a place that only sends for things. Both land
  visibly (`screenshots/hud/pass_relief_post.png`,
  `town_road_service.png`) and four townsfolk remark on them.
  The sink is **10,100** now against 4,001 of one-off quest income and 4,766 a
  full board cycle, so the last third is roughly a cycle and a half of standing
  work rather than a wall. Two chains escalate rather than one: stores → post →
  road, and reading room → survey.
  Board file split three ways on the way — `quests_board.json` was 876 lines and
  the cut is what it takes to be offered the work: the open board (11), the
  standing orders you have to have earned (25), and the commissions you pay
  into (5).
- ~~Late-game recipe tier~~ **Started 2026-08-02.** The tier is *second-order
  brewing*: a bench with `accepts_potions` takes finished bottles as reagents,
  which is both a new decision layer and the only structural sink the deep
  benches' outputs can have (nothing asks for a benchlight solution, so the way
  it stops being vendor trash is for something else to need one). The archive
  reading bench works this way, which its own milestone justifies — "the tower's
  later methods were more modular than the entry lab ever suggested". Two
  recipes so far, `double_read_solution` and `longheld_cordial`, each with three
  reagents, a three-step sequence and two morph branches, against a mid-game
  where 35 of 54 recipes have no branch at all and sequences are almost all two
  steps. Four previously sinkless potions are now required reagents. Two tests:
  a recipe may only ask for a bottle at a bench that takes bottles, and a
  second-order recipe must actually be deep rather than a flat variant wearing
  the label. Still open: two recipes is a proof, not a tier, and the balance has
  not been played — potions default to quality 20 with no traits or elements, so
  a compound brew leans on process bonuses and the catalyst to reach a band.
- ~~The bottle you pour in is worth nothing~~ **Fixed 2026-08-03**, which was the
  balance hole the entry above named. Every potion in the data leaves `quality`
  unset, so the schema default of 20 stood in for a Crude bottle and a Masterwork
  one alike and the tier's whole premise — brew the input well — bought exactly
  nothing. Bottles have carried their grade in `bottle_stock` since the quality
  work; `brew_ingredients` now folds the best held bottle into the reagent the
  same way it folds a wild variant, so quality, traits, preferred-trait matches
  and sequence tokens all pick it up without knowing bottles are graded. The
  brew spends *that* batch: `take_from_inventory` trims the worst, which is
  right for a sale and would have quietly kept the Masterwork the bench just
  poured. On spec the five compound recipes score 51–73 on plain bottles and
  90–100 on Masterwork ones. Elements are deliberately not folded — a batch
  records what a brew resolves, and a potion's element profile is authored.
  The materials list reads the poured grade rather than the item file's 20,
  because that decision has to be visible at the bench; `screenshots/hud/
  compound_bench.png` and a `compound` capture scene are the check. Five tests,
  including one that runs *every* second-order recipe to its own spec twice and
  fails if masterwork reagents do not beat plain ones.
- ~~Two recipes is a proof, not a tier~~ **Five now, 2026-08-03**, one per effect
  kind the bench lacked. **Shelf-Wide Reading** folds the two archive lights the
  audit called vendor trash over a mirror bead, and reads a rank of spines rather
  than a page — the dust says which volumes came off the shelf, so the shelves
  have been keeping the record he removed the whole time (Ione's line, journal
  beat). **Carry-Down Cordial** is a holding salve and a draught quiet enough to
  move somebody under: it treats the journey, not the injury, which is Wren's
  twenty-year complaint. **Long-Haul Draught** takes a *rune* output and a
  greenhouse draught whose faults are the same length and cancels one against the
  other. Filing fixed on the way: the restore and speed recipes are in
  `recipes_restore_archive_reading_bench.json` and
  `recipes_speed_archive_reading_bench.json` rather than bundled into the glow
  file, and `longheld_cordial` moved with them. Still open: the three new outputs
  have no destination but the counter (see the sinkless-potion entry).

## Story & world state

- ~~Write the story bible~~ **Done 2026-08-02** — `docs/story_bible.md`. It
  locks rather than invents: the arcs, reaction lines, journal beats and
  epilogue already commit to a specific history with specific numbers, and the
  document writes that down with the beat id behind every claim so a statement
  can be checked rather than trusted. Covers the wizard (sealed deliberately;
  eleven months of working notes; removed them *after* writing them so nobody
  would find the working and be persuaded by it; "not yet, then"), the failed
  intervention as a slow ward-draw that reads as ordinary bad luck, the
  ecosystem rule and its four load-bearing consequences, a timeline anchored on
  the numbers already in the text, the three acts as the brew gates already
  enforce them, a table of what each townsperson measures and puts down, and six
  writing rules. Two things are marked **OPEN — deliberately** and left that
  way: whether the wizard lives, and who sends the unsigned orders (the epilogue
  says outright that question "was never yours to close"). A test asserts every
  arc-carrying townsperson and every spine beat is named in the document — it
  found two gaps in my first draft, `containment_started` and Mayor Elric.
- ~~Extend visible town-state change past the four hardcoded cases~~ **Done
  2026-08-02.** Areas carry `flourishes`: an id, the beats that earn it (`after_
  any_completed_quest` / `after_any_journal_milestone` — lists, because the
  first one authored already needed an "or"), and a list of shapes. The renderer
  is a generic loop over rect/circle/line, with `pulse` on circles for
  lamplight; the `match` on area id is gone, and adding a flourish is an entry
  in an area file rather than a change in two Rust files. The original four are
  ported unchanged and five more added: lit streets on the outer road after the
  nightwatch, the well row gone quiet, Tarn's market stall reopened, fuller
  greenhouse beds once the stalled bed is treated, and the switchback clear once
  the root wall is. Nine flourishes across three areas. Two tests: every
  flourish waits on a quest or beat that really exists and draws something, and
  a floor on how many places the world changes at all.

## Presentation

- World and character art pass. **Ground floor done 2026-08-02.** The specific
  problem was narrower than "procedural art looks procedural":
  `generate_art.py` had hand-tuned treatments for six areas and an `else` branch
  drawing a uniform 96px grid of rounded rectangles, and *eight* areas fell into
  it — every tower floor, the town square, and the pass. That included
  `tower_entry` and `town_square`, the two rooms that are the whole first
  impression. Each of the eight now has a treatment about what the room is for:
  flagstones with a worn strip down the entry lab, cobbles and cart ruts in the
  square, bed rows under glazing bars, ward rings with drainage, the cut
  channels on the rune floor, shelf ranks around a clear reading floor, the lens
  ring and its chart lines, a switchback across scree. Captures in
  `screenshots/hud/`.
  Still open, and wanting an artist rather than another pass from me: characters
  are four-colour figures and props are primitives. No room reads as scaffolding
  any more, which is a different and lower bar than good.
- ~~Offer a quieter HUD option so the world reads as the visual star~~ **Done
  2026-08-02.** A Quiet HUD toggle in Settings. It keeps the four things a player
  acts on — vitality (it can end the working day), the clock (it decides whether
  ground is gatherable and when you collapse), the potion belt and the status
  strip — and drops the six that frame the picture or repeat the journal: title
  banner, minimap frame, side panel, control tags, coin chip, goal note. Which
  panels exist is one list rather than ten scattered conditionals, so the policy
  is testable: quiet must remove something and must never drop a load-bearing
  panel, and Full is the default. The capture harness takes a `+quiet` scene
  suffix; `screenshots/hud/full.png` and `quiet.png` are the comparison, and it
  is stark — three townsfolk, a market stall and the whole top-left of the square
  are behind panels in one and visible in the other.
- Replace the procedural one-shots with hand-authored ambient audio and music.
  **Silent moments fixed 2026-08-02.** Reading the code first turned up a
  structural gap underneath the stated one: all five existing sounds are
  *inputs* — footsteps, a pickup, a bench opening, a stir, a brew result — and
  everything the player works *towards* was silent. A beat recorded, a request
  delivered, a bank treated, a commission funded, a route opened, a day run out:
  each already raised a toast, so the moment was identified and timed, and made
  no noise. Four families added in the same procedural style: a small dry
  journal tick (it fires for every recorded moment, so it has to survive being
  heard hundreds of times), a warmer work-landed, a route-restored that rises
  rather than resolves, and a collapse that falls away unresolved. They queue in
  `runtime.pending_sounds` and the frame loop drains them, because the code that
  knows a moment happened is several modules from the code that owns the
  speakers — the visual feedbacks beside them already work this way.
  Still open, and genuinely wanting a composer: there is no ambient bed and no
  music at all. The one-shots are procedural and sound it.
