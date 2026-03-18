use super::*;
use crate::content::ui_format;

impl GameplayState {
    pub(super) fn handle_dialogue_inputs(&mut self, data: &GameData) {
        if !is_key_pressed(KeyCode::Enter) && !is_key_pressed(KeyCode::Space) {
            return;
        }

        let Some(npc_id) = self.ui.current_npc_id.clone() else {
            self.ui.dialogue_open = false;
            return;
        };
        let Some(npc) = data.npc(&npc_id) else {
            self.ui.dialogue_open = false;
            self.ui.current_npc_id = None;
            return;
        };

        if npc.quest_id.is_empty() {
            self.ui.dialogue_open = false;
            self.ui.current_npc_id = None;
            return;
        }

        let Some(quest) = data.quest(&npc.quest_id) else {
            self.ui.dialogue_open = false;
            self.ui.current_npc_id = None;
            return;
        };

        if self.progression.completed_quests.contains(&quest.id) {
            self.ui.dialogue_open = false;
            self.ui.current_npc_id = None;
            return;
        }

        if !self.progression.started_quests.contains(&quest.id) {
            if !self.quest_is_available(quest) {
                self.runtime.status_text = self.quest_unlock_summary(quest);
                return;
            }
            self.progression.started_quests.insert(quest.id.clone());
            *self.progression.relationships.entry(npc.id.clone()).or_insert(0) += 1;
            self.push_event_toast(
                ui_format("quests_accepted_toast", &[("title", &quest.title)]),
                Color::from_rgba(255, 230, 170, 255),
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(255, 230, 170, 255),
                false,
                1.2,
            );
            self.runtime.status_text = ui_format(
                "quests_accepted_status",
                &[("title", &quest.title), ("hint", &self.quest_location_hint(data, quest))],
            );
            return;
        }

        if self.quest_requirements_met(data, quest) {
            if let Some(amount) = self.inventory.get_mut(&quest.required_item_id) {
                *amount -= quest.required_amount;
            }
            self.inventory.retain(|_, amount| *amount > 0);
            self.progression.started_quests.remove(&quest.id);
            self.progression.completed_quests.insert(quest.id.clone());
            self.coins += quest.reward_coins;
            if quest.giver_npc_id != "quest_board" {
                *self
                    .progression
                    .relationships
                    .entry(quest.giver_npc_id.clone())
                    .or_insert(0) += 2;
            }
            if quest.id == "healing_for_mira" {
                let milestone = &narrative_text().milestones.first_town_relief;
                self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            } else if quest.id == "cultivation_for_brin" {
                let milestone = &narrative_text().milestones.greenhouse_expanded;
                self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            } else if quest.id == "containment_for_lyra" {
                let milestone = &narrative_text().milestones.containment_stable;
                self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            }
            self.push_event_toast(
                ui_format("quests_complete_toast", &[("title", &quest.title)]),
                Color::from_rgba(188, 255, 220, 255),
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(188, 255, 220, 255),
                true,
                1.8,
            );
            self.trigger_camera_shake(0.14, 3.8);
            self.runtime.status_text = ui_format(
                "quests_delivered_status",
                &[
                    ("item", data.item_name(&quest.required_item_id)),
                    ("coins", &quest.reward_coins.to_string()),
                ],
            );
        } else {
            self.ui.dialogue_open = false;
            self.ui.current_npc_id = None;
        }
    }

    pub(super) fn active_quest_title<'a>(&'a self, data: &'a GameData) -> Option<&'a str> {
        self.progression.started_quests
            .iter()
            .find_map(|quest_id| data.quest(quest_id).map(|quest| quest.title.as_str()))
    }

    pub(super) fn handle_quest_board_inputs(&mut self, data: &GameData) {
        if is_key_pressed(KeyCode::Escape) {
            self.ui.quest_board_open = false;
            self.runtime.status_text = ui_text().statuses.closed_quest_board.clone();
            return;
        }
        let available = self.available_board_quests(data);
        if available.is_empty() {
            return;
        }
        if is_key_pressed(KeyCode::Up) {
            self.ui.shop_index = self.ui.shop_index.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) {
            self.ui.shop_index = (self.ui.shop_index + 1).min(available.len().saturating_sub(1));
        }
        if is_key_pressed(KeyCode::Enter) {
            if let Some(quest_id) = available.get(self.ui.shop_index) {
                self.progression.started_quests.insert(quest_id.clone());
                if let Some(quest) = data.quest(quest_id) {
                    self.push_event_toast(
                        ui_format("quests_accepted_toast", &[("title", &quest.title)]),
                        Color::from_rgba(255, 230, 170, 255),
                    );
                    self.trigger_world_feedback(
                        self.world.player.position,
                        Color::from_rgba(255, 230, 170, 255),
                        false,
                        1.2,
                    );
                }
                self.runtime.status_text = data
                    .quest(quest_id)
                    .map(|quest| {
                        ui_format(
                            "quests_board_accepted_status",
                            &[("title", &quest.title), ("hint", &self.quest_location_hint(data, quest))],
                        )
                    })
                    .unwrap_or_else(|| ui_format("quests_board_accepted_default", &[]));
            }
        }
    }

