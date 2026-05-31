use super::hud_chrome::*;
use super::hud_primitives::*;
use super::HudView;
use macroquad::prelude::*;

pub(super) fn draw_control_tags(view: &HudView) {
    let x = 22.0;
    let y = screen_height() - 184.0;
    for (index, tag) in view.control_tags.iter().enumerate() {
        draw_control_tag(
            Rect::new(x, y + index as f32 * 40.0, 158.0, 32.0),
            &tag.key_label,
            &tag.label,
        );
    }
}

fn draw_control_tag(rect: Rect, key: &str, label: &str) {
    draw_tag_panel(rect);
    draw_keycap(
        Rect::new(rect.x + 12.0, rect.y + 5.0, 40.0, 22.0),
        key,
        false,
    );
    draw_text(
        label,
        rect.x + 64.0,
        rect.y + 22.0,
        19.0,
        Color::from_rgba(44, 34, 26, 255),
    );
    draw_small_diamond(vec2(rect.x + rect.w - 9.0, rect.y + rect.h * 0.5), brass());
}
