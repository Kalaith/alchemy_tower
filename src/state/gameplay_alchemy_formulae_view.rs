use super::GameplayState;
use crate::content::ui_copy;
use crate::data::GameData;
use crate::view_models::alchemy::{AlchemyFormulaRowView, AlchemyFormulaePanelView};

impl GameplayState {
    pub(super) fn alchemy_formulae_panel_view(&self, data: &GameData) -> AlchemyFormulaePanelView {
        AlchemyFormulaePanelView {
            empty_text: ui_copy("overlay_alchemy_no_formulae").to_owned(),
            rows: self
                .known_recipes(data)
                .into_iter()
                .map(|recipe| AlchemyFormulaRowView {
                    title: recipe.name.clone(),
                    meta: self.recipe_memory_meta(data, recipe),
                    detail: self.recipe_memory_detail(data, recipe),
                    lore_note: recipe.lore_note.clone(),
                })
                .collect(),
        }
    }
}