    pub(super) fn available_board_quests(&self, data: &GameData) -> Vec<String> {
        data.quests
            .iter()
            .filter(|quest| quest.giver_npc_id == "quest_board")
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .filter(|quest| self.quest_is_available(quest))
            .map(|quest| quest.id.clone())
            .collect()
    }

    pub(super) fn current_dialogue_text(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let Some(quest) = (!npc.quest_id.is_empty())
            .then(|| data.quest(&npc.quest_id))
            .flatten()
        else {
            return self.append_npc_story_line(&npc.id, npc.dialogue_complete.clone());
        };

        if self.progression.completed_quests.contains(&quest.id) {
            return self.append_npc_story_line(&npc.id, npc.dialogue_complete.clone());
        }
        if !self.progression.started_quests.contains(&quest.id) {
            if !self.quest_is_available(quest) {
                return self.append_npc_story_line(
                    &npc.id,
                    format!(
                    "{} {}",
                    npc.dialogue_start,
                    self.quest_unlock_summary(quest)
                ),
                );
            }
            return self.append_npc_story_line(
                &npc.id,
                format!(
                "{} {}",
                npc.dialogue_start,
                self.npc_context_line(data, npc)
            ),
            );
        }

        if self.quest_requirements_met(data, quest) {
            self.append_npc_story_line(
                &npc.id,
                format!(
                "{}",
                ui_format(
                    "quests_dialogue_smell",
                    &[
                        ("progress", &npc.dialogue_progress),
                        ("context", &self.npc_context_line(data, npc)),
                        ("item", data.item_name(&quest.required_item_id)),
                    ],
                )
            ),
            )
        } else {
            self.append_npc_story_line(
                &npc.id,
                format!(
                "{}",
                ui_format(
                    "quests_dialogue_requirements",
                    &[
                        ("progress", &npc.dialogue_progress),
                        ("context", &self.npc_context_line(data, npc)),
                        ("requirements", &self.quest_requirement_summary(data, quest)),
                    ],
                )
            ),
            )
        }
    }

    pub(super) fn current_dialogue_footer(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let Some(quest) = (!npc.quest_id.is_empty())
            .then(|| data.quest(&npc.quest_id))
            .flatten()
        else {
            return ui_format("quests_dialogue_footer_default", &[]);
        };

        if self.progression.completed_quests.contains(&quest.id) {
            return ui_format("quests_dialogue_footer_default", &[]);
        }
        if !self.progression.started_quests.contains(&quest.id) {
            if !self.quest_is_available(quest) {
                return self.locked_state_text(&self.quest_unlock_summary(quest));
            }
            return ui_format("quests_dialogue_footer_reward", &[("coins", &quest.reward_coins.to_string())]);
        }

        if self.quest_requirements_met(data, quest) {
            ui_format(
                "quests_dialogue_footer_delivery",
                &[
                    ("item", data.item_name(&quest.required_item_id)),
                    ("amount", &quest.required_amount.to_string()),
                    ("coins", &quest.reward_coins.to_string()),
                ],
            )
        } else {
            ui_format(
                "quests_dialogue_footer_unavailable",
                &[
                    ("requirements", &self.unavailable_state_text(&self.quest_requirement_summary(data, quest))),
                    ("item", data.item_name(&quest.required_item_id)),
                    ("amount", &quest.required_amount.to_string()),
                ],
            )
        }
    }

    pub(super) fn quest_requirements_met(&self, data: &GameData, quest: &QuestDefinition) -> bool {
        let carried = self
            .inventory
            .get(&quest.required_item_id)
            .copied()
            .unwrap_or_default();
        if carried < quest.required_amount {
            return false;
        }

        let profile = self.progression.crafted_item_profiles.get(&quest.required_item_id);
        let quality_ok = quest.minimum_quality_band.is_empty()
            || profile
                .map(|profile| {
                    quality_band_rank(&profile.best_quality_band)
                        >= quality_band_rank(&quest.minimum_quality_band)
                })
                .unwrap_or(false);
        let trait_ok = profile
            .map(|profile| trait_requirement_met(quest, &profile.inherited_traits))
            .unwrap_or_else(|| trait_requirement_target(quest) == 0);
        let effect_ok = effect_requirement_met(
            data,
            quest,
            profile.map(|profile| profile.effect_kinds.as_slice()),
        );

        quality_ok && trait_ok && effect_ok
    }

