use super::GameplayState;
use crate::alchemy::resolve_brew;
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

            AlchemyPreviewPanelView::resolved(
                data,
                &preview,
                known,
                stable_preview,
                preview_uncertain,
            )
        } else {
            AlchemyPreviewPanelView::no_station()
        };

        crate::ui::draw_alchemy_preview_panel_view(&view, x, y, w, h);
    }
}
