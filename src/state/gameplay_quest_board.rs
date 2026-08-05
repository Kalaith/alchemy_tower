use super::GameplayState;
use crate::data::GameData;
use crate::input::{cancel_pressed, confirm_pressed, select_next_pressed, select_previous_pressed};

#[path = "gameplay_quest_board_text.rs"]
mod quest_board_text;

/// One selectable line on the board: either a request to accept or a finished
/// request ready to hand in.
pub(super) struct BoardAction {
    pub(super) quest_id: String,
    pub(super) deliver: bool,
}

impl GameplayState {
    pub(super) fn handle_quest_board_inputs(&mut self, data: &GameData) {
        if cancel_pressed() {
            self.clear_overlay();
            self.runtime.status_text = quest_board_text::closed();
            return;
        }
        let actions = self.board_actions(data);
        if actions.is_empty() {
            return;
        }
        if select_previous_pressed() {
            self.ui.shop_index = self.ui.shop_index.saturating_sub(1);
        }
        if select_next_pressed() {
            self.ui.shop_index = (self.ui.shop_index + 1).min(actions.len().saturating_sub(1));
        }
        if confirm_pressed() {
            if let Some(action) = actions.get(self.ui.shop_index) {
                if action.deliver {
                    self.deliver_board_quest(data, &action.quest_id);
                } else {
                    self.accept_board_quest(data, &action.quest_id);
                }
            }
        }
    }

    fn accept_board_quest(&mut self, data: &GameData, quest_id: &str) {
        self.progression.started_quests.insert(quest_id.to_owned());
        if let Some(quest) = data.quest(quest_id) {
            self.trigger_quest_accepted_feedback(quest_board_text::accepted_toast(quest));
        }
        self.runtime.status_text = self.quest_board_accept_status(data, quest_id);
    }

    /// Hand in a finished board request at the board itself. Repeatable
    /// requests return to the board after a cooldown; one-shot requests are
    /// marked complete like any other quest.
    fn deliver_board_quest(&mut self, data: &GameData, quest_id: &str) {
        let Some(quest) = data.quest(quest_id) else {
            return;
        };
        if !self.quest_requirements_met(data, quest) {
            return;
        }
        let delivered_rank = self.spend_bottles_for_quest(data, quest, quest.required_amount);
        self.take_from_inventory(&quest.required_item_id, quest.required_amount);
        self.progression.started_quests.remove(quest_id);
        // Beating the grade the order asked for is paid for, so there is a
        // reason to send good work to the board rather than the worst thing
        // that clears the bar.
        let bonus = self.quality_bonus_coins(quest, delivered_rank);
        self.coins = self
            .coins
            .saturating_sub(quest.coin_cost)
            .saturating_add(quest.reward_coins + bonus);
        // A board order is still somebody's problem solved. Without this the
        // repeatable layer — the bulk of the long tail — earned no standing
        // with anyone, however many times it was run.
        // Beating the bar counts with the person the order serves, on the board
        // as much as face to face — the arc path has awarded this since the
        // quality pass and the board path quietly did not, so the same bottle
        // was worth more standing depending on which counter it crossed.
        let exceptional = self.delivery_was_exceptional(quest, delivered_rank);
        if !quest.rapport_npc_id.is_empty() {
            *self
                .progression
                .relationships
                .entry(quest.rapport_npc_id.clone())
                .or_insert(0) += 1 + i32::from(exceptional);
            if exceptional {
                let npc_id = quest.rapport_npc_id.clone();
                self.remark_on_exceptional_delivery(data, &npc_id);
            }
        }
        self.push_quest_completion_milestones(quest);
        if quest.repeatable {
            let cooldown = quest.repeat_cooldown_days.max(1);
            self.progression
                .board_quest_cooldowns
                .insert(quest_id.to_owned(), self.world.day_index + cooldown);
        } else {
            self.progression
                .completed_quests
                .insert(quest_id.to_owned());
        }
        self.refresh_available_nodes(data);
        self.trigger_quest_complete_feedback(quest_board_text::delivered_toast(quest));
        self.runtime.status_text = quest_board_text::delivered_status(quest);
    }

