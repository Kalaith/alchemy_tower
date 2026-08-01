# TODO — Alchemy Tower

## Core loop & alchemy

- Author 15–25 recipes with overlapping ingredients and more morph paths, so the quality/trait engine drives real brew decisions instead of a lookup. (26 recipes, 12 morph branches across three catalyst tags; the starter recipe now morphs, so the layer teaches itself.)
- Add risk/reward to heat, stir and timing so pushing a quality band is a gamble. (Overcharge/instability does this for heat and stirs. Ingredient order is now a third axis: 13 recipes carry a trait-ordered method.)
- Turn the unlogged-brew salvage fallback into a discovery event: journal and celebrate a combination that resolves to a new stable formula.
- Move recipe, herb, rapport and request tuning into validated data tables with fixtures for edge-case brews.
- Add brew-resolution tests covering ingredient quality, recipe discovery, town requests, and tower-floor unlock conditions.

## Gathering & progression

- Give two or three biomes a signature gathering hook (night-only bloom, weather-triggered spawn, rare combination variant) rather than spreading effort across all thirteen. (The Sunscar has one: night bloom, noon resin, post-rain salt pan. The containment floor has one: the tower's own ward cycle. The moonlit forest's charred hollow has one: it is the valley's winter ground. The rock fields have one: the quarry is the only ground rain improves. The north plains have one: they are the wind. That is four, past the two or three this asked for.)
- Make biome-native ingredients recipe anchors so each biome is the supply line for specific potions.
- Convert at least one floor gate from a brew-count/coin threshold to a mastery check — deliver a Masterwork, discover a morph, satisfy a multi-trait request. (Done: containment→rune workshop needs the Glow Potion mastered. The archive reconstruction now also requires the two ecological chain payoffs.)

## Story, NPCs & world state

- Write the story bible locking the wizard's backstory, the failed intervention, the ecosystem model, and the act-by-act reveal order.
- Both named chains are done: failing-harvest ends with Rowan's bed rows turned, pollinator-collapse ends with Lyra's valley flowering at once. Any further chain is new design, not backlog.
- All six townsfolk have a three-beat arc (setup → complication → payoff). Rapport payoff (friendship gift) is separate and already in.
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

- Decide deliberately whether this is a tight three-to-four-hour experience or needs a long tail; if the latter, add a post-ending sandbox with continued town requests. (The valley has grown to 14 areas and 6 completed arcs; repeatable board orders and standing contracts already carry the mid-game, so the long tail is the direction it has taken in practice.)