    pub(super) fn quest_is_available(&self, quest: &QuestDefinition) -> bool {
        quest
            .prerequisite_quests
            .iter()
            .all(|quest_id| self.progression.completed_quests.contains(quest_id))
            && (quest.required_unlocked_warp.is_empty()
                || self.progression.unlocked_warps.contains(&quest.required_unlocked_warp))
            && self.progression.total_brews >= quest.minimum_total_brews
    }

    pub(super) fn quest_unlock_summary(&self, quest: &QuestDefinition) -> String {
        let mut reasons = Vec::new();
        let missing_prereqs = quest
            .prerequisite_quests
            .iter()
            .filter(|quest_id| !self.progression.completed_quests.contains(*quest_id))
            .cloned()
            .collect::<Vec<_>>();
        if !missing_prereqs.is_empty() {
            reasons.push(ui_format("quests_unlock_finish", &[("quests", &missing_prereqs.join(", "))]));
        }
        if !quest.required_unlocked_warp.is_empty()
            && !self.progression.unlocked_warps.contains(&quest.required_unlocked_warp)
        {
            reasons.push(ui_format("quests_unlock_greenhouse", &[]));
        }
        if self.progression.total_brews < quest.minimum_total_brews {
            reasons.push(ui_format("quests_unlock_brews", &[("brews", &quest.minimum_total_brews.to_string())]));
        }
        if reasons.is_empty() {
            ui_format("quests_unlock_closed", &[])
        } else {
            ui_format("quests_unlock_after", &[("reasons", &reasons.join(" and "))])
        }
    }

    pub(super) fn quest_requirement_summary(
        &self,
        data: &GameData,
        quest: &QuestDefinition,
    ) -> String {
        let carried = self
            .inventory
            .get(&quest.required_item_id)
            .copied()
            .unwrap_or_default();
        let mut requirements = Vec::new();
        if carried < quest.required_amount {
            requirements.push(ui_format("quests_requirement_carry", &[("carried", &carried.to_string()), ("required", &quest.required_amount.to_string())]));
        }
        if !quest.minimum_quality_band.is_empty() {
            let met = self
                .progression
                .crafted_item_profiles
                .get(&quest.required_item_id)
                .map(|profile| {
                    quality_band_rank(&profile.best_quality_band)
                        >= quality_band_rank(&quest.minimum_quality_band)
                })
                .unwrap_or(false);
            if !met {
                requirements.push(ui_format("quests_requirement_quality", &[("band", &quest.minimum_quality_band)]));
            }
        }
        if trait_requirement_target(quest) > 0 {
            let met = self
                .progression
                .crafted_item_profiles
                .get(&quest.required_item_id)
                .map(|profile| trait_requirement_met(quest, &profile.inherited_traits))
                .unwrap_or(false);
            if !met {
                requirements.push(trait_requirement_summary(quest));
            }
        }
        if effect_requirement_target(quest) > 0 {
            let met = effect_requirement_met(
                data,
                quest,
                self.progression.crafted_item_profiles
                    .get(&quest.required_item_id)
                    .map(|profile| profile.effect_kinds.as_slice()),
            );
            if !met {
                requirements.push(effect_requirement_summary(quest));
            }
        }

        if requirements.is_empty() {
            ui_format("quests_requirement_ready", &[])
        } else {
            requirements.join(", ")
        }
    }
}

fn quest_required_traits(quest: &QuestDefinition) -> Vec<&str> {
    let mut traits = quest
        .required_traits
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if !quest.required_trait.is_empty() && !traits.contains(&quest.required_trait.as_str()) {
        traits.push(&quest.required_trait);
    }
    traits
}

fn quest_required_effect_kinds(quest: &QuestDefinition) -> Vec<&str> {
    let mut effects = quest
        .required_effect_kinds
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if !quest.required_effect_kind.is_empty()
        && !effects.contains(&quest.required_effect_kind.as_str())
    {
        effects.push(&quest.required_effect_kind);
    }
    effects
}

fn requirement_target(requirement_count: usize, configured_minimum: u32) -> usize {
    if requirement_count == 0 {
        return 0;
    }
    if configured_minimum == 0 {
        return requirement_count;
    }
    (configured_minimum as usize).min(requirement_count)
}

