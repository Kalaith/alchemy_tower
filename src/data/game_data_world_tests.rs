//! Whether the authored world can actually be walked: that everything the
//! player must reach can be stood next to, and that the townsfolk are somewhere
//! findable at every hour rather than frozen on one mark.
//!
//! Split out of `game_data_reference_tests.rs` at 731 lines. That file asks
//! whether the content's references resolve; this one asks whether the content
//! is where it says it is.

#[cfg(test)]
mod tests {
    use crate::data::load_embedded;

    /// Two separate things this pins, both found the same way — by checking
    /// authored positions against the map rather than looking at a capture.
    ///
    /// Every townsperson must have somewhere to be in all four time windows.
    /// The whole cast had morning, day and evening and nothing else, and
    /// `active_schedule_index` falls back to the last entry, so from nine at
    /// night until six the next morning the entire valley stood frozen on its
    /// teatime mark — which stopped being invisible the moment night became
    /// worth going out in.
    ///
    /// And no mark may sit inside a blocker. Mayor Elric's day and evening
    /// spots were both inside the hall's footprint: reachable, since 48px is
    /// inside a 56px interaction radius, but rendering him through a wall.
    /// Blockers stop the player with a 14px body radius, so anything buried
    /// deeper inside one than its own reach is content you can see and never
    /// touch. `every_gather_node_can_actually_spawn` does not catch this — the
    /// node spawns perfectly, you simply cannot walk to it.
    ///
    /// Three were buried. The worst was `hollow_ashcap_01`, 88px inside a tree
    /// against a 44px reach, and the **only** source of ashcap in the game, so
    /// the two recipes calling for it could never be brewed. The greenhouse's
    /// north planter is inside a blocker too and is fine, because the blocker
    /// is the raised bed itself and 48px of reach clears it — which is why this
    /// measures reach rather than flagging overlap.
    #[test]
    fn everything_the_player_must_reach_can_be_stood_next_to() {
        /// Matches `PLAYER_RADIUS` in the movement code.
        const PLAYER_RADIUS: f32 = 14.0;

        let data = load_embedded().expect("embedded game data should load");
        let reach_bonus = data.config.interaction_range;
        let mut unreachable = Vec::new();

        // How close the player can get to a point sunk inside scenery.
        let closest_approach =
            |area: &crate::data::AreaDefinition, x: f32, y: f32| -> Option<f32> {
                area.blockers
                    .iter()
                    .filter(|b| x >= b.x && x <= b.x + b.w && y >= b.y && y <= b.y + b.h)
                    .map(|b| {
                        let depth = (x - b.x).min(b.x + b.w - x).min(y - b.y).min(b.y + b.h - y);
                        depth + PLAYER_RADIUS
                    })
                    .fold(None, |acc: Option<f32>, d| {
                        Some(acc.map_or(d, |a| a.max(d)))
                    })
            };

        for area in &data.areas {
            for node in &area.gather_nodes {
                if let Some(approach) = closest_approach(area, node.position[0], node.position[1]) {
                    let reach = node.radius + reach_bonus;
                    if approach > reach {
                        unreachable.push(format!(
                            "gather node {} in {}: closest {approach:.0}, reach {reach:.0}",
                            node.id, area.id
                        ));
                    }
                }
            }
        }
        for station in &data.stations {
            let Some(area) = data.area(&station.area_id) else {
                continue;
            };
            if let Some(approach) = closest_approach(area, station.position[0], station.position[1])
            {
                if approach > station.interaction_radius {
                    unreachable.push(format!(
                        "station {} in {}: closest {approach:.0}, reach {:.0}",
                        station.id, area.id, station.interaction_radius
                    ));
                }
            }
        }
        unreachable.sort();

        assert!(
            unreachable.is_empty(),
            "things the player can see but never walk up to:
{unreachable:#?}"
        );
    }

    #[test]
    fn every_townsperson_has_somewhere_to_be_at_every_hour() {
        const WINDOWS: [&str; 4] = ["morning", "day", "evening", "night"];
        // A body has width; standing flush against a wall reads as inside it.
        const CLEARANCE: f32 = 18.0;

        let data = load_embedded().expect("embedded game data should load");
        let mut complaints = Vec::new();

        for npc in &data.npcs {
            for window in WINDOWS {
                if !npc.schedule.iter().any(|entry| entry.time_window == window) {
                    complaints.push(format!("{}: nowhere to be at {window}", npc.id));
                }
            }

            for entry in &npc.schedule {
                let Some(area) = data.area(&entry.area_id) else {
                    complaints.push(format!(
                        "{} at {}: no such area {}",
                        npc.id, entry.time_window, entry.area_id
                    ));
                    continue;
                };
                let [x, y] = entry.position;
                if x < 0.0 || y < 0.0 || x > area.size[0] || y > area.size[1] {
                    complaints.push(format!(
                        "{} at {}: {x},{y} is outside {}",
                        npc.id, entry.time_window, area.id
                    ));
                }
                for blocker in &area.blockers {
                    if x >= blocker.x - CLEARANCE
                        && x <= blocker.x + blocker.w + CLEARANCE
                        && y >= blocker.y - CLEARANCE
                        && y <= blocker.y + blocker.h + CLEARANCE
                    {
                        complaints.push(format!(
                            "{} at {}: {x},{y} stands in scenery in {}",
                            npc.id, entry.time_window, area.id
                        ));
                    }
                }
            }
        }
        complaints.sort();

        assert!(
            complaints.is_empty(),
            "townsfolk standing somewhere they should not be:
{complaints:#?}"
        );
    }

    /// The check above forces four marks, one per window — and for a long time
    /// content satisfied it by naming the same room four times. Six of the nine
    /// townsfolk never left one area all day, which makes the four marks an
    /// address rather than a schedule.
    ///
    /// It is visible, too: the journal's rapport tab prints a *now*, a *later*
    /// and a *usually* line for each person, and for six of them all three said
    /// the same thing. This asks that the schedule system be used rather than
    /// merely satisfied — a valley with fourteen areas where nobody goes
    /// anywhere is a valley where the world is scenery.
    #[test]
    fn a_schedule_is_not_an_address() {
        let data = load_embedded().expect("embedded game data should load");
        let sedentary = data
            .npcs
            .iter()
            .filter(|npc| {
                npc.schedule
                    .iter()
                    .map(|entry| entry.area_id.as_str())
                    .collect::<std::collections::HashSet<_>>()
                    .len()
                    < 2
            })
            .map(|npc| format!("{} never leaves {}", npc.id, npc.area_id))
            .collect::<Vec<_>>();

        assert!(!data.npcs.is_empty(), "no townsfolk loaded at all");
        assert!(
            sedentary.is_empty(),
            "townsfolk whose whole day happens in one room:\n{sedentary:#?}"
        );
    }

    /// A wild biome you travel to should be a *source*, not a corridor. The
    /// measure that says which is which is not node count — the rainforest had
    /// six nodes and one thing you could not pick closer to home, against four
    /// and five elsewhere, and it read as somewhere to pass through.
    ///
    /// Two is a floor, not a target. `north_plains` sits on it deliberately:
    /// it is the starter ground a new player walks first, and the whole point
    /// of it is that most of what grows there also grows elsewhere.
    #[test]
    fn every_wild_biome_is_a_source_of_something() {
        let data = load_embedded().expect("embedded game data should load");
        let mut where_found: std::collections::HashMap<&str, std::collections::BTreeSet<&str>> =
            std::collections::HashMap::new();
        for area in &data.areas {
            for node in &area.gather_nodes {
                where_found
                    .entry(node.item_id.as_str())
                    .or_default()
                    .insert(area.id.as_str());
            }
        }

        let mut corridors = Vec::new();
        for area in &data.areas {
            // Tower floors are rooms in a building and the square is where the
            // player lives; the rule is about ground you cross the valley to
            // reach. The square's four beds are deliberately the same starter
            // herbs the plains grow, so that a first afternoon has somewhere to
            // practise.
            if area.gather_nodes.is_empty()
                || area.id.contains("floor")
                || area.id == "tower_entry"
                || area.id == "town_square"
            {
                continue;
            }
            let exclusive = area
                .gather_nodes
                .iter()
                .map(|node| node.item_id.as_str())
                .filter(|item_id| {
                    where_found
                        .get(item_id)
                        .is_some_and(|areas| areas.len() == 1)
                })
                .collect::<std::collections::BTreeSet<_>>();
            if exclusive.len() < 2 {
                corridors.push(format!(
                    "{}: {} exclusive gatherable(s)",
                    area.id,
                    exclusive.len()
                ));
            }
        }

        corridors.sort();
        assert!(
            corridors.is_empty(),
            "biomes with nothing of their own:
{corridors:#?}"
        );
    }
}
