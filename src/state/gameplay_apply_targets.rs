//! Pouring a brew on something rather than drinking it or handing it over.
//!
//! The game is about applied alchemy, and for a long time the only thing a
//! finished bottle could do to the world was change a townsperson's opinion of
//! the player. Wilted ground, frightened creatures and blocked paths were
//! collision boxes and art — scenery a brew could not touch.
//!
//! A target is a thing that can be treated. It asks for a kind of effect and,
//! optionally, a grade; treating it spends a qualifying bottle and records
//! journal milestones. Those milestones are deliberately the same currency
//! every other gate already reads, so a warp, a station or a patch of ground
//! can wait on something having been *treated* without a line of new gating
//! code.

use super::gameplay_support::quality_band_rank;
use super::GameplayState;
use crate::content::ui_format;
use crate::data::{ApplyTargetDefinition, AreaDefinition, GameData, ItemCategory};

impl GameplayState {
    pub(super) fn target_is_treated(&self, target: &ApplyTargetDefinition) -> bool {
        self.progression.treated_targets.contains(&target.id)
    }

    /// The bottle on the shelf that would do this job, worst acceptable first —
    /// the same courtesy delivery pays, so treating a wilted bed does not cost
    /// the player their best work.
    pub(super) fn bottle_for_target(
        &self,
        data: &GameData,
        target: &ApplyTargetDefinition,
    ) -> Option<String> {
        let wanted_rank = if target.minimum_quality_band.is_empty() {
            0
        } else {
            quality_band_rank(&target.minimum_quality_band)
        };

        let mut best: Option<(u8, String)> = None;
        for (item_id, held) in &self.inventory {
            if *held == 0 {
                continue;
            }
            let Some(item) = data.item(item_id) else {
                continue;
            };
            if item.category != ItemCategory::Potion {
                continue;
            }
            if !item
                .effects
                .iter()
                .any(|effect| effect.kind.to_string() == target.required_effect_kind)
            {
                continue;
            }
            let rank = self.worst_held_band_rank(data, item_id);
            if rank < wanted_rank {
                continue;
            }
            if best.as_ref().is_none_or(|(current, _)| rank < *current) {
                best = Some((rank, item_id.clone()));
            }
        }
        best.map(|(_, item_id)| item_id)
    }

    /// Treat the target with a qualifying bottle. Returns false when there is
    /// nothing on the shelf that would do, so the caller can say so.
    pub(super) fn treat_target(&mut self, data: &GameData, target: &ApplyTargetDefinition) -> bool {
        let Some(item_id) = self.bottle_for_target(data, target) else {
            return false;
        };
        self.take_from_inventory(&item_id, 1);
        self.progression.treated_targets.insert(target.id.clone());
        for milestone in &target.completion_milestones {
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        }
        self.refresh_available_nodes(data);
        self.trigger_quest_complete_feedback(ui_format(
            "target_treated_toast",
            &[("name", &target.name)],
        ));
        self.runtime.status_text = target.treated_note.clone();
        true
    }

