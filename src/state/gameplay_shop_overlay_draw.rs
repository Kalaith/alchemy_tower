use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_shop_overlay(&self, data: &GameData) {
        if let Some(view) = self.shop_overlay_view(data) {
            crate::ui::draw_shop_overlay_view(&view);
        }
    }
}
