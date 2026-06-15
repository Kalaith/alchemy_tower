# Alchemy Tower

Alchemy Tower is a cozy exploration and potion-making game about restoring an abandoned tower and reconnecting with the town around it.

You gather ingredients, learn recipes, help townsfolk, and unlock new tower floors. The focus is discovery, preparation, and gentle progression rather than combat.

## Gameplay

- Explore the tower, town, and surrounding wilds.
- Gather herbs and ingredients under different conditions.
- Brew potions through station-based alchemy.
- Complete town requests and build rapport with locals.
- Restore tower floors to unlock new spaces and routines.

## Goal

Turn the abandoned tower into a working magical home while learning the rhythms of the valley and expanding your recipe knowledge.

## Controls

- WASD: move.
- E: interact.
- Esc: pause.

## Current Scope

Playable exploration, gathering, brewing, requests, tower restoration, inventory flow, and save/load progression.
# Practical Future Improvements

- Add brew-resolution tests for ingredient quality, recipe discovery, town requests, and tower-floor unlock conditions.
- Unify overlay navigation through a typed overlay state model so archive, journal, pause, dialogue, and alchemy screens cannot conflict.
- Move recipe, herb, rapport, and request tuning into validated data tables with small fixtures for edge-case brews.
- Extract repeated overlay widgets into toolkit-backed helpers shared by formula, preview, journal, and archive panels.

