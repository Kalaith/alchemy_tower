# Phase 1 Core Town Cast Plan

## Goal

Define the minimum story, content, and world changes needed to support the Phase 1 cast in a way that feels intentional, personal, and playable.

Phase 1 should establish:

- a small memorable town cast
- the valley’s ecological decline
- the crow as mentor and emotional guide
- alchemy as care and restoration, not abstract crafting

This document is about what actually needs to change in the game content to support that phase.

## Phase 1 Cast

The starting cast should be:

- the Crow
- Mira
- Rowan
- Mayor Elric
- Ione
- Brin
- Lyra

The player should meet this cast repeatedly before many new faces appear.

## Phase 1 Narrative Job

By the end of Phase 1, the player should understand:

- the town is struggling, but not collapsed
- the land is out of balance
- each NPC is feeling a different symptom of the same larger problem
- the tower may once have helped the valley
- the crow understands alchemy and the tower, but not the full truth
- restoring the greenhouse is the first meaningful step toward recovery

## What Needs To Change

## 1. Reframe Existing NPCs Around Human Stakes

Current quest structures already work mechanically. The first content pass should change why people ask for help and what those requests mean.

### Mira

Current phase role:
- overworked caretaker or healer

Needed content changes:
- rewrite dialogue so her request is about strain, headaches, and carrying too much responsibility
- make her concern personal and recurring, not a one-off potion ask
- add post-quest dialogue showing that help actually changes her energy or routine

What this supports:
- alchemy as care
- early emotional trust
- “people need help now” framing

### Rowan

Current phase role:
- nervous courier / lamplighter / dusk traveler

Needed content changes:
- rewrite request around fear of dark routes and the feeling that the roads are changing
- add schedule/dialogue that makes dusk feel meaningfully different for Rowan
- show visible payoff after help, such as safer routes, lit lamps, or calmer dialogue

What this supports:
- route anxiety
- environmental unease
- visible practical payoff

### Mayor Elric

Current phase role:
- civic pressure and town continuity

Needed content changes:
- rewrite dialogue to frame the town’s issue as a slow ecological crisis rather than generic hardship
- give him lines that connect multiple NPC problems into one civic problem
- make him the clearest early source of “the valley is draining”

What this supports:
- shared crisis framing
- player purpose beyond isolated errands

### Ione

Current phase role:
- scholar / archivist-adjacent observer

Needed content changes:
- rewrite dialogue so she notices patterns and fragments rather than dumping lore
- use her to point at the tower’s lost purpose, not just its mystery
- give her post-restoration lines that reinterpret earlier events

What this supports:
- story investigation
- transition from local care to larger understanding

### Brin

Current phase role:
- grower / gardener / farmer

Needed content changes:
- make Brin the clearest voice of crop failure, poor soil, and missing pollination
- rewrite the greenhouse-linked request so it feels like hope for the town, not just a feature unlock
- add visible world payoff after success: healthier beds, public flowers, stronger growth

What this supports:
- greenhouse meaning
- ecosystem framing
- first major restoration payoff

### Lyra

Current phase role:
- creature handler / naturalist / wary observer

Needed content changes:
- rewrite dialogue so creatures are behaving differently because the ecosystem is stressed
- make her less about “containment unlock soon” and more about imbalance in species behavior
- give her lines that connect creature changes to missing plants, polluted places, or disrupted cycles

What this supports:
- future containment arc
- ecosystem interdependence
- broader crisis texture

### The Crow

Current phase role:
- mentor, companion, unsettling witness

Needed content changes:
- introduce the crow early and repeatedly
- give them practical alchemy guidance instead of abstract mystery dialogue
- let them know techniques instinctively while failing to explain why
- make their emotional beats subtle but noticeable when the player restores important spaces

What this supports:
- tutorial delivery
- warmth and continuity
- story thread without needing a hard secret-identity twist

## 2. Rewrite The First Quest Layer

The first quest layer needs to change from “starter item requests” into a coherent town-introduction arc.

Needed changes:

- rewrite every early quest blurb around a person and a local symptom
- connect at least three of the early requests to one shared cause
- make the first greenhouse-related quest feel like the town’s first meaningful win
- ensure each quest completion changes at least one line of dialogue in another NPC

Minimum implementation target:

- Mira quest rewrite
- Rowan quest rewrite
- Brin quest rewrite
- one Mayor Elric follow-up line after each major early help
- one Crow line after each first successful quest turn-in

## 3. Add A Phase 1 Dialogue State Structure

Right now the game has dialogue variants, progress states, and relationship tracking. Phase 1 needs clearer authored dialogue stages.

Each core NPC should have:

