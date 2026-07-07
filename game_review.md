# Alchemy Tower — Design Review

*Senior design / production review, 2026-07-07. Based on the source tree, authored JSON content (`assets/data/*`), `IMPLEMENTED_SYSTEMS.md`, `feedback.md`, and current HUD/overlay screenshots.*

---

# 1. Project Overview

## Project Name
Alchemy Tower

## Genre
Cozy exploration + crafting sim. A single-player, non-combat "restoration and potion-making" game in the lineage of *Potion Craft*, *Stardew Valley*, and *Spiritfarer*.

## Core Concept
- **What the player does:** Explore biomes to gather ingredients (gated by season / weather / time-of-day), brew potions at station-based cauldrons, deliver those potions to townsfolk to solve their problems, and spend the resulting progress unlocking and restoring floors of an abandoned wizard's tower.
- **Fantasy provided:** You are the new caretaker of a sleeping magical tower. The fantasy is *quiet competence and stewardship* — not conquering power, but tending a living system back to health. The narrative theme (surfaced repeatedly in `narrative_text.json`) is explicit and unusually mature: *"The tower is not yours to conquer. It is yours to tend."*
- **What makes it different:** Two things stand out. First, a genuinely deep **alchemy resolution model** — 4-axis element profiles, ingredient traits, quality bands, mastery stages, trait inheritance, morph paths, catalyst/room/timing checks, and instability fallbacks — that is far more simulation-grade than most cozy crafters. Second, a stated design philosophy that **potions are applied to help people/plants/animals/places rather than self-buff the player** (`tasks.md` item 8). That reframing — alchemy as a *helping tool* rather than a *combat consumable* — is the game's most original seed.
- **Target player:** Fans of low-stress "systems gardening" — the *Potion Craft* / *Stardew* / *A Little to the Left* audience who enjoy mastery, collection, and a gentle narrative, and who explicitly do **not** want combat or failure pressure.

## Current State
**Feature-complete but content-thin and unproven for fun.** This is *not* a prototype and calling it "early development" would understate it. The codebase (~22k lines across a disciplined module structure) implements an almost complete vertical slice of every promised system: exploration across 13 areas, gathering, the full alchemy stack, greenhouse cultivation, creature containment, a rune-augmentation workshop, an archive/end-state with a triggerable ending, NPCs with schedules and pathing, a quest system with rich requirement checks, save/load with migration, and a headless screenshot harness.

The gap is not *systems* — it's **content volume, world/character art, and validated fun.** There are only 4 base recipes, 1 morph, 2 creatures, 3 runes, and 8 quests to exercise a mechanical engine built for far more. The most recent development energy (per git log and memory) has gone into **UI polish and HUD chrome**, which was the loudest item in the last playtest. So the honest label is: *feature-complete skeleton in a content-expansion phase, with fun not yet demonstrated.*

---

# 2. Core Gameplay Analysis

## Main Gameplay Loop

> Gather (by biome/season/weather/time) → Brew (station alchemy w/ quality) → Deliver to an NPC / turn in a quest → Earn coins + brew-count + milestones → Unlock & restore the next tower floor → which opens new stations, ingredients, and quests → Repeat.

A secondary, slower loop wraps it: **restore floor → floor grants a new production system (greenhouse beds, creature habitats, rune bench) → that system feeds back into the brew loop.**

## Evaluation
- **Clear?** Structurally yes, and recent work (next-goal HUD messaging, starter recipes pre-known, contextual prompts, tutorial toasts) has made the *opening* legible. The loop is signposted well.
- **Satisfying?** Partially. Gathering is called out by the playtester as the single thing that already "works well," and the delivery→milestone→restoration beat has a nice arc. But the **brew step — the titular core verb — is currently the weakest link for satisfaction**, because 4 recipes give the deep quality system almost nothing to chew on. You learn the three starter recipes and then rarely make a *decision*; you re-brew known formulae.
- **Meaningful decisions?** The *engine* supports rich decisions (which ingredient variant for which trait, catalyst vs. not, push quality band vs. conserve stock). The *content* doesn't yet force them. With only one gated quality threshold ("Fine-or-better") and few recipes, optimal play collapses to "brew the known thing."
- **Variety?** Wide but shallow. 13 areas and 4 seasons of spawn tables is impressive breadth, but many biomes currently exist to be traversed rather than to pose distinct gathering *puzzles*.
- **Long-term motivation?** This is the biggest hole. The loop terminates at the Observatory ending after a fixed, fairly short critical path (≈8 quests). There is no endless/mastery/collection driver strong enough to hold players past the credits, and the mid-game lacks escalating stakes.

