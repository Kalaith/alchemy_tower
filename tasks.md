# Product Notes

This file is for higher-level product and design follow-ups only.

Implementation status and shipped polish tracking live in [polish_backlog.md](/H:/RustGames/alchemy_tower/polish_backlog.md). Current implemented behavior is documented in [IMPLEMENTED_SYSTEMS.md](/H:/RustGames/alchemy_tower/IMPLEMENTED_SYSTEMS.md).

## Open Product Questions

1. Save/load UX
Move save/load into the pause menu before release.

2. Alchemy repetition
Keep `repeat last brew setup` in the alchemy station.

3. Journal identity
Make the journal a player memory system for herbs and potions they have created or learned about, with authored flavor-text summaries and recap-style notes rather than a dry report log.

4. Economy balance
Deferred for now.

5. First-hour validation
Deferred for now. When revisited, record:
- time to first successful brew
- time to first unlock
- time to first NPC hand-in
- time to first meaningful gather discovery
- time to first useful journal/archive revisit

6. Audio scope
Add a minimal procedural placeholder audio layer first, focused on walking, herb pickup, and brewing.

9. UI/System follow-through
- [x] Add save/load actions to the pause menu and keep `F5` / `F9` as secondary shortcuts.
- [x] Add `repeat last brew setup` directly in the alchemy station UI/input flow.
- [x] Rework the journal toward player memory rather than report logging:
  herb memories with learned conditions and flavor summaries
  potion memories with authored recap text for discovered outputs
  a clearer distinction between "seen", "learned", and "successfully brewed"
- [x] Add a minimal procedural placeholder audio pass for:
  walking/footstep loops
  herb pickup
  brewing interactions
- [ ] Decide whether procedural placeholder audio should later be replaced wholesale or only used as a base layer for hand-authored polish.

7. Biome progression follow-through
The biome pass now has authored layouts and seasonal spawn identity. Remaining product questions:
- Which new biome-native ingredients should become recipe anchors or quest asks rather than staying as sell-and-salvage options?
- Which biome should introduce the first stronger progression gate after the current opening plains-to-town rhythm?
