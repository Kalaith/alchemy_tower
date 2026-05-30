use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::shop::{ShopOverlayEntry, ShopOverlayView};

struct ShopEntryDraft {
    item_id: String,
    price: u32,
    enabled: bool,
    safe_to_sell: bool,
}

impl GameplayState {
    pub(super) fn shop_overlay_view(&self, data: &GameData) -> Option<ShopOverlayView> {
        let station = self.nearby_station(data)?;
        let buying = self.shop_buy_tab_active();
        let drafts = if buying {
            station
                .stock
                .iter()
                .map(|stock| ShopEntryDraft {
                    item_id: stock.item_id.clone(),
                    price: stock.price,
                    enabled: self.coins >= stock.price,
                    safe_to_sell: false,
                })
                .collect::<Vec<_>>()
        } else {
            self.sell_candidates(data)
                .into_iter()
                .map(|item_id| {
                    let price = self.sell_price(data, &item_id);
                    let amount = self.inventory.get(&item_id).copied().unwrap_or_default();
                    let safe_to_sell = self.sell_is_safe(data, &item_id);
                    ShopEntryDraft {
                        item_id,
                        price,
                        enabled: amount > 0,
                        safe_to_sell,
                    }
                })
                .collect::<Vec<_>>()
        };
        let safe_sell_banner = if buying {
            None
        } else {
            Some(
                self.selected_shop_entry(&drafts)
                    .map(|draft| {
                        if draft.safe_to_sell {
                            ui_copy("overlay_safe_sell").to_owned()
                        } else {
                            ui_copy("overlay_keep_stock").to_owned()
                        }
                    })
                    .unwrap_or_else(|| ui_copy("overlay_safe_sell").to_owned()),
            )
        };
        let entries = drafts
            .into_iter()
            .enumerate()
            .map(|(index, draft)| {
                let amount = self.inventory.get(&draft.item_id).copied().unwrap_or_default();
                let detail = self.inventory_reference_summary(data, &draft.item_id);
                let price = draft.price.to_string();
                let extra = if buying {
                    ui_format("overlay_buy_price", &[("price", &price)])
                } else if draft.safe_to_sell {
                    ui_format("overlay_sell_price_safe", &[("price", &price)])
                } else {
                    ui_format("overlay_sell_price", &[("price", &price)])
                };
                ShopOverlayEntry {
                    title: data.item_name(&draft.item_id).to_owned(),
                    detail,
                    meta: self.item_card_meta(data, &draft.item_id, amount, &extra),
                    enabled: draft.enabled,
                    selected: self.shop_item_selected(index),
                }
            })
            .collect();

        Some(ShopOverlayView {
            station_name: station.name.clone(),
            coin_count: self.coins,
            buy_tab_active: buying,
            sell_tab_active: !buying,
            stock_title: if buying {
                ui_copy("overlay_shop_stock").to_owned()
            } else {
                ui_copy("overlay_shop_sellable_stock").to_owned()
            },
            sort_label: self.inventory_sort_label().to_owned(),
            empty_text: if buying {
                self.unavailable_state_text(ui_copy("overlay_shop_empty_buy"))
            } else {
                self.unavailable_state_text(ui_copy("overlay_shop_empty_sell"))
            },
            safe_sell_banner,
            entries,
        })
    }
}
