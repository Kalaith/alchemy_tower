use super::hud_atmosphere::*;
use super::hud_banner::*;
use super::hud_banner_hardware::*;
use super::hud_gem_mount::*;
use super::hud_primitives::*;
use super::truncate_text_to_width;
use super::HudView;
use macroquad::prelude::*;

pub(super) fn draw_hud_vignette() {
    let sw = screen_width();
    let sh = screen_height();
    draw_hud_atmosphere(sw, sh);

    for band in 0..6 {
        let t = band as f32 / 5.0;
        let alpha = (95.0 * (1.0 - t)) as u8;
        let inset = band as f32 * 12.0;
        draw_rectangle(0.0, inset, sw, 12.0, Color::from_rgba(0, 0, 0, alpha));
        draw_rectangle(
            0.0,
            sh - inset - 12.0,
            sw,
            12.0,
            Color::from_rgba(0, 0, 0, alpha),
        );
        draw_rectangle(
            inset,
            0.0,
            12.0,
            sh,
            Color::from_rgba(0, 0, 0, alpha.saturating_sub(18)),
        );
        draw_rectangle(
            sw - inset - 12.0,
            0.0,
            12.0,
            sh,
            Color::from_rgba(0, 0, 0, alpha.saturating_sub(18)),
        );
    }

    draw_edge_foliage(sw, sh);
}

pub(super) fn draw_title_banner(view: &HudView) {
    let width = 500.0;
    let height = 62.0;
    let x = screen_width() * 0.5 - width * 0.5;
    let y = 12.0;
    let top_plaque = Rect::new(x + 130.0, y - 8.0, width - 260.0, 24.0);
    let main = Rect::new(x, y + 14.0, width, height);

    draw_banner_backplate(main);
    draw_flourish_line(x - 72.0, y + 28.0, x + 44.0, y + 28.0, true);
    draw_flourish_line(
        x + width - 44.0,
        y + 28.0,
        x + width + 72.0,
        y + 28.0,
        false,
    );
    draw_title_vines(x, y, width);
    draw_ornate_panel(top_plaque, fill_slate(), 0.82);
    draw_title_plaque_caps(top_plaque);
    draw_centered_text_shadowed(
        &view.game_title,
        top_plaque.x,
        top_plaque.y + 17.0,
        top_plaque.w,
        16.0,
        parchment(),
    );

    draw_ornate_panel(main, fill_slate(), 0.96);
    draw_banner_inner_hardware(main);
    draw_small_diamond(vec2(main.x + 11.0, main.y + main.h * 0.5), brass_light());
    draw_small_diamond(
        vec2(main.x + main.w - 11.0, main.y + main.h * 0.5),
        brass_light(),
    );
    draw_centered_text_shadowed(
        &truncate_text_to_width(&view.area_label, main.w - 56.0, 32.0),
        main.x,
        main.y + 43.0,
        main.w,
        34.0,
        bright_ink(),
    );
    draw_gem_mount(vec2(main.x + main.w * 0.5, main.y + main.h + 3.0));
    draw_gem(vec2(main.x + main.w * 0.5, main.y + main.h + 3.0), 13.0);
}
