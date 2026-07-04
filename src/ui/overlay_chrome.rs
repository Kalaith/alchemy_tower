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
    draw_rectangle(x, y, w, h, Color::from_rgba(20, 18, 22, 168));
    // Warm top sheen + brass border to match the HUD's gold framing rather than
    // the old cold grey-blue outline.
    draw_rectangle(x, y, w, 2.0, Color::from_rgba(240, 198, 122, 30));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::from_rgba(223, 184, 111, 96));
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
