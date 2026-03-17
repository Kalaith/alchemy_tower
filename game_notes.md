# Wizard Tower Alchemy (Design Notes)

## Core Premise

You discover an abandoned wizard‑alchemy tower after the wizard disappears (or dies… or transforms into something awkwardly non‑human).

At first you can only access one dusty room. Over time you restore the tower, unlocking deeper magical systems and stranger experiments.

The tower becomes both:

- your base
- your progression tree
- a mystery about the previous wizard

Every room you unlock changes gameplay.

---

## Tower Structure (Vertical Tech Tree)

### Floor 1: Entry Laboratory

Your starting room.

**Contains:**

- cracked alchemy table
- a few weak ingredients
- a small cauldron

**Gameplay loop:**

1. Gather ingredients (plants, bugs, minerals)
2. Brew potions
3. Sell them or use them

**Early potions might be:**

- healing draught
- stamina tonic
- glow potion

Nothing fancy yet—mostly experimentation.

### Floor 2: Ingredient Garden 🌿

You unlock the tower greenhouse.

**Now you can:**

- grow magical plants
- mutate plants using potions
- harvest rare ingredients

**Example ingredients:**

- Moon Fern
- Ember Root
- Whisper Moss

Plants can cross‑pollinate, producing weird variants.

### Floor 3: Creature Containment 🐍

A magical menagerie.

**Now gameplay expands:**

- capture creatures
- harvest venom, scales, essence
- breed magical animals

**Examples:**

- spark toads
- mist moths
- miniature drakes

Each creature unlocks new potion types.

### Floor 4: Rune Workshop 🔮

You discover the wizard also used alchemy + runes.

**Now you can:**

- enchant potions
- create magical items
- alter potion effects

**Example:**

- Healing Potion + Rune of Echo → Healing that triggers twice

### Floor 5: Time‑Locked Archives 📜

A magical library sealed with puzzles.

**Inside:**

- spell blueprints
- lost formulas
- the wizard’s journals

The journals slowly reveal what happened to the wizard.

### Floor 6+: Strange Rooms

Now the tower becomes weird.

**Possible rooms:**

- Portal Observatory (ingredients from other worlds)
- Homunculus Lab (create assistants)
- Weather Engine (affects ingredient growth)
- Philosopher Engine (end‑game crafting)

---

## Main Gameplay Loop

The core cycle might be:

1. Explore world for ingredients
2. Experiment with alchemy
3. Unlock tower rooms
4. Discover wizard secrets
5. Create increasingly bizarre potions

Players gradually go from:

- Village herbalist
- arcane scientist
- unstable magical researcher

---

## The Mystery Layer (Important)

The wizard left clues, but the journals get… concerning.

**Example journal progression:**

- Early journal: “The tower will be my legacy.”
- Later journal: “The experiments are working… mostly.”
- Late journal: “The body rejects the transformation.”

**Final hint:**

The wizard might still be in the tower.

Maybe they are:

- turned into a creature
- trapped in a mirror
- merged with the tower core
- or the player is actually the wizard reborn

---

## Progression Idea

Instead of leveling up a character, you level up the tower itself.

**Tower upgrades might include:**

- bigger brewing cauldrons
- faster ingredient growth
- additional potion effects
- automated brewing

Eventually the tower becomes a magical factory of absurdity.

---

## Fun Twist (Possible)

Potions sometimes have unpredictable side effects.

**Example:** Invisibility potion may also:

- turn you into fog
- summon ghosts
- duplicate you

Players might discover recipes accidentally.

---

## Why This Idea Works

It has:

- a cozy gameplay loop
- a mystery story
- strong progression
- room‑based unlocks (very satisfying)
- lots of expandable systems

It could work as:

- a mobile game
- a PC crafting sim
- an idle tower builder

> 💡 The tower could run while you're offline, brewing potions and growing ingredients. This fits well with simulation‑style systems (e.g., an idle progression loop).

---

## Recommended Format (Top‑Down 2D — Safest Option)

For a solo developer without strong art resources, a **top‑down 2D tile‑based tower** is the easiest and most reliable path.

### Why Top‑Down 2D?

- **Art is cheap and plentiful:** tons of free/cheap tilesets exist.
- **Mix-and-match friendly:** assets can be combined without needing a consistent perspective.
- **Simple technical requirements:** collision, camera, and level layout are straightforward.
- **Room‑based design fits naturally:** each room can be its own screen.

### What you need (minimal art requirements)

- tile sets for floors/walls
- simple object sprites (tables, cauldrons, plants, creatures)
- UI icons

### Tower layout idea

A cross‑section tower view (dollhouse style) works especially well:

```
 ┌───────────────┐
 │ Rune Workshop │
 ├───────────────┤
 │ Creature Lab  │
 ├───────────────┤
 │ Greenhouse    │
 ├───────────────┤
 │ Entry Lab     │
 └───────────────┘
```

Each room can be a single screen, which drastically reduces art needs and keeps the design clear.

---

## Style Tips (Keep it consistent)

Use style limitation as a design choice:

- parchment UI
- sketch style
- minimalist shapes

When everything matches a single style, players stop noticing the lack of assets.
