use super::hud::{draw_beveled_rect, draw_beveled_rect_lines, draw_panel_texture, fill_slate};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub(crate) fn draw_overlay_section_title(x: f32, y: f32, title: &str, meta: Option<&str>) {
    // Small brass diamond marker, echoing the HUD goal-note headers so overlay
    // sections read as part of the same ornate UI family.
    draw_section_diamond(x - 12.0, y - 7.0);
    draw_ui_text(title, x, y, 24.0, dark::TEXT_BRIGHT);
    if let Some(meta) = meta {
        draw_ui_text(meta, x + 208.0, y, 18.0, dark::TEXT_DIM);
    }
}

pub(crate) fn draw_overlay_section_box(x: f32, y: f32, w: f32, h: f32) {
    // Beveled, lightly textured recess with a brass edge — the same treatment as
    // the HUD's inset sub-panels (goal note, status plaques).
    let rect = Rect::new(x, y, w, h);
    let bevel = 6.0;
    draw_beveled_rect(rect, bevel, Color::from_rgba(18, 17, 22, 214));
    draw_panel_texture(rect, bevel, fill_slate(), 0.4);
    draw_beveled_rect_lines(rect, bevel, 1.5, Color::from_rgba(223, 184, 111, 120));
    draw_beveled_rect_lines(
        Rect::new(x + 3.0, y + 3.0, w - 6.0, h - 6.0),
        (bevel - 2.0).max(3.0),
        1.0,
        Color::from_rgba(255, 239, 184, 42),
    );
}

fn draw_section_diamond(cx: f32, cy: f32) {
    let color = Color::from_rgba(242, 205, 126, 235);
    let r = 4.0;
    draw_triangle(
        vec2(cx, cy - r),
        vec2(cx + r, cy),
        vec2(cx, cy + r),
        color,
    );
    draw_triangle(
        vec2(cx, cy - r),
        vec2(cx - r, cy),
        vec2(cx, cy + r),
        color,
    );
}

pub(crate) fn draw_overlay_tab(rect: Rect, label: &str, selected: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            Color::from_rgba(30, 34, 44, 220)
        } else {
            Color::from_rgba(16, 18, 26, 150)
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        if selected {
            Color::from_rgba(255, 238, 196, 92)
        } else {
            Color::from_rgba(160, 170, 190, 56)
        },
    );
    let measured = measure_ui_text(label, None, 18, 1.0);
    draw_ui_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + 21.0,
        18.0,
        if selected {
            Color::from_rgba(248, 242, 230, 255)
        } else {
            dark::TEXT_DIM
        },
    );
}