**Verdict:** The loop is *coherent and well-scaffolded* but currently **motivated more by "see the next room" than by the joy of the core verb.** The fix is not more systems — it's deepening the brew decision and giving the loop stakes.

---

# 3. Existing Systems Review

## Alchemy / Brewing (the core verb)

### Purpose
The signature mechanic — convert gathered materials into useful potions with meaningful quality variance.

### Current Implementation
Three ingredient slots + one catalyst slot; heat / stir / timing controls; exact-count recipe matching with an unstable + trait-salvage fallback; 5 quality bands (Crude→Masterwork); mastery stages; preferred/guaranteed trait inheritance; element-minimum, catalyst, and room-bonus checks; morph outputs; and a genuinely informative preview panel that classifies the setup (unknown salvage / unlogged / known-uncertain / known-imperfect) and lists failure reasons. Mechanically this is the most sophisticated part of the game.

### Strengths
- The resolution model is deep, legible in the preview, and *designed for emergence*. It's a real competitive advantage vs. most cozy crafters.
- Trait inheritance + morphs create the *possibility* of a satisfying "discover a hidden branch" moment.

### Weaknesses
- **Content starvation.** 4 recipes + 1 morph cannot exercise this engine. The depth is invisible to the player.
- **No decision pressure.** Exact-count matching means there's usually one "right" combination; the fallback salvage is a consolation, not a strategic choice.
- **Brewing is a menu, not a mini-game.** *Potion Craft*'s success comes from the *act* of brewing being tactile and expressive. Here, brewing is slot-selection + numeric parameter picking, then a result. The verb itself isn't playful.

### Improvement Ideas
- Author **15–25 recipes** with overlapping ingredients so the same herb pulls in multiple directions (creates real inventory decisions). *(This is the single highest-leverage content task.)*
- Add **discovery-driven experimentation rewards**: brewing an unlogged combination that resolves to a *new* stable formula should be a celebrated, journaled event — turn the salvage fallback into a discovery engine.
- Introduce **quality trade-offs the player can feel**: e.g., higher heat = higher potency but higher instability risk, so band-pushing is a gamble, not a lookup.
- Impact: **Game-changing.** Cost: Medium (engine exists; this is mostly authoring + a little tuning).

---

## Gathering & Biomes

### Purpose
Front-end of the loop; the source of decisions about *where and when* to explore.

### Current Implementation
Authored gather nodes with route/season/weather/time-window gating, daily spawn chance, biome-specific sprite overrides, "seen vs. learned" herb memory, and journal flavor capture. 13 areas, 4-season spawn plans.

### Strengths
- **The playtester's favorite — it already works.** The season/weather/time gating creates natural "come back later" hooks and is a legitimately good cozy mechanic.
- "Learn the herb's conditions only after first collecting it" is an elegant discovery loop.

### Weaknesses
- **Breadth over identity.** Many biomes feel like reskinned traversal rather than distinct gathering *problems*. `tasks.md` item 7 itself flags this unresolved question.
- Gathering has no *skill* or *risk* — walk over node, collect. It's pleasant but frictionless, so it can't carry a full session alone.

### Improvement Ideas
- Give **2–3 biomes a signature gathering hook** (a night-only bloom, a weather-triggered spawn, a rare variant that only appears under a specific combo) rather than spreading effort thin across 13.
- Make biome-native ingredients (`field_bloom`, `sunspike`, etc.) into **recipe anchors** so each biome is a *supply line* for specific potions — this is `tasks.md`'s own open question and answering it ties gathering to brewing.
- Impact: **High.** Cost: Medium.

---

## Tower Restoration & Progression

### Purpose
The macro-loop and the emotional spine ("restore the tower").

### Current Implementation
6-floor path (Entry → Greenhouse → Containment → Rune → Archive → Observatory), each gated by warp requirements (brew count / coin / item / milestone), with next-goal HUD/journal signposting and a scripted ending.

### Strengths
- A clear, legible spine that gives the whole game direction. The "each floor unlocks a new production system" design is sound.
- The narrative payoff (`archive_revelation`, observatory ending) is genuinely well-written and thematically consistent.

### Weaknesses
- **The critical path is short and linear.** Six floors and ~8 quests is a few hours, then the game is "done." No New Game+, no post-ending sandbox, no branching.
- Gates are mostly **brew-count / coin thresholds** — grind checks rather than skill/knowledge checks. They pace the game but don't test mastery.

