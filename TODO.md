# TODO — Alchemy Tower

## Scope — decided

**Long tail. A finished product is 20–25 hours of play.** Everything below is
measured against that target; anything not listed here is considered done.

Where the content currently stands (re-counted 2026-08-03, second count that
day — a week of iterations moves these fast enough that stale figures mislead):
14 areas, 85 gather nodes and 23 routes, 6 apply targets, 170 items of which 104
are potions and 49 ingredients, 62 recipes across 5 benches (with 37 morph
branches, and two benches that take finished bottles) plus 17 rune patterns and
28 mutations, 9 townsfolk with 181 reaction lines, 24 story-arc requests, 24 open
board orders, 34 standing orders, 5 unsigned notes, 3 post-ending orders and 6
commissions, plus an epilogue and work that continues past it. That is the mid-game and most of the way into the last third; the
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

### Applied alchemy, widened 2026-08-03

The verb shipped with exactly the three examples this file listed, and never
grew past them: **2 `Restore`, 1 `Misfire`, across 14 areas**. Two of the four
effect kinds had nothing anywhere in the world to pour them on, so a player who
brewed for glow or speed had no target for it — the same shape as the four dead
`EffectKind`s above, one layer out.

Three more authored, both missing kinds now covered and the count doubled:

- **The Drowned Gallery** (rock_fields, `Glow`, Fine or better) — a flooded
  working the quarry crews abandoned; a lit bottle poured on the water shows the
  seam. Opens **stillwater pearl** and **washvein crystal**.
- **The Closed Hedgerow** (north_plains, `Speed`, any grade) — a season's growth
  across the drovers' gap. Speed the hedge through to seed and it opens rather
  than thickens. Opens **driftseed**.
- **The Fogged Mirror** (observatory_floor, `Glow`, Excellent) — the endgame
  area's own target, and the only one that asks for the top grade. Opens
  **mirrorbead**.

Every target opens ground, which is the rule the first three set. Six targets,
six areas, all four effect kinds sinkable.

**And the reaction guard had never walked them.** The apply-target class was the
fourth journal writer, added on 2026-08-02, and
`every_recorded_moment_gets_remarked_on_by_somebody` only knew the other three —
so *no* target had ever been remarked on, including the original three, and
nothing said so. Extended, it named six unremarked beats at once. Six reaction
lines authored, one speaker each: Rowan on the bed, Lyra on the roost, Tarn on
the root wall, Brin on the gallery, Elric on the hedgerow, Ione on the mirror.
A second guard, `every_effect_a_bottle_can_carry_has_something_to_pour_it_on`,
keeps the next `EffectKind` from shipping with nowhere to go.

### The ground a target opens had no name, 2026-08-03

