use crate::view_models::shop::ShopOverlayView;
use super::{
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle, draw_panel,
    draw_overlay_section_box, draw_overlay_section_title, draw_overlay_tab, draw_selection_card,
    draw_state_banner,
};
use macroquad::prelude::*;

pub(crate) fn draw_shop_overlay_view(view: &ShopOverlayView) {
        draw_overlay_backdrop();
        let x = 160.0;
        let y = 88.0;
        let w = screen_width() - 320.0;
        let h = screen_height() - 176.0;
        draw_panel(x, y, w, h, &view.station_name);
        draw_overlay_subtitle(x, y, &view.subtitle);
        draw_overlay_tab(
            Rect::new(x + 20.0, y + 88.0, 112.0, 30.0),
            &view.buy_tab_label,
            view.buy_tab_active,
        );
        draw_overlay_tab(
            Rect::new(x + 140.0, y + 88.0, 112.0, 30.0),
            &view.sell_tab_label,
            view.sell_tab_active,
        );
        draw_overlay_section_title(
            x + 20.0,
            y + 148.0,
            &view.stock_title,
            Some(&view.sort_text),
        );
        draw_overlay_section_box(x + 20.0, y + 162.0, w - 40.0, h - 224.0);

        let mut row_y = y + 196.0;
        if let Some(safe_banner) = &view.safe_sell_banner {
            draw_state_banner(x + 32.0, row_y - 16.0, w - 64.0, safe_banner, false);
            row_y += 38.0;
        }
        if view.entries.is_empty() {
            draw_state_banner(x + 32.0, row_y - 16.0, w - 64.0, &view.empty_text, false);
        } else {
            for entry in &view.entries {
                draw_selection_card(
                    x + 32.0,
                    row_y - 24.0,
                    w - 64.0,
                    52.0,
                    entry.selected,
                    entry.enabled,
                    &entry.title,
                    &entry.detail,
                    &entry.meta,
                );
                row_y += 60.0;
                if row_y > y + h - 40.0 {
                    break;
                }
            }
        }
        draw_overlay_footer(x, y, w, h, &view.footer_text);
}
