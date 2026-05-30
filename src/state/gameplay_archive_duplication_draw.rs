use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_archive_duplication_section(
        &self,
        data: &GameData,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) {
        let view = self.archive_duplication_section_view(data);
        crate::ui::draw_archive_duplication_section_view(&view, x, y, w, h);
    }
}
