# TODO — Alchemy Tower

## Core loop & alchemy

- Author 15–25 recipes with overlapping ingredients and more morph paths, so the quality/trait engine drives real brew decisions instead of a lookup. (26 recipes, 12 morph branches across three catalyst tags; the starter recipe now morphs, so the layer teaches itself.)
- Add risk/reward to heat, stir and timing so pushing a quality band is a gamble.
- Turn the unlogged-brew salvage fallback into a discovery event: journal and celebrate a combination that resolves to a new stable formula.
- Move recipe, herb, rapport and request tuning into validated data tables with fixtures for edge-case brews.
- Add brew-resolution tests covering ingredient quality, recipe discovery, town requests, and tower-floor unlock conditions.

## Gathering & progression

- Give two or three biomes a signature gathering hook (night-only bloom, weather-triggered spawn, rare combination variant) rather than spreading effort across all thirteen. (The Sunscar has one: night bloom, noon resin, post-rain salt pan. The containment floor has one: the tower's own ward cycle.)
- Make biome-native ingredients recipe anchors so each biome is the supply line for specific potions.
- Convert at least one floor gate from a brew-count/coin threshold to a mastery check — deliver a Masterwork, discover a morph, satisfy a multi-trait request.

## Story, NPCs & world state

- Write the story bible locking the wizard's backstory, the failed intervention, the ecosystem model, and the act-by-act reveal order.
- Build the pollinator-collapse quest chain. (The failing-harvest chain is done: Rowan's three beats end with the town bed rows turned.)
- Give each NPC a three-beat arc (setup → complication → payoff) tied to rapport. Rowan has one; Mira, Brin, Elric, Ione and Lyra still carry a single one-shot request each.
- Make rapport pay off mechanically: a recipe hint, a discount, or a personal side quest.
- Add repeatable town-board requests to sustain the mid-game between story beats.
- Add more visible town-state changes after a chain completes — a reopened stall, fuller greenhouse beds, lit streets. (The mechanism exists: a gather node can name a `required_completed_quest`, which is how the town square starts growing once Rowan's row is turned.)

## Applied alchemy

- Implement the apply-potion-to-target flow (wilted route plant, frightened creature, blocked path) on top of the existing effect-kind system. Delivery is currently just handing an item to an NPC, which leaves the game's stated premise unexpressed.

## Presentation

- World and character art pass: the ornate HUD currently frames a placeholder world, and that inversion is the weakest first impression.
- Offer a quieter HUD option so the world reads as the visual star.
- Replace the procedural placeholder one-shots with hand-authored ambient audio and music.
- Unify overlay navigation through a typed overlay state model so archive, journal, pause, dialogue and alchemy screens cannot conflict.
- Extract repeated overlay widgets into toolkit-backed helpers shared by the formula, preview, journal and archive panels.

## Scope

- Decide deliberately whether this is a tight three-to-four-hour experience or needs a long tail; if the latter, add a post-ending sandbox with continued town requests.
