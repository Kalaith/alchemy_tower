# Polish Backlog

This file is the current source of truth for post-MVP polish work.

The goal is to improve clarity, game feel, and consistency without expanding the design scope unnecessarily.

## Priority Order

1. Documentation cleanup and maintenance
2. HUD readability and information hierarchy
3. Control discoverability and onboarding
4. Progression guidance and next-goal messaging
5. Alchemy usability improvements
6. Juice, audio, and interaction feedback
7. Inventory confidence and decision support
8. Archive and journal retention improvements
9. World readability and pacing polish

## 1. Documentation Cleanup and Maintenance

Why:
- The project has grown past several older notes.
- Stale docs make polish planning harder because they misstate what already exists.

Tasks:
- [x] Replace the outdated `not_yet_implemented.md` note with an accurate current-status pointer.
- [x] Keep `IMPLEMENTED_SYSTEMS.md` aligned with shipped code after each feature or polish pass.
- [x] Keep `tasks.md` focused on design/product notes and avoid mixing it with stale implementation status.
- [x] When a polish item lands, update this file instead of creating one-off scratch notes unless the work needs a dedicated refactor doc.

## 2. HUD Readability and Information Hierarchy

Why:
- The game already tracks useful state, but the HUD presentation can be clearer.
- Current status/progress lines compete for the same visual space.

Tasks:
- [x] Separate status text and tower-progress text into different rows.
- [x] Stop capping displayed tower progress at `10` brews when the underlying progression continues beyond that.
- [x] Make the current quest and its location hint more prominent.
- [x] Add a compact gameplay controls strip such as `E interact`, `J journal`, and `Esc pause`.
- [x] Replace the always-on multi-panel HUD with a compact status layout, slot-based potion belt, and collapsible right-side drawer.
- [x] Review inventory/effects/potion panel spacing at common desktop and smaller window sizes.

## 3. Control Discoverability and Onboarding

Why:
- The MVP already has many useful inputs, but several of them are learned only by reading footer text or code.

Tasks:
- [x] Expand menu control copy to surface the most important actions before gameplay starts.
- [x] Add first-run tutorial toasts for journal, brewing, potion belt use, and save/load.
- [x] Show contextual prompts for nearby stations more aggressively.
- [x] Make alchemy-specific controls easier to parse inside the alchemy overlay.
- [x] Review whether save/load controls should remain developer-like shortcuts or move into pause/menu UI.

## 4. Progression Guidance and Next-Goal Messaging

Why:
- The progression logic exists, but players should not need to hit a gate to understand what they are working toward.

Tasks:
- [x] Add a `Next unlock` or `Current objective` summary to the HUD or journal.
- [x] Surface the closest unlock requirement: brews, coins, item turn-in, or milestone.
- [x] Make route and floor unlock requirements visible before the player walks into the blocked warp.
- [x] Strengthen active quest guidance using the existing location-hint logic.
- [x] Highlight meaningful milestones such as first greenhouse access, archive reconstruction readiness, and observatory readiness.

## 5. Alchemy Usability Improvements

Why:
- Brewing is the center of the game, so readability and confidence here matter more than adding a new system.

Tasks:
- [x] Make failure and instability reasons easier to scan in the preview panel.
- [x] Differentiate fully unknown results from known-but-imperfect outcomes more clearly.
- [x] Show reserved ingredient counts and ready counts more explicitly.
- [x] Add a `repeat last brew setup` action.
- [x] Make recipe discovery, mastery gains, and new best-quality results more visible.
- [x] Review alchemy footer language so it matches the rest of the station UI.

## 6. Juice, Audio, and Interaction Feedback

Why:
- Strong feedback can make the existing MVP feel substantially more finished.

Tasks:
- [ ] Extend the placeholder audio pass beyond movement, gather pickup, and brewing into quest completion, unlocks, selling, and journal discovery.
- [x] Strengthen particles or ring effects for rare gathers, recipe discoveries, and unlocks.
- [x] Add subtle camera shake or emphasis on major interactions.
- [x] Add a stronger day-rollover moment.
- [x] Add a distinct flourish when a floor or route is restored.

## 7. Inventory Confidence and Decision Support

Why:
- Inventory friction grows as content depth increases.
- Players should be able to trust what is safe to sell and what is worth keeping.

Tasks:
- [x] Strengthen badges for quest-critical, recipe-critical, best-quality, and safe-to-sell items.
- [x] Add sort/grouping options for ingredients, potions, quest items, and sellables.
- [x] Improve explanation text around `safe stock`.
- [x] Keep reserved counts and recipe relevance visible wherever items are selected.
- [ ] Review sell pricing and safe-sell rules alongside economy balance.

## 8. Archive and Journal Retention Improvements

Why:
- The archive and journal features exist and are useful, but they can hold more of the player's learned history.

Tasks:
- [x] Increase or redesign the experiment-log cap so longer saves retain more useful history.
- [x] Add paging or filtering to experiment history and archive lists.
- [x] Cross-link recipes, morph outcomes, and best attempts where practical.
- [x] Make the journal feel more like a player memory system than a static report.

## 9. World Readability and Pacing Polish

Why:
- The simulation layer is already there; presentation and pacing are the main remaining gains.

Tasks:
- [x] Give important stations and node types stronger silhouettes and ambient visual identity.
- [x] Replace placeholder shape-only presentation with a generated art pass for areas, stations, characters, item icons, journal tabs, toasts, and feedback effects.
- [x] Improve weather/time-of-day visual feedback.
- [x] Review NPC readability so key turn-in characters remain easy to find.
- [x] Smooth first-hour readability for first gather, first brew, first unlock, and first NPC hand-in with contextual prompts and tutorial hints.
- [x] Complete the authored surface biome pass with distinct layouts, biome-native resources, seasonal spawn plans, biome prop art, and refreshed background plates.
- [ ] Run a fresh-user first-hour pass and revisit archive/journal-return pacing with that feedback.

## Notes

- Prefer polishing the existing loop over adding net-new systems.
- If a polish item reveals a real usability gap, fix the gap directly rather than layering extra UI around it.
- Keep this file current as items land.
