use super::GameplayState;
use crate::data::{GameData, RecipeDefinition};

#[path = "gameplay_disassembly_text.rs"]
mod disassembly_text;

impl GameplayState {
    pub(super) fn available_disassembly_recipes<'a>(
        &self,
        data: &'a GameData,
    ) -> Vec<&'a RecipeDefinition> {
        let mut recipes = data
            .recipes
            .iter()
            .filter(|recipe| self.progression.known_recipes.contains(&recipe.id))
            .filter(|recipe| {
                self.inventory
                    .get(&recipe.output_item_id)
                    .copied()
                    .unwrap_or_default()
                    > 0
            })
            .collect::<Vec<_>>();
        recipes.sort_by(|left, right| left.name.cmp(&right.name));
        recipes
    }

    pub(super) fn disassemble_recipe(&mut self, data: &GameData, recipe: &RecipeDefinition) {
        let Some(output_amount) = self.inventory.get_mut(&recipe.output_item_id) else {
            self.runtime.status_text = disassembly_text::cannot_disassemble(&recipe.name);
            return;
        };
        if *output_amount == 0 {
            self.runtime.status_text = disassembly_text::cannot_disassemble(&recipe.name);
            return;
        }

        *output_amount -= 1;
        if *output_amount == 0 {
            self.inventory.remove(&recipe.output_item_id);
        }

        let mut returned = Vec::new();
        for ingredient in &recipe.ingredients {
            *self
                .inventory
                .entry(ingredient.item_id.clone())
                .or_insert(0) += ingredient.amount;
            self.note_inventory_observation(data, &ingredient.item_id);
            returned.push(disassembly_text::returned_input(
                data,
                &ingredient.item_id,
                ingredient.amount,
            ));
        }

        self.trigger_disassembly_feedback(disassembly_text::toast(&recipe.name));
        self.runtime.status_text = disassembly_text::disassembled(&recipe.name, &returned);
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::GameData;

    #[test]
    fn disassembly_returns_recipe_inputs() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        let recipe = data
            .recipe_for_output("healing_draught")
            .expect("healing draught recipe should exist");

        state.progression.known_recipes.insert(recipe.id.clone());
        state.inventory.insert("healing_draught".to_owned(), 1);

        state.disassemble_recipe(&data, recipe);

        assert_eq!(
            state
                .inventory
                .get("healing_draught")
                .copied()
                .unwrap_or_default(),
            0
        );
        assert_eq!(
            state.inventory.get("sunleaf").copied().unwrap_or_default(),
            1
        );
        assert_eq!(
            state
                .inventory
                .get("whisper_moss")
                .copied()
                .unwrap_or_default(),
            1
        );
    }
}
