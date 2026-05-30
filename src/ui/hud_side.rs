use super::hud_chrome::*;
use super::hud_primitives::*;
pub(super) use super::hud_side_hardware::{
    draw_side_status_divider, draw_side_status_hardware, draw_status_icon_medallion,
};
use super::HudView;
use crate::content::ui_copy;
use macroquad::prelude::*;

pub(super) fn draw_side_status_panel(view: &HudView) {
    let rect = Rect::new(screen_width() - 104.0, 214.0, 84.0, 238.0);
    draw_vertical_plaque_backplate(rect);
    draw_ornate_panel(rect, Color::from_rgba(27, 25, 20, 218), 0.9);
    draw_panel_filigree(rect, 0.58);
    draw_side_status_hardware(rect);
    draw_centered_text(
        ui_copy("hud_drawer_inventory"),
        rect.x,
        rect.y + 25.0,
        rect.w,
        15.0,
        brass_light(),
    );
    draw_status_icon_medallion(
        vec2(rect.x + 30.0, rect.y + 53.0),
        Color::from_rgba(222, 174, 112, 84),
    );
    draw_bag_icon(vec2(rect.x + 29.0, rect.y + 52.0), 0.82);
    draw_text(
        &view.inventory_count.to_string(),
        rect.x + 48.0,
        rect.y + 58.0,
        18.0,
        bright_ink(),
    );
    draw_side_status_divider(rect, rect.y + 78.0);
    draw_centered_text(
        ui_copy("hud_drawer_effects"),
        rect.x,
        rect.y + 103.0,
        rect.w,
        15.0,
        brass_light(),
    );
    if view.effect_count > 0 {
        draw_status_icon_medallion(
            vec2(rect.x + 31.0, rect.y + 128.0),
            Color::from_rgba(85, 222, 207, 82),
        );
        draw_spark_icon(vec2(rect.x + 31.0, rect.y + 128.0), 0.9);
        draw_centered_text_shadowed(
            &view.effect_count.to_string(),
            rect.x,
            rect.y + 127.0,
            rect.w,
            20.0,
            bright_ink(),
        );
    } else {
        draw_centered_text(
            ui_copy("overlay_none"),
            rect.x,
            rect.y + 129.0,
            rect.w,
            17.0,
            bright_ink(),
        );
    }
    draw_side_status_divider(rect, rect.y + 156.0);
    draw_centered_text(
        ui_copy("hud_drawer_journal"),
        rect.x,
        rect.y + 188.0,
        rect.w,
        16.0,
        brass_light(),
    );
    draw_status_icon_medallion(
        vec2(rect.x + 30.0, rect.y + 213.0),
        Color::from_rgba(84, 124, 110, 88),
    );
    draw_book_icon(vec2(rect.x + 21.0, rect.y + 212.0), 0.8);
    draw_keycap(
        Rect::new(rect.x + 28.0, rect.y + 197.0, 30.0, 30.0),
        "J",
        true,
    );
}
