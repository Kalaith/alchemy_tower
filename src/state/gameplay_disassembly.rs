use super::GameplayState;
use crate::content::ui_format;
use crate::data::{GameData, RecipeDefinition};
use macroquad::prelude::Color;

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
            self.runtime.status_text =
                ui_format("progression_no_disassemble", &[("name", &recipe.name)]);
            return;
        };
        if *output_amount == 0 {
            self.runtime.status_text =
                ui_format("progression_no_disassemble", &[("name", &recipe.name)]);
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
            returned.push(format!(
                "{} x{}",
                data.item_name(&ingredient.item_id),
                ingredient.amount
            ));
        }

        self.push_event_toast_with_icon(
            ui_format("progression_disassembly_toast", &[("name", &recipe.name)]),
            Color::from_rgba(214, 204, 170, 255),
            "recipe_logged",
        );
        self.runtime.status_text = ui_format(
            "progression_disassembled",
            &[("name", &recipe.name), ("items", &returned.join(", "))],
        );
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
