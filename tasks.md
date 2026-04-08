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
- [ ] Rework the journal toward player memory rather than report logging:
  herb memories with learned conditions and flavor summaries
  potion memories with authored recap text for discovered outputs
  a clearer distinction between "seen", "learned", and "successfully brewed"
- [x] Add a minimal procedural placeholder audio pass for:
  walking/footstep loops
  herb pickup
  brewing interactions
- [ ] Decide whether procedural placeholder audio should later be replaced wholesale or only used as a base layer for hand-authored polish.

7. Biome-specific herbs
Now that the surface map is split into plains, rocks, forest, lake, desert, and rainforest biomes, define which ingredients should become biome-exclusive and what progression each biome should teach.

8. Surface biome map pass
- [ ] Redesign each non-town, non-tower surface map so it has a distinct authored layout rather than reusing the same blocker pattern.
- [ ] Define a custom traversal identity for each surface biome map:
  `north_plains`: open field roads and low grass obstacles.
  `rock_fields`: broken quarry lanes, choke points, and mineral clusters.
  `moonlit_forest`: winding tree paths, glades, and denser canopy pockets.
  `lake_shore`: shoreline curves, reed beds, and shallow-water edges.
  `sunscar_desert`: dune lanes, exposed stone, and sparse shelter pockets.
  `tropical_rainforest`: layered jungle paths, root arches, and wet clearings.
- [ ] Treat Spring as the starting season baseline for the first balancing pass.
- [ ] For each non-town, non-tower surface map, define which herbs/resources can spawn there in Spring and at what time windows:
  `north_plains`
  `rock_fields`
  `moonlit_forest`
  `lake_shore`
  `sunscar_desert`
  `tropical_rainforest`
- [ ] Expand the same spawn plan after Spring for Summer, Autumn, and Winter so each biome has seasonal identity rather than just location identity.
- [ ] Decide which current shared ingredients remain multi-biome and which become biome-exclusive.
- [ ] Add new biome-native ingredients where the current herb set is too small to differentiate the maps cleanly.
- [ ] Create procedural biome prop art to support map readability:
  rocks and quarry shelves for `rock_fields`
  tree clusters, trunks, and canopy props for `moonlit_forest`
  reeds, shoreline stones, and water-edge props for `lake_shore`
  dunes, dry shrubs, and cracked stone props for `sunscar_desert`
  tropical trees, roots, broad-leaf plants, and wet undergrowth props for `tropical_rainforest`
  grass tufts, flowers, and field markers for `north_plains`
- [ ] Create procedural herb/node art variants so different biomes do not reuse the same visual treatment for every gatherable.
- [ ] Update generated background plates after the custom layouts are authored so each biome reads correctly at gameplay scale.
- [ ] Revisit gather-node placement and NPC route readability once the custom layouts replace the placeholder blocker layouts.
