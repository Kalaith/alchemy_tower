use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_journal_brews_tab(
        &self,
        data: &GameData,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) {
        let view = self.journal_brews_tab_view(data);
        crate::ui::draw_journal_brews_tab_view(&view, x, y, w, h);
    }
}
