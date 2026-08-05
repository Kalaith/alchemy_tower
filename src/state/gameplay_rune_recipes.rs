use super::GameplayState;
use crate::content::narrative_text;
use crate::data::{GameData, RuneRecipeDefinition, StationDefinition};

#[path = "gameplay_rune_recipe_text.rs"]
mod rune_recipe_text;

impl GameplayState {
    pub(super) fn available_rune_recipes<'a>(
        &self,
        data: &'a GameData,
        station: &StationDefinition,
    ) -> Vec<&'a RuneRecipeDefinition> {
        data.rune_recipes
            .iter()
            .filter(|recipe| recipe.station_id == station.id)
            .filter(|recipe| self.has_rune_recipe_inputs(recipe))
            .collect()
    }

    pub(super) fn apply_rune_recipe(&mut self, data: &GameData, recipe: &RuneRecipeDefinition) {
        if !self.transform_worst_held_bottle(data, &recipe.input_item_id, &recipe.output_item_id) {
            return;
        }
        self.take_from_inventory(&recipe.rune_item_id, 1);
        self.note_inventory_observation(data, &recipe.output_item_id);
        self.ensure_potion_memory_learned(&recipe.output_item_id, None);
        let milestone = &narrative_text().milestones.first_rune_imbuing;
        self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        self.runtime.status_text = rune_recipe_text::imbued(data, recipe);
    }

    fn has_rune_recipe_inputs(&self, recipe: &RuneRecipeDefinition) -> bool {
        self.inventory
            .get(&recipe.input_item_id)
            .copied()
            .unwrap_or_default()
            > 0
            && self
                .inventory
                .get(&recipe.rune_item_id)
                .copied()
                .unwrap_or_default()
                > 0
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::{BottleBatchEntry, GameData};

    fn prove_graded_story_delivery(
        data: &GameData,
        recipe_id: &str,
        quest_id: &str,
        score: u32,
        band: &str,
    ) {
        let recipe = data
            .rune_recipes
            .iter()
            .find(|recipe| recipe.id == recipe_id)
            .expect("story rune recipe should exist")
            .clone();
        let quest = data
            .quests
            .iter()
            .find(|quest| quest.id == quest_id)
            .expect("story delivery should exist");
        let mut state = GameplayState::new(data);
        state
            .inventory
            .insert(recipe.input_item_id.clone(), quest.required_amount);
        state
            .inventory
            .insert(recipe.rune_item_id.clone(), quest.required_amount);
        state.progression.bottle_stock.insert(
            recipe.input_item_id.clone(),
            vec![BottleBatchEntry {
                item_id: recipe.input_item_id.clone(),
                quality_score: score,
                quality_band: band.to_owned(),
                traits: vec!["carefully_brewed".to_owned()],
                count: quest.required_amount,
            }],
        );

        for _ in 0..quest.required_amount {
            state.apply_rune_recipe(data, &recipe);
        }

        let output = state.live_batches(&recipe.output_item_id);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].quality_score, score);
        assert_eq!(output[0].quality_band, band);
        assert_eq!(output[0].count, quest.required_amount);
        assert!(output[0]
            .traits
            .iter()
            .any(|value| value == "carefully_brewed"));
        for authored in &data
            .item(&recipe.output_item_id)
            .expect("rune output should exist")
            .traits
        {
            assert!(
                output[0].traits.contains(authored),
                "rune output lost authored trait {authored}"
            );
        }
        assert!(
            state.quest_requirements_met(data, quest),
            "{quest_id} remained impossible after imbuing {band} inputs"
        );
    }

    #[test]
    fn graded_rune_outputs_reopen_mira_and_elrics_story_routes() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        prove_graded_story_delivery(
            &data,
            "splash_poultice_recipe",
            "wellside_relief_for_mira",
            50,
            "Fine",
        );
        prove_graded_story_delivery(
            &data,
            "delay_emberglass_recipe",
            "nightwatch_for_elric",
            70,
            "Excellent",
        );
    }
}
