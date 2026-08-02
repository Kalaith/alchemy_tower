//! Checks over the applied-alchemy layer: that everything a brew can be poured
//! on could actually be treated, and that treating things opens something.
//!
//! Split out of `game_data_progression_tests.rs`, which was at 837 lines when
//! apply targets landed. Those checks ask whether the game can be *finished*;
//! these ask whether its stated premise is on the critical path.

#[cfg(test)]
mod tests {
    use crate::data::load_embedded;

    /// A target asks for a kind of brew and, sometimes, a grade. Both are
    /// strings in a content file, so both can be wrong in ways nothing else
    /// would catch: an effect kind no potion produces makes a target that can
    /// never be treated, and since gates now wait on targets, that is a wall
    /// with no door rather than a missed flourish.
    #[test]
    fn everything_a_brew_can_be_poured_on_can_actually_be_treated() {
        let data = load_embedded().expect("embedded game data should load");
        let mut targets = 0usize;
        let mut impossible = Vec::new();

        for area in &data.areas {
            for target in &area.apply_targets {
                targets += 1;
                let candidates = data
                    .items
                    .iter()
                    .filter(|item| item.category == crate::data::ItemCategory::Potion)
                    .filter(|item| {
                        item.effects
                            .iter()
                            .any(|effect| effect.kind.to_string() == target.required_effect_kind)
                    })
                    .collect::<Vec<_>>();
                if candidates.is_empty() {
                    impossible.push(format!(
                        "{} wants a {} brew, and nothing in the game does that",
                        target.id, target.required_effect_kind
                    ));
                    continue;
                }
                // Something has to be able to reach the grade it asks for.
                if !target.minimum_quality_band.is_empty()
                    && !candidates
                        .iter()
                        .any(|item| data.recipes.iter().any(|r| r.output_item_id == item.id))
                {
                    impossible.push(format!(
                        "{} wants {} of a {} brew, and no recipe makes one",
                        target.id, target.minimum_quality_band, target.required_effect_kind
                    ));
                }
            }
        }

        assert!(
            targets > 0,
            "nothing in the world can have a brew used on it"
        );
        assert!(
            impossible.is_empty(),
            "targets no brew could ever treat:
{impossible:#?}"
        );
    }

    /// The premise on the critical path. Applying a brew to the world was
    /// supposed to open things, not merely decorate them — if no gate anywhere
    /// waits on a treated target, the whole mechanic is optional scenery.
    #[test]
    fn something_in_the_world_opens_only_by_treating_it() {
        let data = load_embedded().expect("embedded game data should load");
        let target_beats = data
            .areas
            .iter()
            .flat_map(|area| area.apply_targets.iter())
            .flat_map(|target| target.completion_milestones.iter())
            .map(|milestone| milestone.id.clone())
            .collect::<std::collections::HashSet<_>>();

        let gated = data
            .areas
            .iter()
            .flat_map(|area| area.warps.iter())
            .any(|warp| target_beats.contains(&warp.required_journal_milestone))
            || data
                .stations
                .iter()
                .any(|station| target_beats.contains(&station.required_journal_milestone));

        assert!(
            gated,
            "no route or facility waits on a brew being poured on anything"
        );
    }

    /// A flourish waits on a quest or a beat, both of which are strings in an
    /// area file. One that names something that does not exist is a piece of
    /// the world that never appears, and nothing on screen would say why.
    #[test]
    fn every_flourish_waits_on_something_real() {
        use crate::data::game_data_narrative_tests::tests::recordable_milestone_ids;

        let data = load_embedded().expect("embedded game data should load");
        let recordable = recordable_milestone_ids(&data);
        let mut flourishes = 0usize;
        let mut dangling = Vec::new();

        for area in &data.areas {
            for flourish in &area.flourishes {
                flourishes += 1;
                for quest_id in &flourish.after_any_completed_quest {
                    if data.quest(quest_id).is_none() {
                        dangling.push(format!("{} waits on quest {quest_id}", flourish.id));
                    }
                }
                for milestone_id in &flourish.after_any_journal_milestone {
                    if !recordable.contains(milestone_id) {
                        dangling.push(format!("{} waits on beat {milestone_id}", flourish.id));
                    }
                }
                if flourish.shapes.is_empty() {
                    dangling.push(format!("{} draws nothing at all", flourish.id));
                }
            }
        }

        assert!(flourishes > 0, "the world never changes for anything");
        assert!(
            dangling.is_empty(),
            "flourishes waiting on things that never happen:
{dangling:#?}"
        );
    }

    /// The point of moving these into data was coverage. Twelve story chains
    /// finish in this game and the world used to acknowledge four of them,
    /// across two areas, because each one was a `match` arm somebody had to
    /// write. This is a floor, not a target.
    #[test]
    fn the_world_changes_in_more_than_a_couple_of_places() {
        let data = load_embedded().expect("embedded game data should load");
        let areas = data
            .areas
            .iter()
            .filter(|area| !area.flourishes.is_empty())
            .count();
        let total = data
            .areas
            .iter()
            .map(|area| area.flourishes.len())
            .sum::<usize>();

        assert!(
            areas >= 3 && total >= 6,
            "only {total} flourishes across {areas} areas; the world barely notices what you do"
        );
    }
}