### Improvement Ideas
- Convert at least one gate from "brew N times" to "**demonstrate mastery**" (deliver a Masterwork, discover a morph, satisfy a multi-trait quest) so progress feels earned by *skill*, not repetition.
- Add a lightweight **post-ending sandbox / continued town requests** so long-term players have somewhere to go. Impact: High for retention. Cost: Small–Medium.

---

## NPCs, Quests & Story

### Purpose
The reason to brew; the emotional hook.

### Current Implementation
7 NPCs with roles, time-of-day schedules, cross-area pathing, rapport counters, staged "Phase 1" dialogue keyed off progression, and 8 quests with rich requirement checks (item/amount, quality band, inherited trait, effect kind, prerequisites, warp gates).

### Strengths
- The dialogue was rewritten from the "robotic placeholder" state the last playtest flagged and is now **genuinely good** — distinct voices, concrete human stakes (Mira's exhausted patients, Rowan's dark road home), and a people-first framing. This is a real turnaround.
- The quest-requirement engine (quality bands, trait gates) is a strong hook that *could* force interesting brews.
- The "why" is now carried in the dialogue and echoed on the brew preview ("For {npc} — {quest}"), fixing the earlier "no reason to help Mira" complaint.

### Weaknesses
- **Only 8 quests, mostly one-shot deliveries.** No recurring relationships, no rapport payoffs the player can *feel*, no branching. Rapport is a counter with little consequence.
- Story is **strong in flavor but thin in arc** — there's a good premise and a good ending, but the middle is a delivery checklist, not rising action.
- NPC schedules/pathing are implemented but risk being *invisible effort* — players rarely notice, and it can create "where is Mira?" friction.

### Improvement Ideas
- Give **each NPC a 3-beat arc** (setup → complication → payoff) tied to rapport, so relationships progress visibly. The staged Phase-1 dialogue system already provides the scaffolding.
- Add **repeatable town requests** (the quest board exists) to sustain the mid-game between story beats.
- Make rapport *do something* (unlocks a recipe hint, a discount, a personal side-quest). Impact: High. Cost: Medium.

---

## Secondary Production Systems (Greenhouse / Containment / Runes)

### Purpose
Per-floor production systems that feed the brew loop.

### Current Implementation
- **Greenhouse:** planter beds, seed planting, daily tending, 4 growth stages, harvest yields, one quest-gated expansion.
- **Containment:** place 2 creature types into habitats → renewable timed output (Lumen Dust, Dew Slime).
- **Runes:** post-brew augmentation (potion + rune → augmented potion), 3 runes.

### Strengths
- Each adds a distinct verb (cultivate / husband / modify) and a reason to return to a floor. Good structural variety.
- The greenhouse "gentle daily tending" fits the cozy fantasy perfectly.

### Weaknesses
- **All three are minimum-viable stubs.** 2 creatures, 3 runes, a handful of seeds. Each is a proof-of-concept, not a system with depth to master.
- **Risk of dilution.** Three shallow production systems compete for the player's attention and the developer's content budget. This is the project's central strategic tension: *breadth of systems has outrun depth of any one.*

### Improvement Ideas
- **Pick one to deepen, keep the others minimal.** The greenhouse is the strongest fit for the fantasy and the most naturally expandable — invest there; leave containment/runes as light spice.
- Impact: Medium (High if it prevents further scope-spread). Cost: Small (mostly a *decision*, then focused authoring).

---

## User Interface & Onboarding

### Purpose
The layer the last playtest identified as the #1 problem.

### Current Implementation
A heavily reworked ornate "field-journal / brass workshop" HUD (medallion vitality, coin chip, parchment goal note, location banner, compass minimap, status plaque, potion belt, control tags), a redesigned two-column alchemy overlay, uniform UI scaling for small windows (`ui_scale.rs`), shared beveled/filigreed overlay chrome, tutorial toasts, and pre-known starter recipes.

### Strengths
- **The last playtest's UI complaints have been substantively addressed** — overlaps fixed, alchemy overlay rebuilt, exit affordances added, empty-belt ghost bottles removed, small-window scaling solved, and the aesthetic unified to the brass palette.
- The HUD is now *attractive* and information-dense in a good way.

