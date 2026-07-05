use super::GameplayState;
use crate::alchemy::resolve_brew;
use crate::content::ui_format;
use crate::data::GameData;
use crate::view_models::alchemy::AlchemyPreviewPanelView;

impl GameplayState {
    pub(super) fn draw_alchemy_preview_panel(
        &self,
        data: &GameData,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) {
        let selected = self.selected_items();
        let view = if selected.is_empty() {
            AlchemyPreviewPanelView::empty_selection()
        } else if let Some(station) = self.nearby_station(data) {
            let preview = resolve_brew(
                data,
                station,
                &selected,
                self.selected_catalyst(),
                self.alchemy.heat,
                self.alchemy.stirs,
                self.alchemy_timing(),
                self.preview_mastery_brews(data, station, &selected),
            );
            let known = preview
                .recipe
                .map(|recipe| self.recipe_is_known(&recipe.id))
                .unwrap_or(false);
            let preview_uncertain = known && self.preview_is_uncertain(&preview);
            let stable_preview = self.brew_is_stable(&preview);
            let quest_line = self.brew_quest_motivation(data, &preview.output_item_id);

            AlchemyPreviewPanelView::resolved(
                data,
                &preview,
                known,
                stable_preview,
                preview_uncertain,
                quest_line,
            )
        } else {
            AlchemyPreviewPanelView::no_station()
        };

        crate::ui::draw_alchemy_preview_panel_view(&view, x, y, w, h);
    }

    /// If the projected brew output would satisfy an open errand, name the
    /// townsperson waiting on it so the bench work stays tied to the story.
    fn brew_quest_motivation(&self, data: &GameData, output_item_id: &str) -> Option<String> {
        let quest = data.quests.iter().find(|quest| {
            quest.required_item_id == output_item_id
                && quest.giver_npc_id != "quest_board"
                && !self.progression.completed_quests.contains(&quest.id)
                && (self.progression.started_quests.contains(&quest.id)
                    || self.quest_is_available(quest))
        })?;
        let npc_name = data
            .npc(&quest.giver_npc_id)
            .map(|npc| npc.name.as_str())
            .unwrap_or_default();
        Some(ui_format(
            "overlay_alchemy_brew_for",
            &[("npc", npc_name), ("quest", &quest.title)],
        ))
    }
}
