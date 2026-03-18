use super::gameplay_npc::npc_motion_seed;
use super::*;
use crate::content::{narrative_text, ui_format};

impl GameplayState {
    pub(super) fn greenhouse_journal_unlocked(&self) -> bool {
        self.progression.unlocked_warps.contains("entry_to_greenhouse")
    }

    pub(super) fn journal_tabs(&self) -> Vec<&'static str> {
        let mut tabs = vec!["Routes", "Notes", "Brews"];
        if self.greenhouse_journal_unlocked() {
            tabs.push("Greenhouse");
        }
        tabs.push("Rapport");
        tabs
    }

    pub(super) fn journal_tab_rect(&self, index: usize, tab_count: usize) -> Rect {
        let x = 120.0;
        let y = 72.0;
        let w = screen_width() - 240.0;
        let tab_y = y + 82.0;
        let tab_w = (w - 40.0) / tab_count.max(1) as f32;
        Rect::new(x + 20.0 + tab_w * index as f32, tab_y, tab_w - 8.0, 30.0)
    }

    pub(super) fn journal_close_rect(&self) -> Rect {
        let x = 120.0;
        let y = 72.0;
        let w = screen_width() - 240.0;
        Rect::new(x + w - 112.0, y + 16.0, 92.0, 28.0)
    }

    pub(super) fn push_event_toast(&mut self, text: impl Into<String>, color: Color) {
        self.runtime.gather_toasts.insert(
            0,
            GatherToast {
                text: text.into(),
                remaining_seconds: 2.2,
                color,
            },
        );
        self.runtime.gather_toasts.truncate(3);
    }

    pub(super) fn resolve_area_collision(&self, area: &AreaDefinition, candidate: Vec2) -> Vec2 {
        let clamped = vec2(
            candidate
                .x
                .clamp(PLAYER_RADIUS, area.size[0] - PLAYER_RADIUS),
            candidate
                .y
                .clamp(PLAYER_RADIUS, area.size[1] - PLAYER_RADIUS),
        );
        if !self.collides(area, clamped) {
            return clamped;
        }
        let x_only = vec2(clamped.x, self.world.player.position.y);
        if !self.collides(area, x_only) {
            return x_only;
        }
        let y_only = vec2(self.world.player.position.x, clamped.y);
        if !self.collides(area, y_only) {
            return y_only;
        }
        self.world.player.position
    }

    pub(super) fn collides(&self, area: &AreaDefinition, point: Vec2) -> bool {
        area.blockers.iter().any(|blocker| {
            let nearest = vec2(
                point.x.clamp(blocker.x, blocker.x + blocker.w),
                point.y.clamp(blocker.y, blocker.y + blocker.h),
            );
            point.distance_squared(nearest) < PLAYER_RADIUS * PLAYER_RADIUS
        })
    }

    pub(super) fn camera_offset(&self, area: &AreaDefinition) -> Vec2 {
        let half = vec2(screen_width() * 0.5, screen_height() * 0.5);
        let unclamped = half - self.world.player.position;
        let min_x = screen_width() - area.size[0] - CAMERA_PADDING;
        let min_y = screen_height() - area.size[1] - CAMERA_PADDING;
        vec2(
            unclamped.x.clamp(min_x.min(CAMERA_PADDING), CAMERA_PADDING),
            unclamped.y.clamp(min_y.min(CAMERA_PADDING), CAMERA_PADDING),
        )
    }

    pub(super) fn npc_draw_position(&self, npc: &NpcDefinition, runtime: &NpcRuntimeState) -> Vec2 {
        if !runtime.moving || runtime.direction.length_squared() <= 0.0 {
            return runtime.position;
        }
        let perpendicular = vec2(-runtime.direction.y, runtime.direction.x);
        let seed = npc_motion_seed(&npc.id);
        let sway = ((get_time() as f32 * 4.5) + seed).sin() * 1.6;
        runtime.position + perpendicular * sway
    }

    pub(super) fn item_card_meta(
        &self,
        data: &GameData,
        item_id: &str,
        amount: u32,
        extra: &str,
    ) -> String {
        let item = data.item(item_id);
        let base = format!(
            "{}  q{} r{}  x{}",
            item.map(|item| item.category.as_str()).unwrap_or("?"),
            item.map(|item| item.quality).unwrap_or_default(),
            item.map(|item| item.rarity).unwrap_or_default(),
            amount
        );
        if extra.is_empty() {
            base
        } else {
            format!("{base}  {extra}")
        }
    }

    pub(super) fn locked_state_text(&self, detail: &str) -> String {
        ui_format("locked_prefix", &[("detail", detail)])
    }

    pub(super) fn unavailable_state_text(&self, detail: &str) -> String {
        ui_format("unavailable_prefix", &[("detail", detail)])
    }

    pub(super) fn current_time_window(&self) -> &'static str {
        let total_minutes = self.current_clock_minutes() as i32;
        match total_minutes {
            360..=659 => "morning",
            660..=1019 => "day",
            1020..=1259 => "evening",
            _ => "night",
        }
    }

    pub(super) fn current_clock_minutes(&self) -> f32 {
        (self.world.day_clock_seconds / self.world.day_length_seconds) * 24.0 * 60.0
    }
}

pub(super) fn rgba(values: [u8; 4]) -> Color {
    Color::from_rgba(values[0], values[1], values[2], values[3])
}

pub(super) fn clock_text(day_clock_seconds: f32, full_day_seconds: f32) -> String {
    let total_minutes = ((day_clock_seconds / full_day_seconds) * 24.0 * 60.0) as i32;
    format!(
        "{:02}:{:02}",
        (total_minutes / 60).rem_euclid(24),
        total_minutes.rem_euclid(60)
    )
}

pub(super) fn effect_name(kind: EffectKind) -> &'static str {
    match kind {
        EffectKind::Glow => "Glow",
        EffectKind::Speed => "Quickstep",
        EffectKind::Misfire => "Residue",
        EffectKind::Restore => "Restore",
    }
}

pub(super) fn quality_band_rank(band: &str) -> u8 {
    match band {
        "Crude" => 0,
        "Serviceable" => 1,
        "Fine" => 2,
        "Excellent" => 3,
        "Masterwork" => 4,
        _ => 0,
    }
}

pub(super) fn planter_stage_label(growth_days: u32, total_days: u32) -> &'static str {
    if growth_days == 0 {
        "seeded"
    } else if growth_days >= total_days {
        "ripe"
    } else if growth_days * 2 >= total_days {
        "budding"
    } else {
        "sprouting"
    }
}

pub(super) fn starting_day_time(data: &GameData) -> f32 {
    data.config.day_length_seconds * 0.30
}

pub(super) fn initial_journal_milestones() -> Vec<JournalMilestoneEntry> {
    vec![narrative_text()
        .milestones
        .entry_lab_recovered
        .to_journal_entry()]
}


