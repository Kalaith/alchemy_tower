use super::{draw_overlay_section_box, draw_overlay_section_title, draw_overlay_tab, GameplayState};
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::ui::{
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle, draw_panel,
    draw_selection_card, draw_state_banner,
};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_shop_overlay(&self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            return;
        };
        draw_overlay_backdrop();
        let x = 160.0;
        let y = 88.0;
        let w = screen_width() - 320.0;
        let h = screen_height() - 176.0;
        draw_panel(x, y, w, h, &station.name);
        draw_overlay_subtitle(
            x,
            y,
            &ui_text()
                .overlays
                .shop_subtitle
                .replace("{coins}", &self.coins.to_string()),
        );
        draw_overlay_tab(
            Rect::new(x + 20.0, y + 88.0, 112.0, 30.0),
            ui_copy("overlay_shop_buy_tab"),
            self.ui.shop_buy_tab,
        );
        draw_overlay_tab(
            Rect::new(x + 140.0, y + 88.0, 112.0, 30.0),
            ui_copy("overlay_shop_sell_tab"),
            !self.ui.shop_buy_tab,
        );
        draw_overlay_section_title(
            x + 20.0,
            y + 148.0,
            if self.ui.shop_buy_tab {
                ui_copy("overlay_shop_stock")
            } else {
                ui_copy("overlay_shop_sellable_stock")
            },
            Some(&ui_format(
                "overlay_sort_mode",
                &[("mode", self.inventory_sort_label())],
            )),
        );
        draw_overlay_section_box(x + 20.0, y + 162.0, w - 40.0, h - 224.0);

        let entries = if self.ui.shop_buy_tab {
            station
                .stock
                .iter()
                .map(|stock| {
                    (
                        stock.item_id.clone(),
                        stock.price,
                        self.coins >= stock.price,
                    )
                })
                .collect::<Vec<_>>()
        } else {
            self.sell_candidates(data)
                .into_iter()
                .map(|item_id| {
                    let price = self.sell_price(data, &item_id);
                    let amount = self.inventory.get(&item_id).copied().unwrap_or_default();
                    (item_id, price, amount > 0)
                })
                .collect::<Vec<_>>()
        };

        let mut row_y = y + 196.0;
        if !self.ui.shop_buy_tab {
            let safe_banner = entries
                .get(self.ui.shop_index)
                .map(|(item_id, _, _)| {
                    if self.sell_is_safe(data, item_id) {
                        ui_copy("overlay_safe_sell").to_owned()
                    } else {
                        ui_copy("overlay_keep_stock").to_owned()
                    }
                })
                .unwrap_or_else(|| ui_copy("overlay_safe_sell").to_owned());
            draw_state_banner(x + 32.0, row_y - 16.0, w - 64.0, &safe_banner, false);
            row_y += 38.0;
        }
        if entries.is_empty() {
            let empty_text = if self.ui.shop_buy_tab {
                self.unavailable_state_text(ui_copy("overlay_shop_empty_buy"))
            } else {
                self.unavailable_state_text(ui_copy("overlay_shop_empty_sell"))
            };
            draw_state_banner(x + 32.0, row_y - 16.0, w - 64.0, &empty_text, false);
        } else {
            for (index, (item_id, price, enabled)) in entries.iter().enumerate() {
                let selected = index == self.ui.shop_index;
                let amount = self.inventory.get(item_id).copied().unwrap_or_default();
                let detail = self.inventory_reference_summary(data, item_id);
                let meta = if self.ui.shop_buy_tab {
                    self.item_card_meta(
                        data,
                        item_id,
                        amount,
                        &ui_format("overlay_buy_price", &[("price", &price.to_string())]),
                    )
                } else {
                    let sell_tag = if self.sell_is_safe(data, item_id) {
                        ui_format("overlay_sell_price_safe", &[("price", &price.to_string())])
                    } else {
                        ui_format("overlay_sell_price", &[("price", &price.to_string())])
                    };
                    self.item_card_meta(data, item_id, amount, &sell_tag)
                };
                draw_selection_card(
                    x + 32.0,
                    row_y - 24.0,
                    w - 64.0,
                    52.0,
                    selected,
                    *enabled,
                    data.item_name(item_id),
                    &detail,
                    &meta,
                );
                row_y += 60.0;
                if row_y > y + h - 40.0 {
                    break;
                }
            }
        }
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &ui_format(
                "overlay_shop_footer",
                &[
                    ("switch", &input_bindings().shop.switch_tab),
                    ("select", &input_bindings().navigation.select),
                    ("confirm", &input_bindings().global.confirm),
                    ("close", &input_bindings().global.cancel),
                ],
            ),
        );
    }
}