    fn quest_board_accept_status(&self, data: &GameData, quest_id: &str) -> String {
        data.quest(quest_id)
            .map(|quest| {
                quest_board_text::accepted_status(quest, &self.quest_location_hint(data, quest))
            })
            .unwrap_or_else(quest_board_text::accepted_default)
    }

    /// Ordered selectable board lines: ready hand-ins first, then requests that
    /// can be accepted.
    pub(super) fn board_actions(&self, data: &GameData) -> Vec<BoardAction> {
        let mut actions: Vec<BoardAction> = data
            .quests
            .iter()
            .filter(|quest| quest.giver_npc_id == "quest_board")
            .filter(|quest| self.progression.started_quests.contains(&quest.id))
            .filter(|quest| self.quest_requirements_met(data, quest))
            .map(|quest| BoardAction {
                quest_id: quest.id.clone(),
                deliver: true,
            })
            .collect();
        actions.extend(
            self.available_board_quests(data)
                .into_iter()
                .map(|quest_id| BoardAction {
                    quest_id,
                    deliver: false,
                }),
        );
        actions
    }

    pub(super) fn available_board_quests(&self, data: &GameData) -> Vec<String> {
        data.quests
            .iter()
            .filter(|quest| quest.giver_npc_id == "quest_board")
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .filter(|quest| self.board_quest_off_cooldown(quest))
            .filter(|quest| self.quest_is_available(quest))
            .map(|quest| quest.id.clone())
            .collect()
    }

    /// A repeatable request stays off the board until its cooldown day arrives.
    fn board_quest_off_cooldown(&self, quest: &crate::data::QuestDefinition) -> bool {
        self.progression
            .board_quest_cooldowns
            .get(&quest.id)
            .is_none_or(|available_day| self.world.day_index >= *available_day)
    }

    pub(super) fn locked_board_quest_summaries(&self, data: &GameData) -> Vec<String> {
        data.quests
            .iter()
            .filter(|quest| quest.giver_npc_id == "quest_board")
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .filter(|quest| !self.quest_is_available(quest))
            .map(|quest| {
                let requirements = self.locked_state_text(&self.quest_unlock_summary(data, quest));
                quest_board_text::locked_line(quest, &requirements)
            })
            .collect()
    }

