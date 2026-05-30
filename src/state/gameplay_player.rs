use super::{GameplayState, OverlayScreen};
use crate::audio::AudioAssets;
use crate::content::{narrative_text, ui_copy, ui_format, ui_text};
use crate::data::{GameData, StationKind};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn update_movement(&mut self, data: &GameData, frame_time: f32) {
        let Some(area) = data.area(&self.world.current_area_id) else {
            return;
        };
        let mut direction = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            direction.x += 1.0;
        }
        if direction.length_squared() == 0.0 {
            self.world.player.moving = false;
            return;
        }

        let normalized = direction.normalize();
        let speed = data.config.move_speed * self.move_speed_multiplier();
        let previous = self.world.player.position;
        let candidate = self.world.player.position + normalized * speed * frame_time;
        self.world.player.position = self.resolve_area_collision(area, candidate);
        self.world.player.facing = normalized;
        self.world.player.moving = self.world.player.position.distance_squared(previous) > 0.01;
    }

    pub(super) fn update_footstep_audio(&mut self, audio: &AudioAssets, frame_time: f32) {
        if !self.world.player.moving {
            self.runtime.footstep_cooldown_seconds =
                self.runtime.footstep_cooldown_seconds.min(0.06);
            return;
        }
        let step_interval = (0.34 / self.move_speed_multiplier()).clamp(0.16, 0.34);
        if self.runtime.footstep_cooldown_seconds > 0.0 {
            self.runtime.footstep_cooldown_seconds =
                (self.runtime.footstep_cooldown_seconds - frame_time).max(0.0);
            return;
        }
        audio.play_footstep_for_area(&self.world.current_area_id);
        self.runtime.footstep_cooldown_seconds = step_interval;
    }

    pub(super) fn handle_interactions(&mut self, data: &GameData, audio: &AudioAssets) {
        if let Some(station) = self.nearby_station(data) {
            if station.kind == StationKind::Alchemy
                && (is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::E))
            {
                self.set_overlay(OverlayScreen::Alchemy);
                self.alchemy.index = 0;
                self.runtime.status_text = ui_text().statuses.open_alchemy.clone();
                audio.play_alchemy_open();
                return;
            }
        }

        let Some(area) = data.area(&self.world.current_area_id) else {
            return;
        };
        if !is_key_pressed(KeyCode::E) {
            return;
        }

        if let Some(npc) = self.nearby_npc(data) {
            self.set_overlay(OverlayScreen::Dialogue(npc.id.clone()));
            self.runtime.status_text = ui_format("gameplay_talking_to", &[("name", &npc.name)]);
            return;
        }

        if let Some(station) = self.nearby_station(data) {
            if station.kind == StationKind::Shop {
                self.set_overlay(OverlayScreen::Shop);
                self.ui.shop_buy_tab = true;
                self.ui.shop_index = 0;
                self.runtime.status_text =
                    ui_format("gameplay_opened_station", &[("name", &station.name)]);
                return;
            }
            if station.kind == StationKind::RuneWorkshop {
                self.set_overlay(OverlayScreen::Rune);
                self.ui.rune_index = 0;
                self.runtime.status_text =
                    ui_format("gameplay_opened_station", &[("name", &station.name)]);
                return;
            }
            if station.kind == StationKind::ArchiveConsole {
                self.set_overlay(OverlayScreen::Archive);
                self.ui.archive_tab = 0;
                self.ui.archive_index = 0;
                self.runtime.status_text =
                    ui_format("gameplay_opened_station", &[("name", &station.name)]);
                return;
            }
            if station.kind == StationKind::EndingFocus {
                if self.has_journal_milestone("archive_revelation") {
                    self.set_overlay(OverlayScreen::Ending);
                    let milestone = &narrative_text().milestones.observatory_ending;
                    self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
                    self.runtime.status_text = ui_format("gameplay_observatory_aligned", &[]);
                } else {
                    self.runtime.status_text = ui_copy("observatory_locked").to_owned();
                }
                return;
            }
            if station.kind == StationKind::QuestBoard {
                self.set_overlay(OverlayScreen::QuestBoard);
                self.ui.shop_index = 0;
                self.runtime.status_text = ui_text().statuses.reading_quest_board.clone();
                return;
            }
            if station.kind == StationKind::RestBed {
                self.sleep_until(data, 7.0 * 60.0, false);
                return;
            }
            if station.kind == StationKind::Planter {
                self.interact_with_planter(data, station);
                return;
            }
            if station.kind == StationKind::Habitat {
                self.interact_with_habitat(data, station);
                return;
            }
        }

        if let Some(warp) = area
            .warps
            .iter()
            .find(|warp| warp.rect.contains_point(self.world.player.position))
        {
            if !self.warp_is_unlocked(warp) {
                if self.can_unlock_warp(warp) {
                    self.pay_warp_costs(warp);
                    self.progression.unlocked_warps.insert(warp.id.clone());
                    for milestone in &warp.unlock_milestones {
                        self.push_journal_milestone(
                            &milestone.id,
                            &milestone.title,
                            &milestone.text,
                        );
                    }
                    self.push_event_toast_with_icon(
                        ui_format("gameplay_route_restored", &[("label", &warp.label)]),
                        Color::from_rgba(188, 255, 220, 255),
                        "route_restored",
                    );
                    self.trigger_world_feedback(
                        vec2(
                            warp.rect.x + warp.rect.w * 0.5,
                            warp.rect.y + warp.rect.h * 0.5,
                        ),
                        Color::from_rgba(188, 255, 220, 255),
                        true,
                        2.0,
                    );
                    self.trigger_camera_shake(0.18, 4.8);
                    self.runtime.status_text =
                        ui_format("gameplay_repaired_access", &[("label", &warp.label)]);
                } else {
                    self.runtime.status_text = self.warp_lock_text(data, warp);
                    return;
                }
            }
            self.world.current_area_id = warp.target_area.clone();
            self.world.player.position = vec2(warp.target_position[0], warp.target_position[1]);
            self.world.player.moving = false;
            self.refresh_available_nodes(data);
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(255, 245, 160, 255),
                false,
                1.2,
            );
            return;
        }

        if let Some(node) = area.gather_nodes.iter().find(|node| {
            !self.world.gathered_nodes.contains(&node.id)
                && self
                    .world
                    .player
                    .position
                    .distance(vec2(node.position[0], node.position[1]))
                    <= node.radius + data.config.interaction_range
        }) {
            if self.node_is_available(node) {
                self.world.gathered_nodes.insert(node.id.clone());
                *self.inventory.entry(node.item_id.clone()).or_insert(0) += 1;
                self.note_inventory_observation(data, &node.item_id);
                let discovery = self.record_field_discovery(data, node);
                self.trigger_gather_feedback(data, node, &discovery);
                self.runtime.status_text = self.gather_status_text(data, node, &discovery);
                audio.play_gather_pickup();
            } else {
                self.runtime.status_text = self.gather_attempt_status(data, node);
            }
        }
    }

}
