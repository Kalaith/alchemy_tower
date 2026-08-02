use super::GameplayState;
use crate::data::{GameData, WildVariantDefinition};

impl GameplayState {
    /// The wild variant this ingredient would come up as under the current sky,
    /// if any. Gathering records the variant's *id* against the stock it just
    /// put in the bag, so the bench can spend it later — the name alone was
    /// enough while this only ever fed a journal line.
    pub(super) fn matching_wild_variant<'a>(
        &self,
        data: &'a GameData,
        item_id: &str,
    ) -> Option<&'a WildVariantDefinition> {
        data.item(item_id)?.wild_variants.iter().find(|variant| {
            variant
                .required_conditions
                .iter()
                .all(|condition| self.condition_matches(condition))
        })
    }

    pub(super) fn current_item_quality_snapshot(
        &self,
        data: &GameData,
        item_id: &str,
    ) -> Option<(u32, String)> {
        let item = data.item(item_id)?;
        let variant = self.matching_wild_variant(data, item_id);
        let quality = item.quality + variant.map(|variant| variant.quality_bonus).unwrap_or(0);
        let variant_name = variant
            .map(|variant| variant.name.clone())
            .unwrap_or_default();
        Some((quality.min(100), variant_name))
    }

    pub(super) fn condition_matches(&self, condition: &str) -> bool {
        let condition = condition.to_ascii_lowercase();
        condition.contains(self.current_season())
            || condition.contains(self.current_weather())
            || condition.contains(self.current_time_window())
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;

    /// A wild variant is a compound-condition reward: gather this herb on the
    /// right day and it comes up better. But the variant's conditions have to
    /// co-occur with the *node's own* season and weather gates, and nothing
    /// checked that they could.
    ///
    /// One could not. `quarry_lichen_sparked` wants wind — wind is the whole
    /// idea of a sparked lichen — and its ledge only ever appeared in clear or
    /// mist, so eight quality points and a `volatile` trait had never once been
    /// reachable. The ledge takes wind now; this stops the next one.
    /// The reachability check above proves the conditions *can* line up. This
    /// proves the payoff actually resolves when they do: on a clear winter
    /// morning the pass flower should come up as its variant and be worth more
    /// than the plain one, through the same call the game makes.
    #[test]
    fn a_clear_winter_morning_opens_the_rimeflower() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        // Day 16 is winter and clear; 08:00 is inside the morning window.
        state.world.day_index = 16;
        state.set_clock_minutes(480.0);
        assert_eq!(state.current_season(), "winter");
        assert_eq!(state.current_weather(), "clear");
        assert_eq!(state.current_time_window(), "morning");

        let (quality, variant) = state
            .current_item_quality_snapshot(&data, "rimeflower")
            .expect("rimeflower should exist");
        let base = data.item("rimeflower").expect("rimeflower").quality;

        assert_eq!(variant, "Full-Open Rimeflower");
        assert!(
            quality > base,
            "the variant should be worth more than the plain flower: {quality} vs {base}"
        );
    }

    #[test]
    fn every_wild_variant_can_actually_be_found() {
        const TIME_WINDOWS: [&str; 4] = ["morning", "day", "evening", "night"];
        // Season cycles every five days, weather every four: twenty covers all.
        const CYCLE_DAYS: u32 = 20;

        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        let mut unreachable = Vec::new();
        for item in &data.items {
            for variant in &item.wild_variants {
                let found = data.areas.iter().any(|area| {
                    area.gather_nodes
                        .iter()
                        .filter(|node| node.item_id == item.id)
                        .any(|node| {
                            (0..CYCLE_DAYS).any(|day| {
                                state.world.day_index = day;
                                let season_ok = node.seasons.is_empty()
                                    || node.seasons.iter().any(|s| s == state.current_season());
                                let weather_ok = node.weathers.is_empty()
                                    || node.weathers.iter().any(|w| w == state.current_weather());
                                if !season_ok || !weather_ok {
                                    return false;
                                }
                                TIME_WINDOWS.iter().any(|window| {
                                    if !node.time_windows.is_empty()
                                        && !node.time_windows.iter().any(|w| w == window)
                                    {
                                        return false;
                                    }
                                    // Put the clock inside that window, then ask
                                    // the real matcher rather than reimplementing it.
                                    state.set_clock_minutes(match *window {
                                        "morning" => 480.0,
                                        "day" => 840.0,
                                        "evening" => 1140.0,
                                        _ => 1320.0,
                                    });
                                    variant
                                        .required_conditions
                                        .iter()
                                        .all(|condition| state.condition_matches(condition))
                                })
                            })
                        })
                });
                if !found {
                    unreachable.push(format!(
                        "{} on {}: {:?}",
                        variant.id, item.id, variant.required_conditions
                    ));
                }
            }
        }
        unreachable.sort();

        assert!(
            unreachable.is_empty(),
            "wild variants whose conditions never line up with their node:
{unreachable:#?}"
        );
    }
}