- intro state
- pre-help concern state
- active request state
- post-help relief state
- town-recovery observation state

The crow should also have:

- first meeting state
- first brew state
- first quest-complete reaction
- first tower-restoration reaction

Implementation need:

- define a simple per-NPC dialogue progression map tied to quest completion and milestone state
- avoid adding large branching trees; use staged authored text instead

## 4. Make The Town Visibly Reflect The Cast

Phase 1 succeeds only if the cast feels embedded in the town.

Needed world changes:

- each core NPC should have a clear “place” in town
- player should be able to associate NPCs with a role, routine, and part of the crisis
- early quest completions should change small visual details in town

Examples:

- Mira’s area looks less strained or more organized after help
- Rowan’s route gains visible lamp safety
- Brin’s beds or public greenery improve
- Mayor Elric has new lines or stance showing cautious optimism

Minimum implementation target:

- at least 2 visible town-state changes in Phase 1
- at least 3 dialogue updates across the cast reacting to those changes

## 5. Clarify Why The Player Should Care

The player motivation needs to be explicit in content, not only implied by systems.

Needed changes:

- opening status and dialogue should establish that the land is failing and the town is running out of resilience
- the crow should frame alchemy as a way to help stabilize what is breaking
- Mayor Elric and Brin should make it clear that success would help the whole town, not just one room in the tower

Phase 1 player takeaway:

- I am not just learning recipes
- I am helping people survive a worsening imbalance
- the tower once mattered to this valley
- restoring it might help everyone

## 6. Support A Small-Cast Presentation

If Phase 1 is meant to feel intimate, the presentation should reinforce that.

Needed changes:

- do not add many new named NPCs yet
- let the journal and quest surfaces focus on the same small group repeatedly
- make recurring names and routines feel familiar
- avoid cluttering the town with extra inactive figures unless they serve atmosphere only

Implementation rule:

- every named NPC in Phase 1 should have a clear story function
- if an NPC cannot be described in one sentence, they are probably premature for this phase

## 7. Tie The Greenhouse To Phase 1 Completion

Phase 1 should have a clear endpoint.

Recommended endpoint:

- the player has helped the core town cast enough to understand the wider problem
- the greenhouse restoration becomes the first communal turning point

What needs to be true by then:

- Brin sees the greenhouse as hope, not just a workstation
- the crow reacts to the greenhouse as a place with emotional history
- the town acknowledges that things may actually improve
- the player understands that each future floor may restore another part of the valley’s missing balance

## Concrete Content Deliverables

## Dialogue

- Rewrite the crow’s early tutorial lines.
- Rewrite Mira’s quest and follow-up dialogue.
- Rewrite Rowan’s quest and follow-up dialogue.
- Rewrite Brin’s quest and greenhouse framing.
- Add stronger civic framing lines for Mayor Elric.
- Add pattern-recognition and tower-context lines for Ione.
- Add creature-imbalance setup lines for Lyra.

## Quests

- Rewrite the first wave of quest text to be people-first.
- Add cross-NPC reactions after completions.
- Make one early quest explicitly point toward ecosystem interdependence.
- Make the greenhouse unlock feel like a narrative step, not just progression cost.

## World State

- Add at least two visible town improvements tied to quest completions.
- Add at least one greenhouse-adjacent visual change tied to recovery.
- Add one or two crow-location appearances tied to tower progress.

## Journal And Status Messaging

- Reword early objective text so it points toward helping the town and stabilizing the valley.
- Make early journal entries sound like discoveries and concerns, not bookkeeping.
- Add milestone text for first meaningful town improvement and first communal recovery step.

## Data And Authoring Work

To support this phase cleanly, the following content authoring work is needed:

- revise NPC dialogue data
- revise quest text and reward framing
- add milestone text for early town-recovery beats
- add any small map/state variants needed for visible change
- add or revise crow dialogue triggers tied to early milestones

## Recommended Implementation Order

1. Rewrite the crow’s early role and the first meeting flow.
2. Rewrite Mira, Rowan, and Brin so the first quest wave carries the crisis properly.
3. Add Mayor Elric and Ione follow-up lines that connect individual problems into one town problem.
4. Add Lyra setup dialogue so the containment arc is seeded early.
5. Implement two visible town-state changes after early completions.
6. Reframe greenhouse restoration as the Phase 1 payoff.

## Success Criteria

Phase 1 is working if:

- the player can name the core town cast and what each one cares about
- the town crisis feels shared, not fragmented
- the crow feels useful and emotionally interesting from the beginning
- early quests feel like helping people, not feeding a machine
- restoring the greenhouse feels like the town’s first real breath of relief
