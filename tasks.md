# Product Notes

This file is for higher-level product and design follow-ups only.

Implementation status and shipped polish tracking live in [polish_backlog.md](/H:/RustGames/alchemy_tower/polish_backlog.md). Current implemented behavior is documented in [IMPLEMENTED_SYSTEMS.md](/H:/RustGames/alchemy_tower/IMPLEMENTED_SYSTEMS.md).

## Open Product Questions

1. Save/load UX
Should save and load remain `F5` / `F9` shortcuts for the MVP, or move into pause/menu UI before release?

2. Alchemy repetition
Should `repeat last brew setup` be a direct input in the alchemy station, a journal/archive action, or both?

3. Journal identity
What should make the journal feel like a player memory system rather than a report log: authored recap text, stronger summaries, pinned notes, or custom player-facing bookmarks?

4. Economy balance
Run a dedicated value pass on buy prices, sell prices, unlock costs, quest rewards, and rune/duplication value so no single loop dominates progression.

5. First-hour validation
Do a fresh-user pass and record:
- time to first successful brew
- time to first unlock
- time to first NPC hand-in
- time to first meaningful gather discovery
- time to first useful journal/archive revisit

6. Audio scope
Decide whether MVP ships silent or with a minimal audio layer for gather, brewing, unlocks, selling, quests, and journal discovery.