    pub(super) fn active_board_quest_titles(&self, data: &GameData) -> Vec<String> {
        // Started requests still being worked on. A board request that is ready
        // to hand in is omitted here because it shows up as a selectable
        // deliver entry instead.
        self.progression
            .started_quests
            .iter()
            .filter_map(|quest_id| data.quest(quest_id))
            .filter(|quest| {
                quest.giver_npc_id != "quest_board" || !self.quest_requirements_met(data, quest)
            })
            .map(|quest| quest.title.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::{BottleBatchEntry, CraftedItemProfileEntry};

    /// One Fine, restorative healing draught actually on the shelf. It has to be
    /// a real bottle now, not merely a best-ever record: a request is checked
    /// against what is being handed over.
    fn stock_healing_draught(state: &mut GameplayState) {
        state.inventory.insert("healing_draught".to_owned(), 1);
        state.progression.bottle_stock.insert(
            "healing_draught".to_owned(),
            vec![BottleBatchEntry {
                item_id: "healing_draught".to_owned(),
                quality_score: 60,
                quality_band: "Fine".to_owned(),
                traits: vec!["restorative".to_owned()],
                count: 1,
            }],
        );
        state.progression.crafted_item_profiles.insert(
            "healing_draught".to_owned(),
            CraftedItemProfileEntry {
                item_id: "healing_draught".to_owned(),
                best_quality_score: 60,
                best_quality_band: "Fine".to_owned(),
                inherited_traits: vec!["restorative".to_owned()],
                effect_kinds: vec!["restore".to_owned()],
            },
        );
    }

    #[test]
    fn repeatable_board_request_returns_after_cooldown() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let quest_id = "board_restorative_stash";
        let quest = data.quest(quest_id).expect("board quest should exist");
        assert!(quest.repeatable);

        state.progression.total_brews = 10;
        state.progression.started_quests.insert(quest_id.to_owned());
        stock_healing_draught(&mut state);

        // A ready request shows up as a deliverable board action.
        assert!(state
            .board_actions(&data)
            .iter()
            .any(|action| action.quest_id == quest_id && action.deliver));

        let coins_before = state.coins;
        state.deliver_board_quest(&data, quest_id);

        // Delivered: paid out, consumed, and NOT permanently completed. The
        // fixture bottle is Fine against a Serviceable order, so the payment
        // includes the bonus for beating the bar by a band.
        let bonus = state.quality_bonus_coins(
            quest,
            crate::state::gameplay::gameplay_support::quality_band_rank("Fine"),
        );
        assert!(bonus > 0, "a Fine bottle beats a Serviceable order");
        assert_eq!(state.coins, coins_before + quest.reward_coins + bonus);
        assert!(!state.progression.completed_quests.contains(quest_id));
        assert!(!state.progression.started_quests.contains(quest_id));
        assert_eq!(
            state.inventory.get(quest_id).copied().unwrap_or_default(),
            0
        );

        // On cooldown today, back on the board once the cooldown day arrives.
        assert!(!state
            .available_board_quests(&data)
            .contains(&quest_id.to_owned()));
        state.world.day_index += quest.repeat_cooldown_days.max(1);
        assert!(state
            .available_board_quests(&data)
            .contains(&quest_id.to_owned()));
    }

    #[test]
    fn non_repeatable_delivery_completes_permanently() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        // star_elixir_for_ione is a one-shot NPC quest; deliver it through the
        // shared board delivery path to confirm the non-repeatable branch.
        let quest_id = "healing_for_mira";
        state.progression.started_quests.insert(quest_id.to_owned());
        stock_healing_draught(&mut state);

        state.deliver_board_quest(&data, quest_id);
        assert!(state.progression.completed_quests.contains(quest_id));
    }

    /// The repeatable layer used to be decoupled from the townsfolk entirely:
    /// thirty-odd orders, none of which earned standing with anybody, however
    /// many times they were run. Every order now names whose work it serves.
    #[test]
    fn a_board_delivery_earns_standing_with_whoever_it_was_for() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let quest = data
            .quest("board_restorative_stash")
            .expect("board quest should exist");
        assert_eq!(quest.rapport_npc_id, "wren_physician");

        state.progression.total_brews = 10;
        state.progression.started_quests.insert(quest.id.clone());
        stock_healing_draught(&mut state);

        let before = state.rapport_value(&quest.rapport_npc_id);
        state.deliver_board_quest(&data, &quest.id);
        assert_eq!(
            state.rapport_value(&quest.rapport_npc_id),
            before + 1,
            "the infirmary learned nothing from being supplied"
        );
    }

    /// A request can now wait on standing rather than on progress. Without the
    /// gate being read, a confidant-only order would sit on the board from the
    /// first day and the upper rapport tiers would go back to being labels.
    #[test]
    fn a_confidant_order_waits_until_the_standing_is_there() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let quest = data
            .quest("board_confidant_coldread_for_ione")
            .expect("the confidant order should exist");
        assert_eq!(
            quest.required_rapport,
            data.config.balance.rapport.confidant
        );

        // Everything but the standing.
        state
            .progression
            .unlocked_warps
            .insert(quest.required_unlocked_warp.clone());
        assert!(!state.quest_is_available(quest));

        state.progression.relationships.insert(
            quest.required_rapport_npc_id.clone(),
            quest.required_rapport,
        );
        assert!(state.quest_is_available(quest));
    }
}
