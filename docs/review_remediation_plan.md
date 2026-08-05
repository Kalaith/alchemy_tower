# Fresh Review Remediation Plan

Recorded 2026-08-05 after a static system audit, content/economy analysis,
`publish.ps1` validation, and a browser playcheck of the deployed WebGL build.

The review found that Alchemy Tower's feature graph is coherent, but runtime
startup, bottle-grade continuity, progression ordering, and a handful of
player-facing summaries do not yet uphold the design. Work through the phases
below in order. Each major phase receives its own commit after its acceptance
checks pass.

## 1. Restore the published WebGL runtime

- [x] Make the shared publisher deploy the JavaScript bundle belonging to the
  resolved Macroquad version, including enabled Macroquad plugins such as audio,
  instead of deploying only Miniquad's `gl.js`.
- [x] Add a publisher regression check that rejects a runtime bundle missing an
  import required by the built game, with `audio_source_is_loaded` covering the
  failure observed in this review.
- [x] Publish Alchemy Tower and verify in a browser that the game reaches its
  menu without missing-import warnings.

Ownership: `rust_management`, because every game uses the shared runtime under
`shared-assets/runtime/`.

## 2. Preserve bottle grades through transformations

- [x] Rune imbuing must consume a definite source bottle and transfer its live
  quality score, band, and inherited traits to the output bottle.
- [x] Potion duplication must copy a definite source bottle's live grade and
  traits rather than creating an untracked Serviceable bottle.
- [x] Add regression tests using mixed-grade stacks and graded rune-only quest
  outputs.
- [x] Prove that Mira's Fine Fieldwide Poultices and Elric's Excellent
  Nightwatch Lanterns can be produced and delivered, reopening the Southern
  Pass progression chain.

## 3. Refresh world availability when its conditions change

- [x] Refresh gather-node availability when the clock crosses a time-window
  boundary, not only on area entry or day rollover.
- [x] Refresh the current area's nodes after a treatment or completed story beat
  changes their quest/milestone gates.
- [x] Add tests that wait across morning/day/evening/night boundaries without
  changing area, and tests that restore a same-area gated node.

## 4. Restore the intended reveal order

- [x] Require Ione's reconciled eleven-month record before archive
  reconstruction can produce `archive_revelation`.
- [x] Add a progression test that proves the archive remains closed before
  `record_reconciled`, `eleven_months_restored`, and `the_previous_hand`, then
  opens after the required evidence is present.

## 5. Make quality consistent at every player-facing boundary

- [x] World treatments must choose and spend a qualifying live bottle from a
  mixed-grade stack, worst acceptable first.
- [x] Quest requirement summaries must describe currently held qualifying
  bottles, not the historical best-ever crafted profile.
- [x] The seventh successful brew must receive the mastery output bonus on the
  same brew that changes the formula's stage to Mastered.
- [x] Drinking a higher-quality potion must improve its authored effect in the
  ways promised by the alchemy design: restoration magnitude and timed-effect
  duration, with data-driven balance values and tests.

## 6. Final balance and release audit

- [ ] Run formatting and focused tests after each phase.
- [ ] Run `publish.ps1` with no parameters after the complete game-side change.
- [ ] Repeat the deployed browser playcheck.
- [ ] Recalculate one-off rewards, repeatable-order income and bottle demand,
  commission sinks, quality multipliers, and the reachable critical path after
  the progression fixes.
- [ ] Confirm every item above with direct tests or runtime evidence; do not mark
  the plan complete from reference-integrity tests alone.