### Weaknesses
- **The HUD chrome is now more polished than the game world it frames.** In the current screenshot, an ornate, gilded, filigreed UI surrounds a flat purple grid, generic brown "table" props, and a tiny placeholder character. This inversion is jarring and is now the game's most visible weakness.
- **Information density risk:** the ornate HUD occupies a lot of screen edge; on smaller windows it can still crowd the play space (the reason `ui_scale` exists at all).

### Improvement Ideas
- **Rebalance art investment toward the world and characters.** The environment art and the player/NPC sprites are the weakest visual element and now the limiting factor on first impression. Impact: High. Cost: Medium–Large (art).
- Consider a slightly *quieter* HUD so the world reads as the star. Impact: Medium. Cost: Small.

---

## Save / Load, Audio, Data Pipeline (supporting systems — brief)
- **Save/load:** versioned, migration-aware, comprehensive. Solid. No action needed.
- **Audio:** procedural placeholder one-shots only (footsteps, pickup, stir, brew result). Functional stand-in; a hand-authored ambient + music pass is a Phase-4 polish item, not urgent, but *cozy games live and die on audio atmosphere* — don't under-rate this later.
- **Data-driven pipeline:** exemplary. Content lives in JSON, which means the recommended content expansion is *authoring, not engineering.* This is a major asset.

---

# 4. Similar Games & Lessons

## Potion Craft: Alchemist Simulator
- **Similar:** Alchemy is the core verb; deep ingredient/quality system; non-combat.
- **Does better:** The *act* of brewing is a tactile, expressive mini-game (drawing a path on the alchemy map). Discovery of new potions is the whole game.
- **Adapt:** Make brewing *feel* like a craft, not a form. Make *discovering* a new recipe the central reward, not an afterthought.
- **Don't copy:** Its map-drawing metaphor is its own; Alchemy Tower's station/quality model is a valid different path — don't abandon it, *enrich* it.

## Stardew Valley
- **Similar:** Cozy loop, seasons/weather/time, townsfolk with schedules and relationships, restoration fantasy.
- **Does better:** NPC relationships have *arcs and payoffs*; every day offers many small overlapping goals; there's a near-endless mastery ceiling.
- **Adapt:** Give NPCs multi-beat arcs and rapport payoffs; provide daily variety and long-tail goals.
- **Don't copy:** Its sprawl (mining/combat/fishing/marriage). Alchemy Tower should stay *focused* — its scope is already at risk.

## Spiritfarer
- **Similar:** Restoration/stewardship fantasy, "tend not conquer" theme, character-driven emotional beats, non-combat production loops.
- **Does better:** Ties every mechanical system to an *emotional character story* — you upgrade the boat *for* someone. Craft always serves narrative.
- **Adapt:** This is the model to follow. Alchemy Tower's "potions help people" philosophy is the same DNA — lean into it hard: every brew should be *for someone*, with an emotional payoff.
- **Don't copy:** Its scope/length; it's a much bigger production.

## A Short Hike / A Little to the Left (short cozy games)
- **Lesson:** A *small, tight, complete* cozy experience can succeed enormously without endless content. If deepening every system is infeasible, the alternative is a **deliberately short, beautifully polished 2–3 hour experience** — which suits this project's linear critical path.

---

# 5. Feature Improvement List

## Critical Improvements
| Priority | Feature | Description | Player Benefit | Dev Cost |
|---|---|---|---|---|
| Critical | Recipe content pass | Author 15–25 recipes with overlapping ingredients + more morphs | Makes the deep alchemy engine *visible* and creates real brew decisions | Medium |
| Critical | World & character art pass | Bring environment/character art up to the HUD's quality bar | Fixes the jarring "gilded UI, bare world" first impression | Medium–Large |
| Critical | Deepen the core verb | Add risk/reward to heat/stir/timing so band-pushing is a gamble; celebrate discovery of new formulae | Turns brewing from a lookup into a decision | Medium |

## High Value Improvements
| Priority | Feature | Description | Player Benefit | Dev Cost |
|---|---|---|---|---|
| High | NPC relationship arcs | 3-beat arc per NPC tied to rapport, with payoffs | Emotional hook + mid-game motivation | Medium |
| High | Repeatable town requests | Use the existing quest board for recurring asks | Sustains the mid-game between story beats | Small |
| High | Tie biomes to recipes | Make biome-native ingredients recipe anchors | Connects gathering to brewing; gives biomes purpose | Medium |
| High | Mastery-based progression gate | One floor gate that requires *skill* not grind | Progress feels earned | Small |