fn matching_requirement_count(required: &[&str], actual: &[String]) -> usize {
    required
        .iter()
        .filter(|required_value| actual.iter().any(|actual_value| actual_value == *required_value))
        .count()
}

fn trait_requirement_target(quest: &QuestDefinition) -> usize {
    requirement_target(
        quest_required_traits(quest).len(),
        quest.minimum_trait_matches,
    )
}

fn effect_requirement_target(quest: &QuestDefinition) -> usize {
    requirement_target(
        quest_required_effect_kinds(quest).len(),
        quest.minimum_effect_matches,
    )
}

fn trait_requirement_met(quest: &QuestDefinition, actual_traits: &[String]) -> bool {
    let required_traits = quest_required_traits(quest);
    let target = trait_requirement_target(quest);
    matching_requirement_count(&required_traits, actual_traits) >= target
}

fn effect_requirement_met(
    data: &GameData,
    quest: &QuestDefinition,
    effect_kinds: Option<&[String]>,
) -> bool {
    let required_effects = quest_required_effect_kinds(quest);
    let target = effect_requirement_target(quest);
    if target == 0 {
        return true;
    }

    let owned_effects = effect_kinds
        .map(|effects| effects.to_vec())
        .or_else(|| {
            data.item(&quest.required_item_id).map(|item| {
                item.effects
                    .iter()
                    .map(|effect| effect.kind.to_string())
                    .collect::<Vec<_>>()
            })
        })
        .unwrap_or_default();

    matching_requirement_count(&required_effects, &owned_effects) >= target
}

fn trait_requirement_summary(quest: &QuestDefinition) -> String {
    let required_traits = quest_required_traits(quest);
    let target = trait_requirement_target(quest);
    if target == 0 {
        return ui_format("quests_trait_ready", &[]);
    }
    if target >= required_traits.len() {
        return ui_format("quests_trait_all", &[("traits", &required_traits.join(" + "))]);
    }
    ui_format(
        "quests_trait_partial",
        &[("target", &target.to_string()), ("count", &required_traits.len().to_string()), ("traits", &required_traits.join(", "))],
    )
}

fn effect_requirement_summary(quest: &QuestDefinition) -> String {
    let required_effects = quest_required_effect_kinds(quest);
    let target = effect_requirement_target(quest);
    if target == 0 {
        return ui_format("quests_effect_ready", &[]);
    }
    if target >= required_effects.len() {
        return ui_format("quests_effect_all", &[("effects", &required_effects.join(" + "))]);
    }
    ui_format(
        "quests_effect_partial",
        &[("target", &target.to_string()), ("count", &required_effects.len().to_string()), ("effects", &required_effects.join(", "))],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_quest() -> QuestDefinition {
        QuestDefinition {
            id: "quest".to_owned(),
            title: "Quest".to_owned(),
            description: String::new(),
            required_item_id: "item".to_owned(),
            required_amount: 1,
            reward_coins: 1,
            giver_npc_id: "npc".to_owned(),
            minimum_quality_band: String::new(),
            required_trait: String::new(),
            required_traits: Vec::new(),
            minimum_trait_matches: 0,
            required_effect_kind: String::new(),
            required_effect_kinds: Vec::new(),
            minimum_effect_matches: 0,
            prerequisite_quests: Vec::new(),
            required_unlocked_warp: String::new(),
            minimum_total_brews: 0,
        }
    }

    #[test]
    fn legacy_single_trait_requirement_still_works() {
        let mut quest = test_quest();
        quest.required_trait = "restorative".to_owned();

        assert!(trait_requirement_met(&quest, &["restorative".to_owned()]));
        assert!(!trait_requirement_met(&quest, &["luminous".to_owned()]));
    }

    #[test]
    fn multi_trait_requirement_supports_thresholds() {
        let mut quest = test_quest();
        quest.required_traits = vec![
            "restorative".to_owned(),
            "calm".to_owned(),
            "luminous".to_owned(),
        ];
        quest.minimum_trait_matches = 2;

        assert!(trait_requirement_met(
            &quest,
            &["restorative".to_owned(), "calm".to_owned()]
        ));
        assert!(!trait_requirement_met(&quest, &["restorative".to_owned()]));
    }

    #[test]
    fn multi_effect_requirement_summary_shows_band_threshold() {
        let mut quest = test_quest();
        quest.required_effect_kinds = vec!["glow".to_owned(), "restore".to_owned()];
        quest.minimum_effect_matches = 1;

        assert_eq!(effect_requirement_summary(&quest), "effects 1/2 glow, restore");
    }
}