    /// The untreated target the player is standing next to, if any. A treated
    /// one is scenery again — the work is done.
    pub(super) fn interaction_apply_target<'a>(
        &self,
        area: &'a AreaDefinition,
    ) -> Option<&'a ApplyTargetDefinition> {
        let player = self.world.player.position;
        area.apply_targets
            .iter()
            .filter(|target| !self.target_is_treated(target))
            .find(|target| {
                let dx = target.position[0] - player.x;
                let dy = target.position[1] - player.y;
                (dx * dx + dy * dy).sqrt() <= target.radius
            })
    }

    pub(super) fn handle_apply_target_interaction(
        &mut self,
        data: &GameData,
        target: &ApplyTargetDefinition,
    ) {
        let target = target.clone();
        if !self.treat_target(data, &target) {
            self.runtime.status_text = self.target_requirement_text(&target);
        }
    }

    /// What the player is told when they have nothing that would work.
    pub(super) fn target_requirement_text(&self, target: &ApplyTargetDefinition) -> String {
        if target.minimum_quality_band.is_empty() {
            ui_format(
                "target_needs_effect",
                &[("effect", &target.required_effect_kind)],
            )
        } else {
            ui_format(
                "target_needs_graded_effect",
                &[
                    ("effect", &target.required_effect_kind),
                    ("band", &target.minimum_quality_band),
                ],
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::{ApplyTargetDefinition, GameData};

    fn a_target(data: &GameData) -> ApplyTargetDefinition {
        data.areas
            .iter()
            .flat_map(|area| area.apply_targets.iter())
            .next()
            .cloned()
            .unwrap_or_else(|| panic!("the world should carry something a brew can be used on"))
    }

    /// The premise, end to end: hold nothing and the target stays as it is;
    /// hold the right brew and it is treated, the bottle is gone, and the beat
    /// is recorded for whatever is waiting on it.
    #[test]
    fn a_brew_poured_on_the_world_changes_it() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let target = a_target(&data);

        assert!(!state.target_is_treated(&target));
        assert!(
            !state.treat_target(&data, &target),
            "an empty bag should treat nothing"
        );

        // Find something that does the job and stock one.
        let bottle = data
            .items
            .iter()
            .find(|item| {
                item.category == crate::data::ItemCategory::Potion
                    && item
                        .effects
                        .iter()
                        .any(|effect| effect.kind.to_string() == target.required_effect_kind)
            })
            .expect("some potion should do what the target asks");
        state.inventory.insert(bottle.id.clone(), 1);

        assert!(state.treat_target(&data, &target));
        assert!(state.target_is_treated(&target));
        assert_eq!(
            state.inventory.get(&bottle.id).copied().unwrap_or_default(),
            0,
            "treating should have spent the bottle"
        );
        for milestone in &target.completion_milestones {
            assert!(state.has_journal_milestone(&milestone.id));
        }
    }

    /// A target that names a grade will not take a worse bottle, or the
    /// quality system stops at the bench door.
    #[test]
    fn a_graded_target_refuses_a_worse_bottle() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let mut target = a_target(&data);
        target.minimum_quality_band = "Masterwork".to_owned();

        let bottle = data
            .items
            .iter()
            .find(|item| {
                item.category == crate::data::ItemCategory::Potion
                    && item
                        .effects
                        .iter()
                        .any(|effect| effect.kind.to_string() == target.required_effect_kind)
            })
            .expect("some potion should do what the target asks");
        state.inventory.insert(bottle.id.clone(), 1);

        assert!(
            state.bottle_for_target(&data, &target).is_none(),
            "a plain bottle should not satisfy a Masterwork demand"
        );
    }

    #[test]
    fn a_treatment_opens_its_same_area_ground_immediately() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let area = data
            .area("moonlit_forest")
            .expect("the forest should exist");
        let target = area
            .apply_targets
            .iter()
            .find(|target| target.id == "forest_startled_roost")
            .expect("the startled roost should exist")
            .clone();
        let milestone = "forest_roost_settled";
        let node_id = "forest_settled_roost_01";
        state.world.current_area_id = area.id.clone();
        state.set_clock_minutes(1320.0);

        // Find a day on which season, weather and daily roll all allow the node;
        // then remove the temporary gate so the treatment is the only change.
        state.push_journal_milestone(milestone, "", "");
        let day = (0..40)
            .find(|day| {
                state.world.day_index = *day;
                state.refresh_available_nodes(&data);
                state.world.available_nodes.contains(node_id)
            })
            .expect("the settled roost should have a valid spawn day");
        state
            .progression
            .journal_milestones
            .retain(|entry| entry.id != milestone);
        state.world.day_index = day;
        state.refresh_available_nodes(&data);
        assert!(!state.world.available_nodes.contains(node_id));

        let bottle = data
            .items
            .iter()
            .find(|item| {
                item.category == crate::data::ItemCategory::Potion
                    && item
                        .effects
                        .iter()
                        .any(|effect| effect.kind.to_string() == target.required_effect_kind)
            })
            .expect("a misfire bottle should settle the roost");
        state.inventory.insert(bottle.id.clone(), 1);

        assert!(state.treat_target(&data, &target));
        assert!(
            state.world.available_nodes.contains(node_id),
            "the player had to leave and re-enter before restored ground appeared"
        );
    }
}
