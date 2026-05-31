use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::{AreaDefinition, GameData};
use crate::view_models::hud::{HudControlTag, HudPotionSlot, HudView, HOTBAR_SLOT_COUNT};

impl GameplayState {
    pub(super) fn build_hud_view(&self, area: &AreaDefinition, data: &GameData) -> HudView {
        let quick = self.quick_potions(data);
        let potions: [HudPotionSlot; HOTBAR_SLOT_COUNT] = std::array::from_fn(|index| {
            if let Some(item_id) = quick.get(index) {
                HudPotionSlot {
                    key_label: quick_potion_key_label(index),
                    icon_id: Some(item_id.clone()),
                    amount: self.inventory.get(item_id).copied().unwrap_or_default(),
                }
            } else {
                HudPotionSlot {
                    key_label: quick_potion_key_label(index),
                    icon_id: None,
                    amount: 0,
                }
            }
        });
        let season_label = title_case_label(self.current_season());
        let weather_label = title_case_label(self.current_weather());
        let vitality_value = format!("{:.0}", self.vitality);

        HudView {
            game_title: ui_copy("menu_title").to_owned(),
            vitality_label: ui_copy("hud_vitality_label").to_owned(),
            vitality_text: ui_format("hud_vitality_value", &[("value", &vitality_value)]),
            coins_label: ui_copy("hud_coins_label").to_owned(),
            coins_value: self.coins.to_string(),
            clock_text: clock_text_12h(
                self.world.day_clock_seconds,
                data.config.day_length_seconds,
            ),
            season_weather_text: ui_format(
                "hud_conditions",
                &[("season", &season_label), ("weather", &weather_label)],
            ),
            day_text: ui_format(
                "hud_day_count",
                &[("day", &(self.world.day_index + 1).to_string())],
            ),
            sleep_warning_text: if self.current_clock_minutes() < 60.0 {
                Some(ui_copy("hud_sleep_warning").to_owned())
            } else {
                None
            },
            goal_prefix: ui_copy("hud_current_goal").to_owned(),
            goal: self.hud_goal(data),
            status_text: self.runtime.status_text.clone(),
            area_label: area.name.clone(),
            inventory_label: ui_copy("hud_drawer_inventory").to_owned(),
            effects_label: ui_copy("hud_drawer_effects").to_owned(),
            no_effects_label: ui_copy("overlay_none").to_owned(),
            journal_label: ui_copy("hud_drawer_journal").to_owned(),
            journal_key_label: input_bindings().global.journal.clone(),
            minimap_north_label: ui_copy("hud_minimap_north").to_owned(),
            control_tags: hud_control_tags(),
            truncation_suffix: ui_copy("hud_truncation_suffix").to_owned(),
            potions,
            inventory_count: self.inventory.values().copied().sum(),
            effect_count: self.runtime.active_effects.len(),
            feedbacks: self.build_hud_feedbacks(area),
        }
    }

}

fn hud_control_tags() -> Vec<HudControlTag> {
    let bindings = input_bindings();
    vec![
        HudControlTag {
            key_label: bindings.alchemy.open.clone(),
            label: ui_copy("hud_control_alchemy").to_owned(),
        },
        HudControlTag {
            key_label: bindings.global.journal.clone(),
            label: ui_copy("hud_drawer_journal").to_owned(),
        },
        HudControlTag {
            key_label: bindings.global.sort.clone(),
            label: ui_copy("hud_control_sort").to_owned(),
        },
        HudControlTag {
            key_label: bindings.global.cancel.clone(),
            label: ui_copy("hud_control_pause").to_owned(),
        },
    ]
}

fn clock_text_12h(day_clock_seconds: f32, full_day_seconds: f32) -> String {
    let total_minutes = ((day_clock_seconds / full_day_seconds) * 24.0 * 60.0) as i32;
    let hour_24 = (total_minutes / 60).rem_euclid(24);
    let minute = total_minutes.rem_euclid(60);
    let period = if hour_24 < 12 {
        ui_copy("hud_time_period_am")
    } else {
        ui_copy("hud_time_period_pm")
    };
    let hour_12 = match hour_24 % 12 {
        0 => 12,
        hour => hour,
    };
    let hour = format!("{hour_12:02}");
    let minute = format!("{minute:02}");
    ui_format(
        "hud_clock",
        &[("hour", &hour), ("minute", &minute), ("period", period)],
    )
}

fn title_case_label(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
}

fn quick_potion_key_label(index: usize) -> &'static str {
    input_bindings()
        .global
        .quick_potions
        .get(index)
        .map(String::as_str)
        .unwrap_or("")
}
