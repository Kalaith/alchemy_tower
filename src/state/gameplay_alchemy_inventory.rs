use super::GameplayState;
use crate::alchemy::resolve_brew;
use crate::audio::AudioAssets;
use crate::content::{narrative_text, ui_format};
use crate::data::{GameData, StationDefinition};
use macroquad::prelude::{vec2, Color};

impl GameplayState {
    pub(super) fn brew_selected(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        audio: &AudioAssets,
    ) {
        let selected = self.selected_items();
        if selected.is_empty() {
            self.runtime.status_text = narrative_text().statuses.cauldron_empty.clone();
            return;
        }
        self.save_last_brew_setup();
        let resolution = resolve_brew(
            data,
            station,
            &selected,
            self.selected_catalyst(),
            self.alchemy.heat,
            self.alchemy.stirs,
            self.alchemy_timing(),
            self.preview_mastery_brews(data, station, &selected),
        );
        let stable_brew = self.brew_is_stable(&resolution);
        let brew_feedback_color = if resolution.recipe.is_none() {
            Color::from_rgba(196, 162, 255, 255)
        } else if stable_brew {
            Color::from_rgba(188, 255, 220, 255)
        } else {
            Color::from_rgba(255, 214, 132, 255)
        };
        self.trigger_world_feedback(
            vec2(station.position[0], station.position[1]),
            brew_feedback_color,
            stable_brew || resolution.recipe.is_none(),
            if stable_brew { 1.9 } else { 1.4 },
        );
        self.trigger_camera_shake(
            if stable_brew { 0.12 } else { 0.08 },
            if stable_brew { 3.6 } else { 2.0 },
        );
        self.consume_brew_inputs(&selected);
        let previous_profile = self.record_brew_inventory_result(data, &resolution, stable_brew);
        self.update_brew_result_status(data, &resolution, stable_brew);
        audio.play_brew_result(self.brew_is_stable(&resolution));
        let current_profile = self
            .progression
            .crafted_item_profiles
            .get(&resolution.output_item_id);
        let improved_best = current_profile
            .zip(previous_profile.as_ref())
            .map(|(current, previous)| current.best_quality_score > previous.best_quality_score)
            .unwrap_or(current_profile.is_some());
        if improved_best {
            if let Some(profile) = current_profile {
                self.push_event_toast_with_icon(
                    ui_format(
                        "inventory_new_best",
                        &[
                            ("item", data.item_name(&resolution.output_item_id)),
                            ("band", &profile.best_quality_band),
                        ],
                    ),
                    Color::from_rgba(188, 255, 220, 255),
                    "best_quality",
                );
            }
        }
        if self.progression.total_brews == 10 {
            self.push_event_toast_with_icon(
                &ui_format("inventory_greenhouse_unlock", &[]),
                Color::from_rgba(200, 255, 200, 255),
                "route_restored",
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(200, 255, 200, 255),
                true,
                2.1,
            );
            self.trigger_camera_shake(0.2, 5.4);
            self.runtime.status_text = narrative_text().statuses.greenhouse_unlock.clone();
        }
        self.alchemy.stirs = 0;
        self.alchemy.timing_index = 0;
        self.alchemy.slots = [None, None, None];
        self.alchemy.catalyst = None;
    }
}
