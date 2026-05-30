use super::gameplay_render_markers::draw_world_marker_plate;
use super::gameplay_support::rgba;
use super::{GameplayState, PLAYER_RADIUS};
use crate::art::{
    draw_blocker_prop, draw_character_frame, draw_gather_node_marker, draw_priority_marker,
    draw_station_marker, draw_texture_centered, ArtAssets,
};
use crate::content::ui_copy;
use crate::data::{AreaDefinition, EffectKind, GameData};
use crate::ui::draw_panel;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_area(&self, area: &AreaDefinition, offset: Vec2, data: &GameData, art: &ArtAssets) {
        if let Some(texture) = art.background(&area.id) {
            draw_texture_ex(
                texture,
                offset.x,
                offset.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(area.size[0], area.size[1])),
                    ..Default::default()
                },
            );
        } else {
            draw_rectangle(
                offset.x,
                offset.y,
                area.size[0],
                area.size[1],
                rgba(area.background),
            );
        }
        self.draw_environment_overlay(area, offset);
        self.draw_phase1_story_flourishes(area, offset);
        for (index, blocker) in area.blockers.iter().enumerate() {
            draw_blocker_prop(area, blocker, index, offset);
        }
        for warp in &area.warps {
            let center = vec2(
                offset.x + warp.rect.x + warp.rect.w * 0.5,
                offset.y + warp.rect.y + warp.rect.h * 0.5,
            );
            let unlock_ready = !self.warp_is_unlocked(warp) && self.can_unlock_warp(warp);
            if unlock_ready {
                let pulse = ((get_time() as f32 * 3.0) + warp.rect.x * 0.01).sin() * 0.5 + 0.5;
                if let Some(texture) = art.effect("warp_glow_effect") {
                    draw_texture_centered(
                        texture,
                        center,
                        vec2(74.0 + pulse * 12.0, 74.0 + pulse * 12.0),
                        Color::new(1.0, 1.0, 1.0, 0.55 + pulse * 0.2),
                    );
                }
                draw_rectangle(
                    offset.x + warp.rect.x,
                    offset.y + warp.rect.y,
                    warp.rect.w,
                    warp.rect.h,
                    Color::new(
                        188.0 / 255.0,
                        255.0 / 255.0,
                        220.0 / 255.0,
                        0.10 + pulse * 0.08,
                    ),
                );
                draw_circle_lines(
                    center.x,
                    center.y,
                    20.0 + pulse * 8.0,
                    2.0,
                    Color::from_rgba(188, 255, 220, 220),
                );
            }
            draw_rectangle_lines(
                offset.x + warp.rect.x,
                offset.y + warp.rect.y,
                warp.rect.w,
                warp.rect.h,
                3.0,
                if unlock_ready {
                    Color::from_rgba(188, 255, 220, 255)
                } else {
                    Color::from_rgba(255, 245, 160, 255)
                },
            );
        }
        for station in self
            .visible_stations(data)
            .into_iter()
            .filter(|station| station.area_id == area.id)
        {
            let center = vec2(
                offset.x + station.position[0],
                offset.y + station.position[1],
            );
            let player_distance = self
                .world
                .player
                .position
                .distance(vec2(station.position[0], station.position[1]));
            let nearby = player_distance <= station.interaction_radius + 60.0;
            let priority = self.station_world_label(data, station);
            draw_station_marker(station, center, priority.is_some(), art);
            if nearby || priority.is_some() {
                draw_world_marker_plate(
                    &station.name,
                    vec2(center.x, center.y + 46.0),
                    dark::TEXT_BRIGHT,
                    false,
                );
            }
            if let Some((label, color)) = priority {
                draw_world_marker_plate(&label, vec2(center.x, center.y - 34.0), color, true);
            }
        }
        for npc in self.visible_npcs(data) {
            let runtime = self.npc_runtime_state(data, npc);
            if runtime.area_id != area.id {
                continue;
            }
            let pos = self.npc_draw_position(npc, &runtime);
            let center = vec2(offset.x + pos.x, offset.y + pos.y);
            let priority = self.npc_world_label(data, npc);
            if let Some(texture) = art.character(&npc.id) {
                let facing = if runtime.direction.length_squared() > 0.0 {
                    runtime.direction
                } else {
                    vec2(0.0, 1.0)
                };
                draw_character_frame(texture, center, facing, runtime.moving, 1.0);
            } else {
                draw_circle(center.x, center.y, 18.0, rgba(npc.color));
            }
            if priority.is_some()
                || self.world.player.position.distance(pos) <= npc.interaction_radius + 54.0
            {
                draw_text(
                    &npc.name,
                    center.x - 34.0,
                    center.y - 28.0,
                    18.0,
                    dark::TEXT_BRIGHT,
                );
            }
            if let Some((label, color)) = priority {
                draw_priority_marker(center, color);
                draw_text(&label, center.x - 34.0, center.y - 50.0, 18.0, color);
            }
        }
        for node in &area.gather_nodes {
            if self.world.gathered_nodes.contains(&node.id) {
                continue;
            }
            let available = self.node_is_available(node);
            if !available {
                continue;
            }
            let color = rgba(node.color);
            let center = vec2(offset.x + node.position[0], offset.y + node.position[1]);
            draw_gather_node_marker(
                node,
                data.item(&node.item_id).map(|item| item.category),
                center,
                color,
                available,
                art,
            );
        }
    }

    pub(super) fn draw_environment_overlay(&self, area: &AreaDefinition, offset: Vec2) {
        let time_tint = match self.current_time_window() {
            "morning" => Color::from_rgba(255, 220, 170, 24),
            "day" => Color::from_rgba(255, 255, 255, 0),
            "evening" => Color::from_rgba(255, 184, 120, 38),
            _ => Color::from_rgba(72, 92, 150, 72),
        };
        if time_tint.a > 0.0 {
            draw_rectangle(offset.x, offset.y, area.size[0], area.size[1], time_tint);
        }

        match self.current_weather() {
            "mist" => {
                draw_rectangle(
                    offset.x,
                    offset.y,
                    area.size[0],
                    area.size[1],
                    Color::from_rgba(220, 228, 240, 28),
                );
                for index in 0..10 {
                    let drift = ((get_time() as f32 * 0.4) + index as f32 * 0.6).sin() * 18.0;
                    let x = offset.x + 80.0 + index as f32 * 110.0 + drift;
                    let y = offset.y + 60.0 + (index % 4) as f32 * 120.0;
                    draw_circle(
                        x,
                        y,
                        42.0 + (index % 3) as f32 * 12.0,
                        Color::from_rgba(240, 244, 248, 20),
                    );
                }
            }
            "rain" => {
                draw_rectangle(
                    offset.x,
                    offset.y,
                    area.size[0],
                    area.size[1],
                    Color::from_rgba(90, 126, 168, 26),
                );
                for index in 0..28 {
                    let wave = ((get_time() as f32 * 2.8) + index as f32 * 0.4).fract();
                    let x = offset.x + (index as f32 * 48.0).rem_euclid(area.size[0]);
                    let y = offset.y + wave * area.size[1];
                    draw_line(
                        x,
                        y,
                        x - 8.0,
                        y + 16.0,
                        2.0,
                        Color::from_rgba(200, 224, 255, 120),
                    );
                }
            }
            "windy" => {
                for index in 0..16 {
                    let wave = ((get_time() as f32 * 1.4) + index as f32 * 0.33).fract();
                    let x = offset.x + wave * area.size[0];
                    let y = offset.y + 30.0 + index as f32 * 34.0;
                    draw_line(
                        x - 10.0,
                        y,
                        x + 22.0,
                        y - 6.0,
                        2.0,
                        Color::from_rgba(232, 232, 210, 64),
                    );
                }
            }
            _ => {}
        }
    }

    pub(super) fn draw_phase1_story_flourishes(&self, area: &AreaDefinition, offset: Vec2) {
        match area.id.as_str() {
            "town_square" => {
                if self
                    .progression
                    .completed_quests
                    .contains("healing_for_mira")
                {
                    let shelf = Color::from_rgba(122, 88, 66, 255);
                    let bottle = Color::from_rgba(176, 226, 255, 255);
                    draw_rectangle(offset.x + 684.0, offset.y + 670.0, 72.0, 18.0, shelf);
                    draw_rectangle(offset.x + 694.0, offset.y + 652.0, 10.0, 18.0, bottle);
                    draw_rectangle(
                        offset.x + 714.0,
                        offset.y + 646.0,
                        12.0,
                        24.0,
                        Color::from_rgba(255, 214, 132, 255),
                    );
                    draw_rectangle(offset.x + 736.0, offset.y + 654.0, 10.0, 16.0, bottle);
                }
                if self.progression.completed_quests.contains("glow_for_rowan") {
                    for (x, y) in [(536.0, 540.0), (610.0, 470.0), (696.0, 404.0)] {
                        let pulse = ((get_time() as f32 * 2.2) + x * 0.01).sin() * 0.5 + 0.5;
                        draw_circle(
                            offset.x + x,
                            offset.y + y,
                            10.0 + pulse * 2.5,
                            Color::from_rgba(255, 228, 150, 120),
                        );
                        draw_circle(
                            offset.x + x,
                            offset.y + y,
                            5.0,
                            Color::from_rgba(255, 244, 188, 255),
                        );
                    }
                }
                if self.has_journal_milestone("greenhouse_repaired")
                    || self
                        .progression
                        .completed_quests
                        .contains("cultivation_for_brin")
                {
                    for (x, y, color) in [
                        (598.0, 760.0, Color::from_rgba(126, 220, 158, 255)),
                        (640.0, 744.0, Color::from_rgba(239, 205, 90, 255)),
                        (676.0, 764.0, Color::from_rgba(188, 255, 220, 255)),
                    ] {
                        draw_circle(offset.x + x, offset.y + y, 8.0, color);
                        draw_line(
                            offset.x + x,
                            offset.y + y + 8.0,
                            offset.x + x,
                            offset.y + y + 18.0,
                            2.0,
                            Color::from_rgba(88, 152, 102, 255),
                        );
                    }
                }
            }
            "greenhouse_floor" => {
                if self
                    .progression
                    .completed_quests
                    .contains("cultivation_for_brin")
                {
                    for (x, y) in [(690.0, 190.0), (742.0, 174.0), (794.0, 190.0)] {
                        draw_circle(
                            offset.x + x,
                            offset.y + y,
                            10.0,
                            Color::from_rgba(126, 220, 158, 255),
                        );
                        draw_circle(
                            offset.x + x + 10.0,
                            offset.y + y - 4.0,
                            7.0,
                            Color::from_rgba(239, 205, 90, 255),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    pub(super) fn draw_player(&self, offset: Vec2, art: &ArtAssets) {
        let center = offset + self.world.player.position;
        if self.effect_active(EffectKind::Glow) {
            draw_circle(
                center.x,
                center.y,
                PLAYER_RADIUS + 18.0,
                Color::from_rgba(215, 202, 255, 70),
            );
        }
        if let Some(texture) = art.player() {
            draw_character_frame(
                texture,
                center,
                self.world.player.facing,
                self.world.player.moving,
                1.0,
            );
        } else {
            draw_circle(
                center.x,
                center.y,
                PLAYER_RADIUS,
                Color::from_rgba(133, 204, 255, 255),
            );
            draw_circle_lines(center.x, center.y, PLAYER_RADIUS, 2.0, WHITE);
            draw_circle(center.x + 5.0, center.y - 4.0, 2.5, WHITE);
        }
    }

    pub(super) fn draw_sleep_flash_overlay(&self) {
        if self.runtime.sleep_flash_seconds <= 0.0 {
            return;
        }
        let t = (self.runtime.sleep_flash_seconds / 1.2).clamp(0.0, 1.0);
        let pulse = ((get_time() as f32 * 16.0).sin() * 0.5 + 0.5) * t;
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(180, 22, 18, (100.0 + pulse * 110.0) as u8),
        );
        draw_panel(
            screen_width() * 0.5 - 260.0,
            screen_height() * 0.5 - 64.0,
            520.0,
            128.0,
            ui_copy("gameplay_sleep_flash_title"),
        );
        draw_text(
            ui_copy("gameplay_fainted_home"),
            screen_width() * 0.5 - 220.0,
            screen_height() * 0.5 + 10.0,
            28.0,
            Color::from_rgba(255, 236, 216, 255),
        );
    }

}

