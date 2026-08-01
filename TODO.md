# TODO — Alchemy Tower

## Scope — decided

**Long tail. A finished product is 20–25 hours of play.** Everything below is
measured against that target; anything not listed here is considered done.

Where the content currently stands: 14 areas, 49 recipes across 6 benches, 9
townsfolk, 24 story-arc requests, 26 repeatable board orders, 11 biome
ingredient tables, plus an epilogue. That is the mid-game; the remaining work is
what makes it a 20–25 hour game rather than a well-furnished 8-hour one.

## Applied alchemy — the largest open gap

- Implement the apply-potion-to-target flow (wilted route plant, frightened
  creature, blocked path) on top of the existing `EffectKind` system. Delivery
  is still just handing a bottle to an NPC; blockers are collision and art only
  (`data/schema_render.rs`, `art/props.rs`), never something a brew can act on.
  Until this exists the game's stated premise is unexpressed.
- Once targets exist, gate two or three route/floor openings behind applying a
  potion rather than delivering one, so the mechanic is on the critical path.

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
  a reason to keep restoring past the final gate.
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