- ~~The five nodes the first apply-target pass unlocked have no route and no
  conditions~~ **Fixed 2026-08-03.** Two rules the whole world followed in one
  place each, which is why nothing had noticed either was a rule:
  **A node belongs to a route.** `route_id` is what the herb journal writes into
  `first_seen_route_id`/`learned_route_id`, so a node without one files the
  pickup as *"an unknown place"* — the player poured a brew on something, opened
  new ground, walked it, picked something, and the journal could not say where
  they had been.
  **No ground in this valley is available all the time.** Every node in the game
  constrains at least one of season, weather or time window, and 44 of them
  constrain all three, because deciding *when* to walk somewhere is the outer
  loop of this game.
  The exception on both counts was exactly the five nodes the first apply-target
  pass opened on 2026-08-02 — three on the re-seated bank above the switchback,
  two at the settled roost. (The three the *second* pass opened have routes and
  conditions, so this was one pass's blind spot rather than a habit.)
  Two routes authored, which also fixes the pass being eight nodes on one route:
  **The Upper Bank** — the bank that lay across the switchback a season ago is
  now the only south-facing soil above the treeline, and it hands you a
  different plant per season: driftbloom early, leanaway thrift through the
  heat, coldiron lichen once the cold is properly in. **The Settled Roost** —
  nothing there can be worked in daylight and, for twenty years, nothing there
  could be worked at all; after dark and in still air the roost comes down and
  stays down. Five nodes given seasons, hours and weather to match, and their
  notes rewritten so the prose and the conditions agree.
  `every_gather_node_has_a_place_and_a_season` is the guard, both halves
  verified separately — stripping the routes names all five, stripping only the
  conditions names the three on the bank. 21 → 23 routes.
  `screenshots/hud/journal_routes.png`.

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
  same stock the bench will.
- ~~Nothing on screen says which stacks hold a variant~~ **Fixed 2026-08-03**,
  six passes after the gap was written down. The bench spends the best unit
  automatically and the belt shows one stack per id, so the player was making
  the decision the whole system exists for — brew now or walk back out for a
  better strain — with no information at all.
  Two surfaces, because they answer different questions. The **materials list**
  marks a stack holding one and reads the *variant-adjusted* quality, which is
  the number the pot will actually get; that figure had been the plain data-file
  value even after the bottle work taught the same row to read a poured grade.
  The **journal** entry says what is in the bag rather than only what was once
  seen: "Noted strain: Static Arcane Dust — 2 in the bag".
  One guard covering both, verified by putting the plain title back.
  `screenshots/hud/bench_variants.png`, `journal_hearsay.png`. Both capture
  scenes now seed a held variant, because a bench with none in the bag proves
  nothing about a marker that only appears when there is one.
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
  band has nothing to beat and still pays flat. Remaining: the band multipliers
  are Rust constants (see the tuning-into-data item below).
- ~~Nobody ever says anything about good work~~ **Fixed 2026-08-03.** Quality
  had paid coin and standing since that pass and not one townsperson remarked on
  it, which for a game about being the valley's alchemist is the wrong way
  round. Eight authored `exceptional_delivery_line`s, one each and in their own
  idiom — Rowan's is "You over-made it. I noticed, obviously. That is the whole
  of what I am going to say about it." — raised as a banner when a delivery
  beats the stated bar by two grades or lands a Masterwork against any bar. It
  stays silent for a bottle that merely cleared the bar, because praise for
  everything is praise for nothing.
  The board path also awards the exceptional +1 rapport now: the arc path has
  since the quality pass and the board quietly did not, so the same bottle was
  worth more standing depending on which counter it crossed.
  Three tests: the remark reaches the screen and is silent when unearned,
  everyone who can receive a delivery has a line, and every line fits the
  banner — which needed the banner to grow to two lines
  (`screenshots/hud/event_toasts.png`).
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

### The planter, read 2026-08-03

- ~~A bed spent whichever bottle sorted first alphabetically~~ **Fixed
  2026-08-03.** A mutation asks for an effect *kind* rather than a named brew —
  the one place in the game that already worked that way — so any glow bottle in
  the bag will do. `planter_mutation_candidate` walked `self.inventory`, which
  is a `BTreeMap`, and took the first match: **planting a bed could spend a
  284-coin Heldstar Vigil because `h` sorts before `k`**, with a 22-coin
  Kindling Tonic sitting beside it. Every other spend in the game already knows
  better — a delivery hands over the worst bottle that qualifies, a sale parts
  with the worst held, the bench pours the best deliberately. It takes the
  cheapest thing that fits now, which is also **what the sinkless tail is for**:
  the four salvage bottles are the cheapest in the game, so a bed reaches for a
  failed brew before a good one.
  The banner names the bottle as well as the bed: a mutation costs a brew, and
  while the toast channel was dead nobody could see anything had been spent.
  *Content:* the **murky concoction** — two coins, the game's only `misfire`
  brew, wanted by nothing — now has three formulas of its own. A bed prefers a
  proper brew and takes the unlabelled one only when that is all there is, which
  is what data order in `mutation_formulas_for_seed` means; the strain it throws
  is faster to come up and no more of it. 25 → 28 formulas.
  Four tests, including the cheapest-bottle rule verified against the old
  alphabetical pick, and the pair it uses is *found* in the data rather than
  named, so a re-priced bottle cannot quietly turn it into a test of nothing.

### The archive console, read 2026-08-03

- ~~Disassembly was a reagent printer~~ **Fixed 2026-08-03.** Taking a bottle
  apart returned **every ingredient at full amount**, and nine recipes brew more
  than one bottle at a time: `coldiron_tincture` and `shiftlong_tonic` turn
  **three reagents into three bottles**, each handing back all three. Six free
  reagents a brew, no travel, no season to wait for, repeatable forever — in a
  game whose entire outer loop is deciding where to walk and when.
  A bottle gives back its **share of the pour** now — the ingredient amount over
  the recipe's output count, rounded down — so a whole batch cannot yield more
  than the batch cost. Six batch recipes divide away to nothing and are no
  longer offered rather than eating a bottle for an empty hand; three return a
  partial share; every one-bottle recipe is untouched.
  ***The mastery bottle is deliberately not in the divisor, and the arithmetic
  is why.*** A brew costs 5 vitality and a gather 1.5, so gathering yields ~3.3
  units per 5 vitality. A mastered one-bottle recipe hands back two bottles'
  worth for the same 5 vitality — 2 or 3 units — which is *worse* than walking
  out and picking them. Counting mastery in the divisor rounds every ordinary
  recipe's return to zero and kills the feature to close a hole that is not
  open. (I tried it that way first; the existing test named the cost.)
  The panel lied too: it listed the *authored* ingredients as "Recovered
  Inputs". It shows the share now, and the help line says the rule.
  Two guards — a full brew's bottles can never yield more than the brew cost,
  and nothing is offered that gives nothing back. `screenshots/hud/
  archive_disassembly.png`.
- ~~Duplication burned the gift catalyst first~~ **Fixed 2026-08-03.**
  `duplication_catalyst_item_id` took the **highest-quality** starlight catalyst
  held, and duplication reads nothing from a catalyst's quality — it is spent,
  not measured. So the console reached past a 24-coin shard sold at two counters
  for Mira's `counterkept_shard`, a friendship gift sold and gathered nowhere,
  and burned it for exactly the same result. It takes the least valuable thing
  that qualifies now, which is the same answer the planter needed the same day.

### The counters, read 2026-08-03

- ~~Two ways to print coins, one cause~~ **Fixed 2026-08-03.** The apothecary
  sold a starlight shard for **28** and bought it back for **33**: five coins a
  click, unbounded, no travel and no cost but the keypress. And the archive
  console would duplicate Tarn's `elevenyear_amber` for **360** and a counter
  would pay **640** for the copy — 280 a click, on a gift item, forever.
  `saltroad_amber` and `backshelf_pearl` were the same shape at smaller margins.
  One cause: **the quality-band multipliers were being applied to raw materials
  as well as to brews.** A bottle's grade is a fact about the work that went
  into it, which is what the multipliers are for; a herb's or a catalyst's
  `quality` is potency, authored once and identical for every unit, so running
  it through a *craft* multiplier expressed nothing and inflated everything —
  the amber is quality 82, which paid 200%. `quality_adjusted_value` scales
  bottles only now. Brewing well is still worth more at a counter; picking up
  the same shard twice is not.
  The first fix was to reprice the shard, and it was wrong: it patched one line
  and left the rule unguarded, so it was reverted once the cause was found and
  the authored 28/34 stands.
  Two guards, both verified against the old multiplier and each naming its own
  cases: **no counter pays more for a thing than it charges** (all 20 stocked
  lines) and **a copy never sells for more than it cost to make** (every
  duplicable item).

### The last biome at the floor, 2026-08-03

- ~~The rainforest was a corridor~~ **Fixed 2026-08-03.** Counting *exclusive*
  gatherables per biome rather than nodes: lake 4, forest 4, quarry 5, pass 5,
  desert 4, plains 2 — and the **rainforest 1**, on **one route**, six nodes,
  five of which grow closer to home. It was somewhere to walk through.
  ***The signature was already written and nothing had been built on it.*** The
  route text has said since it was authored that "the canopy holds the rain long
  after it has stopped falling — the mist under it is not weather, it is the
  ceiling draining, and it keeps going on days the sky is clear." So: **the
  rainforest is the one place where the weather underneath is not the weather
  above**, and the new content is gated *inversely* to every other rain thing.
  **Heldrain Bead** is water standing in a leaf axil that has touched nothing
  since it was cloud — not soil, not stone, not the tower's pipes — and it is
  picked on **clear mornings**, because that is when the ceiling is still
  letting go. **Stranglerfig Sap** runs off the host trunk in the hour the heat
  leaves, out of a tree the fig spent twelve years growing down. A second route,
  **The Drip Line**, and four nodes: 6 → 10 nodes, 1 → 2 routes, 1 → 3
  exclusives.
  Both feed **Truemeasure Tonic** at the cold bench — a dose that restores
  middlingly and does it *identically every time*, which is what Lyra's standing
  survey needs, because a keeper cannot compare this season's animals to last
  season's if the thing she gave them has drifted. It has a standing order of
  its own, so the new bottle is not vendor trash the day it ships.
  Guard: `every_wild_biome_is_a_source_of_something` — two exclusives, a floor
  rather than a target. `north_plains` sits on the floor deliberately (starter
  ground is meant to be shared) and the town square is excluded by name.
  Verified by removing the two new herbs; it names the rainforest.
  `screenshots/hud/rainforest_dripline.png`.
- ~~"New journal note: ." three times over~~ **Fixed 2026-08-03**, found in that
  capture. `preview_area` seeds gate milestones with empty titles, and since the
  banners started drawing, that reads as three untitled notes on screen. An
  untitled beat is a capture scene seeding a gate rather than something the
  player did, so it no longer raises one — and a guard now asserts every
  *authored* beat has a title and text, which is what makes that rule safe.

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
  **Down to 29 of 102 on 2026-08-03** — see the precision-layer entry below,
  which cleared the largest single block of it. What is left, measured rather
  than estimated: **16** plain recipe outputs, **9** rune outputs, and **4**
  bottles a recipe and a branch both make. The rune nine are the next coherent
  group — the rune floor makes seventeen things and nine of them are wanted by
  nothing, which is the same shape one verb over.
  **And 20 of 102 the same day**, the rune nine being the group above. What is
  left is one flat class: **16** plain recipe outputs plus **4** that a recipe
  and a branch both make, spread across five benches with no shared cause.
  That is a tail rather than a hole, and the next pass at it should probably be
  *recipes* — several of the twenty are outputs nothing else in their own bench's
  chain consumes.

### The rune floor's first eight imbuings were all wasted, 2026-08-03

- ~~Nine of seventeen imbuings make something nobody wants~~ **Fixed
  2026-08-03.** The rune floor's whole verb is *take a bottle you can already
  make and rework it into something else*, and nine of its seventeen outputs
  were wanted by nothing at all.
  **And they were the early nine.** The inputs are the glow potion, the healing
  draught, the lantern draught, calmleaf, the verdant restorative and the
  stamina tonic — everything a player learns in act one — plus two salvage
  bottles and the top of the whole chain. So the most natural first use of a
  newly opened floor, *improve the thing I am already good at*, paid out in
  vendor trash every single time, while the eight imbuings the valley did want
  all sat at the far end of the game.
  Nine orders, one buyer each and drawn from what the imbuing actually does:
  a **splashed glow** lights the whole cut at once instead of eleven people
  passing one lamp (Brin); an **echoed healing draught** arrives twice for
  patients who will not sit still for the second (Mira); a **delayed stamina
  tonic** is drunk at the bottom of the pass and works at the top (Tarn); a
  **beacon burst** is not for walking by, it is for being found (Elric); a
  **warded calmleaf** holds a shallow sleep to morning rather than going deeper,
  which is what Wren actually asks for; a **second-spring tonic** feeds a bed
  again in the second week, when a bed decides whether it took (Rowan); three
  **leakfire flares** because the crews would rather have three bad ones than
  one good one; two **second readings** because Ione will not accept a reading
  she cannot check against itself; and one **heldstar vigil**, the deepest thing
  the tower makes, burning dusk to dawn unattended on the hall steps the nights
  the pass is shut.
  Two of those route *salvage* bottles — a failed glow brew and the unnamed
  murky mixture — so the failure tail finally pays at the top of the tower.
  `every_imbuing_the_rune_floor_makes_is_wanted_by_something` is the guard, the
  rune analogue of the morph one, verified by deleting the nine: it names all
  nine. `screenshots/hud/rune_workbench.png`.

### The precision layer had no buyer, 2026-08-03

- ~~Hitting a morph branch pays out in vendor trash~~ **Fixed 2026-08-03.** A
  branch is the hardest thing the brewing system asks for — the quality bar, the
  exact heat and stir count, the timing word, often a named catalyst, sometimes
  a reagent order and the room bonus, *all at once*. There are 33 branches
  making 29 distinct bottles, and **13 of those bottles were wanted by nothing
  at all**: no request, no reagent slot, no rune pattern. They were 13 of the 42
  sinkless potions, and they were the reward for the deepest verb in the tower.
  **Thirteen orders, and every one of them was already written.** Each of these
  bottles carries a description that names its own buyer and had never been
  asked to — "the only property the crews actually asked about" (banked
  cinderlight), "Ione has one on a shelf and has not written a label for it"
  (hollowroot reliquary), "the desert's own lantern" (nightglass), "the plainest
  recipe in the book, taken as far as it goes" (clearspring). So the authoring
  was demand for prose that already existed, not new fiction.
  Seven go on the **open board**, gated only on brew count (16–26), because the
  first time a player hits a branch should be the first time somebody wants what
  it made. Six are **standing orders** off the deep benches and the rune floor,
  gated on the arc that earns them. The thirteenth is the **fourth unsigned
  note**: a wildfire draught, "hot and past where the recipe stops", at roughly
  three times what anyone in the valley would pay for a thing that should be
  handed over with a warning. No address on it. There never is.
  Spread across all eight townsfolk rather than pooling on the two who buy the
  most — Mira, Brin ×2, Tarn ×3, Elric ×2, Lyra, Ione ×2, Wren.
  Economy after: one-off income 4,621, a full repeatable cycle 7,890, commission
  sink 10,100 — still more to fund than to earn in a cycle.
  `a_morph_branch_pays_out_in_something_somebody_wants` is the guard, verified
  by deleting the thirteen: it names every one of them.
  *Filing note:* `quests_board_standing.json` is at 776 lines and wants a split
  before it takes another order.
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

### Dead code found 2026-08-03

- ~~Every event toast in the game was invisible~~ **Fixed 2026-08-03.** This is
  the `quick_potions` shape again, and bigger: `push_event_toast_with_icon` took
  `_text`, `_color`, `_icon_key` and pushed a struct holding **nothing but a
  countdown**, so the entire payoff channel — a beat recorded, a request
  delivered, a route reopened, a formula worked out, a commission funded, a
  mastery reached, plus the **whole tutorial hint layer** — was formatted and
  dropped on arrival. Six icons were generated for it under
  `assets/generated/ui/toasts/` and never loaded, because the two `ui_art.json`
  keys naming them (`toast_icons`, `default_toast_icon`) had no struct field and
  serde discarded them in silence: the third instance of that exact failure
  after the Southern Pass gate and the input-binding labels.
  The toast now carries its text, colour and icon key; the icons are in the
  texture manifest; and the HUD draws the stack above the status strip, newest
  nearest the eye, capped at three and fading out. Quiet mode keeps them —
  they are the payoff channel, not framing. `UiArtCatalog` took
  `deny_unknown_fields`, so the next dropped key fails to load rather than
  looking configured.
  Three tests and a `toasts` capture scene (`screenshots/hud/event_toasts.png`),
  which exists because a banner lasts two seconds and there is no catching one
  by hand.
- ~~The tutorial layer had never been read by anybody~~ **Fixed 2026-08-03**, the
  same day it became visible. Three defects, all of which cost nothing while the
  banners were dead and were live the moment they were not:
  **the shown-flags lived in runtime state**, which is rebuilt on load, so the
  crow's introduction, the save hint and the journal hint replayed every time a
  save was opened — they are `progression.shown_tutorial_hints` now, saved with
  everything else; **`tutorial_potions` was formatted with a `{quick_potions}`
  substitution its copy had no placeholder for**, so the belt keys were looked
  up, joined and dropped, which is the banner bug one layer down; and **three
  hints spelled keys out as literals** ("Press J", "with E") while
  `input_bindings.json` owns them and the rest of the HUD reads it. Six lines
  rewritten to ask for the binding.
  The selector is a list rather than a ladder of `if`s, so a guard can walk it.
  Three tests: a hint that names a key names the *bound* one — checked against
  the copy rather than the rendered string, because "Press J" contains the bound
  key by coincidence — every hint has words behind it with no unfilled
  placeholder, and a hint already seen does not come back after a load.
  The banner cap went from two lines to three, because the crow's opening
  instruction is the longest thing the channel carries and was being cut
  mid-sentence; the fit guard now walks the hints as well as the townsfolk's
  remarks. `screenshots/hud/opening_hint.png`.
  ***Harness note:*** a headless capture runs about twenty times faster than
  real time, and hint pacing is real-time. The default 150 frames photographs
  roughly a tenth of a second, so anything on a timer needs frame counts in the
  thousands — this one took 3,000.

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

### The game's own record of what you did was write-only past the tail, 2026-08-03

- ~~The journal shows the last five beats and no way back~~ **Fixed
  2026-08-03.** Measured: the game authors **54 journal beats** across quests,
  recipe discoveries and apply targets, averaging **244 characters** and running
  to 413 — about fourteen thousand characters of prose written specifically to
  be the player's record of their own campaign. The Notes tab drew
  `.rev().take(5)` and the archive console's timeline `.rev().take(7)`, and
  **those were the only two readers.** Everything older than the last five
  entries was written into the player's journal and then permanently out of
  reach. In a game whose scope note says twenty to twenty-five hours, act one
  was unreadable by act three.
  ***And the five it did show were broken as well.*** The renderer advanced a
  fixed 74px per entry while `draw_wrapped_text` laid the text out to its real
  height, so beats that wrap to three or four lines overlapped each other; the
  section started at y+448 while the milestone rows above it ended at y+480, so
  the last milestone's detail was overprinted by the first note's title on every
  full record; and the whole section had about 80px of a panel whose longest
  beat needs 140. The capture shows all three at once.
  The tab is two columns now — Active Work and Tower Milestones on the left, and
  **The Record** on the right: titles listed newest first, the selected beat
  written out beneath them, "showing 49-54 of 54", walked with the keys the
  routes tab already binds. Same list-and-detail shape as the routes tab and the
  archive console, and the fifth use of the shared `visible_window_start`.
  Two guards. `every_recorded_note_can_be_read_again` walks a finished
  campaign's whole record and fails on any beat the tab can never select again —
  against the old `take(5)` it names forty-nine. `the_longest_recorded_beat_fits
  _the_panel` does the layout arithmetic with the renderer's **own exported
  constants** for every authored beat, so a 500-character beat is a red test
  rather than a paragraph running through the panel frame.
  `recent_journal_milestones` and `JournalMilestoneSummary` are deleted; the
  archive keeps its own seven-line summary, which is a status panel rather than
  a record and is right to be short. `screenshots/hud/journal_notes.png`, and
  the harness took a `notes[:<index>]` scene, because a long record is the only
  state that shows whether the section copes.

### The one place the ending changed nothing was the tower, 2026-08-03

- ~~Post-ending ground is all outdoors~~ **Fixed 2026-08-03.** The pass that
  answered "the ending is a wall" answered it in paperwork; the pass after that
  answered it with ground — and both pieces of ground were **outdoors**, in the
  plains and on the lake shore. Measured afterwards: of the six tower rooms, one
  carried a single post-ending flourish and **no room carried a post-ending
  node, station, apply target or warp**. The building the whole game is about
  reopening was the last place in the valley the ending did not touch.
  The answer is the story bible's, not mine. The ending's thesis is that a tower
  *used* is a different thing from a tower *run*, and the last commission buys
  **a second alchemist**: a stipend, a reference shelf, and a formula book that
  is allowed to be wrong in the margins. So what changes in the tower after the
  ending is that **somebody else works there**, and both new routes are that
  person's leavings:
  **The Second Bench** (tower_entry) — **Firsthand Dross**, what cools in the
  new hand's discard tray overnight. A mixture that went nowhere, half reacted
  and abandoned, and *not yours*: you stopped making this particular mistake a
  long time ago. Somebody is being paid to be allowed to make it.
  **The Copying Table** (archive_floor) — **Margin Ink**, lifted off a page the
  copyist got wrong, scraped back under the same raking light Ione reads
  pressure by. The reference shelf is being made one corrected page at a time,
  and the corrections are being kept.
  Both feed **Second-Draft Tonic** at the entry cauldron — the plainest bench in
  the building, because dross is half a reaction that stopped and needs
  continuing rather than starting. It is a brew made **entirely of two people's
  mistakes**, which is the whole argument: the tower now employs enough people
  for being wrong to be affordable. Ione's post-ending order is for the shelf
  rather than the room — she spent nine years on a record with eleven months cut
  out of it by somebody who did not want their working found.
  Journal beat `a_second_hand_in_the_room`, deliberately answering
  `the_previous_hand`: the previous hand took their working out of the record;
  the second hand leaves theirs in a tray, dated, face up. Ione and the Crow
  each have a word — the Crow's is "They leave the pot cold and the slate
  written up. You did neither for a year and a half."
  `the_rooms_change_after_the_ending_and_not_only_the_valley` is the guard, and
  it derives "a room the player works in" from where the stations are, exactly
  as the flourish guard does — so the valley's outdoor routes correctly do not
  count and a bench on a new floor is covered the day it is placed. Verified by
  ignoring the new nodes: it names 1 room.
  ***Placement lesson, again.*** The first draft put a dross tray 50px from the
  Crow and the ink sheet 60px from Ione, so the NPC prompt won and the node
  could not be gathered at all. Nothing in the data says a node and a person
  are in the same place; only the capture does.
  **25 → 27 routes; 89 → 93 nodes; 173 → 176 items; 63 → 64 recipes.**
  `screenshots/hud/entry_second_bench.png`, `archive_copying_table.png`.

### The rest of the sweep for this project's most repeated bug, 2026-08-03

- ~~`deny_unknown_fields` had only ever been added to the one struct that had
  just failed~~ **Swept 2026-08-03. A correctness pass, not a content one,
  saying so plainly.** The single most repeated failure in this project is **a
  key in a data file that no struct claims**: serde drops it in silence, so the
  file reads as configured and the game ignores it, and nothing says so. It has
  landed at least four times — the Southern Pass's `required_completed_quest`,
  which meant the gate the whole southern half of the map sits behind did not
  exist at runtime; `alchemy.heat` and `alchemy.fill_slots` in the input
  bindings; `toast_icons`/`default_toast_icon` in `ui_art.json`, which is why
  six generated icons sat unloaded for the life of the project; and three
  duplicate entries in the narrative milestone block.
  Each time the fix was the attribute on **that one struct**, and nobody went
  looking for the rest — despite this project's own method note saying that when
  a bug class repeats you should grep for the others. Counted: **11 of 48
  deserialised structs were strict. 37 were not**, including `ItemDefinition`,
  `RecipeDefinition`, `AreaDefinition`, `StationDefinition`, `NpcDefinition`,
  `GatherNodeDefinition`, and the nine file-envelope structs where a stray
  *top-level* key would vanish.
  All 37 are strict now. **The sweep found no dead keys in the current data**,
  which is the good outcome and worth saying rather than dressing up: the value
  is that the fifth instance becomes a red test instead of a mystery.
  Two exclusions, deliberate: `save_models.rs`, `save_memory_models.rs` and
  `schema_progression.rs` stay lenient, because they parse files written by
  *older builds* rather than by an author — `HabitatStateEntry.placed_day` was
  deleted as dead in an earlier pass and every save from before that still
  carries it. Strictness is right for content you control and wrong for a record
  the player already has on disk.
  Three guards in a new `game_data_schema_tests.rs`:
  `every_content_schema_rejects_a_key_it_does_not_read` walks **every `.rs` file
  under `src`** rather than a hand-written list of "the schema files" — which
  matters, because `UiArtCatalog`, the struct that motivated all this, lives in
  `src/art` and a list would have missed exactly it. It also asserts it found at
  least 35 structs, because a source-scanning guard fails *open*.
  `a_key_nothing_reads_is_now_a_load_failure` drives a misspelled key through
  the real loader and asserts the error names it, so the guard's belief about
  what the attribute does is checked rather than assumed. And
  `every_embedded_content_file_still_parses` moves the failure from runtime to
  CI: `ui_text.json` loads through a fallback path that prints to stderr and
  carries on with `[missing ...]` placeholders, so without this a typo would
  ship as a game with no words in it rather than as a red test.

### The bench act two lives at was half vendor trash, 2026-08-03

- ~~Sixteen plain recipe outputs are wanted by nothing~~ **Six of them fixed,
  and the cause found 2026-08-03.** The sinkless tail had been read three times
  as "a flat class with no shared cause". Counted **per bench** it has one:
  **the greenhouse still made twelve bottles and six of them were wanted by
  nothing at all** — the worst room in the building by that measure, and the
  bench act two is spent at. Entry cauldron 7 of 24, cold bench 2 of 9, reading
  bench 1 of 11, rune forge 0 of 7.
  **And the six were not a random half.** Every *wanted* greenhouse output has a
  morph branch or feeds another recipe. Every *unwanted* one is flat: two
  reagents, one bottle, no branch, no downstream, no buyer. And they are the
  **place** recipes — the brew that exists because a particular piece of ground
  does. The salve made of what grows on the terraces Brin spent thirty years
  calling rubble. The tonic that brews a plant together with the seed it throws,
  from opposite ends of a question Rowan asked for nine years. The draught made
  of pollen that did not exist in this valley a season ago. **The game opens the
  ground, authors the brew that ground exists for, and then nobody ever asks for
  the result.**
  Six orders, each in the voice of whoever the bottle's own description already
  named, in a new `quests_board_ground.json` — filed apart because they share a
  cause rather than a tier, and because the standing file is at 658 lines.
  Spread over six townsfolk, with the two lightest buyers (Rowan, Mira) picking
  up one each. Each gated on the arc that opened its ground.
  `no_bench_makes_more_vendor_trash_than_it_makes_work` is the guard, and it is
  a floor rather than a target: a bench must want more of its own output than it
  wastes. Six of twelve is not more, so it fails on the state that prompted it.
  **16 → 10 sinkless potions; greenhouse 6/12 → 0/12.** What is left is 7 at the
  entry cauldron, 2 at the cold bench, 1 at the reading bench.
- ~~Nothing was watching what the demand passes do to the endgame~~ **Guarded
  2026-08-03.** Every pass that routes a sinkless bottle does it by writing a
  **repeatable** order, and a repeatable order is unbounded income. Across four
  such passes a full board cycle went **4,766 → 8,574 → 10,050 → 13,086**, while
  the commission sink was set once at **15,300** and has not moved. Nobody
  noticed, because the existing balance guard compares the sink to *one-off*
  income, which barely changes. `a_single_board_cycle_does_not_pay_for_the_whole
  _last_third` is the tripwire: one lap of the whole repeatable board must not
  fund everything the last third asks for. It passes now with about 15% of
  headroom, which is roughly one more pass of this size — and when it fails the
  answer is another commission, not smaller rewards. The demand is the content;
  the sink is the tuning.

### The schedule moved nine people around and their words stayed put, 2026-08-03

- ~~A townsperson says the same thing wherever they are standing~~ **Fixed
  2026-08-03.** Every NPC carries a four-stop schedule, and it has moved them
  between rooms since before this loop started: Mira walks down to the lake
  shore at dusk, Rowan works the moonlit forest by day *and* by night, Ione
  reads in the entry lab in the afternoon and is in the archive after dark, Brin
  is in the greenhouse, Lyra is at the pens, Elric climbs the tower with the
  notices, Wren walks up with the infirmary list, Tarn comes down off the pass.
  **Ten of the thirty-six stops are away from home, and not one of them had ever
  been asked to explain itself.** The schedule was read for a sprite position
  and for the journal's "here now / usually" hints — never for a word. A player
  who walked to the lake at dusk specifically to find Mira got the line she
  gives behind her counter.
  `NpcScheduleEntry.while_here_line` is one `#[serde(default)]` field, read as
  the conversation's **opener** when they are on that stop with nothing of the
  player's pending. An errand still comes first — forward motion beats flavour —
  so this is what they say between beats and after their arc is done, which is
  most of act three onward.
  Ten lines, one per away stop, each grounded in that person's own arc rather
  than in scenery: Mira tests the lake because the well row and the lake drink
  out of the same table and she would rather find out two years early this time;
  Lyra counts at night because walking into the pens at noon is counting how
  frightened they are of you; Ione works by raking light because raking light
  only works when there is no other light; Wren walks up with the list because
  the list has never once come to her, "which is only what happens when there is
  no one at the top of the hill to send it to."
  Two guards. `a_townsperson_away_from_home_has_a_reason_to_be_there` makes it a
  rule rather than a pass — a tenth NPC inherits it the day they are authored;
  the crow is exempt on purpose, because its four lines are a tutorial ladder an
  away line would shadow, and because the crow does not live anywhere.
  And `every_line_a_townsperson_has_is_reachable` **had to learn about the
  clock** — it walked arcs and town recovery but never the hour, so ten new
  authored strings would have passed it in silence. That is the second time this
  loop a guard over a class of thing has missed a new writer joining the class.
  `screenshots/hud/dialogue_away_from_home.png`; the capture harness took
  `dialogue:<npc>:<beat>:<window>`, and the window→minutes mapping is now one
  `set_time_window` rather than a copy inside `preview_area`.

### The ending was answered in paperwork, 2026-08-03

- ~~Nothing after the ending changes where the player can walk~~ **Fixed
  2026-08-03.** The pass that answered "the ending is a wall" answered it
  entirely in **requests**: three standing orders, a commission and an unsigned
  note. Measured afterwards, the valley itself was still frozen — of **85 gather
  nodes, 23 routes, 6 apply targets and 14 areas, not one waited on
  `observatory_ending`.** A player who finished the thing the whole game builds
  towards could go on being paid and had nowhere new to stand while it happened.
  The fix is on the story bible's own model rather than invented. The bible is
  explicit that **restoration is to the ground, not to the tower**, that
  **recovery is measurable and the measurements are the drama**, and that **new
  content should diagnose before it fixes.** So what opens after the ending is
  two pieces of ordinary ground that came back on their own, each the
  second-order payoff of an arc the ending already required, and each a thing
  somebody counts rather than announces:
  **The Seed Year** (north_plains) — **Rattleseed** cannot set its own seed and
  spent twenty years as the same crowns making no pods, because there was
  nothing flying to set it. Lyra counted eleven pollinators where the book said
  ninety; she is counting pods on four marked crowns now. Autumn work, and dry —
  a wet pod does not rattle and does not keep.
  **The Clear Shelf** (lake_shore) — **Sunkbell** flowers on the bed rather than
  the surface, so it opens only where light reaches the bottom. The shelf
  carried a foot of suspended silt on a still day for twenty years and everyone
  put that down to the lake being the lake. Nobody dredged it; it settled.
  Both feed **Seedhold Solution** at the ward-cooled bench — a poor draught
  whose actual purpose is that seed steeped in it goes on being seed, so a
  valley that has only just got its seed back never has to bet the whole of it
  on one autumn again. Rowan's post-ending standing order is the beginning of a
  four-year seed store she has wanted to place since she was nineteen.
  One journal beat, `the_ground_answered`, with Lyra and Rowan on it.
  `the_ending_opens_ground_and_not_only_paperwork` is the guard, and it asks for
  **two routes** rather than a node count, because one route is a corner and
  this project already wrote down for flourishes that a world change satisfied
  in a single place is not a world change.
  **23 → 25 routes; 85 → 89 nodes; 170 → 173 items; 62 → 63 recipes.**
  `screenshots/hud/plains_seed_year.png`, `lake_clear_shelf.png`.

### The game had sixty-two formulae and told you how to make three, 2026-08-03

- ~~Fifty-nine recipes are learned only by guessing their exact reagents~~
  **Fixed 2026-08-03.** Discovery is the design — `starter_known` belongs only
  on the three entry basics and this pass does not change that. What the game
  gave the player to work with was the problem. Three routes to knowing a
  formula exist: it is flagged starter, or you hit its exact ingredient multiset
  at its exact bench, or — no third one. Quests never teach a formula, no
  counter sells one, disassembly requires the recipe to be *known already*, and
  the herb journal, which is the game's own memory of everything ever gathered,
  said of every undiscovered use only **"Used in formulae you have not yet
  discovered."** A count, with no direction in it at all, against **46
  two-reagent formulae** (1,485 pairs across the 54 things that can go in a pot)
  and **16 three-reagent** ones. The bench does confirm an exact hit before you
  spend anything, so it is browsing rather than blind guessing — which makes it
  tedium rather than impossibility, and no better a use of the player's evening.
  The journal points now, and it still names neither the formula nor the
  reagent. It names **where the missing half comes from**: ground you can walk
  to, a counter that stocks it, or the fact that it has to be brewed rather than
  picked — which is also the only way the second-order tier announces that it
  exists. Once the player has met everything the nearest formula wants, the line
  stops sending them out and names **the bench** instead, because for a player
  who has been round the valley that is the whole of the useful answer.
  Nothing is authored per recipe: the hint is derived, so a formula added
  tomorrow is pointed at the day it ships.
  *Two rules the derivation had to learn.* Ground the player cannot work yet is
  worse than no hint, because "not here" and "not yet" read the same — whisper
  moss grows in seven places and the terraces under the tower wall, which open
  at the end of Brin's arc, sorted ahead of the plains. And season, weather and
  hour are deliberately **not** consulted: the hint answers *where*, and the
  conditions line directly above it already answers *when*.
  Three guards: every reagent that feeds a formula points somewhere, the hint
  names neither the formula nor the reagent it wants, and no reagent is sent to
  ground that is still shut. All three verified by breaking them.
  `screenshots/hud/journal_formula_hint.png`, `journal_hearsay.png`.
  *The entry box is now genuinely full.* The first draft of the copy overran by
  6px on Inkgall Bead — three lines of conditions plus two of hint against room
  for four — and the fit guard from the hearsay pass named it. The copy is
  shorter for it, and the capture harness took a `journal:<herb>` scene so the
  worst entry in the game can be looked at rather than trusted to arithmetic.

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
- ~~The ending is a wall~~ **Fixed 2026-08-03.** Every request, node, warp and
  flourish in the game was reachable *before* the observatory. So the moment a
  player finished the thing the whole game builds towards, the valley had nine
  sentences of last words and then **nothing new ever happened again** — in a
  game whose scope note says a finished product is 20–25 hours. `loop.md` had
  this in Deferred as needing a new system. It did not: `observatory_ending` is
  a journal beat, and beats are the currency warps, stations and gather nodes
  already gate with. Quests were the one thing that could not read them —
  they could wait on another quest, a warp, a brew count, a mastered formula or
  somebody's standing, and none of those can say "after the ending."
  `QuestDefinition.required_journal_milestone` fixes that in one
  `#[serde(default)]` field, and the struct took `deny_unknown_fields` at the
  same time — a gate key with no reader behind it is this project's most
  repeated failure and a request is the worst place for it.
  **The Second Bench** (5,200, six purified draughts, and the formula has to be
  *mastered*) is the sixth commission and the last thing the game asks for: it
  fits out a second bench in the entry laboratory and pays somebody to stand at
  it for a year — a stipend, a reference shelf, and a formula book that is
  allowed to be wrong in the margins. The draughts are the reference shelf. It
  is the plainest thing in the book, which is the point: **the last thing the
  game asks for is the first thing you learned, made well enough that somebody
  else can learn it from the bottle.** It lands visibly in `tower_entry`
  (`screenshots/hud/tower_entry_lab.png`) and Elric, Ione and the Crow each have
  a word about it — the Crow's is "You just bought the thing that makes you
  unnecessary."
  Three standing orders in a new `quests_board_afterward.json`: the square's
  lamps as *a line in the budget rather than a favour* (Elric), the infirmary's
  own restock list rather than Wren walking up the hill (Wren), and the survey's
  rounds handed to a keeper who is not Lyra. All three are about the valley
  placing ordinary business with the tower instead of being rescued by it.
  And a **fifth unsigned note**, the first to arrive after the observatory,
  which says only that whoever writes them is glad it was finished properly.
  No question in it, which is worse. Ione copies all five into the record and
  writes "still open" underneath — the story bible marks that question as
  deliberately unclosed and it stays that way.
  Economy: sink **15,300** against 4,881 one-off and 8,574 a full cycle.
  Two guards: `something_in_the_game_happens_after_the_ending` (reads the ending
  beat off the narrative spine, so renaming it cannot hollow the test out) and
  `a_beat_gated_request_waits_for_the_beat`, which drives the new field through
  shut → named → open rather than trusting the expression.
  *Filing done as promised:* `quests_board_standing.json` was 776 lines; the
  unsigned chain moved to `quests_board_unsigned.json` (it is a story with its
  own beats, not a supply arrangement), leaving 658.
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
- ~~The tier is one room, not a tier~~ **Fixed 2026-08-03.** Second-order
  brewing existed at exactly **one bench in a five-bench tower**, which makes it
  a feature of the archive rather than a layer of the game. The measure that
  showed it was counting, per bench, how many of its own outputs anything
  anywhere consumes: the archive ate four of its own and the greenhouse fed
  four, and **the entry cauldron's twenty-four outputs and the rune forge's five
  fed nothing at all, anywhere.** The forge was the worst room in the building —
  five recipes, three of them making bottles nobody wanted, and a whole floor of
  the tower behind it.
  The **channel forge takes finished bottles now**, which is the most natural
  place in the tower for it: the rune workbench on that same floor already
  reworks finished bottles, so the floor's premise is exactly this. Its room
  bonus already said what it is for — "the channels that take an imbuing run hot
  the whole length of the bench" — so the forge's second-order character is
  *heat you do not have to stand over*, against the archive's *reading*.
  Two recipes, and the point of both is that they eat the forge's own dead
  stock. **Banked-Through Tonic** folds a bankfire tonic and a kindling tonic
  down the hot channel: nothing happens for a while and then you are simply not
  cold, and you stay not cold until it gets light. **Held-Heat Lamp** folds the
  channelfire lantern and the cinderlight lamp into a working light that is also
  a brazier, because a crew in a wet cut has only ever been able to carry one of
  the two down and has always chosen the light and then stood about being cold.
  Both are the first recipes in the game to *require* a `kilnfire` catalyst —
  that tag was asked for by five morph branches and no recipe at all.
  Every branch on both reaches a bottle something already wants, so the tier
  adds two bottles rather than six: **Lastthaw Cordial** and **Kept-Warm Tonic**
  off the banked-through, **Hearthchannel Lantern** and **Banked Cinderlight**
  off the lamp. Cool the banked-through right down and you get the keeper the
  winter stores want; drive it with road amber and you get the cordial that
  survives the cart. That is a three-way decision between things the valley
  wants, which is what a branch is for.
  Two standing orders route the new pair: the relief post's four hours kept warm
  (Wren, after the post is funded) and one bottle instead of two for the lower
  cut (Brin). Brin also has a word about the beat — "I watched you do it and I
  still do not like it… I have decided I do not have to like a thing to want
  four more of them by Thursday."
  `more_than_one_bench_takes_a_finished_bottle_and_means_it` is the guard: a
  tier is not one room, and a bench that advertises it will take a bottle has to
  have something that asks for one. Verified by turning the forge back off.
  **Rune forge 3-of-5 sinkless → 0-of-7; 20 → 16 sinkless potions; 60 → 62
  recipes.** `screenshots/hud/compound_bench.png` is the forge, not the archive.

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
- ~~The tower does not notice what you do to it~~ **Fixed 2026-08-03.** The
  floor above is a *count*, and a count is satisfied by putting everything in
  one room — which is what happened. **Nine of the first fourteen flourishes
  were in the town square.** The tower, the building this game is about
  reopening, changed in two of its six rooms; `containment_floor`,
  `rune_workshop_floor` and `observatory_floor` changed for nothing, and so did
  **`tower_entry`** — the room the player starts in, sleeps in, brews in for the
  first several hours and crosses on the way to everything.
  Seven flourishes, all on beats that already existed:
  - **the entry lab in use** (`first_town_relief`) — a second stool at the
    cauldron, delivered stock crated by the wall, a line of cloths drying. It
    stops being one person improvising the day the town first sends work up.
  - **the ledger post** (`tower_entered_the_ledger`) — a board by the door with
    the town's notices pinned to it. The tower is now somewhere that receives
    post.
  - **the previous hand** (`eleven_months_restored`) — the wizard's eleven
    months of working notes, stacked back on the case they were taken from,
    with a lamp left burning over them. The most loaded of the seven: the
    epilogue's open question standing in the room where the game starts.
  - **the pens settled** (`containment_stable`) — steady lamps instead of
    flickering ones, bedding down, a filled trough.
  - **the channels running** (`discovered_the_channels_hold`) — the cut floor
    channels lit end to end and a finished lamp left on the forge bench: the
    rune floor making something rather than mending something.
  - **the mirror silvered** (`observatory_mirror_cleared`) — the endgame room's
    lens throwing a clean fan of light in three directions. This one is the
    apply target from earlier the same day getting a *visible* payoff on top of
    the ground it opens.
  - **the shelves' own record** (`the_shelves_kept_their_own_record`) — raking
    light across a rank of spines and the gaps where volumes came off.
  `every_room_the_player_works_in_changes_for_something` is the guard, and it
  derives "a room the player works in" from where the **stations** are rather
  than from a list, so a bench on a new floor is covered the day it is placed.
  Verified by deleting the four: it names all four.
  `screenshots/hud/tower_entry_lab.png`, `containment_pens.png`,
  `rune_channels.png`, `observatory_silvered.png`.
  ***Placement lesson, re-learned twice in one pass.*** The first draft put the
  drying line and the wizard's notes in the top band the title banner and the
  clock own, and the observatory's chart-floor lines under the potion belt. The
  capture is the only way to know — reasoning about screen coordinates does not
  work, because the camera follows the player and the offset changes with where
  they stand.

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