## Nice To Have
| Priority | Feature | Description | Player Benefit | Dev Cost |
|---|---|---|---|---|
| Nice | Hand-authored audio/music | Replace placeholder one-shots with cozy ambient + music | Atmosphere; cozy games depend on it | Medium |
| Nice | Post-ending sandbox | Continued requests after the Observatory | Long-tail retention | Small–Medium |
| Nice | Quieter HUD option | Let the world read as the visual star | Better first impression | Small |
| Nice | "Applied alchemy" targets | Implement the person/planter/creature/route apply-flow (`tasks.md` #8) | Delivers the game's most original idea | Medium |

## Avoid / Do Not Add
- **Combat / enemies / stamina-drain survival.** Directly contradicts the cozy, non-combat fantasy and the "tend not conquer" theme. The codebase deliberately has none — keep it that way.
- **A fourth or fifth production system.** Scope is already spread too thin; do not add more breadth.
- **Multiplayer.** No design justification; would multiply cost against the core solo fantasy.
- **Dialogue trees with branching choices** (for now) — tempting, but the linear, well-written dialogue is working; branching is a large cost that doesn't address the core weakness (thin content depth).

---

# 6. Missing Gameplay Elements

## The "apply potion to a target" flow
- **Expectation:** The game's own stated philosophy (`tasks.md` #8) is that potions help people/plants/animals/routes — but delivery is currently just "hand item to NPC." The *applied-alchemy* verb (restore/reveal/grow/calm/overdrive on a target) is designed but **not implemented.**
- **Does it need it?** *Yes — this is the missing expression of the unique selling point.* Right now the game *says* it's about applied alchemy but mechanically it's about delivering quest items.
- **Implementation:** Start small — a few in-world targets (a wilted route plant, a frightened creature, a blocked path) that consume a matching potion for a visible effect. Reuse the existing effect-kind system.
- **Priority:** High (it's the differentiator).

## Reason to keep brewing after the story ends
- **Expectation:** Cozy-crafter players expect a long tail (collection, mastery, endless requests).
- **Needs it?** Only if the goal is retention. If the goal is a *tight short game*, an ending is fine — but then market it as such.
- **Priority:** Medium; decide the game's intended length first.

## Meaningful failure / consequence
- **Expectation:** Some tension so success feels earned.
- **Needs it?** Lightly. Not combat-style failure — but the instability system could carry gentle stakes (a wasted brew, a comedic misfire) so quality *matters*.
- **Priority:** Medium.

## Things NOT missing (resist adding)
Combat, hunger, crafting-of-tools, base-defense, procedural dungeons — players might expect these from adjacent genres, but none serve this fantasy. Their absence is a *feature*.

---

# 7. Content & Replayability Analysis

**Current reasons to keep playing:** see the next tower floor; discover the next herb/condition; finish the town's story. These are **finite** — the game ends at the Observatory after a linear path.

- **Variety:** Wide (13 areas, 4 seasons, 5 systems) but shallow per-system.
- **Progression:** Clear but short and mostly grind-gated.
- **Unlocks:** Floors are the main unlock currency; good structure, limited quantity.
- **Randomness:** Daily spawns + quality variance provide light run-to-run difference; not enough to drive replay.
- **Player choice:** Mechanically supported, content-starved (see Alchemy).
- **Different strategies:** Not really — optimal play is "brew the known thing." Deepening recipes fixes this.
- **Emergent gameplay:** *Latent* in the trait/morph/quality engine but unrealized for lack of content.
- **Long-term goals:** The weakest axis — there's a finish line but no horizon beyond it.

**Recommendation:** Replayability is the wrong target for this game. Aim instead for a **satisfying, complete first playthrough** (deepen recipes, add NPC arcs, polish art) and treat post-ending sandbox as a bonus. A "wide and thin" game does not become replayable by adding *more* thin systems — it becomes replayable (or simply *good*) by making the core verb and story worth the trip.

---

# 8. Player Experience Review

## First 10 Minutes
The player understands quite a lot now, thanks to recent onboarding work: the goal note names an objective ("Something For The Headaches"), starter recipes are pre-known, the Crow guides the first gather/brew, and contextual prompts point at the cauldron. **This is the game's most-improved area.** Remaining friction: the visual first impression — an ornate HUD around a bare purple-grid world undersells the game before the (good) writing lands.

## First Hour
The hook is **the writing and the restoration promise**, not yet the mechanics. A player who reads the dialogue will feel the town's stakes and want to help. A player who skips dialogue will find the brewing shallow (few recipes, one-right-answer) and may not see why to continue. **The hook is currently narrative-dependent** — which is fragile, because the story arc thins out after the opening. Strengthening mid-game recipes and NPC arcs is what converts the first hour into the second.

## Long-Term
Presently weak: a linear path to a fixed ending, no mastery ceiling, no post-game. This is acceptable *if* the game commits to being a tight 2–4 hour cozy experience — but that intent should be a deliberate decision, not a default.

---

# 9. Development Roadmap

## Phase 1: Make It Fun (make the core verb worth doing)
- **Goals:** Turn brewing from a lookup into a decision, and make discovery rewarding.
- **Features:** Recipe content pass (15–25 recipes, overlapping ingredients, more morphs); risk/reward on brew parameters; celebrated discovery of unlogged formulae; tie 2–3 biomes to specific recipes.
- **Why first:** The core verb is the game. No amount of art, story, or systems saves a hollow center. This is also *cheap* — it's authoring against an engine that already exists.

## Phase 2: Add Depth (give the loop stakes and heart)
- **Goals:** Emotional and strategic motivation to keep going.
- **Features:** 3-beat NPC arcs w/ rapport payoffs; repeatable town-board requests; one mastery-based progression gate; implement the **applied-alchemy target flow** (the USP).
- **Why second:** Once brewing is fun, depth makes it *matter*. The applied-alchemy verb is what makes the game unlike its competitors — but only worth building once the core loop is proven.

## Phase 3: Add Content (widen the proven loop, don't widen systems)
- **Goals:** Length and variety *within* the working systems — not new systems.
- **Features:** More recipes/ingredients/creatures/runes; more quests and NPC beats; signature gathering hooks per biome; optional post-ending sandbox.
- **Why third:** Only expand what's demonstrated to be fun. Explicitly resist adding a 6th system.

## Phase 4: Polish (make the world match the frame)
- **Goals:** Bring presentation up to the standard the HUD has already set.
- **Features:** World + character art pass; hand-authored ambient audio and music; a quieter/cleaner HUD option; game-feel juice on brew/gather/deliver.
- **Why last:** Polish amplifies a fun game and wastes effort on an unfun one. The art gap is real and important, but it comes *after* the core verb works — a beautiful world around a hollow loop still fails.

*Order rationale: this project's risk is inverted from a typical prototype. It has too many systems and not enough fun-per-system. So the roadmap deliberately deepens and proves the core before adding any breadth, and polishes presentation last despite it being the most visible weakness.*

---

# 10. Final Assessment

## Strongest Idea
The fusion of a **simulation-grade alchemy engine** with a **"potions help people, not the player" philosophy** and a **"tend, don't conquer" restoration theme.** No cozy crafter combines this depth of resolution with this stewardship framing. The narrative writing already proves the theme can land emotionally.

## Biggest Risk
**Breadth has outrun depth.** The game has ~5 production systems, 13 areas, and a full end-state — but 4 recipes, 8 quests, and 2 creatures. It is *wide and thin*, and the recent effort went into UI chrome rather than the hollow core. If development keeps adding systems instead of deepening the brew verb and the story, the game will remain a polished-looking demo that isn't fun to *play* past the novelty. The secondary risk is the **art inversion** — a gilded UI around a placeholder world — killing the first impression before the strengths surface.

## Missing Ingredient
**A core verb worth repeating.** Brewing needs enough recipes and enough decision-tension that discovering and mastering potions is intrinsically satisfying — independent of the story or the next unlock. Everything else is scaffolding around that.

## Unique Selling Point
*"A cozy alchemy game where you don't fight anything — you brew potions to quietly heal a struggling town and a sleeping tower, and every potion is for someone."* The stewardship fantasy + genuine crafting depth + strong writing is a defensible niche, if the core verb delivers.

## Recommendation
**Continue development, but redesign the priorities — stop adding systems, deepen the core.**

Freeze the system count. Redirect all effort into: (1) a recipe/brew-depth pass to make the titular verb fun, (2) NPC arcs + applied-alchemy targets to deliver the emotional USP, and (3) an art pass to close the presentation gap. The engineering is largely done and done well (clean data-driven architecture means content is *authoring*, not coding); the disciplined codebase and the strong writing are real assets worth protecting. The project's danger is not that it's under-built — it's that it's *broad and hollow*, and one more system would make that worse. Commit to a **focused, complete, 3–4 hour cozy experience** built around a brew verb that's genuinely worth doing, and this becomes a shippable, distinctive small game rather than an impressive-but-flat tech demo.
